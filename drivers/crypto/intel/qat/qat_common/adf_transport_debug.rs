// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// External Linux kernel and driver declarations supplied by the surrounding build.
use core::ffi::{c_char, c_int, c_longlong, c_void};

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}
#[repr(C)]
pub struct adf_etr_ring_data {
    pub ring_size: u32,
    pub msg_size: u32,
    pub base_addr: *mut u8,
    pub bank: *mut adf_etr_bank_data,
    pub ring_number: i32,
    pub ring_debug: *mut adf_etr_ring_debug_entry,
}
#[repr(C)]
pub struct adf_etr_bank_data {
    pub accel_dev: *mut adf_accel_dev,
    pub bank_number: i32,
    pub csr_addr: *mut c_void,
    pub rings: *mut adf_etr_ring_data,
    pub ring_mask: u32,
    pub bank_debug_dir: *mut dentry,
    pub bank_debug_cfg: *mut dentry,
}
#[repr(C)]
pub struct adf_etr_ring_debug_entry {
    pub ring_name: [c_char; 16],
    pub debug: *mut dentry,
}
#[repr(C)]
pub struct adf_accel_dev {
    pub transport: *mut adf_transport,
}
#[repr(C)]
pub struct adf_transport {
    pub debug: *mut dentry,
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct adf_hw_csr_ops {
    pub read_csr_ring_head: Option<unsafe extern "C" fn(*mut c_void, i32, i32) -> c_int>,
    pub read_csr_ring_tail: Option<unsafe extern "C" fn(*mut c_void, i32, i32) -> c_int>,
    pub read_csr_e_stat: Option<unsafe extern "C" fn(*mut c_void, i32) -> c_int>,
}

extern "C" {
    static mut ring_read_lock: mutex;
    static mut bank_read_lock: mutex;
    static SEQ_START_TOKEN: c_void;
    static adf_ring_debug_fops: c_void;
    static adf_bank_debug_fops: c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...);
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry,
                           data: *mut c_void, fops: *const c_void) -> *mut dentry;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove(entry: *mut dentry);
    fn seq_puts(file: *mut seq_file, text: *const c_char);
    fn seq_printf(file: *mut seq_file, fmt: *const c_char, ...);
    fn seq_hex_dump(file: *mut seq_file, prefix: *const c_char, flags: u32,
                    rowsize: u32, groupsize: u32, buffer: *const c_void,
                    len: usize, ascii: bool);
    fn get_csr_ops(dev: *mut adf_accel_dev) -> *mut adf_hw_csr_ops;
    fn get_num_rings_per_bank(dev: *mut adf_accel_dev) -> u8;
    fn size_to_ring_size_in_bytes(size: u32) -> usize;
    fn msg_size_to_bytes(size: u32) -> usize;
}

const S_IRUSR: u32 = 0o400;

unsafe fn adf_ring_start(sfile: *mut seq_file, pos: *mut i64) -> *mut c_void {
    let ring = (*sfile).private as *mut adf_etr_ring_data;
    let num_msg = size_to_ring_size_in_bytes((*ring).ring_size) / msg_size_to_bytes((*ring).msg_size);
    let val = *pos;
    mutex_lock(&mut ring_read_lock);
    if val == 0 { return &SEQ_START_TOKEN as *const c_void as *mut c_void; }
    if val >= num_msg as i64 { return core::ptr::null_mut(); }
    let result = (*ring).base_addr.add(msg_size_to_bytes((*ring).msg_size) * (*pos as usize));
    *pos += 1;
    result as *mut c_void
}

unsafe fn adf_ring_next(sfile: *mut seq_file, _v: *mut c_void, pos: *mut i64) -> *mut c_void {
    let ring = (*sfile).private as *mut adf_etr_ring_data;
    let num_msg = size_to_ring_size_in_bytes((*ring).ring_size) / msg_size_to_bytes((*ring).msg_size);
    let val = *pos;
    *pos += 1;
    if val >= num_msg as i64 { return core::ptr::null_mut(); }
    (*ring).base_addr.add(msg_size_to_bytes((*ring).msg_size) * val as usize) as *mut c_void
}

unsafe fn adf_ring_show(sfile: *mut seq_file, v: *mut c_void) -> c_int {
    let ring = (*sfile).private as *mut adf_etr_ring_data;
    let bank = (*ring).bank;
    let csr_ops = get_csr_ops((*bank).accel_dev);
    let csr = (*bank).csr_addr;
    if v == &SEQ_START_TOKEN as *const c_void as *mut c_void {
        let head = ((*csr_ops).read_csr_ring_head.unwrap())(csr, (*bank).bank_number, (*ring).ring_number);
        let tail = ((*csr_ops).read_csr_ring_tail.unwrap())(csr, (*bank).bank_number, (*ring).ring_number);
        let empty = ((*csr_ops).read_csr_e_stat.unwrap())(csr, (*bank).bank_number);
        seq_puts(sfile, b"------- Ring configuration -------\0".as_ptr() as *const c_char);
        seq_printf(sfile, b"ring name: %s\n\0".as_ptr() as *const c_char, (*ring).ring_debug);
        seq_printf(sfile, b"ring num %d, bank num %d\n\0".as_ptr() as *const c_char, (*ring).ring_number, (*bank).bank_number);
        seq_printf(sfile, b"head %x, tail %x, empty: %d\n\0".as_ptr() as *const c_char, head, tail, (empty & (1 << (*ring).ring_number)) >> (*ring).ring_number);
        seq_printf(sfile, b"ring size %lld, msg size %d\n\0".as_ptr() as *const c_char, size_to_ring_size_in_bytes((*ring).ring_size) as c_longlong, msg_size_to_bytes((*ring).msg_size));
        seq_puts(sfile, b"----------- Ring data ------------\n\0".as_ptr() as *const c_char);
        return 0;
    }
    seq_hex_dump(sfile, b"\0".as_ptr() as *const c_char, 1, 32, 4, v, msg_size_to_bytes((*ring).msg_size), false);
    0
}

unsafe fn adf_ring_stop(_sfile: *mut seq_file, _v: *mut c_void) { mutex_unlock(&mut ring_read_lock); }

pub unsafe fn adf_ring_debugfs_add(ring: *mut adf_etr_ring_data, name: *const c_char) -> c_int {
    let ring_debug = kzalloc_obj::<adf_etr_ring_debug_entry>();
    if ring_debug.is_null() { return -12; }
    strscpy((*ring_debug).ring_name.as_mut_ptr(), name);
    let mut entry_name = [0 as c_char; 16];
    snprintf(entry_name.as_mut_ptr(), entry_name.len(), b"ring_%02d\0".as_ptr() as *const c_char, (*ring).ring_number);
    (*ring_debug).debug = debugfs_create_file(entry_name.as_ptr(), S_IRUSR, (*(*ring).bank).bank_debug_dir, ring as *mut c_void, &adf_ring_debug_fops);
    (*ring).ring_debug = ring_debug;
    0
}

pub unsafe fn adf_ring_debugfs_rm(ring: *mut adf_etr_ring_data) {
    if !(*ring).ring_debug.is_null() {
        debugfs_remove((*(*ring).ring_debug).debug);
        kfree((*ring).ring_debug as *mut c_void);
        (*ring).ring_debug = core::ptr::null_mut();
    }
}

unsafe fn adf_bank_start(sfile: *mut seq_file, pos: *mut i64) -> *mut c_void {
    let bank = (*sfile).private as *mut adf_etr_bank_data;
    let num = get_num_rings_per_bank((*bank).accel_dev);
    mutex_lock(&mut bank_read_lock);
    if *pos == 0 { return &SEQ_START_TOKEN as *const c_void as *mut c_void; }
    if *pos >= num as i64 { return core::ptr::null_mut(); }
    pos as *mut c_void
}

unsafe fn adf_bank_next(sfile: *mut seq_file, _v: *mut c_void, pos: *mut i64) -> *mut c_void {
    let bank = (*sfile).private as *mut adf_etr_bank_data;
    if { *pos += 1; *pos } >= get_num_rings_per_bank((*bank).accel_dev) as i64 { return core::ptr::null_mut(); }
    pos as *mut c_void
}

unsafe fn adf_bank_show(sfile: *mut seq_file, v: *mut c_void) -> c_int {
    let bank = (*sfile).private as *mut adf_etr_bank_data;
    if v == &SEQ_START_TOKEN as *const c_void as *mut c_void {
        seq_printf(sfile, b"------- Bank %d configuration -------\n\0".as_ptr() as *const c_char, (*bank).bank_number);
        return 0;
    }
    let ring_id = *((v as *mut i32).offset(0)) - 1;
    let ring = (*bank).rings.add(ring_id as usize);
    if (*bank).ring_mask & (1 << ring_id) == 0 { return 0; }
    let ops = get_csr_ops((*bank).accel_dev);
    let head = ((*ops).read_csr_ring_head.unwrap())((*bank).csr_addr, (*bank).bank_number, (*ring).ring_number);
    let tail = ((*ops).read_csr_ring_tail.unwrap())((*bank).csr_addr, (*bank).bank_number, (*ring).ring_number);
    let empty = ((*ops).read_csr_e_stat.unwrap())((*bank).csr_addr, (*bank).bank_number);
    seq_printf(sfile, b"ring num %02d, head %04x, tail %04x, empty: %d\n\0".as_ptr() as *const c_char, (*ring).ring_number, head, tail, (empty & (1 << (*ring).ring_number)) >> (*ring).ring_number);
    0
}

unsafe fn adf_bank_stop(_sfile: *mut seq_file, _v: *mut c_void) { mutex_unlock(&mut bank_read_lock); }

pub unsafe fn adf_bank_debugfs_add(bank: *mut adf_etr_bank_data) -> c_int {
    let dev = (*bank).accel_dev;
    let parent = (*(*dev).transport).debug;
    let mut name = [0 as c_char; 16];
    snprintf(name.as_mut_ptr(), name.len(), b"bank_%02d\0".as_ptr() as *const c_char, (*bank).bank_number);
    (*bank).bank_debug_dir = debugfs_create_dir(name.as_ptr(), parent);
    (*bank).bank_debug_cfg = debugfs_create_file(b"config\0".as_ptr() as *const c_char, S_IRUSR, (*bank).bank_debug_dir, bank as *mut c_void, &adf_bank_debug_fops);
    0
}

pub unsafe fn adf_bank_debugfs_rm(bank: *mut adf_etr_bank_data) {
    debugfs_remove((*bank).bank_debug_cfg);
    debugfs_remove((*bank).bank_debug_dir);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
