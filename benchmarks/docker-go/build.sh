#!/usr/bin/env bash

set -ex

IMAGE_NAME="faas-benchmark-go"

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

docker build -t $IMAGE_NAME -f Dockerfile ..
