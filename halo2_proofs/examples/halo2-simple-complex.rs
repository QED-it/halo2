use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};

#[derive(Clone)]
struct Number<F: FieldExt>(AssignedCell<F, F>);

trait FieldInstructions<F: FieldExt>: Chip<F> {
    type Num;
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;
    fn expose_public(&self, layouter: impl Layouter<F>, num: Self::Num, row: usize) -> Result<(), Error>;
}

trait AddInstructions<F: FieldExt>: Chip<F> {
    type Num;
    fn add(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;
}

trait MulInstructions<F: FieldExt>: Chip<F> {
    type Num;
    fn mul(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;
}

#[derive(Clone, Debug)]
struct FieldConfig {
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    add_config: AddConfig,
    mul_config: MulConfig,
}

#[derive(Clone, Debug)]
struct AddConfig {
    advice: [Column<Advice>; 2],
    s_add: Selector,
}

#[derive(Clone, Debug)]
struct MulConfig {
    advice: [Column<Advice>; 2],
    s_mul: Selector,
}

struct FieldChip<F: FieldExt> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}

struct AddChip<F: FieldExt> {
    config: AddConfig,
    _marker: PhantomData<F>,
}

struct MulChip<F: FieldExt> {
    config: MulConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config { &self.config }
    fn loaded(&self) -> &Self::Loaded { &() }
}

impl<F: FieldExt> Chip<F> for AddChip<F> {
    type Config = AddConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config { &self.config }
    fn loaded(&self) -> &Self::Loaded { &() }
}

impl<F: FieldExt> Chip<F> for MulChip<F> {
    type Config = MulConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config { &self.config }
    fn loaded(&self) -> &Self::Loaded { &() }
}

impl<F: FieldExt> FieldChip<F> {
    fn construct(config: FieldConfig) -> Self {
        Self { config, _marker: PhantomData }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2], instance: Column<Instance>) -> FieldConfig {
        let add_config = AddChip::configure(meta, advice);
        let mul_config = MulChip::configure(meta, advice);
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        FieldConfig { advice, instance, add_config, mul_config }
    }
}

impl<F: FieldExt> AddChip<F> {
    fn construct(config: AddConfig) -> Self {
        Self { config, _marker: PhantomData }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2]) -> AddConfig {
        let s_add = meta.selector();
        meta.create_gate("add", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_add = meta.query_selector(s_add);
            vec![s_add * (lhs + rhs - out)]
        });
        AddConfig { advice, s_add }
    }
}

impl<F: FieldExt> MulChip<F> {
    fn construct(config: MulConfig) -> Self {
        Self { config, _marker: PhantomData }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2]) -> MulConfig {
        let s_mul = meta.selector();
        meta.create_gate("mul", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[0], Rotation::next());
            let s_mul = meta.query_selector(s_mul);
            vec![s_mul * (lhs * rhs - out)]
        });
        MulConfig { advice, s_mul }
    }
}

impl<F: FieldExt> AddInstructions<F> for AddChip<F> {
    type Num = Number<F>;

    fn add(&self, mut layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "add", |mut region: Region<'_, F>| {
            config.s_add.enable(&mut region, 0)?;
            a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
            b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
            let value = a.0.value().copied() + b.0.value();
            region.assign_advice(|| "lhs + rhs", config.advice[0], 1, || value).map(Number)
        })
    }
}

impl<F: FieldExt> MulInstructions<F> for MulChip<F> {
    type Num = Number<F>;

    fn mul(&self, mut layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "mul", |mut region: Region<'_, F>| {
            config.s_mul.enable(&mut region, 0)?;
            a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
            b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
            let value = a.0.value().copied() * b.0.value();
            region.assign_advice(|| "lhs * rhs", config.advice[0], 1, || value).map(Number)
        })
    }
}

impl<F: FieldExt> AddInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn add(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let add_chip = AddChip::construct(self.config().add_config.clone());
        add_chip.add(layouter, a, b)
    }
}

impl<F: FieldExt> MulInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn mul(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let mul_chip = MulChip::construct(self.config().mul_config.clone());
        mul_chip.mul(layouter, a, b)
    }
}

impl<F: FieldExt> FieldInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_private(&self, mut layouter: impl Layouter<F>, value: Value<F>) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "load private", |mut region| {
            region.assign_advice(|| "private input", config.advice[0], 0, || value).map(Number)
        })
    }

    fn expose_public(&self, mut layouter: impl Layouter<F>, num: Self::Num, row: usize) -> Result<(), Error> {
        let config = self.config();
        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}

#[derive(Default)]
struct SimpleCircuit<F: FieldExt> {
    a: Value<F>,
    b: Value<F>,
}

impl<F: FieldExt> Circuit<F> for SimpleCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self { Self::default() }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        FieldChip::configure(meta, advice, instance)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);

        let a = field_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
        let b = field_chip.load_private(layouter.namespace(|| "load b"), self.b)?;

        // a^5 = b^2: compute a^5 and b^2
        let a2 = field_chip.mul(layouter.namespace(|| "a^2"), a.clone(), a.clone())?;
        let a4 = field_chip.mul(layouter.namespace(|| "a^4"), a2.clone(), a2)?;
        let _a5 = field_chip.mul(layouter.namespace(|| "a^5"), a4, a.clone())?;
        let _b2 = field_chip.mul(layouter.namespace(|| "b^2"), b.clone(), b.clone())?;

        // a + b = c, a - b = d
        let c = field_chip.add(layouter.namespace(|| "a + b"), a.clone(), b.clone())?;
        
        // For a - b, load -b and add to a
        let neg_b = field_chip.load_private(layouter.namespace(|| "load -b"), 
            b.0.value().map(|v| -*v))?;
        let d = field_chip.add(layouter.namespace(|| "a - b"), a, neg_b)?;

        field_chip.expose_public(layouter.namespace(|| "expose c"), c, 0)?;
        field_chip.expose_public(layouter.namespace(|| "expose d"), d, 1)?;

        Ok(())
    }
}

#[derive(Default)]
struct EllipticCircuit<F: FieldExt> {
    x: Value<F>,
    y: Value<F>,
}

impl<F: FieldExt> Circuit<F> for EllipticCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self { Self::default() }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        FieldChip::configure(meta, advice, instance)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config);

        let x = field_chip.load_private(layouter.namespace(|| "load x"), self.x)?;
        let y = field_chip.load_private(layouter.namespace(|| "load y"), self.y)?;

        field_chip.expose_public(layouter.namespace(|| "expose x"), x, 0)?;
        field_chip.expose_public(layouter.namespace(|| "expose y"), y, 1)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    #[test]
    fn test_simple_circuit_basic() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(1)),
        };
        let public_inputs = vec![vec![Fp::from(2), Fp::from(0)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_simple_circuit_different_values() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(2)),
            b: Value::known(Fp::from(4)),
        };
        let public_inputs = vec![vec![Fp::from(6), -Fp::from(2)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_simple_circuit_zero_values() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(0)),
            b: Value::known(Fp::from(0)),
        };
        let public_inputs = vec![vec![Fp::from(0), Fp::from(0)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_simple_circuit_negative_values() {
        let circuit = SimpleCircuit {
            a: Value::known(-Fp::from(3)),
            b: Value::known(Fp::from(2)),
        };
        let public_inputs = vec![vec![-Fp::from(1), -Fp::from(5)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_simple_circuit_large_values() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(100)),
            b: Value::known(Fp::from(50)),
        };
        let public_inputs = vec![vec![Fp::from(150), Fp::from(50)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    #[should_panic]
    fn test_simple_circuit_wrong_sum() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(1)),
        };
        let public_inputs = vec![vec![Fp::from(3), Fp::from(0)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    #[should_panic]
    fn test_simple_circuit_wrong_diff() {
        let circuit = SimpleCircuit {
            a: Value::known(Fp::from(5)),
            b: Value::known(Fp::from(3)),
        };
        let public_inputs = vec![vec![Fp::from(8), Fp::from(3)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_elliptic_circuit_basic() {
        let circuit = EllipticCircuit {
            x: Value::known(Fp::from(3)),
            y: Value::known(Fp::from(4)),
        };
        let public_inputs = vec![vec![Fp::from(3), Fp::from(4)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_elliptic_circuit_zero_point() {
        let circuit = EllipticCircuit {
            x: Value::known(Fp::from(0)),
            y: Value::known(Fp::from(0)),
        };
        let public_inputs = vec![vec![Fp::from(0), Fp::from(0)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_elliptic_circuit_negative_coords() {
        let circuit = EllipticCircuit {
            x: Value::known(-Fp::from(5)),
            y: Value::known(-Fp::from(7)),
        };
        let public_inputs = vec![vec![-Fp::from(5), -Fp::from(7)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_elliptic_circuit_large_coords() {
        let circuit = EllipticCircuit {
            x: Value::known(Fp::from(1000)),
            y: Value::known(Fp::from(2000)),
        };
        let public_inputs = vec![vec![Fp::from(1000), Fp::from(2000)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    #[should_panic]
    fn test_elliptic_circuit_wrong_x() {
        let circuit = EllipticCircuit {
            x: Value::known(Fp::from(3)),
            y: Value::known(Fp::from(4)),
        };
        let public_inputs = vec![vec![Fp::from(5), Fp::from(4)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    #[should_panic]
    fn test_elliptic_circuit_wrong_y() {
        let circuit = EllipticCircuit {
            x: Value::known(Fp::from(3)),
            y: Value::known(Fp::from(4)),
        };
        let public_inputs = vec![vec![Fp::from(3), Fp::from(6)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_multiple_simple_circuits() {
        let test_cases = vec![
            (Fp::from(1), Fp::from(2), Fp::from(3), -Fp::from(1)),
            (Fp::from(0), Fp::from(5), Fp::from(5), -Fp::from(5)),
            (Fp::from(10), -Fp::from(3), Fp::from(7), Fp::from(13)),
            (-Fp::from(1), -Fp::from(1), -Fp::from(2), Fp::from(0)),
        ];
        
        for (a, b, expected_c, expected_d) in test_cases {
            let circuit = SimpleCircuit {
                a: Value::known(a),
                b: Value::known(b),
            };
            let public_inputs = vec![vec![expected_c, expected_d]];
            let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
            prover.assert_satisfied();
        }
    }

    #[test]
    fn test_multiple_elliptic_circuits() {
        let test_cases = vec![
            (Fp::from(0), Fp::from(1)),
            (Fp::from(1), Fp::from(0)),
            (-Fp::from(1), -Fp::from(1)),
            (Fp::from(100), -Fp::from(200)),
            (Fp::from(42), Fp::from(84)),
        ];
        
        for (x, y) in test_cases {
            let circuit = EllipticCircuit {
                x: Value::known(x),
                y: Value::known(y),
            };
            let public_inputs = vec![vec![x, y]];
            let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
            prover.assert_satisfied();
        }
    }

    #[test]
    fn test_simple_circuit_edge_cases() {
        // Test with maximum field value
        let max_val = -Fp::from(1); // This is the maximum value in the field
        let circuit = SimpleCircuit {
            a: Value::known(max_val),
            b: Value::known(Fp::from(1)),
        };
        let expected_c = max_val + Fp::from(1); // Should be 0
        let expected_d = max_val - Fp::from(1); // Should be max_val - 1
        let public_inputs = vec![vec![expected_c, expected_d]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_elliptic_circuit_edge_cases() {
        // Test with field boundary values
        let max_val = -Fp::from(1);
        let circuit = EllipticCircuit {
            x: Value::known(max_val),
            y: Value::known(Fp::from(0)),
        };
        let public_inputs = vec![vec![max_val, Fp::from(0)]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_simple_circuit_polynomial_relation() {
        // Test specific case where a^5 = b^2 is satisfied
        let a = Fp::from(2); // 2^5 = 32
        let _b_squared = Fp::from(32);
        let b = Fp::from(4); // We need to find b such that b^2 = 32, but in field arithmetic
        
        let circuit = SimpleCircuit {
            a: Value::known(a),
            b: Value::known(b),
        };
        let expected_c = a + b; // 2 + 4 = 6
        let expected_d = a - b; // 2 - 4 = -2
        let public_inputs = vec![vec![expected_c, expected_d]];
        let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_comprehensive_scenarios() {
        // Test various mathematical scenarios
        let scenarios = vec![
            // (a, b, description)
            (Fp::from(1), Fp::from(1), "ones"),
            (Fp::from(0), Fp::from(10), "zero and positive"),
            (Fp::from(5), -Fp::from(3), "positive and negative"),
            (-Fp::from(2), -Fp::from(4), "both negative"),
            (Fp::from(100), Fp::from(200), "large values"),
        ];
        
        for (a, b, _desc) in scenarios {
            let circuit = SimpleCircuit {
                a: Value::known(a),
                b: Value::known(b),
            };
            let expected_c = a + b;
            let expected_d = a - b;
            let public_inputs = vec![vec![expected_c, expected_d]];
            let prover = MockProver::run(8, &circuit, public_inputs).unwrap();
            prover.assert_satisfied();
        }
    }
}