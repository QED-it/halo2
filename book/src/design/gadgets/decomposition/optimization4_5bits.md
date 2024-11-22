# Optimization for 4/5 bits range checks

In the ZSA Orchard circuit, we frequently perform range checks on 4, 5 and 10 bits.
While 10-bit range checks are already efficient, as they are executed with a single lookup,
we aim to optmize the range checks on 4 and 5 bits to enhance the overall circuit performance.

To achieve this, we would like to keep **one** lookup table for performance concerns.
Thus, we propose a slght modification to the lookup table.
Additionally, we need to update the [combined lookup expression](../decomposition.md#combined-lookup-expression)
to incorporate this optimization.

## Lookup table

Previously, the lookup table used in the Orchard circuit had the following structure:

$$
\begin{array}{|c|c|c|}
\hline
table\_idx & table\_x       & table\_y       \\\hline
\hline
  0        & X(S[0])        & Y(S[0])        \\\hline
  1        & X(S[1])        & Y(S[1])        \\\hline
  2        & X(S[2])        & Y(S[2])        \\\hline
 \vdots    & \vdots         & \vdots         \\\hline
 2^{10}-1  & X(S[2^{10}-1]) & Y(S[2^{10}-1]) \\\hline
\end{array}
$$

To check that a field element $v$ is a 10-bit value, we check that
$v$ belongs to the first column of the lookup table called $table\_idx$.

The $table\_x$ and $table\_y$ columns are used to efficiently evaluate
[Sinsemilla hash](../sinsemilla.md#generator-lookup-table).

In order to efficiently perform 4 and 5-bit range checks, we propose extending this table by adding new rows and a column.
The new column will indicate the type of range check could be performed
with this corresponding row (4, 5 or 10 bits).

$$
\begin{array}{|c|c|c|c|}
\hline
table\_idx & table\_x       & table\_y       & table\_range\_check \\\hline
\hline
  0        & X(S[0])        & Y(S[0])        & 0                   \\\hline
  1        & X(S[1])        & Y(S[1])        & 0                   \\\hline
  2        & X(S[2])        & Y(S[2])        & 0                   \\\hline
 \vdots    & \vdots         & \vdots         & \vdots              \\\hline
 2^{10}-1  & X(S[2^{10}-1]) & Y(S[2^{10}-1]) & 0                   \\\hline
  0        & X(S[0])        & Y(S[0])        & 4                   \\\hline
  1        & X(S[1])        & Y(S[1])        & 4                   \\\hline
  2        & X(S[2])        & Y(S[2])        & 4                   \\\hline
 \vdots    & \vdots         & \vdots         & \vdots              \\\hline
 2^4-1     & X(S[2^4-1])    & Y(S[2^4-1])    & 4                   \\\hline
  0        & X(S[0])        & Y(S[0])        & 5                   \\\hline
  1        & X(S[1])        & Y(S[1])        & 5                   \\\hline
  2        & X(S[2])        & Y(S[2])        & 5                   \\\hline
 \vdots    & \vdots         & \vdots         & \vdots              \\\hline
 2^5-1     & X(S[2^5-1])    & Y(S[2^5-1])    & 5                   \\\hline
\end{array}
$$

To check that a field element $v$ is a 4-bit value (resp. 5-bit value),
we check that $(v, 4)$ (resp. $(v, 5)$) belongs to the table formed by the columns $(table\_idx, table\_range\_check)$.

## Combined lookup expression

The [combined lookup expression](../decomposition.md#combined-lookup-expression)
must be updated to account for the new column and to leverage the optimization
for 4-bit and 5-bit range checks.

We would like to check that $(value, tag)$ belongs to the table defined by the columns $(table\_idx, table\_range\_check)$ of our lookup table where
$$\begin{cases} value = q_\mathit{lookup} \cdot  \left( (1 - q_\mathit{range\_check})  \left(q_\mathit{running} \cdot (z_i - 2^K \cdot z_{i+1}) + (1 - q_\mathit{running}) \cdot \textsf{word} \right) + q_\mathit{range\_check}  \cdot z_i \right) \\
tag = q_\mathit{lookup} \cdot q_\mathit{range\_check} \cdot num\_bits \end{cases}$$
with
$ q_\mathit{range\_check} = \begin{cases}
1 \text{ for 4 or 5-bit range checks}\\
0 \text{ otherwise}
\end{cases}$
and
$num\_bits = \begin{cases}
4 \text{ for 4-bit range checks}\\
5 \text{ for 5-bit range checks}\\
0 \text{ otherwise}
\end{cases}$.

