// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2019-2020 Intel Corporation
 *
 * Please see Documentation/driver-api/auxiliary_bus.rst for more information.
 */

// C includes and pr_fmt are supplied by the surrounding kernel bindings.

/**
 * DOC: PURPOSE
 *
 * In some subsystems, the functionality of the core device (PCI/ACPI/other) is
 * too complex for a single device to be managed by a monolithic driver (e.g.
 * Sound Open Firmware), multiple devices might implement a common intersection
 * of functionality (e.g. NICs + RDMA), or a driver may want to export an
 * interface for another subsystem to drive (e.g. SIOV Physical Function export
 * Virtual Function management).  A split of the functionality into child-
 * devices representing sub-domains of functionality makes it possible to
 * compartmentalize, layer, and distribute domain-specific concerns via a Linux
 * device-driver model.
 *
 * An example for this kind of requirement is the audio subsystem where a
 * single IP is handling multiple entities such as HDMI, Soundwire, local
 * devices such as mics/speakers etc. The split for the core's functionality
 * can be arbitrary or be defined by the DSP firmware topology and include
 * hooks for test/debug. This allows for the audio core device to be minimal
 * and focused on hardware-specific control and communication.
 *
 * Each auxiliary_device represents a part of its parent functionality. The
 * generic behavior can be extended and specialized as needed by encapsulating
 * an auxiliary_device within other domain-specific structures and the use of
 * .ops callbacks. Devices on the auxiliary bus do not share any structures and
 * the use of a communication channel with the parent is domain-specific.
 */

// The remaining documentation in the C source describes API usage and examples;
// it is retained here as a source-level documentation marker.

unsafe fn auxiliary_match_id(
    mut id: *const auxiliary_device_id,
    auxdev: *const auxiliary_device,
) -> *const auxiliary_device_id {
    let auxdev_name = dev_name(&(*auxdev).dev);
    let p = strrchr(auxdev_name, b'.' as i32);
    let match_size: usize;

    if p.is_null() {
        return core::ptr::null();
    }
    match_size = p.offset_from(auxdev_name) as usize;

    while !(*id).name[0].eq(&0) {
        // use dev_name(&auxdev->dev) prefix before last '.' char to match to
        if strlen((*id).name.as_ptr()) == match_size
            && strncmp(auxdev_name, (*id).name.as_ptr(), match_size) == 0
        {
            return id;
        }
        id = id.add(1);
    }
    core::ptr::null()
}

unsafe extern "C" fn auxiliary_match(
    dev: *mut device,
    drv: *const device_driver,
) -> i32 {
    let auxdev = to_auxiliary_dev(dev);
    let auxdrv = to_auxiliary_drv(drv);
    (!auxiliary_match_id((*auxdrv).id_table, auxdev).is_null()) as i32
}

unsafe extern "C" fn auxiliary_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> i32 {
    let name = dev_name(dev);
    let p = strrchr(name, b'.' as i32);
    add_uevent_var(
        env,
        c"MODALIAS=%s%.*s".as_ptr(),
        AUXILIARY_MODULE_PREFIX,
        p.offset_from(name) as i32,
        name,
    )
}

unsafe extern "C" fn auxiliary_bus_probe(dev: *mut device) -> i32 {
    let auxdrv = to_auxiliary_drv((*dev).driver);
    let auxdev = to_auxiliary_dev(dev);
    let ret = dev_pm_domain_attach(dev, PD_FLAG_ATTACH_POWER_ON | PD_FLAG_DETACH_POWER_OFF);
    if ret != 0 {
        dev_warn(dev, c"Failed to attach to PM Domain : %d\n".as_ptr(), ret);
        return ret;
    }
    ((*auxdrv).probe.unwrap())(auxdev, auxiliary_match_id((*auxdrv).id_table, auxdev))
}

unsafe extern "C" fn auxiliary_bus_remove(dev: *mut device) {
    let auxdrv = to_auxiliary_drv((*dev).driver);
    let auxdev = to_auxiliary_dev(dev);
    if let Some(remove) = (*auxdrv).remove {
        remove(auxdev);
    }
}

unsafe extern "C" fn auxiliary_bus_shutdown(dev: *mut device) {
    let mut auxdrv: *const auxiliary_driver = core::ptr::null();
    let mut auxdev: *mut auxiliary_device = core::ptr::null_mut();
    if !(*dev).driver.is_null() {
        auxdrv = to_auxiliary_drv((*dev).driver);
        auxdev = to_auxiliary_dev(dev);
    }
    if !auxdrv.is_null() {
        if let Some(shutdown) = (*auxdrv).shutdown {
            shutdown(auxdev);
        }
    }
}

static auxiliary_bus_type: bus_type = bus_type {
    name: c"auxiliary".as_ptr(),
    probe: Some(auxiliary_bus_probe),
    remove: Some(auxiliary_bus_remove),
    shutdown: Some(auxiliary_bus_shutdown),
    match_: Some(auxiliary_match),
    uevent: Some(auxiliary_uevent),
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> i32 {
    let dev = &mut (*auxdev).dev;
    if dev.parent.is_null() {
        pr_err(c"auxiliary_device has a NULL dev->parent\n".as_ptr());
        return -EINVAL;
    }
    if (*auxdev).name.is_null() {
        pr_err(c"auxiliary_device has a NULL name\n".as_ptr());
        return -EINVAL;
    }
    dev.bus = &auxiliary_bus_type;
    device_initialize(&mut (*auxdev).dev);
    mutex_init(&mut (*auxdev).sysfs.lock);
    0
}

pub unsafe fn __auxiliary_device_add(auxdev: *mut auxiliary_device, modname: *const i8) -> i32 {
    let dev = &mut (*auxdev).dev;
    if modname.is_null() {
        dev_err(dev, c"auxiliary device modname is NULL\n".as_ptr());
        return -EINVAL;
    }
    let mut ret = dev_set_name(dev, c"%s.%s.%d".as_ptr(), modname, (*auxdev).name, (*auxdev).id);
    if ret != 0 {
        dev_err(dev, c"auxiliary device dev_set_name failed: %d\n".as_ptr(), ret);
        return ret;
    }
    ret = device_add(dev);
    if ret != 0 {
        dev_err(dev, c"adding auxiliary device failed!: %d\n".as_ptr(), ret);
    }
    ret
}

pub unsafe fn __auxiliary_driver_register(
    auxdrv: *mut auxiliary_driver,
    owner: *mut module,
    modname: *const i8,
) -> i32 {
    if (*auxdrv).probe.is_none() || (*auxdrv).id_table.is_null() {
        return -EINVAL;
    }
    (*auxdrv).driver.name = if !(*auxdrv).name.is_null() {
        kasprintf(GFP_KERNEL, c"%s.%s".as_ptr(), modname, (*auxdrv).name)
    } else {
        kasprintf(GFP_KERNEL, c"%s".as_ptr(), modname)
    };
    if (*auxdrv).driver.name.is_null() {
        return -ENOMEM;
    }
    (*auxdrv).driver.owner = owner;
    (*auxdrv).driver.bus = &auxiliary_bus_type;
    (*auxdrv).driver.mod_name = modname;
    let ret = driver_register(&mut (*auxdrv).driver);
    if ret != 0 {
        kfree((*auxdrv).driver.name as *mut core::ffi::c_void);
    }
    ret
}

pub unsafe fn auxiliary_driver_unregister(auxdrv: *mut auxiliary_driver) {
    driver_unregister(&mut (*auxdrv).driver);
    kfree((*auxdrv).driver.name as *mut core::ffi::c_void);
}

unsafe extern "C" fn auxiliary_device_release(dev: *mut device) {
    let auxdev = to_auxiliary_dev(dev);
    of_node_put((*dev).of_node);
    kfree(auxdev as *mut core::ffi::c_void);
}

pub unsafe fn auxiliary_device_create(
    dev: *mut device,
    modname: *const i8,
    devname: *const i8,
    platform_data: *mut core::ffi::c_void,
    id: i32,
) -> *mut auxiliary_device {
    let auxdev = kzalloc_obj::<auxiliary_device>();
    if auxdev.is_null() { return core::ptr::null_mut(); }
    (*auxdev).id = id;
    (*auxdev).name = devname;
    (*auxdev).dev.parent = dev;
    (*auxdev).dev.platform_data = platform_data;
    (*auxdev).dev.release = Some(auxiliary_device_release);
    device_set_of_node_from_dev(&mut (*auxdev).dev, dev);
    if auxiliary_device_init(auxdev) != 0 {
        of_node_put((*auxdev).dev.of_node);
        kfree(auxdev as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    if __auxiliary_device_add(auxdev, modname) != 0 {
        auxiliary_device_uninit(auxdev);
        return core::ptr::null_mut();
    }
    auxdev
}

pub unsafe fn auxiliary_device_destroy(auxdev: *mut core::ffi::c_void) {
    let auxdev = auxdev as *mut auxiliary_device;
    auxiliary_device_delete(auxdev);
    auxiliary_device_uninit(auxdev);
}

pub unsafe fn __devm_auxiliary_device_create(
    dev: *mut device,
    modname: *const i8,
    devname: *const i8,
    platform_data: *mut core::ffi::c_void,
    id: i32,
) -> *mut auxiliary_device {
    let auxdev = auxiliary_device_create(dev, modname, devname, platform_data, id);
    if auxdev.is_null() { return core::ptr::null_mut(); }
    if devm_add_action_or_reset(dev, Some(auxiliary_device_destroy), auxdev as *mut core::ffi::c_void) != 0 {
        return core::ptr::null_mut();
    }
    auxdev
}

pub unsafe fn dev_is_auxiliary(dev: *mut device) -> bool {
    (*dev).bus == &auxiliary_bus_type
}

pub unsafe fn auxiliary_bus_init() {
    WARN_ON(bus_register(&auxiliary_bus_type) != 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
