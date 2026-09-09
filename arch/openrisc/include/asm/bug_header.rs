/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent: declarations from <asm-generic/bug.h> are supplied by
// the corresponding translated dependency.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn die(str_: *const core::ffi::c_char, regs: *mut pt_regs, err: core::ffi::c_long) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
