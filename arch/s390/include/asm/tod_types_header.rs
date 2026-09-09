/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from <linux/types.h>; __uint128_t is represented by u128. */

#[repr(C)]
pub union tod_clock {
    pub val: u128,
    pub fields: tod_clock_fields,
    pub eitod_fields: tod_clock_eitod_fields,
    pub us_fields: tod_clock_us_fields,
}

/*
 * C bit-field view of tod_clock:
 *   ei  : 8 bits, epoch index
 *   tod : 64 bits, bits 0-63 of TOD clock
 *   _   : 40 bits
 *   pf  : 16 bits, programmable field
 *
 * Rust has no native bit-field syntax; the packed 128-bit representation is
 * retained and the masks/shifts below provide the corresponding view.
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct tod_clock_fields(pub u128);

impl tod_clock_fields {
    pub const EI_SHIFT: u32 = 0;
    pub const EI_MASK: u128 = 0xff;
    pub const TOD_SHIFT: u32 = 8;
    pub const TOD_MASK: u128 = 0xffff_ffff_ffff_ffff;
    pub const PF_SHIFT: u32 = 112;
    pub const PF_MASK: u128 = 0xffff;
}

/* C bit-field view: eitod: 72 bits, followed by 56 padding bits. */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct tod_clock_eitod_fields(pub u128);

impl tod_clock_eitod_fields {
    pub const EITOD_SHIFT: u32 = 0;
    pub const EITOD_MASK: u128 = 0xffff_ffff_ffff_ffff_ff;
}

/* C bit-field view: us: 60 bits, sus: 12 bits, followed by 56 padding bits. */
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct tod_clock_us_fields(pub u128);

impl tod_clock_us_fields {
    pub const US_SHIFT: u32 = 0;
    pub const US_MASK: u128 = 0x0fff_ffff_ffff_ffff;
    pub const SUS_SHIFT: u32 = 60;
    pub const SUS_MASK: u128 = 0xfff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
