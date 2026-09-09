/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * s3c24xx/s3c64xx SoC series Camera Interface (CAMIF) driver
 *
 * Copyright (C) 2012 Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
 */

// Dependencies supplied by the surrounding kernel/media translation.

/**
 * struct s3c_camif_sensor_info - an image sensor description
 * @i2c_board_info: pointer to an I2C sensor subdevice board info
 * @clock_frequency: frequency of the clock the host provides to a sensor
 * @mbus_type: media bus type
 * @i2c_bus_num: i2c control bus id the sensor is attached to
 * @flags: the parallel bus flags defining signals polarity (V4L2_MBUS_*)
 * @use_field: 1 if parallel bus FIELD signal is used (only s3c64xx)
 */
#[repr(C)]
pub struct s3c_camif_sensor_info {
    pub i2c_board_info: i2c_board_info,
    pub clock_frequency: core::ffi::c_ulong,
    pub mbus_type: v4l2_mbus_type,
    pub i2c_bus_num: u16,
    pub flags: u16,
    pub use_field: u8,
}

#[repr(C)]
pub struct s3c_camif_plat_data {
    pub sensor: s3c_camif_sensor_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
