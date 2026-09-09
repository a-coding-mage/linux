// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2025 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
 * Copyright (c) 2025 The Linux Foundation
 *
 * A "simple" faux bus that allows devices to be created and added
 * automatically to it.  This is to be used whenever you need to create a
 * device that is not associated with any "real" system resources, and
 * do not want to have to deal with a bus/driver binding logic.  It is
 * intended to be very simple, with only a create and a destroy function
 * available.
 */

// Dependencies are supplied by the surrounding kernel Rust environment.

#[repr(C)]
struct faux_object {
    faux_dev: faux_device,
    faux_ops: *const faux_device_ops,
    groups: *const *const attribute_group,
}

unsafe fn to_faux_object(dev: *mut device) -> *mut faux_object {
    // faux_device embeds struct device as its first member.
    dev as *mut faux_object
}

static mut faux_bus_root: *mut device = core::ptr::null_mut();

unsafe fn faux_match(_dev: *mut device, _drv: *const device_driver) -> i32 {
    /* Match always succeeds, we only have one driver */
    1
}

unsafe fn faux_probe(dev: *mut device) -> i32 {
    let faux_obj = &mut *to_faux_object(dev);
    let faux_dev = &mut faux_obj.faux_dev;
    let faux_ops = faux_obj.faux_ops;
    let mut ret: i32;

    if !faux_ops.is_null() && !(*faux_ops).probe.is_none() {
        ret = ((*faux_ops).probe.unwrap())(faux_dev as *mut faux_device);
        if ret != 0 {
            return ret;
        }
    }

    /*
     * Add groups after the probe succeeds to ensure resources are
     * initialized correctly
     */
    ret = device_add_groups(dev, faux_obj.groups);
    if ret != 0 && !faux_ops.is_null() && !(*faux_ops).remove.is_none() {
        ((*faux_ops).remove.unwrap())(faux_dev as *mut faux_device);
    }

    ret
}

unsafe fn faux_remove(dev: *mut device) {
    let faux_obj = &mut *to_faux_object(dev);
    let faux_dev = &mut faux_obj.faux_dev;
    let faux_ops = faux_obj.faux_ops;

    device_remove_groups(dev, faux_obj.groups);

    if !faux_ops.is_null() && !(*faux_ops).remove.is_none() {
        ((*faux_ops).remove.unwrap())(faux_dev as *mut faux_device);
    }
}

static faux_bus_type: bus_type = bus_type {
    name: "faux\0".as_ptr() as *const _,
    match_: Some(faux_match),
    probe: Some(faux_probe),
    remove: Some(faux_remove),
};

static mut faux_driver: device_driver = device_driver {
    name: "faux_driver\0".as_ptr() as *const _,
    bus: &faux_bus_type,
    probe_type: PROBE_FORCE_SYNCHRONOUS,
    suppress_bind_attrs: true,
};

unsafe fn faux_device_release(dev: *mut device) {
    let faux_obj = to_faux_object(dev);
    kfree(faux_obj as *mut core::ffi::c_void);
}

/**
 * faux_device_create_with_groups - Create and register with the driver
 *	core a faux device and populate the device with an initial
 *	set of sysfs attributes.
 */
#[no_mangle]
pub unsafe extern "C" fn faux_device_create_with_groups(
    name: *const core::ffi::c_char,
    parent: *mut device,
    faux_ops: *const faux_device_ops,
    groups: *const *const attribute_group,
) -> *mut faux_device {
    if faux_bus_root.is_null() {
        return core::ptr::null_mut();
    }

    let faux_obj = kzalloc(core::mem::size_of::<faux_object>()) as *mut faux_object;
    if faux_obj.is_null() {
        return core::ptr::null_mut();
    }

    /* Save off the callbacks and groups so we can use them in the future */
    (*faux_obj).faux_ops = faux_ops;
    (*faux_obj).groups = groups;

    /* Initialize the device portion and register it with the driver core */
    let faux_dev = &mut (*faux_obj).faux_dev as *mut faux_device;
    let dev = faux_dev as *mut device;

    device_initialize(dev);
    (*dev).release = Some(faux_device_release);
    (*dev).parent = if !parent.is_null() { parent } else { faux_bus_root };
    (*dev).bus = &faux_bus_type;
    dev_set_name(dev, "%s\0".as_ptr() as *const _, name);
    device_set_pm_not_required(dev);

    let ret = device_add(dev);
    if ret != 0 {
        pr_err("%s: device_add for faux device '%s' failed with %d\n\0", name, ret);
        put_device(dev);
        return core::ptr::null_mut();
    }

    /*
     * Verify that we did bind the driver to the device (i.e. probe worked),
     * if not, let's fail the creation as trying to guess if probe was
     * successful is almost impossible to determine by the caller.
     */
    if (*dev).driver.is_null() {
        dev_dbg(dev, "probe did not succeed, tearing down the device\n\0");
        faux_device_destroy(faux_dev);
        return core::ptr::null_mut();
    }

    faux_dev
}

#[no_mangle]
pub unsafe extern "C" fn faux_device_create(
    name: *const core::ffi::c_char,
    parent: *mut device,
    faux_ops: *const faux_device_ops,
) -> *mut faux_device {
    faux_device_create_with_groups(name, parent, faux_ops, core::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn faux_device_destroy(faux_dev: *mut faux_device) {
    if faux_dev.is_null() {
        return;
    }

    let dev = faux_dev as *mut device;
    device_del(dev);
    /* The final put_device() will clean up the memory we allocated for this device. */
    put_device(dev);
}

pub unsafe extern "C" fn faux_bus_init() -> i32 {
    let root = root_device_register("faux\0".as_ptr() as *const _);
    if is_err(root) {
        return ptr_err(root);
    }

    let mut ret = bus_register(&faux_bus_type);
    if ret != 0 {
        root_device_unregister(root);
        return ret;
    }

    ret = driver_register(&mut faux_driver);
    if ret != 0 {
        bus_unregister(&faux_bus_type);
        root_device_unregister(root);
        return ret;
    }

    faux_bus_root = root;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
