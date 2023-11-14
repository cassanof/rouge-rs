use pyo3::prelude::*;

pub mod dedup;
pub mod rouge;
pub mod tokenizer;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn rouge_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<rouge::RougeLScorer>()?;
    m.add_class::<rouge::Score>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
