// SPDX-License-Identifier: GPL-2.0
//! Binary-coded decimal conversions with the original exported C ABI.
//!
//! This implementation is compiled once through `bcd_rust.rs`. Independent
//! native Rust callers use `kernel::bcd`, which imports only the shared pure
//! helpers and therefore introduces no second copy of these exported symbols.

#[path = "../include/linux/bcd_header.rs"]
mod conversion;

pub use conversion::*;

/// Converts a packed byte exactly as the original C `_bcd2bin` function.
#[no_mangle]
pub extern "C" fn _bcd2bin(val: u8) -> u32 {
    bcd2bin(val)
}

/// Converts an integer with C's wrapping `_bin2bcd` arithmetic and byte result.
#[no_mangle]
pub extern "C" fn _bin2bcd(val: u32) -> u8 {
    bin2bcd(val)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
