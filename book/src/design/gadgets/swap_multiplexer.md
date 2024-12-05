# Swap and Multiplexer

Swap gates are utilized to verify the Merkle path of commitment notes.
Multiplixer are utilized in OrchardZSA to evaluate note and value commitments
based on the transaction type (ZSA or native transaction).

## Swap

Given an input pair of field elements $(a,b)$ and a boolean value $swap$,
we would like to return
- $(a, b)$ if $swap = 0$, and
- $(b, a)$ if $swap = 1$.

### Layout

Let $(a_s, b_s) = SWAP(a, b, swap) = \begin{cases}
(a, b) \text{ if } swap = 0 \\
(b, a) \text{ if } swap = 1
\end{cases}$.

We set all values $a_s$, $b_s$, $a$, $b$ and $swap$ on the same row in the layout.

$$
\begin{array}{|c|c|c|c|c|c|}
\hline
a_0 & a_1 & a_2 & a_3 &  a_4 & q_\texttt{swap} \\\hline
a   & b   & a_s & b_s & swap & 1               \\\hline
\end{array}
$$

### Constraints

$$
\begin{array}{|c|l|}
\hline
\text{Degree} & \text{Constraint} \\\hline
3 & q_\texttt{swap} \cdot \BoolCheck{swap} = 0 \\\hline
3 & q_\texttt{swap} \cdot (a_s - \Ternary{swap}{b}{a}) = 0 \\\hline
3 & q_\texttt{swap} \cdot (b_s - \Ternary{swap}{a}{b}) = 0 \\\hline
\end{array}
$$

where $\Ternary{swap}{x}{y} = swap \cdot x + (1 - swap) y$.

## Multiplexer

Given two curve points $(x_0, y_0)$ and $(x_1, y_1)$ and a boolean value $choice$,
we would like to return
- $(x_0, y_0)$ if $choice=0$, and
- $(x_1, y_1)$ if $choice=1$.

To perform this $MUX$ operation, we will call twice the $SWAP$ gates once for each coordinate.
Let $(x, y) = MUX((x_0, y_0), (x_1, y_1), choice)$.
We have
$$x=SWAP(x_0, x_1, choice)[0]$$
$$y=SWAP(y_0, y_1, choice)[0]$$

### Layout

$$
\begin{array}{|c|c|c|c|c|c|}
\hline
a_0 & a_1 & a_2                       & a_3                       &  a_4   & q_\texttt{swap} \\\hline
x_0 & x_1 & Swap(x_0, x_1, choice)[0] & Swap(x_0, x_1, choice)[1] & choice & 1               \\\hline
y_0 & y_1 & Swap(y_0, y_1, choice)[0] & Swap(y_0, y_1, choice)[1] & choice & 1               \\\hline
\end{array}
$$
