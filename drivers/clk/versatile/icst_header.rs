/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
 *
 *  Support functions for calculating clocks/divisors for the ICST
 *  clock generators.  See https://www.idt.com/ for more information
 *  on these devices.
 */

#[repr(C)]
pub struct icst_params {
    pub ref_: libc::c_ulong,
    pub vco_max: libc::c_ulong, /* inclusive */
    pub vco_min: libc::c_ulong, /* exclusive */
    pub vd_min: libc::c_ushort, /* inclusive */
    pub vd_max: libc::c_ushort, /* inclusive */
    pub rd_min: libc::c_uchar, /* inclusive */
    pub rd_max: libc::c_uchar, /* inclusive */
    pub s2div: *const libc::c_uchar, /* chip specific s2div array */
    pub idx2s: *const libc::c_uchar, /* chip specific idx2s array */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icst_vco {
    pub v: libc::c_ushort,
    pub r: libc::c_uchar,
    pub s: libc::c_uchar,
}

unsafe extern "C" {
    pub fn icst_hz(p: *const icst_params, vco: icst_vco) -> libc::c_ulong;
    pub fn icst_hz_to_vco(p: *const icst_params, freq: libc::c_ulong) -> icst_vco;
}

/*
 * ICST307 VCO frequency must be between 6MHz and 200MHz (3.3 or 5V).
 * This frequency is pre-output divider.
 */
pub const ICST307_VCO_MIN: libc::c_ulong = 6000000;
pub const ICST307_VCO_MAX: libc::c_ulong = 200000000;

unsafe extern "C" {
    pub static icst307_s2div: [libc::c_uchar; 0];
    pub static icst307_idx2s: [libc::c_uchar; 0];
}

/*
 * ICST525 VCO frequency must be between 10MHz and 200MHz (3V) or 320MHz (5V).
 * This frequency is pre-output divider.
 */
pub const ICST525_VCO_MIN: libc::c_ulong = 10000000;
pub const ICST525_VCO_MAX_3V: libc::c_ulong = 200000000;
pub const ICST525_VCO_MAX_5V: libc::c_ulong = 320000000;

unsafe extern "C" {
    pub static icst525_s2div: [libc::c_uchar; 0];
    pub static icst525_idx2s: [libc::c_uchar; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
