/* Halo2 Example: Polynomial Equality Circuit
 *
 * This circuit proves knowledge of two field elements (a, b) such that:
 * 1. a^5 = b^2 (polynomial equality constraint)
 * 2. Computes sum = a + b and diff = a - b as public outputs
 *
 * The circuit demonstrates:
 * - Modular chip design with separate AddChip and MulChip
 * - Copy constraints to enforce equality between computed values
 * - Public instance columns for exposing outputs
 */
use std::marker::PhantomData;

use group::ff::Field;
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};

#[derive(Clone)]
struct Number<F: Field>(AssignedCell<F, F>);

trait FieldInstructions<F: Field>: Chip<F> {
    type Num;
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;
}

trait AddInstructions<F: Field>: Chip<F> {
    type Num;
    fn add(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
}

trait MulInstructions<F: Field>: Chip<F> {
    type Num;
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
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

struct FieldChip<F: Field> {
    config: FieldConfig,
    _marker: PhantomData<F>,
}

struct AddChip<F: Field> {
    config: AddConfig,
    _marker: PhantomData<F>,
}

struct MulChip<F: Field> {
    config: MulConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> Chip<F> for FieldChip<F> {
    type Config = FieldConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config {
        &self.config
    }
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> Chip<F> for AddChip<F> {
    type Config = AddConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config {
        &self.config
    }
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> Chip<F> for MulChip<F> {
    type Config = MulConfig;
    type Loaded = ();
    fn config(&self) -> &Self::Config {
        &self.config
    }
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<F: Field> FieldChip<F> {
    fn construct(config: FieldConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 2],
        instance: Column<Instance>,
    ) -> FieldConfig {
        let add_config = AddChip::configure(meta, advice);
        let mul_config = MulChip::configure(meta, advice);
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        FieldConfig {
            advice,
            instance,
            add_config,
            mul_config,
        }
    }
}

impl<F: Field> AddChip<F> {
    fn construct(config: AddConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
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

impl<F: Field> MulChip<F> {
    fn construct(config: MulConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
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

impl<F: Field> AddInstructions<F> for AddChip<F> {
    type Num = Number<F>;

    fn add(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(
            || "add",
            |mut region: Region<'_, F>| {
                config.s_add.enable(&mut region, 0)?;
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
                let value = a.0.value().copied() + b.0.value();
                region
                    .assign_advice(|| "lhs + rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}

impl<F: Field> MulInstructions<F> for MulChip<F> {
    type Num = Number<F>;

    fn mul(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(
            || "mul",
            |mut region: Region<'_, F>| {
                config.s_mul.enable(&mut region, 0)?;
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
                let value = a.0.value().copied() * b.0.value();
                region
                    .assign_advice(|| "lhs * rhs", config.advice[0], 1, || value)
                    .map(Number)
            },
        )
    }
}

impl<F: Field> AddInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn add(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let add_chip = AddChip::construct(self.config().add_config.clone());
        add_chip.add(layouter, a, b)
    }
}

impl<F: Field> MulInstructions<F> for FieldChip<F> {
    type Num = Number<F>;
    fn mul(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let mul_chip = MulChip::construct(self.config().mul_config.clone());
        mul_chip.mul(layouter, a, b)
    }
}

impl<F: Field> FieldInstructions<F> for FieldChip<F> {
    type Num = Number<F>;

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<F>,
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(
            || "load private",
            |mut region| {
                region
                    .assign_advice(|| "private input", config.advice[0], 0, || value)
                    .map(Number)
            },
        )
    }
}

/// Circuit that proves a^5 = b^2 and computes a+b, a-b
#[derive(Default)]
struct PolynomialEqualityCircuit<F: Field> {
    // Private witness: first field element
    witness_a: Value<F>,
    // Private witness: second field element satisfying a^5 = b^2
    witness_b: Value<F>,
}

impl<F: Field> Circuit<F> for PolynomialEqualityCircuit<F> {
    type Config = FieldConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        FieldChip::configure(meta, advice, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let field_chip = FieldChip::<F>::construct(config.clone());

        // Load private witnesses a and b
        let witness_a =
            field_chip.load_private(layouter.namespace(|| "load witness a"), self.witness_a)?;
        let witness_b =
            field_chip.load_private(layouter.namespace(|| "load witness b"), self.witness_b)?;

        // Compute a^5: a -> a^2 -> a^4 -> a^5
        let a_squared = field_chip.mul(
            layouter.namespace(|| "a^2"),
            witness_a.clone(),
            witness_a.clone(),
        )?;
        let a_fourth =
            field_chip.mul(layouter.namespace(|| "a^4"), a_squared.clone(), a_squared)?;
        let a_fifth = field_chip.mul(layouter.namespace(|| "a^5"), a_fourth, witness_a.clone())?;

        // Compute b^2
        let b_squared = field_chip.mul(
            layouter.namespace(|| "b^2"),
            witness_b.clone(),
            witness_b.clone(),
        )?;

        // Constrain a^5 = b^2 using copy constraint (both cells must have same value)
        layouter.assign_region(
            || "constrain a^5 = b^2",
            |mut region| {
                a_fifth
                    .0
                    .copy_advice(|| "a^5", &mut region, config.advice[0], 0)?;
                b_squared
                    .0
                    .copy_advice(|| "b^2", &mut region, config.advice[0], 0)?;
                Ok(())
            },
        )?;

        // Compute sum = a + b
        let sum = field_chip.add(layouter.namespace(|| "a + b"), witness_a.clone(), witness_b)?;

        // Compute diff = a - b (by loading -b and adding)
        let neg_b = field_chip
            .load_private(layouter.namespace(|| "load -b"), self.witness_b.map(|v| -v))?;
        let diff = field_chip.add(layouter.namespace(|| "a - b"), witness_a, neg_b)?;

        // Expose sum and diff as public outputs (instance columns 0 and 1)
        layouter.constrain_instance(sum.0.cell(), field_chip.config().instance, 0)?;
        layouter.constrain_instance(diff.0.cell(), field_chip.config().instance, 1)?;

        Ok(())
    }
}

fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    let k = 8;

    // Choose values where a^5 = b^2 holds: a=4, b=32
    // Verification: 4^5 = 1024, 32^2 = 1024 ✓
    let witness_a = Fp::from(4);
    let witness_b = Fp::from(32);

    // Compute expected public outputs
    let expected_sum = witness_a + witness_b; // 4 + 32 = 36
    let expected_diff = witness_a - witness_b; // 4 - 32 = -28

    let circuit = PolynomialEqualityCircuit {
        witness_a: Value::known(witness_a),
        witness_b: Value::known(witness_b),
    };

    // Public inputs: [sum, diff]
    let public_inputs = vec![vec![expected_sum, expected_diff]];

    let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    println!("PolynomialEqualityCircuit verification passed!");
    println!(
        "Proved: a^5 = b^2 where a={:?}, b={:?}",
        witness_a, witness_b
    );
    println!(
        "Public outputs: sum={:?}, diff={:?}",
        expected_sum, expected_diff
    );
}
