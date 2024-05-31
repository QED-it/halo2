# Conditional swap

The conditional swap gadgets include two instructions: swap and multiplexer (mux).
We use a multiplexer to compute differently for ZEC or for non-ZEC assets in Orchard protocol.
The graph shows where each new gadget (hash from private initial point and MUX on non-identity points) will be used.

![](https://imgur.com/kDQLoQQ.png)

## Multiplexer

Given a boolean flag $\textsf{choice}$, mux is used for selecting one of two values ($\textsf{left}$ and $\textsf{right}$) based on the boolean flag.
If $\textsf{choice}$ is true, it returns $\textsf{right}$; otherwise, it returns $\textsf{left}$. This functionality is crucial for circuits that require conditional logic.


## Chip instructions

```rust,ignore,no_run
pub trait CondSwapInstructions<F: Field>: UtilitiesInstructions<F> {
    /// Given an input `(choice, left, right)` where `choice` is a boolean flag,
    /// returns `left` if `choice` is not set and `right` if `choice` is set.
    fn mux(
        &self,
        layouter: &mut impl Layouter<F>,
        choice: Self::Var,
        left: Self::Var,
        right: Self::Var,
    ) -> Result<Self::Var, Error>;
}
```

## Implement chip traits

```rust,ignore,no_run
impl<F: PrimeField> CondSwapInstructions<F> for CondSwapChip<F> {
    fn mux(
        &self,
        layouter: &mut impl Layouter<F>,
        choice: Self::Var,
        left: Self::Var,
        right: Self::Var,
    ) -> Result<Self::Var, Error> {
        let config = self.config();

        layouter.assign_region(
            || "mux",
            |mut region| {
                // Enable `q_swap` selector
                config.q_swap.enable(&mut region, 0)?;

                // Copy in `a` value
                let left = left.copy_advice(|| "copy left", &mut region, config.a, 0)?;

                // Copy in `b` value
                let right = right.copy_advice(|| "copy right", &mut region, config.b, 0)?;

                // Copy `choice` value
                let choice = choice.copy_advice(|| "copy choice", &mut region, config.swap, 0)?;

                let a_swapped = left
                    .value()
                    .zip(right.value())
                    .zip(choice.value())
                    .map(|((left, right), choice)| {
                        if *choice == F::from(0_u64) {
                            left
                        } else {
                            right
                        }
                    })
                    .cloned();
                let b_swapped = left
                    .value()
                    .zip(right.value())
                    .zip(choice.value())
                    .map(|((left, right), choice)| {
                        if *choice == F::from(0_u64) {
                            right
                        } else {
                            left
                        }
                    })
                    .cloned();

                region.assign_advice(|| "out b_swap", self.config.b_swapped, 0, || b_swapped)?;
                region.assign_advice(|| "out a_swap", self.config.a_swapped, 0, || a_swapped)
            },
        )
    }
}
```

## Multiplexer logic on ECC Points

Mux can also be extended to work with elliptic curve points, facilitating operations like conditional selections between points. 
Based on a boolean flag $\textsf{choice}$, it selects between two given points $\textsf{left}$ and $\textsf{right}$. 
If $\textsf{choice}$ is true, it returns the point $\textsf{right}$; otherwise, it returns the point $\textsf{left}$. 

```rust,ignore,no_run
impl CondSwapChip<pallas::Base> {
    /// Given an input `(choice, left, right)` where `choice` is a boolean flag and `left` and `right` are `EccPoint`,
    /// returns `left` if `choice` is not set and `right` if `choice` is set.
    pub fn mux_on_points(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        choice: &AssignedCell<pallas::Base, pallas::Base>,
        left: &EccPoint,
        right: &EccPoint,
    ) -> Result<EccPoint, plonk::Error> {
        let x_cell = self.mux(&mut layouter, choice.clone(), left.x(), right.x())?;
        let y_cell = self.mux(&mut layouter, choice.clone(), left.y(), right.y())?;
        Ok(EccPoint::from_coordinates_unchecked(
            x_cell.into(),
            y_cell.into(),
        ))
    }

    /// Given an input `(choice, left, right)` where `choice` is a boolean flag and `left` and `right` are
    /// `NonIdentityEccPoint`, returns `left` if `choice` is not set and `right` if `choice` is set.
    pub fn mux_on_non_identity_points(
        &self,
        mut layouter: impl Layouter<pallas::Base>,
        choice: &AssignedCell<pallas::Base, pallas::Base>,
        left: &NonIdentityEccPoint,
        right: &NonIdentityEccPoint,
    ) -> Result<NonIdentityEccPoint, plonk::Error> {
        let x_cell = self.mux(&mut layouter, choice.clone(), left.x(), right.x())?;
        let y_cell = self.mux(&mut layouter, choice.clone(), left.y(), right.y())?;
        Ok(NonIdentityEccPoint::from_coordinates_unchecked(
            x_cell.into(),
            y_cell.into(),
        ))
    }
}
```



