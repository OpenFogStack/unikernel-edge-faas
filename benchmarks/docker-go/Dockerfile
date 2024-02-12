FROM golang:alpine

WORKDIR /root
COPY app-go/* ./

RUN apk add zip
RUN ./genfile.sh
RUN go build -ldflags "-X main.readPath=/root/static_file" .

FROM alpine
COPY --from=0 /root/benchmark /root/benchmark
COPY --from=0 /root/static_file /root/static_file

ENTRYPOINT ["/root/benchmark"]
