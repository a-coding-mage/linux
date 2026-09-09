/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Software async crypto daemon
 *
 * Added AEAD support to cryptd.
 *    Authors: Tadeusz Struk (tadeusz.struk@intel.com)
 *             Adrian Hoban <adrian.hoban@intel.com>
 *             Gabriele Paoloni <gabriele.paoloni@intel.com>
 *             Aidan O'Mahony (aidan.o.mahony@intel.com)
 *    Copyright (c) 2010, Intel Corporation.
 */

/* External dependency supplied by the crypto subsystem. */
pub struct crypto_aead;

#[repr(C)]
pub struct cryptd_aead {
    pub base: crypto_aead,
}

#[inline]
pub unsafe fn __cryptd_aead_cast(tfm: *mut crypto_aead) -> *mut cryptd_aead {
    tfm as *mut cryptd_aead
}

unsafe extern "C" {
    pub fn cryptd_alloc_aead(
        alg_name: *const core::ffi::c_char,
        type_: u32,
        mask: u32,
    ) -> *mut cryptd_aead;

    pub fn cryptd_aead_child(tfm: *mut cryptd_aead) -> *mut crypto_aead;

    /* Must be called without moving CPUs. */
    pub fn cryptd_aead_queued(tfm: *mut cryptd_aead) -> bool;

    pub fn cryptd_free_aead(tfm: *mut cryptd_aead);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
