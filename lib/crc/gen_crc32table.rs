// SPDX-License-Identifier: GPL-2.0
//! Generate the generic CRC32 and CRC32C lookup tables at kernel build time.

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

#[path = "../../include/linux/crc32poly_header.rs"]
// The shared translated header exposes constants for its library consumers.
#[allow(unreachable_pub)]
mod crc32poly;
use crc32poly::{CRC32C_POLY_LE, CRC32_POLY_BE, CRC32_POLY_LE};

// As in the C generator, the table is linear: table[i ^ j] = table[i] ^ table[j].
fn crc32init_le(polynomial: u32) -> [u32; 256] {
    let mut table = [0; 256];
    let mut crc = 1;
    let mut i = 128;
    while i != 0 {
        crc = (crc >> 1) ^ if crc & 1 != 0 { polynomial } else { 0 };
        for j in (0..256).step_by(2 * i) {
            table[i + j] = crc ^ table[j];
        }
        i >>= 1;
    }
    table
}

fn crc32init_be() -> [u32; 256] {
    let mut table = [0; 256];
    let mut crc = 0x80000000u32;
    let mut i = 1;
    while i < 256 {
        crc = (crc << 1)
            ^ if crc & 0x80000000 != 0 {
                CRC32_POLY_BE
            } else {
                0
            };
        for j in 0..i {
            table[i + j] = crc ^ table[j];
        }
        i <<= 1;
    }
    table
}

fn output_table(table: &[u32; 256]) {
    for row in table.chunks_exact(4) {
        println!(
            "\t0x{:08x}, 0x{:08x}, 0x{:08x}, 0x{:08x},",
            row[0], row[1], row[2], row[3]
        );
    }
}

#[no_mangle]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    println!("/* this file is generated - do not edit */\n");
    for (name, table) in [
        ("crc32table_le", crc32init_le(CRC32_POLY_LE)),
        ("crc32table_be", crc32init_be()),
        ("crc32ctable_le", crc32init_le(CRC32C_POLY_LE)),
    ] {
        println!("static const u32 ____cacheline_aligned {name}[256] = {{");
        output_table(&table);
        println!("}};");
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
