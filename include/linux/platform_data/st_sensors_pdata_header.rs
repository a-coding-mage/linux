/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STMicroelectronics sensors platform-data driver
 *
 * Copyright 2013 STMicroelectronics Inc.
 *
 * Denis Ciocca <denis.ciocca@st.com>
 */

/// Platform data for the ST sensors.
///
/// `drdy_int_pin`: Redirect DRDY on pin 1 (1) or pin 2 (2).
/// Available only for accelerometer, magnetometer and pressure sensors.
/// Accelerometer DRDY on LSM330 available only on pin 1 (see datasheet).
/// Magnetometer DRDY is supported only on LSM9DS0 and LSM303D.
///
/// `open_drain`: set the interrupt line to be open drain if possible.
/// `spi_3wire`: enable spi-3wire mode.
/// `pullups`: enable/disable i2c controller pullup resistors.
/// `wakeup_source`: enable/disable device as wakeup generator.
#[repr(C)]
pub struct st_sensors_platform_data {
    pub drdy_int_pin: u8,
    pub open_drain: bool,
    pub spi_3wire: bool,
    pub pullups: bool,
    pub wakeup_source: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
