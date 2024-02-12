#!/usr/bin/env bash

IN=$(ls out-perf)
OUT="startup-instructions.csv"

target() {
  echo "$1" | grep -Eoe 'osv|nanos|linux'
}

lang() {
  echo "$1" | grep -Eoe 'go|node'
}

iteration() {
  echo "$1" | grep -Eoe '[0-9]+'
}

cached() {
  echo "$1" | grep 'uncached' > /dev/null
  if [[ $? -eq 0 ]]
  then
    echo "0"
  else
    echo "1"
  fi
}

echo "target,language,cached,instructions" > $OUT
for f in $IN
do
  T=$(target $f)
  L=$(lang $f)
  I=$(iteration $f)
  C=$(cached $f)

  X=$(cat out-perf/$f | grep instructions | awk '{ print $1 }'| sed 's/,//g')
  echo "$T,$L,$C,$X" >> $OUT
done

pushd container
./extract.sh
cat results-perf.csv >> ../$OUT
popd

