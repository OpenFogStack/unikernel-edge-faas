#!/usr/bin/env bash

set -ex

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

CMDLINE="--ip=eth0,10.12.0.2,255.255.255.0 --defaultgw=10.12.0.1 --nameserver=10.12.0.1 --rootfs=rofs /benchmark.so -- http://10.12.0.1:4000/ready"
# CMDLINE="--verbose $CMDLINE"

MEM=4g

qemu-system-x86_64 \
 -enable-kvm -cpu host -m $MEM -smp 1 \
 -kernel build/loader-stripped.elf -append "$CMDLINE" \
 -no-reboot -nodefaults -no-user-config -nographic \
 -serial stdio \
 -drive id=test,file=build/benchmark.raw,format=raw,if=none \
 -device virtio-blk-pci,drive=test \
 -netdev tap,id=n0,ifname=osv,script=no,downscript=no \
 -device virtio-net-pci,netdev=n0

