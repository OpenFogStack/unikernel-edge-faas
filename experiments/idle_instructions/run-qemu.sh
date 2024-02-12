#!/usr/bin/env bash

set -ex

CMDLINE="--ip=eth0,10.10.0.2,255.255.255.0 --defaultgw=10.10.0.1 --nameserver=10.10.0.1 --rootfs=rofs --env=TERM=unknown /benchmark.so http://10.10.0.1:3000/ready"
# CMDLINE="--verbose $CMDLINE"

qemu-system-x86_64 \
 -enable-kvm -cpu host -m 512m -smp 1 \
 -kernel targets/osv-go/loader-stripped.elf -append "$CMDLINE" \
 -no-reboot -nodefaults -no-user-config -nographic \
 -serial stdio \
 -drive id=test,file=targets/osv-go/benchmark.raw,format=raw,if=none \
 -device virtio-blk-pci,drive=test \
 -netdev tap,id=n0,ifname=tap0,script=no,downscript=no \
 -device virtio-net-pci,netdev=n0

