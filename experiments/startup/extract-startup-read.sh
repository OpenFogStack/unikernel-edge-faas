#!/usr/bin/env bash

IN=$(ls out)
OUT="startup-read.csv"

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

kernel() {
  if [[ $1 == "nanos" ]]
  then
    echo "kernel.img"
  elif [[ $1 == "linux" ]]
  then
    echo "vmlinux"
  elif [[ $1 == "osv" ]]
  then
    echo "loader-stripped.elf"
  else
    echo "unknown target $1" >2
    exit 1
  fi
}


rootfs() {
  if [[ $1 == "nanos" ]]
  then
    echo "benchmark.img"
  elif [[ $1 == "linux" ]]
  then
    echo "benchmark-rootfs.ext4"
  elif [[ $1 == "osv" ]]
  then
    echo "benchmark.raw"
  else
    echo "unknown target $1" >2
    exit 1
  fi
}

echo "target,language,cached,file,time" > $OUT
for f in $IN
do
  T=$(target $f)
  L=$(lang $f)
  I=$(iteration $f)
  C=$(cached $f)

  k=$(kernel $T)
  r=$(rootfs $T)
  echo "target $T $k $r"
  K=$(cat out/$f | grep "file_time\[$k\]" | head -n 1 | awk ' {print $2} ')
  R=$(cat out/$f | grep "file_time\[$r\]" | head -n 1 | awk ' {print $2} ')


  echo "$T,$L,$C,kernel,$K" >> $OUT
  echo "$T,$L,$C,rootfs,$R" >> $OUT

  # printf "$f\tT=$T\tL=$L\tI=$I\tC=$C\tSETUP=$SETUP\tSTARTUP=$STARTUP\n"
done

