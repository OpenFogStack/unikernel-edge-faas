#!/usr/bin/env bash

FILE=fib.csv

echo "target,time,attempt" > $FILE

bench() {
  TARGET=$1
  N=$2

  URL="http://localhost:8123/invoke/$TARGET/fib?n=$N"

  # Start instance so we get a subsequent warm start
  curl http://localhost:8123/invoke/$TARGET/hello

  for attempt in first second
  do
    sleep 2
    R=$(hey -c 1 -n 1 -o csv $URL | grep -v "response")
    C=$(echo "$R" | awk -F ',' '{ print $7 }')
    X=$(echo "$R" | awk -F ',' '{ print $1 }')
    echo "X=$X, C=$C"
    if [[ "$C" != '200' ]]
    then
      echo "Got status $C"
      exit 1
    fi

    echo "$TARGET,$X,$attempt" >> $FILE
  done
}
  
for i in $(seq 1)
do
  for t in linux nanos docker osv gvisor
  do
    bench $t-go 1000000000
    sleep 6
  done
done
