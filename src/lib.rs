use boa_engine::Context;
use pyo3::create_exception;
use pyo3::prelude::*;
use std::cell::RefCell;

create_exception!(boa, JSException, pyo3::exceptions::PyException);

thread_local!(static CONTEXT: RefCell<Context> = RefCell::new(Context::default()));

#[pyfunction]
fn eval_js(code: &str) -> PyResult<String> {
    let mut context = Context::default();
    match context.eval(code) {
        Ok(value) => Ok(value.to_string(&mut context).unwrap().as_str().to_string()),
        Err(err) => Err(JSException::new_err(
            err.to_string(&mut context).unwrap().as_str().to_string(),
        )),
    }
}

#[pyfunction]
fn execute(code: &str) -> PyResult<String> {
    CONTEXT.with(|f| {
        let mut context = f.borrow_mut();
        match context.eval(code) {
            Ok(value) => Ok(value.to_string(&mut context).unwrap().as_str().to_string()),
            Err(err) => Err(JSException::new_err(
                err.to_string(&mut context).unwrap().as_str().to_string(),
            )),
        }
    })
}

#[pymodule]
fn boa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(eval_js, m)?)?;
    Ok(())
}
