/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright © 2011 Ivan Djelic <ivan.djelic@parrot.com>
 *
 * This file is the header for the NAND BCH ECC implementation.
 */

// C header guard: __MTD_NAND_ECC_SW_BCH_H__

// C dependencies:
// #include <linux/mtd/nand.h>
// #include <linux/bch.h>

/// Private software BCH ECC engine structure.
#[repr(C)]
pub struct nand_ecc_sw_bch_conf {
    /// Save request context and tweak the original request to fit the engine needs.
    pub req_ctx: nand_ecc_req_tweak_ctx,
    /// Number of bytes needed to store a code (one code per step).
    pub code_size: core::ffi::c_uint,
    /// Buffer to use when calculating ECC bytes.
    pub calc_buf: *mut u8,
    /// Buffer to use when reading (raw) ECC bytes from the chip.
    pub code_buf: *mut u8,
    /// BCH control structure.
    pub bch: *mut bch_control,
    /// Error location array.
    pub errloc: *mut core::ffi::c_uint,
    /// XOR ECC mask, allows erased pages to be decoded as valid.
    pub eccmask: *mut core::ffi::c_uchar,
}

// CONFIG_MTD_NAND_ECC_SW_BCH is a build-time configuration condition from C.
#[cfg(feature = "CONFIG_MTD_NAND_ECC_SW_BCH")]
unsafe extern "C" {
    pub fn nand_ecc_sw_bch_calculate(
        nand: *mut nand_device,
        buf: *const core::ffi::c_uchar,
        code: *mut core::ffi::c_uchar,
    ) -> core::ffi::c_int;
    pub fn nand_ecc_sw_bch_correct(
        nand: *mut nand_device,
        buf: *mut core::ffi::c_uchar,
        read_ecc: *mut core::ffi::c_uchar,
        calc_ecc: *mut core::ffi::c_uchar,
    ) -> core::ffi::c_int;
    pub fn nand_ecc_sw_bch_init_ctx(nand: *mut nand_device) -> core::ffi::c_int;
    pub fn nand_ecc_sw_bch_cleanup_ctx(nand: *mut nand_device);
    pub fn nand_ecc_sw_bch_get_engine() -> *mut nand_ecc_engine;
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_BCH"))]
pub unsafe fn nand_ecc_sw_bch_calculate(
    _nand: *mut nand_device,
    _buf: *const core::ffi::c_uchar,
    _code: *mut core::ffi::c_uchar,
) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_BCH"))]
pub unsafe fn nand_ecc_sw_bch_correct(
    _nand: *mut nand_device,
    _buf: *mut core::ffi::c_uchar,
    _read_ecc: *mut core::ffi::c_uchar,
    _calc_ecc: *mut core::ffi::c_uchar,
) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_BCH"))]
pub unsafe fn nand_ecc_sw_bch_init_ctx(_nand: *mut nand_device) -> core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_BCH"))]
pub unsafe fn nand_ecc_sw_bch_cleanup_ctx(_nand: *mut nand_device) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
