// SPDX-License-Identifier: GPL-2.0
/*
 * driver.c - centralized device driver management
 *
 * Copyright (c) 2002-3 Patrick Mochel
 * Copyright (c) 2002-3 Open Source Development Labs
 * Copyright (c) 2007 Greg Kroah-Hartman <gregkh@suse.de>
 * Copyright (c) 2007 Novell Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn next_device(i: *mut klist_iter) -> *mut device {
    let n = klist_next(i);
    let mut dev: *mut device = core::ptr::null_mut();
    let dev_prv: *mut device_private;

    if !n.is_null() {
        dev_prv = to_device_private_driver(n);
        dev = (*dev_prv).device;
    }
    dev
}

/**
 * driver_for_each_device - Iterator for devices bound to a driver.
 * @drv: Driver we're iterating.
 * @start: Device to begin with
 * @data: Data to pass to the callback.
 * @fn: Function to call for each device.
 *
 * Iterate over the @drv's list of devices calling @fn for each one.
 */
unsafe fn driver_for_each_device(
    drv: *mut device_driver,
    start: *mut device,
    data: *mut core::ffi::c_void,
    f: device_iter_t,
) -> i32 {
    let mut i: klist_iter = core::mem::zeroed();
    let mut dev: *mut device;
    let mut error: i32 = 0;

    if drv.is_null() {
        return -EINVAL;
    }

    klist_iter_init_node(
        &mut (*(*drv).p).klist_devices,
        &mut i,
        if !start.is_null() {
            &mut (*(*start).p).knode_driver
        } else {
            core::ptr::null_mut()
        },
    );
    while error == 0 {
        dev = next_device(&mut i);
        if dev.is_null() {
            break;
        }
        error = f(dev, data);
    }
    klist_iter_exit(&mut i);
    error
}

/**
 * driver_find_device - device iterator for locating a particular device.
 * @drv: The device's driver
 * @start: Device to begin with
 * @data: Data to pass to match function
 * @match: Callback function to check device
 *
 * This is similar to the driver_for_each_device() function above, but
 * it returns a reference to a device that is 'found' for later use, as
 * determined by the @match callback.
 *
 * The callback should return 0 if the device doesn't match and non-zero
 * if it does.  If the callback returns non-zero, this function will
 * return to the caller and not iterate over any more devices.
 */
unsafe fn driver_find_device(
    drv: *const device_driver,
    start: *mut device,
    data: *const core::ffi::c_void,
    r#match: device_match_t,
) -> *mut device {
    let mut i: klist_iter = core::mem::zeroed();
    let mut dev: *mut device;

    if drv.is_null() || (*drv).p.is_null() {
        return core::ptr::null_mut();
    }

    klist_iter_init_node(
        &mut (*(*drv).p).klist_devices,
        &mut i,
        if !start.is_null() {
            &mut (*(*start).p).knode_driver
        } else {
            core::ptr::null_mut()
        },
    );
    loop {
        dev = next_device(&mut i);
        if dev.is_null() {
            break;
        }
        if r#match(dev, data) != 0 {
            get_device(dev);
            break;
        }
    }
    klist_iter_exit(&mut i);
    dev
}

/**
 * driver_create_file - create sysfs file for driver.
 * @drv: driver.
 * @attr: driver attribute descriptor.
 */
unsafe fn driver_create_file(
    drv: *const device_driver,
    attr: *const driver_attribute,
) -> i32 {
    if !drv.is_null() {
        sysfs_create_file(&(*(*drv).p).kobj, &(*attr).attr)
    } else {
        -EINVAL
    }
}

/**
 * driver_remove_file - remove sysfs file for driver.
 * @drv: driver.
 * @attr: driver attribute descriptor.
 */
unsafe fn driver_remove_file(drv: *const device_driver, attr: *const driver_attribute) {
    if !drv.is_null() {
        sysfs_remove_file(&(*(*drv).p).kobj, &(*attr).attr);
    }
}

unsafe fn driver_add_groups(
    drv: *const device_driver,
    groups: *const *const attribute_group,
) -> i32 {
    sysfs_create_groups(&(*(*drv).p).kobj, groups)
}

unsafe fn driver_remove_groups(
    drv: *const device_driver,
    groups: *const *const attribute_group,
) {
    sysfs_remove_groups(&(*(*drv).p).kobj, groups);
}

/**
 * driver_register - register driver with bus
 * @drv: driver to register
 *
 * We pass off most of the work to the bus_add_driver() call,
 * since most of the things we have to do deal with the bus
 * structures.
 */
unsafe fn driver_register(drv: *mut device_driver) -> i32 {
    let mut ret: i32;
    let other: *mut device_driver;

    if !bus_is_registered((*drv).bus) {
        pr_err("Driver '%s' was unable to register with bus_type '%s' because the bus was not initialized.\n", (*drv).name, (*(*drv).bus).name);
        return -EINVAL;
    }

    if ((!(*drv).bus).probe.is_null() && !(*drv).probe.is_null())
        || ((!(*drv).bus).remove.is_null() && !(*drv).remove.is_null())
        || ((!(*drv).bus).shutdown.is_null() && !(*drv).shutdown.is_null())
    {
        pr_warn("Driver '%s' needs updating - please use bus_type methods\n", (*drv).name);
    }

    other = driver_find((*drv).name, (*drv).bus);
    if !other.is_null() {
        pr_err("Error: Driver '%s' is already registered, aborting...\n", (*drv).name);
        return -EBUSY;
    }

    ret = bus_add_driver(drv);
    if ret != 0 {
        return ret;
    }
    ret = driver_add_groups(drv, (*drv).groups);
    if ret != 0 {
        bus_remove_driver(drv);
        return ret;
    }
    kobject_uevent(&(*(*drv).p).kobj, KOBJ_ADD);
    deferred_probe_extend_timeout();
    ret
}

/**
 * driver_unregister - remove driver from system.
 * @drv: driver.
 *
 * Again, we pass off most of the work to the bus-level call.
 */
unsafe fn driver_unregister(drv: *mut device_driver) {
    if drv.is_null() || (*drv).p.is_null() {
        WARN(1, "Unexpected driver unregister!\n");
        return;
    }
    driver_remove_groups(drv, (*drv).groups);
    bus_remove_driver(drv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
