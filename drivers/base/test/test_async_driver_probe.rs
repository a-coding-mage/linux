// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Google, Inc.
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// External Linux kernel types, constants, macros, and functions are supplied
// by the surrounding kernel translation environment.
use core::ffi::{c_char, c_int, c_void};

const TEST_PROBE_DELAY: c_int = 5 * 1000;
const TEST_PROBE_THRESHOLD: c_int = TEST_PROBE_DELAY / 2;

extern "C" {
    static mut warnings: atomic_t;
    static mut errors: atomic_t;
    static mut timeout: atomic_t;
    static mut async_completed: atomic_t;

    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_inc(v: *mut atomic_t);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn msleep(msecs: c_int);
    fn dev_to_node(dev: *mut device) -> c_int;
    fn numa_node_id() -> c_int;
    fn ktime_get() -> ktime_t;
    fn ktime_ms_delta( later: ktime_t, earlier: ktime_t) -> i64;
    fn cpu_to_node(cpu: c_int) -> c_int;
    fn platform_device_alloc(name: *mut c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn set_dev_node(dev: *mut device, nid: c_int);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub probe_type: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

pub type ktime_t = i64;

const PROBE_PREFER_ASYNCHRONOUS: c_int = 1;
const PROBE_FORCE_SYNCHRONOUS: c_int = 2;
const NUMA_NO_NODE: c_int = -1;
const NR_CPUS: usize = 1;
const CONFIG_NUMA: bool = false;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const EINVAL: c_int = 22;

unsafe fn test_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;

    /*
     * Determine if we have hit the "timeout" limit for the test if we
     * have then report it as an error, otherwise we wil sleep for the
     * required amount of time and then report completion.
     */
    if atomic_read(&timeout) != 0 {
        dev_err(dev, b"async probe took too long\0".as_ptr() as *const c_char);
        atomic_inc(&mut errors);
    } else {
        dev_dbg(dev, b"sleeping for %d msecs in probe\n\0".as_ptr() as *const c_char, TEST_PROBE_DELAY);
        msleep(TEST_PROBE_DELAY);
        dev_dbg(dev, b"done sleeping\n\0".as_ptr() as *const c_char);
    }

    /*
     * Report NUMA mismatch if device node is set and we are not
     * performing an async init on that node.
     */
    if (*(*dev).driver).probe_type == PROBE_PREFER_ASYNCHRONOUS {
        if CONFIG_NUMA && dev_to_node(dev) != numa_node_id() {
            dev_warn(dev, b"NUMA node mismatch %d != %d\n\0".as_ptr() as *const c_char,
                     dev_to_node(dev), numa_node_id());
            atomic_inc(&mut warnings);
        }

        atomic_inc(&mut async_completed);
    }

    0
}

static mut async_driver: platform_driver = platform_driver {
    driver: device_driver { probe_type: PROBE_PREFER_ASYNCHRONOUS },
    probe: Some(test_probe),
};

static mut sync_driver: platform_driver = platform_driver {
    driver: device_driver { probe_type: PROBE_FORCE_SYNCHRONOUS },
    probe: Some(test_probe),
};

static mut async_dev: [*mut platform_device; NR_CPUS * 2] = [core::ptr::null_mut(); NR_CPUS * 2];
static mut sync_dev: [*mut platform_device; 2] = [core::ptr::null_mut(); 2];

unsafe fn test_platform_device_register_node(name: *mut c_char, id: c_int, nid: c_int) -> *mut platform_device {
    let pdev = platform_device_alloc(name, id);
    if pdev.is_null() {
        return (-ENOMEM) as isize as *mut platform_device;
    }

    if nid != NUMA_NO_NODE {
        set_dev_node(&mut (*pdev).dev, nid);
    }

    let ret = platform_device_add(pdev);
    if ret != 0 {
        platform_device_put(pdev);
        return (ret as isize) as *mut platform_device;
    }

    pdev
}

unsafe extern "C" fn test_async_probe_init() -> c_int {
    let mut pdev: *mut *mut platform_device = core::ptr::null_mut();
    let mut async_id = 0;
    let mut sync_id = 0;
    let mut duration: u64;
    let mut calltime: ktime_t;
    let mut err: c_int = 0;
    let mut nid: c_int;
    let mut cpu: c_int = 0;

    pr_info(b"registering first set of asynchronous devices...\n\0".as_ptr() as *const c_char);
    while cpu < NR_CPUS as c_int {
        nid = cpu_to_node(cpu);
        pdev = &mut async_dev[async_id as usize];
        *pdev = test_platform_device_register_node(b"test_async_driver\0".as_ptr() as *mut c_char, async_id, nid);
        if (*pdev).is_null() {
            err = -ENOMEM;
            *pdev = core::ptr::null_mut();
            pr_err(b"failed to create async_dev: %d\n\0".as_ptr() as *const c_char, err);
            break;
        }
        async_id += 1;
        cpu += 1;
    }
    if !pdev.is_null() && (*pdev).is_null() { return err; }

    pr_info(b"registering asynchronous driver...\n\0".as_ptr() as *const c_char);
    calltime = ktime_get();
    err = platform_driver_register(&mut async_driver);
    if err != 0 { platform_driver_unregister(&mut async_driver); return err; }
    duration = ktime_ms_delta(ktime_get(), calltime) as u64;
    pr_info(b"registration took %lld msecs\n\0".as_ptr() as *const c_char, duration);
    if duration > TEST_PROBE_THRESHOLD as u64 { err = -ETIMEDOUT; platform_driver_unregister(&mut async_driver); return err; }

    if atomic_read(&async_completed) != async_id || atomic_read(&errors) != 0 || atomic_read(&warnings) != 0 {
        atomic_inc(&mut timeout);
        err = -ETIMEDOUT;
    } else { pr_info(b"completed successfully\n\0".as_ptr() as *const c_char); return 0; }

    platform_driver_unregister(&mut async_driver);
    while sync_id > 0 { sync_id -= 1; platform_device_unregister(sync_dev[sync_id as usize]); }
    while async_id > 0 { async_id -= 1; platform_device_unregister(async_dev[async_id as usize]); }
    if err == 0 { err = -EINVAL; } else { atomic_inc(&mut errors); }
    pr_err(b"Test failed with %d errors and %d warnings\n\0".as_ptr() as *const c_char, atomic_read(&errors), atomic_read(&warnings));
    err
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn test_async_probe_exit() {
    platform_driver_unregister(&mut async_driver);
    platform_driver_unregister(&mut sync_driver);
    let mut id = 2;
    while id > 0 { id -= 1; platform_device_unregister(sync_dev[id as usize]); }
    id = (NR_CPUS * 2) as c_int;
    while id > 0 { id -= 1; platform_device_unregister(async_dev[id as usize]); }
}

// module_init(test_async_probe_init);
// module_exit(test_async_probe_exit);
// MODULE_DESCRIPTION("Test module for asynchronous driver probing");
// MODULE_AUTHOR("Dmitry Torokhov <dtor@chromium.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
