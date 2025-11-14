#![allow(clippy::borrow_deref_ref)] // TODO: broken clippy lint?
// Copyright 2021-2024 SecureDNA Stiftung (SecureDNA Foundation) <licensing@securedna.org>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::convert::TryFrom;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction; 

use crate::{
    errors::TranslationError,
    trans_table::{reverse_complement_bytes, TranslationTable},
    Nucleotide, NucleotideAmbiguous,
};

impl From<TranslationError> for PyErr {
    fn from(err: TranslationError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

#[pyfunction]
fn _check_table(table: u8) -> PyResult<()> {
    TranslationTable::try_from(table)?;
    Ok(())
}

#[pyfunction]
fn _translate(py: Python, table: u8, dna: &[u8]) -> PyResult<Py<PyAny>> {
    let table = TranslationTable::try_from(table)?;
    // 'dna' is now already a byte slice, so we pass it directly.
    let bytes = table.translate_dna_bytes::<NucleotideAmbiguous>(dna)?; 
    Ok(PyBytes::new(py, &bytes).into())
}

#[pyfunction]
fn _translate_strict(py: Python, table: u8, dna: &[u8]) -> PyResult<Py<PyAny>> {
    let table = TranslationTable::try_from(table)?;
    let bytes = table.translate_dna_bytes::<Nucleotide>(dna)?;
    Ok(PyBytes::new(py, &bytes).into())
}

#[pyfunction]
fn _reverse_complement(py: Python, dna: &[u8]) -> PyResult<Py<PyAny>> {
    let bytes = reverse_complement_bytes::<NucleotideAmbiguous>(dna)?;
    Ok(PyBytes::new(py, &bytes).into())
}

#[pyfunction]
fn _reverse_complement_strict(py: Python, dna: &[u8]) -> PyResult<Py<PyAny>> {
    let bytes = reverse_complement_bytes::<Nucleotide>(dna)?;
    Ok(PyBytes::new(py, &bytes).into())
}

#[pymodule]
fn quickdna(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_check_table, m)?)?;
    m.add_function(wrap_pyfunction!(_translate, m)?)?;
    m.add_function(wrap_pyfunction!(_translate_strict, m)?)?;
    m.add_function(wrap_pyfunction!(_reverse_complement, m)?)?;
    m.add_function(wrap_pyfunction!(_reverse_complement_strict, m)?)?;
    Ok(())
}
