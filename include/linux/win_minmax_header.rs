/* SPDX-License-Identifier: GPL-2.0 */
/*
 * win_minmax.h: windowed min/max tracker by Kathleen Nichols.
 *
 */

// Dependency intent: u32 corresponds to Linux's __u32/u32 type.

/* A single data point for our parameterized min-max tracker */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minmax_sample {
    pub t: u32, /* time measurement was taken */
    pub v: u32, /* value measured */
}

/* State for the parameterized min-max tracker */
#[repr(C)]
pub struct minmax {
    pub s: [minmax_sample; 3],
}

pub unsafe fn minmax_get(m: *const minmax) -> u32 {
    (*m).s[0].v
}

pub unsafe fn minmax_reset(m: *mut minmax, t: u32, meas: u32) -> u32 {
    let val = minmax_sample { t, v: meas };

    (*m).s[2] = (*m).s[1];
    (*m).s[1] = (*m).s[0];
    (*m).s[0] = val;
    (*m).s[0].v
}

extern "C" {
    pub fn minmax_running_max(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
    pub fn minmax_running_min(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
