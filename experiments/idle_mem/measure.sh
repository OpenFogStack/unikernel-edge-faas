#!/usr/bin/env bash

measure() {
  sudo /home/felix/src/unifaas/target/release/unifaas --registry registry &
  sleep 10

  ./bench.sh $1 $2

  sleep 2

  sudo pkill -SIGINT unifaas

  sleep 20
}

for t in nanos osv docker linux gvisor
do
  for l in go node
  do
    measure $t $l
  done
done