//! Chip implementing a Merkle hash using Sinsemilla as the hash function.

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{ConstraintSystem, Error},
};
use pasta_curves::pallas;

use crate::{
    sinsemilla::{
        merkle::{
            chip::{MerkleChip, MerkleConfig},
            MerkleInstructions,
        },
        primitives as sinsemilla,
    },
    sinsemilla_opt::{chip::SinsemillaChipOptimized, SinsemillaInstructionsOptimized},
    utilities_opt::{
        cond_swap::CondSwapInstructionsOptimized,
        lookup_range_check::DefaultLookupRangeCheckConfigOptimized,
    },
    {
        ecc::FixedPoints,
        sinsemilla::{chip::SinsemillaConfig, CommitDomains, HashDomains, SinsemillaInstructions},
        utilities::{
            cond_swap::{CondSwapChip, CondSwapInstructions},
            UtilitiesInstructions,
        },
    },
};

/// Chip implementing `MerkleInstructions`.
///
/// This chip specifically implements `MerkleInstructions::hash_layer` as the `MerkleCRH`
/// function `hash = SinsemillaHash(Q, 𝑙⋆ || left⋆ || right⋆)`, where:
/// - `𝑙⋆ = I2LEBSP_10(l)`
/// - `left⋆ = I2LEBSP_255(left)`
/// - `right⋆ = I2LEBSP_255(right)`
///
/// This chip does **NOT** constrain `left⋆` and `right⋆` to be canonical encodings of
/// `left` and `right`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleChipOptimized<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    base: MerkleChip<Hash, Commit, Fixed, DefaultLookupRangeCheckConfigOptimized>,
}

impl<Hash, Commit, Fixed> Chip<pallas::Base> for MerkleChipOptimized<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    type Config = MerkleConfig<Hash, Commit, Fixed, DefaultLookupRangeCheckConfigOptimized>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.base.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<Hash, Commit, F> MerkleChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    /// Configures the [`MerkleChip`].
    pub fn configure(
        meta: &mut ConstraintSystem<pallas::Base>,
        sinsemilla_config: SinsemillaConfig<
            Hash,
            Commit,
            F,
            DefaultLookupRangeCheckConfigOptimized,
        >,
    ) -> MerkleConfig<Hash, Commit, F, DefaultLookupRangeCheckConfigOptimized> {
        MerkleChip::configure(meta, sinsemilla_config)
    }

    /// Constructs a [`MerkleChip`] given a [`MerkleConfig`].
    pub fn construct(
        config: MerkleConfig<Hash, Commit, F, DefaultLookupRangeCheckConfigOptimized>,
    ) -> Self {
        MerkleChipOptimized {
            base: MerkleChip { config },
        }
    }
}

impl<Hash, Commit, F, const MERKLE_DEPTH: usize>
    MerkleInstructions<pallas::Affine, MERKLE_DEPTH, { sinsemilla::K }, { sinsemilla::C }>
    for MerkleChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine> + Eq,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash> + Eq,
{
    #[allow(non_snake_case)]
    fn hash_layer(
        &self,
        layouter: impl Layouter<pallas::Base>,
        Q: pallas::Affine,
        // l = MERKLE_DEPTH - layer - 1
        l: usize,
        left: Self::Var,
        right: Self::Var,
    ) -> Result<Self::Var, Error> {
        MerkleInstructions::<pallas::Affine, MERKLE_DEPTH, { sinsemilla::K }, { sinsemilla::C }>::hash_layer(
            &self.base, layouter, Q, l, left, right,
        )
    }
}

impl<Hash, Commit, F> UtilitiesInstructions<pallas::Base> for MerkleChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type Var = AssignedCell<pallas::Base, pallas::Base>;
}

impl<Hash, Commit, F> CondSwapInstructions<pallas::Base> for MerkleChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    #[allow(clippy::type_complexity)]
    fn swap(
        &self,
        layouter: impl Layouter<pallas::Base>,
        pair: (Self::Var, Value<pallas::Base>),
        swap: Value<bool>,
    ) -> Result<(Self::Var, Self::Var), Error> {
        self.base.swap(layouter, pair, swap)
    }
}

impl<Hash, Commit, F> SinsemillaInstructions<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>
    for MerkleChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type CellValue = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::CellValue;

    type Message = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::Message;
    type MessagePiece = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::MessagePiece;
    type RunningSum = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::RunningSum;

    type X = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::X;
    type NonIdentityPoint = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::NonIdentityPoint;
    type FixedPoints = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::FixedPoints;

    type HashDomains = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::HashDomains;
    type CommitDomains = <SinsemillaChipOptimized<Hash, Commit, F> as SinsemillaInstructions<
        pallas::Affine,
        { sinsemilla::K },
        { sinsemilla::C },
    >>::CommitDomains;

    fn witness_message_piece(
        &self,
        layouter: impl Layouter<pallas::Base>,
        value: Value<pallas::Base>,
        num_words: usize,
    ) -> Result<Self::MessagePiece, Error> {
        let config = self.config().sinsemilla_config.clone();
        let chip = SinsemillaChipOptimized::<Hash, Commit, F>::construct(config);
        chip.witness_message_piece(layouter, value, num_words)
    }

    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn hash_to_point(
        &self,
        layouter: impl Layouter<pallas::Base>,
        Q: pallas::Affine,
        message: Self::Message,
    ) -> Result<(Self::NonIdentityPoint, Vec<Vec<Self::CellValue>>), Error> {
        let config = self.config().sinsemilla_config.clone();
        let chip = SinsemillaChipOptimized::<Hash, Commit, F>::construct(config);
        chip.hash_to_point(layouter, Q, message)
    }

    fn extract(point: &Self::NonIdentityPoint) -> Self::X {
        SinsemillaChipOptimized::<Hash, Commit, F>::extract(point)
    }
}

impl<Hash, Commit, F> CondSwapInstructionsOptimized<pallas::Base>
    for MerkleChip<Hash, Commit, F, DefaultLookupRangeCheckConfigOptimized>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    fn mux(
        &self,
        layouter: &mut impl Layouter<pallas::Base>,
        choice: Self::Var,
        left: Self::Var,
        right: Self::Var,
    ) -> Result<Self::Var, Error> {
        let config = self.config().cond_swap_config.clone();
        let chip = CondSwapChip::<pallas::Base>::construct(config);
        chip.mux(layouter, choice, left, right)
    }
}

impl<Hash, Commit, F>
    SinsemillaInstructionsOptimized<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>
    for MerkleChip<Hash, Commit, F, DefaultLookupRangeCheckConfigOptimized>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn hash_to_point_with_private_init(
        &self,
        layouter: impl Layouter<pallas::Base>,
        Q: &Self::NonIdentityPoint,
        message: Self::Message,
    ) -> Result<(Self::NonIdentityPoint, Vec<Vec<Self::CellValue>>), Error> {
        let config = self.config().sinsemilla_config.clone();
        let chip = SinsemillaChipOptimized::<Hash, Commit, F>::construct(config);
        chip.hash_to_point_with_private_init(layouter, Q, message)
    }
}
