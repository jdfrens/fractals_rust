# Fractals (Rust)

Generates escape-time fractals in Rust.

[![CircleCI](https://circleci.com/gh/jdfrens/fractals_rust/tree/master.svg?style=svg)](https://circleci.com/gh/jdfrens/fractals_rust/tree/master)

I'm also [generating fractals in Elixir](https://github.com/jdfrens/fractals_elixir).


## Installation

Clone, get deps, compile CLI, run QA tests, look at the pretty pictures.

```
$ git clone --recurse-submodules git@github.com:jdfrens/fractals_elixir.git
$ cd fractals_rust

# run the tests
$ cargo test

# generate a boring Mandelbrot
$ cargo run data/mandelbrot/mandelbrot-black.yml
$ open images/fractal.png
```

The app is very restricted.  I just started.
