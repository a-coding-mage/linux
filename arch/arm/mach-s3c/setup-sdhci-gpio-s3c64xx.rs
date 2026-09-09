// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// S3C64XX - Helper functions for setting up SDHCI device(s) GPIO (HSMMC)

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external here.  The original C includes are not executable Rust items.

use core::ffi::c_void;

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: Device,
}

#[repr(C)]
pub struct s3c_sdhci_platdata {
    pub cd_type: i32,
}

extern "C" {
    fn s3c_gpio_cfgrange_nopull(pin: i32, count: i32, config: i32);
    fn s3c_gpio_setpull(pin: i32, pull: i32);
    fn s3c_gpio_cfgpin(pin: i32, config: i32);

    // These correspond to the GPIO helper macros and constants from the
    // included Samsung GPIO headers.
    fn S3C64XX_GPG(pin: i32) -> i32;
    fn S3C64XX_GPH(pin: i32) -> i32;
    fn S3C64XX_GPC(pin: i32) -> i32;
    fn S3C_GPIO_SFN(function: i32) -> i32;
}

// Build-time header constants retained as external symbols for the dependent
// translation unit.
extern "C" {
    static S3C_GPIO_PULL_UP: i32;
    static S3C_SDHCI_CD_INTERNAL: i32;
}

pub unsafe fn s3c64xx_setup_sdhci0_cfg_gpio(dev: *mut platform_device, width: i32) {
    let pdata = (*dev).dev.platform_data as *mut s3c_sdhci_platdata;

    /* Set all the necessary GPG pins to special-function 2 */
    s3c_gpio_cfgrange_nopull(S3C64XX_GPG(0), 2 + width, S3C_GPIO_SFN(2));

    if (*pdata).cd_type == S3C_SDHCI_CD_INTERNAL {
        s3c_gpio_setpull(S3C64XX_GPG(6), S3C_GPIO_PULL_UP);
        s3c_gpio_cfgpin(S3C64XX_GPG(6), S3C_GPIO_SFN(2));
    }
}

pub unsafe fn s3c64xx_setup_sdhci1_cfg_gpio(dev: *mut platform_device, width: i32) {
    let pdata = (*dev).dev.platform_data as *mut s3c_sdhci_platdata;

    /* Set all the necessary GPH pins to special-function 2 */
    s3c_gpio_cfgrange_nopull(S3C64XX_GPH(0), 2 + width, S3C_GPIO_SFN(2));

    if (*pdata).cd_type == S3C_SDHCI_CD_INTERNAL {
        s3c_gpio_setpull(S3C64XX_GPG(6), S3C_GPIO_PULL_UP);
        s3c_gpio_cfgpin(S3C64XX_GPG(6), S3C_GPIO_SFN(3));
    }
}

pub unsafe fn s3c64xx_setup_sdhci2_cfg_gpio(_dev: *mut platform_device, width: i32) {
    /* Set all the necessary GPH pins to special-function 3 */
    s3c_gpio_cfgrange_nopull(S3C64XX_GPH(6), width, S3C_GPIO_SFN(3));

    /* Set all the necessary GPC pins to special-function 3 */
    s3c_gpio_cfgrange_nopull(S3C64XX_GPC(4), 2, S3C_GPIO_SFN(3));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
