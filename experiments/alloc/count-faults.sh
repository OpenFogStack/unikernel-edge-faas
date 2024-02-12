#!/usr/bin/env bash

sudo bpftrace -e 'kfunc:kvm_mmu_page_fault /strcontains(comm, "fc_vcpu")/ { @ = count() }' -c "$(which curl) 10.10.0.2:8080/alloc"

# gvisor, replace pid with pid from runsc-sandbox process
# sudo bpftrace -e 'kfunc:kvm_mmu_page_fault /pid == 132967/ { @ = count() }' -c "$(which curl) 172.17.0.2:8080/alloc"