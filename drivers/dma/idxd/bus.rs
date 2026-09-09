// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */

// Dependencies supplied by the Linux kernel and idxd.h are intentionally
// referenced here rather than reimplemented in this translation unit.

pub unsafe fn __idxd_driver_register(
    idxd_drv: *mut idxd_device_driver,
    owner: *mut module,
    mod_name: *const core::ffi::c_char,
) -> i32 {
    let drv: *mut device_driver = unsafe { &mut (*idxd_drv).drv };

    if unsafe { (*idxd_drv).r#type.is_null() } {
        unsafe {
            pr_debug(
                c"driver type not set (%ps)\n".as_ptr(),
                __builtin_return_address(0),
            );
        }
        return -EINVAL;
    }

    unsafe {
        (*drv).name = (*idxd_drv).name;
        (*drv).bus = &mut dsa_bus_type;
        (*drv).owner = owner;
        (*drv).mod_name = mod_name;
    }

    unsafe { driver_register(drv) }
}

// EXPORT_SYMBOL_GPL(__idxd_driver_register);

pub unsafe fn idxd_driver_unregister(idxd_drv: *mut idxd_device_driver) {
    unsafe {
        driver_unregister(&mut (*idxd_drv).drv);
    }
}

// EXPORT_SYMBOL_GPL(idxd_driver_unregister);

unsafe fn idxd_config_bus_match(
    dev: *mut device,
    drv: *const device_driver,
) -> i32 {
    let idxd_drv: *const idxd_device_driver = unsafe {
        container_of_const(drv, core::mem::offset_of!(idxd_device_driver, drv))
    };
    let idxd_dev: *mut idxd_dev = unsafe { confdev_to_idxd_dev(dev) };
    let mut i: usize = 0;

    while unsafe { (*idxd_drv).r#type.add(i).read() != IDXD_DEV_NONE } {
        if unsafe { (*idxd_dev).r#type == (*idxd_drv).r#type.add(i).read() } {
            return 1;
        }
        i += 1;
    }

    0
}

unsafe fn idxd_config_bus_probe(dev: *mut device) -> i32 {
    let idxd_drv: *mut idxd_device_driver = unsafe {
        container_of((*dev).driver, core::mem::offset_of!(idxd_device_driver, drv))
    };
    let idxd_dev: *mut idxd_dev = unsafe { confdev_to_idxd_dev(dev) };

    unsafe { ((*idxd_drv).probe)(idxd_dev) }
}

unsafe fn idxd_config_bus_remove(dev: *mut device) {
    let idxd_drv: *mut idxd_device_driver = unsafe {
        container_of((*dev).driver, core::mem::offset_of!(idxd_device_driver, drv))
    };
    let idxd_dev: *mut idxd_dev = unsafe { confdev_to_idxd_dev(dev) };

    unsafe { ((*idxd_drv).remove)(idxd_dev) };
}

unsafe fn idxd_bus_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> i32 {
    unsafe { add_uevent_var(env, c"MODALIAS=" IDXD_DEVICES_MODALIAS_FMT, 0) }
}

pub static mut dsa_bus_type: bus_type = bus_type {
    name: c"dsa".as_ptr(),
    r#match: Some(idxd_config_bus_match),
    probe: Some(idxd_config_bus_probe),
    remove: Some(idxd_config_bus_remove),
    uevent: Some(idxd_bus_uevent),
};

// EXPORT_SYMBOL_GPL(dsa_bus_type);

unsafe fn dsa_bus_init() -> i32 {
    unsafe { bus_register(&mut dsa_bus_type) }
}

// module_init(dsa_bus_init);

unsafe fn dsa_bus_exit() {
    unsafe {
        bus_unregister(&mut dsa_bus_type);
    }
}

// module_exit(dsa_bus_exit);

// MODULE_DESCRIPTION("IDXD driver dsa_bus_type driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
