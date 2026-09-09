/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright Altera Corporation (C) 2013. All rights reserved
 */

// C header guard: __ASM_NIOS2_SYSCALLS_H

unsafe extern "C" {
    pub fn sys_cacheflush(addr: core::ffi::c_ulong, len: core::ffi::c_ulong,
                           op: core::ffi::c_uint) -> core::ffi::c_int;

    // asmlinkage; `uargs` is a __user pointer in the C declaration.
    pub fn __sys_clone3(uargs: *mut clone_args, size: usize) -> core::ffi::c_long;
}

// C dependency: #include <asm-generic/syscalls.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
