use pyo3::ffi;
use pyo3::ffi::PyBUF_WRITE;
use pyo3::prelude::*;
use pyo3::types::PyMemoryView;
use pyo3::Bound;

pub trait FromPtr {
    unsafe fn from_raw_ptr<'py>(
        py: Python<'py>,
        src: *mut u8,
        size: usize,
    ) -> PyResult<Bound<'py, PyMemoryView>>;
}

impl FromPtr for PyMemoryView {
    unsafe fn from_raw_ptr<'py>(
        py: Python<'py>,
        src: *mut u8,
        size: usize,
    ) -> PyResult<Bound<'py, Self>> {
        unsafe {
            let x = Bound::from_owned_ptr_or_err(
                py,
                ffi::PyMemoryView_FromMemory(src as *mut i8, size as isize, PyBUF_WRITE),
            )?;
            Ok(x.downcast_into_unchecked())
        }
    }
}
