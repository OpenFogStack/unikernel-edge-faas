#!/usr/bin/env bash

measure() {
  L=$1
  R=$2
  I=$3
  T=$L-$R
  pushd $T

  start_time="$(date -u +%s.%N)"

  for i in $(seq $I)
  do
    sudo $R run $(uuid) &
    echo "one started"
  done

  mid_time="$(date -u +%s.%N)"
  mid_elapsed="$(bc <<<"$mid_time-$start_time" | sed 's/^\./0./')"

  for i in $(seq $I)
  do
    wait
    echo "one finished"
  done

  end_time="$(date -u +%s.%N)"
  elapsed="$(bc <<<"$end_time-$start_time" | sed 's/^\./0./')"

  popd

  echo "$T with $I instances took $elapsed s"
  echo "[start took $mid_elapsed s]"
  echo "$R,$L,$I,$elapsed" >> results.csv
}

echo "target,language,instances,time" > results.csv

for r in runc runsc
do
  for l in node go
  do
    for j in $(seq 1 10)
    do
      for i in $(seq 10 10 100)
      do
        measure $l $r $i
        sleep 10
      done
    done
  done
done
