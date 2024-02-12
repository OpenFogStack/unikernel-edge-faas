#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

sns.set(font_scale=1)

df = pd.read_csv('results.csv')

print(df)

fig, (ax0, ax1) = plt.subplots(1, 2, figsize=(10, 5), sharey=True)
order = ['runc', 'runsc']

def plot_instructions(df, ax, lang):
    data = df.loc[df['language'] == lang]
    ax.set_title("language = {}".format(lang))
    ax.set_ylabel("time (s)")
    legend = None
    if lang == 'go':
        legend = 'auto'
    sns.lineplot(ax=ax, legend=legend, data=data, hue_order=order, x='instances', y='time', hue='target')

plot_instructions(df, ax0, 'go')
plot_instructions(df, ax1, 'node')

plt.savefig("burst-runc-runsc.pdf")

