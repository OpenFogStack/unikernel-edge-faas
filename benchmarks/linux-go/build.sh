#!/usr/bin/env bash

# Create an alpine linux rootfs from an alpine docker image

ROOTFS_IMAGE=alpine-benchmark-rootfs
ROOTFS_MOUNT=benchmark-rootfs-export
ROOTFS_DISK=build/benchmark-rootfs.ext4

set -ex

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

mkdir -p build

# Cleanup artifacts from previous build
sudo rm -rf $ROOTFS_MOUNT $ROOTFS_DISK

# Create rootfs and mount it, this is then passed as bind mount to docker
mkdir $ROOTFS_MOUNT
dd if=/dev/zero of=$ROOTFS_DISK bs=1M count=1000
mkfs.ext4 $ROOTFS_DISK
sudo mount $ROOTFS_DISK $ROOTFS_MOUNT

# Create rootfs from docker container
# The container runs scripts/extract-rootfs.sh to copy its contents to
# the mounted rootfs
docker build -t $ROOTFS_IMAGE -f Dockerfile ..
docker run -v $(pwd)/$ROOTFS_MOUNT:/$ROOTFS_MOUNT \
  -e ROOTFS_MOUNT="/$ROOTFS_MOUNT" $ROOTFS_IMAGE

# Cleanup
sudo umount $ROOTFS_MOUNT && rm -rf $ROOTFS_MOUNT

# Download prebuilt 5.10 kernel for firecracker
ARCH=x86_64
wget https://s3.amazonaws.com/spec.ccfc.min/firecracker-ci/v1.5/${ARCH}/vmlinux-5.10.186 -O build/vmlinux
