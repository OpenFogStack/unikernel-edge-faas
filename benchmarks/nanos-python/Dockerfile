FROM ubuntu:22.04

RUN apt-get update && apt-get -y --no-install-recommends install \
    build-essential \
    git \
    ca-certificates \
    wget \
    nasm \
    curl

RUN curl -O -L https://go.dev/dl/go1.21.0.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.21.0.linux-amd64.tar.gz

ENV PATH=$PATH:/usr/local/go/bin:/root/go/bin

RUN wget https://github.com/bufbuild/buf/releases/download/v1.13.1/buf-Linux-x86_64 && \
    mv buf-Linux-x86_64 /usr/bin/buf && \
    chmod +x /usr/bin/buf && \
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest && \
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest && \
    curl -L https://github.com/grpc-ecosystem/grpc-gateway/releases/download/v2.15.0/protoc-gen-openapiv2-v2.15.0-linux-x86_64 -o ~/go/bin/protoc-gen-openapiv2 && \
    chmod +x ~/go/bin/protoc-gen-openapiv2 && \
    curl -L https://github.com/grpc-ecosystem/grpc-gateway/releases/download/v2.15.0/protoc-gen-grpc-gateway-v2.15.0-linux-x86_64 -o ~/go/bin/protoc-gen-grpc-gateway \
    && chmod +x ~/go/bin/protoc-gen-grpc-gateway

RUN git clone https://github.com/nanovms/ops.git \
    $(go env GOPATH)/src/github.com/nanovms/ops && \
    cd $(go env GOPATH)/src/github.com/nanovms/ops && \
    git checkout 0.1.37 && \
    make deps && \
    make generate && \
    make build
    
ENV PATH=$PATH:/root/go/src/github.com/nanovms/ops

RUN mkdir /images
WORKDIR /root

RUN git clone https://github.com/felixmoebius/nanos.git -b faas && \
    cd nanos && \
    git clone https://github.com/nanovms/mbedtls.git vendor/mbedtls && \
    touch vendor/mbedtls/.vendored && \
    git -C vendor/mbedtls checkout 3d10e6cabe0a215a3284061ac6d1e1210c3c4472 && \
    make && \
    cp output/platform/pc/bin/kernel.img /images/kernel.img

RUN mkdir /root/app
WORKDIR /root/app

COPY nanos-python/config.json ./
COPY app-python/* ./

# RUN apt-get -y --no-install-recommends install python3-virtualenv
# RUN python3 -m virtualenv --always-copy venv && \
#     . venv/bin/activate && \
#     pip install aiohttp && \
#     deactivate && pwd && ls
    
RUN ops image create --package eyberg/python:3.10.6 -c config.json -i benchmark -a server.py && \
    cp $HOME/.ops/images/benchmark /images/benchmark.img

