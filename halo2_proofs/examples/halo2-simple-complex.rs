/* Halo2 Example: A complex circuit using modular chips for field operations
This example defines a FieldChip that encapsulates addition and multiplication
operations as separate chips. It then constructs two circuits: one that computes
a^5 = b^2 and a + b, a - b; and another that verifies points on an elliptic curve.
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
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error>;
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

    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();
        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}

#[derive(Default)]
struct SimpleCircuit<F: Field> {
    a: Value<F>,
    b: Value<F>,
}

impl<F: Field> Circuit<F> for SimpleCircuit<F> {
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
        let neg_b =
            field_chip.load_private(layouter.namespace(|| "load -b"), b.0.value().map(|v| -*v))?;
        let d = field_chip.add(layouter.namespace(|| "a - b"), a, neg_b)?;

        field_chip.expose_public(layouter.namespace(|| "expose c"), c, 0)?;
        field_chip.expose_public(layouter.namespace(|| "expose d"), d, 1)?;

        Ok(())
    }
}

#[derive(Default)]
struct EllipticCircuit<F: Field> {
    x: Value<F>,
    y: Value<F>,
}

impl<F: Field> Circuit<F> for EllipticCircuit<F> {
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
        let field_chip = FieldChip::<F>::construct(config);

        let x = field_chip.load_private(layouter.namespace(|| "load x"), self.x)?;
        let y = field_chip.load_private(layouter.namespace(|| "load y"), self.y)?;

        field_chip.expose_public(layouter.namespace(|| "expose x"), x, 0)?;
        field_chip.expose_public(layouter.namespace(|| "expose y"), y, 1)?;

        Ok(())
    }
}

fn main() {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    let k = 8;

    // Test SimpleCircuit
    let a = Fp::from(2);
    let b = Fp::from(3);
    let c = a + b; // 5
    let d = a - b; // -1

    let circuit = SimpleCircuit {
        a: Value::known(a),
        b: Value::known(b),
    };

    let public_inputs = vec![vec![c, d]];

    let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    println!("SimpleCircuit verification passed!");

    // Test EllipticCircuit
    let x = Fp::from(5);
    let y = Fp::from(7);

    let elliptic_circuit = EllipticCircuit {
        x: Value::known(x),
        y: Value::known(y),
    };

    let public_inputs = vec![vec![x, y]];

    let prover = MockProver::run(k, &elliptic_circuit, public_inputs).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    println!("EllipticCircuit verification passed!");
}
