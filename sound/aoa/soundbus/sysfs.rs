// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/kernel.h>, <linux/of.h>, <linux/stat.h>
// FIX UP: "soundbus.h"

use core::ffi::{c_char, c_int, c_void};

pub type ssize_t = isize;

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct soundbus_dev {
    pub modalias: [c_char; 0],
    pub ofdev: platform_device,
}

unsafe extern "C" {
    fn to_soundbus_device(dev: *mut device) -> *mut soundbus_dev;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn of_node_get_device_type(node: *mut c_void) -> *const c_char;

    static dev_attr_modalias: device_attribute_with_attr;
    static dev_attr_name: device_attribute_with_attr;
    static dev_attr_type: device_attribute_with_attr;
}

#[repr(C)]
pub struct device_attribute_with_attr {
    pub attr: attribute,
}

unsafe extern "C" fn modalias_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let _ = attr;
    let sdev: *mut soundbus_dev = unsafe { to_soundbus_device(dev) };
    let of: *mut platform_device = unsafe { &mut (*sdev).ofdev };

    if unsafe { *(*sdev).modalias.as_ptr() } != 0 {
        unsafe { sysfs_emit(buf, c"%s\n".as_ptr(), (*sdev).modalias.as_ptr()) }
    } else {
        unsafe {
            sysfs_emit(
                buf,
                c"of:N%pOFn%c%s\n".as_ptr(),
                (*of).dev.of_node,
                'T' as c_int,
                of_node_get_device_type((*of).dev.of_node),
            )
        }
    }
}

// static DEVICE_ATTR_RO(modalias);

unsafe extern "C" fn name_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let _ = attr;
    let sdev: *mut soundbus_dev = unsafe { to_soundbus_device(dev) };
    let of: *mut platform_device = unsafe { &mut (*sdev).ofdev };

    unsafe { sysfs_emit(buf, c"%pOFn\n".as_ptr(), (*of).dev.of_node) }
}

// static DEVICE_ATTR_RO(name);

unsafe extern "C" fn type_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let _ = attr;
    let sdev: *mut soundbus_dev = unsafe { to_soundbus_device(dev) };
    let of: *mut platform_device = unsafe { &mut (*sdev).ofdev };

    unsafe {
        sysfs_emit(
            buf,
            c"%s\n".as_ptr(),
            of_node_get_device_type((*of).dev.of_node),
        )
    }
}

// static DEVICE_ATTR_RO(type);

#[unsafe(no_mangle)]
pub static mut soundbus_dev_attrs: [*mut attribute; 4] = unsafe {
    [
        &dev_attr_name.attr as *const attribute as *mut attribute,
        &dev_attr_type.attr as *const attribute as *mut attribute,
        &dev_attr_modalias.attr as *const attribute as *mut attribute,
        core::ptr::null_mut(),
    ]
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
