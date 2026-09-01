/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * signal function definitions for NOLIBC
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

/* Original C header includes nolibc.h to make sure to include all global symbols. */
/* Original dependencies: std.h, arch.h, types.h, sys.h. */

use core::ffi::c_int;

unsafe extern "C" {
    fn _sys_getpid() -> c_int;
    fn _sys_kill(pid: c_int, signal: c_int) -> c_int;
}

/* This one is not marked static as it's needed by libgcc for divide by zero. */
/* Original attributes: weak, unused, section(".text.nolibc_raise"). */
#[no_mangle]
#[link_section = ".text.nolibc_raise"]
pub unsafe extern "C" fn raise(signal: c_int) -> c_int {
    unsafe { _sys_kill(_sys_getpid(), signal) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
