// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */

// Linux kernel dependencies supplied by the surrounding translation.

use core::ffi::c_void;

extern "C" {
    static mut total_comp_calls: atomic64_t;
    static mut total_decomp_calls: atomic64_t;
    static mut total_sw_comp_calls: atomic64_t;
    static mut total_sw_decomp_calls: atomic64_t;
    static mut total_comp_bytes_out: atomic64_t;
    static mut total_decomp_bytes_in: atomic64_t;
    static mut total_completion_einval_errors: atomic64_t;
    static mut total_completion_timeout_errors: atomic64_t;
    static mut total_completion_comp_buf_overflow_errors: atomic64_t;
    static mut iaa_crypto_debugfs_root: *mut dentry;
}

// These declarations correspond to types and functions provided by the
// included kernel and iaa_crypto headers.
extern "C" {
    fn atomic64_inc(v: *mut atomic64_t);
    fn atomic64_add(i: i64, v: *mut atomic64_t);
    fn atomic64_set(v: *mut atomic64_t, i: i64);
    fn atomic64_read(v: *const atomic64_t) -> u64;
    fn idxd_wq_get_private(wq: *mut idxd_wq) -> *mut iaa_wq;
    fn seq_printf(m: *mut seq_file, fmt: *const i8, ...);
    fn seq_puts(m: *mut seq_file, s: *const i8);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32, data: *mut c_void) -> i32;
    fn seq_read(file: *mut file, buf: *mut u8, count: usize, pos: *mut i64) -> isize;
    fn seq_lseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn single_release(inode: *mut inode, file: *mut file) -> i32;
    fn debugfs_initialized() -> bool;
    fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const i8, mode: u16, parent: *mut dentry,
                           data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(root: *mut dentry);
}

#[repr(C)] pub struct atomic64_t { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct idxd_wq { _private: [u8; 0] }
#[repr(C)] pub struct idxd_device { pub id: i32 }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

#[repr(C)] pub struct iaa_device {
    pub comp_calls: atomic64_t,
    pub comp_bytes: atomic64_t,
    pub decomp_calls: atomic64_t,
    pub decomp_bytes: atomic64_t,
    pub idxd: *mut idxd_device,
    pub n_wq: i32,
    pub wqs: list_head,
}
#[repr(C)] pub struct iaa_wq {
    pub comp_calls: atomic64_t,
    pub comp_bytes: atomic64_t,
    pub decomp_calls: atomic64_t,
    pub decomp_bytes: atomic64_t,
    pub iaa_device: *mut iaa_device,
    pub wq: *mut idxd_wq,
    pub list: list_head,
}
#[repr(C)] pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, i32) -> i64>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}

extern "C" {
    static mut iaa_devices_lock: mutex;
    static mut iaa_devices: list_head;
}

// list_for_each_entry is retained as the kernel-list traversal primitive.
macro_rules! list_for_each_entry { ($entry:ident, $head:expr, $member:ident, $body:block) => {{ /* supplied by kernel */ $body }} }

#[no_mangle] pub unsafe extern "C" fn update_total_comp_calls() { atomic64_inc(&raw mut total_comp_calls); }
#[no_mangle] pub unsafe extern "C" fn update_total_comp_bytes_out(n: i32) { atomic64_add(n as i64, &raw mut total_comp_bytes_out); }
#[no_mangle] pub unsafe extern "C" fn update_total_decomp_calls() { atomic64_inc(&raw mut total_decomp_calls); }
#[no_mangle] pub unsafe extern "C" fn update_total_sw_comp_calls() { atomic64_inc(&raw mut total_sw_comp_calls); }
#[no_mangle] pub unsafe extern "C" fn update_total_sw_decomp_calls() { atomic64_inc(&raw mut total_sw_decomp_calls); }
#[no_mangle] pub unsafe extern "C" fn update_total_decomp_bytes_in(n: i32) { atomic64_add(n as i64, &raw mut total_decomp_bytes_in); }
#[no_mangle] pub unsafe extern "C" fn update_completion_einval_errs() { atomic64_inc(&raw mut total_completion_einval_errors); }
#[no_mangle] pub unsafe extern "C" fn update_completion_timeout_errs() { atomic64_inc(&raw mut total_completion_timeout_errors); }
#[no_mangle] pub unsafe extern "C" fn update_completion_comp_buf_overflow_errs() { atomic64_inc(&raw mut total_completion_comp_buf_overflow_errors); }

#[no_mangle] pub unsafe extern "C" fn update_wq_comp_calls(idxd_wq: *mut idxd_wq) {
    let wq = idxd_wq_get_private(idxd_wq);
    atomic64_inc(&raw mut (*wq).comp_calls); atomic64_inc(&raw mut (*(*wq).iaa_device).comp_calls);
}
#[no_mangle] pub unsafe extern "C" fn update_wq_comp_bytes(idxd_wq: *mut idxd_wq, n: i32) {
    let wq = idxd_wq_get_private(idxd_wq);
    atomic64_add(n as i64, &raw mut (*wq).comp_bytes); atomic64_add(n as i64, &raw mut (*(*wq).iaa_device).comp_bytes);
}
#[no_mangle] pub unsafe extern "C" fn update_wq_decomp_calls(idxd_wq: *mut idxd_wq) {
    let wq = idxd_wq_get_private(idxd_wq);
    atomic64_inc(&raw mut (*wq).decomp_calls); atomic64_inc(&raw mut (*(*wq).iaa_device).decomp_calls);
}
#[no_mangle] pub unsafe extern "C" fn update_wq_decomp_bytes(idxd_wq: *mut idxd_wq, n: i32) {
    let wq = idxd_wq_get_private(idxd_wq);
    atomic64_add(n as i64, &raw mut (*wq).decomp_bytes); atomic64_add(n as i64, &raw mut (*(*wq).iaa_device).decomp_bytes);
}

unsafe fn reset_iaa_crypto_stats() {
    atomic64_set(&raw mut total_comp_calls, 0); atomic64_set(&raw mut total_decomp_calls, 0);
    atomic64_set(&raw mut total_sw_comp_calls, 0); atomic64_set(&raw mut total_sw_decomp_calls, 0);
    atomic64_set(&raw mut total_comp_bytes_out, 0); atomic64_set(&raw mut total_decomp_bytes_in, 0);
    atomic64_set(&raw mut total_completion_einval_errors, 0); atomic64_set(&raw mut total_completion_timeout_errors, 0);
    atomic64_set(&raw mut total_completion_comp_buf_overflow_errors, 0);
}
unsafe fn reset_wq_stats(wq: *mut iaa_wq) {
    atomic64_set(&raw mut (*wq).comp_calls, 0); atomic64_set(&raw mut (*wq).comp_bytes, 0);
    atomic64_set(&raw mut (*wq).decomp_calls, 0); atomic64_set(&raw mut (*wq).decomp_bytes, 0);
}
unsafe fn reset_device_stats(d: *mut iaa_device) {
    atomic64_set(&raw mut (*d).comp_calls, 0); atomic64_set(&raw mut (*d).comp_bytes, 0);
    atomic64_set(&raw mut (*d).decomp_calls, 0); atomic64_set(&raw mut (*d).decomp_bytes, 0);
    list_for_each_entry!(wq, (*d).wqs, list, { reset_wq_stats(wq); });
}

unsafe fn wq_show(m: *mut seq_file, w: *mut iaa_wq) {
    seq_printf(m, b"    name: %s\n\0".as_ptr() as *const i8, (*(*w).wq as *mut i8));
    seq_printf(m, b"    comp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*w).comp_calls));
    seq_printf(m, b"    comp_bytes: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*w).comp_bytes));
    seq_printf(m, b"    decomp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*w).decomp_calls));
    seq_printf(m, b"    decomp_bytes: %llu\n\n\0".as_ptr() as *const i8, atomic64_read(&(*w).decomp_bytes));
}

unsafe fn device_stats_show(m: *mut seq_file, d: *mut iaa_device) {
    seq_puts(m, b"iaa device:\n\0".as_ptr() as *const i8);
    seq_printf(m, b"  id: %d\n\0".as_ptr() as *const i8, (*(*d).idxd).id);
    seq_printf(m, b"  n_wqs: %d\n\0".as_ptr() as *const i8, (*d).n_wq);
    seq_printf(m, b"  comp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*d).comp_calls));
    seq_printf(m, b"  comp_bytes: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*d).comp_bytes));
    seq_printf(m, b"  decomp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*d).decomp_calls));
    seq_printf(m, b"  decomp_bytes: %llu\n\0".as_ptr() as *const i8, atomic64_read(&(*d).decomp_bytes));
    seq_puts(m, b"  wqs:\n\0".as_ptr() as *const i8);
    list_for_each_entry!(wq, (*d).wqs, list, { wq_show(m, wq); });
}

unsafe extern "C" fn global_stats_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    seq_puts(m, b"global stats:\n\0".as_ptr() as *const i8);
    seq_printf(m, b"  total_comp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_comp_calls));
    seq_printf(m, b"  total_decomp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_decomp_calls));
    seq_printf(m, b"  total_sw_comp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_sw_comp_calls));
    seq_printf(m, b"  total_sw_decomp_calls: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_sw_decomp_calls));
    seq_printf(m, b"  total_comp_bytes_out: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_comp_bytes_out));
    seq_printf(m, b"  total_decomp_bytes_in: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_decomp_bytes_in));
    seq_printf(m, b"  total_completion_einval_errors: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_completion_einval_errors));
    seq_printf(m, b"  total_completion_timeout_errors: %llu\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_completion_timeout_errors));
    seq_printf(m, b"  total_completion_comp_buf_overflow_errors: %llu\n\n\0".as_ptr() as *const i8, atomic64_read(&raw const total_completion_comp_buf_overflow_errors)); 0
}

unsafe extern "C" fn wq_stats_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    mutex_lock(&raw mut iaa_devices_lock); list_for_each_entry!(d, iaa_devices, list, { device_stats_show(m, d); }); mutex_unlock(&raw mut iaa_devices_lock); 0
}
unsafe extern "C" fn iaa_crypto_stats_reset(_data: *mut c_void, _value: u64) -> i32 {
    reset_iaa_crypto_stats(); mutex_lock(&raw mut iaa_devices_lock); list_for_each_entry!(d, iaa_devices, list, { reset_device_stats(d); }); mutex_unlock(&raw mut iaa_devices_lock); 0
}

unsafe extern "C" fn wq_stats_open(_inode: *mut inode, file: *mut file) -> i32 { single_open(file, wq_stats_show, file as *mut c_void) }
unsafe extern "C" fn global_stats_open(_inode: *mut inode, file: *mut file) -> i32 { single_open(file, global_stats_show, file as *mut c_void) }

static WQ_STATS_FOPS: file_operations = file_operations { open: Some(wq_stats_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(single_release) };
static GLOBAL_STATS_FOPS: file_operations = file_operations { open: Some(global_stats_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(single_release) };

#[no_mangle] pub unsafe extern "C" fn iaa_crypto_debugfs_init() -> i32 {
    if !debugfs_initialized() { return -19; }
    iaa_crypto_debugfs_root = debugfs_create_dir(b"iaa_crypto\0".as_ptr() as *const i8, core::ptr::null_mut());
    debugfs_create_file(b"global_stats\0".as_ptr() as *const i8, 0o644, iaa_crypto_debugfs_root, core::ptr::null_mut(), &GLOBAL_STATS_FOPS);
    debugfs_create_file(b"wq_stats\0".as_ptr() as *const i8, 0o644, iaa_crypto_debugfs_root, core::ptr::null_mut(), &WQ_STATS_FOPS);
    debugfs_create_file(b"stats_reset\0".as_ptr() as *const i8, 0o644, iaa_crypto_debugfs_root, core::ptr::null_mut(), core::ptr::null()); 0
}
#[no_mangle] pub unsafe extern "C" fn iaa_crypto_debugfs_cleanup() { debugfs_remove_recursive(iaa_crypto_debugfs_root); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
