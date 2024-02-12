#!/usr/bin/env bash

IN=$(ls out)
OUT="startup-times.csv"

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

echo "target,language,cached,setup,startup" > $OUT
for f in $IN
do
  T=$(target $f)
  L=$(lang $f)
  I=$(iteration $f)
  C=$(cached $f)

  
  X=$(cat out/$f | grep "vcpu_run first" | awk '{ print $8 }')
  Y=$(cat out/$f | grep "tcp_recvmsg"    | awk '{ print $8 }')

  SETUP=$(echo "$X / 1000000" | bc -l)
  STARTUP=$(echo "($Y - $X) / 1000000" | bc -l)

  echo "$T,$L,$C,$SETUP,$STARTUP" >> $OUT

  # printf "$f\tT=$T\tL=$L\tI=$I\tC=$C\tSETUP=$SETUP\tSTARTUP=$STARTUP\n"
done

