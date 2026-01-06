# What is this?

The solving of optimization problems has always seemed like dark magic to me.

At some point, and for some reason, I wanted to learn
linear programming in general, and the [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm)
in particular.
For me, that meant implementing it myself, and making it work on a set
of not too computationally intensive problems.

When attempting to do that I found a couple of things that inspired me
to start this project:

* The ideas behind the simplex algorithm are geometrically easy to grasp,
  but how these ideas translate to matrix operations were not straightforward
  for me to wrap my head around.
  Reading @@cormen2022introduction

I have taken a lot of inspiration from the frankly amazing blog series
['monadic parser combinators'](https://lorgonblog.wordpress.com/2007/12/02/c-3-0-lambda-and-the-first-post-of-a-series-about-monadic-parser-combinators/)
by Brian McNamara.
There, he goes through how to build up parser library that, by the end,
becomes very expressive.
The really inspirational part of this blog series is how iteratively Brian
builds up the library. Every step is firmly motivated,
and the code is a natural extension of this motivation.

Although optimization problems are a very different beast than creating
a parser library, I aim for something similar in the optimization problem space.