#!/usr/bin/env bash

for t in osv nanos linux
do
  for l in node go
  do
    ./bench.sh $t $l
  done
done
