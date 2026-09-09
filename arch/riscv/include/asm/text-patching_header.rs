/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 SiFive
 */

use core::ffi::c_void;

pub unsafe extern "C" {
    pub fn patch_insn_write(addr: *mut c_void, insn: *const c_void, len: usize) -> i32;
    pub fn patch_text_nosync(addr: *mut c_void, insns: *const c_void, len: usize) -> i32;
    pub fn patch_text_set_nosync(addr: *mut c_void, c: u8, len: usize) -> i32;
    pub fn patch_text(addr: *mut c_void, insns: *mut u32, len: usize) -> i32;

    pub static mut riscv_patch_in_stop_machine: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
