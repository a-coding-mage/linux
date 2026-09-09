/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn patch_text(addr: *mut core::ffi::c_void, insn: u32);
    pub fn __patch_text_real(
        addr: *mut core::ffi::c_void,
        insn: u32,
        remap: bool,
    );
}

#[inline]
pub unsafe fn __patch_text(addr: *mut core::ffi::c_void, insn: u32) {
    __patch_text_real(addr, insn, true);
}

#[inline]
pub unsafe fn __patch_text_early(addr: *mut core::ffi::c_void, insn: u32) {
    __patch_text_real(addr, insn, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
