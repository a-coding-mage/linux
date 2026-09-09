/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (c) 2021 Mellanox Technologies Ltd.  All rights reserved.
 */

// Dependency supplied by <linux/sysfs.h>.

pub struct ib_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ib_port_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(
        ibdev: *mut ib_device,
        port_num: u32,
        attr: *mut ib_port_attribute,
        buf: *mut core::ffi::c_char,
    ) -> isize>,
    pub store: Option<unsafe extern "C" fn(
        ibdev: *mut ib_device,
        port_num: u32,
        attr: *mut ib_port_attribute,
        buf: *const core::ffi::c_char,
        count: usize,
    ) -> isize>,
}

// These macros correspond to the C definitions using __ATTR_RW,
// __ATTR_RW_MODE, __ATTR_RO, and __ATTR_WO from linux/sysfs.h. Rust cannot
// concatenate identifiers in macro_rules!, so the generated object name is
// supplied explicitly as the first argument.
#[macro_export]
macro_rules! IB_PORT_ATTR_RW {
    ($object:ident, $name:ident) => {
        pub static mut $object: ib_port_attribute = ib_port_attribute {
            attr: __ATTR_RW!($name),
            show: None,
            store: None,
        };
    };
}

#[macro_export]
macro_rules! IB_PORT_ATTR_ADMIN_RW {
    ($object:ident, $name:ident) => {
        pub static mut $object: ib_port_attribute = ib_port_attribute {
            attr: __ATTR_RW_MODE!($name, 0o600),
            show: None,
            store: None,
        };
    };
}

#[macro_export]
macro_rules! IB_PORT_ATTR_RO {
    ($object:ident, $name:ident) => {
        pub static mut $object: ib_port_attribute = ib_port_attribute {
            attr: __ATTR_RO!($name),
            show: None,
            store: None,
        };
    };
}

#[macro_export]
macro_rules! IB_PORT_ATTR_WO {
    ($object:ident, $name:ident) => {
        pub static mut $object: ib_port_attribute = ib_port_attribute {
            attr: __ATTR_WO!($name),
            show: None,
            store: None,
        };
    };
}

unsafe extern "C" {
    pub fn ib_port_sysfs_get_ibdev_kobj(
        kobj: *mut kobject,
        port_num: *mut u32,
    ) -> *mut ib_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
