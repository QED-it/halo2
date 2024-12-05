# Variable-base sign scalar multiplication

In the OrchardZSA circuit, we need to evaluate $[\mathsf{v}] \mathsf{AssetBase}$
where $\mathsf{v} = \mathsf{v^{old}} - \mathsf{v^{new}} = s \cdot m$ with $m \in [0, 2^{64})$ and $s \in \{-1, 1\}$.

We will evaluate it in three steps:

1. Check that $m$ is a 64-bit unsigned integer using the [decomposition](../decomposition.md) gadget
   with $W = 6, K = 10$ and checking that the last window is a 4-bit value.
2. Evaluate $comm = [m] \mathsf{AssetBase}$ by using
   a [variable-base long scalar multiplication](./var-base-scalar-mul.md).
3. Evaluate $[\mathsf{v}] \mathsf{AssetBase} = [s] comm$ by using a variable-base sign scalar multiplication.

## Variable-base sign scalar multiplication gate

We would like to evaluate $[s] comm$ where $s \in \{-1, 1\}$.

To do that, we will reuse the sign mul gate with the $q_\texttt{mul_fixed_short}$ selector
from the [fixed-base short signed scalar multiplication](./fixed-base-scalar-mul.md#short-signed-scalar)
gadget with $u=0$ for the last window.

### Layout

The layout of the variable-base sign scalar multiplication is:

$$
\begin{array}{|c|c|c|c|c|}
\hline
y_P & y_{QR} & u & \text{window} & q_\texttt{mul_fixed_short} \\\hline
signed\_y & y & 0 & s & 1 \\\hline
\end{array}
$$

where $y$ is equal to the $y$-coordinate of the point $comm$ and
$signed\_y = s \cdot y = \begin{cases}
y \text{ if } s = 1 \\
-y \text{ if } s = -1
\end{cases}$.

### Constraints

$$
\begin{array}{|c|l|l|}
\hline
\text{Degree} & \text{Constraint} & \text{Comment} \\\hline
3 & q_\texttt{mul_fixed_short} \cdot u \cdot (1-u) = 0 & \text{The last window must be a single bit.}\\\hline
3 & q_\texttt{mul_fixed_short} \cdot \left(signed\_y - y\right) \cdot \left(signed\_y + y\right) = 0 & \text{$signed\_y$ is equal to $y$ or $-y$.}\\\hline
3 & q_\texttt{mul_fixed_short} \cdot \left(s^2 - 1\right) = 0 & \text{The sign must be $1$ or $-1$.}\\\hline
3 & q_\texttt{mul_fixed_short} \cdot \left(s \cdot signed\_y - y\right) = 0 & \text{The correct sign is witnessed.}\\\hline
\end{array}
$$

The first constraint is not necessary here, but we keep it to not create a new gate.

Finally, we could create the point $[s] comm$ such that
- its $x$-coordinate is equal to the $x$-coordinate of the point $comm$, and
- its $y$-coordinate is equal to $signed\_y$.
