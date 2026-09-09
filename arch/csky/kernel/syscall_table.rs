// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Translated from linux/syscalls.h and asm/syscalls.h.

// C preprocessor mapping:
// #undef __SYSCALL
// #define __SYSCALL(nr, call) [nr] = (call),
// #define __SYSCALL_WITH_COMPAT(nr, native, compat) __SYSCALL(nr, native)

// C preprocessor aliases:
// #define sys_fadvise64_64 sys_csky_fadvise64_64
// #define sys_sync_file_range sys_sync_file_range2

unsafe extern "C" {
    static sys_ni_syscall: *const core::ffi::c_void;
}

// __NR_syscalls is supplied by the target syscall definitions. The included
// asm/syscall_table_32.h expansion supplies the syscall-specific entries.
#[no_mangle]
pub static mut sys_call_table: [*const core::ffi::c_void; __NR_syscalls] = {
    let mut table = [sys_ni_syscall; __NR_syscalls];
    // asm/syscall_table_32.h:
    // __SYSCALL(nr, call) expands to table[nr] = call.
    table
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
