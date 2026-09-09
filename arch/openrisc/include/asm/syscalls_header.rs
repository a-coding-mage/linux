/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// C header guard: __ASM_OPENRISC_SYSCALLS_H

use core::ffi::c_void;

// `asmlinkage` is an architecture/compiler calling-convention annotation in C.
unsafe extern "C" {
    pub fn sys_or1k_atomic(
        type_: core::ffi::c_ulong,
        v1: *mut core::ffi::c_ulong,
        v2: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_long;
}

// Dependency intent preserved from: #include <asm-generic/syscalls.h>

// `struct clone_args` and `size_t` are supplied by other headers/dependencies.
unsafe extern "C" {
    pub fn __sys_clone(
        clone_flags: core::ffi::c_ulong,
        newsp: core::ffi::c_ulong,
        parent_tid: *mut c_void,
        child_tid: *mut c_void,
        tls: core::ffi::c_int,
    ) -> core::ffi::c_long;

    pub fn __sys_clone3(uargs: *mut clone_args, size: usize) -> core::ffi::c_long;

    pub fn __sys_fork() -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
