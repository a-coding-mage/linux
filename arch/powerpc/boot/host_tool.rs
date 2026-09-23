// SPDX-License-Identifier: GPL-2.0
//! Byte-preserving diagnostics shared by the PowerPC boot-image host tools.

use std::io::{self, Write};
use std::path::Path;

pub(crate) struct Failure {
    status: i32,
    message: Vec<u8>,
}

impl Failure {
    pub(crate) fn new(status: i32, message: impl AsRef<[u8]>) -> Self {
        Self {
            status,
            message: message.as_ref().to_owned(),
        }
    }

    pub(crate) fn path(status: i32, prefix: &str, path: &Path, suffix: &str) -> Self {
        let mut message = prefix.as_bytes().to_vec();
        message.extend_from_slice(path.as_os_str().as_encoded_bytes());
        message.extend_from_slice(suffix.as_bytes());
        Self::new(status, message)
    }

    pub(crate) fn io(status: i32, prefix: &[u8], error: io::Error) -> Self {
        let text = error.to_string();
        let text = text.split(" (os error ").next().unwrap_or(&text);
        let mut message = prefix.to_vec();
        if !prefix.is_empty() {
            message.extend_from_slice(b": ");
        }
        message.extend_from_slice(text.as_bytes());
        message.push(b'\n');
        Self::new(status, message)
    }

    pub(crate) fn print(&self) {
        let _ = io::stderr().lock().write_all(&self.message);
    }
}

pub(crate) fn finish(result: Result<(), Failure>) {
    if let Err(error) = result {
        error.print();
        std::process::exit(error.status);
    }
}
