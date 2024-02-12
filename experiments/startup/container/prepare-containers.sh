
sudo rm -rf node-runsc node-runc go-runsc go-runc

mkdir node-runsc
pushd node-runsc

mkdir rootfs
docker export $(docker create faas-benchmark-node) | tar -xf - -C rootfs  
mv rootfs/root/main.js rootfs/main.js
runsc spec -- /usr/bin/node /main.js
popd

mkdir node-runc
pushd node-runc

mkdir rootfs
docker export $(docker create faas-benchmark-node) | tar -xf - -C rootfs  
mv rootfs/root/main.js rootfs/main.js
runc spec
sed -i 's;"sh";"/usr/bin/node", "/main.js";' config.json
popd

mkdir go-runsc
pushd go-runsc

mkdir rootfs
docker export $(docker create faas-benchmark-go) | tar -xf - -C rootfs  
mv rootfs/root/benchmark rootfs/benchmark
runsc spec -- /benchmark
popd

mkdir go-runc
pushd go-runc

mkdir rootfs
docker export $(docker create faas-benchmark-go) | tar -xf - -C rootfs  
mv rootfs/root/benchmark rootfs/benchmark
runc spec
sed -i 's;"sh";"/benchmark";' config.json
popd

