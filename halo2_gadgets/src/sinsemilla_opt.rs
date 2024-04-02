//! The [Sinsemilla] hash function.
//!
//! [Sinsemilla]: https://zips.z.cash/protocol/protocol.pdf#concretesinsemillahash

use std::fmt::Debug;

use pasta_curves::arithmetic::CurveAffine;

use halo2_proofs::{circuit::Layouter, plonk::Error};

use crate::{
    ecc::{self, EccInstructions},
    sinsemilla::{CommitDomain, HashDomain, Message, SinsemillaInstructions},
};

pub mod chip;
pub mod merkle;
pub mod primitives;

/// FIXME: add a doc
pub trait SinsemillaInstructionsOptimized<C: CurveAffine, const K: usize, const MAX_WORDS: usize>:
    SinsemillaInstructions<C, K, MAX_WORDS>
{
    /// Hashes a message to an ECC curve point.
    /// This returns both the resulting point, as well as the message
    /// decomposition in the form of intermediate values in a cumulative
    /// sum.
    /// The initial point `Q` is a private point.
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    fn hash_to_point_with_private_init(
        &self,
        layouter: impl Layouter<C::Base>,
        Q: &Self::NonIdentityPoint,
        message: Self::Message,
    ) -> Result<(Self::NonIdentityPoint, Vec<Self::RunningSum>), Error>;
}

impl<C: CurveAffine, SinsemillaChip, EccChip, const K: usize, const MAX_WORDS: usize>
    HashDomain<C, SinsemillaChip, EccChip, K, MAX_WORDS>
where
    SinsemillaChip: SinsemillaInstructionsOptimized<C, K, MAX_WORDS> + Clone + Debug + Eq,
    EccChip: EccInstructions<
            C,
            NonIdentityPoint = <SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::NonIdentityPoint,
            FixedPoints = <SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::FixedPoints,
        > + Clone
        + Debug
        + Eq,
{
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    /// Evaluate the Sinsemilla hash of `message` from the private initial point `Q`.
    pub fn hash_to_point_with_private_init(
        &self,
        layouter: impl Layouter<C::Base>,
        Q: &<SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::NonIdentityPoint,
        message: Message<C, SinsemillaChip, K, MAX_WORDS>,
    ) -> Result<(ecc::NonIdentityPoint<C, EccChip>, Vec<SinsemillaChip::RunningSum>), Error> {
        assert_eq!(self.sinsemilla_chip, message.chip);
        self.sinsemilla_chip
            .hash_to_point_with_private_init(layouter, Q, message.inner)
            .map(|(point, zs)| (ecc::NonIdentityPoint::from_inner(self.ecc_chip.clone(), point), zs))
    }

}

impl<C: CurveAffine, SinsemillaChip, EccChip, const K: usize, const MAX_WORDS: usize>
    CommitDomain<C, SinsemillaChip, EccChip, K, MAX_WORDS>
where
    SinsemillaChip: SinsemillaInstructionsOptimized<C, K, MAX_WORDS> + Clone + Debug + Eq,
    EccChip: EccInstructions<
            C,
            NonIdentityPoint = <SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::NonIdentityPoint,
            FixedPoints = <SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::FixedPoints,
        > + Clone
        + Debug
        + Eq,
{
    #[allow(clippy::type_complexity)]
    /// Evaluates the Sinsemilla hash of `message` from the public initial point `Q` stored
    /// into `CommitDomain`.
    pub fn hash(
        &self,
        layouter: impl Layouter<C::Base>,
        message: Message<C, SinsemillaChip, K, MAX_WORDS>,
    ) -> Result<
        (
            ecc::NonIdentityPoint<C, EccChip>,
            Vec<SinsemillaChip::RunningSum>,
        ),
        Error,
    > {
        assert_eq!(self.M.sinsemilla_chip, message.chip);
        self.M.hash_to_point(layouter, message)
    }

    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    /// Evaluates the Sinsemilla hash of `message` from the private initial point `Q`.
    pub fn hash_with_private_init(
        &self,
        layouter: impl Layouter<C::Base>,
        Q: &<SinsemillaChip as SinsemillaInstructions<C, K, MAX_WORDS>>::NonIdentityPoint,
        message: Message<C, SinsemillaChip, K, MAX_WORDS>,
    ) -> Result<
        (
            ecc::NonIdentityPoint<C, EccChip>,
            Vec<SinsemillaChip::RunningSum>,
        ),
        Error,
    > {
        assert_eq!(self.M.sinsemilla_chip, message.chip);
        self.M.hash_to_point_with_private_init(layouter, Q, message)
    }

    #[allow(clippy::type_complexity)]
    /// Returns the public initial point `Q` stored into `CommitDomain`.
    pub fn q_init(&self) -> C {
        self.M.Q
    }

    #[allow(clippy::type_complexity)]
    /// Evaluates the blinding factor equal to $\[r\] R$ where `r` is stored in the `CommitDomain`.
    pub fn blinding_factor(
        &self,
        mut layouter: impl Layouter<C::Base>,
        r: ecc::ScalarFixed<C, EccChip>,
    ) -> Result<
        ecc::Point<C, EccChip>,
        Error,
    > {
        let (blind, _) = self.R.mul(layouter.namespace(|| "[r] R"), r)?;
        Ok(blind)
    }
}
