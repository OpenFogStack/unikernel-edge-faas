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


order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot(exp):
    df = pd.read_csv("{}.csv".format(exp))
    df = df.replace("docker", "runc")
    df = df.replace("gvisor", "runsc")
    if exp == "fib":
        df["time"] = df["time"] * 1000

    data = df.loc[df["attempt"] == "first"]
    ax = sns.barplot(
        order=order,
        data=data,
        x="target",
        hue="target",
        y="time",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )
    add_bar_values(ax)
    ax.set_ylabel("task completion\ntime (ms)")
    ax.set_xlabel(None)

    # fig.suptitle(title)
    plt.savefig(f"img/{exp}.pdf", bbox_inches="tight")
    ax.clear()


plot("matrix")
plot("fib")
plot("fft")
