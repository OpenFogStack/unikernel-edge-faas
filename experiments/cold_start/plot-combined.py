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

df = pd.read_csv('cold-start.csv')
df['time'] = df['time'] * 1000

print(df)

fig, (ax0, ax1, ax2) = plt.subplots(1, 3, figsize=(15,5), sharey=True)
df = df.replace("docker", "runc")
df = df.replace("gvisor", "runsc")
order = ['linux', 'nanos', 'osv', 'runc', 'runsc']

def plot_read(df, ax, lang):
    data = df.loc[df['language'] == lang]
    ax.set_title("language = {}".format(lang))
    sns.barplot(ax=ax, data=data, order=order, x='target', y='time')
    add_bar_values(ax)
    ax.set_ylabel("time (ms)")

plot_read(df, ax0, 'go')
plot_read(df, ax1, 'node')

df = pd.read_csv('container/results.csv')
df['time'] = df['time'] * 1000

order = ['runc', 'runsc']
hue_order = ['go', 'node']

def plot_time(df, ax):
    pal = sns.color_palette()
    cols = [pal[5], pal[6]]
    ax.set_title("without docker")
    sns.barplot(ax=ax, data=df, order=order, hue_order=hue_order, x='target', y='time', hue='language', palette=cols)
    add_bar_values(ax)
    ax.set_ylabel("time (ms)")

plot_time(df, ax2)

plt.savefig("cold-start-runc-runsc.pdf")

# fig.suptitle("Cold start time for single instance")
plt.savefig("cold-start.pdf")

