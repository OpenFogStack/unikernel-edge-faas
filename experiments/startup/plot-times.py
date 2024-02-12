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
        y_offset = -15
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            (bar.get_height() + bar.get_y() + y_offset) / 2,
            round(bar.get_height()),
            ha="center",
            color="black",
            size=8,
        )


df = pd.read_csv("startup-times.csv")
df["cached"] = pd.Categorical.from_codes(df["cached"], ["no", "yes"])

df["cold_start"] = df["setup"] + df["startup"]
print(df)

cached = df.loc[df["cached"] == "yes"]
uncached = df.loc[df["cached"] == "no"]
startup = df[["target", "startup", "language", "cached"]]
setup = df[["target", "setup", "language", "cached"]]
startup_cached = cached[["target", "startup", "language"]]
startup_uncached = uncached[["target", "startup", "language"]]
setup_cached = cached[["target", "setup", "language"]]
setup_uncached = uncached[["target", "setup", "language"]]
cold_start_cached = cached[["target", "cold_start", "language"]]
cold_start_uncached = uncached[["target", "cold_start", "language"]]

ax = sns.barplot(
    data=df.loc[setup["language"] == "go"], x="target", y="setup", hue="cached"
)
ax.set_xlabel(None)
ax.set_ylabel("time (ms)")
add_bar_values(ax)
sns.move_legend(ax, title="page cache", loc="best")
# plt.title("Setup time")
plt.savefig("setup-time.pdf", bbox_inches="tight")
ax.clear()

# cold start latency
ax = sns.barplot(data=cold_start_uncached, x="target", y="cold_start", hue="language")
ax.set_ylabel("time (ms)")
ax.set_xlabel(None)
add_bar_values(ax)
plt.savefig("img/cold-start-time-no-page.png", bbox_inches="tight")
ax.clear()

ax = sns.barplot(data=cold_start_cached, x="target", y="cold_start", hue="language")
ax.set_ylabel("time (ms)")
ax.set_xlabel(None)
add_bar_values(ax)
# fig.suptitle("Cold start latency")
plt.savefig("img/cold-start-time-page.png", bbox_inches="tight")
ax.clear()

# startup
ax = sns.barplot(data=startup_uncached, x="target", y="startup", hue="language")
ax.set_xlabel(None)
ax.set_ylabel("time (ms)")
add_bar_values(ax)
plt.savefig("img/startup-time-no-page.png", bbox_inches="tight")
ax.clear()

ax = sns.barplot(data=startup_cached, x="target", y="startup", hue="language")
ax.set_xlabel(None)
ax.set_ylabel("time (ms)")
add_bar_values(ax)
plt.savefig("img/startup-time-page.png", bbox_inches="tight")
ax.clear()
