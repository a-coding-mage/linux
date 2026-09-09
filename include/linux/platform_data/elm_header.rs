/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * BCH Error Location Module
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com/
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bch_ecc {
    BCH4_ECC = 0,
    BCH8_ECC,
    BCH16_ECC,
}

/* ELM support 8 error syndrome process */
pub const ERROR_VECTOR_MAX: i32 = 8;

/**
 * struct elm_errorvec - error vector for elm
 * @error_reported:       set true for vectors error is reported
 * @error_uncorrectable:  number of uncorrectable errors
 * @error_count:          number of correctable errors in the sector
 * @error_loc:            buffer for error location
 *
 */
#[repr(C)]
pub struct elm_errorvec {
    pub error_reported: bool,
    pub error_uncorrectable: bool,
    pub error_count: i32,
    pub error_loc: [i32; 16],
}

/* CONFIG_MTD_NAND_OMAP_BCH is a build-time configuration condition. */
#[cfg(feature = "CONFIG_MTD_NAND_OMAP_BCH")]
extern "C" {
    pub fn elm_decode_bch_error_page(
        dev: *mut device,
        ecc_calc: *mut u8,
        err_vec: *mut elm_errorvec,
    );
    pub fn elm_config(
        dev: *mut device,
        bch_type: bch_ecc,
        ecc_steps: i32,
        ecc_step_size: i32,
        ecc_syndrome_size: i32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_MTD_NAND_OMAP_BCH"))]
pub unsafe fn elm_decode_bch_error_page(
    _dev: *mut device,
    _ecc_calc: *mut u8,
    _err_vec: *mut elm_errorvec,
) {
}

#[cfg(not(feature = "CONFIG_MTD_NAND_OMAP_BCH"))]
pub unsafe fn elm_config(
    _dev: *mut device,
    _bch_type: bch_ecc,
    _ecc_steps: i32,
    _ecc_step_size: i32,
    _ecc_syndrome_size: i32,
) -> i32 {
    -ENOSYS
}

/* External dependency supplied by the including kernel environment. */
pub enum device {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
