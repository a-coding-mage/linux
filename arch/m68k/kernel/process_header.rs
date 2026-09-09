/* SPDX-License-Identifier: GPL-2.0 */

// The C `asmlinkage` calling-convention annotation is represented by the
// platform's external C ABI here.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn m68k_clone(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn m68k_clone3(regs: *mut pt_regs) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
