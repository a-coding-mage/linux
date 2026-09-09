/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2004-2009 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C - I2C Controller platform_device info
 */

/* C header guard: __I2C_S3C2410_H */

pub const S3C_IICFLG_FILTER: u32 = 1u32 << 0; /* enable s3c2440 filter */

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

/**
 * struct s3c2410_platform_i2c - Platform data for s3c I2C.
 * @bus_num: The bus number to use (if possible).
 * @flags: Any flags for the I2C bus (E.g. S3C_IICFLK_FILTER).
 * @slave_addr: The I2C address for the slave device (if enabled).
 * @frequency: The desired frequency in Hz of the bus.  This is
 *             guaranteed to not be exceeded.  If the caller does
 *             not care, use zero and the driver will select a
 *             useful default.
 * @sda_delay: The delay (in ns) applied to SDA edges.
 * @cfg_gpio: A callback to configure the pins for I2C operation.
 */
#[repr(C)]
pub struct s3c2410_platform_i2c {
    pub bus_num: i32,
    pub flags: u32,
    pub slave_addr: u32,
    pub frequency: core::ffi::c_ulong,
    pub sda_delay: u32,
    pub cfg_gpio: Option<unsafe extern "C" fn(dev: *mut platform_device)>,
}

/**
 * s3c_i2c0_set_platdata - set platform data for i2c0 device
 * @i2c: The platform data to set, or NULL for default data.
 *
 * Register the given platform data for use with the i2c0 device. This
 * call copies the platform data, so the caller can use __initdata for
 * their copy.
 *
 * This call will set cfg_gpio if is null to the default platform
 * implementation.
 *
 * Any user of s3c_device_i2c0 should call this, even if it is with
 * NULL to ensure that the device is given the default platform data
 * as the driver will no longer carry defaults.
 */
unsafe extern "C" {
    pub fn s3c_i2c0_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c1_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c2_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c3_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c4_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c5_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c6_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s3c_i2c7_set_platdata(i2c: *mut s3c2410_platform_i2c);
    pub fn s5p_i2c_hdmiphy_set_platdata(i2c: *mut s3c2410_platform_i2c);

    /* defined by architecture to configure gpio */
    pub fn s3c_i2c0_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c1_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c2_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c3_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c4_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c5_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c6_cfg_gpio(dev: *mut platform_device);
    pub fn s3c_i2c7_cfg_gpio(dev: *mut platform_device);

    pub static mut default_i2c_data: s3c2410_platform_i2c;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
