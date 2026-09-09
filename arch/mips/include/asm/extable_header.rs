/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct exception_table_entry {
    pub insn: usize,
    pub nextinsn: usize,
}

// Opaque forward declaration corresponding to `struct pt_regs`.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn fixup_exception(regs: *mut pt_regs) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
