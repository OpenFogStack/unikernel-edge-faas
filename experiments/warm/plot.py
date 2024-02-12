#!/usr/bin/env python3

import pandas as pd
import seaborn as sns
import matplotlib as mpl
import matplotlib.pyplot as plt

sns.set(font_scale=0.9, style="whitegrid", font="CMU Sans Serif")

mpl.rcParams["pdf.fonttype"] = 42
mpl.rcParams["ps.fonttype"] = 42
mpl.rcParams["figure.figsize"] = (4.5, 1.5)
mpl.rcParams["figure.dpi"] = 100


def add_bar_values(ax):
    for bar in ax.patches:
        y_offset = 0
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            (bar.get_height() + bar.get_y() + y_offset) / 2,
            # 0.3,
            round(bar.get_height(), 1),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("overhead.csv")
df["time"] = df["time"] * 1000

print(df)

fig, ax = plt.subplots(1, 1)


def plot(df, ax):
    sns.barplot(ax=ax, data=df, x="kind", y="time")
    add_bar_values(ax)
    ax.set_ylabel("time (ms)")


plot(df, ax)

# fig.suptitle("Overhead introduced by faas platform")
plt.savefig("img/overhead.pdf", bbox_inches="tight")
