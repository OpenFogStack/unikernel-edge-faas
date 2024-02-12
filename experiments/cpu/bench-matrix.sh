#!/usr/bin/env bash

FILE=matrix.csv

echo "target,time,attempt" > $FILE

for i in $(seq 100)
do
  for TARGET in nanos linux osv docker gvisor
  do
    # Start instance so we get a subsequent warm start
    curl http://localhost:8123/invoke/$TARGET-go/hello
    TIME1=$(curl http://localhost:8123/invoke/$TARGET-go/matrix?n=1024 | sed 's/Done (\(.*\))/\1/')
    sleep 0.5
    TIME2=$(curl http://localhost:8123/invoke/$TARGET-go/matrix?n=1024 | sed 's/Done (\(.*\))/\1/')
    echo "$TARGET took $TIME1 and $TIME2"
    echo "$TARGET,$TIME1,first" >> $FILE
    echo "$TARGET,$TIME2,second" >> $FILE
    sleep 5
  done
done
