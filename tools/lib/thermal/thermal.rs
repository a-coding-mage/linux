// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>
// C dependencies: <stdio.h>, <limits.h>, <thermal.h>, "thermal_nl.h"

use core::ffi::{c_char, c_int, c_void};

pub const INT_MAX: c_int = c_int::MAX;

#[repr(C)]
pub struct thermal_threshold {
    pub temperature: c_int,
}

#[repr(C)]
pub struct thermal_cdev {
    pub id: c_int,
}

#[repr(C)]
pub struct thermal_trip {
    pub id: c_int,
}

#[repr(C)]
pub struct thermal_zone {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct thermal_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thermal_handler {
    pub ops: *mut thermal_ops,
}

pub type cb_th_t = Option<unsafe extern "C" fn(*mut thermal_threshold, *mut c_void) -> c_int>;
pub type cb_tc_t = Option<unsafe extern "C" fn(*mut thermal_cdev, *mut c_void) -> c_int>;
pub type cb_tt_t = Option<unsafe extern "C" fn(*mut thermal_trip, *mut c_void) -> c_int>;
pub type cb_tz_t = Option<unsafe extern "C" fn(*mut thermal_zone, *mut c_void) -> c_int>;

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn thermal_cmd_get_trip(th: *mut c_void, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_threshold_get(th: *mut c_void, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_get_governor(th: *mut c_void, tz: *mut thermal_zone) -> c_int;
    fn thermal_cmd_get_tz(th: *mut thermal_handler, tz: *mut *mut thermal_zone) -> c_int;
    fn thermal_cmd_exit(th: *mut thermal_handler);
    fn thermal_events_exit(th: *mut thermal_handler);
    fn thermal_sampling_exit(th: *mut thermal_handler);
    fn thermal_events_init(th: *mut thermal_handler) -> c_int;
    fn thermal_sampling_init(th: *mut thermal_handler) -> c_int;
    fn thermal_cmd_init(th: *mut thermal_handler) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn for_each_thermal_threshold(
    th: *mut thermal_threshold,
    cb: cb_th_t,
    arg: *mut c_void,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;

    if th.is_null() {
        return 0;
    }

    i = 0;
    while unsafe { (*th.offset(i as isize)).temperature } != INT_MAX {
        ret |= unsafe { cb.unwrap()(th.offset(i as isize), arg) };
        i += 1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn for_each_thermal_cdev(
    cdev: *mut thermal_cdev,
    cb: cb_tc_t,
    arg: *mut c_void,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;

    if cdev.is_null() {
        return 0;
    }

    i = 0;
    while unsafe { (*cdev.offset(i as isize)).id } != -1 {
        ret |= unsafe { cb.unwrap()(cdev.offset(i as isize), arg) };
        i += 1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn for_each_thermal_trip(
    tt: *mut thermal_trip,
    cb: cb_tt_t,
    arg: *mut c_void,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;

    if tt.is_null() {
        return 0;
    }

    i = 0;
    while unsafe { (*tt.offset(i as isize)).id } != -1 {
        ret |= unsafe { cb.unwrap()(tt.offset(i as isize), arg) };
        i += 1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn for_each_thermal_zone(
    tz: *mut thermal_zone,
    cb: cb_tz_t,
    arg: *mut c_void,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;

    if tz.is_null() {
        return 0;
    }

    i = 0;
    while unsafe { (*tz.offset(i as isize)).id } != -1 {
        ret |= unsafe { cb.unwrap()(tz.offset(i as isize), arg) };
        i += 1;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thermal_zone_find_by_name(
    tz: *mut thermal_zone,
    name: *const c_char,
) -> *mut thermal_zone {
    let mut i: c_int;

    if tz.is_null() || name.is_null() {
        return core::ptr::null_mut();
    }

    i = 0;
    while unsafe { (*tz.offset(i as isize)).id } != -1 {
        if unsafe { strcmp((*tz.offset(i as isize)).name, name) } == 0 {
            return unsafe { tz.offset(i as isize) };
        }
        i += 1;
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thermal_zone_find_by_id(
    tz: *mut thermal_zone,
    id: c_int,
) -> *mut thermal_zone {
    let mut i: c_int;

    if tz.is_null() || id < 0 {
        return core::ptr::null_mut();
    }

    i = 0;
    while unsafe { (*tz.offset(i as isize)).id } != -1 {
        if unsafe { (*tz.offset(i as isize)).id } == id {
            return unsafe { tz.offset(i as isize) };
        }
        i += 1;
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn __thermal_zone_discover(tz: *mut thermal_zone, th: *mut c_void) -> c_int {
    if unsafe { thermal_cmd_get_trip(th, tz) } < 0 {
        return -1;
    }

    if unsafe { thermal_cmd_threshold_get(th, tz) } != 0 {
        return -1;
    }

    if unsafe { thermal_cmd_get_governor(th, tz) } != 0 {
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thermal_zone_discover(
    th: *mut thermal_handler,
) -> *mut thermal_zone {
    let mut tz: *mut thermal_zone = core::ptr::null_mut();

    if unsafe { thermal_cmd_get_tz(th, &mut tz) } < 0 {
        return core::ptr::null_mut();
    }

    if unsafe { for_each_thermal_zone(tz, Some(__thermal_zone_discover), th as *mut c_void) } != 0 {
        return core::ptr::null_mut();
    }

    tz
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thermal_exit(th: *mut thermal_handler) {
    unsafe {
        thermal_cmd_exit(th);
        thermal_events_exit(th);
        thermal_sampling_exit(th);

        free(th as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thermal_init(ops: *mut thermal_ops) -> *mut thermal_handler {
    let th: *mut thermal_handler;

    th = unsafe { malloc(core::mem::size_of::<thermal_handler>()) as *mut thermal_handler };
    if th.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        (*th).ops = ops;
    }

    if unsafe { thermal_events_init(th) } != 0 {
        unsafe {
            free(th as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    if unsafe { thermal_sampling_init(th) } != 0 {
        unsafe {
            free(th as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    if unsafe { thermal_cmd_init(th) } != 0 {
        unsafe {
            free(th as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    th
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
