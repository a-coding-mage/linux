// SPDX-License-Identifier: GPL-2.0-or-later
//! Generate the compact CPU feature strings consumed by x86 setup code.
// Copyright 2008 rPath, Inc. - All Rights Reserved

// The shared definitions export every CPU and bug bit for target-kernel users.
// This host consumer only needs the name table and its dimension.
#[allow(dead_code, missing_docs, non_snake_case, unreachable_pub)]
#[path = "../include/asm/cpufeatures_header.rs"]
mod cpufeatures;

use std::io::{self, Write};
use std::process::ExitCode;

fn write_table(output: &mut impl Write, flags: &[Option<&str>]) -> io::Result<()> {
    writeln!(output, "#include <asm/cpufeaturemasks.h>\n")?;
    writeln!(output, "static const char x86_cap_strs[] =")?;
    for (index, flag) in flags.iter().enumerate() {
        let word = index / 32;
        let bit = index % 32;
        if index + 1 == flags.len() {
            // The last record is unconditional and uses the compiler's final NUL.
            writeln!(
                output,
                "\t\"\\x{word:02x}\\x{bit:02x}\"\"{}\"",
                flag.unwrap_or("")
            )?;
        } else if let Some(name) = flag {
            writeln!(output, "#if REQUIRED_MASK{word} & (1 << {bit})\n\t\"\\x{word:02x}\\x{bit:02x}\"\"{name}\\0\"\n#endif")?;
        }
    }
    writeln!(output, "\t;")
}

fn main() -> ExitCode {
    let mut output = io::BufWriter::new(io::stdout().lock());
    match write_table(&mut output, &cpufeatures::X86_CAP_FLAGS).and_then(|()| output.flush()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("mkcpustr: {error}");
            ExitCode::FAILURE
        }
    }
}
