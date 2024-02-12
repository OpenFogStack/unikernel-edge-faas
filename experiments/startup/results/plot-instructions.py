#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

def add_bar_values(ax):
    for bar in ax.patches:
        y_offset = 4
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
    'Target':          ['osv (rofs)', 'osv (zfs)', 'nanos', 'linux'],
    'no page cache': [200, 516, 124, 953],
    'full page cache':   [143, 487, 106, 822]
}
df = pd.DataFrame(data) 

ax = df.set_index('Target').plot(kind='bar', stacked=False)
add_bar_values(ax)

plt.title('Million instructions executed during startup', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('million instructions')
plt.savefig("instruction-count.png")

