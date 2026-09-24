// SPDX-License-Identifier: GPL-2.0
//! Generate the generic CRC64 lookup tables at kernel build time.
/*
 * Copyright 2018 SUSE Linux.
 *   Author: Coly Li <colyli@suse.de>
 */
// C startup preserves inherited SIGPIPE and ignores argv, including non-UTF8.
#![no_main]

use std::ffi::{c_char, c_int};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

struct CStdout;

impl std::fmt::Write for CStdout {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        // SAFETY: the format is NUL terminated and precision bounds the read
        // to this live string. All generated fragments are short ASCII.
        // Ignore printf errors, just as the original C generator does.
        unsafe {
            printf(c"%.*s".as_ptr(), text.len() as c_int, text.as_ptr());
        }
        Ok(())
    }
}

macro_rules! println {
    ($($args:tt)*) => {{
        let _ = std::fmt::write(&mut CStdout, format_args!("{}\n", format_args!($($args)*)));
    }};
}

const CRC64_ECMA182_POLY: u64 = 0x42F0E1EBA9EA3693;
const CRC64_NVME_POLY: u64 = 0x9A6C9329AC4BC9B5;

fn generate_reflected_crc64_table(poly: u64) -> [u64; 256] {
    let mut table = [0; 256];
    for (i, entry) in table.iter_mut().enumerate() {
        let mut crc = 0;
        let c = i as u64;
        for j in 0..8 {
            crc = (crc >> 1) ^ if (crc ^ (c >> j)) & 1 != 0 { poly } else { 0 };
        }
        *entry = crc;
    }
    table
}

fn generate_crc64_table(poly: u64) -> [u64; 256] {
    let mut table = [0; 256];
    for (i, entry) in table.iter_mut().enumerate() {
        let mut crc = 0;
        let mut c = (i as u64) << 56;
        for _ in 0..8 {
            crc = (crc << 1)
                ^ if (crc ^ c) & 0x8000000000000000 != 0 {
                    poly
                } else {
                    0
                };
            c <<= 1;
        }
        *entry = crc;
    }
    table
}

fn output_table(table: &[u64; 256]) {
    for row in table.chunks_exact(2) {
        println!("\t0x{:016x}ULL, \t0x{:016x}ULL,", row[0], row[1]);
    }
    println!("}};");
}

#[no_mangle]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    println!("/* this file is generated - do not edit */\n");
    println!("#include <linux/types.h>");
    println!("#include <linux/cache.h>\n");
    println!("static const u64 ____cacheline_aligned crc64table[256] = {{");
    output_table(&generate_crc64_table(CRC64_ECMA182_POLY));
    println!("\nstatic const u64 ____cacheline_aligned crc64nvmetable[256] = {{");
    output_table(&generate_reflected_crc64_table(CRC64_NVME_POLY));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
