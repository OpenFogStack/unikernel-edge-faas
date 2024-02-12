#!/usr/bin/env bash

mkdir -p out
set -x

measure() {
  L=$1
  R=$2
  T=$L-$R
  pushd $T
  sudo perf stat -e instructions -x ' ' --delay=15000 -I 1000 --interval-count 100 -- $R run $(uuid) 2> ../out/$T
  popd
}

measure go runsc
measure node runsc
# measure $1 $2