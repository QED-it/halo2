//! Optimized ECC gadgets.

pub mod chip;
#[cfg(test)]
pub(crate) mod tests {
    use ff::PrimeField;
    use group::{prime::PrimeCurveAffine, Curve, Group};

    use halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner, Value},
        dev::MockProver,
        plonk::{Circuit, ConstraintSystem, Error},
    };
    use lazy_static::lazy_static;
    use pasta_curves::pallas;

    use crate::ecc::{
        chip::{
            find_zs_and_us, BaseFieldElem, EccChip, EccConfig, FixedPoint, FullScalar, ShortScalar,
            H, NUM_WINDOWS, NUM_WINDOWS_SHORT,
        },
        FixedPoints,
    };
    use crate::utilities::lookup_range_check::LookupRangeCheckConfig;
    use crate::utilities_opt::lookup_range_check::LookupRangeCheckConfigOptimized;

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub(crate) struct TestFixedBases;
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub(crate) struct FullWidth(pallas::Affine, &'static [(u64, [pallas::Base; H])]);
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub(crate) struct BaseField;
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub(crate) struct Short;

    lazy_static! {
        static ref BASE: pallas::Affine = pallas::Point::generator().to_affine();
        static ref ZS_AND_US: Vec<(u64, [pallas::Base; H])> =
            find_zs_and_us(*BASE, NUM_WINDOWS).unwrap();
        static ref ZS_AND_US_SHORT: Vec<(u64, [pallas::Base; H])> =
            find_zs_and_us(*BASE, NUM_WINDOWS_SHORT).unwrap();
    }

    impl FullWidth {
        pub(crate) fn from_pallas_generator() -> Self {
            FullWidth(*BASE, &ZS_AND_US)
        }

        pub(crate) fn from_parts(
            base: pallas::Affine,
            zs_and_us: &'static [(u64, [pallas::Base; H])],
        ) -> Self {
            FullWidth(base, zs_and_us)
        }
    }

    impl FixedPoint<pallas::Affine> for FullWidth {
        type FixedScalarKind = FullScalar;

        fn generator(&self) -> pallas::Affine {
            self.0
        }

        fn u(&self) -> Vec<[[u8; 32]; H]> {
            self.1
                .iter()
                .map(|(_, us)| {
                    [
                        us[0].to_repr(),
                        us[1].to_repr(),
                        us[2].to_repr(),
                        us[3].to_repr(),
                        us[4].to_repr(),
                        us[5].to_repr(),
                        us[6].to_repr(),
                        us[7].to_repr(),
                    ]
                })
                .collect()
        }

        fn z(&self) -> Vec<u64> {
            self.1.iter().map(|(z, _)| *z).collect()
        }
    }

    impl FixedPoint<pallas::Affine> for BaseField {
        type FixedScalarKind = BaseFieldElem;

        fn generator(&self) -> pallas::Affine {
            *BASE
        }

        fn u(&self) -> Vec<[[u8; 32]; H]> {
            ZS_AND_US
                .iter()
                .map(|(_, us)| {
                    [
                        us[0].to_repr(),
                        us[1].to_repr(),
                        us[2].to_repr(),
                        us[3].to_repr(),
                        us[4].to_repr(),
                        us[5].to_repr(),
                        us[6].to_repr(),
                        us[7].to_repr(),
                    ]
                })
                .collect()
        }

        fn z(&self) -> Vec<u64> {
            ZS_AND_US.iter().map(|(z, _)| *z).collect()
        }
    }

    impl FixedPoint<pallas::Affine> for Short {
        type FixedScalarKind = ShortScalar;

        fn generator(&self) -> pallas::Affine {
            *BASE
        }

        fn u(&self) -> Vec<[[u8; 32]; H]> {
            ZS_AND_US_SHORT
                .iter()
                .map(|(_, us)| {
                    [
                        us[0].to_repr(),
                        us[1].to_repr(),
                        us[2].to_repr(),
                        us[3].to_repr(),
                        us[4].to_repr(),
                        us[5].to_repr(),
                        us[6].to_repr(),
                        us[7].to_repr(),
                    ]
                })
                .collect()
        }

        fn z(&self) -> Vec<u64> {
            ZS_AND_US_SHORT.iter().map(|(z, _)| *z).collect()
        }
    }

    impl FixedPoints<pallas::Affine> for TestFixedBases {
        type FullScalar = FullWidth;
        type ShortScalar = Short;
        type Base = BaseField;
    }

    struct MyCircuit {
        test_errors: bool,
    }

    #[allow(non_snake_case)]
    impl Circuit<pallas::Base> for MyCircuit {
        type Config = EccConfig<TestFixedBases>;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            MyCircuit { test_errors: false }
        }

        fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
            let advices = [
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
                meta.advice_column(),
            ];
            let lookup_table = meta.lookup_table_column();
            let table_range_check_tag = meta.lookup_table_column();
            let lagrange_coeffs = [
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
                meta.fixed_column(),
            ];
            // Shared fixed column for loading constants
            let constants = meta.fixed_column();
            meta.enable_constant(constants);

            let range_check = LookupRangeCheckConfigOptimized::configure(
                meta,
                advices[9],
                lookup_table,
                table_range_check_tag,
            );
            EccChip::<TestFixedBases>::configure(meta, advices, lagrange_coeffs, range_check)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<pallas::Base>,
        ) -> Result<(), Error> {
            let chip = EccChip::construct(config.clone());

            // Load 10-bit lookup table. In the Action circuit, this will be
            // provided by the Sinsemilla chip.
            config.lookup_config.load(&mut layouter)?;

            // Generate a random non-identity point P
            let p_val = pallas::Point::random(rand::rngs::OsRng).to_affine(); // P
            let p = super::NonIdentityPoint::new(
                chip.clone(),
                layouter.namespace(|| "P"),
                Value::known(p_val),
            )?;
            let p_neg = -p_val;
            let p_neg = super::NonIdentityPoint::new(
                chip.clone(),
                layouter.namespace(|| "-P"),
                Value::known(p_neg),
            )?;

            // Generate a random non-identity point Q
            let q_val = pallas::Point::random(rand::rngs::OsRng).to_affine(); // Q
            let q = super::NonIdentityPoint::new(
                chip.clone(),
                layouter.namespace(|| "Q"),
                Value::known(q_val),
            )?;

            // Make sure P and Q are not the same point.
            assert_ne!(p_val, q_val);

            // Test that we can witness the identity as a point, but not as a non-identity point.
            {
                let _ = super::Point::new(
                    chip.clone(),
                    layouter.namespace(|| "identity"),
                    Value::known(pallas::Affine::identity()),
                )?;

                super::NonIdentityPoint::new(
                    chip.clone(),
                    layouter.namespace(|| "identity"),
                    Value::known(pallas::Affine::identity()),
                )
                    .expect_err("Trying to witness the identity should return an error");
            }

            // Test witness non-identity point
            {
                super::chip::witness_point::tests::test_witness_non_id(
                    chip.clone(),
                    layouter.namespace(|| "witness non-identity point"),
                )
            }

            // Test complete addition
            {
                super::chip::add::tests::test_add(
                    chip.clone(),
                    layouter.namespace(|| "complete addition"),
                    p_val,
                    &p,
                    q_val,
                    &q,
                    &p_neg,
                )?;
            }

            // Test incomplete addition
            {
                super::chip::add_incomplete::tests::test_add_incomplete(
                    chip.clone(),
                    layouter.namespace(|| "incomplete addition"),
                    p_val,
                    &p,
                    q_val,
                    &q,
                    &p_neg,
                    self.test_errors,
                )?;
            }

            // Test variable-base scalar multiplication
            {
                super::chip::mul::tests::test_mul(
                    chip.clone(),
                    layouter.namespace(|| "variable-base scalar mul"),
                    &p,
                    p_val,
                )?;
            }

            // Test variable-base sign-scalar multiplication
            {
                super::chip::mul_fixed::short::tests::test_mul_sign(
                    chip.clone(),
                    layouter.namespace(|| "variable-base sign-scalar mul"),
                )?;
            }

            // Test full-width fixed-base scalar multiplication
            {
                super::chip::mul_fixed::full_width::tests::test_mul_fixed(
                    chip.clone(),
                    layouter.namespace(|| "full-width fixed-base scalar mul"),
                )?;
            }

            // Test signed short fixed-base scalar multiplication
            {
                super::chip::mul_fixed::short::tests::test_mul_fixed_short(
                    chip.clone(),
                    layouter.namespace(|| "signed short fixed-base scalar mul"),
                )?;
            }

            // Test fixed-base scalar multiplication with a base field element
            {
                super::chip::mul_fixed::base_field_elem::tests::test_mul_fixed_base_field(
                    chip,
                    layouter.namespace(|| "fixed-base scalar mul with base field element"),
                )?;
            }

            Ok(())
        }
    }

    #[test]
    fn ecc_chip() {
        let k = 13;
        let circuit = MyCircuit { test_errors: true };
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()))
    }

    #[cfg(feature = "test-dev-graph")]
    #[test]
    fn print_ecc_chip() {
        use plotters::prelude::*;

        let root = BitMapBackend::new("ecc-chip-layout.png", (1024, 7680)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Ecc Chip Layout", ("sans-serif", 60)).unwrap();

        let circuit = MyCircuit { test_errors: false };
        halo2_proofs::dev::CircuitLayout::default()
            .render(13, &circuit, &root)
            .unwrap();
    }
}