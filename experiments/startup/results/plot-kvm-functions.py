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
    'function':    ['kvm_vcpu_halt', 'vmx_vpu_run', 'kvm_mmu_page_fault'],
    'nanos':       [10.5, 15.5, 7.2],
    'osv (rofs)':  [7.2, 21.6, 10.1],
    'osv (zfs)':   [19.1, 50.0, 11.6],
    'linux':       [144.2, 155.7, 18.4],
}
df = pd.DataFrame(data)

ax = df.set_index('function').plot(kind='bar', stacked=False)
add_bar_values(ax)

plt.title('Time spent executing various KVM functions', fontsize=10)
plt.xticks(rotation='horizontal')
plt.xlabel(None)
plt.ylabel('ms')
plt.savefig("kvm-function-timings.png")

