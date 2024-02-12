#!/usr/bin/env bash

function make_target() {
  S=$1
  D=targets/$2
  C=$3

  cp -r ../../$S $D
  cp configs/$C $D/config.json
}

rm -rf targets && mkdir targets
make_target linux-go/build        linux-go          linux-go.json
make_target linux-node/build      linux-node        linux-node.json
make_target nanos-go/build        nanos-go          nanos-go.json
make_target nanos-node/build      nanos-node        nanos-node.json
make_target osv-go/build-rofs     osv-go            osv-go.json
make_target osv-node/build-rofs   osv-node          osv-node.json
