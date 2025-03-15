from datetime import timedelta
from rustflow.reach import muskingum_routing as mkr
import pandas

if __name__ == "__main__":
    df = pandas.read_csv("./data/gage_flow_data.csv", index_col=0, parse_dates=True)
    inflow = df["Flow (cfs)"]
    outflow = mkr(inflow=inflow,
                  k=timedelta(hours=1),
                  x=0.25,
                  time_step=timedelta(minutes=15),
                  sub_reaches=12)
    o_df = pandas.DataFrame({"flow": outflow}, index=df.index)
    o_df.to_csv("./data/outflow.csv")
