//! Native Python extension for `secure-core-ffi`, built with PyO3.
//!
//! This crate is a thin wrapper: all AES-256-GCM logic lives in the
//! `SecurityContext` type from the main `secure-core-ffi` crate. This
//! module only translates between Rust `Result`s and Python exceptions.
//!
//! Uses the PyO3 0.22+ `Bound<'py, T>` API.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use security_core::{CoreError, SecurityContext as CoreContext};

fn to_py_err(err: CoreError) -> PyErr {
    match err {
        CoreError::InvalidKeyLength => PyValueError::new_err(err.to_string()),
        CoreError::PayloadTooShort => PyValueError::new_err(err.to_string()),
        CoreError::EncryptionFailed | CoreError::DecryptionFailed => {
            PyRuntimeError::new_err(err.to_string())
        }
    }
}

/// AES-256-GCM security core, backed by the same Rust implementation used
/// by the C/C++/Node.js and WebAssembly bindings.
#[pyclass(module = "secure_core_ffi")]
struct SecurityCore {
    inner: CoreContext,
}

#[pymethods]
impl SecurityCore {
    /// Creates a new security core from a 32-byte (256-bit) key.
    #[new]
    fn new(key: &[u8]) -> PyResult<Self> {
        CoreContext::new(key)
            .map(|inner| SecurityCore { inner })
            .map_err(to_py_err)
    }

    /// Encrypts `data` and returns `nonce || ciphertext || tag` as `bytes`.
    fn encrypt<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        let out = self.inner.encrypt(data).map_err(to_py_err)?;
        Ok(PyBytes::new(py, &out))
    }

    /// Decrypts a payload produced by `encrypt` and returns the plaintext
    /// as `bytes`. Raises `RuntimeError` if the key is wrong or the data
    /// was corrupted/tampered with.
    fn decrypt<'py>(&self, py: Python<'py>, payload: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        let out = self.inner.decrypt(payload).map_err(to_py_err)?;
        Ok(PyBytes::new(py, &out))
    }
}

/// `secure_core_ffi`: native Python bindings for the `secure-core-ffi`
/// AES-256-GCM security core.
#[pymodule]
fn secure_core_ffi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SecurityCore>()?;
    Ok(())
}
