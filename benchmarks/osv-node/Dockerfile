FROM ubuntu:22.04

RUN apt-get update && apt-get -y --no-install-recommends install \
	build-essential \
	python3 \
	python3-dev \
	python3-bottle \
	curl \
	wget \
	ca-certificates \
	git \
	bash \
	golang \
	rsync

WORKDIR /root

# Need older version of openssl to build osv...
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.20_amd64.deb \
  && dpkg -i libssl1.1_1.1.1f-1ubuntu2.20_amd64.deb

RUN git clone https://github.com/cloudius-systems/osv.git \
	&& cd osv \
	&& git checkout d620c79961f7cb523c0ce3640bff059aee1ec5eb \
	&& git submodule update --init --recursive \
	&& ./scripts/setup.py

RUN apt-get -y --no-install-recommends install nodejs

WORKDIR /root/osv

RUN mkdir /images

RUN apt-get -y --no-install-recommends install zip
COPY osv-node/app	/root/osv/apps/benchmark
COPY app-node/*		/root/osv/apps/benchmark/

# RUN cd apps/benchmark && ./genfile.sh

RUN ./scripts/build -j$(nproc) fs=rofs fs_size_mb=200 image=benchmark
RUN qemu-img convert -O raw build/release/usr.img build/release/usr.raw
# RUN cp build/release/usr.img /images/benchmark-rofs.img
RUN cp build/release/usr.raw /images/benchmark-rofs.raw
RUN cp build/release/loader-stripped.elf /images/loader-stripped-rofs.elf


RUN ./scripts/build -j$(nproc) fs=zfs fs_size_mb=200 image=benchmark
RUN qemu-img convert -O raw build/release/usr.img build/release/usr.raw
# RUN cp build/release/usr.img /images/benchmark-zfs.img
RUN cp build/release/usr.raw /images/benchmark-zfs.raw
RUN cp build/release/loader-stripped.elf /images/loader-stripped-zfs.elf

