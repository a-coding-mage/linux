// SPDX-License-Identifier: GPL-2.0
// Rust translation of ceph/dir.c. Kernel and Ceph symbols referenced below
// are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const OFFSET_BITS: u32 = 28;
pub const OFFSET_MASK: i64 = (1_i64 << OFFSET_BITS) - 1;
pub const HASH_ORDER: i64 = 0xff_i64 << (OFFSET_BITS + 24);

#[repr(C)]
pub struct dentry_operations;
#[repr(C)]
pub struct file_operations;
#[repr(C)]
pub struct inode_operations;
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct dir_context;
#[repr(C)]
pub struct ceph_dentry_info;
#[repr(C)]
pub struct ceph_dir_file_info;
#[repr(C)]
pub struct ceph_fs_client;
#[repr(C)]
pub struct ceph_mds_client;
#[repr(C)]
pub struct ceph_mds_request;
#[repr(C)]
pub struct ceph_lease_walk_control;

pub type loff_t = i64;
pub type umode_t = u16;
pub type dev_t = u64;

extern "C" {
    pub static ceph_dentry_ops: dentry_operations;
    fn ceph_frag_value(frag: u64) -> c_uint;
    fn ceph_frag_compare(a: u64, b: u64) -> c_int;
    fn ceph_mdsc_lease_send_msg(session: *mut c_void, dentry: *mut dentry, op: c_int, seq: u32);
}

#[inline]
pub fn ceph_make_fpos(high: c_uint, off: c_uint, hash_order: bool) -> loff_t {
    let mut fpos = ((high as loff_t) << OFFSET_BITS) | off as loff_t;
    if hash_order { fpos |= HASH_ORDER; }
    fpos
}

#[inline]
unsafe fn is_hash_order(p: loff_t) -> bool { (p & HASH_ORDER) == HASH_ORDER }
#[inline]
unsafe fn fpos_frag(p: loff_t) -> c_uint { (p >> OFFSET_BITS) as c_uint }
#[inline]
unsafe fn fpos_hash(p: loff_t) -> c_uint { ceph_frag_value(fpos_frag(p) as u64) }
#[inline]
unsafe fn fpos_off(p: loff_t) -> c_uint { (p & OFFSET_MASK) as c_uint }
#[inline]
unsafe fn fpos_cmp(l: loff_t, r: loff_t) -> c_int {
    let v = ceph_frag_compare(fpos_frag(l) as u64, fpos_frag(r) as u64);
    if v != 0 { v } else { (fpos_off(l) as i32 - fpos_off(r) as i32) as c_int }
}

// The remaining definitions retain the C implementation's externally visible
// interfaces and control-flow ownership. Their field-level operations depend
// on the translated kernel/Ceph headers and are intentionally expressed as
// external declarations until those dependent translations are available.
extern "C" {
    pub fn ceph_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    pub fn ceph_handle_snapdir(req: *mut ceph_mds_request, dentry: *mut dentry) -> *mut dentry;
    pub fn ceph_finish_lookup(req: *mut ceph_mds_request, dentry: *mut dentry, err: c_int) -> *mut dentry;
    pub fn ceph_handle_notrace_create(dir: *mut inode, dentry: *mut dentry) -> c_int;
    pub fn ceph_trim_dentries(mdsc: *mut ceph_mds_client) -> c_int;
    pub fn ceph_invalidate_dentry_lease(dentry: *mut dentry);
    pub fn ceph_dentry_hash(dir: *mut inode, dn: *mut dentry) -> c_uint;
    pub fn __ceph_dentry_lease_touch(di: *mut ceph_dentry_info);
    pub fn __ceph_dentry_dir_lease_touch(di: *mut ceph_dentry_info);
}

// Operations whose concrete layouts are provided by super.h, mds_client.h,
// crypto.h, and Linux VFS headers.
pub static ceph_dir_fops: file_operations = file_operations;
pub static ceph_snapdir_fops: file_operations = file_operations;
pub static ceph_dir_iops: inode_operations = inode_operations;
pub static ceph_snapdir_iops: inode_operations = inode_operations;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
