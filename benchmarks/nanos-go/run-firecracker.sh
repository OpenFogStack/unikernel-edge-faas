#!/usr/bin/env bash

set -e

script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
cd $script_dir

IMAGE=build/benchmark.img
KERNEL=build/kernel.img

cat > machine.json <<eof
{
  "boot-source": {
    "kernel_image_path": "$KERNEL",
    "boot_args": "net.ip=10.12.0.2 net.gw=10.12.0.1 net.mask=255.255.255.0 callback.url=http://10.12.0.1:4000/ready"
  },
  "drives": [
    {
      "drive_id": "rootfs",
      "path_on_host": "$IMAGE",
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
