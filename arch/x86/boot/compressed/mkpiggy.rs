// SPDX-License-Identifier: GPL-2.0-only
/* ----------------------------------------------------------------------- *
 *
 *  Copyright (C) 2009 Intel Corporation. All rights reserved.
 *
 *  H. Peter Anvin <hpa@linux.intel.com>
 *
 * -----------------------------------------------------------------------
 *
 * Outputs a small assembly wrapper with the appropriate symbols defined.
 */

use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let mut retval: i32 = 1;

    if argc < 2 {
        let _ = writeln!(io::stderr(), "Usage: {} compressed_file", argv[0]);
        process::exit(retval);
    }

    /* Get the information for the compressed kernel image first */

    let mut f = match File::open(&argv[1]) {
        Ok(file) => Some(file),
        Err(error) => {
            let _ = writeln!(io::stderr(), "{}: {}", argv[1], error);
            None
        }
    };

    if f.is_none() {
        process::exit(retval);
    }

    let file = f.as_mut().unwrap();

    if let Err(error) = file.seek(SeekFrom::End(-4)) {
        let _ = writeln!(io::stderr(), "{}: {}", argv[1], error);
    }

    let mut olen_bytes = [0u8; 4];
    if let Err(error) = file.read_exact(&mut olen_bytes) {
        let _ = writeln!(io::stderr(), "{}: {}", argv[1], error);
        process::exit(retval);
    }

    let ilen: i64 = match file.stream_position() {
        Ok(position) => position as i64,
        Err(error) => {
            let _ = writeln!(io::stderr(), "{}: {}", argv[1], error);
            process::exit(retval);
        }
    };
    let olen: u32 = u32::from_le_bytes(olen_bytes);

    println!(".section \".rodata..compressed\",\"a\",@progbits");
    println!(".globl z_input_len");
    println!("z_input_len = {}", ilen);
    println!(".globl z_output_len");
    println!("z_output_len = {}", olen as u64);

    println!(".globl input_data, input_data_end");
    println!("input_data:");
    println!(".incbin \"{}\"", argv[1]);
    println!("input_data_end:");

    println!(".section \".rodata\",\"a\",@progbits");
    println!(".globl input_len");
    println!("input_len:\n\t.long {}", ilen);
    println!(".globl output_len");
    println!("output_len:\n\t.long {}", olen as u64);

    retval = 0;
    drop(f);
    process::exit(retval);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
