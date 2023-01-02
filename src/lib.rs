use boa_engine::Context;

use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(boa, JSException, pyo3::exceptions::PyException);

#[pyclass(unsendable)]
struct BoaPyContext {
    context: Context,
}

#[pymethods]
impl BoaPyContext {
    #[new]
    fn new() -> Self {
        let context = Context::default();
        Self { context }
    }

    fn execute(&mut self, code: &str) -> PyResult<String> {
        match self.context.eval(code) {
            Ok(value) => Ok(value
                .to_string(&mut self.context)
                .unwrap()
                .as_str()
                .to_string()),
            Err(err) => Err(JSException::new_err(
                err.to_string(&mut self.context)
                    .unwrap()
                    .as_str()
                    .to_string(),
            )),
        }
    }
}

#[pymodule]
fn boa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BoaPyContext>()?;
    m.add("JSException", _py.get_type::<JSException>())?;
    Ok(())
}
