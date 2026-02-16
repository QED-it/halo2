/* Halo2 Example: Prove knowledge of a point (x, y) that is NOT equal to reference point (x0, y0)
 *
 * This circuit demonstrates proving that a witness point (x, y) is different from
 * a public reference point (x0, y0). The inequality is proven by showing that
 * either x != x0 OR y != y0 (or both).
 *
 * We prove this by computing: (x - x0) * (y - y0) and showing the result is non-zero,
 * which guarantees at least one coordinate differs.
 */
use std::marker::PhantomData;

use group::ff::Field;
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Instance, Selector},
    poly::Rotation,
};

#[derive(Clone)]
struct Number<F: Field>(AssignedCell<F, F>);

trait FieldInstructions<F: Field>: Chip<F> {
    type Num;
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;
    fn load_constant(&self, layouter: impl Layouter<F>, a: F) -> Result<Self::Num, Error>;
}

trait AddInstructions<F: Field>: Chip<F> {
    type Num;
    fn add(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
    fn sub(
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
    nonzero_config: NonZeroConfig,
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

#[derive(Clone, Debug)]
struct NonZeroConfig {
    advice: [Column<Advice>; 2],
    s_nonzero: Selector,
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

struct NonZeroChip<F: Field> {
    config: NonZeroConfig,
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

impl<F: Field> Chip<F> for NonZeroChip<F> {
    type Config = NonZeroConfig;
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
        let nonzero_config = NonZeroChip::configure(meta, advice);
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        FieldConfig {
            advice,
            instance,
            add_config,
            mul_config,
            nonzero_config,
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

impl<F: Field> NonZeroChip<F> {
    fn construct(config: NonZeroConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>, advice: [Column<Advice>; 2]) -> NonZeroConfig {
        let s_nonzero = meta.selector();
        meta.create_gate("nonzero", |meta| {
            let value = meta.query_advice(advice[0], Rotation::cur());
            let inverse = meta.query_advice(advice[1], Rotation::cur());
            let s_nonzero = meta.query_selector(s_nonzero);
            // Constraint: value * inverse = 1 (proves value != 0)
            vec![s_nonzero * (value * inverse - Expression::Constant(F::ONE))]
        });
        NonZeroConfig { advice, s_nonzero }
    }

    fn constrain_nonzero(
        &self,
        mut layouter: impl Layouter<F>,
        value: Number<F>,
    ) -> Result<(), Error> {
        let config = self.config();
        layouter.assign_region(
            || "constrain nonzero",
            |mut region: Region<'_, F>| {
                config.s_nonzero.enable(&mut region, 0)?;
                value
                    .0
                    .copy_advice(|| "value", &mut region, config.advice[0], 0)?;
                // Compute and assign inverse
                // For zero values, this will assign F::ZERO which will fail the constraint
                let inverse = value.0.value().map(|&v| {
                    if v == F::ZERO {
                        F::ZERO
                    } else {
                        v.invert().unwrap()
                    }
                });
                region.assign_advice(|| "inverse", config.advice[1], 0, || inverse)?;
                Ok(())
            },
        )
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

    fn sub(
        &self,
        mut layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(
            || "sub",
            |mut region: Region<'_, F>| {
                config.s_add.enable(&mut region, 0)?;
                a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
                let value = a.0.value().copied() - b.0.value();
                region
                    .assign_advice(|| "lhs - rhs", config.advice[0], 1, || value)
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
    fn sub(
        &self,
        layouter: impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        let add_chip = AddChip::construct(self.config().add_config.clone());
        add_chip.sub(layouter, a, b)
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

    fn load_constant(
        &self,
        mut layouter: impl Layouter<F>,
        constant: F,
    ) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(
            || "load constant",
            |mut region| {
                region
                    .assign_advice(
                        || "constant",
                        config.advice[0],
                        0,
                        || Value::known(constant),
                    )
                    .map(Number)
            },
        )
    }
}

/// Circuit that proves knowledge of a point (x, y) different from reference point (x0, y0)
#[derive(Default)]
struct PointInequalityCircuit<F: Field> {
    // Witness: the point we know that is different from (x0, y0)
    witness_x: Value<F>,
    witness_y: Value<F>,
    // Reference point (x0, y0) - provided as witness but will be public
    ref_x0: Value<F>,
    ref_y0: Value<F>,
}

impl<F: Field> Circuit<F> for PointInequalityCircuit<F> {
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

        // Load witness point (x, y) - private
        let witness_x =
            field_chip.load_private(layouter.namespace(|| "load witness x"), self.witness_x)?;
        let witness_y =
            field_chip.load_private(layouter.namespace(|| "load witness y"), self.witness_y)?;

        // Load reference point (x0, y0) - will be made public
        let ref_x0 = field_chip.load_private(layouter.namespace(|| "load ref x0"), self.ref_x0)?;
        let ref_y0 = field_chip.load_private(layouter.namespace(|| "load ref y0"), self.ref_y0)?;

        // Make reference point public (instance columns 0 and 1)
        layouter.constrain_instance(ref_x0.0.cell(), config.instance, 0)?;
        layouter.constrain_instance(ref_y0.0.cell(), config.instance, 1)?;

        // Compute difference: dx = x - x0
        let dx = field_chip.sub(layouter.namespace(|| "x - x0"), witness_x, ref_x0)?;

        // Compute difference: dy = y - y0
        let dy = field_chip.sub(layouter.namespace(|| "y - y0"), witness_y, ref_y0)?;

        // Constrain that at least one of dx or dy is non-zero by proving that the sum of their squares is non-zero
        let dx_squared = field_chip.mul(layouter.namespace(|| "dx^2"), dx.clone(), dx)?;
        let dy_squared = field_chip.mul(layouter.namespace(|| "dy^2"), dy.clone(), dy)?;
        let sum_squares =
            field_chip.add(layouter.namespace(|| "dx^2 + dy^2"), dx_squared, dy_squared)?;

        // Constrain that sum_squares is non-zero
        let nonzero_chip = NonZeroChip::construct(config.nonzero_config);
        nonzero_chip.constrain_nonzero(layouter.namespace(|| "sum_squares != 0"), sum_squares)?;

        Ok(())
    }
}

fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    let k = 8;

    // Reference point (x0, y0) = (5, 7) - the point we want to prove we're NOT equal to
    let ref_x0 = Fp::from(5);
    let ref_y0 = Fp::from(7);

    // Witness point (x, y) = (10, 12) - a different point
    let witness_x = Fp::from(10);
    let witness_y = Fp::from(12);

    let circuit = PointInequalityCircuit {
        witness_x: Value::known(witness_x),
        witness_y: Value::known(witness_y),
        ref_x0: Value::known(ref_x0),
        ref_y0: Value::known(ref_y0),
    };

    // Public inputs: [x0, y0] - only the reference point
    let public_inputs = vec![vec![ref_x0, ref_y0]];

    let prover = MockProver::run(k, &circuit, public_inputs.clone()).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    println!("PointInequalityCircuit verification passed!");
    println!(
        "Proved knowledge of point ({:?}, {:?}) != reference point ({:?}, {:?})",
        witness_x, witness_y, ref_x0, ref_y0
    );

    // Test that equal points are rejected
    println!("\nTesting that equal points are rejected...");
    let bad_circuit = PointInequalityCircuit {
        witness_x: Value::known(ref_x0),
        witness_y: Value::known(ref_y0),
        ref_x0: Value::known(ref_x0),
        ref_y0: Value::known(ref_y0),
    };

    let bad_prover = MockProver::run(k, &bad_circuit, public_inputs).unwrap();
    match bad_prover.verify() {
        Ok(()) => println!("ERROR: Circuit accepted equal points!"),
        Err(_) => println!("SUCCESS: Circuit correctly rejected equal points!"),
    }
}
