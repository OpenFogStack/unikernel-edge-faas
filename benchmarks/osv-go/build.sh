#!/usr/bin/env bash

set -ex

IMAGE_NAME="osv-benchmark-buildenv"
BUILD_DIR="build"

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

rm -rf $BUILD_DIR build-rofs build-zfs
mkdir  $BUILD_DIR build-rofs build-zfs
docker build -t $IMAGE_NAME -f Dockerfile ..

# docker run -it $IMAGE_NAME bash

docker run --rm -v $(realpath $BUILD_DIR):/export $IMAGE_NAME \
   bash -c "cp /images/* /export"
sudo chown $(id -u $USER):$(id -g $USER) -R $BUILD_DIR

# mv $BUILD_DIR/benchmark-rofs.img build-rofs/benchmark.img
mv $BUILD_DIR/benchmark-rofs.raw build-rofs/benchmark.raw
mv $BUILD_DIR/loader-stripped-rofs.elf build-rofs/loader-stripped.elf

# mv $BUILD_DIR/benchmark-zfs.img build-zfs/benchmark.img
mv $BUILD_DIR/benchmark-zfs.raw build-zfs/benchmark.raw
mv $BUILD_DIR/loader-stripped-zfs.elf build-zfs/loader-stripped.elf
rm -r $BUILD_DIR
