/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PPS API kernel header
 *
 * Copyright (C) 2009   Rodolfo Giometti <giometti@linux.it>
 */

/* Dependencies supplied by the corresponding kernel headers are intentionally
 * referenced here rather than reimplemented. */

use core::ffi::c_void;

/* Global defines */

#[repr(C)]
pub struct pps_device;

/* The specific PPS source info */
#[repr(C)]
pub struct pps_source_info {
    pub name: [core::ffi::c_char; PPS_MAX_NAME_LEN], /* symbolic name */
    pub path: [core::ffi::c_char; PPS_MAX_NAME_LEN], /* path of connected device */
    pub mode: i32, /* PPS allowed mode */

    pub echo: Option<unsafe extern "C" fn(pps: *mut pps_device, event: i32, data: *mut c_void)>, /* PPS echo function */

    pub owner: *mut module,
    pub dev: *mut device, /* Parent device for device_create */
}

#[repr(C)]
pub struct pps_event_time {
    #[cfg(CONFIG_NTP_PPS)]
    pub ts_raw: timespec64,
    pub ts_real: timespec64,
}

/* The main struct */
#[repr(C)]
pub struct pps_device {
    pub info: pps_source_info, /* PSS source info */

    pub params: pps_kparams, /* PPS current params */

    pub assert_sequence: u32, /* PPS assert event seq # */
    pub clear_sequence: u32, /* PPS clear event seq # */
    pub assert_tu: pps_ktime,
    pub clear_tu: pps_ktime,
    pub current_mode: i32, /* PPS mode at event time */

    pub last_ev: u32, /* last PPS event id */
    pub last_fetched_ev: u32, /* last fetched PPS event id */
    pub queue: wait_queue_head_t, /* PPS event queue */

    pub id: u32, /* PPS source unique ID */
    pub lookup_cookie: *const c_void, /* For pps_lookup_dev() only */
    pub dev: device,
    pub async_queue: *mut fasync_struct, /* fasync method */
    pub lock: spinlock_t,
    pub kc_removed: bool,
}

/* Opaque types supplied by kernel headers. */
#[repr(C)] pub struct module;
#[repr(C)] pub struct device;
#[repr(C)] pub struct fasync_struct;

/* Global variables */

extern "C" {
    pub static pps_groups: *const *const attribute_group;

    /* Internal functions. */
    pub fn pps_register_cdev(pps: *mut pps_device) -> i32;
    pub fn pps_unregister_cdev(pps: *mut pps_device);

    /* Exported functions */
    pub fn pps_register_source(info: *mut pps_source_info, default_params: i32) -> *mut pps_device;
    pub fn pps_unregister_source(pps: *mut pps_device);
    pub fn pps_event(pps: *mut pps_device, ts: *mut pps_event_time, event: i32, data: *mut c_void);
    /* Look up a pps_device by magic cookie */
    pub fn pps_lookup_dev(cookie: *const c_void) -> *mut pps_device;
}

pub unsafe fn timespec_to_pps_ktime(kt: *mut pps_ktime, ts: timespec64) {
    (*kt).sec = ts.tv_sec;
    (*kt).nsec = ts.tv_nsec;
}

pub unsafe fn pps_get_ts(ts: *mut pps_event_time) {
    #[cfg(CONFIG_NTP_PPS)]
    {
        let mut snap: system_time_snapshot = core::mem::zeroed();
        ktime_get_snapshot_id(CLOCK_REALTIME, &mut snap);
        (*ts).ts_real = ktime_to_timespec64(snap.systime);
        (*ts).ts_raw = ktime_to_timespec64(snap.monoraw);
    }
    #[cfg(not(CONFIG_NTP_PPS))]
    {
        ktime_get_real_ts64(&mut (*ts).ts_real);
    }
}

/* Subtract known time delay from PPS event time(s) */
pub unsafe fn pps_sub_ts(ts: *mut pps_event_time, delta: timespec64) {
    (*ts).ts_real = timespec64_sub((*ts).ts_real, delta);
    #[cfg(CONFIG_NTP_PPS)]
    {
        (*ts).ts_raw = timespec64_sub((*ts).ts_raw, delta);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
