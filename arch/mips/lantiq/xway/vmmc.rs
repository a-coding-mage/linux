// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2012 John Crispin <john@phrozen.org>
 */

// C dependencies supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

type dma_addr_t = usize;

const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 1;

extern "C" {
    fn panic(format: *const c_char) -> !;
    fn dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flags: c_uint,
    ) -> *mut c_void;
    fn gpiod_count(dev: *mut device, con_id: *const c_char) -> c_int;
    fn devm_gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        index: c_uint,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn ptr_err_or_zero(ptr: *mut gpio_desc) -> c_int;
    fn dev_err(dev: *mut device, format: *const c_char, ...);
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn dev_info(dev: *mut device, format: *const c_char, ...);
    fn builtin_platform_driver(driver: *mut platform_driver);
}

extern "C" {
    fn CPHYSADDR(address: *mut c_void) -> usize;
}

static mut cp1_base: *mut c_uint = core::ptr::null_mut();

pub unsafe extern "C" fn ltq_get_cp1_base() -> *mut c_uint {
    if cp1_base.is_null() {
        panic(b"no cp1 base was set\0".as_ptr() as *const c_char);
    }

    cp1_base
}

// EXPORT_SYMBOL(ltq_get_cp1_base);

unsafe extern "C" fn vmmc_probe(pdev: *mut platform_device) -> c_int {
    const CP1_SIZE: usize = 1 << 20;
    let mut gpio: *mut gpio_desc;
    let mut gpio_count: c_int;
    let mut dma: dma_addr_t = 0;
    let error: c_int;

    cp1_base = CPHYSADDR(dma_alloc_coherent(
        pdev as *mut device,
        CP1_SIZE,
        &mut dma,
        GFP_KERNEL,
    )) as *mut c_uint;

    gpio_count = gpiod_count(pdev as *mut device, core::ptr::null());
    while gpio_count > 0 {
        gpio_count -= 1;
        gpio = devm_gpiod_get_index(
            pdev as *mut device,
            core::ptr::null(),
            gpio_count as c_uint,
            GPIOD_OUT_HIGH,
        );
        error = ptr_err_or_zero(gpio);
        if error != 0 {
            dev_err(
                pdev as *mut device,
                b"failed to request GPIO idx %d: %d\n\0".as_ptr() as *const c_char,
                gpio_count,
                error,
            );
            continue;
        }

        gpiod_set_consumer_name(gpio, b"vmmc-relay\0".as_ptr() as *const c_char);
    }

    dev_info(
        pdev as *mut device,
        b"reserved %dMB at 0x%p\0".as_ptr() as *const c_char,
        CP1_SIZE >> 20,
        cp1_base,
    );

    0
}

static vmmc_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"lantiq,vmmc-xway\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut vmmc_driver: platform_driver = platform_driver {
    probe: Some(vmmc_probe),
    driver: device_driver {
        name: b"lantiq,vmmc\0".as_ptr() as *const c_char,
        of_match_table: vmmc_match.as_ptr(),
    },
};

// builtin_platform_driver(vmmc_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
