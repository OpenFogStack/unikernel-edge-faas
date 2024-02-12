#!/usr/bin/env bash

IN=$(ls out)
OUT="startup-kvm.csv"

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

echo "target,language,cached,function,time" > $OUT
for f in $IN
do
  T=$(target $f)
  L=$(lang $f)
  I=$(iteration $f)
  C=$(cached $f)

  for func in kvm_vcpu_halt kvm_mmu_page_fault vmx_vcpu_run
  do
    X=$(cat out/$f | grep $func | awk '{ print $4 }')
    echo "$T,$L,$C,$func,$X" >> $OUT
  done
done

