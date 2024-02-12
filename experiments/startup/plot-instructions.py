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


df = pd.read_csv("startup-instructions.csv")
df["cached"] = pd.Categorical.from_codes(df["cached"], ["no", "yes"])
df["instructions"] = df["instructions"] / 1000_000

print(df)


order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_instructions(df, lang, cache):
    ax = sns.barplot(
        order=order,
        hue_order=order,
        data=df.loc[(df["language"] == lang) & (df["cached"] == cache)],
        x="target",
        hue="target",
        y="instructions",
        err_kws={"linewidth": 1},
        capsize=0.2,
    )

    if lang == "go":
        y_offsets = [0, 200, 250, 200, 0]
    else:
        y_offsets = [0, 0, 0, 0, 0]

    add_bar_values(ax, y_offsets)
    # ax.tick_params(labelrotation=15)
    ax.set_xlabel(None)
    ax.set_ylabel("million instructions\nfor cold start")
    plt.savefig(
        f"img/startup-instructions-{lang}-{cache}-page.pdf", bbox_inches="tight"
    )
    ax.clear()


plot_instructions(df, "go", "no")
plot_instructions(df, "go", "yes")
plot_instructions(df, "node", "no")
plot_instructions(df, "node", "yes")
# fig.suptitle("Million instructions executed during cold start")
