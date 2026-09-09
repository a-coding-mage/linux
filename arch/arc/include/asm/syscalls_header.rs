/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// C header guard: _ASM_ARC_SYSCALLS_H

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation unit:
// <linux/compiler.h>, <linux/linkage.h>, <linux/types.h>

unsafe extern "C" {
    pub fn sys_clone_wrapper(a0: i32, a1: i32, a2: i32, a3: i32, a4: i32) -> i32;
    pub fn sys_clone3_wrapper(arg: *mut c_void, size: usize) -> i32;
    // The source declaration contains a missing comma between its two uint32_t parameters.
    pub fn sys_cacheflush(a0: u32, a1: u32) -> i32;
    pub fn sys_arc_settls(arg: *mut c_void) -> i32;
    pub fn sys_arc_gettls() -> i32;
    pub fn sys_arc_usr_cmpxchg(ptr: *mut i32, old: i32, new: i32) -> i32;
}

// Declarations from <asm-generic/syscalls.h> are supplied by another translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
