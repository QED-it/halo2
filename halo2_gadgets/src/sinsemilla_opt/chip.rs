//! Chip implementations for the Sinsemilla_optimized gadgets.

mod generator_table;
mod hash_to_point;

use crate::ecc::{chip::NonIdentityEccPoint, FixedPoints};
use crate::sinsemilla::message::{Message, MessagePiece};
use crate::sinsemilla::primitives as sinsemilla;
use halo2_proofs::{circuit::{AssignedCell, Chip, Layouter, Value}, impl_trait_Chip_for, plonk::{
    Advice, Column, ConstraintSystem, Error, Expression, Fixed, TableColumn, VirtualCells,
}, poly::Rotation};
use pasta_curves::pallas;
use pasta_curves::pallas::{Affine, Base};

use crate::sinsemilla::chip::{
    create_common_config, SinsemillaChipProps, SinsemillaConfigCommon, SinsemillaConfigProps,
};
use crate::sinsemilla::primitives::{C, K};
use crate::sinsemilla::{CommitDomains, HashDomains, SinsemillaInstructions};
use crate::sinsemilla_opt::ExtendedSinsemillaInstructions;
use crate::utilities_opt::lookup_range_check::LookupRangeCheckConfigOptimized;
use generator_table::GeneratorTableConfigOptimized;
use crate::{impl_trait_SinsemillaInstructions_for_SinsemillaChip};

/// Configuration for the SinsemillaConfigOptimized hash chip
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SinsemillaConfigOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    base: SinsemillaConfigCommon<Hash, Commit, F>,

    /// The lookup table where $(\mathsf{idx}, x_p, y_p)$ are loaded for the $2^K$
    /// generators of the Sinsemilla hash.
    pub(super) generator_table: GeneratorTableConfigOptimized,
    /// An advice column configured to perform lookup range checks.
    lookup_config: LookupRangeCheckConfigOptimized<pallas::Base, { sinsemilla::K }>,
}
impl<Hash, Commit, F> SinsemillaConfigProps<Hash, Commit, F>
    for SinsemillaConfigOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type LookupConfigType = LookupRangeCheckConfigOptimized<pallas::Base, { sinsemilla::K }>;
    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F> {
        &self.base
    }

    fn lookup_config(&self) -> Self::LookupConfigType {
        self.lookup_config
    }

    /// Query an advice value 'y_q' from a specific advice column `x_p` at the previous rotation.
    fn get_y_q(&self, meta: &mut VirtualCells<Base>) -> Expression<Base> {
        meta.query_advice(self.base.double_and_add.x_p, Rotation::prev())
    }
}

/// A chip that implements 10-bit Sinsemilla using a lookup table and 5 advice columns.
///
/// [Chip description](https://zcash.github.io/halo2/design/gadgets/sinsemilla.html#plonk--halo-2-constraints).
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SinsemillaChipOptimized<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    config: SinsemillaConfigOptimized<Hash, Commit, Fixed>,
}

impl<Hash, Commit, F> SinsemillaChipProps<Hash, Commit, F>
    for SinsemillaChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
    type Loaded = <Self as Chip<pallas::Base>>::Loaded;

    type RangeCheckConfigType = LookupRangeCheckConfigOptimized<pallas::Base, { sinsemilla::K }>;

    type SinsemillaConfigType = SinsemillaConfigOptimized<Hash, Commit, F>;

    type LookupTableColumnType = (TableColumn, TableColumn, TableColumn, TableColumn);

    fn base(&self) -> &SinsemillaConfigCommon<Hash, Commit, F> {
        &self.config.base
    }

    fn config(&self) -> Self::SinsemillaConfigType {
        self.config.clone()
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

    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    fn configure(
        meta: &mut ConstraintSystem<pallas::Base>,
        advices: [Column<Advice>; 5],
        witness_pieces: Column<Advice>,
        fixed_y_q: Column<Fixed>,
        lookup: Self::LookupTableColumnType,
        range_check: Self::RangeCheckConfigType,
    ) -> Self::SinsemillaConfigType {
        // Enable equality on all advice columns
        for advice in advices.iter() {
            meta.enable_equality(*advice);
        }

        let base_config = create_common_config(meta, advices, witness_pieces, fixed_y_q);
        let config = SinsemillaConfigOptimized::<Hash, Commit, F> {
            base: base_config,
            generator_table: GeneratorTableConfigOptimized {
                table_idx: lookup.0,
                table_x: lookup.1,
                table_y: lookup.2,
                table_range_check_tag: lookup.3,
            },
            lookup_config: range_check,
        };

        // Set up lookup argument
        GeneratorTableConfigOptimized::configure(meta, config.clone());

        config.configure_from_y_q(meta);
        config
    }
}

impl_trait_Chip_for!(SinsemillaChipOptimized<Hash, Commit, Fixed>, SinsemillaConfigOptimized<Hash, Commit, Fixed>);
impl_trait_SinsemillaInstructions_for_SinsemillaChip!(SinsemillaChipOptimized<Hash, Commit, F>);

// Implement `ExtendedSinsemillaInstructions` for `SinsemillaChip`
impl<Hash, Commit, F>
    ExtendedSinsemillaInstructions<pallas::Affine, { sinsemilla::K }, { sinsemilla::C }>
    for SinsemillaChipOptimized<Hash, Commit, F>
where
    Hash: HashDomains<pallas::Affine>,
    F: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, F, Hash>,
{
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
}
