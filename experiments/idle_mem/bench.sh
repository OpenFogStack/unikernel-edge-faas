#!/usr/bin/env bash

T=$1
L=$2
TARGET=$T-$L

echo "Running benchmark for $TARGET"

mkdir -p out

function get_available_memory_mb() {
  cat /proc/meminfo | grep MemAvailable | awk '{ print $2 }' | xargs -I {} echo "{} / 1024" | bc
}

FILE="out/$TARGET"
URL="http://localhost:8123/invoke/$TARGET/hello"
BASEMEM=$(get_available_memory_mb)
COUNT=0
INCREMENT=10
ITER=10

for i in $(seq $ITER)
do
  sleep 2
  echo "Starting $INCREMENT more instances"
  hey -n $INCREMENT -c $INCREMENT $URL > /dev/null
  COUNT=$((COUNT + INCREMENT))
  sleep 10
  AVAILABLE=$(get_available_memory_mb)
  MEM=$((BASEMEM - AVAILABLE))
  echo "count=$COUNT, mem=$MEM"
  printf "$T,$L,$COUNT,$MEM\n" >> $FILE
done
