// SPDX-License-Identifier: GPL-2.0-only
/*
 * System call table for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation.

pub const __SYSCALL_WITH_COMPAT: () = ();

// #define sys_mmap2 sys_mmap_pgoff
// #define sys_fadvise64_64 sys_hexagon_fadvise64_64

unsafe extern "C" {
    fn ksys_fadvise64_64(
        fd: i32,
        offset: i64,
        len: i64,
        advice: i32,
    ) -> isize;
}

pub unsafe extern "C" fn sys_hexagon_fadvise64_64(
    fd: i32,
    advice: i32,
    offset: u64,
    len: u64,
) -> isize {
    ksys_fadvise64_64(fd, offset as i64, len as i64, advice)
}

// The C initializer expands the architecture-provided syscall_table_32.h
// through __SYSCALL(nr, call).  The included entries are supplied externally.
pub static mut sys_call_table: [*mut core::ffi::c_void; __NR_syscalls] =
    [core::ptr::null_mut(); __NR_syscalls];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
