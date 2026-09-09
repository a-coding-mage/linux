// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Base S3C64XX I2C bus 1 gpio configuration

// The declarations below are supplied by the corresponding Linux headers and
// other translation units.

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn s3c_gpio_cfgall_range(
        start: u32,
        nr: u32,
        cfg: u32,
        pull: u32,
    );

    fn S3C64XX_GPB(pin: u32) -> u32;
    fn S3C_GPIO_SFN(function: u32) -> u32;

    static S3C_GPIO_PULL_UP: u32;
}

pub unsafe fn s3c_i2c1_cfg_gpio(_dev: *mut platform_device) {
    s3c_gpio_cfgall_range(
        S3C64XX_GPB(2),
        2,
        S3C_GPIO_SFN(6),
        S3C_GPIO_PULL_UP,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
