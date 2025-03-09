from datetime import timedelta
from pydrology import muskingum_routing

if __name__ == "__main__":
    print(help(muskingum_routing))
    print(muskingum_routing(inflow=[1, 2, 3, 5],
                            k=timedelta(hours=1),
                            x=0.25,
                            time_step=timedelta(minutes=15)))
