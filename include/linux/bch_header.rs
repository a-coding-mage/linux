/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic binary BCH encoding/decoding library
 *
 * Copyright © 2011 Parrot S.A.
 *
 * Author: Ivan Djelic <ivan.djelic@parrot.com>
 *
 * Description:
 *
 * This library provides runtime configurable encoding/decoding of binary
 * Bose-Chaudhuri-Hocquenghem (BCH) codes.
 */

// Dependency supplied by the surrounding translation unit.
#[repr(C)]
pub struct gf_poly {
    _private: [u8; 0],
}

/**
 * struct bch_control - BCH control structure
 * @m:          Galois field order
 * @n:          maximum codeword size in bits (= 2^m-1)
 * @t:          error correction capability in bits
 * @ecc_bits:   ecc exact size in bits, i.e. generator polynomial degree (<=m*t)
 * @ecc_bytes:  ecc max size (m*t bits) in bytes
 * @a_pow_tab:  Galois field GF(2^m) exponentiation lookup table
 * @a_log_tab:  Galois field GF(2^m) log lookup table
 * @mod8_tab:   remainder generator polynomial lookup tables
 * @ecc_buf:    ecc parity words buffer
 * @ecc_buf2:   ecc parity words buffer
 * @xi_tab:     GF(2^m) base for solving degree 2 polynomial roots
 * @syn:        syndrome buffer
 * @cache:      log-based polynomial representation buffer
 * @elp:        error locator polynomial
 * @poly_2t:    temporary polynomials of degree 2t
 * @swap_bits:  swap bits within data and syndrome bytes
 */
#[repr(C)]
pub struct bch_control {
    pub m: ::core::ffi::c_uint,
    pub n: ::core::ffi::c_uint,
    pub t: ::core::ffi::c_uint,
    pub ecc_bits: ::core::ffi::c_uint,
    pub ecc_bytes: ::core::ffi::c_uint,
    pub a_pow_tab: *mut u16,
    pub a_log_tab: *mut u16,
    pub mod8_tab: *mut u32,
    pub ecc_buf: *mut u32,
    pub ecc_buf2: *mut u32,
    pub xi_tab: *mut ::core::ffi::c_uint,
    pub syn: *mut ::core::ffi::c_uint,
    pub cache: *mut ::core::ffi::c_int,
    pub elp: *mut gf_poly,
    pub poly_2t: [*mut gf_poly; 4],
    pub swap_bits: bool,
}

extern "C" {
    pub fn bch_init(
        m: ::core::ffi::c_int,
        t: ::core::ffi::c_int,
        prim_poly: ::core::ffi::c_uint,
        swap_bits: bool,
    ) -> *mut bch_control;

    pub fn bch_free(bch: *mut bch_control);

    pub fn bch_encode(
        bch: *mut bch_control,
        data: *const u8,
        len: ::core::ffi::c_uint,
        ecc: *mut u8,
    );

    pub fn bch_decode(
        bch: *mut bch_control,
        data: *const u8,
        len: ::core::ffi::c_uint,
        recv_ecc: *const u8,
        calc_ecc: *const u8,
        syn: *const ::core::ffi::c_uint,
        errloc: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
