/* SPDX-License-Identifier: GPL-2.0 */
/* Microchip switch tag common header
 *
 * Copyright (C) 2022 Microchip Technology Inc.
 */

/* Dependency supplied by the surrounding DSA implementation: net/dsa.h */

/* All time stamps from the KSZ consist of 2 bits for seconds and 30 bits for
 * nanoseconds. This is NOT the same as 32 bits for nanoseconds.
 */
pub const KSZ_TSTAMP_SEC_MASK: u32 = 0xc000_0000;
pub const KSZ_TSTAMP_NSEC_MASK: u32 = 0x3fff_ffff;

#[inline]
pub unsafe fn ksz_decode_tstamp(tstamp: u32) -> ktime_t {
    let ns: u64 = (((tstamp & KSZ_TSTAMP_SEC_MASK) >> 30) as u64)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add((tstamp & KSZ_TSTAMP_NSEC_MASK) as u64);

    ns_to_ktime(ns)
}

#[repr(C)]
pub struct ksz_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}

#[repr(C)]
pub struct ksz_tagger_data {
    pub xmit_work_fn: Option<unsafe extern "C" fn(work: *mut kthread_work)>,
    pub hwtstamp_set_state:
        Option<unsafe extern "C" fn(ds: *mut dsa_switch, on: bool)>,
}

#[repr(C)]
pub struct ksz_skb_cb {
    pub clone: *mut sk_buff,
    pub ptp_type: core::ffi::c_uint,
    pub update_correction: bool,
    pub tstamp: u32,
}

#[inline]
pub unsafe fn KSZ_SKB_CB(skb: *mut sk_buff) -> *mut ksz_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut ksz_skb_cb
}

#[inline]
pub unsafe fn ksz_tagger_data(ds: *mut dsa_switch) -> *mut ksz_tagger_data {
    (*ds).tagger_data
}

/* External types and functions supplied by the surrounding implementation. */
extern "C" {
    pub type ktime_t;
    pub type dsa_port;
    pub type sk_buff;
    pub type kthread_work;
    pub type dsa_switch;

    pub static NSEC_PER_SEC: u64;
    pub fn ns_to_ktime(ns: u64) -> ktime_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
