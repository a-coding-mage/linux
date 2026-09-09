// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of the memory encryption/decryption API.
 *
 * Since the low-level details of the operation depend on the
 * Confidential Computing environment (e.g. pKVM, CCA, ...), this just
 * acts as a top-level dispatcher to whatever hooks may have been
 * registered.
 *
 * Author: Will Deacon <will@kernel.org>
 * Copyright (C) 2024 Google LLC
 *
 * "Hello, boils and ghouls!"
 */

use core::ffi::c_ulong;

// Declarations supplied by the corresponding kernel headers.
#[repr(C)]
pub struct arm64_mem_crypt_ops {
    pub encrypt: Option<unsafe extern "C" fn(addr: c_ulong, numpages: i32) -> i32>,
    pub decrypt: Option<unsafe extern "C" fn(addr: c_ulong, numpages: i32) -> i32>,
}

static mut crypt_ops: *const arm64_mem_crypt_ops = core::ptr::null();

pub unsafe extern "C" fn arm64_mem_crypt_ops_register(
    ops: *const arm64_mem_crypt_ops,
) -> i32 {
    // WARN_ON(crypt_ops): preserve the warning condition; the warning
    // facility is supplied by the kernel environment.
    if !crypt_ops.is_null() {
        return -16; // -EBUSY
    }

    crypt_ops = ops;
    0
}

pub unsafe extern "C" fn set_memory_encrypted(addr: c_ulong, numpages: i32) -> i32 {
    // likely(!crypt_ops) || WARN_ON(!PAGE_ALIGNED(addr))
    if crypt_ops.is_null() || (addr & 0xfff) != 0 {
        return 0;
    }

    ((*crypt_ops).encrypt.expect("encrypt hook is registered"))(addr, numpages)
}

pub unsafe extern "C" fn set_memory_decrypted(addr: c_ulong, numpages: i32) -> i32 {
    // likely(!crypt_ops) || WARN_ON(!PAGE_ALIGNED(addr))
    if crypt_ops.is_null() || (addr & 0xfff) != 0 {
        return 0;
    }

    ((*crypt_ops).decrypt.expect("decrypt hook is registered"))(addr, numpages)
}

// EXPORT_SYMBOL_GPL(set_memory_encrypted);
// EXPORT_SYMBOL_GPL(set_memory_decrypted);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
