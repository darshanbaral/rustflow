use pyo3::prelude::*;
pub mod reach;
use reach::muskingum::muskingum_routing;

#[pymodule]
fn pydrology(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(muskingum_routing, m)?)?;
    Ok(())
}
