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


df = pd.read_csv("read.csv")
df["time"] = df["time"] * 1000

print(df)
df = df.replace("docker", "runc")
df = df.replace("gvisor", "runsc")

order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_read(df, attempt):
    data = df.loc[df["attempt"] == attempt]
    # ax.set_title("{} read".format(attempt))
    ax = sns.barplot(
        order=order,
        hue_order=order,
        data=data,
        x="target",
        hue="target",
        y="time",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )
    add_bar_values(ax)
    ax.set_ylabel("total\nread time (ms)")
    ax.set_xlabel(None)


plot_read(df, "first")
# plot_read(df, ax1, 'second')

# fig.suptitle("Time to read 50 MiB from disk")
plt.savefig("img/read.pdf", bbox_inches="tight")
