/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * itg3200.h -- support InvenSense ITG3200
 *              Digital 3-Axis Gyroscope driver
 *
 * Copyright (c) 2011 Christian Strobel <christian.strobel@iis.fraunhofer.de>
 * Copyright (c) 2011 Manuel Stahl <manuel.stahl@iis.fraunhofer.de>
 * Copyright (c) 2012 Thorsten Nowak <thorsten.nowak@iis.fraunhofer.de>
 */

// Dependency supplied by the Linux IIO implementation: linux/iio/iio.h

/* Register with I2C address (34h) */
pub const ITG3200_REG_ADDRESS: u8 = 0x00;

/* Sample rate divider
 * Range: 0 to 255
 * Default value: 0x00 */
pub const ITG3200_REG_SAMPLE_RATE_DIV: u8 = 0x15;

/* Digital low pass filter settings */
pub const ITG3200_REG_DLPF: u8 = 0x16;
/* DLPF full scale range */
pub const ITG3200_DLPF_FS_SEL_2000: u8 = 0x18;
/* Bandwidth (Hz) and internal sample rate
 * (kHz) of DLPF */
pub const ITG3200_DLPF_256_8: u8 = 0x00;
pub const ITG3200_DLPF_188_1: u8 = 0x01;
pub const ITG3200_DLPF_98_1: u8 = 0x02;
pub const ITG3200_DLPF_42_1: u8 = 0x03;
pub const ITG3200_DLPF_20_1: u8 = 0x04;
pub const ITG3200_DLPF_10_1: u8 = 0x05;
pub const ITG3200_DLPF_5_1: u8 = 0x06;

pub const ITG3200_DLPF_CFG_MASK: u8 = 0x07;

/* Configuration for interrupt operations */
pub const ITG3200_REG_IRQ_CONFIG: u8 = 0x17;
/* Logic level */
pub const ITG3200_IRQ_ACTIVE_LOW: u8 = 0x80;
pub const ITG3200_IRQ_ACTIVE_HIGH: u8 = 0x00;
/* Drive type */
pub const ITG3200_IRQ_OPEN_DRAIN: u8 = 0x40;
pub const ITG3200_IRQ_PUSH_PULL: u8 = 0x00;
/* Latch mode */
pub const ITG3200_IRQ_LATCH_UNTIL_CLEARED: u8 = 0x20;
pub const ITG3200_IRQ_LATCH_50US_PULSE: u8 = 0x00;
/* Latch clear method */
pub const ITG3200_IRQ_LATCH_CLEAR_ANY: u8 = 0x10;
pub const ITG3200_IRQ_LATCH_CLEAR_STATUS: u8 = 0x00;
/* Enable interrupt when device is ready */
pub const ITG3200_IRQ_DEVICE_RDY_ENABLE: u8 = 0x04;
/* Enable interrupt when data is available */
pub const ITG3200_IRQ_DATA_RDY_ENABLE: u8 = 0x01;

/* Determine the status of ITG-3200 interrupts */
pub const ITG3200_REG_IRQ_STATUS: u8 = 0x1A;
/* Status of 'device is ready'-interrupt */
pub const ITG3200_IRQ_DEVICE_RDY_STATUS: u8 = 0x04;
/* Status of 'data is available'-interrupt */
pub const ITG3200_IRQ_DATA_RDY_STATUS: u8 = 0x01;

/* Sensor registers */
pub const ITG3200_REG_TEMP_OUT_H: u8 = 0x1B;
pub const ITG3200_REG_TEMP_OUT_L: u8 = 0x1C;
pub const ITG3200_REG_GYRO_XOUT_H: u8 = 0x1D;
pub const ITG3200_REG_GYRO_XOUT_L: u8 = 0x1E;
pub const ITG3200_REG_GYRO_YOUT_H: u8 = 0x1F;
pub const ITG3200_REG_GYRO_YOUT_L: u8 = 0x20;
pub const ITG3200_REG_GYRO_ZOUT_H: u8 = 0x21;
pub const ITG3200_REG_GYRO_ZOUT_L: u8 = 0x22;

/* Power management */
pub const ITG3200_REG_POWER_MANAGEMENT: u8 = 0x3E;
/* Reset device and internal registers to the
 * power-up-default settings */
pub const ITG3200_RESET: u8 = 0x80;
/* Enable low power sleep mode */
pub const ITG3200_SLEEP: u8 = 0x40;
/* Put according gyroscope in standby mode */
pub const ITG3200_STANDBY_GYRO_X: u8 = 0x20;
pub const ITG3200_STANDBY_GYRO_Y: u8 = 0x10;
pub const ITG3200_STANDBY_GYRO_Z: u8 = 0x08;
/* Determine the device clock source */
pub const ITG3200_CLK_INTERNAL: u8 = 0x00;
pub const ITG3200_CLK_GYRO_X: u8 = 0x01;
pub const ITG3200_CLK_GYRO_Y: u8 = 0x02;
pub const ITG3200_CLK_GYRO_Z: u8 = 0x03;
pub const ITG3200_CLK_EXT_32K: u8 = 0x04;
pub const ITG3200_CLK_EXT_19M: u8 = 0x05;

/**
 * struct itg3200 - device instance specific data
 * @i2c:    actual i2c_client
 * @trig:   data ready trigger from itg3200 pin
 **/
#[repr(C)]
pub struct itg3200 {
    pub i2c: *mut i2c_client,
    pub trig: *mut iio_trigger,
    pub orientation: iio_mount_matrix,
    /* lock to protect against multiple access to the device */
    pub lock: mutex,
}

#[repr(C)]
pub enum ITG3200_SCAN_INDEX {
    ITG3200_SCAN_TEMP,
    ITG3200_SCAN_GYRO_X,
    ITG3200_SCAN_GYRO_Y,
    ITG3200_SCAN_GYRO_Z,
    ITG3200_SCAN_ELEMENTS,
}

extern "C" {
    pub fn itg3200_write_reg_8(indio_dev: *mut iio_dev, reg_address: u8, val: u8) -> i32;
    pub fn itg3200_read_reg_8(indio_dev: *mut iio_dev, reg_address: u8, val: *mut u8) -> i32;
}

/* CONFIG_IIO_BUFFER is a build-time kernel configuration condition. */
#[cfg(feature = "CONFIG_IIO_BUFFER")]
extern "C" {
    pub fn itg3200_remove_trigger(indio_dev: *mut iio_dev);
    pub fn itg3200_probe_trigger(indio_dev: *mut iio_dev) -> i32;
    pub fn itg3200_buffer_configure(indio_dev: *mut iio_dev) -> i32;
    pub fn itg3200_buffer_unconfigure(indio_dev: *mut iio_dev);
}

#[cfg(not(feature = "CONFIG_IIO_BUFFER"))]
#[inline]
pub unsafe fn itg3200_remove_trigger(_indio_dev: *mut iio_dev) {}

#[cfg(not(feature = "CONFIG_IIO_BUFFER"))]
#[inline]
pub unsafe fn itg3200_probe_trigger(_indio_dev: *mut iio_dev) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_IIO_BUFFER"))]
#[inline]
pub unsafe fn itg3200_buffer_configure(_indio_dev: *mut iio_dev) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_IIO_BUFFER"))]
#[inline]
pub unsafe fn itg3200_buffer_unconfigure(_indio_dev: *mut iio_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
