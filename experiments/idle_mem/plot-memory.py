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


df = pd.read_csv("memory.csv")

print(df)

df = df.replace("docker", "runc")
df = df.replace("gvisor", "runsc")
order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_mem(df, lang):
    data = df.loc[df["language"] == lang]
    legend = "auto"
    ax = sns.lineplot(
        legend=legend,
        hue_order=order,
        data=data,
        x="instances",
        y="memory",
        hue="target",
    )
    ax.legend(title=None, loc="center left")
    ax.set_xlabel("number of concurrent instances")
    ax.set_ylabel("total memory\nfootprint (MiB)")
    plt.savefig(f"img/memory-{lang}.pdf", bbox_inches="tight")
    ax.clear()


plot_mem(df, "go")
plot_mem(df, "node")

df["memory_per_instance"] = df["memory"] / df["instances"]
print(df)


def plot_per_instance(df, lang):
    data = df.loc[df["language"] == lang]
    ax = sns.barplot(
        order=order,
        hue_order=order,
        data=data,
        x="target",
        hue="target",
        y="memory_per_instance",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )
    ax.set_ylabel("memory\nfootprint (MiB)")
    ax.set_xlabel(None)
    if lang == "go":
        y_offsets = [0, 0, 0, 20, 0]
    else:
        y_offsets = [0, 0, 0, 50, 0]

    add_bar_values(ax, y_offsets)
    plt.savefig(f"img/memory-single-{lang}.pdf", bbox_inches="tight")
    ax.clear()


plot_per_instance(df, "go")
plot_per_instance(df, "node")

# fig.suptitle("Memory usage during idle per instance")
# plt.show()
