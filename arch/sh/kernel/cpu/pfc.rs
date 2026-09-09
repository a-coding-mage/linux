// SPDX-License-Identifier: GPL-2.0
/*
 * SH Pin Function Control Initialization
 *
 * Copyright (C) 2012  Renesas Solutions Corp.
 */

use core::ffi::c_char;

// Declarations supplied by the corresponding platform and CPU headers.
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: i32,
    pub num_resources: u32,
    pub resource: *mut resource,
}

unsafe extern "C" {
    fn platform_device_register(dev: *mut platform_device) -> i32;
}

static mut sh_pfc_device: platform_device = platform_device {
    name: core::ptr::null(),
    id: -1,
    num_resources: 0,
    resource: core::ptr::null_mut(),
};

pub unsafe fn sh_pfc_register(
    name: *const c_char,
    resource: *mut resource,
    num_resources: u32,
) -> i32 {
    sh_pfc_device.name = name;
    sh_pfc_device.num_resources = num_resources;
    sh_pfc_device.resource = resource;

    platform_device_register(&raw mut sh_pfc_device)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
