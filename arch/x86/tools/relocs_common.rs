// SPDX-License-Identifier: GPL-2.0
//! Command-line interface for kernel relocation extraction.

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::relocs_header::Options;
use crate::{Failure, Result};

const USAGE: &str = "relocs [--abs-syms|--abs-relocs|--reloc-info|--text|--realmode] vmlinux\n";

fn io_error(error: std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error ").next().unwrap_or(&text).to_owned()
}

pub(crate) fn run() -> Result<()> {
    let mut options = Options::default();
    let mut filename = None;
    for argument in std::env::args_os().skip(1) {
        match argument.to_str() {
            Some("--abs-syms") => options.absolute_symbols = true,
            Some("--abs-relocs") => options.absolute_relocations = true,
            Some("--reloc-info") => options.relocation_info = true,
            Some("--text") => options.text = true,
            Some("--realmode") => options.real_mode = true,
            _ => {
                if argument.as_encoded_bytes().starts_with(b"-") || filename.is_some() {
                    return Err(USAGE.into());
                }
                filename = Some(PathBuf::from(argument));
            }
        }
    }
    let filename = filename.ok_or_else(|| Failure::from(USAGE))?;
    let mut file = File::open(&filename).map_err(|error| {
        Failure::named(
            "Cannot open ",
            filename.as_os_str().as_encoded_bytes(),
            &format!(": {}\n", io_error(error)),
        )
    })?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|error| {
        Failure::named(
            "Cannot read ",
            filename.as_os_str().as_encoded_bytes(),
            &format!(": {}", io_error(error)),
        )
    })?;
    if bytes.len() < 16 {
        return Err(Failure::named(
            "Cannot read ",
            filename.as_os_str().as_encoded_bytes(),
            ": Success",
        ));
    }
    let output = crate::process(&bytes, &options)?;
    std::io::stdout()
        .lock()
        .write_all(&output)
        .map_err(|error| format!("Cannot write relocations: {}\n", io_error(error)).into())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
