use boa_engine::Context;
use pyo3::create_exception;
use pyo3::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

create_exception!(boa, JSException, pyo3::exceptions::PyException);

thread_local!(static CONTEXT_COLLECTIONS: RefCell<HashMap<String, RefCell<Context>>> = RefCell::new(HashMap::new()));

fn context_eval(context_cell: &RefCell<Context>, code: &str) -> PyResult<String> {
    let mut context = context_cell.borrow_mut();
    match context.eval(code) {
        Ok(value) => Ok(value.to_string(&mut context).unwrap().as_str().to_string()),
        Err(err) => Err(JSException::new_err(
            err.to_string(&mut context).unwrap().as_str().to_string(),
        )),
    }
}

#[pyclass(subclass)]
struct Interpreter {
    name: String,
}

#[pymethods]
impl Interpreter {
    #[new]
    fn new() -> Self {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        CONTEXT_COLLECTIONS.with(|f| {
            let mut context_dict = f.borrow_mut();
            let context = RefCell::new(Context::default());
            context_dict.insert(random_string.to_string(), context)
        });
        return Self {
            name: random_string.to_string(),
        };
    }

    pub fn execute(&self, code: &str) -> PyResult<String> {
        CONTEXT_COLLECTIONS.with(|f| {
            let context_dict = f.borrow_mut();
            let context_cell = &context_dict[&self.name];
            context_eval(context_cell, code)
        })
    }
}

#[pyfunction]
fn eval_js(code: &str) -> PyResult<String> {
    context_eval(&RefCell::new(Context::default()), code)
}

#[pymodule]
fn boa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eval_js, m)?)?;
    m.add_class::<Interpreter>()?;
    Ok(())
}
