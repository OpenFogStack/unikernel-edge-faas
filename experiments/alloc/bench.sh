#!/usr/bin/env bash

T=$1
L=$2
TARGET=$T-$L

echo "Running benchmark for $TARGET"

mkdir -p out

FILE="out/$TARGET"
URL="http://localhost:8123/invoke/$TARGET/alloc"

curl http://localhost:8123/invoke/$TARGET/hello

for attempt in first second
do
  sleep 5
  R=$(hey -c 1 -n 1 -o csv $URL | grep -v "response")
  C=$(echo "$R" | awk -F ',' '{ print $7 }')
  X=$(echo "$R" | awk -F ',' '{ print $1 }')
  echo "X=$X, C=$C"
  if [[ "$C" != '200' ]]
  then
    echo "Got status $C"
    exit 1
  fi

  echo "$T,$X,$attempt" >> $FILE
done
