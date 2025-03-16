use pyo3::prelude::*;
use pyo3::types::PyDelta;
use std::time::Duration;

/// Computes the outflow hydrograph using the Muskingum routing method.
///
/// The Muskingum method is a hydrological routing technique used to predict the
/// outflow hydrograph from a river reach, given the inflow hydrograph and
/// reach characteristics.
///
/// # Arguments
///
/// * `inflow` - A vector of `f64` values representing the inflow hydrograph.
///   Each value represents the inflow at a specific time step.
/// * `k` - A `Py<PyDelta>` representing the storage time constant of the reach.
///   It represents the time it takes for the volume of water in the reach to change
///   by a given amount.
/// * `x` - A `f64` representing the weighting factor for inflow and outflow.
///   This value must be between 0.0 and 0.5 (inclusive). It defines the
///   relative importance of inflow and outflow in determining storage.
///   * x = 0 : storage is a function of outflow only.
///   * x = 0.5 : storage is a function of inflow only.
///   * 0 < x < 0.5: storage is a function of both inflow and outflow.
/// * `time_step` - A `Py<PyDelta>` representing the time step used for the
///   inflow hydrograph.
///
/// # Returns
///
/// A `PyResult<Vec<f64>>` containing the outflow hydrograph as a vector of `f64` values.
/// Returns a `PyValueError` if `x` is outside the range of 0.0 to 0.5 (inclusive).
///
/// # Errors
///
/// This function will return a `PyValueError` if:
/// - `x` is not within the range [0.0, 0.5].
///
/// # Example
///
/// ```python
/// from datetime import timedelta
/// from rustflow import muskingum_routing
///
/// inflow = [1.0, 2.0, 3.0, 5.0, 4.0, 2.0, 1.0]  # Example inflow hydrograph
/// k = timedelta(hours=1)  # Storage time constant
/// x = 0.25  # Weighting factor
/// time_step = timedelta(minutes=15)  # Time step of inflow data
///
/// outflow = muskingum_routing(inflow=inflow, k=k, x=x, time_step=time_step)
/// print(outflow)
/// ```
#[pyfunction]
pub fn muskingum_routing(
    py: Python,
    inflow: Vec<f64>,
    k: Py<PyDelta>,
    x: f64,
    time_step: Py<PyDelta>,
    sub_reaches: i64,
    initial_outflow: Option<f64>,
) -> PyResult<Vec<f64>> {
    let initial_outflow = initial_outflow.unwrap_or(inflow[0]);

    if x < 0.0 || x > 0.5 {
        let warnings = py.import("warnings")?;
        warnings.call_method1("warn", ("`x` is outside of recommended range [0.0, 0.5].",))?;
    }
    let time_step_duration: Duration = time_step.extract(py)?;
    let dt_s: f64 = time_step_duration.as_secs() as f64;

    let k_duration: Duration = k.extract(py)?;
    let k_s: f64 = k_duration.as_secs() as f64 / sub_reaches as f64;

    let mut outflow = muskingum_routing_rs(inflow, dt_s, k_s, x, Some(initial_outflow));

    for _ in 0..(sub_reaches - 1) {
        outflow = muskingum_routing_rs(outflow, dt_s, k_s, x, None)
    }

    Ok(outflow)
}

fn muskingum_routing_rs(
    q_in: Vec<f64>,
    dt: f64,
    k: f64,
    x: f64,
    initial_outflow: Option<f64>,
) -> Vec<f64> {
    let initial_outflow = initial_outflow.unwrap_or(q_in[0]);
    let den: f64 = 2.0 * k * (1.0 - x) + dt;
    let c0 = (dt - 2.0 * k * x) / den;
    let c1 = (dt + 2.0 * k * x) / den;
    let c2 = (2.0 * k * (1.0 - x) - dt) / den;

    let mut outflow: Vec<f64> = Vec::with_capacity(q_in.len());
    let mut previous_inflow: f64 = 0.0;
    outflow.push(initial_outflow);
    let mut previous_outflow: f64 = initial_outflow;

    let mut current_outflow: f64;
    for &current_inflow in q_in.iter().skip(1) {
        current_outflow = c0 * current_inflow + c1 * previous_inflow + c2 * previous_outflow;
        outflow.push(current_outflow);
        previous_outflow = current_outflow;
        previous_inflow = current_inflow;
    }

    outflow
}
