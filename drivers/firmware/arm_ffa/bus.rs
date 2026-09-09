// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 ARM Ltd.
 */

// Linux dependencies supplied by the surrounding kernel/Rust environment:
// arm_ffa, device, fs, kernel, module, slab, types, and common.

const FFA_UEVENT_MODALIAS_FMT: *const u8 = b"arm_ffa:%04x:%pUb\0".as_ptr();

// DEFINE_IDA(ffa_bus_id)
extern "C" {
    static mut ffa_bus_id: ida;
}

unsafe extern "C" fn ffa_device_match(
    dev: *mut device,
    drv: *const device_driver,
) -> i32 {
    let mut id_table: *const ffa_device_id = to_ffa_driver((*drv)).id_table;
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);
    if id_table.is_null() {
        return 0;
    }

    while !uuid_is_null(&(*id_table).uuid) {
        /*
         * FF-A v1.0 doesn't provide discovery of UUIDs, just the
         * partition IDs, so match it unconditionally here and handle
         * it via the installed bus notifier during driver binding.
         */
        if uuid_is_null(&(*ffa_dev).uuid) {
            return 1;
        }

        if uuid_equal(&(*ffa_dev).uuid, &(*id_table).uuid) {
            return 1;
        }
        id_table = id_table.add(1);
    }

    0
}

unsafe extern "C" fn ffa_device_probe(dev: *mut device) -> i32 {
    let ffa_drv: *mut ffa_driver = to_ffa_driver((*dev).driver);
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);

    // UUID can be still NULL with FF-A v1.0, so just skip probing them
    if uuid_is_null(&(*ffa_dev).uuid) {
        return -ENODEV;
    }

    ((*ffa_drv).probe)(ffa_dev)
}

unsafe extern "C" fn ffa_device_remove(dev: *mut device) {
    let ffa_drv: *mut ffa_driver = to_ffa_driver((*dev).driver);

    if let Some(remove) = (*ffa_drv).remove {
        remove(to_ffa_dev(dev));
    }
}

unsafe extern "C" fn ffa_device_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> i32 {
    let ffa_dev: *const ffa_device = to_ffa_dev(dev as *mut device);

    add_uevent_var(env, FFA_UEVENT_MODALIAS_FMT, (*ffa_dev).vm_id, &(*ffa_dev).uuid)
}

unsafe extern "C" fn modalias_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> ssize_t {
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);

    sysfs_emit(buf, FFA_UEVENT_MODALIAS_FMT, (*ffa_dev).vm_id, &(*ffa_dev).uuid)
}

// DEVICE_ATTR_RO(modalias)
extern "C" {
    static dev_attr_modalias: device_attribute;
}

unsafe extern "C" fn partition_id_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> ssize_t {
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);

    sprintf(buf, b"0x%04x\n\0".as_ptr(), (*ffa_dev).vm_id)
}

// DEVICE_ATTR_RO(partition_id)
extern "C" {
    static dev_attr_partition_id: device_attribute;
}

unsafe extern "C" fn uuid_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> ssize_t {
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);

    sprintf(buf, b"%pUb\n\0".as_ptr(), &(*ffa_dev).uuid)
}

// DEVICE_ATTR_RO(uuid)
extern "C" {
    static dev_attr_uuid: device_attribute;
}

#[no_mangle]
pub static mut ffa_bus_type: bus_type = bus_type {
    name: b"arm_ffa\0".as_ptr(),
    match_: Some(ffa_device_match),
    probe: Some(ffa_device_probe),
    remove: Some(ffa_device_remove),
    uevent: Some(ffa_device_uevent),
    dev_groups: ffa_device_attributes_groups,
};

#[no_mangle]
pub unsafe extern "C" fn ffa_driver_register(
    driver: *mut ffa_driver,
    owner: *mut module,
    mod_name: *const u8,
) -> i32 {
    let ret: i32;

    if (*driver).probe.is_none() || (*driver).id_table.is_null() {
        return -EINVAL;
    }

    (*driver).driver.bus = &mut ffa_bus_type;
    (*driver).driver.name = (*driver).name;
    (*driver).driver.owner = owner;
    (*driver).driver.mod_name = mod_name;

    ret = driver_register(&mut (*driver).driver);
    if ret == 0 {
        pr_debug!("registered new ffa driver %s\n", (*driver).name);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn ffa_driver_unregister(driver: *mut ffa_driver) {
    driver_unregister(&mut (*driver).driver);
}

unsafe extern "C" fn ffa_release_device(dev: *mut device) {
    let ffa_dev: *mut ffa_device = to_ffa_dev(dev);

    ida_free(&mut ffa_bus_id, (*ffa_dev).id);
    kfree(ffa_dev);
}

unsafe extern "C" fn __ffa_devices_unregister(
    dev: *mut device,
    _data: *mut c_void,
) -> i32 {
    device_unregister(dev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ffa_devices_unregister() {
    bus_for_each_dev(
        &mut ffa_bus_type,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        Some(__ffa_devices_unregister),
    );
}

#[no_mangle]
pub unsafe extern "C" fn ffa_device_is_valid(ffa_dev: *mut ffa_device) -> bool {
    let mut valid = false;
    let mut dev: *mut device = core::ptr::null_mut();
    let mut tmp_dev: *mut ffa_device;

    loop {
        dev = bus_find_next_device(&mut ffa_bus_type, dev);
        tmp_dev = to_ffa_dev(dev);
        if tmp_dev == ffa_dev {
            valid = true;
            break;
        }
        put_device(dev);
        if dev.is_null() {
            break;
        }
    }

    put_device(dev);
    valid
}

#[no_mangle]
pub unsafe extern "C" fn ffa_device_register(
    part_info: *const ffa_partition_info,
    ops: *const ffa_ops,
    parent: *mut device,
) -> *mut ffa_device {
    let id: i32;
    let ret: i32;
    let dev: *mut device;
    let ffa_dev: *mut ffa_device;

    if part_info.is_null() {
        return core::ptr::null_mut();
    }

    id = ida_alloc_min(&mut ffa_bus_id, 1, GFP_KERNEL);
    if id < 0 {
        return core::ptr::null_mut();
    }

    ffa_dev = kzalloc_obj::<ffa_device>();
    if ffa_dev.is_null() {
        ida_free(&mut ffa_bus_id, id);
        return core::ptr::null_mut();
    }

    dev = &mut (*ffa_dev).dev;
    (*dev).parent = parent;
    (*dev).bus = &mut ffa_bus_type;
    (*dev).release = Some(ffa_release_device);
    (*dev).dma_mask = &mut (*dev).coherent_dma_mask;
    dev_set_name(dev, b"arm-ffa-%d\0".as_ptr(), id);

    (*ffa_dev).id = id;
    (*ffa_dev).vm_id = (*part_info).id;
    (*ffa_dev).properties = (*part_info).properties;
    (*ffa_dev).ops = ops;
    uuid_copy(&mut (*ffa_dev).uuid, &(*part_info).uuid);

    ret = device_register(dev);
    if ret != 0 {
        dev_err(dev, b"unable to register device %s err=%d\n\0".as_ptr(), dev_name(dev), ret);
        put_device(dev);
        return core::ptr::null_mut();
    }

    ffa_dev
}

#[no_mangle]
pub unsafe extern "C" fn ffa_device_unregister(ffa_dev: *mut ffa_device) {
    if ffa_dev.is_null() {
        return;
    }

    device_unregister(&mut (*ffa_dev).dev);
}

unsafe extern "C" fn arm_ffa_bus_init() -> i32 {
    bus_register(&mut ffa_bus_type)
}

unsafe extern "C" fn arm_ffa_bus_exit() {
    ffa_devices_unregister();
    bus_unregister(&mut ffa_bus_type);
    ida_destroy(&mut ffa_bus_id);
}

// subsys_initcall(arm_ffa_bus_init);
// module_exit(arm_ffa_bus_exit);
// MODULE_ALIAS("ffa-core");
// MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
// MODULE_DESCRIPTION("ARM FF-A bus");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
