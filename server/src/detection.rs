use pyo3::prelude::*;

pub fn predict(pixels: &[u8], output_path: &str) -> PyResult<Vec<(String, f64)>> {
    Python::with_gil(|py| {
        let code = include_str!("../detection.py");
        let detection = PyModule::from_code(py, code, "detection.py", "detection")?;
        let res: Vec<(String, f64)> = detection
            .getattr("predict")?
            .call1((pixels, output_path))?
            .extract()?;
        Ok(res)
    })
}
