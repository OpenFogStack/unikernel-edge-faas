#!/usr/bin/env bash

TARGET=nanos-go

echo "Running benchmark for $TARGET"

sudo ~/src/unifaas/target/release/unifaas --registry registry &
p=$!

FILE="data.out"
FAAS="http://localhost:8123/invoke/$TARGET/hello"
# This happens to be the first ip allocated by the faas networking stack
DIRECT="http://10.100.0.2:8080/hello"

sleep 10

# cold start
curl $FAAS

sleep 1

# These are configured as single use with a long keepalive
# Start 50 of these to put some load on the faas system
# i.e. make sure we have to keep track of more than one instance
hey -n 50 -c 1 http://localhost:8123/invoke/nanos-go-single/hello

sleep 5

request() {
  URL=$1
  KIND=$2
  R=$(hey -c 1 -n 1 -o csv $URL | grep -v "response")
  C=$(echo "$R" | awk -F ',' '{ print $7 }')
  X=$(echo "$R" | awk -F ',' '{ print $1 }')
  echo "X=$X, C=$C"
  if [[ "$C" != '200' ]]
  then
    echo "Got status $C"
    exit 1
  fi

  echo "$X,$KIND" >> $FILE
}

echo "time,kind" >> $FILE
for i in $(seq 1000)
do
  request $FAAS faas
  sleep 1
  request $DIRECT direct
  sleep 1
done

sudo pkill -SIGINT unifaas
wait $p

