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
            ((bar.get_height() + bar.get_y() + y_offset) / 2)
            - (bar.get_height() + bar.get_y()) * 0.15,
            # 0.3,
            round(bar.get_height(), 1),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("startup-read.csv")
df["cached"] = pd.Categorical.from_codes(df["cached"], ["no", "yes"])
df["time"] = df["time"] / 1000_000

print(df)

cached = df.loc[df["cached"] == "yes"]
uncached = df.loc[df["cached"] == "no"]
kernel = df.loc[df["file"] == "kernel"]
rootfs = df.loc[df["file"] == "rootfs"]
kernel_cached = df.loc[(df["cached"] == "yes") & (df["file"] == "kernel")]
rootfs_cached = df.loc[(df["cached"] == "yes") & (df["file"] == "rootfs")]
kernel_uncached = df.loc[(df["cached"] == "no") & (df["file"] == "kernel")]
rootfs_uncached = df.loc[(df["cached"] == "no") & (df["file"] == "rootfs")]

print(rootfs_cached)

# rootfs read times
fig, axs = plt.subplots(1, 2, figsize=(10, 5), sharey=True)
axs[0].set_title("no page cache")
sns.barplot(ax=axs[0], data=rootfs_uncached, x="target", y="time", hue="language")
add_bar_values(axs[0])
axs[1].set_title("full page cache")
sns.barplot(ax=axs[1], data=rootfs_cached, x="target", y="time", hue="language")
add_bar_values(axs[1])
axs[0].set_ylabel("time (ms)")
axs[1].set_ylabel("time (ms)")
axs[0].set_xlabel(None)
axs[1].set_xlabel(None)
# fig.suptitle("Time spent reading from rootfs during startup")
plt.savefig("img/rootfs-time.png", bbox_inches="tight")

fig, axs = plt.subplots(1, 2, figsize=(10, 5), sharey=True)
axs[0].set_title("no page cache")
sns.barplot(ax=axs[0], data=kernel_uncached, x="target", y="time")
add_bar_values(axs[0])
axs[1].set_title("full page cache")
sns.barplot(ax=axs[1], data=kernel_cached, x="target", y="time")
add_bar_values(axs[1])
axs[0].set_ylabel("time (ms)")
axs[1].set_ylabel("time (ms)")
axs[0].set_xlabel(None)
axs[1].set_xlabel(None)
# fig.suptitle("Time spent reading from kernel ELF during startup")
plt.savefig("img/kernel-time.png", bbox_inches="tight")

df = pd.read_csv("startup-size.csv")
df["size"] = df["size"] / (1024 * 1024)
kernel_size = df.loc[df["file"] == "kernel"]
rootfs_size = df.loc[df["file"] == "rootfs"]
print(rootfs_size)
fig, axs = plt.subplots(1, 1, figsize=(5, 5))
sns.barplot(ax=axs, data=rootfs_size, x="target", y="size", hue="language")
# axs.set_title("MiB read from rootfs during startup")
axs.set_ylabel("MiB")
axs.set_xlabel(None)
add_bar_values(axs)
plt.savefig("img/rootfs-size.png", bbox_inches="tight")

df = pd.read_csv("startup-size.csv")
df["size"] = df["size"] / (1024 * 1024)
kernel_size = df.loc[df["file"] == "kernel"]
rootfs_size = df.loc[df["file"] == "rootfs"]
print(rootfs_size)
fig, axs = plt.subplots(1, 1, figsize=(5, 5))
sns.barplot(ax=axs, data=kernel_size, x="target", y="size")
# axs.set_title("MiB read from kernel ELF during startup")
axs.set_ylabel("MiB")
axs.set_xlabel(None)
add_bar_values(axs)
plt.savefig("img/kernel-size.png", bbox_inches="tight")
