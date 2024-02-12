#!/usr/bin/env bash

# for t in nanos osv linux docker gvisor
for t in gvisor-kvm
do
  for l in go node
  do
    ./bench.sh $t $l 10
  done
done