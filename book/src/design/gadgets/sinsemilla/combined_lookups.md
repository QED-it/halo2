# Combined lookup table

Recall the lookup table designed for [range check](https://zcash.github.io/halo2/design/gadgets/decomposition.html) 
validates if an element is within a range of up to 10 bits, as detailed below.

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

Recall the lookup table $\mathcal{P}$ designed for [Sinsemilla hash](https://zcash.github.io/halo2/design/gadgets/sinsemilla.html) 
validates if $(m_{i+1},\, x_{P,i},\, y_{P,i}) \in \mathcal{P}$, as detailed below.

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

To enhance efficiency and reduce redundancy, a single combined table is utilized instead of two separate tables, 
each containing at least 10-bit rows, for the two lookup arguments mentioned above. The combined table is outlined below.

$$
\begin{array}{|c|c|c|l|}
\hline
 table_{idx}   & table_x         & table_y    & \text{table\_{range}\_{check}}   \\\hline
 0             & x_{P[0]}        & y_{P[0]}   & 0    \\\hline
 1             & x_{P[1]}        & y_{P[1]}   & 0    \\\hline
 \vdots        & \vdots  & \vdots  & \vdots    \\\hline
 2^{10} - 1    & x_{P[2^{10}-1]} & y_{P[2^{10}-1]} & 0 \\\hline
 0     & x_{P[0]}        & y_{P[0]}  & 4    \\\hline
1     & x_{P[1]}        & y_{P[1]}   & 4    \\\hline
 \vdots        & \vdots  & \vdots & \vdots    \\\hline
 2^{4} - 1  & x_{P[2^{4} - 1]}        & y_{P[2^{4} - 1]}   & 4 \\\hline
0  & x_{P[0]}        & y_{P[0]}   & 5    \\\hline
1  & x_{P[1]}        & y_{P[1]}   & 5    \\\hline
 \vdots      & \vdots   & \vdots & \vdots     \\\hline
2^{5} - 1   & x_{P[2^{5} - 1]}        & y_{P[2^{5} - 1]}   & 5 \\\hline
\end{array}
$$

- For range checks, the columns $table_{idx}$ and table_range_check
are used. Validation that lookup 
values lie within the predetermined range is achieved by comparing these 'lookup values' against the paired columns 
($table_{idx}$, table_range_check) in the combined lookup table.
- For Sinsemilla lookups, the columns $table_{idx}$, $table_x$ and $table_y$ are employed. This process ensures that 
lookup values are within the acceptable boundaries by matching these 'lookup values' against the ($table_{idx}$, $table_x$, $table_y$) 
columns, also within the combined lookup table.

