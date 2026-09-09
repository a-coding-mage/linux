/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2014, Samsung Electronics Co. Ltd. All Rights Reserved.
 */

// Dependency supplied by linux/iio/iio.h in the original header.

pub const SSP_TIME_SIZE: usize = 4;
pub const SSP_ACCELEROMETER_SIZE: usize = 6;
pub const SSP_GYROSCOPE_SIZE: usize = 6;
pub const SSP_BIO_HRM_RAW_SIZE: usize = 8;
pub const SSP_BIO_HRM_RAW_FAC_SIZE: usize = 36;
pub const SSP_BIO_HRM_LIB_SIZE: usize = 8;

/**
 * enum ssp_sensor_type - SSP sensor type
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ssp_sensor_type {
    SSP_ACCELEROMETER_SENSOR = 0,
    SSP_GYROSCOPE_SENSOR,
    SSP_GEOMAGNETIC_UNCALIB_SENSOR,
    SSP_GEOMAGNETIC_RAW,
    SSP_GEOMAGNETIC_SENSOR,
    SSP_PRESSURE_SENSOR,
    SSP_GESTURE_SENSOR,
    SSP_PROXIMITY_SENSOR,
    SSP_TEMPERATURE_HUMIDITY_SENSOR,
    SSP_LIGHT_SENSOR,
    SSP_PROXIMITY_RAW,
    SSP_ORIENTATION_SENSOR,
    SSP_STEP_DETECTOR,
    SSP_SIG_MOTION_SENSOR,
    SSP_GYRO_UNCALIB_SENSOR,
    SSP_GAME_ROTATION_VECTOR,
    SSP_ROTATION_VECTOR,
    SSP_STEP_COUNTER,
    SSP_BIO_HRM_RAW,
    SSP_BIO_HRM_RAW_FAC,
    SSP_BIO_HRM_LIB,
    SSP_SENSOR_MAX,
}

pub enum ssp_data {}
pub enum iio_dev {}

/**
 * struct ssp_sensor_data - Sensor object
 * @process_data:    Callback to feed sensor data.
 * @type:            Used sensor type.
 * @buffer:          Received data buffer.
 */
#[repr(C)]
pub struct ssp_sensor_data {
    pub process_data:
        Option<unsafe extern "C" fn(indio_dev: *mut iio_dev, buf: *mut core::ffi::c_void, timestamp: i64) -> i32>,
    pub type_: ssp_sensor_type,
    pub buffer: *mut u8,
}

unsafe extern "C" {
    pub fn ssp_register_consumer(indio_dev: *mut iio_dev, type_: ssp_sensor_type);

    pub fn ssp_enable_sensor(
        data: *mut ssp_data,
        type_: ssp_sensor_type,
        delay: u32,
    ) -> i32;

    pub fn ssp_disable_sensor(data: *mut ssp_data, type_: ssp_sensor_type) -> i32;

    pub fn ssp_get_sensor_delay(data: *mut ssp_data, type_: ssp_sensor_type) -> u32;

    pub fn ssp_change_delay(
        data: *mut ssp_data,
        type_: ssp_sensor_type,
        delay: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
