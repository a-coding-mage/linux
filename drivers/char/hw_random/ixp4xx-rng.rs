// SPDX-License-Identifier: GPL-2.0
/*
 * drivers/char/hw_random/ixp4xx-rng.c
 *
 * RNG driver for Intel IXP4xx family of NPUs
 *
 * Author: Deepak Saxena <dsaxena@plexity.net>
 *
 * Copyright 2005 (c) MontaVista Software, Inc.
 *
 * Fixes by Michael Buesch
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn cpu_is_ixp46x() -> bool;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: i32) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
    fn raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct hwrng {
    pub name: *const u8,
    pub data_read: Option<unsafe extern "C" fn(rng: *mut hwrng, buffer: *mut u32) -> i32>,
    pub priv_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> i32>,
}

const ENOSYS: i32 = 38;

unsafe extern "C" fn ixp4xx_rng_data_read(rng: *mut hwrng, buffer: *mut u32) -> i32 {
    let rng_base = (*rng).priv_data as *mut core::ffi::c_void;

    *buffer = raw_readl(rng_base);

    4
}

static mut ixp4xx_rng_ops: hwrng = hwrng {
    name: b"ixp4xx\0".as_ptr(),
    data_read: Some(ixp4xx_rng_data_read),
    priv_data: 0,
};

unsafe extern "C" fn ixp4xx_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;

    if !cpu_is_ixp46x() { /* includes IXP455 */
        return -ENOSYS;
    }

    let rng_base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(rng_base) {
        return ptr_err(rng_base);
    }

    ixp4xx_rng_ops.priv_data = rng_base as usize;
    devm_hwrng_register(dev, &raw mut ixp4xx_rng_ops)
}

static mut ixp4xx_rng_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"intel,ixp46x-rng\0".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut ixp4xx_rng_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"ixp4xx-hwrandom\0".as_ptr(),
        of_match_table: ixp4xx_rng_of_match.as_ptr(),
    },
    probe: Some(ixp4xx_rng_probe),
};

// module_platform_driver(ixp4xx_rng_driver);
unsafe fn ixp4xx_rng_module_init() -> i32 {
    platform_driver_register(&raw mut ixp4xx_rng_driver)
}

// MODULE_AUTHOR("Deepak Saxena <dsaxena@plexity.net>");
// MODULE_DESCRIPTION("H/W Pseudo-Random Number Generator (RNG) driver for IXP45x/46x");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
