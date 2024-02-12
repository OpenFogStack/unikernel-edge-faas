#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

def add_bar_values(ax):
    for bar in ax.patches:
        y_offset = 0.5
        print(bar.get_height())
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + bar.get_y() + y_offset,
            round(bar.get_height(), 1),
            ha='center',
            color='black',
            size=8
        )

data = {
    'Target':    ['osv', 'nanos', 'linux'],
    'Size':      [6.1, 1.7, 43.6]
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=True)
add_bar_values(ax)

plt.title('Kernel ELF size in MiB', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('MiB')
plt.savefig("kernel-size.png")

