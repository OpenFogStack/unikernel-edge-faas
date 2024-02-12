#!/usr/bin/env bash

function make_target() {
  S=$1
  D=targets/$2
  C=$3

  cp -r ../../$S $D
}

rm -rf targets && mkdir targets
make_target linux-go/build        linux-go
make_target linux-node/build      linux-node
make_target nanos-go/build        nanos-go
make_target nanos-node/build      nanos-node
make_target osv-go/build-zfs      osv-go
make_target osv-node/build-rofs   osv-node
