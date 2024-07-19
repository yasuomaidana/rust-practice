use pyo3::prelude::*;

fn fibbonaci_number(n:u32) -> u32 {
    if n<=1 {
        return n;
    }
    fibbonaci_number(n-1) + fibbonaci_number(n-2)
}

/// Retrieves the fibonnaci number of a given n.
#[pyfunction]
fn calculate_fibonacci(n:u32) -> PyResult<u32> {
    Ok(fibbonaci_number(n))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fibonacci_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_fibonacci, m)?)?;
    Ok(())
}
