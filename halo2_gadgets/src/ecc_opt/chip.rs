use pasta_curves::pallas;
use halo2_proofs::plonk::{Advice, Column};
use crate::ecc::chip::{EccChip, EccConfig};
use crate::ecc::EccInstructions;
use crate::{impl_trait_Chip_fixedpoints_for, impl_trait_UtilitiesInstructions_for};
use crate::utilities::lookup_range_check::LookupRangeCheckConfig;
use crate::utilities_opt::lookup_range_check::LookupRangeCheckConfigOptimized;

// TODO: EccConfig has LookupRangeCheckConfig
/// Configuration for [`EccChip`].
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct EccConfigOptimized<FixedPoints: FixedPoints<pallas::Affine>> {
    /// Advice columns needed by instructions in the ECC chip.
    pub advices: [Column<Advice>; 10],

    /// Incomplete addition
    add_incomplete: crate::ecc::chip::add_incomplete::Config,

    /// Complete addition
    add: crate::ecc::chip::add::Config,

    /// Variable-base scalar multiplication
    mul: crate::ecc::chip::mul::Config,

    /// Fixed-base full-width scalar multiplication
    mul_fixed_full: crate::ecc::chip::mul_fixed::full_width::Config<FixedPoints>,
    /// Fixed-base signed short scalar multiplication
    mul_fixed_short: crate::ecc::chip::mul_fixed::short::Config<FixedPoints>,
    /// Fixed-base mul using a base field element as a scalar
    mul_fixed_base_field: crate::ecc::chip::mul_fixed::base_field_elem::Config<FixedPoints>,

    /// Witness point
    witness_point: crate::ecc::chip::witness_point::Config,

    /// Lookup range check using 10-bit lookup table
    pub lookup_config: LookupRangeCheckConfigOptimized<pallas::Base, { crate::sinsemilla::primitives::K }>,
}

/// An [`EccInstructions`] chip that uses 10 advice columns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EccChipOptimized<FixedPoints: crate::ecc::FixedPoints<pallas::Affine>> {
    config: EccConfigOptimized<FixedPoints>,
}

impl_trait_Chip_fixedpoints_for!(EccChipOptimized<FixedPoints>,EccConfigOptimized<FixedPoints>);
impl_trait_UtilitiesInstructions_for!(EccChipOptimized<Fixed>);
