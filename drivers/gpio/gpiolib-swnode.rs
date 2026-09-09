// SPDX-License-Identifier: GPL-2.0+
/*
 * Software Node helpers for the GPIO API
 *
 * Copyright 2022 Google LLC
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct gpio_device { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)]
pub struct software_node { pub name: *const c_char }
#[repr(C)]
pub struct fwnode_reference_args {
    pub fwnode: *mut fwnode_handle,
    pub args: [c_uint; 16],
}

extern "C" {
    fn to_software_node(fwnode: *const fwnode_handle) -> *const software_node;
    fn gpio_device_find_by_fwnode(fwnode: *mut fwnode_handle) -> *mut gpio_device;
    fn fwnode_property_get_reference_args(
        fwnode: *const fwnode_handle, propname: *const c_char,
        suffix: *const c_char, nargs: c_uint, index: c_uint,
        args: *mut fwnode_reference_args,
    ) -> c_int;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn gpio_device_get_desc(gdev: *mut gpio_device, hwnum: c_uint) -> *mut gpio_desc;
    fn gpio_device_put(gdev: *mut gpio_device);
    fn software_node_register(node: *const software_node) -> c_int;
    fn software_node_unregister(node: *const software_node);
}

extern "C" {
    static swnode_gpio_undefined: software_node;
}

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOTCONN: c_int = 107;
const EPROBE_DEFER: c_int = 517;

#[inline]
fn err_ptr<T>(errno: c_int) -> *mut T {
    errno.wrapping_neg() as isize as *mut T
}

unsafe fn swnode_get_gpio_device(fwnode: *mut fwnode_handle) -> *mut gpio_device {
    let gdev_node = to_software_node(fwnode);
    if !gdev_node.is_null() && gdev_node == &swnode_gpio_undefined as *const _ {
        return err_ptr(ENOENT);
    }

    let gdev = gpio_device_find_by_fwnode(fwnode);
    if gdev.is_null() { err_ptr(EPROBE_DEFER) } else { gdev }
}

unsafe fn swnode_gpio_get_reference(
    fwnode: *const fwnode_handle, propname: *const c_char,
    idx: c_uint, args: *mut fwnode_reference_args,
) -> c_int {
    fwnode_property_get_reference_args(fwnode, propname, core::ptr::null(), 2, idx, args)
}

pub unsafe fn swnode_find_gpio(
    fwnode: *mut fwnode_handle, con_id: *const c_char,
    idx: c_uint, flags: *mut c_ulong,
) -> *mut gpio_desc {
    if to_software_node(fwnode).is_null() { return err_ptr(EINVAL); }

    let mut args = fwnode_reference_args { fwnode: core::ptr::null_mut(), args: [0; 16] };
    let mut propname = [0 as c_char; 32];
    let mut ret = 0;
    // for_each_gpio_property_name(propname, con_id): macro expansion is supplied by the GPIO property API.
    let _ = con_id;
    ret = swnode_gpio_get_reference(fwnode, propname.as_ptr(), idx, &mut args);
    if ret == -ENOTCONN { return err_ptr(EPROBE_DEFER); }
    if ret != 0 { return err_ptr(-ret); }

    let gdev = swnode_get_gpio_device(args.fwnode);
    fwnode_handle_put(args.fwnode);
    if (gdev as isize) < 0 { return gdev as *mut gpio_desc; }

    *flags = args.args[1] as c_ulong;
    gpio_device_get_desc(gdev, args.args[0])
}

pub unsafe fn swnode_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> c_int {
    let mut args = fwnode_reference_args { fwnode: core::ptr::null_mut(), args: [0; 16] };
    let mut propname = [0 as c_char; 32];
    let _ = con_id;
    let mut count = 0;
    while swnode_gpio_get_reference(fwnode, propname.as_ptr(), count as c_uint, &mut args) == 0 {
        fwnode_handle_put(args.fwnode);
        count += 1;
    }
    if count != 0 { count } else { -ENOENT }
}

#[cfg(feature = "CONFIG_GPIO_SWNODE_UNDEFINED")]
pub static swnode_gpio_undefined_local: software_node = software_node {
    name: b"swnode-gpio-undefined\0".as_ptr() as *const c_char,
};

#[cfg(feature = "CONFIG_GPIO_SWNODE_UNDEFINED")]
unsafe fn swnode_gpio_init() -> c_int {
    let ret = software_node_register(&swnode_gpio_undefined);
    if ret < 0 { /* pr_err("failed to register swnode: %d\\n", ret) */ }
    ret
}

#[cfg(feature = "CONFIG_GPIO_SWNODE_UNDEFINED")]
unsafe fn swnode_gpio_cleanup() {
    software_node_unregister(&swnode_gpio_undefined);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
