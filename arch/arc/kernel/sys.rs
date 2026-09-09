// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit:
// linux/syscalls.h, linux/signal.h, linux/unistd.h, asm/syscalls.h

// C preprocessor aliases:
// #define sys_clone  sys_clone_wrapper
// #define sys_clone3 sys_clone3_wrapper
// #define sys_mmap2  sys_mmap_pgoff

// C preprocessor syscall-table expansion macros:
macro_rules! __SYSCALL {
    ($nr:expr, $call:expr) => {
        [$nr] = $call,
    };
}

macro_rules! __SYSCALL_WITH_COMPAT {
    ($nr:expr, $native:expr, $compat:expr) => {
        __SYSCALL!($nr, $native)
    };
}

extern "C" {
    fn sys_ni_syscall();
}

// NR_syscalls is supplied by the translated kernel headers.
pub static mut sys_call_table: [*mut core::ffi::c_void; NR_syscalls] =
    [sys_ni_syscall as *mut core::ffi::c_void; NR_syscalls];

// The C source includes <asm/syscall_table_32.h> here. Its generated
// __SYSCALL entries are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
