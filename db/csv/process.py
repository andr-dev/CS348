import os
import shutil
import pandas as pd
import csv

path_r = os.path.realpath("raw")
path_p = os.path.realpath("processed")

shutil.rmtree(path_p)

os.makedirs(path_p, exist_ok=False)

for csv_file in os.listdir(path_r):
    print(f'processing {csv_file} ...')
    df = pd.read_csv(path_r + "/" + csv_file)

    if csv_file == "teambans.csv" or csv_file == "teamstats.csv":
        df['teamid'] = df['teamid'].replace({100: 0, 200: 1})
    
    df.to_csv(path_p + "/" + csv_file, index=False, quoting=csv.QUOTE_NONNUMERIC)

    print("done!")

os.system('cat processed/stats1.csv >> processed/stats.csv')
os.system('cat processed/stats2.csv | tail -n +2 >> processed/stats.csv')
os.system('rm processed/stats1.csv processed/stats2.csv')
