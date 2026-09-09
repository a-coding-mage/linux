/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ppp-comp.h - Definitions for doing PPP packet compression.
 *
 * Copyright 1994-1998 Paul Mackerras.
 */

// Dependency intent: declarations from <uapi/linux/ppp-comp.h> are supplied
// by the surrounding translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct compstat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

/*
 * The following symbols control whether we include code for
 * various compression methods.
 */

// By default, include BSD-Compress.
pub const DO_BSD_COMPRESS: i32 = 1;
// By default, include Deflate.
pub const DO_DEFLATE: i32 = 1;
pub const DO_PREDICTOR_1: i32 = 0;
pub const DO_PREDICTOR_2: i32 = 0;

/*
 * Structure giving methods for compression/decompression.
 */

#[repr(C)]
pub struct compressor {
    pub compress_proto: i32, /* CCP compression protocol number */

    /* Allocate space for a compressor (transmit side) */
    pub comp_alloc: Option<unsafe extern "C" fn(options: *mut u8, opt_len: i32) -> *mut c_void>,

    /* Free space used by a compressor */
    pub comp_free: Option<unsafe extern "C" fn(state: *mut c_void)>,

    /* Initialize a compressor */
    pub comp_init: Option<unsafe extern "C" fn(
        state: *mut c_void,
        options: *mut u8,
        opt_len: i32,
        unit: i32,
        opthdr: i32,
        debug: i32,
    ) -> i32>,

    /* Reset a compressor */
    pub comp_reset: Option<unsafe extern "C" fn(state: *mut c_void)>,

    /* Compress a packet */
    pub compress: Option<unsafe extern "C" fn(
        state: *mut c_void,
        rptr: *mut u8,
        obuf: *mut u8,
        isize: i32,
        osize: i32,
    ) -> i32>,

    /* Return compression statistics */
    pub comp_stat: Option<unsafe extern "C" fn(state: *mut c_void, stats: *mut compstat)>,

    /* Allocate space for a decompressor (receive side) */
    pub decomp_alloc: Option<unsafe extern "C" fn(options: *mut u8, opt_len: i32) -> *mut c_void>,

    /* Free space used by a decompressor */
    pub decomp_free: Option<unsafe extern "C" fn(state: *mut c_void)>,

    /* Initialize a decompressor */
    pub decomp_init: Option<unsafe extern "C" fn(
        state: *mut c_void,
        options: *mut u8,
        opt_len: i32,
        unit: i32,
        opthdr: i32,
        mru: i32,
        debug: i32,
    ) -> i32>,

    /* Reset a decompressor */
    pub decomp_reset: Option<unsafe extern "C" fn(state: *mut c_void)>,

    /* Decompress a packet. */
    pub decompress: Option<unsafe extern "C" fn(
        state: *mut c_void,
        ibuf: *mut u8,
        isize: i32,
        obuf: *mut u8,
        osize: i32,
    ) -> i32>,

    /* Update state for an incompressible packet received */
    pub incomp: Option<unsafe extern "C" fn(state: *mut c_void, ibuf: *mut u8, icnt: i32)>,

    /* Return decompression statistics */
    pub decomp_stat: Option<unsafe extern "C" fn(state: *mut c_void, stats: *mut compstat)>,

    /* Used in locking compressor modules */
    pub owner: *mut module,
    /* Extra skb space needed by the compressor algorithm */
    pub comp_extra: u32,
}

/*
 * The return value from decompress routine is the length of the
 * decompressed packet if successful, otherwise DECOMP_ERROR
 * or DECOMP_FATALERROR if an error occurred.
 *
 * We need to make this distinction so that we can disable certain
 * useful functionality, namely sending a CCP reset-request as a result
 * of an error detected after decompression.  This is to avoid infringing
 * a patent held by Motorola.
 * Don't you just lurve software patents.
 */

pub const DECOMP_ERROR: i32 = -1; /* error detected before decomp. */
pub const DECOMP_FATALERROR: i32 = -2; /* error detected after decomp. */

extern "C" {
    pub fn ppp_register_compressor(compressor: *mut compressor) -> i32;
    pub fn ppp_unregister_compressor(compressor: *mut compressor);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
