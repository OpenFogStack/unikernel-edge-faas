#!/usr/bin/env bash

for i in $(seq 99)
do
  for t in linux nanos docker osv gvisor
  do
    ./bench.sh $t go
    sleep 6
  done
done