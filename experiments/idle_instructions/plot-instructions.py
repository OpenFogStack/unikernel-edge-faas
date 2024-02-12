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
    for i, bar in enumerate(ax.patches):
        print(bar.get_height(), bar.get_y())
        # o = [80000, 600000, 110000, 80000, 40000]
        o = [110000, 80000, 600000, 37000, 80000]
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            o[i],
            round(bar.get_height()),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("instructions.csv")

print(df)


def get_median(df, target, lang):
    data = df.loc[(df["language"] == lang) & (df["target"] == target)]
    median = data["instructions"].median()
    print("{}-{} median = {}".format(target, lang, median))


for lang in ["go", "node"]:
    for target in ["osv", "linux", "nanos", "runc", "runsc"]:
        get_median(df, target, lang)

order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_instructions(df, lang):
    data = df.loc[df["language"] == lang]
    ax = sns.barplot(
        data=data,
        order=order,
        hue_order=order,
        x="target",
        hue="target",
        y="instructions",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )
    ax.set_ylabel("instructions\nduring 1s idle")
    ax.set_xlabel(None)
    add_bar_values(ax)
    ax.set_yscale("log")
    plt.savefig(f"img/instructions-idle-log-{lang}.pdf", bbox_inches="tight")
    ax.clear()


plot_instructions(df, "go")
plot_instructions(df, "node")

# fig.suptitle("Instructions executed during 1000ms idle")
