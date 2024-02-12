#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

def add_bar_values(ax, y_offset):
    for bar in ax.patches:
        print(bar.get_height())
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + bar.get_y() + y_offset,
            round(bar.get_height(), 1),
            ha='center',
            color='black',
            size=8
        )

# Kernel is only read during setup, rootfs only during boot
data = {
    'Target':    ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'Kernel':    [19.9, 19.4, 11.8, 43],
    'Rootfs':    [10.7, 11.7, 11.7, 40.9]
}

df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=False)
add_bar_values(ax, 0.5)

plt.title('Time spent reading from disk (vfs_read) during startup (without page cache)', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('ms')
plt.savefig("disk-read-time.png")

data = {
    'Target':    ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'Kernel':    [4, 4.6, 1.7, 12.7],
    'Rootfs':    [1.5, 1.8, 3.8, 1.9]
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=False)
add_bar_values(ax, 0.2)

plt.title('Time spent reading from disk (vfs_read) during startup (with full page cache)', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('ms')
plt.savefig("disk-read-time-cached.png")

# osv with zfs needs to load the zfs module
# ld.so and libc.so are built into the osv kernel
# The application is around 7.6MiB, same for osv shared object
# There is no need to read in the whole elf file and instead rely on
# demand paging
data = {
    'Target':    ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'Rootfs':    [4635, 6575, 9842, 8359],
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=False)
add_bar_values(ax, 130)

plt.title('KiB read from rootfs disk during startup', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('KiB')
plt.savefig("rootfs-bytes-read.png")
