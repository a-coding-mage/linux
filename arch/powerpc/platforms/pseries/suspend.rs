// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010 Brian King IBM Corporation
 */

// Kernel and architecture dependencies are supplied by the surrounding Rust
// translation. The declarations below preserve the interfaces used here.

use core::ffi::{c_char, c_int, c_long, c_void};

extern "C" {
    static mut suspend_dev: device;

    fn plpar_hcall(opcode: c_ulong, retbuf: *mut c_ulong, arg: u64) -> c_long;
    fn rtas_ibm_suspend_me(arg: *mut c_void) -> c_int;
    fn capable(cap: c_int) -> bool;
    fn simple_strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn ssleep(seconds: c_uint);
    fn pm_suspend(state: c_int) -> c_int;
    fn post_mobility_fixup();
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn subsys_system_register(subsys: *const bus_type, groups: *const *const c_void) -> c_int;
    fn bus_get_dev_root(subsys: *const bus_type) -> *mut device;
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> c_int;
    fn put_device(dev: *mut device);
    fn bus_unregister(subsys: *const bus_type);
    fn firmware_has_feature(feature: c_ulong) -> bool;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;

    static dev_attr_hibernate: device_attribute;
}

type c_ulong = usize;
type c_uint = u32;
type u64 = u64;
type ssize_t = isize;
type size_t = usize;
type suspend_state_t = c_int;

#[repr(C)]
pub struct device {
    pub id: c_int,
    pub bus: *const bus_type,
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    pub name: *const c_char,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct platform_suspend_ops {
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> c_int>,
}

const H_VASI_STATE: c_ulong = 0;
const H_VASI_ENABLED: c_long = 0;
const H_VASI_SUSPENDING: c_long = 1;
const PLPAR_HCALL_BUFSIZE: usize = 4;
const CAP_SYS_ADMIN: c_int = 21;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const EPERM: c_int = 1;
const PM_SUSPEND_MEM: c_int = 3;
const FW_FEATURE_LPAR: c_ulong = 0;
const USER_DT_UPDATE: c_int = 0;
const KERN_DT_UPDATE: c_int = 1;

static mut suspend_dev: device = device { id: 0, bus: core::ptr::null() };

unsafe extern "C" fn pseries_suspend_begin(stream_id: u64) -> c_int {
    let mut vasi_state: c_long;
    let mut rc: c_long;
    let mut retbuf: [c_ulong; PLPAR_HCALL_BUFSIZE] = [0; PLPAR_HCALL_BUFSIZE];

    rc = plpar_hcall(H_VASI_STATE, retbuf.as_mut_ptr(), stream_id);
    vasi_state = retbuf[0] as c_long;

    if rc != 0 {
        // pr_err("pseries_suspend_begin: vasi_state returned %ld\n", rc);
        return rc as c_int;
    } else if vasi_state == H_VASI_ENABLED {
        return -EAGAIN;
    } else if vasi_state != H_VASI_SUSPENDING {
        // pr_err("pseries_suspend_begin: vasi_state returned state %ld\n", vasi_state);
        return -EIO;
    }
    0
}

unsafe extern "C" fn pseries_suspend_enter(_state: suspend_state_t) -> c_int {
    rtas_ibm_suspend_me(core::ptr::null_mut())
}

unsafe extern "C" fn store_hibernate(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: size_t,
) -> ssize_t {
    let stream_id: u64;
    let mut rc: c_int;

    if !capable(CAP_SYS_ADMIN) {
        return -EPERM as ssize_t;
    }

    stream_id = simple_strtoul(buf, core::ptr::null_mut(), 16) as u64;

    loop {
        rc = pseries_suspend_begin(stream_id);
        if rc != -EAGAIN {
            break;
        }
        ssleep(1);
    }

    if rc == 0 {
        rc = pm_suspend(PM_SUSPEND_MEM);
    }

    if rc == 0 {
        rc = count as c_int;
        post_mobility_fixup();
    }

    rc as ssize_t
}

unsafe extern "C" fn show_hibernate(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    // DEVICE_ATTR(hibernate, 0644, show_hibernate, store_hibernate)
    sysfs_emit(buf, c"%d\n".as_ptr(), KERN_DT_UPDATE) as ssize_t
}

// static DEVICE_ATTR(hibernate, 0644, show_hibernate, store_hibernate);

static suspend_subsys: bus_type = bus_type {
    name: c"power".as_ptr(),
    dev_name: c"power".as_ptr(),
};

static pseries_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(suspend_valid_only_mem),
    enter: Some(pseries_suspend_enter),
};

unsafe extern "C" fn pseries_suspend_sysfs_register(dev: *mut device) -> c_int {
    let dev_root: *mut device;
    let mut rc: c_int;

    rc = subsys_system_register(&suspend_subsys, core::ptr::null());
    if rc != 0 {
        return rc;
    }

    (*dev).id = 0;
    (*dev).bus = &suspend_subsys;

    dev_root = bus_get_dev_root(&suspend_subsys);
    if !dev_root.is_null() {
        rc = device_create_file(dev_root, &dev_attr_hibernate);
        put_device(dev_root);
        if rc != 0 {
            bus_unregister(&suspend_subsys);
            return rc;
        }
    }

    0
}

unsafe extern "C" fn pseries_suspend_init() -> c_int {
    let rc: c_int;

    if !firmware_has_feature(FW_FEATURE_LPAR) {
        return 0;
    }

    rc = pseries_suspend_sysfs_register(&raw mut suspend_dev);
    if rc != 0 {
        return rc;
    }

    suspend_set_ops(&pseries_suspend_ops);
    0
}

// machine_device_initcall(pseries, pseries_suspend_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
