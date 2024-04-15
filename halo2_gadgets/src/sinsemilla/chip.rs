//! Chip implementations for the Sinsemilla gadgets.

use super::{
    message::{Message, MessagePiece},
    primitives as sinsemilla, CommitDomains, HashDomains, SinsemillaInstructions,
};
use crate::{
    ecc::{
        chip::{DoubleAndAdd, NonIdentityEccPoint},
        FixedPoints,
    },
    utilities::lookup_range_check::LookupRangeCheckConfig,
};
use std::marker::PhantomData;
use ff::PrimeField;
use pasta_curves::arithmetic::CurveAffine;

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{
        Advice, Column, ConstraintSystem, Constraints, Error, Expression, Fixed, Selector,
        TableColumn, VirtualCells,
    },
    poly::Rotation,
};
use pasta_curves::pallas;
use pasta_curves::pallas::Base;
use proptest::test_runner::Config;

mod generator_table;
use generator_table::GeneratorTableConfig;
use halo2_proofs::circuit::Region;
use halo2_proofs::plonk::Assigned;
use crate::sinsemilla::primitives::{INV_TWO_POW_K, lebs2ip_k, SINSEMILLA_S};

mod hash_to_point;

/// Configuration for the Sinsemilla hash chip
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SinsemillaConfigCommon<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    /// Binary selector used in lookup argument and in the body of the Sinsemilla hash.
    pub(crate) q_sinsemilla1: Selector,
    /// Non-binary selector used in lookup argument and in the body of the Sinsemilla hash.
    pub(crate) q_sinsemilla2: Column<Fixed>,
    /// q_sinsemilla2 is used to define a synthetic selector,
    ///         q_sinsemilla3 = (q_sinsemilla2) ⋅ (q_sinsemilla2 - 1)
    /// Simple selector used to constrain hash initialization to be consistent with
    /// the y-coordinate of the domain $Q$.
    pub(crate) q_sinsemilla4: Selector,
    /// Fixed column used to load the y-coordinate of the domain $Q$.
    pub(crate) fixed_y_q: Column<Fixed>,
    /// Logic specific to merged double-and-add.
    pub(crate) double_and_add: DoubleAndAdd,
    /// Advice column used to load the message.
    pub(crate) bits: Column<Advice>,
    /// Advice column used to witness message pieces. This may or may not be the same
    /// column as `bits`.
    pub(crate) witness_pieces: Column<Advice>,
    _marker: PhantomData<(Hash, Commit, F)>,
}
/// Configuration for the Sinsemilla hash chip
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SinsemillaConfig<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    base: SinsemillaConfigCommon<Hash, Commit, F>,
    /// The lookup table where $(\mathsf{idx}, x_p, y_p)$ are loaded for the $2^K$
    /// generators of the Sinsemilla hash.
    pub(super) generator_table: GeneratorTableConfig,
    /// An advice column configured to perform lookup range checks.
    lookup_config: LookupRangeCheckConfig<pallas::Base, { sinsemilla::K }>,
}
// TODO: add doc, rename it to SinsemillaConfigProps
pub trait SinsemillaConfigProps<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type LookupConfigType;
    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F>;

    /// Returns an array of all advice columns in this config, in arbitrary order.
    fn advices(&self) -> [Column<Advice>; 5] {
        [
            self.base().double_and_add.x_a,
            self.base().double_and_add.x_p,
            self.base().bits,
            self.base().double_and_add.lambda_1,
            self.base().double_and_add.lambda_2,
        ]
    }

    /// Returns the lookup range check config used in this config.
    fn lookup_config(&self) -> Self::LookupConfigType;

    /// Derives the expression `q_s3 = (q_s2) * (q_s2 - 1)`.
    fn q_s3(&self, meta: &mut VirtualCells<pallas::Base>) -> Expression<pallas::Base> {
        let one = Expression::Constant(pallas::Base::one());
        let q_s2 = meta.query_fixed(self.base().q_sinsemilla2);
        q_s2.clone() * (q_s2 - one)
    }

    fn get_y_q(&self, meta: &mut VirtualCells<pallas::Base>) -> Expression<pallas::Base>;

    fn configure_from_y_q(
        &self,
        meta: &mut ConstraintSystem<pallas::Base>,
    ) where
    {
        let two = pallas::Base::from(2);

        // Closures for expressions that are derived multiple times
        // x_r = lambda_1^2 - x_a - x_p
        let x_r = |meta: &mut VirtualCells<pallas::Base>, rotation| {
            self.base().double_and_add.x_r(meta, rotation)
        };

        // Y_A = (lambda_1 + lambda_2) * (x_a - x_r)
        let Y_A = |meta: &mut VirtualCells<pallas::Base>, rotation| {
            self.base().double_and_add.Y_A(meta, rotation)
        };

        // Check that the initial x_A, x_P, lambda_1, lambda_2 are consistent with y_Q.
        // https://p.z.cash/halo2-0.1:sinsemilla-constraints?partial
        meta.create_gate("Initial y_Q", |meta| {
            let q_s4 = meta.query_selector(self.base().q_sinsemilla4);
            let y_q = self.get_y_q(meta); // We use the closure we passed to get the y_q.

            // Y_A = (lambda_1 + lambda_2) * (x_a - x_r)
            let Y_A_cur = Y_A(meta, Rotation::cur());

            // 2 * y_q - Y_{A,0} = 0
            let init_y_q_check = y_q * two - Y_A_cur;

            Constraints::with_selector(q_s4, Some(("init_y_q_check", init_y_q_check)))
        });

        // https://p.z.cash/halo2-0.1:sinsemilla-constraints?partial
        meta.create_gate("Sinsemilla gate", |meta| {
            let q_s1 = meta.query_selector(self.base().q_sinsemilla1);
            let q_s3 = self.q_s3(meta);

            let lambda_1_next = meta.query_advice(self.base().double_and_add.lambda_1, Rotation::next());
            let lambda_2_cur = meta.query_advice(self.base().double_and_add.lambda_2, Rotation::cur());
            let x_a_cur = meta.query_advice(self.base().double_and_add.x_a, Rotation::cur());
            let x_a_next = meta.query_advice(self.base().double_and_add.x_a, Rotation::next());

            // x_r = lambda_1^2 - x_a_cur - x_p
            let x_r = x_r(meta, Rotation::cur());

            // Y_A = (lambda_1 + lambda_2) * (x_a - x_r)
            let Y_A_cur = Y_A(meta, Rotation::cur());

            // Y_A = (lambda_1 + lambda_2) * (x_a - x_r)
            let Y_A_next = Y_A(meta, Rotation::next());

            // lambda2^2 - (x_a_next + x_r + x_a_cur) = 0
            let secant_line =
                lambda_2_cur.clone().square() - (x_a_next.clone() + x_r + x_a_cur.clone());

            // lhs - rhs = 0, where
            //    - lhs = 4 * lambda_2_cur * (x_a_cur - x_a_next)
            //    - rhs = (2 * Y_A_cur + (2 - q_s3) * Y_A_next + 2 * q_s3 * y_a_final)
            let y_check = {
                // lhs = 4 * lambda_2_cur * (x_a_cur - x_a_next)
                let lhs = lambda_2_cur * pallas::Base::from(4) * (x_a_cur - x_a_next);

                // rhs = 2 * Y_A_cur + (2 - q_s3) * Y_A_next + 2 * q_s3 * y_a_final
                let rhs = {
                    // y_a_final is assigned to the lambda1 column on the next offset.
                    let y_a_final = lambda_1_next;

                    Y_A_cur * two
                        + (Expression::Constant(two) - q_s3.clone()) * Y_A_next
                        + q_s3 * two * y_a_final
                };
                lhs - rhs
            };

            Constraints::with_selector(q_s1, [("Secant line", secant_line), ("y check", y_check)])
        });
    }


}
// TODO: add doc
impl<Hash, Commit, F> SinsemillaConfigProps<Hash, Commit, F> for SinsemillaConfig<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type LookupConfigType = LookupRangeCheckConfig<pallas::Base, { sinsemilla::K }>;
    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F> {
        &self.base
    }

    fn lookup_config(&self) -> Self::LookupConfigType {
        self.lookup_config
    }

    // todo: add doc
    fn get_y_q(&self, meta: &mut VirtualCells<Base>) -> Expression<Base> {
        meta.query_fixed(self.base.fixed_y_q)
    }
}


/// A chip that implements 10-bit Sinsemilla using a lookup table and 5 advice columns.
///
/// [Chip description](https://zcash.github.io/halo2/design/gadgets/sinsemilla.html#plonk--halo-2-constraints).
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SinsemillaChip<Hash, Commit, Fixed>
    where
        Hash: HashDomains<pallas::Affine>,
        Fixed: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    config: SinsemillaConfig<Hash, Commit, Fixed>,
}

// TODO: add doc,rename it to SinsemillaChipProps
pub trait SinsemillaChipProps<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type Loaded;

    type RangeCheckConfigType;
    type SinsemillaConfigType: SinsemillaConfigProps<Hash, Commit, F>;
    type LookupType;

    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F>;

    /// Reconstructs this chip from the given config.
    fn construct(config: Self::SinsemillaConfigType) -> Self;

    /// Loads the lookup table required by this chip into the circuit.
    fn load(
        config: Self::SinsemillaConfigType,
        layouter: &mut impl Layouter<pallas::Base>,
    ) -> Result<Self::Loaded, Error>;
    fn configure(
        meta: &mut ConstraintSystem<pallas::Base>,
        advices: [Column<Advice>; 5],
        witness_pieces: Column<Advice>,
        fixed_y_q: Column<Fixed>,
        lookup: Self::LookupType,
        range_check: Self::RangeCheckConfigType,
    ) -> Self::SinsemillaConfigType;

}

impl<Hash, Commit, F> SinsemillaChipProps<Hash, Commit, F> for SinsemillaChip<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type Loaded = <Self as Chip<pallas::Base>>::Loaded;

    type RangeCheckConfigType = LookupRangeCheckConfig<pallas::Base, { sinsemilla::K }>;

    type SinsemillaConfigType = SinsemillaConfig<Hash, Commit, F>;

    type LookupType = (TableColumn, TableColumn, TableColumn);

    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F> {
        &self.config.base
    }

    /// Reconstructs this chip from the given config.
    fn construct(config: Self::SinsemillaConfigType) -> Self {
        Self { config }
    }

    /// Loads the lookup table required by this chip into the circuit.
    fn load(
        config: Self::SinsemillaConfigType,
        layouter: &mut impl Layouter<pallas::Base>,
    ) -> Result<<Self as Chip<pallas::Base>>::Loaded, Error> {
        // Load the lookup table.
        config.generator_table.load(layouter)
    }

    /// # Side-effects
    ///
    /// All columns in `advices` and will be equality-enabled.
    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    fn configure(
        meta: &mut ConstraintSystem<pallas::Base>,
        advices: [Column<Advice>; 5],
        witness_pieces: Column<Advice>,
        fixed_y_q: Column<Fixed>,
        lookup: Self::LookupType,
        range_check: Self::RangeCheckConfigType,
    ) -> Self::SinsemillaConfigType {
        // Enable equality on all advice columns
        for advice in advices.iter() {
            meta.enable_equality(*advice);
        }

        let base_config = create_common_config(meta, advices, witness_pieces, fixed_y_q);
        let config = SinsemillaConfig::<Hash, Commit, F> {
            base: base_config,
            generator_table: GeneratorTableConfig {
                table_idx: lookup.0,
                table_x: lookup.1,
                table_y: lookup.2,
            },
            lookup_config: range_check,
        };

        // Set up lookup argument
        GeneratorTableConfig::configure(meta, config.clone());

        config.configure_from_y_q(meta);
        config
    }
}

// TODO: add doc
pub fn create_common_config<Hash, Commit, F>(
    meta: &mut ConstraintSystem<pallas::Base>,
    advices: [Column<Advice>; 5],
    witness_pieces: Column<Advice>,
    fixed_y_q: Column<Fixed>,
) -> SinsemillaConfigCommon<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    SinsemillaConfigCommon {
        q_sinsemilla1: meta.complex_selector(),
        q_sinsemilla2: meta.fixed_column(),
        q_sinsemilla4: meta.selector(),
        fixed_y_q,
        double_and_add: DoubleAndAdd {
            x_a: advices[0],
            x_p: advices[1],
            lambda_1: advices[3],
            lambda_2: advices[4],
        },
        bits: advices[2],
        witness_pieces,
        _marker: PhantomData,
    }
}


// TODO: remove duplicate?
impl<Hash, Commit, Fixed> Chip<pallas::Base> for SinsemillaChip<Hash, Commit, Fixed>
    where
        Hash: HashDomains<pallas::Affine>,
        Fixed: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    type Config = SinsemillaConfig<Hash, Commit, Fixed>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}



// TODO: remove duplicate?

// Implement `SinsemillaInstructions` for `SinsemillaChip`
impl<Hash, Commit, F> SinsemillaInstructions<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>
for SinsemillaChip<Hash, Commit, F>
    where
        Hash: HashDomains<pallas::Affine>,
        F: FixedPoints<pallas::Affine>,
        Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type CellValue = AssignedCell<pallas::Base, pallas::Base>;

    type Message = Message<pallas::Base, { sinsemilla::K }, { sinsemilla::C }>;
    type MessagePiece = MessagePiece<pallas::Base, { sinsemilla::K }>;

    type RunningSum = Vec<Self::CellValue>;

    type X = AssignedCell<pallas::Base, pallas::Base>;
    type NonIdentityPoint = NonIdentityEccPoint;
    type FixedPoints = F;

    type HashDomains = Hash;
    type CommitDomains = Commit;

    fn witness_message_piece(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        field_elem: Value<pallas::Base>,
        num_words: usize,
    ) -> Result<Self::MessagePiece, Error> {
        let config = self.config().clone();

        let cell = layouter.assign_region(
            || "witness message piece",
            |mut region| {
                region.assign_advice(
                    || "witness message piece",
                    config.base.witness_pieces,
                    0,
                    || field_elem,
                )
            },
        )?;
        Ok(MessagePiece::new(cell, num_words))
    }

    // TODO: in the opt version: hash_message_vanilla -> hash_message
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn hash_to_point(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        Q: pallas::Affine,
        message: Self::Message,
    ) -> Result<(Self::NonIdentityPoint, Vec<Self::RunningSum>), Error> {
        layouter.assign_region(
            || "hash_to_point",
            |mut region| self.hash_message_vanilla(&mut region, Q, &message),
        )
    }

    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn hash_to_point_with_private_init(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        Q: &Self::NonIdentityPoint,
        message: Self::Message,
    ) -> Result<(Self::NonIdentityPoint, Vec<Self::RunningSum>), Error> {
        layouter.assign_region(
            || "hash_to_point",
            |mut region| self.hash_message_with_private_init(&mut region, Q, &message),
        )
    }
    fn extract(point: &Self::NonIdentityPoint) -> Self::X {
        point.x()
    }
}