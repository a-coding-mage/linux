// SPDX-License-Identifier: GPL-2.0

use std::io::{self, Read, Write};
use std::process;

const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const SELFMAG: usize = 4;
const ELFMAG: [u8; SELFMAG] = [0x7f, b'E', b'L', b'F'];

fn main() {
    let mut ei = [0u8; EI_NIDENT];
    let mut stdin = io::stdin();

    if stdin.read_exact(&mut ei).is_err() {
        let _ = writeln!(io::stderr(), "Error: input truncated");
        process::exit(1);
    }
    if ei[..SELFMAG] != ELFMAG {
        let _ = writeln!(io::stderr(), "Error: not ELF");
        process::exit(1);
    }
    match ei[EI_CLASS] {
        ELFCLASS32 => {
            println!("#define KERNEL_ELFCLASS ELFCLASS32");
        }
        ELFCLASS64 => {
            println!("#define KERNEL_ELFCLASS ELFCLASS64");
        }
        _ => {
            process::exit(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
