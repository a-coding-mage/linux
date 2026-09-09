/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * SVM helper functions
 *
 * Copyright 2018 IBM Corporation
 */

// Dependency intent: symbols from <asm/svm.h> and <linux/types.h> are
// supplied by other translated files.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn is_secure_guest() -> bool;

    pub fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int;
    pub fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int;
}

pub unsafe fn force_dma_unencrypted(dev: *mut device) -> bool {
    let _ = dev;
    unsafe { is_secure_guest() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
