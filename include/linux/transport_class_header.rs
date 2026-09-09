/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * transport_class.h - a generic container for all transport classes
 *
 * Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
 */

// C header dependencies: linux/device.h, linux/bug.h, linux/attribute_container.h

use core::ffi::c_int;

#[repr(C)]
pub struct transport_container;

#[repr(C)]
pub struct transport_class {
    pub class: class,
    pub setup: Option<unsafe extern "C" fn(*mut transport_container, *mut device, *mut device) -> c_int>,
    pub configure: Option<unsafe extern "C" fn(*mut transport_container, *mut device, *mut device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut transport_container, *mut device, *mut device) -> c_int>,
}

#[macro_export]
macro_rules! DECLARE_TRANSPORT_CLASS {
    ($cls:ident, $nm:expr, $su:expr, $rm:expr, $cfg:expr) => {
        static mut $cls: transport_class = transport_class {
            class: class { name: $nm },
            setup: $su,
            remove: $rm,
            configure: $cfg,
        };
    };
}

#[repr(C)]
pub struct anon_transport_class {
    pub tclass: transport_class,
    pub container: attribute_container,
}

#[macro_export]
macro_rules! DECLARE_ANON_TRANSPORT_CLASS {
    ($cls:ident, $mtch:expr, $cfg:expr) => {
        static mut $cls: anon_transport_class = anon_transport_class {
            tclass: transport_class {
                class: class { name: core::ptr::null() },
                setup: None,
                remove: None,
                configure: $cfg,
            },
            container: attribute_container { match_: $mtch },
        };
    };
}

#[macro_export]
macro_rules! class_to_transport_class {
    ($x:expr) => {
        container_of!($x, transport_class, class)
    };
}

#[repr(C)]
pub struct transport_container {
    pub ac: attribute_container,
    pub statistics: *const attribute_group,
    pub encryption: *const attribute_group,
}

#[macro_export]
macro_rules! attribute_container_to_transport_container {
    ($x:expr) => {
        container_of!($x, transport_container, ac)
    };
}

extern "C" {
    pub fn transport_remove_device(dev: *mut device);
    pub fn transport_add_device(dev: *mut device) -> c_int;
    pub fn transport_setup_device(dev: *mut device);
    pub fn transport_configure_device(dev: *mut device);
    pub fn transport_destroy_device(dev: *mut device);
}

#[inline]
pub unsafe fn transport_register_device(dev: *mut device) -> c_int {
    let ret: c_int;

    transport_setup_device(dev);
    ret = transport_add_device(dev);
    if ret != 0 {
        transport_destroy_device(dev);
    }

    ret
}

#[inline]
pub unsafe fn transport_unregister_device(dev: *mut device) {
    transport_remove_device(dev);
    transport_destroy_device(dev);
}

#[inline]
pub unsafe fn transport_container_register(tc: *mut transport_container) {
    attribute_container_register(&mut (*tc).ac);
}

#[inline]
pub unsafe fn transport_container_unregister(tc: *mut transport_container) {
    if unlikely(attribute_container_unregister(&mut (*tc).ac)) {
        BUG!();
    }
}

extern "C" {
    pub fn transport_class_register(tc: *mut transport_class) -> c_int;
    pub fn anon_transport_class_register(tc: *mut anon_transport_class);
    pub fn transport_class_unregister(tc: *mut transport_class);
    pub fn anon_transport_class_unregister(tc: *mut anon_transport_class);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
