#!/usr/bin/env bash

set -e

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

rm -rf registry && mkdir -p registry/{nanos-go,nanos-go-single,osv-go,linux-go,docker-go,nanos-node,osv-node,linux-node,docker-node,gvisor-go,gvisor-node}

KEEPALIVE=100000
SINGLEUSE=false

cat >registry/nanos-go/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/nanos-go/benchmark.img"
memory = 512
cmdline = "net.ip=%ip net.gw=%gateway net.mask=%netmask callback.url=%callback"
kernel = "../../targets/nanos-go/kernel.img"
hypervisor = { type = "firecracker" }
eof

cat >registry/nanos-go-single/function.toml <<eof
concurrent-requests = 1
single-use = true
keepalive = 100000

[vm]
image = "../../targets/nanos-go/benchmark.img"
memory = 512
cmdline = "net.ip=%ip net.gw=%gateway net.mask=%netmask callback.url=%callback"
kernel = "../../targets/nanos-go/kernel.img"
hypervisor = { type = "firecracker" }
eof

cat >registry/nanos-node/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/nanos-node/benchmark.img"
memory = 512
cmdline = "net.ip=%ip net.gw=%gateway net.mask=%netmask callback.url=%callback"
kernel = "../../targets/nanos-node/kernel.img"
hypervisor = { type = "firecracker" }
eof

cat >registry/osv-go/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/osv-go/benchmark.raw"
memory = 512
cmdline = "--ip=eth0,%ip,%netmask --defaultgw=%gateway --nameserver=%gateway --rootfs=rofs /benchmark.so %callback"
kernel = "../../targets/osv-go/loader-stripped.elf"
hypervisor = { type = "firecracker" }
eof

cat >registry/osv-node/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/osv-node/benchmark.raw"
memory = 512
cmdline = "--ip=eth0,%ip,%netmask --defaultgw=%gateway --nameserver=%gateway --rootfs=rofs /node main.js %callback"
kernel = "../../targets/osv-node/loader-stripped.elf"
hypervisor = { type = "firecracker" }
eof

cat >registry/linux-go/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/linux-go/benchmark-rootfs.ext4"
memory = 512
# console=ttyS0
cmdline = "callback=%callback ip=%ip::%gateway:%netmask:vm:eth0:off ro noapic acpi=off pci=off nomodule random.trust_cpu=on"
kernel = "../../targets/linux-go/vmlinux"
hypervisor = { type = "firecracker" }
eof

cat >registry/linux-node/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[vm]
image = "../../targets/linux-node/benchmark-rootfs.ext4"
memory = 512
# console=ttyS0
cmdline = "callback=%callback ip=%ip::%gateway:%netmask:vm:eth0:off ro noapic acpi=off pci=off nomodule random.trust_cpu=on"
kernel = "../../targets/linux-node/vmlinux"
hypervisor = { type = "firecracker" }
eof

cat >registry/docker-go/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[container]
image = "faas-benchmark-go"
memory = 512
eof

cat >registry/docker-node/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[container]
image = "faas-benchmark-node"
memory = 512
eof

cat >registry/gvisor-go/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[container]
image = "faas-benchmark-go"
memory = 512
runtime = "runsc-kvm"
eof

cat >registry/gvisor-node/function.toml <<eof
concurrent-requests = 1
single-use = $SINGLEUSE
keepalive = $KEEPALIVE

[container]
image = "faas-benchmark-node"
memory = 512
runtime = "runsc-kvm"
eof
