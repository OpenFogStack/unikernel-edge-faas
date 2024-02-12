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
            # (bar.get_height() + bar.get_y() + y_offset) / 2,
            0.3,
            round(bar.get_height(), 1),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("startup-kvm.csv")
df["cached"] = pd.Categorical.from_codes(df["cached"], ["no", "yes"])
df["time"] = df["time"] / 1000_000

print(df)

cached = df.loc[df["cached"] == "yes"]
uncached = df.loc[df["cached"] == "no"]
vmx_vcpu_run_cached = cached.loc[cached["function"] == "vmx_vcpu_run"]
vmx_vcpu_run_uncached = cached.loc[cached["function"] == "vmx_vcpu_run"]


def plot_functions(df, ax, lang, cache):
    sns.barplot(
        ax=ax,
        data=df.loc[(df["language"] == lang) & (df["cached"] == cache)],
        x="function",
        y="time",
        hue="target",
    )
    # ax.tick_params(labelrotation=15)
    ax.set_title("language = {}, pagecache = {}".format(lang, cache))


fig, axs = plt.subplots(2, 2, figsize=(12, 12), sharey=True)
plot_functions(df, axs[0][0], "go", "no")
plot_functions(df, axs[0][1], "go", "yes")
plot_functions(df, axs[1][0], "node", "no")
plot_functions(df, axs[1][1], "node", "yes")
fig.suptitle("Time spent in various kvm functions by fc_vcpu thread")
plt.savefig("img/kvm-functions.png", bbox_inches="tight")
