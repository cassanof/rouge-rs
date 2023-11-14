use pyo3::prelude::*;

pub mod dedup;
pub mod rouge;
pub mod tokenizer;

#[pyfunction]
pub fn rouge_l_dedup(solutions: Vec<&str>, threshold: f64) -> Vec<&str> {
    let scorer = rouge::RougeLScorer::default();
    dedup::rouge_dedup(&scorer, &solutions, threshold)
}

#[pyfunction]
pub fn rouge_l_grouped_dedup(
    solutions: Vec<&str>,
    threshold: f64,
    dedup_prob: f64,
    max_group_size: usize,
) -> Vec<&str> {
    dedup::rouge_l_grouped_dedup(&solutions, threshold, dedup_prob, max_group_size)
}

#[pymodule]
fn rouge_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<rouge::RougeLScorer>()?;
    m.add_class::<rouge::Score>()?;
    m.add_function(wrap_pyfunction!(rouge_l_grouped_dedup, m)?)?;
    m.add_function(wrap_pyfunction!(rouge_l_dedup, m)?)?;
    Ok(())
}
