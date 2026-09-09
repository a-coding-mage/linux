// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Base S3C64XX I2C bus 0 gpio configuration

// Dependencies supplied by the Linux kernel and the corresponding platform
// headers are intentionally left external to this translation unit.

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    fn s3c_gpio_cfgall_range(
        start: u32,
        nr: u32,
        cfg: u32,
        pull: u32,
    );

    // These declarations represent the source macros/constants supplied by
    // gpio-cfg.h and gpio-samsung.h.
    fn S3C64XX_GPB(pin: u32) -> u32;
    fn S3C_GPIO_SFN(function: u32) -> u32;
    static S3C_GPIO_PULL_UP: u32;
}

pub unsafe fn s3c_i2c0_cfg_gpio(_dev: *mut platform_device) {
    s3c_gpio_cfgall_range(
        S3C64XX_GPB(5),
        2,
        S3C_GPIO_SFN(2),
        S3C_GPIO_PULL_UP,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
