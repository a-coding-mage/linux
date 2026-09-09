// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012  Google, Inc.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_void};

extern "C" {
    static mut psinfo: *mut pstore_info;
    static mut oops_in_progress: i32;

    fn core_kernel_text(ip: usize) -> bool;
    fn kaslr_offset() -> usize;
    fn ftrace_test_recursion_trylock(ip: usize, parent_ip: usize) -> i32;
    fn ftrace_test_recursion_unlock(bit: i32);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn raw_smp_processor_id() -> u32;
    fn pstore_ftrace_write_timestamp(rec: *mut pstore_ftrace_record, stamp: u64);
    fn pstore_ftrace_encode_cpu(rec: *mut pstore_ftrace_record, cpu: u32);
    fn ftrace_ops_set_global_filter(ops: *mut ftrace_ops);
    fn register_ftrace_function(ops: *mut ftrace_ops) -> isize;
    fn unregister_ftrace_function(ops: *mut ftrace_ops) -> isize;
    fn kstrtou8_from_user(buf: *const c_char, count: usize, base: u32, out: *mut u8) -> isize;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn simple_read_from_buffer(buf: *mut c_char, count: usize, ppos: *mut i64,
                               src: *const c_void, size: usize) -> isize;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry,
                           data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dir: *mut dentry);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct pstore_ftrace_record {
    pub ip: usize,
    pub parent_ip: usize,
    pub data: [u8; 16],
}

#[repr(C)]
pub struct pstore_record {
    pub type_: u32,
    pub buf: *mut c_char,
    pub size: usize,
    pub psi: *mut pstore_info,
}

#[repr(C)]
pub struct pstore_info { pub write: Option<unsafe extern "C" fn(*mut pstore_record)> }
#[repr(C)] pub struct ftrace_regs { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

#[repr(C)]
pub struct ftrace_ops {
    pub func: Option<unsafe extern "C" fn(usize, usize, *mut ftrace_ops, *mut ftrace_regs)>,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> isize>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
}
#[repr(C)] pub struct inode { _private: [u8; 0] }

const PSTORE_TYPE_FTRACE: u32 = 2;
const GFP_KERNEL: u32 = 0;
const ENOMEM: isize = 12;

/* This doesn't need to be atomic: speed is chosen over correctness here. */
static mut pstore_ftrace_stamp: u64 = 0;

#[inline]
unsafe fn adjust_ip(mut ip: usize) -> usize {
    // CONFIG_RANDOMIZE_BASE && !PSTORE_CPU_IN_IP && IS_BUILTIN(CONFIG_PSTORE)
    if core_kernel_text(ip) { ip = ip.wrapping_sub(kaslr_offset()); }
    ip
}

#[inline]
pub unsafe fn decode_ip(ip: usize) -> usize { ip }

unsafe extern "C" fn pstore_ftrace_call(ip: usize, parent_ip: usize,
                                          _op: *mut ftrace_ops, _fregs: *mut ftrace_regs) {
    let mut flags: usize = 0;
    let mut rec: pstore_ftrace_record = core::mem::zeroed();
    let mut record = pstore_record { type_: PSTORE_TYPE_FTRACE, buf: &mut rec as *mut _ as *mut c_char,
        size: core::mem::size_of::<pstore_ftrace_record>(), psi: psinfo };
    if oops_in_progress != 0 { return; }
    let bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if bit < 0 { return; }
    local_irq_save(&mut flags);
    rec.ip = adjust_ip(ip);
    rec.parent_ip = adjust_ip(parent_ip);
    pstore_ftrace_write_timestamp(&mut rec, pstore_ftrace_stamp);
    pstore_ftrace_stamp = pstore_ftrace_stamp.wrapping_add(1);
    pstore_ftrace_encode_cpu(&mut rec, raw_smp_processor_id());
    if let Some(write) = (*psinfo).write { write(&mut record); }
    local_irq_restore(flags);
    ftrace_test_recursion_unlock(bit);
}

static mut pstore_ftrace_ops: ftrace_ops = ftrace_ops { func: Some(pstore_ftrace_call) };
static mut pstore_ftrace_lock: mutex = mutex { _private: [] };
static mut record_ftrace: bool = false;
static mut pstore_ftrace_dir: *mut dentry = core::ptr::null_mut();

unsafe fn pstore_set_ftrace_enabled(on: bool) -> isize {
    if on == record_ftrace { return 0; }
    let ret = if on { ftrace_ops_set_global_filter(&mut pstore_ftrace_ops); register_ftrace_function(&mut pstore_ftrace_ops) }
              else { unregister_ftrace_function(&mut pstore_ftrace_ops) };
    if ret == 0 { record_ftrace = on; }
    ret
}

unsafe extern "C" fn pstore_ftrace_knob_write(_f: *mut file, buf: *const c_char, count: usize, _ppos: *mut i64) -> isize {
    let mut on = 0u8;
    let ret = kstrtou8_from_user(buf, count, 2, &mut on);
    if ret != 0 { return ret; }
    mutex_lock(&mut pstore_ftrace_lock);
    let ret = pstore_set_ftrace_enabled(on != 0);
    mutex_unlock(&mut pstore_ftrace_lock);
    if ret == 0 { count as isize } else { ret }
}

unsafe extern "C" fn pstore_ftrace_knob_read(_f: *mut file, buf: *mut c_char, count: usize, ppos: *mut i64) -> isize {
    let val = [b'0' + record_ftrace as u8, b'\n'];
    simple_read_from_buffer(buf, count, ppos, val.as_ptr() as *const c_void, val.len())
}

static pstore_knob_fops: file_operations = file_operations { open: None, read: Some(pstore_ftrace_knob_read), write: Some(pstore_ftrace_knob_write) };

pub unsafe extern "C" fn pstore_register_ftrace() {
    if (*psinfo).write.is_none() { return; }
    pstore_ftrace_dir = debugfs_create_dir(b"pstore\0".as_ptr() as *const c_char, core::ptr::null_mut());
    pstore_set_ftrace_enabled(record_ftrace);
    debugfs_create_file(b"record_ftrace\0".as_ptr() as *const c_char, 0o600, pstore_ftrace_dir, core::ptr::null_mut(), &pstore_knob_fops);
}

pub unsafe extern "C" fn pstore_unregister_ftrace() {
    mutex_lock(&mut pstore_ftrace_lock);
    if record_ftrace { unregister_ftrace_function(&mut pstore_ftrace_ops); record_ftrace = false; }
    mutex_unlock(&mut pstore_ftrace_lock);
    debugfs_remove_recursive(pstore_ftrace_dir);
}

pub unsafe extern "C" fn pstore_ftrace_combine_log(dest_log: *mut *mut c_char, dest_log_size: *mut usize,
    src_log: *const c_char, src_log_size: usize) -> isize {
    let record_size = core::mem::size_of::<pstore_ftrace_record>();
    let dest_off = *dest_log_size % record_size; let dest_size = *dest_log_size - dest_off;
    let src_off = src_log_size % record_size; let src_size = src_log_size - src_off;
    let total = dest_size + src_size; let merged_buf = kmalloc(total, GFP_KERNEL);
    if merged_buf.is_null() { return -ENOMEM; }
    let mut out = merged_buf as *mut pstore_ftrace_record;
    let drec = (*dest_log).add(dest_off) as *const pstore_ftrace_record;
    let srec = src_log.add(src_off) as *const pstore_ftrace_record;
    let mut di = 0usize; let mut si = 0usize; let mut ds = dest_size; let mut ss = src_size;
    while ds > 0 && ss > 0 {
        if pstore_ftrace_read_timestamp(drec.add(di)) < pstore_ftrace_read_timestamp(srec.add(si)) { *out = *drec.add(di); di += 1; ds -= record_size; }
        else { *out = *srec.add(si); si += 1; ss -= record_size; }
        out = out.add(1);
    }
    while ds > 0 { *out = *drec.add(di); out = out.add(1); di += 1; ds -= record_size; }
    while ss > 0 { *out = *srec.add(si); out = out.add(1); si += 1; ss -= record_size; }
    kfree(*dest_log as *mut c_void); *dest_log = merged_buf as *mut c_char; *dest_log_size = total; 0
}

extern "C" { fn pstore_ftrace_read_timestamp(rec: *const pstore_ftrace_record) -> u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
