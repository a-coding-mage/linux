/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2000-2010 Steven J. Hill <sjhill@realitydiluted.com>
 *                          David Woodhouse <dwmw2@infradead.org>
 *                          Thomas Gleixner <tglx@kernel.org>
 *
 * This file is the header for the NAND Hamming ECC implementation.
 */

// Dependency supplied by the Linux NAND headers:
// struct nand_ecc_req_tweak_ctx, struct nand_device

#[repr(C)]
pub struct nand_ecc_sw_hamming_conf {
    pub req_ctx: nand_ecc_req_tweak_ctx,
    pub code_size: ::core::ffi::c_uint,
    pub calc_buf: *mut u8,
    pub code_buf: *mut u8,
    pub sm_order: ::core::ffi::c_uint,
}

// Equivalent build-time condition for IS_ENABLED(CONFIG_MTD_NAND_ECC_SW_HAMMING).
#[cfg(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING")]
extern "C" {
    pub fn nand_ecc_sw_hamming_init_ctx(nand: *mut nand_device) -> ::core::ffi::c_int;
    pub fn nand_ecc_sw_hamming_cleanup_ctx(nand: *mut nand_device);
    pub fn ecc_sw_hamming_calculate(
        buf: *const u8,
        step_size: ::core::ffi::c_uint,
        code: *mut u8,
        sm_order: bool,
    ) -> ::core::ffi::c_int;
    pub fn nand_ecc_sw_hamming_calculate(
        nand: *mut nand_device,
        buf: *const u8,
        code: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn ecc_sw_hamming_correct(
        buf: *mut u8,
        read_ecc: *mut u8,
        calc_ecc: *mut u8,
        step_size: ::core::ffi::c_uint,
        sm_order: bool,
    ) -> ::core::ffi::c_int;
    pub fn nand_ecc_sw_hamming_correct(
        nand: *mut nand_device,
        buf: *mut u8,
        read_ecc: *mut u8,
        calc_ecc: *mut u8,
    ) -> ::core::ffi::c_int;
}

// !CONFIG_MTD_NAND_ECC_SW_HAMMING: these inline definitions return -ENOTSUPP.
#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn nand_ecc_sw_hamming_init_ctx(_nand: *mut nand_device) -> ::core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn nand_ecc_sw_hamming_cleanup_ctx(_nand: *mut nand_device) {}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn ecc_sw_hamming_calculate(
    _buf: *const u8,
    _step_size: ::core::ffi::c_uint,
    _code: *mut u8,
    _sm_order: bool,
) -> ::core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn nand_ecc_sw_hamming_calculate(
    _nand: *mut nand_device,
    _buf: *const u8,
    _code: *mut u8,
) -> ::core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn ecc_sw_hamming_correct(
    _buf: *mut u8,
    _read_ecc: *mut u8,
    _calc_ecc: *mut u8,
    _step_size: ::core::ffi::c_uint,
    _sm_order: bool,
) -> ::core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MTD_NAND_ECC_SW_HAMMING"))]
#[inline]
pub unsafe fn nand_ecc_sw_hamming_correct(
    _nand: *mut nand_device,
    _buf: *mut u8,
    _read_ecc: *mut u8,
    _calc_ecc: *mut u8,
) -> ::core::ffi::c_int {
    -ENOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
