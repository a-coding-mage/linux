/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Chrome OS EC MEMS Sensor Hub driver.
 *
 * Copyright 2019 Google LLC
 */

// Translated from the C header. Types supplied by included kernel headers are
// intentionally referenced but not defined here.

use core::ffi::c_void;

pub struct iio_dev;
pub struct device;
pub struct cros_ec_dev;
pub struct cros_ec_command;
pub struct ec_params_motion_sense;
pub struct ec_response_motion_sense;
pub struct mutex;
pub struct notifier_block;
pub struct ec_response_motion_sense_fifo_info;

pub type u8 = core::ffi::c_uchar;
pub type s16 = core::ffi::c_short;
pub type s32 = core::ffi::c_int;
pub type s64 = core::ffi::c_long;
pub type ktime_t = s64;
pub type bool_ = u8;

/**
 * struct cros_ec_sensor_platform - ChromeOS EC sensor platform information.
 * @sensor_num: Id of the sensor, as reported by the EC.
 */
#[repr(C)]
pub struct cros_ec_sensor_platform {
    pub sensor_num: u8,
}

/**
 * typedef cros_ec_sensorhub_push_data_cb_t - Callback function to send datum
 *                                             to specific sensors.
 */
pub type cros_ec_sensorhub_push_data_cb_t =
    unsafe extern "C" fn(indio_dev: *mut iio_dev, data: *mut s16, timestamp: s64) -> s32;

#[repr(C)]
pub struct cros_ec_sensorhub_sensor_push_data {
    pub indio_dev: *mut iio_dev,
    pub push_data_cb: Option<cros_ec_sensorhub_push_data_cb_t>,
}

pub const CROS_EC_SENSOR_LAST_TS: i32 = 0;
pub const CROS_EC_SENSOR_NEW_TS: i32 = 1;
pub const CROS_EC_SENSOR_ALL_TS: i32 = 2;

#[repr(C, packed)]
pub struct cros_ec_sensors_ring_sample {
    pub sensor_id: u8,
    pub flag: u8,
    pub vector: [s16; 3],
    pub timestamp: s64,
}

/* State used for cros_ec_ring_fix_overflow */
#[repr(C)]
pub struct cros_ec_sensors_ec_overflow_state {
    pub offset: s64,
    pub last: s64,
}

/* Length of the filter, how long to remember entries for */
pub const CROS_EC_SENSORHUB_TS_HISTORY_SIZE: usize = 64;

/**
 * struct cros_ec_sensors_ts_filter_state - Timestamp filetr state.
 */
#[repr(C)]
pub struct cros_ec_sensors_ts_filter_state {
    pub x_offset: s64,
    pub y_offset: s64,
    pub x_history: [s64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub y_history: [s64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub m_history: [s64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub history_len: core::ffi::c_int,
    pub temp_buf: [s64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub median_m: s64,
    pub median_error: s64,
}

/* struct cros_ec_sensors_ts_batch_state - State of batch of a single sensor. */
#[repr(C)]
pub struct cros_ec_sensors_ts_batch_state {
    pub penul_ts: s64,
    pub penul_len: core::ffi::c_int,
    pub last_ts: s64,
    pub last_len: core::ffi::c_int,
    pub newest_sensor_event: s64,
}

/* struct cros_ec_sensorhub - Sensor Hub device data. */
#[repr(C)]
pub struct cros_ec_sensorhub {
    pub dev: *mut device,
    pub ec: *mut cros_ec_dev,
    pub sensor_num: core::ffi::c_int,
    pub msg: *mut cros_ec_command,
    pub params: *mut ec_params_motion_sense,
    pub resp: *mut ec_response_motion_sense,
    pub cmd_lock: mutex,
    pub notifier: notifier_block,
    pub ring: *mut cros_ec_sensors_ring_sample,
    pub fifo_timestamp: [ktime_t; CROS_EC_SENSOR_ALL_TS as usize],
    pub fifo_info: *mut ec_response_motion_sense_fifo_info,
    pub fifo_size: core::ffi::c_int,
    pub batch_state: *mut cros_ec_sensors_ts_batch_state,
    pub overflow_a: cros_ec_sensors_ec_overflow_state,
    pub overflow_b: cros_ec_sensors_ec_overflow_state,
    pub filter: cros_ec_sensors_ts_filter_state,
    pub tight_timestamps: core::ffi::c_int,
    pub future_timestamp_count: s32,
    pub future_timestamp_total_ns: s64,
    pub push_data: *mut cros_ec_sensorhub_sensor_push_data,
}

extern "C" {
    pub fn cros_ec_sensorhub_register_push_data(
        sensorhub: *mut cros_ec_sensorhub,
        sensor_num: u8,
        indio_dev: *mut iio_dev,
        cb: Option<cros_ec_sensorhub_push_data_cb_t>,
    ) -> s32;

    pub fn cros_ec_sensorhub_unregister_push_data(
        sensorhub: *mut cros_ec_sensorhub,
        sensor_num: u8,
    );

    pub fn cros_ec_sensorhub_ring_allocate(sensorhub: *mut cros_ec_sensorhub) -> s32;
    pub fn cros_ec_sensorhub_ring_add(sensorhub: *mut cros_ec_sensorhub) -> s32;
    pub fn cros_ec_sensorhub_ring_remove(arg: *mut c_void);
    pub fn cros_ec_sensorhub_ring_fifo_enable(
        sensorhub: *mut cros_ec_sensorhub,
        on: bool_,
    ) -> s32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
