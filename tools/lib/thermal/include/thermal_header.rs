/* SPDX-License-Identifier: LGPL-2.1+ */
/* Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org> */

use core::ffi::{c_char, c_int, c_void};

/* Depends on <linux/thermal.h> for THERMAL_NAME_LENGTH. */

/* LIBTHERMAL_API maps to default symbol visibility in C. */

pub const THERMAL_THRESHOLD_WAY_UP: c_int = 0x1;
pub const THERMAL_THRESHOLD_WAY_DOWN: c_int = 0x2;

#[repr(C)]
pub struct thermal_sampling_ops {
    pub tz_temp: Option<unsafe extern "C" fn(tz_id: c_int, temp: c_int, arg: *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct thermal_events_ops {
    pub tz_create:
        Option<unsafe extern "C" fn(name: *const c_char, tz_id: c_int, arg: *mut c_void) -> c_int>,
    pub tz_delete: Option<unsafe extern "C" fn(tz_id: c_int, arg: *mut c_void) -> c_int>,
    pub tz_enable: Option<unsafe extern "C" fn(tz_id: c_int, arg: *mut c_void) -> c_int>,
    pub tz_disable: Option<unsafe extern "C" fn(tz_id: c_int, arg: *mut c_void) -> c_int>,
    pub trip_high: Option<
        unsafe extern "C" fn(tz_id: c_int, trip_id: c_int, temp: c_int, arg: *mut c_void) -> c_int,
    >,
    pub trip_low: Option<
        unsafe extern "C" fn(tz_id: c_int, trip_id: c_int, temp: c_int, arg: *mut c_void) -> c_int,
    >,
    pub trip_add: Option<
        unsafe extern "C" fn(
            tz_id: c_int,
            trip_id: c_int,
            type_: c_int,
            temp: c_int,
            hyst: c_int,
            arg: *mut c_void,
        ) -> c_int,
    >,
    pub trip_change: Option<
        unsafe extern "C" fn(
            tz_id: c_int,
            trip_id: c_int,
            type_: c_int,
            temp: c_int,
            hyst: c_int,
            arg: *mut c_void,
        ) -> c_int,
    >,
    pub trip_delete:
        Option<unsafe extern "C" fn(tz_id: c_int, trip_id: c_int, arg: *mut c_void) -> c_int>,
    pub cdev_add: Option<
        unsafe extern "C" fn(
            name: *const c_char,
            cdev_id: c_int,
            max_state: c_int,
            arg: *mut c_void,
        ) -> c_int,
    >,
    pub cdev_delete: Option<unsafe extern "C" fn(cdev_id: c_int, arg: *mut c_void) -> c_int>,
    pub cdev_update:
        Option<unsafe extern "C" fn(cdev_id: c_int, cur_state: c_int, arg: *mut c_void) -> c_int>,
    pub gov_change:
        Option<unsafe extern "C" fn(tz_id: c_int, gov_name: *const c_char, arg: *mut c_void) -> c_int>,
    pub threshold_add: Option<
        unsafe extern "C" fn(
            tz_id: c_int,
            temperature: c_int,
            direction: c_int,
            arg: *mut c_void,
        ) -> c_int,
    >,
    pub threshold_delete: Option<
        unsafe extern "C" fn(
            tz_id: c_int,
            temperature: c_int,
            direction: c_int,
            arg: *mut c_void,
        ) -> c_int,
    >,
    pub threshold_flush: Option<unsafe extern "C" fn(tz_id: c_int, arg: *mut c_void) -> c_int>,
    pub threshold_up: Option<
        unsafe extern "C" fn(tz_id: c_int, temp: c_int, prev_temp: c_int, arg: *mut c_void) -> c_int,
    >,
    pub threshold_down: Option<
        unsafe extern "C" fn(tz_id: c_int, temp: c_int, prev_temp: c_int, arg: *mut c_void) -> c_int,
    >,
}

#[repr(C)]
pub struct thermal_ops {
    pub sampling: thermal_sampling_ops,
    pub events: thermal_events_ops,
}

#[repr(C)]
pub struct thermal_trip {
    pub id: c_int,
    pub type_: c_int,
    pub temp: c_int,
    pub hyst: c_int,
}

#[repr(C)]
pub struct thermal_threshold {
    pub temperature: c_int,
    pub direction: c_int,
}

#[repr(C)]
pub struct thermal_zone {
    pub id: c_int,
    pub temp: c_int,
    pub name: [c_char; THERMAL_NAME_LENGTH as usize],
    pub governor: [c_char; THERMAL_NAME_LENGTH as usize],
    pub trip: *mut thermal_trip,
    pub thresholds: *mut thermal_threshold,
}

#[repr(C)]
pub struct thermal_cdev {
    pub id: c_int,
    pub name: [c_char; THERMAL_NAME_LENGTH as usize],
    pub max_state: c_int,
    pub min_state: c_int,
    pub cur_state: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum thermal_error_t {
    THERMAL_ERROR = -1,
    THERMAL_SUCCESS = 0,
}

#[repr(C)]
pub struct thermal_handler {
    _unused: [u8; 0],
}

pub type cb_tz_t =
    Option<unsafe extern "C" fn(arg1: *mut thermal_zone, arg2: *mut c_void) -> c_int>;

pub type cb_tt_t =
    Option<unsafe extern "C" fn(arg1: *mut thermal_trip, arg2: *mut c_void) -> c_int>;

pub type cb_tc_t =
    Option<unsafe extern "C" fn(arg1: *mut thermal_cdev, arg2: *mut c_void) -> c_int>;

pub type cb_th_t =
    Option<unsafe extern "C" fn(arg1: *mut thermal_threshold, arg2: *mut c_void) -> c_int>;

unsafe extern "C" {
    pub fn for_each_thermal_zone(tz: *mut thermal_zone, cb: cb_tz_t, arg: *mut c_void) -> c_int;

    pub fn for_each_thermal_trip(tt: *mut thermal_trip, cb: cb_tt_t, arg: *mut c_void) -> c_int;

    pub fn for_each_thermal_cdev(cdev: *mut thermal_cdev, cb: cb_tc_t, arg: *mut c_void)
        -> c_int;

    pub fn for_each_thermal_threshold(
        th: *mut thermal_threshold,
        cb: cb_th_t,
        arg: *mut c_void,
    ) -> c_int;

    pub fn thermal_zone_find_by_name(
        tz: *mut thermal_zone,
        name: *const c_char,
    ) -> *mut thermal_zone;

    pub fn thermal_zone_find_by_id(tz: *mut thermal_zone, id: c_int) -> *mut thermal_zone;

    pub fn thermal_zone_discover(th: *mut thermal_handler) -> *mut thermal_zone;

    pub fn thermal_init(ops: *mut thermal_ops) -> *mut thermal_handler;

    pub fn thermal_exit(th: *mut thermal_handler);

    /*
     * Netlink thermal events
     */
    pub fn thermal_events_exit(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_events_init(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_events_handle(th: *mut thermal_handler, arg: *mut c_void) -> thermal_error_t;

    pub fn thermal_events_fd(th: *mut thermal_handler) -> c_int;

    /*
     * Netlink thermal commands
     */
    pub fn thermal_cmd_exit(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_cmd_init(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_cmd_get_tz(
        th: *mut thermal_handler,
        tz: *mut *mut thermal_zone,
    ) -> thermal_error_t;

    pub fn thermal_cmd_get_cdev(
        th: *mut thermal_handler,
        tc: *mut *mut thermal_cdev,
    ) -> thermal_error_t;

    pub fn thermal_cmd_get_trip(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
    ) -> thermal_error_t;

    pub fn thermal_cmd_get_governor(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
    ) -> thermal_error_t;

    pub fn thermal_cmd_get_temp(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
    ) -> thermal_error_t;

    pub fn thermal_cmd_threshold_get(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
    ) -> thermal_error_t;

    pub fn thermal_cmd_threshold_add(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
        temperature: c_int,
        direction: c_int,
    ) -> thermal_error_t;

    pub fn thermal_cmd_threshold_delete(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
        temperature: c_int,
        direction: c_int,
    ) -> thermal_error_t;

    pub fn thermal_cmd_threshold_flush(
        th: *mut thermal_handler,
        tz: *mut thermal_zone,
    ) -> thermal_error_t;

    /*
     * Netlink thermal samples
     */
    pub fn thermal_sampling_exit(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_sampling_init(th: *mut thermal_handler) -> thermal_error_t;

    pub fn thermal_sampling_handle(th: *mut thermal_handler, arg: *mut c_void)
        -> thermal_error_t;

    pub fn thermal_sampling_fd(th: *mut thermal_handler) -> c_int;
}
