.PHONY: module
module: benchmark.so
	echo '/benchmark.so: $${MODULE_DIR}/benchmark.so' > usr.manifest
	echo '/static_file: $${MODULE_DIR}/static_file' >> usr.manifest

benchmark.so: benchmark.go
	go build -buildmode=pie -ldflags "-linkmode external -X main.readPath=/static_file" -o benchmark.so .

clean:
	rm -f benchmark*.so usr.manifest
