/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header __ASM_PATCHING_H.
// Dependency intent: u32, u64, and size_t are supplied by the Linux types layer.

unsafe extern "C" {
    pub fn aarch64_insn_read(addr: *mut core::ffi::c_void, insnp: *mut u32) -> i32;
    pub fn aarch64_insn_write(addr: *mut core::ffi::c_void, insn: u32) -> i32;

    pub fn aarch64_insn_write_literal_u64(addr: *mut core::ffi::c_void, val: u64) -> i32;
    pub fn aarch64_insn_set(
        dst: *mut core::ffi::c_void,
        insn: u32,
        len: usize,
    ) -> *mut core::ffi::c_void;
    pub fn aarch64_insn_copy(
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        len: usize,
    ) -> *mut core::ffi::c_void;

    pub fn aarch64_insn_patch_text_nosync(addr: *mut core::ffi::c_void, insn: u32) -> i32;
    pub fn aarch64_insn_patch_text(
        addrs: *mut *mut core::ffi::c_void,
        insns: *mut u32,
        cnt: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
