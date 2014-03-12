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

These programs are built using [`redo`](https://github.com/apenwarr/redo),
which is my preferred build system for small projects. I understand that not
everyone has `redo`, so I've included `do`, which is a small shell
implementation of `redo` that rebuilds everything rather than doing clever
dependency-tracking like `redo` proper. You can build any of these projects with

    $ cd project-dir
    $ ../do

and clean any of them with

    $ cd project-dir
    $ ../do clean

The only project that really _needs_ a build script is the presentation,
which gets built with [`pandoc`](http://johnmacfarlane.net/pandoc/) and
then compiled with `xelatex`, but each one has a build script in case
you want to poke at the commands used to build the projects.
