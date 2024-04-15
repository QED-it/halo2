
//! Gadget and chip for a conditional swap utility.


use crate::ecc::chip::{EccPoint, NonIdentityEccPoint};
use group::ff::{Field, PrimeField};
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{self, Advice, Column, ConstraintSystem, Constraints, Error, Selector},
    poly::Rotation,
};
use pasta_curves::pallas;
use std::marker::PhantomData;
use crate::utilities::UtilitiesInstructions;

/// Instructions for a conditional swap gadget.



#[cfg(test)]
mod tests {
    use group::ff::{Field, PrimeField};
    use halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner, Value},
        dev::MockProver,
        plonk::{Circuit, ConstraintSystem, Error},
    };
    use pasta_curves::pallas::Base;
    use rand::rngs::OsRng;
    use crate::utilities::cond_swap::{CondSwapChip, CondSwapConfig};
    use crate::utilities::lookup_range_check::LookupRangeCheck;
    use crate::utilities_opt::lookup_range_check::LookupRangeCheckConfigOptimized;


    #[test]
    fn test_mux() {
        use crate::{
            ecc::{
                chip::{EccChip, EccConfig},
                tests::TestFixedBases,
                NonIdentityPoint, Point,
            },
        };

        use group::{cofactor::CofactorCurveAffine, Curve, Group};
        use halo2_proofs::{
            circuit::{Layouter, SimpleFloorPlanner, Value},
            dev::MockProver,
            plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
        };
        use pasta_curves::arithmetic::CurveAffine;
        use pasta_curves::{pallas, EpAffine};

        use rand::rngs::OsRng;

        #[derive(Clone, Debug)]
        pub struct MyConfig {
            primary: Column<Instance>,
            advice: Column<Advice>,
            cond_swap_config: CondSwapConfig,
            ecc_config: EccConfig<TestFixedBases>,
        }

        #[derive(Default)]
        struct MyCircuit {
            left_point: Value<EpAffine>,
            right_point: Value<EpAffine>,
            choice: Value<pallas::Base>,
        }

        impl Circuit<pallas::Base> for MyCircuit {
            type Config = MyConfig;
            type FloorPlanner = SimpleFloorPlanner;

            fn without_witnesses(&self) -> Self {
                Self::default()
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

                for advice in advices.iter() {
                    meta.enable_equality(*advice);
                }

                // Instance column used for public inputs
                let primary = meta.instance_column();
                meta.enable_equality(primary);

                let cond_swap_config =
                    CondSwapChip::configure(meta, advices[0..5].try_into().unwrap());

                let table_idx = meta.lookup_table_column();
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
                meta.enable_constant(lagrange_coeffs[0]);

                let range_check = LookupRangeCheckConfigOptimized::configure(
                    meta,
                    advices[9],
                    table_idx,
                );

                let ecc_config = EccChip::<TestFixedBases>::configure(
                    meta,
                    advices,
                    lagrange_coeffs,
                    range_check,
                );

                MyConfig {
                    primary,
                    advice: advices[0],
                    cond_swap_config,
                    ecc_config,
                }
            }

            fn synthesize(
                &self,
                config: Self::Config,
                mut layouter: impl Layouter<pallas::Base>,
            ) -> Result<(), Error> {
                // Construct a CondSwap chip
                let cond_swap_chip = CondSwapChip::construct(config.cond_swap_config);

                // Construct an ECC chip
                let ecc_chip = EccChip::construct(config.ecc_config);

                // Assign choice
                let choice = layouter.assign_region(
                    || "load private",
                    |mut region| {
                        region.assign_advice(|| "load private", config.advice, 0, || self.choice)
                    },
                )?;

                // Test mux on non identity points
                // Assign left point
                let left_non_identity_point = NonIdentityPoint::new(
                    ecc_chip.clone(),
                    layouter.namespace(|| "left point"),
                    self.left_point.map(|left_point| left_point),
                )?;

                // Assign right point
                let right_non_identity_point = NonIdentityPoint::new(
                    ecc_chip.clone(),
                    layouter.namespace(|| "right point"),
                    self.right_point.map(|right_point| right_point),
                )?;

                // Apply mux
                let result_non_identity_point = cond_swap_chip.mux_on_non_identity_points(
                    layouter.namespace(|| "MUX"),
                    &choice,
                    left_non_identity_point.inner(),
                    right_non_identity_point.inner(),
                )?;

                // Check equality with instance
                layouter.constrain_instance(
                    result_non_identity_point.x().cell(),
                    config.primary,
                    0,
                )?;
                layouter.constrain_instance(
                    result_non_identity_point.y().cell(),
                    config.primary,
                    1,
                )?;

                // Test mux on points
                // Assign left point
                let left_point = Point::new(
                    ecc_chip.clone(),
                    layouter.namespace(|| "left point"),
                    self.left_point.map(|left_point| left_point),
                )?;

                // Assign right point
                let right_point = Point::new(
                    ecc_chip,
                    layouter.namespace(|| "right point"),
                    self.right_point.map(|right_point| right_point),
                )?;

                // Apply mux
                let result = cond_swap_chip.mux_on_points(
                    layouter.namespace(|| "MUX"),
                    &choice,
                    left_point.inner(),
                    right_point.inner(),
                )?;

                // Check equality with instance
                layouter.constrain_instance(result.x().cell(), config.primary, 0)?;
                layouter.constrain_instance(result.y().cell(), config.primary, 1)
            }
        }

        // Test different circuits
        let mut circuits = vec![];
        let mut instances = vec![];
        for choice in [false, true] {
            let choice_value = if choice {
                pallas::Base::one()
            } else {
                pallas::Base::zero()
            };
            let left_point = pallas::Point::random(OsRng).to_affine();
            let right_point = pallas::Point::random(OsRng).to_affine();
            circuits.push(MyCircuit {
                left_point: Value::known(left_point),
                right_point: Value::known(right_point),
                choice: Value::known(choice_value),
            });
            let expected_output = if choice { right_point } else { left_point };
            let (expected_x, expected_y) = if bool::from(expected_output.is_identity()) {
                (pallas::Base::zero(), pallas::Base::zero())
            } else {
                let coords = expected_output.coordinates().unwrap();
                (*coords.x(), *coords.y())
            };
            instances.push([[expected_x, expected_y]]);
        }

        for (circuit, instance) in circuits.iter().zip(instances.iter()) {
            let prover = MockProver::<pallas::Base>::run(
                5,
                circuit,
                instance.iter().map(|p| p.to_vec()).collect(),
            )
                .unwrap();
            assert_eq!(prover.verify(), Ok(()));
        }
    }
}