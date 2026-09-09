// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 *
 * This file initializes secvar operations for PowerPC Secureboot
 */

// Dependency-provided declarations from <linux/cache.h>, <asm/secvar.h>, and
// <asm/bug.h> are referenced here without reproducing their implementations.

#[repr(C)]
pub struct secvar_operations {
    _private: [u8; 0],
}

pub static mut secvar_ops: *const secvar_operations = core::ptr::null();

// Linux errno value for EBUSY.
const EBUSY: i32 = 16;

pub unsafe fn set_secvar_ops(ops: *const secvar_operations) -> i32 {
    if WARN_ON_ONCE!(secvar_ops) {
        return -EBUSY;
    }

    secvar_ops = ops;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
