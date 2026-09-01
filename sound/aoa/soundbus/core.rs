// SPDX-License-Identifier: GPL-2.0-only
/*
 * soundbus
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

use core::ffi::{c_char, c_int, c_void};

// Original C dependencies:
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/of_platform.h>
// #include "soundbus.h"
//
// MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Apple Soundbus");

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const KERN_ERR: *const c_char = b"\x013\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
    pub of_node: *mut device_node,
    pub bus: *const bus_type,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub bus: *const bus_type,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct soundbus_dev {
    pub ofdev: platform_device,
    pub attach_codec: Option<unsafe extern "C" fn(*mut soundbus_dev, *mut c_void) -> c_int>,
    pub pcmname: *const c_char,
    pub pcmid: c_int,
    pub modalias: *const c_char,
}

#[repr(C)]
pub struct soundbus_driver {
    pub driver: device_driver,
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut soundbus_dev) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut soundbus_dev)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut soundbus_dev)>,
}

#[repr(C)]
pub struct kobj_uevent_env {
    pub buflen: c_int,
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub uevent: Option<unsafe extern "C" fn(*const device, *mut kobj_uevent_env) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut device)>,
    pub dev_groups: *const *const c_void,
}

unsafe extern "C" {
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn add_uevent_var(env: *mut kobj_uevent_env, format: *const c_char, ...) -> c_int;
    fn of_node_get_device_type(np: *mut device_node) -> *const c_char;
    fn of_get_property(
        np: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_char;
    fn printk(format: *const c_char, ...) -> c_int;
    fn dev_set_name(dev: *mut device, name: *const c_char, ...);
    fn of_device_register(ofdev: *mut platform_device) -> c_int;
    fn of_device_unregister(ofdev: *mut platform_device);
    fn driver_register(drv: *mut device_driver) -> c_int;
    fn driver_unregister(drv: *mut device_driver);
    fn bus_register(bus: *const bus_type) -> c_int;
    fn bus_unregister(bus: *const bus_type);

    // soundbus_dev_attrs is declared in sysfs.c; ATTRIBUTE_GROUPS(soundbus_dev)
    // creates soundbus_dev_groups in the original C source.
    static soundbus_dev_groups: *const *const c_void;
}

#[inline]
unsafe fn to_soundbus_device(dev: *mut device) -> *mut soundbus_dev {
    dev as *mut soundbus_dev
}

#[inline]
unsafe fn to_soundbus_device_const(dev: *const device) -> *const soundbus_dev {
    dev as *const soundbus_dev
}

#[inline]
unsafe fn to_soundbus_driver(drv: *mut device_driver) -> *mut soundbus_driver {
    drv as *mut soundbus_driver
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_dev_get(dev: *mut soundbus_dev) -> *mut soundbus_dev {
    let tmp: *mut device;

    if dev.is_null() {
        return core::ptr::null_mut();
    }
    tmp = unsafe { get_device(core::ptr::addr_of_mut!((*dev).ofdev.dev)) };
    if !tmp.is_null() {
        unsafe { to_soundbus_device(tmp) }
    } else {
        core::ptr::null_mut()
    }
}

// EXPORT_SYMBOL_GPL(soundbus_dev_get);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_dev_put(dev: *mut soundbus_dev) {
    if !dev.is_null() {
        unsafe { put_device(core::ptr::addr_of_mut!((*dev).ofdev.dev)) };
    }
}

// EXPORT_SYMBOL_GPL(soundbus_dev_put);

unsafe extern "C" fn soundbus_probe(dev: *mut device) -> c_int {
    let mut error: c_int = -ENODEV;
    let drv: *mut soundbus_driver;
    let soundbus_dev: *mut soundbus_dev;

    drv = unsafe { to_soundbus_driver((*dev).driver) };
    soundbus_dev = unsafe { to_soundbus_device(dev) };

    if unsafe { (*drv).probe.is_none() } {
        return error;
    }

    unsafe { soundbus_dev_get(soundbus_dev) };

    error = unsafe { ((*drv).probe.unwrap_unchecked())(soundbus_dev) };
    if error != 0 {
        unsafe { soundbus_dev_put(soundbus_dev) };
    }

    error
}

unsafe extern "C" fn soundbus_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> c_int {
    let soundbus_dev: *const soundbus_dev;
    let of: *const platform_device;
    let mut compat: *const c_char;
    let mut retval: c_int = 0;
    let mut cplen: c_int = 0;
    let mut seen: c_int = 0;

    if dev.is_null() {
        return -ENODEV;
    }

    soundbus_dev = unsafe { to_soundbus_device_const(dev) };
    if soundbus_dev.is_null() {
        return -ENODEV;
    }

    of = unsafe { core::ptr::addr_of!((*soundbus_dev).ofdev) };

    /* stuff we want to pass to /sbin/hotplug */
    retval = unsafe {
        add_uevent_var(
            env,
            c"OF_NAME=%pOFn".as_ptr(),
            (*of).dev.of_node,
        )
    };
    if retval != 0 {
        return retval;
    }

    retval = unsafe {
        add_uevent_var(
            env,
            c"OF_TYPE=%s".as_ptr(),
            of_node_get_device_type((*of).dev.of_node),
        )
    };
    if retval != 0 {
        return retval;
    }

    /* Since the compatible field can contain pretty much anything
     * it's not really legal to split it out with commas. We split it
     * up using a number of environment variables instead. */

    compat = unsafe {
        of_get_property(
            (*of).dev.of_node,
            c"compatible".as_ptr(),
            core::ptr::addr_of_mut!(cplen),
        )
    };
    while !compat.is_null() && cplen > 0 {
        let tmp: c_int = unsafe { (*env).buflen };
        retval = unsafe {
            add_uevent_var(
                env,
                c"OF_COMPATIBLE_%d=%s".as_ptr(),
                seen,
                compat,
            )
        };
        if retval != 0 {
            return retval;
        }
        let delta = unsafe { (*env).buflen - tmp };
        compat = unsafe { compat.add(delta as usize) };
        cplen -= delta;
        seen += 1;
    }

    retval = unsafe { add_uevent_var(env, c"OF_COMPATIBLE_N=%d".as_ptr(), seen) };
    if retval != 0 {
        return retval;
    }
    retval = unsafe {
        add_uevent_var(
            env,
            c"MODALIAS=%s".as_ptr(),
            (*soundbus_dev).modalias,
        )
    };

    retval
}

unsafe extern "C" fn soundbus_device_remove(dev: *mut device) {
    let soundbus_dev: *mut soundbus_dev = unsafe { to_soundbus_device(dev) };
    let drv: *mut soundbus_driver = unsafe { to_soundbus_driver((*dev).driver) };

    if unsafe { !(*dev).driver.is_null() && (*drv).remove.is_some() } {
        unsafe { ((*drv).remove.unwrap_unchecked())(soundbus_dev) };
    }
    unsafe { soundbus_dev_put(soundbus_dev) };
}

unsafe extern "C" fn soundbus_device_shutdown(dev: *mut device) {
    let soundbus_dev: *mut soundbus_dev = unsafe { to_soundbus_device(dev) };
    let drv: *mut soundbus_driver = unsafe { to_soundbus_driver((*dev).driver) };

    if unsafe { !(*dev).driver.is_null() && (*drv).shutdown.is_some() } {
        unsafe { ((*drv).shutdown.unwrap_unchecked())(soundbus_dev) };
    }
}

static soundbus_bus_type: bus_type = bus_type {
    name: c"aoa-soundbus".as_ptr(),
    probe: Some(soundbus_probe),
    uevent: Some(soundbus_uevent),
    remove: Some(soundbus_device_remove),
    shutdown: Some(soundbus_device_shutdown),
    dev_groups: unsafe { soundbus_dev_groups },
};

static mut DEVCOUNT: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_add_one(dev: *mut soundbus_dev) -> c_int {
    /* sanity checks */
    if unsafe {
        (*dev).attach_codec.is_none()
            || (*dev).ofdev.dev.of_node.is_null()
            || !(*dev).pcmname.is_null()
            || (*dev).pcmid != -1
    } {
        unsafe {
            printk(
                c"%ssoundbus: adding device failed sanity check!\n".as_ptr(),
                KERN_ERR,
            )
        };
        return -EINVAL;
    }

    unsafe {
        DEVCOUNT += 1;
        dev_set_name(
            core::ptr::addr_of_mut!((*dev).ofdev.dev),
            c"soundbus:%x".as_ptr(),
            DEVCOUNT,
        );
        (*dev).ofdev.dev.bus = core::ptr::addr_of!(soundbus_bus_type);
        of_device_register(core::ptr::addr_of_mut!((*dev).ofdev))
    }
}

// EXPORT_SYMBOL_GPL(soundbus_add_one);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_remove_one(dev: *mut soundbus_dev) {
    unsafe { of_device_unregister(core::ptr::addr_of_mut!((*dev).ofdev)) };
}

// EXPORT_SYMBOL_GPL(soundbus_remove_one);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_register_driver(drv: *mut soundbus_driver) -> c_int {
    /* initialize common driver fields */
    unsafe {
        (*drv).driver.name = (*drv).name;
        (*drv).driver.bus = core::ptr::addr_of!(soundbus_bus_type);
    }

    /* register with core */
    unsafe { driver_register(core::ptr::addr_of_mut!((*drv).driver)) }
}

// EXPORT_SYMBOL_GPL(soundbus_register_driver);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn soundbus_unregister_driver(drv: *mut soundbus_driver) {
    unsafe { driver_unregister(core::ptr::addr_of_mut!((*drv).driver)) };
}

// EXPORT_SYMBOL_GPL(soundbus_unregister_driver);

unsafe extern "C" fn soundbus_init() -> c_int {
    unsafe { bus_register(core::ptr::addr_of!(soundbus_bus_type)) }
}

unsafe extern "C" fn soundbus_exit() {
    unsafe { bus_unregister(core::ptr::addr_of!(soundbus_bus_type)) };
}

// subsys_initcall(soundbus_init);
// module_exit(soundbus_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
