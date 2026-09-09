/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

/* baboon.c */
extern "C" {
    pub fn baboon_init();
}

/* iop.c */
extern "C" {
    pub fn iop_init();
}

/* misc.c */
extern "C" {
    pub fn mac_hwclk(op: ::core::ffi::c_int, t: *mut rtc_time) -> ::core::ffi::c_int;
}

/* macboing.c */
extern "C" {
    pub fn mac_mksound(freq: u32, length: u32);
}

/* oss.c */
extern "C" {
    pub fn oss_init();
}

/* psc.c */
extern "C" {
    pub fn psc_init();
}

/* via.c */
extern "C" {
    pub fn via_init();
    pub fn via_init_clock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
