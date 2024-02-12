#!/usr/bin/env bash

OUT=results.csv

measure() {
  L=$1
  R=$2
  I=$3
  T=$L-$R
  pushd $T

  ID=$(uuid)
  start_time="$(date -u +%s.%N)"
  sudo $R run $ID
  end_time="$(date -u +%s.%N)"
  elapsed="$(bc <<<"$end_time-$start_time" | sed 's/^\./0./')"
  echo "$T took $elapsed"
  popd
  echo "$R,$L,$elapsed" >> $OUT
}

echo "target,language,time" > $OUT

for R in runc runsc
do
  for L in node go
  do
    for i in $(seq 100)
    do
      measure $L $R $i
      sleep 5
    done
  done
done