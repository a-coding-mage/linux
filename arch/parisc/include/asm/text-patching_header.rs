/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* stop machine and patch kernel text */
unsafe extern "C" {
    pub fn patch_text(addr: *mut c_void, insn: u32);
    pub fn patch_text_multiple(addr: *mut c_void, insn: *mut u32, len: u32);

    /* patch kernel text with machine already stopped (e.g. in kgdb) */
    pub fn __patch_text(addr: *mut c_void, insn: u32);
    pub fn __patch_text_multiple(addr: *mut c_void, insn: *mut u32, len: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
