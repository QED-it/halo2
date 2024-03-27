# Decomposition
We present two variants of lookup decomposition: non-optimized and optimized versions.
The optimized version introduces an enhancement specifically targeting [optimized short range check on 4 and 5 bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#optimized-short-range-check-on-4-and-5-bits).

This analysis of lookup decomposition (call $\texttt{lookup}$) is applied in different contexts:
- [10 bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#lookup-decomposition) (for running sum $(z_{i} - 2^K \cdot z_{i+1})$)
- [n bits](https://zcash.github.io/halo2/design/gadgets/decomposition.html#short-range-check), where $n\leq 10$. (for a single element short range check)
- [4 and 5](https://zcash.github.io/halo2/design/gadgets/decomposition.html#optimized-short-range-check-on-4-and-5-bits) bits (for optimized short range check)

Furthermore, the [range check](https://zcash.github.io/halo2/design/gadgets/decomposition.html#short-range-decomposition) 
approach (call $\texttt{range\_check}$) is used for verifying values within ranges of up to 3 bits

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

This gadget makes use of a $K$-bit lookup table to decompose a field element $\alpha$ into $K$-bit words, where $K=10$. 
Each $K$-bit word $k_i = z_i - 2^K \cdot z_{i+1}$ is range-constrained by a lookup in the $K$-bit table:

$$
\begin{array}{|c|c|c|}
\hline
 table_{idx} & table_x         & table_y         \\\hline
 0           & x_{P[0]}        & y_{P[0]}        \\\hline
 1           & x_{P[1]}        & y_{P[1]}        \\\hline
 2           & x_{P[2]}        & y_{P[2]}        \\\hline
 \vdots      & \vdots          & \vdots          \\\hline
 2^{10} - 1  & x_{P[2^{10}-1]} & y_{P[2^{10}-1]} \\\hline
\end{array}
$$

The region layout for the lookup decomposition uses a single advice column $z$, and two selectors $q_{lookup}$ and $q_{running}.$ 
The selector $q_\mathit{range\_check}$ is only used in the optimized version.
$$ 
\begin{array}{|c|c|c|c|}
\hline
    z    & q_\mathit{lookup} & q_\mathit{running} & q_\mathit{range\_check}  \\\hline
\hline
  z_0    &     1      &       1   & 0  \\\hline
  z_1    &     1      &       1   & 0  \\\hline
\vdots   &   \vdots   &     \vdots  & \vdots\\\hline
z_{n-1}  &     1      &       1    & 0 \\\hline
z_n      &     0      &       0    & 0 \\\hline
\end{array}
$$

For each $i,$ the lookup input expression is:
- Non-optimized: $$q_\mathit{lookup} \cdot q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1})~\text{belongs to}~table_{idx}$$
- Optimized: $$q_\mathit{lookup} \cdot (1 - q_\mathit{range\_check}) \cdot q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1})~\text{belongs to}~table_{idx}$$

### Short range check
In the short range check, the field element is directly witnessed.
Using two $K$-bit lookups, we can range-constrain a field element $\alpha$ to be $n$ bits, where $n \leq K.$ To do this:

1. Constrain $0 \leq \alpha < 2^K$ to be within $K$ bits using a $K$-bit lookup.
2. Constrain $0 \leq \alpha \cdot 2^{K - n} < 2^K$ to be within $K$ bits using a $K$-bit lookup.


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

where $\alpha' = \alpha \cdot 2^{K - n}.$ Note that $2^{K-n}$ is assigned to a fixed column at keygen, and copied in at proving time. This is used in the gate enabled by the $q_\mathit{bitshift}$ selector to check that $\alpha$ was shifted correctly:
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
\begin{array}{|c|c|c|c|c|c|}
\hline
\textsf{word} & q_\mathit{lookup} & q_\mathit{running} & q_\mathit{bitshift} & q_\mathit{range\_check}& fixed\_col\\\hline
\hline
\alpha        &     1      &      0      &       0   &       0 &       0   \\\hline
\alpha'       &     1      &      0      &       1    &       0 &       2^{K-n}\\\hline
\end{array}
$$


The lookup input expression is:
- Non-optimized: $$q_\mathit{lookup}  \cdot (1 - q_\mathit{running}) \cdot \textsf{word}~\text{belongs to}~table_{idx}$$
- Optimized: $$q_\mathit{lookup} \cdot (1 - q_\mathit{range\_check}) \cdot (1 - q_\mathit{running}) \cdot \textsf{word}~\text{belongs to}~table_{idx}$$

## Optimized short range check on 4 and 5 bits

We further optimize the short range check for 4 and 5 bits, whereby the field element is directly witnessed. Using one 4 (or 5)-bit lookup table, we can constrain a field element 
$\alpha$ to be 4 (or 5)-bit.

This optimization adds some rows and one column in the lookup table. The full changes are as follows.

- Added $2^4 + 2^5$ rows in the lookup table
- Added a table_range_check column in the lookup table
- Added 2 selectors: $q_\mathit{range\_check\_4}, q_\mathit{range\_check\_5}$
- Optimized range check on 1 row

### Lookup table (in the optimized version)

We extend the K-bit lookup table to a lookup table with $2^{10}+2^{4}+2^{5}$ pre-computed random generators.
These are loaded into the following lookup table:

$$
\begin{array}{|c|c|c|l|}
\hline
 table_{idx} & table_x         & table_y     & \text{table\_{range}\_{check}}   \\\hline
 0           & x_{P[0]}        & y_{P[0]}    & 0    \\\hline
 1           & x_{P[1]}        & y_{P[1]}    & 0    \\\hline
 \vdots      & \vdots          & \vdots      & \vdots    \\\hline
 2^{10} - 1  & x_{P[2^{10}-1]} & y_{P[2^{10}-1]} & 0 \\\hline
2^{10} + 0           & x_{P[0]}        & y_{P[0]}    & 4    \\\hline
 2^{10}+ 1           & x_{P[1]}        & y_{P[1]}    & 4    \\\hline
 \vdots      & \vdots          & \vdots      & \vdots    \\\hline
 2^{10} + 2^{4} - 1  & x_{P[2^{4}-1]} & y_{P[2^{4}-1]} & 4 \\\hline
 2^{10} + 2^{4} + 0           & x_{P[0]}        & y_{P[0]}    & 5    \\\hline
  2^{10} + 2^{4} + 1           & x_{P[1]}        & y_{P[1]}    & 5    \\\hline
 \vdots      & \vdots          & \vdots      & \vdots    \\\hline
  2^{10} + 2^{4} + 2^{5} - 1  & x_{P[2^{5}-1]} & y_{P[2^{5}-1]} & 5 \\\hline
\end{array}
$$


### Lookup decomposition on 4 and 5 bits
The 4 and 5 bits variant of the lookup decomposition introduces two selectors $q_\mathit{range\_check\_4}$ and $q_\mathit{range\_check\_5}$. 
We can calculate $q_\mathit{range\_check}$ to see if the 4-bit and 5-bit checks are activated. 
$q_\mathit{range\_check} = 1$ if $q_\mathit{range\_check\_4}=1$ or $q_\mathit{range\_check\_5}=1$.
We further calculate the number of bits in a short field element, denoted as $num_\mathit{bits}$.
$num_\mathit{bits} = 4$ if $q_\mathit{range\_check\_4}=1$, and $num_\mathit{bits} = 5$ if $q_\mathit{range\_check\_5}=1$.

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

Each 4-bit element $\beta$ is range-constrained by a lookup in the lookup table with $\text{table\_{range}\_{check}} = 4$ and $table_{idx}\in \{ 2^{10},\dots, 2^{10} + 2^{4}-1\}$.
Each 5-bit element $\gamma$ is range-constrained by a lookup in the lookup table with $\text{table\_{range}\_{check}} = 5$ and $table_{idx}\in \{ 2^{10} + 2^{4},\dots,   2^{10} + 2^{4} + 2^{5} - 1\}$.

The region layout for the lookup decomposition shows below.

$$
\begin{array}{|c|c|c|c|}
\hline
z_{cur} & num_\mathit{bits} & q_\mathit{lookup} & q_\mathit{range\_check} \\\hline
   \beta  &4  &   1    &    1    \\\hline
\gamma & 5 &   1    &    1    \\\hline
\end{array}
$$

We have two lookup input expressions:
$$q_\mathit{lookup} \cdot  q_\mathit{range\_check}\cdot z_{cur} ~\text{belongs to}~table_{idx}$$
$$q_\mathit{lookup} \cdot q_\mathit{range\_check} \cdot num_\mathit{bits}~\text{belongs to}~\text{table\_{range}\_{check}} $$

## Combined lookup expression
Since the lookup decomposition, its short variant and optimized range checks all make use of the same lookup table, we combine their lookup input expressions:

### Non-optimized lookup expression
$$q_\mathit{lookup} \cdot \left(q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1}) + (1 - q_\mathit{running}) \cdot \textsf{word} \right)~\text{belongs to}~table_{idx}$$

### Optimized lookup expression

#### First lookup expression
$$q_\mathit{lookup} \cdot \left[(1 - q_\mathit{range\_check}) \cdot \left(q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1}) + (1 - q_\mathit{running}) \cdot \textsf{word}\right) + q_\mathit{range\_check}\cdot z_{cur}  \right]~\text{belongs to}~table_{idx}$$
where $z_i$ and $\textsf{word}$ are the same cell as $z_{cur}$ (but distinguished here for clarity of usage).
The entire expression switches between adding lookups and directly using the current value based on whether a range check is being performed, effectively integrating different types of lookups and checks within the same framework.

#### Second lookup expression

$$q_\mathit{lookup} \cdot q_\mathit{range\_check} \cdot num_\mathit{bits}~\text{belongs to}~\text{table\_{range}\_{check}}$$
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



