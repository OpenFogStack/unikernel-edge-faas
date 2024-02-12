#!/usr/bin/env bash

set -e

TARGET=$1

FLAMEGRAPH=~/src/FlameGraph

pushd targets/$TARGET
firecracker --no-api --config-file config.json < /dev/null &
popd

sleep 10

sudo perf record -F max -g -p $(pgrep firecracker) -- sleep 10
sudo perf script -i perf.data | $FLAMEGRAPH/stackcollapse-perf.pl > out.perf-folded

# Remove unknown symbols from stack
sed 's/\[unknown\];//g' -i out.perf-folded

COLORS=hot
$FLAMEGRAPH/flamegraph.pl --cp --colors $COLORS out.perf-folded > flamegraph-idle-$TARGET.svg
sudo rm out.perf-folded perf.data*

pkill firecracker

sleep 5
