use pyo3::prelude::*;

pub fn plot_data_from_csv(filename: &str, keyword: &str) -> PyResult<()> {
    Python::attach(|py| {
        let sys = PyModule::import(py, "sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("append", (".",))?;

        let plt = PyModule::import(py, "plot_trend")?;
        plt.call_method1("plot_data_from_csv", (filename, keyword))?;
        Ok(())
    })
}
