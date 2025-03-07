use pyo3::prelude::*;

#[pyfunction]
fn muskingum_routing(inflow: Vec<f64>, k: f64, x: f64, time_step: i64) -> PyResult<Vec<f64>> {
    let secs: f64 = time_step as f64;
    let c0 = secs / (2.0 * k * (1.0 - x) + secs);
    let c1 = (k * x + secs / 2.0 - k * x * secs) / (k * (1.0 - x) + secs / 2.0);
    let c2 = (k - k * x - secs / 2.0 + k * x * secs) / (k * (1.0 - x) + secs / 2.0);

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

#[pymodule]
fn pydrology(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(muskingum_routing, m)?)?;
    Ok(())
}
