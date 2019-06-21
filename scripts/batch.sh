#!/bin/zsh

for i in $@; do
  cargo run $i
done
