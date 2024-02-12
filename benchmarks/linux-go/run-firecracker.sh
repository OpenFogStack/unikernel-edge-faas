#!/usr/bin/env bash

set -ex

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

IP_ARG="ip=10.12.0.2::10.12.0.1:255.255.255.252:vm:eth0:off"
CMDLINE="ro noapic acpi=off pci=off nomodule random.trust_cpu=on $IP_ARG"
CMDLINE="console=ttyS0 $CMDLINE callback=http://10.12.0.1:4000/ready"

cat > machine.json <<eof
{
  "boot-source": {
    "kernel_image_path": "build/vmlinux",
    "boot_args": "$CMDLINE"
  },
  "drives": [
    {
      "drive_id": "rootfs",
      "path_on_host": "build/benchmark-rootfs.ext4",
      "is_root_device": true,
      "is_read_only": false
    }
  ],
  "network-interfaces": [
    {
      "iface_id": "eth0",
      "guest_mac": "AA:FC:00:00:00:01",
      "host_dev_name": "osv"
    }
  ],
  "machine-config": {
    "vcpu_count": 1,
    "mem_size_mib": 1024
  }
}
eof

firecracker --no-api --config-file machine.json
rm machine.json
