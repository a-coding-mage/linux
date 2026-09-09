/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding architecture headers:
// asm/hypervisor.h and asm/rsi.h

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arm64_mem_crypt_ops {
    pub encrypt: Option<unsafe extern "C" fn(addr: c_ulong, numpages: c_int) -> c_int>,
    pub decrypt: Option<unsafe extern "C" fn(addr: c_ulong, numpages: c_int) -> c_int>,
}

unsafe extern "C" {
    pub fn arm64_mem_crypt_ops_register(ops: *const arm64_mem_crypt_ops) -> c_int;

    pub fn set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int;
    pub fn set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int;

    pub fn realm_register_memory_enc_ops() -> c_int;

    fn is_realm_world() -> bool;
    fn is_protected_kvm_guest() -> bool;
}

pub unsafe fn force_dma_unencrypted(_dev: *mut device) -> bool {
    unsafe { is_realm_world() || is_protected_kvm_guest() }
}

/*
 * For Arm CCA guests, canonical addresses are "encrypted", so no changes
 * required for dma_addr_encrypted().
 * The unencrypted DMA buffers must be accessed via the unprotected IPA,
 * "top IPA bit" set.
 */
#[inline]
pub const fn dma_addr_unencrypted(x: c_ulong) -> c_ulong {
    (x) | PROT_NS_SHARED
}

/* Clear the "top" IPA bit while converting back */
#[inline]
pub const fn dma_addr_canonical(x: c_ulong) -> c_ulong {
    (x) & !PROT_NS_SHARED
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
