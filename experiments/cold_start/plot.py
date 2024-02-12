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


def add_bar_values(ax, y_offsets):
    for i, bar in enumerate(ax.patches):
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            (bar.get_height() + bar.get_y() + y_offsets[i]) / 2,
            # 0.3,
            round(bar.get_height(), 1),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("cold-start.csv")
df["time"] = df["time"] * 1000

df = df.replace("docker", "runc")
df = df.replace("gvisor", "runsc")


print(df)

order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_read(df, lang):
    data = df.loc[df["language"] == lang]
    ax = sns.barplot(
        data=data,
        order=order,
        hue_order=order,
        x="target",
        hue="target",
        y="time",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )
    if lang == "go":
        y_offsets = [0, -40, -40, 0, 0]
    else:
        y_offsets = [0, 0, 0, 0, 0]

    add_bar_values(ax, y_offsets)
    ax.set_ylabel("cold start\ntime (ms)")
    ax.set_xlabel(None)
    plt.savefig(f"img/cold-start-{lang}.pdf", bbox_inches="tight")
    ax.clear()


plot_read(df, "go")
plot_read(df, "node")

# fig.suptitle("Cold start time for single instance")
