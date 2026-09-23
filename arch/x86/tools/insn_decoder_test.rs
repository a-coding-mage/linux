// SPDX-License-Identifier: GPL-2.0-or-later
//! Compare x86 instruction lengths with reformatted objdump output.
// Copyright (C) IBM Corporation, 2009.

#[allow(dead_code)] // Both instruction-test tools import the shared full API.
mod insn_test_common;
use insn_test_common::{
    cstr,
    decoder::{Instruction, Mode},
    dump_insn, fgets, unsigned, Options,
};
use std::ffi::OsString;
use std::io::{self, BufRead, Write};
use std::os::unix::ffi::OsStrExt;

// Match the reference's 256 + KSYM_NAME_LEN (512) fgets capacity. Differential
// long-symbol tests guard this external format boundary if the header changes.
const LINE_SIZE: usize = 768;

fn usage(program: &[u8]) -> Vec<u8> {
    let mut out = b"Usage: objdump -d a.out | awk -f objdump_reformat.awk | ".to_vec();
    out.extend_from_slice(program);
    out.extend_from_slice(
        b" [-y|-n] [-v]\n\t-y\t64bit mode\n\t-n\t32bit mode\n\t-v\tverbose mode\n",
    );
    out
}

fn warning(output: &mut dyn Write, program: &[u8], text: &[u8]) -> io::Result<()> {
    output.write_all(program)?;
    output.write_all(b": warning: ")?;
    output.write_all(text)
}

fn malformed(output: &mut dyn Write, program: &[u8], number: i32, line: &[u8]) -> io::Result<i32> {
    output.write_all(program)?;
    output.write_all(format!(": error: malformed line {number}:\n").as_bytes())?;
    output.write_all(line)?;
    Ok(3)
}

fn run(
    args: &[OsString],
    input: &mut dyn BufRead,
    output: &mut dyn Write,
    errors: &mut dyn Write,
) -> io::Result<i32> {
    let program = args[0].as_bytes();
    let (mut mode, mut verbose) = (Mode::Bits32, false);
    for option in Options::new(args, b"ynv") {
        match option {
            Ok((b'y', _)) => mode = Mode::Bits64,
            Ok((b'n', _)) => mode = Mode::Bits32,
            Ok((b'v', _)) => verbose = true,
            Ok(_) => unreachable!(),
            Err(message) => {
                errors.write_all(&message)?;
                errors.write_all(&usage(program))?;
                return Ok(1);
            }
        }
    }
    let (mut instructions, mut warnings) = (0i32, 0i32);
    while let Ok(Some(raw)) = fgets(input, LINE_SIZE) {
        let line = cstr(&raw.bytes);
        if line.first() == Some(&b'<') {
            continue;
        }
        instructions = instructions.wrapping_add(1);
        let Some(first_tab) = line.iter().position(|&byte| byte == b'\t') else {
            return malformed(errors, program, instructions, line);
        };
        let mut at = first_tab + 1;
        while line.get(at) == Some(&b' ') {
            at += 1;
        }
        let Some(second_tab) = line[at..].iter().position(|&byte| byte == b'\t') else {
            return malformed(errors, program, instructions, line);
        };
        let end = at + second_tab;
        let mut bytes = [0u8; 16];
        let mut count = 0;
        while at < end {
            let (value, consumed) = unsigned(&line[at..end], 16);
            if consumed == 0 {
                break;
            }
            let Some(byte) = bytes.get_mut(count) else {
                // The C tool overruns its 16-byte stack buffer on this input.
                return malformed(errors, program, instructions, line);
            };
            *byte = value as u8;
            count += 1;
            at += 3;
        }
        let mut insn = Instruction::new(&bytes, mode);
        if insn.decode().is_err() || insn.length as usize != count {
            warnings = warnings.wrapping_add(1);
            warning(
                errors,
                program,
                b"Found an x86 instruction decoder bug, please report this.\n",
            )?;
            warning(errors, program, line)?;
            warning(
                errors,
                program,
                format!(
                    "objdump says {count} bytes, but insn_get_length() says {}\n",
                    insn.length
                )
                .as_bytes(),
            )?;
            if verbose {
                let mut dump = Vec::new();
                dump_insn(&mut dump, &insn);
                errors.write_all(&dump)?;
            }
        }
    }
    if warnings != 0 {
        warning(
            errors,
            program,
            format!("Decoded and checked {instructions} instructions with {warnings} failures\n")
                .as_bytes(),
        )?;
    } else {
        output.write_all(b"  ")?;
        output.write_all(program)?;
        output.write_all(
            format!(": success: Decoded and checked {instructions} instructions\n").as_bytes(),
        )?;
    }
    Ok(0)
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let status = match run(
        &args,
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
    ) {
        Ok(status) => status,
        Err(err) => {
            let _ = io::stderr().write_all(&insn_test_common::error(b"write", &err));
            1
        }
    };
    std::process::exit(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
