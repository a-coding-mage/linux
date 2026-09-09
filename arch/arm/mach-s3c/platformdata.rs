// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2010 Ben Dooks <ben-linux <at> fluff.org>
//
// Helper for platform data setting

use core::ffi::{c_char, c_int, c_void};
// Supplied by the kernel and by the platform-device headers.
const GFP_KERNEL: c_int = 0x20;

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub dev: device,
}

#[repr(C)]
pub struct s3c_sdhci_platdata {
    pub cd_type: c_int,
    pub ext_cd_init: Option<unsafe extern "C" fn() -> c_int>,
    pub ext_cd_cleanup: Option<unsafe extern "C" fn()>,
    pub ext_cd_gpio: c_int,
    pub ext_cd_gpio_invert: c_int,
    pub max_width: c_int,
    pub cfg_gpio: Option<unsafe extern "C" fn()>,
    pub host_caps: u32,
    pub host_caps2: u32,
    pub pm_caps: u32,
}

extern "C" {
    fn kmemdup(src: *const c_void, size: usize, flags: c_int) -> *mut c_void;
    fn printk(fmt: *const c_char, ...);
}

pub unsafe extern "C" fn s3c_set_platdata(
    pd: *mut c_void,
    pdsize: usize,
    pdev: *mut platform_device,
) -> *mut c_void {
    let mut npd: *mut c_void;

    if pd.is_null() {
        // too early to use dev_name(), may not be registered
        let fmt = b"<3>%s: no platform data supplied\n\0";
        printk(
            fmt.as_ptr() as *const c_char,
            (*pdev).name,
        );
        return core::ptr::null_mut();
    }

    npd = kmemdup(pd, pdsize, GFP_KERNEL);
    if npd.is_null() {
        return core::ptr::null_mut();
    }

    (*pdev).dev.platform_data = npd;
    npd
}

pub unsafe extern "C" fn s3c_sdhci_set_platdata(
    pd: *mut s3c_sdhci_platdata,
    set: *mut s3c_sdhci_platdata,
) {
    (*set).cd_type = (*pd).cd_type;
    (*set).ext_cd_init = (*pd).ext_cd_init;
    (*set).ext_cd_cleanup = (*pd).ext_cd_cleanup;
    (*set).ext_cd_gpio = (*pd).ext_cd_gpio;
    (*set).ext_cd_gpio_invert = (*pd).ext_cd_gpio_invert;

    if (*pd).max_width != 0 {
        (*set).max_width = (*pd).max_width;
    }
    if (*pd).cfg_gpio.is_some() {
        (*set).cfg_gpio = (*pd).cfg_gpio;
    }
    if (*pd).host_caps != 0 {
        (*set).host_caps |= (*pd).host_caps;
    }
    if (*pd).host_caps2 != 0 {
        (*set).host_caps2 |= (*pd).host_caps2;
    }
    if (*pd).pm_caps != 0 {
        (*set).pm_caps |= (*pd).pm_caps;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
