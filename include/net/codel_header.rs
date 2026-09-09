/*
 * Codel - The Controlled-Delay Active Queue Management algorithm
 *
 *  Copyright (C) 2011-2012 Kathleen Nichols <nichols@pollere.com>
 *  Copyright (C) 2011-2012 Van Jacobson <van@pollere.net>
 *  Copyright (C) 2012 Michael D. Taht <dave.taht@bufferbloat.net>
 *  Copyright (C) 2012,2015 Eric Dumazet <edumazet@google.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met.
 *
 * Alternatively, provided that this notice is retained in full, this
 * software may be distributed under the terms of the GNU General
 * Public License ("GPL") version 2.
 */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

pub type CodelTimeT = u32;
pub type CodelTdiffT = i32;

pub const CODEL_SHIFT: u32 = 10;

// NSEC_PER_MSEC is supplied by the kernel environment.
#[inline]
pub const fn ms2time(a: u64) -> u64 {
    (a * NSEC_PER_MSEC) >> CODEL_SHIFT
}

extern "C" {
    pub fn ktime_get_ns() -> u64;
}

#[inline]
pub unsafe fn codel_get_time() -> CodelTimeT {
    (ktime_get_ns() >> CODEL_SHIFT) as CodelTimeT
}

#[inline]
pub const fn codel_time_after(a: CodelTimeT, b: CodelTimeT) -> bool {
    (a.wrapping_sub(b) as i32) > 0
}

#[inline]
pub const fn codel_time_before(a: CodelTimeT, b: CodelTimeT) -> bool {
    codel_time_after(b, a)
}

#[inline]
pub const fn codel_time_after_eq(a: CodelTimeT, b: CodelTimeT) -> bool {
    (a.wrapping_sub(b) as i32) >= 0
}

#[inline]
pub const fn codel_time_before_eq(a: CodelTimeT, b: CodelTimeT) -> bool {
    codel_time_after_eq(b, a)
}

// NSEC_PER_USEC is supplied by the kernel environment.
#[inline]
pub fn codel_time_to_us(val: CodelTimeT) -> u32 {
    (((val as u64) << CODEL_SHIFT) / NSEC_PER_USEC) as u32
}

#[repr(C)]
pub struct CodelParams {
    pub target: CodelTimeT,
    pub ce_threshold: CodelTimeT,
    pub interval: CodelTimeT,
    pub mtu: u32,
    pub ecn: bool,
    pub ce_threshold_selector: u8,
    pub ce_threshold_mask: u8,
}

#[repr(C)]
pub struct CodelVars {
    pub count: u32,
    pub lastcount: u32,
    pub dropping: bool,
    pub rec_inv_sqrt: u16,
    pub first_above_time: CodelTimeT,
    pub drop_next: CodelTimeT,
    pub ldelay: CodelTimeT,
}

pub const REC_INV_SQRT_BITS: u32 = 8 * core::mem::size_of::<u16>() as u32;
pub const REC_INV_SQRT_SHIFT: u32 = 32 - REC_INV_SQRT_BITS;

#[repr(C)]
pub struct CodelStats {
    pub maxpacket: u32,
    pub drop_count: u32,
    pub drop_len: u32,
    pub ecn_mark: u32,
    pub ce_mark: u32,
}

pub const CODEL_DISABLED_THRESHOLD: i32 = i32::MAX;

#[repr(C)]
pub struct SkBuff {
    _private: [u8; 0],
}

pub type CodelSkbLenT = unsafe extern "C" fn(skb: *const SkBuff) -> u32;
pub type CodelSkbTimeT = unsafe extern "C" fn(skb: *const SkBuff) -> CodelTimeT;
pub type CodelSkbDropT = unsafe extern "C" fn(skb: *mut SkBuff, ctx: *mut c_void);
pub type CodelSkbDequeueT = unsafe extern "C" fn(
    vars: *mut CodelVars,
    ctx: *mut c_void,
) -> *mut SkBuff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
