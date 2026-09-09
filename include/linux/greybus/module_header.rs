/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus Module code
 *
 * Copyright 2016 Google Inc.
 * Copyright 2016 Linaro Ltd.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/device.h

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_host_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_interface {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gb_module {
    pub dev: device,
    pub hd: *mut gb_host_device,

    pub hd_node: list_head,

    pub module_id: u8,
    pub num_interfaces: usize,

    pub disconnected: bool,

    pub interfaces: [*mut gb_interface; 0],
}

// Equivalent of: container_of(d, struct gb_module, dev)
#[macro_export]
macro_rules! to_gb_module {
    ($d:expr) => {
        unsafe {
            ($d as *mut u8).sub(core::mem::offset_of!($crate::gb_module, dev))
                as *mut $crate::gb_module
        }
    };
}

extern "C" {
    pub fn gb_module_create(
        hd: *mut gb_host_device,
        module_id: u8,
        num_interfaces: usize,
    ) -> *mut gb_module;
    pub fn gb_module_add(module: *mut gb_module) -> c_int;
    pub fn gb_module_del(module: *mut gb_module);
    pub fn gb_module_put(module: *mut gb_module);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
