#!/usr/bin/env bash

extract() {
  grep -v abled $1 | grep -v Interrupt | grep -v Terminate | awk '{ print $2 }' | xargs -I{} echo "$2,$3,{}" >> instructions.csv
}

echo "target,language,instructions" > instructions.csv

extract container/out/go-runsc runsc go
extract container/out/node-runsc runsc node

for t in osv linux nanos
do
  for l in node go
  do
    extract out/$l-$t $t $l
  done
done
