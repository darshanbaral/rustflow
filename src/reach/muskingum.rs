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
/// from pydrology import muskingum_routing
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
) -> PyResult<Vec<f64>> {
    if x < 0.0 || x > 0.5 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "`x` must be between 0 and 0.5 (inclusive)",
        ));
    }
    let time_step_duration: Duration = time_step.extract(py)?;
    let k_duration: Duration = k.extract(py)?;
    let k_val: f64 = k_duration.as_secs() as f64;
    let delta: f64 = time_step_duration.as_secs() as f64;
    let c0 = delta / (2.0 * k_val * (1.0 - x) + delta);
    let c1 = (k_val * x + delta / 2.0 - k_val * x * delta) / (k_val * (1.0 - x) + delta / 2.0);
    let c2 =
        (k_val - k_val * x - delta / 2.0 + k_val * x * delta) / (k_val * (1.0 - x) + delta / 2.0);

    let mut outflow: Vec<f64> = Vec::with_capacity(inflow.len());
    let mut previous_inflow: f64 = 0.0;
    let mut previous_outflow: f64 = c0 * inflow[0];
    let mut is_first_value: bool = true;

    let mut current_outflow: f64;
    for &current_inflow in &inflow {
        if is_first_value {
            current_outflow = current_inflow;
            is_first_value = false
        } else {
            current_outflow = c0 * current_inflow + c1 * previous_inflow + c2 * previous_outflow;
        }
        outflow.push(current_outflow);
        previous_outflow = current_outflow;
        previous_inflow = current_inflow;
    }

    Ok(outflow)
}
