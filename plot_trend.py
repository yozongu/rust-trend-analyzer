import csv
from datetime import datetime
import matplotlib.pyplot as plt
import matplotlib.dates as mdates

def plot_data_from_csv(filename, keyword):
    dates = []
    counts = []

    with open(filename) as f:
        reader = csv.DictReader(f)
        for row in reader:
            dates.append(datetime.strptime(row["date"], "%Y-%m-%d"))
            counts.append(int(row["count"]))

    plt.figure(figsize=(12, 5))
    plt.plot(dates, counts, linewidth=0.7)
    plt.xlabel("Date")
    plt.ylabel("Article Mentions")
    plt.title("Keyword: \"" + keyword + "\" Trend Over Time")

    ax = plt.gca()
    ax.xaxis.set_major_locator(mdates.MonthLocator())
    ax.xaxis.set_major_formatter(mdates.DateFormatter("%b %Y"))
    plt.xticks(rotation=45, ha="right")

    plt.tight_layout()
    plt.savefig("trend.png")
    print("Saved trend.png")