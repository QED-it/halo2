//! Chip implementing a Merkle hash using Sinsemilla as the hash function.

use halo2_proofs::{circuit::{AssignedCell, Chip, Layouter, Value}, impl_trait_Chip_for, plonk::{Advice, Column, ConstraintSystem, Constraints, Error, Selector}, poly::Rotation};
use pasta_curves::pallas;

use super::MerkleInstructions;

use crate::sinsemilla::chip::{SinsemillaChipProps, SinsemillaConfigProps};
use crate::sinsemilla_opt::chip::{SinsemillaChipOptimized, SinsemillaConfigOptimized};
use crate::{sinsemilla::{primitives as sinsemilla, MessagePiece}, utilities::RangeConstrained, {
    ecc::FixedPoints,
    sinsemilla::{CommitDomains, HashDomains, SinsemillaInstructions},
    utilities::{
        cond_swap::{CondSwapChip, CondSwapConfig, CondSwapInstructions},
        UtilitiesInstructions,
    },
}, impl_trait_CondSwapInstructions_for, impl_trait_SinsemillaInstructions_for_MerkleChip, impl_trait_UtilitiesInstructions_for, impl_trait_MerkleInstructions_for_MerkleChip, impl_trait_MerkleChipProps_for_MerkleChip};
use group::ff::PrimeField;

/// Configuration for the `MerkleChipOptimized` implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleConfigOptimized<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    advices: [Column<Advice>; 5],
    q_decompose: Selector,
    pub(super) cond_swap_config: CondSwapConfig,
    pub(super) sinsemilla_config: SinsemillaConfigOptimized<Hash, Commit, Fixed>,
}

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
    config: MerkleConfigOptimized<Hash, Commit, Fixed>,
}

impl_trait_Chip_for!(MerkleChipOptimized<Hash, Commit, Fixed>, MerkleConfigOptimized<Hash, Commit, Fixed>);
impl_trait_MerkleChipProps_for_MerkleChip!(
    MerkleConfigOptimized,
    MerkleChipOptimized<Hash, Commit, F>,
    MerkleConfigOptimized<Hash, Commit, F>,
    SinsemillaConfigOptimized<Hash, Commit, F>
);
impl_trait_MerkleInstructions_for_MerkleChip!(MerkleChipOptimized<Hash, Commit, F>);
impl_trait_UtilitiesInstructions_for!(MerkleChipOptimized<Hash, Commit, F>);
impl_trait_CondSwapInstructions_for!(MerkleChipOptimized<Hash, Commit, F>);
impl_trait_SinsemillaInstructions_for_MerkleChip!(MerkleChipOptimized<Hash, Commit, F>, SinsemillaChipOptimized<Hash, Commit, F>,SinsemillaChipOptimized::<Hash, Commit, F>::construct);
