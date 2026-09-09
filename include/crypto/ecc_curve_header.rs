/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021 HiSilicon */

/**
 * struct ecc_point - elliptic curve point in affine coordinates
 *
 * @x:		X coordinate in vli form.
 * @y:		Y coordinate in vli form.
 * @ndigits:	Length of vlis in u64 qwords.
 */
#[repr(C)]
pub struct ecc_point {
    pub x: *mut u64,
    pub y: *mut u64,
    pub ndigits: u8,
}

/**
 * struct ecc_curve - definition of elliptic curve
 *
 * @name:	Short name of the curve.
 * @nbits:	The number of bits of a curve.
 * @g:		Generator point of the curve.
 * @p:		Prime number, if Barrett's reduction is used for this curve
 *		pre-calculated value 'mu' is appended to the @p after ndigits.
 *		Use of Barrett's reduction is heuristically determined in
 *		vli_mmod_fast().
 * @n:		Order of the curve group.
 * @a:		Curve parameter a.
 * @b:		Curve parameter b.
 */
#[repr(C)]
pub struct ecc_curve {
    pub name: *mut core::ffi::c_char,
    pub nbits: u32,
    pub g: ecc_point,
    pub p: *mut u64,
    pub n: *mut u64,
    pub a: *mut u64,
    pub b: *mut u64,
}

/**
 * ecc_get_curve() - get elliptic curve;
 * @curve_id:           Curves IDs:
 *                      defined in 'include/crypto/ecdh.h';
 *
 * Returns curve if get curve succssful, NULL otherwise
 */
unsafe extern "C" {
    pub fn ecc_get_curve(curve_id: core::ffi::c_uint) -> *const ecc_curve;
}

/**
 * ecc_get_curve25519() - get curve25519 curve;
 *
 * Returns curve25519
 */
unsafe extern "C" {
    pub fn ecc_get_curve25519() -> *const ecc_curve;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
