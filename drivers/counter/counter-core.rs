// SPDX-License-Identifier: GPL-2.0
/*
 * Generic Counter interface
 * Copyright (C) 2020 William Breathitt Gray
 */
// Linux kernel headers and local counter headers are supplied by other files.

const COUNTER_NAME: &[u8] = b"counter\0";

/* Provides a unique ID for each counter device */
static mut COUNTER_IDA: Ida = Ida::new();

#[repr(C)]
struct CounterDeviceAllocHelper {
    counter: CounterDevice,
    // Flexible array member, aligned to ARCH_DMA_MINALIGN in the C source.
    privdata: [core::ffi::c_ulong; 0],
}

unsafe fn counter_device_release(dev: *mut Device) {
    let counter = container_of_device(dev);

    counter_chrdev_remove(counter);
    ida_free(&mut COUNTER_IDA, (*dev).id);
    kfree(container_of_counter(counter));
}

static COUNTER_DEVICE_TYPE: DeviceType = DeviceType {
    name: b"counter_device\0".as_ptr(),
    release: Some(counter_device_release),
};

static COUNTER_BUS_TYPE: BusType = BusType {
    name: b"counter\0".as_ptr(),
    dev_name: b"counter\0".as_ptr(),
};

static mut COUNTER_DEVT: DevT = DevT(0);

/**
 * counter_priv - access counter device private data
 * @counter: counter device
 *
 * Get the counter device private data
 */
pub unsafe fn counter_priv(counter: *const CounterDevice) -> *mut core::ffi::c_void {
    let ch = container_of_counter_const(counter);
    (*ch).privdata.as_ptr() as *mut core::ffi::c_void
}

/**
 * counter_alloc - allocate a counter_device
 * @sizeof_priv: size of the driver private data
 *
 * This is part one of counter registration. The structure is allocated
 * dynamically to ensure the right lifetime for the embedded struct device.
 *
 * If this succeeds, call counter_put() to get rid of the counter_device again.
 */
pub unsafe fn counter_alloc(sizeof_priv: usize) -> *mut CounterDevice {
    let ch = kzalloc(core::mem::size_of::<CounterDeviceAllocHelper>() + sizeof_priv);
    if ch.is_null() { return core::ptr::null_mut(); }

    let counter = &mut (*ch).counter as *mut CounterDevice;
    let dev = &mut (*counter).dev as *mut Device;

    // Acquire unique ID
    let err = ida_alloc(&mut COUNTER_IDA);
    if err < 0 { kfree(ch); return core::ptr::null_mut(); }
    (*dev).id = err;

    mutex_init(&mut (*counter).ops_exist_lock);
    (*dev).type_ = &COUNTER_DEVICE_TYPE;
    (*dev).bus = &COUNTER_BUS_TYPE;
    (*dev).devt = mkdev(major(COUNTER_DEVT), (*dev).id);

    let err = counter_chrdev_add(counter);
    if err < 0 {
        ida_free(&mut COUNTER_IDA, (*dev).id);
        kfree(ch);
        return core::ptr::null_mut();
    }

    device_initialize(dev);
    let err = dev_set_name(dev, COUNTER_NAME.as_ptr(), (*dev).id);
    if err != 0 {
        put_device(dev);
        return core::ptr::null_mut();
    }
    counter
}

pub unsafe fn counter_put(counter: *mut CounterDevice) {
    put_device(&mut (*counter).dev);
}

/** Complete registration of a counter. */
pub unsafe fn counter_add(counter: *mut CounterDevice) -> i32 {
    let dev = &mut (*counter).dev;
    if !(*counter).parent.is_null() {
        (*dev).parent = (*counter).parent;
        (*dev).of_node = (*(*counter).parent).of_node;
    }
    let err = counter_sysfs_add(counter);
    if err < 0 { return err; }
    cdev_device_add(&mut (*counter).chrdev, dev)
}

/** Unregister Counter from the system. */
pub unsafe fn counter_unregister(counter: *mut CounterDevice) {
    if counter.is_null() { return; }
    cdev_device_del(&mut (*counter).chrdev, &mut (*counter).dev);
    mutex_lock(&mut (*counter).ops_exist_lock);
    (*counter).ops = core::ptr::null_mut();
    wake_up(&mut (*counter).events_wait);
    mutex_unlock(&mut (*counter).ops_exist_lock);
}

unsafe fn devm_counter_release(counter: *mut core::ffi::c_void) {
    counter_unregister(counter as *mut CounterDevice);
}

unsafe fn devm_counter_put(counter: *mut core::ffi::c_void) {
    counter_put(counter as *mut CounterDevice);
}

/** Device-managed counter allocation. */
pub unsafe fn devm_counter_alloc(dev: *mut Device, sizeof_priv: usize) -> *mut CounterDevice {
    let counter = counter_alloc(sizeof_priv);
    if counter.is_null() { return core::ptr::null_mut(); }
    let err = devm_add_action_or_reset(dev, Some(devm_counter_put), counter as *mut _);
    if err < 0 { return core::ptr::null_mut(); }
    counter
}

/** Device-managed completion of counter registration. */
pub unsafe fn devm_counter_add(dev: *mut Device, counter: *mut CounterDevice) -> i32 {
    let err = counter_add(counter);
    if err < 0 { return err; }
    devm_add_action_or_reset(dev, Some(devm_counter_release), counter as *mut _)
}

const COUNTER_DEV_MAX: u32 = 256;

unsafe fn counter_init() -> i32 {
    let err = bus_register(&COUNTER_BUS_TYPE);
    if err < 0 { return err; }
    let err = alloc_chrdev_region(&mut COUNTER_DEVT, 0, COUNTER_DEV_MAX, COUNTER_NAME.as_ptr());
    if err < 0 {
        bus_unregister(&COUNTER_BUS_TYPE);
        return err;
    }
    0
}

unsafe fn counter_exit() {
    unregister_chrdev_region(COUNTER_DEVT, COUNTER_DEV_MAX);
    bus_unregister(&COUNTER_BUS_TYPE);
}

// subsys_initcall(counter_init);
// module_exit(counter_exit);
// MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
// MODULE_DESCRIPTION("Generic Counter interface");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
