// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPIC timer wakeup driver
 *
 * Copyright 2013 Freescale Semiconductor, Inc.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct mpic_timer {
    pub irq: c_int,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct subsystem {
    _private: [u8; 0],
}

type time64_t = i64;
type ssize_t = isize;
type irqreturn_t = c_int;

const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static mut sysfs_lock: mutex;
    static mut mpic_subsys: subsystem;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn disable_irq_wake(irq: c_int) -> c_int;
    fn enable_irq_wake(irq: c_int) -> c_int;
    fn mpic_free_timer(timer: *mut mpic_timer);
    fn mpic_get_remain_time(timer: *mut mpic_timer, interval: *mut time64_t);
    fn mpic_request_timer(
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        dev_id: *mut c_void,
        interval: time64_t,
    ) -> *mut mpic_timer;
    fn mpic_start_timer(timer: *mut mpic_timer);
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn kstrtoll(buf: *const c_char, base: c_int, value: *mut time64_t) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn bus_get_dev_root(subsys: *mut subsystem) -> *mut device;
    fn device_create_file(dev: *mut device, attr: *mut device_attribute) -> c_int;
    fn device_remove_file(dev: *mut device, attr: *mut device_attribute);
    fn put_device(dev: *mut device);
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn init_work(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
}

#[repr(C)]
struct fsl_mpic_timer_wakeup {
    timer: *mut mpic_timer,
    free_work: work_struct,
}

static mut fsl_wakeup: *mut fsl_mpic_timer_wakeup = core::ptr::null_mut();

unsafe extern "C" fn fsl_free_resource(ws: *mut work_struct) {
    // free_work is the first field after timer; recover the containing object.
    let wakeup = (ws as *mut u8).sub(core::mem::offset_of!(fsl_mpic_timer_wakeup, free_work))
        as *mut fsl_mpic_timer_wakeup;

    mutex_lock(&raw mut sysfs_lock);

    if !(*wakeup).timer.is_null() {
        disable_irq_wake((*(*wakeup).timer).irq);
        mpic_free_timer((*wakeup).timer);
    }

    (*wakeup).timer = core::ptr::null_mut();
    mutex_unlock(&raw mut sysfs_lock);
}

unsafe extern "C" fn fsl_mpic_timer_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let wakeup = dev_id as *mut fsl_mpic_timer_wakeup;

    schedule_work(&mut (*wakeup).free_work);

    if !(*wakeup).timer.is_null() { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe extern "C" fn fsl_timer_wakeup_show(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut interval: time64_t = 0;

    mutex_lock(&raw mut sysfs_lock);
    if !(*fsl_wakeup).timer.is_null() {
        mpic_get_remain_time((*fsl_wakeup).timer, &mut interval);
        interval += 1;
    }
    mutex_unlock(&raw mut sysfs_lock);

    sysfs_emit(buf, b"%lld\0".as_ptr() as *const c_char, interval)
}

unsafe extern "C" fn fsl_timer_wakeup_store(
    _dev: *mut device,
    _attr: *mut device_attribute,
    buf: *const c_char,
    count: usize,
) -> ssize_t {
    let mut interval: time64_t = 0;
    let mut ret: c_int;

    if kstrtoll(buf, 0, &mut interval) != 0 { return -EINVAL as ssize_t; }

    mutex_lock(&raw mut sysfs_lock);

    if !(*fsl_wakeup).timer.is_null() {
        disable_irq_wake((*(*fsl_wakeup).timer).irq);
        mpic_free_timer((*fsl_wakeup).timer);
        (*fsl_wakeup).timer = core::ptr::null_mut();
    }

    if interval == 0 {
        mutex_unlock(&raw mut sysfs_lock);
        return count as ssize_t;
    }

    (*fsl_wakeup).timer = mpic_request_timer(
        fsl_mpic_timer_irq,
        fsl_wakeup as *mut c_void,
        interval,
    );
    if (*fsl_wakeup).timer.is_null() {
        mutex_unlock(&raw mut sysfs_lock);
        return -EINVAL as ssize_t;
    }

    ret = enable_irq_wake((*(*fsl_wakeup).timer).irq);
    if ret != 0 {
        mpic_free_timer((*fsl_wakeup).timer);
        (*fsl_wakeup).timer = core::ptr::null_mut();
        mutex_unlock(&raw mut sysfs_lock);
        return ret as ssize_t;
    }

    mpic_start_timer((*fsl_wakeup).timer);
    mutex_unlock(&raw mut sysfs_lock);
    count as ssize_t
}

static mut mpic_attributes: device_attribute = device_attribute { _private: [] };

unsafe extern "C" fn fsl_wakeup_sys_init() -> c_int {
    let mut dev_root: *mut device;
    let mut ret: c_int = -EINVAL;

    fsl_wakeup = kzalloc(core::mem::size_of::<fsl_mpic_timer_wakeup>())
        as *mut fsl_mpic_timer_wakeup;
    if fsl_wakeup.is_null() { return -ENOMEM; }

    init_work(&mut (*fsl_wakeup).free_work, fsl_free_resource);

    dev_root = bus_get_dev_root(&raw mut mpic_subsys);
    if !dev_root.is_null() {
        ret = device_create_file(dev_root, &raw mut mpic_attributes);
        put_device(dev_root);
        if ret != 0 { kfree(fsl_wakeup as *mut c_void); }
    }

    ret
}

unsafe extern "C" fn fsl_wakeup_sys_exit() {
    let dev_root = bus_get_dev_root(&raw mut mpic_subsys);
    if !dev_root.is_null() {
        device_remove_file(dev_root, &raw mut mpic_attributes);
        put_device(dev_root);
    }

    mutex_lock(&raw mut sysfs_lock);
    if !(*fsl_wakeup).timer.is_null() {
        disable_irq_wake((*(*fsl_wakeup).timer).irq);
        mpic_free_timer((*fsl_wakeup).timer);
    }
    kfree(fsl_wakeup as *mut c_void);
    mutex_unlock(&raw mut sysfs_lock);
}

// module_init(fsl_wakeup_sys_init);
// module_exit(fsl_wakeup_sys_exit);
// MODULE_DESCRIPTION("Freescale MPIC global timer wakeup driver");
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Wang Dongsheng <dongsheng.wang@freescale.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
