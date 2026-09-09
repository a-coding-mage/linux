/* SPDX-License-Identifier: GPL-2.0-only */
/* gain-time-scale conversion helpers for IIO light sensors
 *
 * Copyright (c) 2023 Matti Vaittinen <mazziesaccount@gmail.com>
 */

/* Dependency declarations supplied by the surrounding kernel translation. */
use core::ffi::c_void;

pub type U64 = u64;
pub type CInt = i32;
pub type Bool = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_gain_sel_pair {
    pub gain: CInt,
    pub sel: CInt,
}

#[repr(C)]
pub struct iio_itime_sel_mul {
    pub time_us: CInt,
    pub sel: CInt,
    pub mul: CInt,
}

#[repr(C)]
pub struct iio_gts {
    pub max_scale: U64,
    pub hwgain_table: *const iio_gain_sel_pair,
    pub num_hwgain: CInt,
    pub itime_table: *const iio_itime_sel_mul,
    pub num_itime: CInt,
    pub per_time_avail_scale_tables: *mut *mut CInt,
    pub avail_all_scales_table: *mut CInt,
    pub num_avail_all_scales: CInt,
    pub avail_time_tables: *mut CInt,
    pub num_avail_time_tables: CInt,
}

#[macro_export]
macro_rules! GAIN_SCALE_GAIN {
    ($gain:expr, $sel:expr) => {
        $crate::iio_gain_sel_pair { gain: $gain, sel: $sel }
    };
}

#[macro_export]
macro_rules! GAIN_SCALE_ITIME_US {
    ($itime:expr, $sel:expr, $mul:expr) => {
        $crate::iio_itime_sel_mul { time_us: $itime, sel: $sel, mul: $mul }
    };
}

pub unsafe fn iio_gts_find_itime_by_time(
    gts: *mut iio_gts,
    time: CInt,
) -> *const iio_itime_sel_mul {
    if (*gts).num_itime == 0 {
        return core::ptr::null();
    }

    let mut i: CInt = 0;
    while i < (*gts).num_itime {
        let itime = (*gts).itime_table.add(i as usize);
        if (*itime).time_us == time {
            return itime;
        }
        i += 1;
    }

    core::ptr::null()
}

pub unsafe fn iio_gts_find_itime_by_sel(
    gts: *mut iio_gts,
    sel: CInt,
) -> *const iio_itime_sel_mul {
    let mut i: CInt = 0;
    while i < (*gts).num_itime {
        let itime = (*gts).itime_table.add(i as usize);
        if (*itime).sel == sel {
            return itime;
        }
        i += 1;
    }

    core::ptr::null()
}

extern "C" {
    pub fn devm_iio_init_iio_gts(
        dev: *mut device,
        max_scale_int: CInt,
        max_scale_nano: CInt,
        gain_tbl: *const iio_gain_sel_pair,
        num_gain: CInt,
        tim_tbl: *const iio_itime_sel_mul,
        num_times: CInt,
        gts: *mut iio_gts,
    ) -> CInt;

    pub fn iio_gts_find_sel_by_gain(gts: *mut iio_gts, gain: CInt) -> CInt;
    pub fn iio_find_closest_gain_low(gts: *mut iio_gts, gain: CInt, in_range: *mut Bool) -> CInt;
    pub fn iio_gts_find_gain_by_sel(gts: *mut iio_gts, sel: CInt) -> CInt;
    pub fn iio_gts_get_min_gain(gts: *mut iio_gts) -> CInt;
    pub fn iio_gts_total_gain_to_scale(gts: *mut iio_gts, total_gain: CInt, scale_int: *mut CInt, scale_nano: *mut CInt) -> CInt;
    pub fn iio_gts_find_gain_sel_for_scale_using_time(gts: *mut iio_gts, time_sel: CInt, scale_int: CInt, scale_nano: CInt, gain_sel: *mut CInt) -> CInt;
    pub fn iio_gts_find_gain_time_sel_for_scale(gts: *mut iio_gts, scale_int: CInt, scale_nano: CInt, gain_sel: *mut CInt, time_sel: *mut CInt) -> CInt;
    pub fn iio_gts_get_scale(gts: *mut iio_gts, gain: CInt, time: CInt, scale_int: *mut CInt, scale_nano: *mut CInt) -> CInt;
    pub fn iio_gts_find_new_gain_sel_by_old_gain_time(gts: *mut iio_gts, old_gain: CInt, old_time_sel: CInt, new_time_sel: CInt, new_gain: *mut CInt) -> CInt;
    pub fn iio_gts_find_new_gain_by_old_gain_time(gts: *mut iio_gts, old_gain: CInt, old_time: CInt, new_time: CInt, new_gain: *mut CInt) -> CInt;
    pub fn iio_gts_find_new_gain_by_gain_time_min(gts: *mut iio_gts, old_gain: CInt, old_time: CInt, new_time: CInt, new_gain: *mut CInt, in_range: *mut Bool) -> CInt;
    pub fn iio_gts_avail_times(gts: *mut iio_gts, vals: *mut *const CInt, ty: *mut CInt, length: *mut CInt) -> CInt;
    pub fn iio_gts_all_avail_scales(gts: *mut iio_gts, vals: *mut *const CInt, ty: *mut CInt, length: *mut CInt) -> CInt;
    pub fn iio_gts_avail_scales_for_time(gts: *mut iio_gts, time: CInt, vals: *mut *const CInt, ty: *mut CInt, length: *mut CInt) -> CInt;
    pub fn iio_gts_get_total_gain(gts: *mut iio_gts, gain: CInt, time: CInt) -> CInt;
}

const EINVAL: CInt = -22;

pub unsafe fn iio_gts_find_int_time_by_sel(gts: *mut iio_gts, sel: CInt) -> CInt {
    let itime = iio_gts_find_itime_by_sel(gts, sel);
    if itime.is_null() { return EINVAL; }
    (*itime).time_us
}

pub unsafe fn iio_gts_find_sel_by_int_time(gts: *mut iio_gts, time: CInt) -> CInt {
    let itime = iio_gts_find_itime_by_time(gts, time);
    if itime.is_null() { return EINVAL; }
    (*itime).sel
}

pub unsafe fn iio_gts_valid_time(gts: *mut iio_gts, time_us: CInt) -> Bool {
    !iio_gts_find_itime_by_time(gts, time_us).is_null()
}

pub unsafe fn iio_gts_valid_gain(gts: *mut iio_gts, gain: CInt) -> Bool {
    iio_gts_find_sel_by_gain(gts, gain) >= 0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
