//! Chip implementing a Merkle hash using Sinsemilla as the hash function.

use halo2_proofs::{
    circuit::{Chip, Layouter},
    plonk::Error,
};
use pasta_curves::pallas;

use crate::{
    sinsemilla::{merkle::chip::MerkleChip, primitives as sinsemilla},
    {
        sinsemilla::SinsemillaInstructions, utilities::cond_swap::CondSwapChip,
        utilities_opt::cond_swap::CondSwapInstructionsOptimized,
    },
};

impl<SinsemillaChip> CondSwapInstructionsOptimized<pallas::Base> for MerkleChip<SinsemillaChip>
where
    SinsemillaChip: Chip<pallas::Base>
        + SinsemillaInstructions<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>,
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

// FIXME: uncomment and implement this properly
/*
impl<SinsemillaChip>
    SinsemillaInstructionsOptimized<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>
    for MerkleChip<SinsemillaChip>
where
    SinsemillaChip: Chip<pallas::Base>
        + SinsemillaInstructions<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>,
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
*/
