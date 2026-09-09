// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC sys_call_table.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// The declarations supplied by linux/syscalls.h, linux/signal.h,
// linux/unistd.h, asm/syscalls.h, and asm/syscall_table_32.h are external
// dependencies of this translation.

#[macro_export]
macro_rules! __SYSCALL {
    ($nr:expr, $call:expr) => {
        [$nr] = $call,
    };
}

#[macro_export]
macro_rules! __SYSCALL_WITH_COMPAT {
    ($nr:expr, $native:expr, $compat:expr) => {
        $crate::__SYSCALL!($nr, $native)
    };
}

// #define sys_mmap2 sys_mmap_pgoff
// #define sys_clone __sys_clone
// #define sys_clone3 __sys_clone3
// #define sys_fork __sys_fork
pub use crate::sys_mmap_pgoff as sys_mmap2;
pub use crate::__sys_clone as sys_clone;
pub use crate::__sys_clone3 as sys_clone3;
pub use crate::__sys_fork as sys_fork;

// The C initializer expands the entries from <asm/syscall_table_32.h>.
// That generated header is an external dependency and is intentionally not
// reimplemented here.
#[allow(non_upper_case_globals)]
pub static mut sys_call_table: [*mut core::ffi::c_void; crate::__NR_syscalls] =
    [core::ptr::null_mut(); crate::__NR_syscalls];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
