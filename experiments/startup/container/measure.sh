#!/usr/bin/env bash

mkdir -p out-perf

measure() {
  L=$1
  R=$2
  V=$3
  I=$4
  T=$L-$R
  if [[ "$V" = "e" ]]
  then
    C=0
  else
    C=1
  fi
  vmtouch -$V $T
  pushd $T
  sudo perf stat -- $R run $T 2>&1 | tee ../out-perf/$T-$C-$I
  popd
  
}

for i in $(seq 100)
do
  for L in node go
  do
    for R in runc runsc
    do
      for V in e t
      do
        measure $L $R $V $i
      done
    done
  done
done