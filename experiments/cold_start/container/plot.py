#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

sns.set(font_scale=1)

def add_bar_values(ax):
    for bar in ax.patches:
        y_offset = 0
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            (bar.get_height() + bar.get_y() + y_offset) / 2,
            # 0.3,
            round(bar.get_height(), 1),
            ha='center',
            color='black',
            size=8
        )

df = pd.read_csv('results.csv')
df['time'] = df['time'] * 1000

print(df)

fig, ax = plt.subplots(1, 1, figsize=(5, 5), sharey=True)
order = ['runc', 'runsc']
hue_order = ['go', 'node']

def plot_time(df, ax):
    sns.barplot(ax=ax, data=df, order=order, hue_order=hue_order, x='target', y='time', hue='language')
    add_bar_values(ax)
    ax.set_ylabel("time (ms)")
    ax.set_xlabel(None)

plot_time(df, ax)

plt.savefig("cold-start-runc-runsc.pdf")

