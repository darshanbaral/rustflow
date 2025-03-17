from ..rustflow import reach
from typing import Optional
from datetime import timedelta


# Expose Rust functions
def muskingum_routing(
        inflow: list[float],
        k: timedelta,
        x: float,
        time_step: timedelta,
        sub_reaches: Optional[int] = 1,
        initial_outflow: Optional[float] = None,
):
    if not isinstance(inflow, list):
        inflow = list(inflow)

    if initial_outflow is None:
        initial_outflow = inflow[0]

    return reach.muskingum_routing(
        inflow, k, x, time_step, sub_reaches, initial_outflow
    )
