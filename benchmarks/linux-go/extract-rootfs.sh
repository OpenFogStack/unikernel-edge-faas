#!/bin/sh

echo "Extracting rootfs to $ROOTFS_MOUNT"
for d in home; do tar c "/$d" | tar x -C $ROOTFS_MOUNT; done;
for d in bin dev etc lib root sbin usr var static; do tar c "/$d" | tar x -C $ROOTFS_MOUNT; done;
for dir in proc run sys; do mkdir $ROOTFS_MOUNT/${dir}; done;
