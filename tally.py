#%%
import csv
from datetime import time, timedelta

total_time = {
    'Mon': timedelta(),
    'Tue': timedelta(),
    'Wed': timedelta(),
    'Thu': timedelta(),
    'Fri': timedelta(),
}

def nice_format_time(time: time) -> str:
    """
    Format a time to HH:MM:SS string with leading zeros
    """
    hours, remainder = divmod(time.seconds, 3600)
    minutes, seconds = divmod(remainder, 60)
    return f"{str(hours).zfill(2)}:{str(minutes).zfill(2)}:{str(seconds).zfill(2)}"

with open('log.csv', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        day_of_week = row[1]
        time_str = row[-1]
        h, m, s = map(int, time_str.split(':'))
        total_time[day_of_week] += timedelta(hours=h, minutes=m, seconds=s)
print("")
for day, time in total_time.items():
    print(f"\t{day}\t{nice_format_time(time)}")

total_time = sum(total_time.values(), timedelta())
print(f"""\t-----------------
\tTot\t{nice_format_time(total_time)}
\t-----------------\n""")
# %%
# %%
