#!/usr/bin/env bash

T=$1
L=$2
S=$3
TARGET=$T-$L

echo "Running benchmark for $TARGET starting at $S"

mkdir -p out
COUNT=$S

kill_unifaas() {
  sudo pkill -SIGINT unifaas
  sleep 40
}

while [[ $COUNT -lt 101 ]]
do
  sudo ~/src/unifaas/target/release/unifaas --registry registry &
  sleep 10
  echo "COUNT=$COUNT"
  R=$(hey -c $COUNT -n $COUNT -t 0 -o csv http://localhost:8123/invoke/$TARGET/hello | grep -v "response") 
  for r in $R
  do
    C=$(echo "$r" | awk -F ',' '{ print $7 }')
    if [[ "$C" != '200' ]]
    then
      echo "Got status $C"
      kill_unifaas
      exit 1
    fi
  done
  for r in $R
  do
    echo "r=$r"
    C=$(echo "$r" | awk -F ',' '{ print $7 }')
    X=$(echo "$r" | awk -F ',' '{ print $1 }')
    echo "X=$X, C=$C"
    echo "$T,$L,$COUNT,$X" >> out/$TARGET
  done
  COUNT=$((COUNT+10))
  kill_unifaas
done

exit 0