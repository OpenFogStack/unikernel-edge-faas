FROM alpine:3.13
RUN apk update \
	&& apk add openrc nodejs sudo zip util-linux

RUN addgroup -S alpine && adduser -S alpine -G alpine -h /home/alpine -s /bin/sh \
	&& echo "alpine:alpine" | chpasswd \
	&& echo '%alpine ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/alpine \
	&& ln -s agetty /etc/init.d/agetty.ttyS0 \
	&& echo ttyS0 > /etc/securetty

RUN rc-update add agetty.ttyS0 default \
	&& rc-update add devfs boot \
	&& rc-update add procfs boot \
	&& rc-update add sysfs boot \
	&& rc-update add local default

COPY app-node/main.js /bin/main.js

COPY ./linux-node/faas.openrc /etc/init.d/faas
RUN rc-update add faas default

RUN chown -R alpine:alpine /home/alpine \
	&& chmod 0740 /home/alpine \
	&& mkdir -p /run/openrc \
	&& touch /run/openrc/softlevel \
	&& touch /var/log/benchmark.log /var/log/benchmark.err

COPY linux-node/extract-rootfs.sh /extract-rootfs.sh
ENTRYPOINT ["/extract-rootfs.sh"]
