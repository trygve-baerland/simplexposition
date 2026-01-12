# Geometric interpretation

## A plain plane

A vector \\(v \in \mathbb{R}^n\\) and a point \\(p \in \mathbb{R}^n\\) describe
a plane, also in \\( \mathbb{R}^n \\), as the set of points \\( x \\) such that
\\( x - p \\) is perpendicular to \\( v \\).

That is,
\\[
  (x - p)^{\top}v = 0,
\\]
or, rearranged,
\begin{equation}
  \label{plane-definition}
  x^{\top}v = p^{\top}v \ .
\end{equation}

The figure below tries to show this relation geometrically.
Here, we've taken \\( n = 2\\), 
\\( p = (1, 1) \\), and \\( v = \left[1,1\right]^{\top}\\).
The stapled line marks the points that constitute the plane
described by \\(p\\) and \\(v\\).

<svg width="75%" height="500" viewBox="-1 -1 12 12" xmlns="http://www.w3.org/2000/svg">

  <!-- Axes -->
  <line x1="-1" y1="10" x2="10" y2="10" stroke="var(--fg)" stroke-width="0.1" marker-end="url(#arrowhead)"/>
  <line x1="0" y1="11" x2="0" y2="0" stroke="var(--fg)" stroke-width="0.1" marker-end="url(#arrowhead)"/>
  <!-- Arrow marker -->
  <defs>
    <marker
      id="arrowhead"
      viewBox="0 0 4 4"
      refX="2"
      refY="2"
      markerUnits="strokeWidth"
      markerWidth="4"
      markerHeight="4"
      orient="auto">
      <path d="M 0 0 L 4 2 L 0 4 z" fill="var(--fg)" />
    </marker>
  </defs>

  <line x1="1" y1="1" x2="9" y2="9" stroke="var(--fg)" stroke-width="0.1" stroke-dasharray="0.5 0.25"/>
  <line x1="5" y1="5" x2="7" y2="3" 
    stroke="var(--fg)" 
    stroke-width="0.1"
    marker-end="url(#arrowhead)"
  />

  <circle cx="5" cy="5" r="0.2" fill="var(--fg)" />
  <text x="4" y="6" font-size="1" fill="var(--fg)" font-style="italic">p</text>
  <text x="7.5" y="3" font-size="1" fill="var(--fg)" font-style="italic">v</text>
</svg>

We are now in a position to make an important observation:

> [!IMPORTANT]
> A plane is **not** uniquely defined by the choice of vector
> nor choice of point.

If \\( v \\) and \\( p \\) describe a plane, then the scaled vector 
\\( a v\\), with \\( a \in \mathbb{R} \\), \\( a \neq 0 \\), and any point
\\( p^* \\) on the plane also describe the same plane.

For the scaled vector see that if 

\\[
  v^\top(x - p) = 0 \Rightarrow a v^\top (x - p) = 0.
\\]

For \\( p^* \\), we add and subtract \\( v^\top p^* \\) to find that

\begin{align*}
  0 &= v^\top (x - p) \\\\
  &= v^\top (x - p) + v^\top p^* - v^\top p^* \\\\
  &= v^\top (x - p^* ) + v^\top ( p^* - p) \\\\
  &= v^\top (x - p^* ).
\end{align*}

In the last step we used that \\( p^* \\) is on the plane,
and so \\( v\top (p^* - p) = 0 \\).

A plane bisects \\( \mathbb{R}^n \\) into two half-spaces.
That is, any point in \\( \mathbb{R}^n \\) is either on the plane,
is below it, or is above it.
Moreover, the equation \eqref{plane-definition} separates these half-spaces.

> [!IMPORTANT]
> A plane splits \\( \mathbb{R}^n \\) into the half-spaces described by
> \\( v^\top x \leq v^\top p \\) and
> \\( v^\top x \geq v^\top p \\).

Looking back at our previous example, we can mark these regions with
what relation holds where.

<svg width="75%" height="500" viewBox="-1 -1 12 12" xmlns="http://www.w3.org/2000/svg" xmlns:xhtml="http://www.w3.org/2000/xhtml">

  <!-- Axes -->
  <line x1="-1" y1="10" x2="10" y2="10" stroke="var(--fg)" stroke-width="0.1" marker-end="url(#arrowhead)"/>
  <line x1="0" y1="11" x2="0" y2="0" stroke="var(--fg)" stroke-width="0.1" marker-end="url(#arrowhead)"/>

  
  <line x1="1" y1="1" x2="9" y2="9" stroke="var(--fg)" stroke-width="0.1" stroke-dasharray="0.5 0.25"/>
  <line x1="5" y1="5" x2="7" y2="3" 
    stroke="var(--fg)" 
    stroke-width="0.1"
    marker-end="url(#arrowhead)"
  />

  <circle cx="5" cy="5" r="0.2" fill="var(--fg)" />
  <text x="4" y="6" font-size="1" fill="var(--fg)" font-style="italic">p</text>
  <text x="7.5" y="3" font-size="1" fill="var(--fg)" font-style="italic">v</text>

  <foreignObject x="0" y="7" width="5" height="1">
    <xhtml:div> 
      \[ {v}^{\top} x \leq v^{\top}p \]
    </xhtml:div>
  </foreignObject>

  <foreignObject x="5" y="0" width="5" height="1">
    <xhtml:div>
      \[ {v}^{\top} x \geq v^{\top}p \]
    </xhtml:div>
  </foreignObject>
</svg>

## Set of planes as a convex polygon

Keeping in mind that a plane bisects the space it sits in into
two half-spaces, we now consider the intersection of a set of such
half-spaces.

That is, we are given a set of vector, \\( v_1, v_2, \ldots, v_m \\),
and points, \\( p_1, p_2, \ldots, p_m \\),
and we want to say something about which \\( x \in \mathbb{R}^n \\)
are contained in all the half-spaces

\begin{align*}
  v_1^\top x &\leq v_1^\top p_1, \\\\
  v_2^\top x &\leq v_2^\top p_2, \\\\
  &\cdots \\\\
  v_m^\top x &\leq v_m^\top p_m.
\end{align*}

First of all, we collect the above inequalities into
the matrix inequality

\begin{equation}
  \label{matrix-inequality}
  A x \leq b.
\end{equation}

Here, \\( A \in \mathbb{R}^{m \times n} \\) is given as

\\[
  A = \begin{pmatrix}
    v_1^\top \\\\
    v_2^\top \\\\
    \vdots   \\\\
    v_m^\top \\\\
  \end{pmatrix},
\\]

i.e. **each row of \\( A \\) is one of the vectors describing a bounding plane.**

Similarly, \\( b \in \mathbb{R}^m \\) is 

\\[
  b = \begin{pmatrix}
    v_1^\top p_1 \\\\
    v_2^\top p_2 \\\\
    \vdots       \\\\
    v_m^\top p_m
  \end{pmatrix}.
\\]

## Now what?

With some grasp of a geometric interpretation of matrix inequalities,
there are some questions we should definitely discuss the following questions.

> [!CAUTION]
> When is the convex polygon empty?
> That is, can we relate some properties of \\( A \\) and \\( b \\)
> to when there are no \\( x \\) that satisfy the inequality \\( A x \leq b \\).

> [!CAUTION]
> Similarly, can we relate properties of the matrix inequality to
> when the convex polygon is bounded?

> [!NOTE]
> This is for later, but the all-positive constraints have a different sign
> from the other constraints (they're \\( \geq \\)-constraints), and in the initial
> tableau they exactly correspond to the unknowns that are not the basic variables.
> Might be important, I don't know.