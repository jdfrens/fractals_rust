#!/bin/zsh

for i in data/mandelbrot/*.yml; do
  cargo run $i
done
