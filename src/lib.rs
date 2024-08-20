#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn fibonacci_py(_: Python, val: u64) -> PyResult<u64> {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..val {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    Ok(a)
}

py_module_initializer!(fibonacci, initlibfibonacci, PyInit_fibonacci, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "fibonacci", py_fn!(py, fibonacci_py(val: u64)))?;
    Ok(())
});


#[cfg(test)]
mod tests {
    use super::*;
    use cpython::Python;

    #[test]
    fn it_works() {
        use cpython::Python;

        let result = fibonacci_py(Python::
            acquire_gil()
            .python(), 4)
            .unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn fibonacci_zero() {
        let result = fibonacci_py(Python::
            acquire_gil()
            .python(), 0)
            .unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn fibonacci_one() {
        let result = fibonacci_py(Python::
            acquire_gil()
            .python(), 1)
            .unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn fibonacci_large_value() {
        let result = fibonacci_py(Python::
            acquire_gil()
            .python(), 10)
            .unwrap();
        assert_eq!(result, 55);
    }
}
