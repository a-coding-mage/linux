// SPDX-License-Identifier: GPL-2.0-only
/*
 * System Specific setup for Traverse Technologies GEOS.
 * At the moment this means setup of GPIO control of LEDs.
 *
 * Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
 * Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
 *                and Philip Prindeville <philipp@redfish-solutions.com>
 */

// External declarations supplied by the Linux kernel and geode-common.h.
#[repr(C)]
pub struct geode_led {
    pub gpio: ::core::ffi::c_int,
    pub active_low: bool,
}

extern "C" {
    fn geode_create_restart_key(gpio: ::core::ffi::c_int);
    fn geode_create_leds(
        name: *const ::core::ffi::c_char,
        leds: *const geode_led,
        count: usize,
    );
    fn is_geode() -> bool;
    fn dmi_get_system_info(field: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn strcmp(a: *const ::core::ffi::c_char, b: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn printk(fmt: *const ::core::ffi::c_char, ...);
}

// Values supplied by the Linux DMI and logging interfaces.
extern "C" {
    static DMI_SYS_VENDOR: ::core::ffi::c_int;
    static DMI_PRODUCT_NAME: ::core::ffi::c_int;
    static KBUILD_MODNAME: ::core::ffi::c_char;
}

static GEOS_LEDS: [geode_led; 3] = [
    geode_led { gpio: 6, active_low: true },
    geode_led { gpio: 25, active_low: false },
    geode_led { gpio: 27, active_low: false },
];

unsafe fn register_geos() {
    geode_create_restart_key(3);
    geode_create_leds(b"geos\0".as_ptr() as *const ::core::ffi::c_char,
                      GEOS_LEDS.as_ptr(),
                      GEOS_LEDS.len());
}

unsafe fn geos_init() -> ::core::ffi::c_int {
    let vendor: *const ::core::ffi::c_char;
    let product: *const ::core::ffi::c_char;

    if !is_geode() {
        return 0;
    }

    vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    if vendor.is_null()
        || strcmp(vendor, b"Traverse Technologies\0".as_ptr() as *const ::core::ffi::c_char) != 0
    {
        return 0;
    }

    product = dmi_get_system_info(DMI_PRODUCT_NAME);
    if product.is_null()
        || strcmp(product, b"Geos\0".as_ptr() as *const ::core::ffi::c_char) != 0
    {
        return 0;
    }

    printk(b"%s: system is recognized as \"%s %s\"\n\0".as_ptr() as *const ::core::ffi::c_char,
           &KBUILD_MODNAME,
           vendor,
           product);

    register_geos();

    0
}

// Equivalent to the kernel's device_initcall(geos_init) registration macro.
// device_initcall!(geos_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
