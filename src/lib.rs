use pyo3::prelude::*;

#[pyfunction]
fn modify_list(input_list: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut result_list: Vec<f64> = Vec::new();
    let mut previous_value: Option<f64> = None;

    for value in input_list.iter() {
        if previous_value.is_none() {
            result_list.push(value + 5.0);
        } else {
            if previous_value.unwrap() > 3.5 {
                result_list.push(value + 3.0);
            } else {
                result_list.push(value + 5.0);
            }
        }
        previous_value = Some(*value);
    }

    Ok(result_list)
}

#[pymodule]
fn resop(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(modify_list, m)?)?;
    Ok(())
}