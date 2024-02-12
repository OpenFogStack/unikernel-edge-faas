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


data = []
order = ["linux", "nanos", "osv", "runc", "runsc"]
for t in order:
    file = open("data/{}-simple".format(t), "r")
    for i, line in enumerate(file.read().splitlines()):
        i = 2**i
        data.append({"target": t, "i": i, "rps": float(line)})

df = pd.DataFrame(data)
print(df)

ax = sns.lineplot(
    data=df, x="i", y="rps", hue="target", markers=True, style="target", dashes=False
)
ax.legend(title=None)
# plt.title('Requests per second (for 32768 requests)', fontsize=10)
plt.xlabel("number of concurrent requests")
plt.ylabel("completed\nrequests/s")
plt.xscale("log", base=2)
plt.savefig("img/simple.pdf", bbox_inches="tight")
