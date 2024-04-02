//! Elliptic curve operations.

use std::fmt::Debug;

use halo2_proofs::{
    arithmetic::CurveAffine,
    circuit::{AssignedCell, Layouter},
    plonk::Error,
};

use crate::ecc::{EccInstructions, Point};

pub(crate) mod chip;

/// The set of circuit instructions required to use the ECC gadgets.
pub trait EccInstructionsOptimized<C: CurveAffine>: EccInstructions<C> {
    /// Witnesses the given constant point as a private input to the circuit.
    /// This allows the point to be the identity, mapped to (0, 0) in
    /// affine coordinates.
    fn witness_point_from_constant(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        value: C,
    ) -> Result<Self::Point, Error>;

    /// Performs variable-base sign-scalar multiplication, returning `[sign] point`
    /// `sign` must be in {-1, 1}.
    fn mul_sign(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        sign: &AssignedCell<C::Base, C::Base>,
        point: &Self::Point,
    ) -> Result<Self::Point, Error>;
}

impl<C: CurveAffine, EccChip: EccInstructionsOptimized<C> + Clone + Debug + Eq> Point<C, EccChip> {
    /// Constructs a new point with the given fixed value.
    pub fn new_from_constant(
        chip: EccChip,
        mut layouter: impl Layouter<C::Base>,
        value: C,
    ) -> Result<Self, Error> {
        let point = chip.witness_point_from_constant(&mut layouter, value);
        point.map(|inner| Point { chip, inner })
    }

    /// Returns `[sign] self`.
    /// `sign` must be in {-1, 1}.
    pub fn mul_sign(
        &self,
        mut layouter: impl Layouter<C::Base>,
        sign: &AssignedCell<C::Base, C::Base>,
    ) -> Result<Point<C, EccChip>, Error> {
        self.chip
            .mul_sign(&mut layouter, sign, &self.inner)
            .map(|point| Point {
                chip: self.chip.clone(),
                inner: point,
            })
    }
}
