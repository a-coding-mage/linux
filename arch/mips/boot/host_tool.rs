// SPDX-License-Identifier: GPL-2.0
//! Byte-preserving host-tool diagnostics shared by MIPS boot generators.

use std::io::{self, Write};

pub(crate) fn errno(error: &io::Error) -> String {
    let message = error.to_string();
    message
        .split(" (os error ")
        .next()
        .unwrap_or(&message)
        .to_owned()
}

pub(crate) fn perror(prefix: &str, error: io::Error) -> Vec<u8> {
    format!("{prefix}: {}\n", errno(&error)).into_bytes()
}

pub(crate) fn diagnostic(message: impl AsRef<[u8]>) {
    let _ = io::stderr().lock().write_all(message.as_ref());
}

pub(crate) fn finish(result: Result<(), Vec<u8>>) {
    if let Err(message) = result {
        diagnostic(message);
        std::process::exit(1);
    }
}
