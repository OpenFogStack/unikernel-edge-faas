#!/usr/bin/env bash

for i in $(seq 100)
do
  for t in linux nanos docker osv gvisor
  do
    for l in go node
    do
      ./bench.sh $t $l
      sleep 2
    done
  done
done