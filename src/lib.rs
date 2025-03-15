use pyo3::prelude::*;
pub mod reach;

#[pymodule]
fn rustflow(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let reach_module = PyModule::new(m.py(), "reach")?;
    reach_module.add_function(wrap_pyfunction!(
        reach::muskingum::muskingum_routing,
        m.py()
    )?)?;
    m.add_submodule(&reach_module)?;
    Ok(())
}
