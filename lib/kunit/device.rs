// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit-managed device implementation
 *
 * Implementation of struct kunit_device helpers for fake devices whose
 * lifecycle is managed by KUnit.
 *
 * Copyright (C) 2023, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// Kernel and KUnit dependencies supplied by other translation units.

/* Wrappers for use with kunit_add_action(). */
extern "C" {
    fn device_unregister_wrapper(data: *mut core::ffi::c_void);
    fn driver_unregister_wrapper(data: *mut core::ffi::c_void);
}

/* The root device for the KUnit bus, parent of all kunit_devices. */
static mut kunit_bus_device: *mut device = core::ptr::null_mut();

/* A device owned by a KUnit test. */
#[repr(C)]
struct kunit_device {
    dev: device,
    /* The KUnit test which owns this device. */
    owner: *mut kunit,
    /* If the driver is managed by KUnit and unique to this device. */
    driver: *const device_driver,
}

#[inline]
unsafe fn to_kunit_device(d: *mut device) -> *mut kunit_device {
    (d as *mut u8).sub(core::mem::offset_of!(kunit_device, dev)) as *mut kunit_device
}

static kunit_bus_type: bus_type = bus_type {
    name: b"kunit\0".as_ptr() as *const core::ffi::c_char,
};

/* Register the 'kunit_bus' used for fake devices. */
#[no_mangle]
pub unsafe extern "C" fn kunit_bus_init() -> i32 {
    let mut error: i32;

    kunit_bus_device = root_device_register(b"kunit\0".as_ptr() as *const core::ffi::c_char);
    if is_err(kunit_bus_device) {
        return ptr_err(kunit_bus_device);
    }

    error = bus_register(&kunit_bus_type);
    if error != 0 {
        root_device_unregister(kunit_bus_device);
    }
    error
}

/* Unregister the 'kunit_bus' in case the KUnit module is unloaded. */
#[no_mangle]
pub unsafe extern "C" fn kunit_bus_shutdown() {
    /* Make sure the bus exists before we unregister it. */
    if is_err_or_null(kunit_bus_device) {
        return;
    }

    bus_unregister(&kunit_bus_type);
    root_device_unregister(kunit_bus_device);
    kunit_bus_device = core::ptr::null_mut();
}

/* Release a 'fake' KUnit device. */
unsafe extern "C" fn kunit_device_release(d: *mut device) {
    kfree(to_kunit_device(d));
}

/*
 * Create and register a KUnit-managed struct device_driver on the kunit_bus.
 * Returns an error pointer on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_driver_create(test: *mut kunit, name: *const core::ffi::c_char) -> *mut device_driver {
    let mut driver: *mut device_driver;
    let mut err: i32 = -12;

    driver = kunit_kzalloc(test, core::mem::size_of::<device_driver>(), 0) as *mut device_driver;
    if driver.is_null() {
        return err_ptr(err);
    }

    (*driver).name = kunit_kstrdup_const(test, name, 0);
    (*driver).bus = &kunit_bus_type;
    (*driver).owner = THIS_MODULE;

    err = driver_register(driver);
    if err != 0 {
        kunit_kfree(test, driver as *mut core::ffi::c_void);
        return err_ptr(err);
    }

    kunit_add_action(test, driver_unregister_wrapper, driver as *mut core::ffi::c_void);
    driver
}

/* Helper which creates a kunit_device, attaches it to the kunit_bus. */
unsafe fn kunit_device_register_internal(test: *mut kunit, name: *const core::ffi::c_char) -> *mut kunit_device {
    let kunit_dev = kzalloc(core::mem::size_of::<kunit_device>(), 0) as *mut kunit_device;
    let mut err: i32 = -12;
    if kunit_dev.is_null() {
        return err_ptr(err);
    }

    (*kunit_dev).owner = test;
    err = dev_set_name(&mut (*kunit_dev).dev, b"%s.%s\0".as_ptr() as *const core::ffi::c_char, (*test).name, name);
    if err != 0 {
        kfree(kunit_dev as *mut core::ffi::c_void);
        return err_ptr(err);
    }

    (*kunit_dev).dev.release = Some(kunit_device_release);
    (*kunit_dev).dev.bus = &kunit_bus_type;
    (*kunit_dev).dev.parent = kunit_bus_device;

    err = device_register(&mut (*kunit_dev).dev);
    if err != 0 {
        put_device(&mut (*kunit_dev).dev);
        return err_ptr(err);
    }

    (*kunit_dev).dev.dma_mask = &mut (*kunit_dev).dev.coherent_dma_mask;
    (*kunit_dev).dev.coherent_dma_mask = dma_bit_mask(32);
    kunit_add_action(test, device_unregister_wrapper, &mut (*kunit_dev).dev as *mut device as *mut core::ffi::c_void);
    kunit_dev
}

/*
 * Create and register a new KUnit-managed device, using the user-supplied device_driver.
 * On failure, returns an error pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_device_register_with_driver(test: *mut kunit, name: *const core::ffi::c_char, _drv: *const device_driver) -> *mut device {
    let kunit_dev = kunit_device_register_internal(test, name);
    if is_err_or_null(kunit_dev as *mut device) {
        return kunit_dev as *mut device;
    }
    &mut (*kunit_dev).dev
}

/*
 * Create and register a new KUnit-managed device, including a matching device_driver.
 * On failure, returns an error pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn kunit_device_register(test: *mut kunit, name: *const core::ffi::c_char) -> *mut device {
    let drv = kunit_driver_create(test, name);
    if is_err(drv) {
        return drv as *mut device;
    }

    let dev = kunit_device_register_internal(test, name);
    if is_err(dev as *mut device) {
        kunit_release_action(test, driver_unregister_wrapper, drv as *mut core::ffi::c_void);
        return dev as *mut device;
    }

    /* Request the driver be freed. */
    (*dev).driver = drv;
    &mut (*dev).dev
}

/* Unregisters a KUnit-managed device early (including the driver, if automatically created). */
#[no_mangle]
pub unsafe extern "C" fn kunit_device_unregister(test: *mut kunit, dev: *mut device) {
    let driver = (*to_kunit_device(dev)).driver;

    kunit_release_action(test, device_unregister_wrapper, dev as *mut core::ffi::c_void);
    if !driver.is_null() {
        let driver_name = (*driver).name;
        kunit_release_action(test, driver_unregister_wrapper, driver as *mut core::ffi::c_void);
        kunit_kfree_const(test, driver_name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
