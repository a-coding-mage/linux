/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ppp-comp.h - Definitions for doing PPP packet compression.
 *
 * Copyright 1994-1998 Paul Mackerras.
 *
 *  This program is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU General Public License
 *  version 2 as published by the Free Software Foundation.
 */

/* CCP codes. */
pub const CCP_CONFREQ: u8 = 1;
pub const CCP_CONFACK: u8 = 2;
pub const CCP_TERMREQ: u8 = 5;
pub const CCP_TERMACK: u8 = 6;
pub const CCP_RESETREQ: u8 = 14;
pub const CCP_RESETACK: u8 = 15;

/* Max # bytes for a CCP option */
pub const CCP_MAX_OPTION_LENGTH: usize = 32;

/* Parts of a CCP packet. */
#[inline]
pub unsafe fn CCP_CODE(dp: *const u8) -> u8 {
    *dp
}

#[inline]
pub unsafe fn CCP_ID(dp: *const u8) -> u8 {
    *dp.add(1)
}

#[inline]
pub unsafe fn CCP_LENGTH(dp: *const u8) -> u16 {
    ((*dp.add(2) as u16) << 8) + *dp.add(3) as u16
}

pub const CCP_HDRLEN: usize = 4;

#[inline]
pub unsafe fn CCP_OPT_CODE(dp: *const u8) -> u8 {
    *dp
}

#[inline]
pub unsafe fn CCP_OPT_LENGTH(dp: *const u8) -> u8 {
    *dp.add(1)
}

pub const CCP_OPT_MINLEN: usize = 2;

/* Definitions for BSD-Compress. */
pub const CI_BSD_COMPRESS: u8 = 21; /* config. option for BSD-Compress */
pub const CILEN_BSD_COMPRESS: u8 = 3; /* length of config. option */

/* Macros for handling the 3rd byte of the BSD-Compress config option. */
#[inline]
pub const fn BSD_NBITS(x: u8) -> u8 { x & 0x1F } /* number of bits requested */
#[inline]
pub const fn BSD_VERSION(x: u8) -> u8 { x >> 5 } /* version of option format */
pub const BSD_CURRENT_VERSION: u8 = 1; /* current version number */
#[inline]
pub const fn BSD_MAKE_OPT(v: u8, n: u8) -> u8 { (v << 5) | n }

pub const BSD_MIN_BITS: u8 = 9; /* smallest code size supported */
pub const BSD_MAX_BITS: u8 = 15; /* largest code size supported */

/* Definitions for Deflate. */
pub const CI_DEFLATE: u8 = 26; /* config option for Deflate */
pub const CI_DEFLATE_DRAFT: u8 = 24; /* value used in original draft RFC */
pub const CILEN_DEFLATE: u8 = 4; /* length of its config option */

pub const DEFLATE_MIN_SIZE: u8 = 9;
pub const DEFLATE_MAX_SIZE: u8 = 15;
pub const DEFLATE_METHOD_VAL: u8 = 8;
#[inline]
pub const fn DEFLATE_SIZE(x: u8) -> u8 { (x >> 4) + 8 }
#[inline]
pub const fn DEFLATE_METHOD(x: u8) -> u8 { x & 0x0F }
#[inline]
pub const fn DEFLATE_MAKE_OPT(w: u8) -> u8 { ((w - 8) << 4) + DEFLATE_METHOD_VAL }
pub const DEFLATE_CHK_SEQUENCE: u8 = 0;

/* Definitions for MPPE. */
pub const CI_MPPE: u8 = 18; /* config option for MPPE */
pub const CILEN_MPPE: u8 = 6; /* length of config option */

/* Definitions for other, as yet unsupported, compression methods. */
pub const CI_PREDICTOR_1: u8 = 1; /* config option for Predictor-1 */
pub const CILEN_PREDICTOR_1: u8 = 2; /* length of its config option */
pub const CI_PREDICTOR_2: u8 = 2; /* config option for Predictor-2 */
pub const CILEN_PREDICTOR_2: u8 = 2; /* length of its config option */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
