use pyo3::prelude::*;

fn predict(img_path: &str) -> PyResult<()> {
    Python::with_gil(|py| {
        let code = include_str!("../detection.py");
        let detection = PyModule::from_code(py, code, "detection.py", "detection")?;
        let _ = detection.getattr("predict")?.call1((img_path, img_path))?;
        Ok(())
    })
}
