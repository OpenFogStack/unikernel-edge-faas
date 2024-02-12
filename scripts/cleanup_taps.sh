#!/usr/bin/env bash

set -x

ip link | grep faas | sed -e "s/.*\(faas[0-9]*\).*/\1/g" | xargs -n1 sudo ip link del
