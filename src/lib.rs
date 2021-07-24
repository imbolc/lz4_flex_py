use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

create_exception!(lz4_flex, Lz4FlexError, PyException);

/// Compress all bytes of input into output.
/// The uncompressed size will be prepended as a little endian u32.
/// Can be used in conjunction with `decompress_size_prepended`
#[pyfunction]
fn compress_prepend_size(py: Python, input: &[u8]) -> PyObject {
    let data = lz4_flex::compress_prepend_size(input);
    PyBytes::new(py, &data).into()
}

/// Decompress all bytes of input.
/// The first 4 bytes are the uncompressed size in litte endian.
/// Can be used in conjunction with `compress_prepend_size`
#[pyfunction]
fn decompress_size_prepended(py: Python, input: &[u8]) -> PyResult<PyObject> {
    let data = lz4_flex::decompress_size_prepended(input)
        .map_err(|e| Lz4FlexError::new_err(e.to_string()))?;
    Ok(PyBytes::new(py, &data).into())
}

#[pymodule]
fn lz4_flex(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("Lz4FlexError", py.get_type::<Lz4FlexError>())?;
    m.add_function(wrap_pyfunction!(compress_prepend_size, m)?)?;
    m.add_function(wrap_pyfunction!(decompress_size_prepended, m)?)?;
    Ok(())
}
