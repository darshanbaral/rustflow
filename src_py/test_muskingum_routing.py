from datetime import timedelta
from rustflow import muskingum_routing
import pandas

if __name__ == "__main__":
    df = pandas.read_csv("../data/gage_flow_data.csv", index_col=0, parse_dates=True)
    inflow = df["Flow (cfs)"].to_list()
    outflow = muskingum_routing(inflow=inflow,
                                k=timedelta(hours=1),
                                x=0.25,
                                time_step=timedelta(minutes=15))
    o_df = pandas.DataFrame({"flow": outflow}, index=df.index)
    o_df.to_csv("../data/outflow.csv")
