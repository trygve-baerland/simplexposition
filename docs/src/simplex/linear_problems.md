# Linear optimization problems with linear constraints

This whole section is aimed at minimizing (or maximizing) a linear
objective functional under a set of linear constraints.
In math-speak,

\begin{equation}
\label{linprog}
  \begin{cases}
    \underset{x \in \mathbb{R}^n}{\mathrm{min}} \ c^{\intercal}x \\\\
    A x = b \\\\
    x \geq 0
  \end{cases}
\end{equation}

where \\(c \in \mathbb{R}^n \\), \\(A \in \mathbb{R}^{m \times n} \\),
and \\( b \in \mathbb{R}^m \\).

The \\(x\\) in \eqref{linprog} is the vector of unknowns. 
