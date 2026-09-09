// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */
// Linux kernel headers and "idxd.h" provide the dependent types and symbols.

unsafe extern "C" {
    fn device_driver_detach(dev: *mut device);
}

// Equivalent of DRIVER_ATTR_IGNORE_LOCKDEP(unbind, 0200, NULL, unbind_store).
// The generated driver-attribute object is supplied by the surrounding kernel
// compatibility layer.

unsafe extern "C" fn unbind_store(
    drv: *mut device_driver,
    buf: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let bus: *const bus_type = unsafe { (*drv).bus };
    let dev: *mut device = unsafe { bus_find_device_by_name(bus, core::ptr::null_mut(), buf) };
    let mut rc: i32 = -ENODEV;

    if dev.is_null() {
        return -ENODEV as isize;
    }

    if unsafe { !(*dev).driver.is_null() } {
        unsafe { device_driver_detach(dev) };
        rc = count as i32;
    }

    unsafe { put_device(dev) };

    rc as isize
}

// static DRIVER_ATTR_IGNORE_LOCKDEP(unbind, 0200, NULL, unbind_store);
extern "C" {
    static driver_attr_unbind: driver_attribute;
}

unsafe extern "C" fn bind_store(
    drv: *mut device_driver,
    buf: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let bus: *const bus_type = unsafe { (*drv).bus };
    let dev: *mut device = unsafe { bus_find_device_by_name(bus, core::ptr::null_mut(), buf) };
    let mut alt_drv: *mut device_driver = core::ptr::null_mut();
    let mut rc: i32 = -ENODEV;
    let idxd_dev: *mut idxd_dev;

    if dev.is_null() {
        return -ENODEV as isize;
    }

    if unsafe { !(*dev).driver.is_null() || drv != core::ptr::addr_of_mut!(dsa_drv.drv) } {
        unsafe { put_device(dev) };
        return rc as isize;
    }

    idxd_dev = unsafe { confdev_to_idxd_dev(dev) };
    if unsafe { is_idxd_dev(idxd_dev) } {
        alt_drv = unsafe { driver_find(b"idxd\0".as_ptr() as *const _, bus) };
    } else if unsafe { is_idxd_wq_dev(idxd_dev) } {
        let wq: *mut idxd_wq = unsafe { confdev_to_wq(dev) };

        if unsafe { is_idxd_wq_kernel(wq) } {
            alt_drv = unsafe { driver_find(b"dmaengine\0".as_ptr() as *const _, bus) };
        } else if unsafe { is_idxd_wq_user(wq) } {
            alt_drv = unsafe { driver_find(b"user\0".as_ptr() as *const _, bus) };
        }
    }
    if alt_drv.is_null() {
        unsafe { put_device(dev) };
        return rc as isize;
    }

    rc = unsafe { device_driver_attach(alt_drv, dev) };
    if rc < 0 {
        unsafe { put_device(dev) };
        return rc as isize;
    }

    unsafe { put_device(dev) };

    count as isize
}

// static DRIVER_ATTR_IGNORE_LOCKDEP(bind, 0200, NULL, bind_store);
extern "C" {
    static driver_attr_bind: driver_attribute;
}

static mut dsa_drv_compat_attrs: [*mut attribute; 3] = [
    unsafe { core::ptr::addr_of_mut!((*core::ptr::addr_of!(driver_attr_bind)).attr) },
    unsafe { core::ptr::addr_of_mut!((*core::ptr::addr_of!(driver_attr_unbind)).attr) },
    core::ptr::null_mut(),
];

static dsa_drv_compat_attr_group: attribute_group = attribute_group {
    attrs: unsafe { core::ptr::addr_of_mut!(dsa_drv_compat_attrs[0]) },
};

static mut dsa_drv_compat_groups: [*const attribute_group; 2] = [
    core::ptr::addr_of!(dsa_drv_compat_attr_group),
    core::ptr::null(),
];

unsafe extern "C" fn idxd_dsa_drv_probe(_idxd_dev: *mut idxd_dev) -> i32 {
    -ENODEV
}

unsafe extern "C" fn idxd_dsa_drv_remove(_idxd_dev: *mut idxd_dev) {}

static mut dev_types: [idxd_dev_type; 1] = [IDXD_DEV_NONE];

#[no_mangle]
pub static mut dsa_drv: idxd_device_driver = idxd_device_driver {
    name: b"dsa\0".as_ptr() as *const _,
    probe: Some(idxd_dsa_drv_probe),
    remove: Some(idxd_dsa_drv_remove),
    type_: unsafe { core::ptr::addr_of_mut!(dev_types[0]) },
    drv: device_driver {
        suppress_bind_attrs: true,
        groups: unsafe { core::ptr::addr_of!(dsa_drv_compat_groups[0]) },
    },
};

// module_idxd_driver(dsa_drv);
// MODULE_IMPORT_NS("IDXD");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
