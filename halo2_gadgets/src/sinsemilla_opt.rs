//! Optimized Sinsemilla gadgets.

use crate::sinsemilla::SinsemillaInstructions;
use halo2_proofs::circuit::Layouter;
use halo2_proofs::plonk::Error;
use pasta_curves::arithmetic::CurveAffine;

pub mod chip;
pub mod merkle;

/// The `ExtendedSinsemillaInstructions` trait extends the functionality of
/// the `SinsemillaInstructions` trait, providing additional function hash_to_point_with_private_init.
pub trait ExtendedSinsemillaInstructions<C: CurveAffine, const K: usize, const MAX_WORDS: usize>:
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
