use super::{NonIdentityEccPoint, SinsemillaChipOptimized};
use crate::sinsemilla::chip::hash_to_point::{
    hash_all_pieces, verify_sinsemilla_hash, EccPointQ, X, Y,
};
use crate::sinsemilla::{CommitDomains, HashDomains, SinsemillaInstructions};
use crate::{
    ecc::FixedPoints,
    sinsemilla::primitives::{self as sinsemilla},
};
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Region},
    plonk::{Assigned, Error},
};
use pasta_curves::{arithmetic::CurveAffine, pallas};

impl<Hash, Commit, Fixed> SinsemillaChipOptimized<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    /// [Specification](https://p.z.cash/halo2-0.1:sinsemilla-constraints?partial).
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    pub(crate) fn hash_message(
        &self,
        region: &mut Region<'_, pallas::Base>,
        Q: pallas::Affine,
        message: &<Self as SinsemillaInstructions<
            pallas::Affine,
            { sinsemilla::K },
            { sinsemilla::C },
        >>::Message,
    ) -> Result<
        (
            NonIdentityEccPoint,
            Vec<Vec<AssignedCell<pallas::Base, pallas::Base>>>,
        ),
        Error,
    > {
        let (offset, x_a, y_a) = self.public_initialization(region, Q)?;

        let (x_a, y_a, zs_sum) = hash_all_pieces(self, region, offset, message, x_a, y_a)?;

        verify_sinsemilla_hash(message, EccPointQ::PublicPoint(Q), x_a, y_a, zs_sum)
    }

    /// [Specification](https://p.z.cash/halo2-0.1:sinsemilla-constraints?partial).
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    pub(crate) fn hash_message_with_private_init(
        &self,
        region: &mut Region<'_, pallas::Base>,
        Q: &NonIdentityEccPoint,
        message: &<Self as SinsemillaInstructions<
            pallas::Affine,
            { sinsemilla::K },
            { sinsemilla::C },
        >>::Message,
    ) -> Result<
        (
            NonIdentityEccPoint,
            Vec<Vec<AssignedCell<pallas::Base, pallas::Base>>>,
        ),
        Error,
    > {
        let (offset, x_a, y_a) = self.private_initialization(region, Q)?;

        let (x_a, y_a, zs_sum) = hash_all_pieces(self, region, offset, message, x_a, y_a)?;

        verify_sinsemilla_hash(message, EccPointQ::PrivatePoint(Q), x_a, y_a, zs_sum)
    }

    #[allow(non_snake_case)]
    /// Assign the coordinates of the initial public point `Q`
    ///
    /// | offset | x_A | x_P | q_sinsemilla4 |
    /// --------------------------------------
    /// |   0    |     | y_Q |               |
    /// |   1    | x_Q |     |       1       |
    fn public_initialization(
        &self,
        region: &mut Region<'_, pallas::Base>,
        Q: pallas::Affine,
    ) -> Result<(usize, X<pallas::Base>, Y<pallas::Base>), Error> {
        let config = self.config().clone();
        let mut offset = 0;

        // Get the `x`- and `y`-coordinates of the starting `Q` base.
        let x_q = *Q.coordinates().unwrap().x();
        let y_q = *Q.coordinates().unwrap().y();

        // Constrain the initial x_a, lambda_1, lambda_2, x_p using the q_sinsemilla4
        // selector.
        let y_a: Y<pallas::Base> = {
            // Enable `q_sinsemilla4` on the second row.
            config.base.q_sinsemilla4.enable(region, offset + 1)?;
            let y_a: AssignedCell<Assigned<pallas::Base>, pallas::Base> = region
                .assign_advice_from_constant(
                    || "fixed y_q",
                    config.base.double_and_add.x_p,
                    offset,
                    y_q.into(),
                )?;

            y_a.value_field().into()
        };
        offset += 1;

        // Constrain the initial x_q to equal the x-coordinate of the domain's `Q`.
        let x_a: X<pallas::Base> = {
            let x_a = region.assign_advice_from_constant(
                || "fixed x_q",
                config.base.double_and_add.x_a,
                offset,
                x_q.into(),
            )?;

            x_a.into()
        };

        Ok((offset, x_a, y_a))
    }

    #[allow(non_snake_case)]
    /// Assign the coordinates of the initial private point `Q`
    ///
    /// | offset | x_A | x_P | q_sinsemilla4 |
    /// --------------------------------------
    /// |   0    |     | y_Q |               |
    /// |   1    | x_Q |     |         1     |
    fn private_initialization(
        &self,
        region: &mut Region<'_, pallas::Base>,
        Q: &NonIdentityEccPoint,
    ) -> Result<(usize, X<pallas::Base>, Y<pallas::Base>), Error> {
        let config = self.config().clone();
        let mut offset = 0;

        // Assign `x_Q` and `y_Q` in the region and constrain the initial x_a, lambda_1, lambda_2,
        // x_p, y_Q using the q_sinsemilla4 selector.
        let y_a: Y<pallas::Base> = {
            // Enable `q_sinsemilla4` on the second row.
            config.base.q_sinsemilla4.enable(region, offset + 1)?;
            let q_y: AssignedCell<Assigned<pallas::Base>, pallas::Base> = Q.y().into();
            let y_a: AssignedCell<Assigned<pallas::Base>, pallas::Base> = q_y.copy_advice(
                || "fixed y_q",
                region,
                config.base.double_and_add.x_p,
                offset,
            )?;

            y_a.value_field().into()
        };
        offset += 1;

        let x_a: X<pallas::Base> = {
            let q_x: AssignedCell<Assigned<pallas::Base>, pallas::Base> = Q.x().into();
            let x_a = q_x.copy_advice(
                || "fixed x_q",
                region,
                config.base.double_and_add.x_a,
                offset,
            )?;

            x_a.into()
        };

        Ok((offset, x_a, y_a))
    }
}
