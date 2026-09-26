// SPDX-License-Identifier: GPL-2.0
//! Exercise the actual generated errname declaration through a protected call.

use kernel::ffi::{c_char, c_int};

/// Return the selected provider's permanent errno string through its bindings.
#[inline(never)]
#[no_mangle]
pub extern "C" fn errname_rust_call(error: c_int) -> *const c_char {
    let actual: unsafe extern "C" fn(c_int) -> *const c_char = kernel::bindings::errname;
    // SAFETY: errname accepts every int and returns permanent storage or null.
    unsafe { core::ptr::read_volatile(&actual)(error) }
}

/// Call the safe public Rust bit-reversal helpers with the C input widths.
#[no_mangle]
pub extern "C" fn bitrev_rust_call(operation: u32, value: u64) -> u32 {
    match operation {
        0 => kernel::bitrev::bitrev8(value as u8) as u32,
        1 => kernel::bitrev::bitrev16(value as u16) as u32,
        2 => kernel::bitrev::bitrev32(value as u32),
        3 => kernel::bitrev::bitrev8x4(value as u32),
        _ => 0,
    }
}
