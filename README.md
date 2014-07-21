Rust Examples
=============

These are small snippets of the Rust, in varying degrees of elaborateness,
meant to accompany the presentation given in `presentation/rust.pdf`. I am not
a particularly experienced Rust programmer, so these programs may differ
significantly from conventional Rust style or idiom, or may be written in
a suboptimal way.

Right now, the examples include:

- An interpreter for the untyped lambda-calculus
- A regular-expression matcher

Build System
------------

The presentation is built using [`redo`](https://github.com/apenwarr/redo),
which is my preferred build system for small projects. I understand that not
everyone has `redo`, so I've included `do`, which is a small shell
implementation of `redo` that rebuilds everything rather than doing clever
dependency-tracking like `redo` proper. You can build the presentation with

    $ cd presentation
    $ ../do

and clean it with

    $ cd presentation
    $ ../do clean

The Rust projects have been updated to use [Cargo](http://crates.io/), the
proper Rust build system.
