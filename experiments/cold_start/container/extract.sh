#!/usr/bin/env bash

extract() {
  L=$1
  R=$2
  C=$3
  ls out-perf/$L-$R-$C-* | xargs grep instructions | awk '{ print $2 }' | xargs -I{} echo "$R,$L,$C,{}" >> results-perf.csv
}

extract node runc 0
extract node runc 1
extract node runsc 0
extract node runsc 1
extract go runc 0
extract go runc 1
extract go runsc 0
extract go runsc 1
