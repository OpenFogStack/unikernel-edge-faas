#!/usr/bin/env bash

# https://github.com/brendangregg/FlameGraph
FLAMEGRAPH=~/src/FlameGraph
# TARGET=$1
# TITLE=$2

# sudo perf record -F max -g $(which firecracker) --no-api --config-file configs/$TARGET.json < /dev/null
# sudo perf script -i perf.data | $FLAMEGRAPH/stackcollapse-perf.pl > out.perf-folded

# # Remove unknown symbols from stack
# sed 's/\[unknown\];//g' -i out.perf-folded
# # Filter out process exit and kvm cleanup
# sed '/do_exit/d' -i out.perf-folded

# COLORS=hot
# $FLAMEGRAPH/flamegraph.pl --title="$TITLE" --cp --colors $COLORS out.perf-folded > perf.svg
# sudo rm out.perf-folded perf.data*

set -x
OUTDIR=flamegraphs

measure() {
  TARGET=$1
  OUT=$2
  CACHED=$3

  echo "measuring $TARGET OUT=$OUT CACHED=$CACHED"

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

  sudo perf record -F max -g $(which firecracker) --no-api --config-file config.json < /dev/null
  sudo perf script -i perf.data | $FLAMEGRAPH/stackcollapse-perf.pl > out.perf-folded

  # Remove unknown symbols from stack
  sed 's/\[unknown\];//g' -i out.perf-folded
  # Filter out process exit and kvm cleanup
  sed '/do_exit/d' -i out.perf-folded

  COLORS=hot
  $FLAMEGRAPH/flamegraph.pl --cp --colors $COLORS out.perf-folded > perf.svg
  sudo rm out.perf-folded perf.data*

  mv perf.svg ../$OUT.svg

  echo "done"

  popd > /dev/null
  rm -r dut
}

TARGETS=$(ls targets)
printf "found targets:\n$TARGETS\n\n"

mkdir $OUTDIR

for target in $TARGETS
do
  CACHED=1
  measure $target "$OUTDIR/flamegraph-$target-cached" $CACHED
done
for target in $TARGETS
do
  CACHED=0
  measure $target "$OUTDIR/flamegraph-$target-uncached" $CACHED
done

