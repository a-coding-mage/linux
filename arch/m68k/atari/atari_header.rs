/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

/* ataints.c */
unsafe extern "C" {
    pub fn atari_init_IRQ();
}

/* atasound.c */
unsafe extern "C" {
    pub fn atari_microwire_cmd(cmd: ::core::ffi::c_int);
    pub fn atari_mksound(hz: u32, ticks: u32);
}

/* time.c */
unsafe extern "C" {
    pub fn atari_sched_init();
    pub fn atari_mste_hwclk(op: ::core::ffi::c_int, t: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn atari_tt_hwclk(op: ::core::ffi::c_int, t: *mut rtc_time) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
