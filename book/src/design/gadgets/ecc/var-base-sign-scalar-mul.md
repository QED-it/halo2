# Variable-base sign-scalar multiplication

In the Orchard circuit we need to compute $-P$, that is the negation of a point $P$. 
Specifically, the verifier must compute the value balance verification equation, which includes the operation for the negation of a point.


## Signed Scalar

A signed scalar is witnessed as a magnitude $\mathsf{v}$ and sign $\mathsf{sign}$ such that

$$
\mathsf{sign} \in \{-1, 1\}, \\
\mathsf{v} \in (0, 2^{l_{\mathsf{value}}}), \\
\mathsf{w} = \mathsf{sign} \cdot \mathsf{v}.
$$

This is used for computing the value balance verification equation. We want to compute 
$$-\mathsf{ValueCommit_0^{OrchardZSA}}(\mathsf{AssetBase},\mathsf{v}) = -[\mathsf{v}] \mathsf{AssetBase} - [0] \mathcal{R},$$ 
where $(\mathsf{AssetBase},\mathsf{v})\in\mathsf{AssetBurn}$ and 
$\mathsf{v}\in \{1,\dots, 2^{l_{\mathsf{value}}}-1\}$.

This computation of $-[\mathsf{v}]\mathsf{AssetBase}$ can break into two parts:

- variable-base scalar multiplication, computing $[\mathsf{v}]\mathsf{AssetBase}$.
- variable-base sign-scalar multiplication, computing the negation $-P$, where $P=[\mathsf{v}]\mathsf{AssetBase}$.

## Compute $[\mathsf{sign}]P$

- Input: $P=(\texttt{unsigned\_x},\texttt{unsigned\_y})$
- Output: $[\mathsf{sign}]P=(\texttt{signed\_x},\texttt{signed\_y})$

In complete addition, we have 
$$ (x, y) + (x, -y) = \mathcal{O}. $$
For any $P=(x,y)$, we have $-P=(x,-y)$.

Given a sign $\mathsf{sign}$ and a point $P$, we compute $[\mathsf{sign}]P$ as follows:
 
1. $\texttt{signed\_x} = \texttt{unsigned\_x}$
2. If $\mathsf{sign} = -1$, the y-coordinate is negated, i.e. $\texttt{signed\_y} = -\texttt{unsigned\_y}$.
3. If $\mathsf{sign} = 1$, the y-coordinate remains unchanged, i.e. $\texttt{signed\_y} = \texttt{unsigned\_y}$.

## Constraints
We multiply the point by a sign $\mathsf{sign}$, using the $q_\texttt{mul\_fixed\_short}$ gate.
$$
\begin{array}{|c|l|l|}
\hline
\text{Degree} & \text{Constraint} & \text{Comment} \\\hline
3 & q_\texttt{mul\_fixed\_short} \cdot \left(\mathsf{sign}^2 - 1\right) = 0  &\text{Sign check. The sign must be $1$ or $-1$.}\\\hline
3 & q_\texttt{mul\_fixed\_short} \cdot \left(\mathsf{sign} \cdot \texttt{unsigned\_y} - \texttt{signed\_y} \right) = 0  &\text{Negation check. $\mathsf{sign}  \cdot \texttt{unsigned\_y} = \texttt{signed\_y}$.}\\\hline
\end{array}
$$

