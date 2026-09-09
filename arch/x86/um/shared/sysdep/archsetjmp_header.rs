/* SPDX-License-Identifier: GPL-2.0 */

// On i386, the declarations from `archsetjmp_32.h` are required; otherwise,
// the declarations from `archsetjmp_64.h` are required.

use core::ffi::{c_int, c_ulong};

/// Opaque declaration corresponding to the external C `jmp_buf` type.
#[repr(C)]
pub struct jmp_buf {
    _private: [u8; 0],
}

extern "C" {
    pub fn get_thread_reg(reg: c_int, buf: *mut jmp_buf) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
