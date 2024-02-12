This directory contains the benchmarking setup and various related scripts.

The directories `app-<lang>` contain function code in Node, Go and Python (unused).
The directories `<target>-<lang>` contain the build environment to build the microVM/unikernel/container from the given function.
Theses directories contain a `build.sh` script, which will start the respective build.

The directory `experiments` contains the actual benchmarking scripts.

The resulting plots can be found in `experiments/<experiment>/img/`, the raw data is usually available as a csv file in `experiments/<experiment>/`
