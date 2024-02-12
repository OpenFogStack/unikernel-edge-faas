#!/usr/bin/env bash

set -ex
OUTDIR=out

measure() {
  TARGET=$1
  OUT=$2
  CACHED=$3

  echo "measuring $TARGET OUT=$OUT CACHED=$CACHED"

  sudo ./firecracker-startup.bt 2>&1 > $OUT &
  p=$!

  cp -r targets/$TARGET dut
  pushd dut > /dev/null

  if [[ $CACHED -gt 0 ]]
  then
    echo "priming page cache"
    vmtouch -t * > /dev/null
  else
    echo "evicting page cache"
    vmtouch -e * > /dev/null
  fi

  # Wait for bpftrace to attach probes
  echo "sleeping"
  sleep 6
  echo "starting firecracker"
  firecracker --no-api --config-file config.json &> ../firecracker.log &

  # Wait for bpftrace to exit
  wait $p
  pkill firecracker || true

  echo "done"

  popd > /dev/null
  rm -r dut
}

TARGETS=$(ls targets)
printf "found targets:\n$TARGETS\n\n"

mkdir $OUTDIR

for i in $(seq 100)
do
  echo "iteration $i"
  for target in $TARGETS
  do
    CACHED=1
    measure $target "$OUTDIR/$target-cached-$i" $CACHED
  done
  for target in $TARGETS
  do
    CACHED=0
    measure $target "$OUTDIR/$target-uncached-$i" $CACHED
  done
done

