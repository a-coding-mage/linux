// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright Altera Corporation (C) 2013. All rights reserved
 */

// The declarations below are supplied by the Linux syscall and architecture
// headers in the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    pub fn sys_ni_syscall();
    pub fn sys_mmap_pgoff();
    pub fn __sys_clone3();
}

// #define __SYSCALL(nr, call) [nr] = (call),
// #define __SYSCALL_WITH_COMPAT(nr, native, compat) __SYSCALL(nr, native)
// #define sys_mmap2 sys_mmap_pgoff
// #define sys_clone3 __sys_clone3

// The __NR_syscalls constant and entries from asm/syscall_table_32.h are
// provided by the corresponding kernel headers.
pub static mut sys_call_table: [*mut c_void; __NR_syscalls] =
    [sys_ni_syscall as *mut c_void; __NR_syscalls];

// #include <asm/syscall_table_32.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
