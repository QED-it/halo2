use pasta_curves::{arithmetic::CurveAffine, pallas};

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Region},
    plonk::{Assigned, Error},
};

use crate::{
    ecc::{chip::NonIdentityEccPoint, FixedPoints},
    sinsemilla::{
        chip::{
            hash_to_point::{X, Y},
            SinsemillaChip,
        },
        primitives::{self as sinsemilla},
        CommitDomains, HashDomains, SinsemillaInstructions,
    },
};

impl<Hash, Commit, Fixed> SinsemillaChip<Hash, Commit, Fixed>
where
    Hash: HashDomains<pallas::Affine>,
    Fixed: FixedPoints<pallas::Affine>,
    Commit: CommitDomains<pallas::Affine, Fixed, Hash>,
{
    /// [Specification](https://p.z.cash/halo2-0.1:sinsemilla-constraints?partial).
    #[allow(non_snake_case)]
    #[allow(clippy::type_complexity)]
    pub(crate) fn hash_message_optimized(
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
        let (offset, x_a, y_a) = if self
            .config()
            .generator_table_optimized
            .table_range_check_tag
            .is_some()
        {
            self.public_initialization_optimized(region, Q)?
        } else {
            self.public_initialization(region, Q)?
        };

        let (x_a, y_a, zs_sum) = self.hash_all_pieces(region, offset, message, x_a, y_a)?;

        #[cfg(test)]
        Self::check_hash_result(&Q, message, &x_a, &y_a);

        x_a.value()
            .zip(y_a.value())
            .error_if_known_and(|(x_a, y_a)| x_a.is_zero_vartime() || y_a.is_zero_vartime())?;
        Ok((
            NonIdentityEccPoint::from_coordinates_unchecked(x_a.0, y_a),
            zs_sum,
        ))
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

        let (x_a, y_a, zs_sum) = self.hash_all_pieces(region, offset, message, x_a, y_a)?;

        // FIXME: try to avoid duplication with `check_hash_result` method
        // - it's basically the same code except the following lines:
        //
        // hash_message_with_private_init:
        // ...
        // .zip(Q.point())
        // .assert_if_known(|((field_elems, (x_a, y_a)), Q)| {
        // ...
        //
        // check_hash_result:
        // ...
        // .assert_if_known(|(field_elems, (x_a, y_a))| {
        // ...
        #[cfg(test)]
        #[allow(non_snake_case)]
        // Check equivalence to result from primitives::sinsemilla::hash_to_point
        {
            use group::{prime::PrimeCurveAffine, Curve};
            use pasta_curves::arithmetic::CurveExt;

            use ff::PrimeFieldBits;

            use halo2_proofs::circuit::Value;

            use crate::sinsemilla::primitives::{lebs2ip_k, K, S_PERSONALIZATION};

            let field_elems: Value<Vec<_>> = message
                .iter()
                .map(|piece| piece.field_elem().map(|elem| (elem, piece.num_words())))
                .collect();

            field_elems
                .zip(x_a.value().zip(y_a.value()))
                .zip(Q.point())
                .assert_if_known(|((field_elems, (x_a, y_a)), Q)| {
                    // Get message as a bitstring.
                    let bitstring: Vec<bool> = field_elems
                        .iter()
                        .flat_map(|(elem, num_words)| {
                            elem.to_le_bits().into_iter().take(K * num_words)
                        })
                        .collect();

                    let hasher_S = pallas::Point::hash_to_curve(S_PERSONALIZATION);
                    let S = |chunk: &[bool]| hasher_S(&lebs2ip_k(chunk).to_le_bytes());

                    // We can use complete addition here because it differs from
                    // incomplete addition with negligible probability.
                    let expected_point = bitstring
                        .chunks(K)
                        .fold(Q.to_curve(), |acc, chunk| (acc + S(chunk)) + acc);
                    let actual_point =
                        pallas::Affine::from_xy(x_a.evaluate(), y_a.evaluate()).unwrap();
                    expected_point.to_affine() == actual_point
                });
        }

        x_a.value()
            .zip(y_a.value())
            .error_if_known_and(|(x_a, y_a)| x_a.is_zero_vartime() || y_a.is_zero_vartime())?;
        Ok((
            NonIdentityEccPoint::from_coordinates_unchecked(x_a.0, y_a),
            zs_sum,
        ))
    }

    #[allow(non_snake_case)]
    /// Assign the coordinates of the initial public point `Q`
    ///
    /// | offset | x_A | x_P | q_sinsemilla4 |
    /// --------------------------------------
    /// |   0    |     | y_Q |               |
    /// |   1    | x_Q |     |       1       |
    fn public_initialization_optimized(
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
            config.q_sinsemilla4.enable(region, offset + 1)?;
            let y_a: AssignedCell<Assigned<pallas::Base>, pallas::Base> = region
                .assign_advice_from_constant(
                    || "fixed y_q",
                    config.double_and_add.x_p,
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
                config.double_and_add.x_a,
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
            config.q_sinsemilla4.enable(region, offset + 1)?;
            let q_y: AssignedCell<Assigned<pallas::Base>, pallas::Base> = Q.y().into();
            let y_a: AssignedCell<Assigned<pallas::Base>, pallas::Base> =
                q_y.copy_advice(|| "fixed y_q", region, config.double_and_add.x_p, offset)?;

            y_a.value_field().into()
        };
        offset += 1;

        let x_a: X<pallas::Base> = {
            let q_x: AssignedCell<Assigned<pallas::Base>, pallas::Base> = Q.x().into();
            let x_a = q_x.copy_advice(|| "fixed x_q", region, config.double_and_add.x_a, offset)?;

            x_a.into()
        };

        Ok((offset, x_a, y_a))
    }
}
