# Unikernel Edge FaaS

UniFaaS implements a simple FaaS system which is able to start functions using arbitrary Firecracker VMs or Docker containers with different container runtimes.
In particular this can be used to package functions as unikernels to execute on Firecracker.
There is also support for executing VMs with QEMU and QEMU-microVM, though this has not been used as part of the evaluation.

## Research

If you use this software in a publication, please cite it as:

### Text

F. Moebius, T. Pfandzelter, and D. Bermbach, **Are Unikernels Ready for Serverless on the Edge?**, 2024.

### BibTeX

```bibtex
@article{moebius2024unikernel,
    author = "Moebius, Felix and Pfandzelter, Tobias and Bermbach, David",
    title = "Are Unikernels Ready for Serverless on the Edge?",
    year = 2024,
}
```

For a full list of publications, please see [our website](https://www.tu.berlin/en/3s/research/publications).

## Building

Build using cargo

```sh
cargo build  --release
```

UniFaaS will create tap network interfaces to attach to Firecracker and therefore needs to be run with `CAP_NET_ADMIN` capabilities or simply as root.
To execute Firecracker VMs the Firecracker executable must be in the user's path, or `FIRECRACKER_EXE` needs to be set accordingly in the process environment.
To execute Docker containers you need a working installation of Docker.

```sh
sudo FIRECRACKER_EXE=/path/to/firecracker ./target/release/unifaas -r <path/to/registry>
```

UniFaaS will create tap interfaces named `faasN`, which receive addresses from the subnet `10.100.0.0/16`.
The address range is currently not configurable, so you need to make sure this does not conflict with any existing interfaces.
The tap interfaces will be removed when the process is instructed to terminate through a `SIGINT`.
If this does not work for some reason, the script `scripts/cleanup_taps.sh` can be used to remove them.

## Registry

The registry is organized as a directory containing a subdirectory with a `function.toml` file for each function.
For example, the following registry defines two functions `foo` and `bar`:

```text
registry/
├── bar
│   └── function.toml
└── foo
    └── function.toml
```

An HTTP request to `localhost:8123/invoke/foo/hello` will invoke the HTTP endpoint `/hello` on function `foo`.

### Config

The `function.toml` file defines how to run the function.
For details refer to [config.rs](./src/registry/config.rs)

The following options are available for both containers and VMs.

```text
concurrent-requests = 1
single-use = true
keepalive = 1
```

`concurrent-requests` defined how many requests a single function instance will handle at any moment

`single-use` if set to `true`, will terminate the instance after every request

`keepalive` defines how long to keep an instance running after the last request has completed

### Firecracker config

Functions executed as virtual machines additionally contain a `[vm]` configuration table, which should be mostly self-explanatory.

```toml
[vm]
kernel = "path/to/kernel.img"
image = "path/to/rootfs.img"
memory = 512
cpus = 1
cmdline = "net.ip=%ip net.gw=%gateway net.mask=%netmask callback.url=%callback"
hypervisor = { type = "firecracker", copy-rootfs = true }
```

The patterns `%ip`, `%gateway`, `%netmask` and `%callback` in the command line are substitute with instance specific values.
When the instance is ready to handle the first request it must invoke the HTTP callback passed as `%callback`.

### Container config

Functions executed as containers additionally contain a `[container]` configuration table.

```toml
[container]
image = "image-name"
memory = 512
runtime = "runsc"
```

The `runtime` entry is optional and defaults to `runc`. It can be used to start the container with a particular container runtime as specified in `/etc/docker/daemon.json`.
