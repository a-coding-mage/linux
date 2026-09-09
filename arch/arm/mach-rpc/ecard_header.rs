/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  ecard.h
 *
 *  Copyright 2007 Russell King
 */

/* Definitions internal to ecard.c - for its use only.
 *
 * External expansion card header as read from the card
 */
#[repr(C)]
pub struct ex_ecid {
    // C bit-fields; each member occupies the corresponding bit in the byte.
    pub r_irq: u8,
    pub r_zero: u8,
    pub r_fiq: u8,
    pub r_id: u8,
    pub r_a: u8,

    pub r_cd: u8,
    pub r_is: u8,
    pub r_w: u8,
    pub r_r1: u8,

    pub r_r2: u8,

    pub r_prod: [u8; 2],

    pub r_manu: [u8; 2],

    pub r_country: u8,

    pub r_fiqmask: u8,
    pub r_fiqoff: [u8; 3],

    pub r_irqmask: u8,
    pub r_irqoff: [u8; 3],
}

/* Chunk directory entry as read from the card */
#[repr(C)]
pub struct ex_chunk_dir {
    pub r_id: u8,
    pub r_len: [u8; 3],
    pub r_start: core::ffi::c_ulong,
    pub d: ex_chunk_dir_d,
}

#[repr(C)]
pub union ex_chunk_dir_d {
    pub string: [core::ffi::c_char; 256],
    pub data: [core::ffi::c_char; 1],
}

#[inline]
pub unsafe fn c_id(x: *const ex_chunk_dir) -> u8 {
    unsafe { (*x).r_id }
}

#[inline]
pub unsafe fn c_len(x: *const ex_chunk_dir) -> u32 {
    unsafe {
        (*x).r_len[0] as u32
            | ((*x).r_len[1] as u32) << 8
            | ((*x).r_len[2] as u32) << 16
    }
}

#[inline]
pub unsafe fn c_start(x: *const ex_chunk_dir) -> core::ffi::c_ulong {
    unsafe { (*x).r_start }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum card_type_t {
    ECARD_IOC = 0,
    ECARD_MEMC = 1,
    ECARD_EASI = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum card_speed_t {
    ECARD_SLOW = 0,
    ECARD_MEDIUM = 1,
    ECARD_FAST = 2,
    ECARD_SYNC = 3,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
