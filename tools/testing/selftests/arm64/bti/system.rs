// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

// Translated from system.c. External declarations/constants are supplied by
// the corresponding translated headers/dependencies.

extern "C" {
    fn syscall(number: core::ffi::c_long, ...) -> isize;
    fn unreachable() -> !;

    static __NR_exit: core::ffi::c_long;
    static __NR_write: core::ffi::c_long;
}

#[no_mangle]
pub unsafe extern "C" fn exit(n: core::ffi::c_int) -> ! {
    syscall(__NR_exit, n);
    unreachable();
}

#[no_mangle]
pub unsafe extern "C" fn write(
    fd: core::ffi::c_int,
    buf: *const core::ffi::c_void,
    size: usize,
) -> isize {
    syscall(__NR_write, fd, buf, size)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
