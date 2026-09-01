/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tmon.h contains data structures and constants used by TMON
 *
 * Copyright (C) 2012 Intel Corporation. All rights reserved.
 *
 * Author Name Jacob Pan <jacob.jun.pan@linux.intel.com>
 */

use core::ffi::{c_char, c_double, c_int, c_ulong, c_void};

pub const MAX_DISP_TEMP: c_int = 125;
pub const MAX_CTRL_TEMP: c_int = 105;
pub const MIN_CTRL_TEMP: c_int = 40;
pub const MAX_NR_TZONE: usize = 16;
pub const MAX_NR_CDEV: usize = 32;
pub const MAX_NR_TRIP: usize = 16;
/* number of cooling devices that can bind
 * to a thermal zone trip.
 */
pub const MAX_NR_CDEV_TRIP: usize = 12;
pub const MAX_TEMP_KC: c_int = 140000;
/* starting char position to draw sensor data, such as tz names
 * trip point list, etc.
 */
pub const DATA_LEFT_ALIGN: c_int = 10;
pub const NR_LINES_TZDATA: c_int = 1;
pub const TMON_LOG_FILE: &[u8] = b"/var/tmp/tmon.log\0";

unsafe extern "C" {
    pub static mut ticktime: c_ulong;
    pub static mut time_elapsed: c_double;
    pub static mut target_temp_user: c_ulong;
    pub static mut dialogue_on: c_int;
    pub static mut ctrl_cdev: [c_char; 0];
    pub static mut input_lock: libc::pthread_mutex_t;
    pub static mut tmon_exit: c_int;
    pub static mut target_thermal_zone: c_int;
}

/* use fixed size record to simplify data processing and transfer
 * TBD: more info to be added, e.g. programmable trip point data.
*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_data_record {
    pub tv: libc::timeval,
    pub temp: [c_ulong; MAX_NR_TZONE],
    pub pid_out_pct: c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdev_info {
    pub type_: [c_char; 64],
    pub instance: c_int,
    pub max_state: c_ulong,
    pub cur_state: c_ulong,
    pub flag: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum trip_type {
    THERMAL_TRIP_CRITICAL,
    THERMAL_TRIP_HOT,
    THERMAL_TRIP_PASSIVE,
    THERMAL_TRIP_ACTIVE,
    NR_THERMAL_TRIP_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trip_point {
    pub type_: trip_type,
    pub temp: c_ulong,
    pub hysteresis: c_ulong,
    pub attribute: c_int, /* programmability etc. */
}

/* thermal zone configuration information, binding with cooling devices could
 * change at runtime.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tz_info {
    pub type_: [c_char; 256], /* e.g. acpitz */
    pub instance: c_int,
    pub passive: c_int, /* active zone has passive node to force passive mode */
    pub nr_cdev: c_int, /* number of cooling device binded */
    pub nr_trip_pts: c_int,
    pub tp: [trip_point; MAX_NR_TRIP],
    pub cdev_binding: c_ulong, /* bitmap for attached cdevs */
    /* cdev bind trip points, allow one cdev bind to multiple trips */
    pub trip_binding: [c_ulong; MAX_NR_CDEV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmon_platform_data {
    pub nr_tz_sensor: c_int,
    pub nr_cooling_dev: c_int,
    /* keep track of instance ids since there might be gaps */
    pub max_tz_instance: c_int,
    pub max_cdev_instance: c_int,
    pub tzi: *mut tz_info,
    pub cdi: *mut cdev_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_ops {
    pub set_ratio: Option<unsafe extern "C" fn(ratio: c_ulong)>,
    pub get_ratio: Option<unsafe extern "C" fn(ratio: c_ulong) -> c_ulong>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cdev_types {
    CDEV_TYPE_PROC,
    CDEV_TYPE_FAN,
    CDEV_TYPE_MEM,
    CDEV_TYPE_NR,
}

/* REVISIT: the idea is to group sensors if possible, e.g. on intel mid
 * we have "skin0", "skin1", "sys", "msicdie"
 * on DPTF enabled systems, we might have PCH, TSKN, TAMB, etc.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tzone_types {
    TZONE_TYPE_ACPI,
    TZONE_TYPE_PCH,
    TZONE_TYPE_NR,
}

/* limit the output of PID controller adjustment */
pub const LIMIT_HIGH: c_int = 95;
pub const LIMIT_LOW: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pid_params {
    pub kp: c_double, /* Controller gain from Dialog Box */
    pub ki: c_double, /* Time-constant for I action from Dialog Box */
    pub kd: c_double, /* Time-constant for D action from Dialog Box */
    pub ts: c_double,
    pub k_lpf: c_double,

    pub t_target: c_double,
    pub y_k: c_double,
}

unsafe extern "C" {
    pub fn init_thermal_controller() -> c_int;
    pub fn controller_handler(xk: c_double, yk: *mut c_double);

    pub static mut ptdata: tmon_platform_data;
    pub static mut p_param: pid_params;

    pub static mut tmon_log: *mut libc::FILE;
    pub static mut cur_thermal_record: c_int; /* index to the trec array */
    pub static mut trec: [thermal_data_record; 0];
    pub static mut trip_type_name: [*const c_char; 0];
    pub static mut no_control: c_ulong;

    pub fn initialize_curses();
    pub fn show_controller_stats(line: *mut c_char);
    pub fn show_title_bar();
    pub fn setup_windows();
    pub fn disable_tui();
    pub fn show_sensors_w();
    pub fn show_data_w();
    pub fn write_status_bar(x: c_int, line: *mut c_char);
    pub fn show_control_w();

    pub fn show_cooling_device();
    pub fn show_dialogue();
    pub fn update_thermal_data() -> c_int;

    pub fn probe_thermal_sysfs() -> c_int;
    pub fn free_thermal_data();
    pub fn resize_handler(sig: c_int);
    pub fn set_ctrl_state(state: c_ulong);
    pub fn get_ctrl_state(state: *mut c_ulong);
    pub fn handle_tui_events(arg: *mut c_void) -> *mut c_void;
    pub fn sysfs_set_ulong(path: *mut c_char, filename: *mut c_char, val: c_ulong) -> c_int;
    pub fn zone_instance_to_index(zone_inst: c_int) -> c_int;
    pub fn close_windows();
}

pub const PT_COLOR_DEFAULT: c_int = 1;
pub const PT_COLOR_HEADER_BAR: c_int = 2;
pub const PT_COLOR_ERROR: c_int = 3;
pub const PT_COLOR_RED: c_int = 4;
pub const PT_COLOR_YELLOW: c_int = 5;
pub const PT_COLOR_GREEN: c_int = 6;
pub const PT_COLOR_BRIGHT: c_int = 7;
pub const PT_COLOR_BLUE: c_int = 8;

/* each thermal zone uses 12 chars, 8 for name, 2 for instance, 2 space
 * also used to list trip points in forms of AAAC, which represents
 * A: Active
 * C: Critical
 */
pub const TZONE_RECORD_SIZE: c_int = 12;
pub const TZ_LEFT_ALIGN: c_int = 32;
pub const CDEV_NAME_SIZE: c_int = 20;
pub const CDEV_FLAG_IN_CONTROL: c_int = 1 << 0;

/* dialogue box starts */
pub const DIAG_X: c_int = 48;
pub const DIAG_Y: c_int = 8;
pub const THERMAL_SYSFS: &[u8] = b"/sys/class/thermal\0";
pub const CDEV: &[u8] = b"cooling_device\0";
pub const TZONE: &[u8] = b"thermal_zone\0";
pub const TDATA_LEFT: c_int = 16;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
