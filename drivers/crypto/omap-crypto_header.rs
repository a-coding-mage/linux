/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP Crypto driver common support routines.
 *
 * Copyright (c) 2017 Texas Instruments Incorporated
 *   Tero Kristo <t-kristo@ti.com>
 */

// C header dependency: struct scatterlist is supplied by the surrounding
// translation unit or its dependencies.
#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(i32)]
pub enum OmapCryptoError {
    OMAP_CRYPTO_NOT_ALIGNED = 1,
    OMAP_CRYPTO_BAD_DATA_LENGTH,
}

pub const OMAP_CRYPTO_DATA_COPIED: u32 = 1u32 << 0;
pub const OMAP_CRYPTO_SG_COPIED: u32 = 1u32 << 1;

pub const OMAP_CRYPTO_COPY_MASK: u32 = 0x3;

pub const OMAP_CRYPTO_COPY_DATA: u32 = 1u32 << 0;
pub const OMAP_CRYPTO_FORCE_COPY: u32 = 1u32 << 1;
pub const OMAP_CRYPTO_ZERO_BUF: u32 = 1u32 << 2;
pub const OMAP_CRYPTO_FORCE_SINGLE_ENTRY: u32 = 1u32 << 3;

extern "C" {
    pub fn omap_crypto_align_sg(
        sg: *mut *mut scatterlist,
        total: i32,
        bs: i32,
        new_sg: *mut scatterlist,
        flags: u16,
        flags_shift: u8,
        dd_flags: *mut core::ffi::c_ulong,
    ) -> i32;

    pub fn omap_crypto_cleanup(
        sg: *mut scatterlist,
        orig: *mut scatterlist,
        offset: i32,
        len: i32,
        flags_shift: u8,
        flags: core::ffi::c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
