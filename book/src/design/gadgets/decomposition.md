# Decomposition

In this section, our objective is to address the lookup constraint that verifies whether an element falls within a range of up to 10 bits.
This analysis of lookup decomposition (call $\texttt{lookup}$) is applied in different contexts:
- [10 bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#lookup-decomposition) (for running sum $(z_{i} - 2^K \cdot z_{i+1})$)
- [n bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#short-range-check), where $n\leq 10$. (for a single element short range check)
- [4 and 5 bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#optimized-short-range-check-on-4-and-5-bits) (to optimize the range check for 4 and 5 bits)
- [up to 3 bits.](https://zcash.github.io/halo2/design/gadgets/decomposition.html#short-range-decomposition) 
The range check method, denoted as $\texttt{range\_check}$, is employed to validate values falling within specified ranges.

Furthermore, we present two variants of lookup decomposition: non-optimized and optimized versions.
The optimized version introduces an enhancement specifically targeting [optimized short range check on 4 and 5 bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#optimized-short-range-check-on-4-and-5-bits).

## Lookup tables
### Lookup table: 10 bits
The K-bit lookup table is loaded with $2^K$ elements:
$$
\begin{array}{|c|}
\hline
 table_{idx}        \\\hline
 0               \\\hline
 1                  \\\hline
 2                \\\hline
 \vdots            \\\hline
 2^{10} - 1  \\\hline
\end{array}
$$

### Lookup table: 4, 5 and 10 bits

To implement optimized 4 and 5 bits range check, the K-bit lookup table is extended to a lookup table with $2^{10}+2^{4}+2^{5}$ 
elements.
These are loaded into the following lookup table:

$$
\begin{array}{|c|l|}
\hline
 table_{idx}   & \text{table\_{range}\_{check}}   \\\hline
 0             & 0    \\\hline
 1             & 0    \\\hline
 \vdots        & \vdots    \\\hline
 2^{10} - 1    & 0 \\\hline
 0     & 4    \\\hline
1     & 4    \\\hline
 \vdots        & \vdots    \\\hline
 2^{4} - 1  & 4 \\\hline
 0  & 5    \\\hline
1  & 5    \\\hline
 \vdots      & \vdots       \\\hline
2^{5} - 1   & 5 \\\hline
\end{array}
$$

## Decompose into $K$-bit values

Given a field element $\alpha$, these gadgets decompose it into $W$ $K$-bit windows $$\alpha = k_0 + 2^{K} \cdot k_1 + 2^{2K} \cdot k_2 + \cdots + 2^{(W-1)K} \cdot k_{W-1}$$ where each $k_i$ a $K$-bit value.

This is done using a running sum $z_i, i \in [0..W).$ We initialize the running sum $z_0 = \alpha,$ and compute subsequent terms $z_{i+1} = \frac{z_i - k_i}{2^{K}}.$ This gives us:

$$
\begin{aligned}
z_0 &= \alpha \\
    &= k_0 + 2^{K} \cdot k_1 + 2^{2K} \cdot k_2 +  2^{3K} \cdot k_3 + \cdots, \\
z_1 &= (z_0 - k_0) / 2^K \\
    &= k_1 + 2^{K} \cdot k_2 +  2^{2K} \cdot k_3 + \cdots, \\
z_2 &= (z_1 - k_1) / 2^K \\
    &= k_2 +  2^{K} \cdot k_3 + \cdots, \\
    &\vdots \\
\downarrow &\text{ (in strict mode)} \\
z_W &= (z_{W-1} - k_{W-1}) / 2^K \\
    &= 0 \text{ (because } z_{W-1} = k_{W-1} \text{)}
\end{aligned}
$$

### Strict mode
Strict mode constrains the running sum output $z_{W}$ to be zero, thus range-constraining the field element to be within $W \cdot K$ bits.

In strict mode, we are also assured that $z_{W-1} = k_{W-1}$ gives us the last window in the decomposition.
## Lookup decomposition

This gadget makes use of a lookup table to decompose a field element $\alpha$ into $K$-bit words, where $K=10$. 
Each $K$-bit word $k_i = z_i - 2^K \cdot z_{i+1}$ is range-constrained by a lookup in the [table](https://zcash.github.io/halo2/design/gadgets/decomposition.html#lookup-tables).
The lookup constraint is 
$$k_i\in~\text{the lookup table}$$

The region layout for the lookup decomposition uses a single advice column $z$, and two selectors $q_{lookup}$ and $q_{running}.$ 
$$ 
\begin{array}{|c|c|c|}
\hline
    z    & q_\mathit{lookup} & q_\mathit{running} \\\hline
\hline
  z_0    &     1      &       1     \\\hline
  z_1    &     1      &       1     \\\hline
\vdots   &   \vdots   &     \vdots  \\\hline
z_{n-1}  &     1      &       1     \\\hline
z_n      &     0      &       0     \\\hline
\end{array}
$$

For each $i,$ the lookup input expression is:
- Non-optimized: $$q_\mathit{lookup} \cdot q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1})$$
- Optimized: $$q_\mathit{lookup} \cdot (1 - q_\mathit{range\_check}) \cdot q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1})$$
The $table_{idx}$ fixed column contains values from $[0..2^K)$ ($[0..2^K + 2^4 + 2^5)$ in the optimized version). 
Looking up the above input value in the $table_{idx}$ column constrains it to be within this range.

### Short range check
In the short range check, the field element is directly witnessed.
Using two $K$-bit lookups, we can range-constrain a field element $\alpha$ to be $n$ bits, where $n \leq K.$ To do this:

1. Constrain $0 \leq \alpha < 2^K$ to be within $K$ bits using a $K$-bit lookup.
2. Constrain $0 \leq \alpha' < 2^K$ to be within $K$ bits using a $K$-bit lookup, where $\alpha' = \alpha \cdot 2^{K - n}.$


The lookup constraints are 
1. $\alpha\in~\text{the lookup table}$
2. $\alpha'\in~\text{the lookup table}$

The short variant of the lookup decomposition introduces a $q_{bitshift}$ selector. The same advice column $z$ has here been renamed to $\textsf{word}$ for clarity:
$$
\begin{array}{|c|c|c|c|}
\hline
\textsf{word} & q_\mathit{lookup} & q_\mathit{running} & q_\mathit{bitshift} \\\hline
\hline
\alpha        &     1      &      0      &       0      \\\hline
\alpha'       &     1      &      0      &       1      \\\hline
2^{K-n}       &     0      &      0      &       0      \\\hline
\end{array}
$$

Note that $2^{K-n}$ is used in the gate enabled by the $q_\mathit{bitshift}$ selector to check that $\alpha$ was shifted correctly:
$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
       2      & q_\mathit{bitshift} \cdot ((\alpha \cdot 2^{K - n}) - \alpha') \\\hline
\end{array}
$$

### Short range check with 1 fixed column (optimized)
In the optimized version, we move $2^{K-n} $ into a fixed column.

$$
\begin{array}{|c|c|c|c|c|}
\hline
\textsf{word} & q_\mathit{lookup} & q_\mathit{running} & q_\mathit{bitshift} &  fixed\_col\\\hline
\hline
\alpha        &     1      &      0      &       0   &             0   \\\hline
\alpha'       &     1      &      0      &       1    &           2^{K-n}\\\hline
\end{array}
$$


The lookup input expression is:
- Non-optimized: $$q_\mathit{lookup}  \cdot (1 - q_\mathit{running}) \cdot \textsf{word}$$
- Optimized: $$q_\mathit{lookup} \cdot (1 - q_\mathit{range\_check}) \cdot (1 - q_\mathit{running}) \cdot \textsf{word}$$
Looking up the above input value in the $table_{idx}$ column constrains it to be within this range.

## Optimized short range check on 4 and 5 bits

We further optimize the short range check for 4 and 5 bits, whereby the field element is directly witnessed. Using only 
one lookup table, we can constrain a field element $\alpha$ to be 4 (or 5)-bit.

This optimization adds some rows and one column in the lookup table. The full changes are as follows.

- Added $2^4 + 2^5$ rows in the lookup table
- Added a table_range_check column in the lookup table
- Added 2 selectors: $q_\mathit{range\_check\_4}, q_\mathit{range\_check\_5}$

The 4 and 5 bits variant of the lookup decomposition introduces two selectors $q_\mathit{range\_check\_4}$ and $q_\mathit{range\_check\_5}$. 
We can calculate $q_\mathit{range\_check}$ to see if the 4-bit and 5-bit checks are activated. 
$q_\mathit{range\_check} = 1$ if $q_\mathit{range\_check\_4}=1$ or $q_\mathit{range\_check\_5}=1$.
We further calculate the number of bits in a short field element, denoted as $num_\mathit{bits}$.
$num_\mathit{bits} = 5$ if $q_\mathit{range\_check\_5}=1$, $num_\mathit{bits} = 4$ if $q_\mathit{range\_check\_4}=1$ and $q_\mathit{range\_check\_5}=0$,
$num_\mathit{bits} = 0$ otherwise. 

$$
\begin{array}{|c|c|c|c|}
\hline
q_\mathit{range\_check\_4} & q_\mathit{range\_check\_5} & q_\mathit{range\_check} & num_\mathit{bits}\\\hline
   0   &   1    &    1  & 5 \\\hline
   1   &   0    &    1  & 4\\\hline
   0   &   0    &    0  & 0 \\\hline
   1   &   1    &    1  & 5 \\\hline
\end{array}
$$

The region layout for the lookup decomposition shows below.

On 4 bits
$$
\begin{array}{|c|c|c|}
\hline
z_{cur}  & q_\mathit{lookup} & q_\mathit{range\_check\_4} \\\hline
   \beta   &   1    &    1    \\\hline
\end{array}
$$

On 5 bits
$$
\begin{array}{|c|c|c|c|}
\hline
z_{cur}  & q_\mathit{lookup} & q_\mathit{range\_check\_5} \\\hline
\gamma &   1    &    1    \\\hline
\end{array}
$$

We have two lookup input expressions:
- Looking up the value $$q_\mathit{lookup} \cdot  q_\mathit{range\_check}\cdot z_{cur} $$ in the $table_{idx}$ column constrains it to be within this range.

- Looking up the value $$q_\mathit{lookup} \cdot q_\mathit{range\_check} \cdot num_\mathit{bits} $$
in the $\text{table\_{range}\_{check}}$ column constrains it to be within this range.

## Combined lookup expression
Since the lookup decomposition, its short variant and optimized range checks all make use of the same lookup table, we combine their lookup input expressions:

### Non-optimized lookup expression
Looking up the value 
$$q_\mathit{lookup} \cdot \left(q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1}) + (1 - q_\mathit{running}) \cdot \textsf{word} \right)$$
in the $table_{idx}$ column constrains it to be within this range.

### Optimized lookup expression

#### First lookup expression
Looking up the value 
$$q_\mathit{lookup} \cdot \left[(1 - q_\mathit{range\_check}) \cdot \left(q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1}) + (1 - q_\mathit{running}) \cdot \textsf{word}\right) + q_\mathit{range\_check}\cdot z_{cur}  \right]$$
in the $table_{idx}$ column constrains it to be within this range.
$z_i$ and $\textsf{word}$ are the same cell as $z_{cur}$ (but distinguished here for clarity of usage).
The entire expression switches between adding lookups and directly using the current value based on whether a range check is being performed, effectively integrating different types of lookups and checks within the same framework.

#### Second lookup expression
Looking up the value 
$$q_\mathit{lookup} \cdot q_\mathit{range\_check} \cdot num_\mathit{bits}$$
in the $\text{table\_{range}\_{check}}$ column constrains it to be within this range.
This expression activates when both lookup operations and range checks are active, and multiplies by the number of bits (4 or 5) involved in the check. This effectively tags the operation with its bit length, allowing the system to differentiate between 4-bit and 5-bit checks.

### Remark
The lookup range checks affected by the selector values are summarized as follows:
$$
\begin{array}{|c|c|c|c|}
\hline
q_\mathit{lookup} & q_\mathit{running} & q_\mathit{range\_check} & Remarks \\\hline
0    & \text{0 or 1}  &   \text{0 or 1}   & \text{No range check} \\\hline
1    &  1  &   0    & \text{Running sum decomposition} ((z_{i} - 2^K \cdot z_{i+1}) ~\text{is on 10 bits})   \\\hline
1    &  0  &   0     & \text{Short range check} (z_{cur} ~\text{is on 10 bits})   \\\hline
1    &  \text{0 or 1} &    1    & \text{Optimized short range check on 4 or 5 bits} \\\hline
\end{array}
$$

## Short range decomposition
For a short range (for instance, $[0, \texttt{range})$ where $\texttt{range} \leq 8$), we can range-constrain each word using a degree-$\texttt{range}$ polynomial constraint instead of a lookup: $$\RangeCheck{word}{range} = \texttt{word} \cdot (1 - \texttt{word}) \cdots (\texttt{range} - 1 - \texttt{word}).$$