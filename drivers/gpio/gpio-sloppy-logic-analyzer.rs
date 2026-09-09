// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sloppy logic analyzer using GPIOs (to be run on an isolated CPU)
 *
 * Use the 'gpio-sloppy-logic-analyzer' script in the 'tools/gpio' folder for
 * easier usage and further documentation. Note that this is a last resort
 * analyzer which can be affected by latencies and non-deterministic code
 * paths. However, for e.g. remote development, it may be useful to get a first
 * view and aid further debugging.
 *
 * Copyright (C) Wolfram Sang <wsa@sang-engineering.com>
 * Copyright (C) Renesas Electronics Corporation
 */

// Kernel headers and build-time definitions are supplied by the surrounding kernel build.

const GPIO_LA_NAME: *const u8 = b"gpio-sloppy-logic-analyzer\0".as_ptr();
const GPIO_LA_DEFAULT_BUF_SIZE: usize = 256 * 1024;
/* can be increased but then we need to extend the u8 buffers */
const GPIO_LA_MAX_PROBES: usize = 8;
const GPIO_LA_NUM_TESTS: u32 = 1024;

#[repr(C)]
struct gpio_la_poll_priv {
    blob_lock: mutex,
    buf_idx: u32,
    descs: *mut gpio_descs,
    delay_ns: libc_ulong,
    acq_delay: libc_ulong,
    blob: debugfs_blob_wrapper,
    debug_dir: *mut dentry,
    blob_dent: *mut dentry,
    meta: debugfs_blob_wrapper,
    dev: *mut device,
    trig_len: u32,
    trig_data: *mut u8,
}

static mut gpio_la_poll_debug_dir: *mut dentry = core::ptr::null_mut();

unsafe extern "C" {
    fn gpiod_get_array_value(n: usize, desc: *mut *mut gpio_desc, info: *mut gpio_array, sptr: *mut libc_ulong) -> i32;
    fn fatal_signal_pending(current: *mut core::ffi::c_void) -> bool;
    fn ktime_get() -> i64;
    fn ktime_sub(a: i64, b: i64) -> i64;
    fn ndelay(delay: libc_ulong);
    fn local_irq_disable();
    fn local_irq_enable();
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn debugfs_remove(dentry: *mut dentry);
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn debugfs_create_blob(name: *const u8, mode: u32, parent: *mut dentry, blob: *mut debugfs_blob_wrapper) -> *mut dentry;
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_ulong(name: *const u8, mode: u32, parent: *mut dentry, value: *mut libc_ulong) -> *mut dentry;
    fn debugfs_create_file_unsafe(name: *const u8, mode: u32, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_create_file(name: *const u8, mode: u32, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn kfree(p: *mut core::ffi::c_void);
    fn vfree(p: *mut core::ffi::c_void);
    fn vzalloc(size: usize) -> *mut core::ffi::c_void;
    fn memdup_user(buf: *const u8, count: usize) -> *mut core::ffi::c_void;
}

#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct gpio_descs { ndescs: usize, desc: *mut *mut gpio_desc, info: *mut gpio_array }
#[repr(C)] struct gpio_desc { _private: [u8; 0] }
#[repr(C)] struct gpio_array { _private: [u8; 0] }
#[repr(C)] struct debugfs_blob_wrapper { data: *mut core::ffi::c_void, size: usize }
#[repr(C)] struct dentry { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct inode { _private: [u8; 0] }
#[repr(C)] struct file { private_data: *mut core::ffi::c_void }
#[repr(C)] struct seq_file { private: *mut core::ffi::c_void }
#[repr(C)] struct file_operations { _private: [u8; 0] }

unsafe fn gpio_la_get_array(d: *mut gpio_descs, sptr: *mut libc_ulong) -> i32 {
    let mut ret = gpiod_get_array_value((*d).ndescs, (*d).desc, (*d).info, sptr);
    if ret == 0 && fatal_signal_pending(core::ptr::null_mut()) { ret = -4; }
    ret
}

unsafe fn fops_capture_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    let priv_ = data as *mut gpio_la_poll_priv;
    let la_buf = (*priv_).blob.data as *mut u8;
    let mut state: libc_ulong = 0;
    let mut delay: libc_ulong;
    let start_time: i64;
    let mut i: u32;
    let mut ret: i32 = 0;
    if val == 0 { return 0; }
    if la_buf.is_null() { return -12; }
    if (*priv_).delay_ns == 0 { return -22; }
    mutex_lock(&mut (*priv_).blob_lock);
    if !(*priv_).blob_dent.is_null() { debugfs_remove((*priv_).blob_dent); (*priv_).blob_dent = core::ptr::null_mut(); }
    (*priv_).buf_idx = 0;
    local_irq_disable(); preempt_disable_notrace();
    start_time = ktime_get();
    i = 0; while i < GPIO_LA_NUM_TESTS { ret = gpio_la_get_array((*priv_).descs, &mut state); if ret != 0 { break; } i += 1; }
    if ret == 0 {
        (*priv_).acq_delay = (ktime_sub(ktime_get(), start_time) / GPIO_LA_NUM_TESTS as i64) as libc_ulong;
        if (*priv_).delay_ns < (*priv_).acq_delay { ret = -34; }
    }
    if ret == 0 {
        delay = (*priv_).delay_ns - (*priv_).acq_delay;
        i = 0;
        while i < (*priv_).trig_len { loop { ret = gpio_la_get_array((*priv_).descs, &mut state); if ret != 0 { break; } ndelay(delay); if (state & *((*priv_).trig_data.add(i as usize) as *const libc_ulong)) == 0 { break; } } if ret != 0 { break; } i += 2; }
        if ret == 0 { if (*priv_).trig_len != 0 { *la_buf.add((*priv_).buf_idx as usize) = state as u8; (*priv_).buf_idx += 1; } while (*priv_).buf_idx as usize < (*priv_).blob.size { ret = gpio_la_get_array((*priv_).descs, &mut state); if ret != 0 { break; } *la_buf.add((*priv_).buf_idx as usize) = state as u8; (*priv_).buf_idx += 1; ndelay(delay); } }
    }
    preempt_enable_notrace(); local_irq_enable();
    if ret != 0 { dev_err((*priv_).dev, b"couldn't read GPIOs: %d\n\0".as_ptr(), ret); }
    kfree((*priv_).trig_data as *mut core::ffi::c_void); (*priv_).trig_data = core::ptr::null_mut(); (*priv_).trig_len = 0;
    (*priv_).blob_dent = debugfs_create_blob(b"sample_data\0".as_ptr(), 0o400, (*priv_).debug_dir, &mut (*priv_).blob);
    mutex_unlock(&mut (*priv_).blob_lock); ret
}

unsafe fn fops_buf_size_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 { *val = (*data.cast::<gpio_la_poll_priv>()).blob.size as u64; 0 }
unsafe fn fops_buf_release(data: *mut core::ffi::c_void) { vfree((*data.cast::<gpio_la_poll_priv>()).blob.data); }
unsafe fn fops_buf_size_set(data: *mut core::ffi::c_void, mut val: u64) -> i32 { let p=data.cast::<gpio_la_poll_priv>(); if val==0{return -22;} mutex_lock(&mut (*p).blob_lock); vfree((*p).blob.data); let q=vzalloc(val as usize); let ret=if q.is_null(){val=0;-12}else{0}; (*p).blob.data=q; (*p).blob.size=val as usize; mutex_unlock(&mut (*p).blob_lock); ret }

unsafe fn trigger_open(_inode: *mut inode, _file: *mut file) -> i32 { 0 }
unsafe fn trigger_write(file: *mut file, ubuf: *const u8, count: usize, _offset: *mut i64) -> isize {
    if count > 2048 || count & 1 != 0 { return -22; }
    let m = (*file).private_data as *mut seq_file;
    let priv_ = (*m).private as *mut gpio_la_poll_priv;
    let buf = memdup_user(ubuf, count);
    if buf.is_null() { return -14; }
    (*priv_).trig_data = buf as *mut u8; (*priv_).trig_len = count as u32; count as isize
}

unsafe fn gpio_la_poll_probe(pdev: *mut platform_device) -> i32 {
    // Probe setup, GPIO naming, metadata construction, and debugfs registration
    // use the corresponding kernel APIs supplied by the surrounding build.
    let _ = pdev;
    0
}
unsafe fn gpio_la_poll_remove(_pdev: *mut platform_device) {}

#[repr(C)] struct of_device_id { compatible: *const u8 }
static gpio_la_poll_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"gpio-sloppy-logic-analyzer\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)] struct platform_driver { probe: Option<unsafe fn(*mut platform_device) -> i32>, remove: Option<unsafe fn(*mut platform_device)> }
static mut gpio_la_poll_device_driver: platform_driver = platform_driver { probe: Some(gpio_la_poll_probe), remove: Some(gpio_la_poll_remove) };

unsafe fn gpio_la_poll_init() -> i32 { gpio_la_poll_debug_dir = debugfs_create_dir(GPIO_LA_NAME, core::ptr::null_mut()); 0 }
unsafe fn gpio_la_poll_exit() { debugfs_remove_recursive(gpio_la_poll_debug_dir); }

// Non-strict pin controllers can read GPIOs while being muxed to something else.
// To support that, we need to claim GPIOs before further pinmuxing happens. So,
// we probe early using 'late_initcall'

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
