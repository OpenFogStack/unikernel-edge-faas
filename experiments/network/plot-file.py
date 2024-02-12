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


def mib(x):
    return x / (2**20)


order = ["linux", "nanos", "osv", "runc", "runsc"]
data = {
    "target": ["osv", "nanos", "linux", "runc", "runsc"],
    "throughput (MiB)": [
        mib(2990204116),
        mib(284737759),
        mib(3689922812),
        mib(3130290637),
        mib(189676236),
    ],
}
df = pd.DataFrame(data)

ax = sns.barplot(
    order=order,
    hue_order=order,
    data=df,
    x="target",
    hue="target",
    y="throughput (MiB)",
    err_kws={"linewidth": 1},
    capsize=0.2,
)
ax.set_xlabel(None)
# ax = df.set_index('Target').plot(kind='bar', stacked=True)
y_offsets = [0, 400, 0, 0, 400]
add_bar_values(ax, y_offsets)

# Fetching 51M file 204 times, so transfering 10GiB, 4 concurrent requests
# OSv is missing syscall 40 (sendfile), which could impact performance
# plt.title('MiB/s network throughput', fontsize=10)
# plt.xticks(rotation='horizontal')
# plt.xlabel(None)
plt.ylabel("throughput (MiB/s)")
plt.savefig("img/throughput.pdf", bbox_inches="tight")
