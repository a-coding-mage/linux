/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ChromeOS EC sensor hub
 *
 * Copyright (C) 2016 Google, Inc
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.

pub const CROS_EC_SENSOR_X: i32 = 0;
pub const CROS_EC_SENSOR_Y: i32 = 1;
pub const CROS_EC_SENSOR_Z: i32 = 2;
pub const CROS_EC_SENSOR_MAX_AXIS: usize = 3;

/* EC returns sensor values using signed 16 bit registers */
pub const CROS_EC_SENSOR_BITS: i32 = 16;

/*
 * 4 16 bit channels are allowed.
 * Good enough for current sensors, they use up to 3 16 bit vectors.
 */
pub const CROS_EC_SAMPLE_SIZE: usize = core::mem::size_of::<i64>() * 2;

pub type CrosEcSensorsCaptureT = unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t;

/**
 * struct cros_ec_sensors_core_state - state data for EC sensors IIO driver
 * @ec:                         cros EC device structure
 * @cmd_lock:                  lock used to prevent simultaneous access to the
 *                             commands.
 * @msg:                       cros EC command structure
 * @param:                     motion sensor parameters structure
 * @resp:                      motion sensor response structure
 * @type:                      type of motion sensor
 * @range_updated:             True if the range of the sensor has been
 *                             updated.
 * @curr_range:                If updated, the current range value.
 *                             It will be reapplied at every resume.
 * @calib:                     calibration parameters. Note that trigger
 *                             captured data will always provide the calibrated
 *                             data
 * @samples:                   static array to hold data from a single capture.
 *                             For each channel we need 2 bytes, except for
 *                             the timestamp. The timestamp is always last and
 *                             is always 8-byte aligned.
 * @read_ec_sensors_data:      function used for accessing sensors values
 * @fifo_max_event_count:      Size of the EC sensor FIFO
 * @frequencies:               Table of known available frequencies:
 *                             0, Min and Max in mHz
 */
#[repr(C)]
pub struct CrosEcSensorsCoreState {
    pub ec: *mut cros_ec_device,
    pub cmd_lock: mutex,
    pub msg: *mut cros_ec_command,
    pub param: ec_params_motion_sense,
    pub resp: *mut ec_response_motion_sense,
    pub type_: motionsensor_type,
    pub range_updated: bool,
    pub curr_range: i32,
    pub calib: [CalibData; CROS_EC_SENSOR_MAX_AXIS],
    pub sign: [i8; CROS_EC_SENSOR_MAX_AXIS],
    pub samples: [u8; CROS_EC_SAMPLE_SIZE],
    pub read_ec_sensors_data: Option<
        unsafe extern "C" fn(indio_dev: *mut iio_dev, scan_mask: u64, data: *mut i16) -> i32,
    >,
    pub fifo_max_event_count: u32,
    pub frequencies: [i32; 6],
}

#[repr(C)]
pub struct CalibData {
    pub offset: i16,
    pub scale: u16,
}

pub unsafe extern "C" fn cros_ec_sensors_read_lpc(
    indio_dev: *mut iio_dev,
    scan_mask: u64,
    data: *mut i16,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_read_cmd(
    indio_dev: *mut iio_dev,
    scan_mask: u64,
    data: *mut i16,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_core_init(
    pdev: *mut platform_device,
    indio_dev: *mut iio_dev,
    physical_device: bool,
    trigger_capture: Option<CrosEcSensorsCaptureT>,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_core_register(
    dev: *mut device,
    indio_dev: *mut iio_dev,
    push_data: cros_ec_sensorhub_push_data_cb_t,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_capture(irq: i32, p: *mut c_void) -> irqreturn_t;

pub unsafe extern "C" fn cros_ec_sensors_push_data(
    indio_dev: *mut iio_dev,
    data: *mut i16,
    timestamp: i64,
) -> i32;

pub unsafe extern "C" fn cros_ec_motion_send_host_cmd(
    st: *mut CrosEcSensorsCoreState,
    opt_length: u16,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_core_read(
    st: *mut CrosEcSensorsCoreState,
    chan: *const iio_chan_spec,
    val: *mut i32,
    val2: *mut i32,
    mask: i64,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_core_read_avail(
    indio_dev: *mut iio_dev,
    chan: *const iio_chan_spec,
    vals: *mut *const i32,
    type_: *mut i32,
    length: *mut i32,
    mask: i64,
) -> i32;

pub unsafe extern "C" fn cros_ec_sensors_core_write(
    st: *mut CrosEcSensorsCoreState,
    chan: *const iio_chan_spec,
    val: i32,
    val2: i32,
    mask: i64,
) -> i32;

pub static mut cros_ec_sensors_pm_ops: dev_pm_ops;
pub static mut cros_ec_sensors_ext_info: [iio_chan_spec_ext_info; 0];
pub static mut cros_ec_sensors_limited_info: [iio_chan_spec_ext_info; 0];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
