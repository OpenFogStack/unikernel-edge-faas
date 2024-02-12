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

df = pd.read_csv("time.csv")

# fig, (ax0, ax1) = plt.subplots(1, 2, figsize=(10, 5), sharey=True)
# df = df.replace("docker", "docker (runc)", regex=True)
# df = df.replace("gvisor", "docker (runsc)", regex=True)
print(df)
df = df.replace("docker", "runc")
df = df.replace("gvisor", "runsc")
order = ["linux", "nanos", "osv", "runc", "runsc"]


def plot_instructions(df, lang):
    data = df.loc[df["language"] == lang]

    legend = "auto"

    ax = sns.lineplot(
        legend=legend,
        data=data,
        hue_order=order,
        x="instances",
        y="time",
        hue="target",
    )
    ax.set_xlabel("number of concurrent cold starts")
    ax.set_ylabel("total time (s)")
    ax.legend(title=None, loc="center left")

    plt.savefig(f"img/burst-{lang}.pdf", bbox_inches="tight")
    ax.clear()


plot_instructions(df, "go")
plot_instructions(df, "node")

# fig.suptitle("Cold start latency for n concurrent invocations")
# plt.savefig("burst.pdf", bbox_inches="tight")
