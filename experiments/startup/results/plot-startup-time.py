#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

def add_bar_values(ax):
    for bar in ax.patches:
        y_offset = -15
        print(bar.get_height())
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + bar.get_y() + y_offset,
            round(bar.get_height()),
            ha='center',
            color='black',
            size=8
        )

data = {
    'Target':    ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'Setup':     [61, 77, 47, 93],
    'Boot':      [63, 106, 56, 357]
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=True)
add_bar_values(ax)

plt.title('Firecracker setup and boot times (without page cache)', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('ms')
plt.savefig("startup-time.png")

data = {
    'Target':    ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'Setup':     [42, 56, 47, 60],
    'Boot':      [55, 113, 46, 319]
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=True)
add_bar_values(ax)

plt.title('Firecracker setup and boot times (with full page cache)', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('ms')
plt.savefig("startup-time-cache.png")