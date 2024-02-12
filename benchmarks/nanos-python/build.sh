#!/usr/bin/env bash

set -ex

IMAGE_NAME="nanos-benchmark-python-buildenv"
BUILD_DIR="build"

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

rm -rf $BUILD_DIR && mkdir $BUILD_DIR
docker build -t $IMAGE_NAME -f Dockerfile ..

# docker run -it $IMAGE_NAME bash

docker run --rm -v $(realpath $BUILD_DIR):/export $IMAGE_NAME \
  bash -c "cp /images/* /export"
sudo chown $(id -u $USER):$(id -g $USER) -R $BUILD_DIR
