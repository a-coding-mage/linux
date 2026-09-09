/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2008 Michael Neuling IBM Corporation
 */

// C header guard: _ASM_POWERPC_SETJMP_H

pub const JMP_BUF_LEN: usize = 23;

pub type JmpBuf = [core::ffi::c_long; JMP_BUF_LEN];

extern "C" {
    // C attribute: returns_twice
    pub fn setjmp(env: *mut core::ffi::c_long) -> core::ffi::c_int;
    // C attribute: noreturn
    pub fn longjmp(env: *mut core::ffi::c_long, val: core::ffi::c_int) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
