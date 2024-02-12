#!/usr/bin/env bash

mkdir -p out

measure() {
  L=$1
  R=$2
  pushd targets/$R-$L
  sleep 120 && sudo pkill firecracker &
  sudo perf stat -e instructions --delay=15000 -I 1000 --interval-count 100 -x ' ' -- $(which firecracker) --no-api --config-file config.json 2> ../../out/$L-$R 
  popd
}

for l in node go
do
  for t in osv linux nanos
  do
    measure $l $t
    sleep 10
  done
done
