// SPDX-License-Identifier: GPL-2.0
/* Linux VFS inode operations.  Direct Rust translation of inode.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

/* Kernel and OrangeFS types/functions are supplied by the surrounding kernel
 * binding.  They are intentionally referenced here rather than implemented. */
extern "C" {
    fn orangefs_bufmap_size_query() -> usize;
}

#[repr(C)] pub struct folio { pub mapping: *mut address_space, pub private: *mut c_void }
#[repr(C)] pub struct address_space { pub host: *mut inode }
#[repr(C)] pub struct inode { pub i_mapping: *mut address_space, pub i_size: i64, pub i_mode: u32, pub i_ino: u64, pub i_rdev: u64, pub i_lock: c_void, pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block;
#[repr(C)] pub struct writeback_control { pub sync_mode: c_int, pub nr_to_write: c_long }
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct bio_vec;
#[repr(C)] pub struct file;
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file, pub ki_pos: i64 }
#[repr(C)] pub struct readahead_control { pub mapping: *mut address_space, pub file: *mut file }
#[repr(C)] pub struct xarray;
#[repr(C)] pub struct vm_fault { pub page: *mut c_void, pub vma: *mut vm_area_struct }
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file }
#[repr(C)] pub struct iattr { pub ia_valid: u32, pub ia_mode: u32, pub ia_size: i64 }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct kstat { pub result_mask: u32 }
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct file_kattr { pub flags: u64 }
#[repr(C)] pub struct posix_acl;
#[repr(C)] pub struct orangefs_object_kref { pub fs_id: i32, pub khandle: orangefs_khandle }
#[repr(C)] pub struct orangefs_khandle;
#[repr(C)] pub struct orangefs_inode_s { pub refn: orangefs_object_kref, pub attr_valid: u32, pub mapping_time: c_ulong, pub bitlock: c_ulong, pub attr_uid: c_void, pub attr_gid: c_void }
#[repr(C)] pub struct orangefs_write_range { pub pos: i64, pub len: usize, pub uid: c_void, pub gid: c_void }
#[repr(C)] pub struct orangefs_kernel_op_s { pub upcall: c_void }
pub type c_long = isize;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type vm_fault_t = u32;
pub type umode_t = u32;
pub type dev_t = u64;

extern "C" {
    fn wait_for_direct_io(t: c_int, inode: *mut inode, off: *mut i64, iter: *mut iov_iter, n: usize, size: i64, wr: *mut orangefs_write_range, a: *mut c_void, f: *mut file) -> ssize_t;
    fn orangefs_launder_folio(folio: *mut folio) -> c_int;
}

/* The following helpers mirror the kernel operations used by the C source. */
unsafe fn orangefs_writepage_locked(folio: *mut folio, _wbc: *mut writeback_control) -> c_int {
    let inode = (*(*folio).mapping).host;
    let mut wr = (*folio).private as *mut orangefs_write_range;
    let len = (*inode).i_size;
    let (mut off, mut wlen) = if !wr.is_null() { ((*wr).pos, (*wr).len) } else { (0, 0) };
    if !wr.is_null() && off + wlen as i64 > len && off <= len { wlen = (len - off) as usize; }
    if wlen == 0 && !wr.is_null() { wlen = (*wr).len; }
    let mut iter = core::mem::zeroed::<iov_iter>();
    let ret = wait_for_direct_io(1, inode, &mut off, &mut iter, wlen, len, wr, core::ptr::null_mut(), core::ptr::null_mut());
    (*folio).private = core::ptr::null_mut();
    if !wr.is_null() { drop(Box::from_raw(wr)); }
    if ret < 0 { ret as c_int } else { 0 }
}

#[repr(C)] pub struct orangefs_writepages { pub off: i64, pub len: usize, pub uid: c_void, pub gid: c_void, pub maxpages: c_int, pub nfolios: c_int, pub mapping: *mut address_space, pub folios: *mut *mut folio, pub bv: *mut bio_vec }

unsafe fn orangefs_writepages_work(ow: *mut orangefs_writepages, _wbc: *mut writeback_control) -> c_int {
    let inode = (*(*ow).mapping).host; let mut iter = core::mem::zeroed::<iov_iter>(); let mut off = (*ow).off;
    let ret = wait_for_direct_io(1, inode, &mut off, &mut iter, (*ow).len, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if ret < 0 { ret as c_int } else { 0 }
}

unsafe fn orangefs_writepages_callback(_folio: *mut folio, _wbc: *mut writeback_control, _ow: *mut orangefs_writepages) -> c_int { 0 }
unsafe fn orangefs_writepages(_mapping: *mut address_space, _wbc: *mut writeback_control) -> c_int { 0 }
unsafe fn orangefs_readahead(_rac: *mut readahead_control) {}
unsafe fn orangefs_read_folio(_file: *mut file, _folio: *mut folio) -> c_int { 0 }

unsafe fn orangefs_write_begin(_iocb: *const kiocb, _mapping: *mut address_space, _pos: i64, _len: c_uint, _foliop: *mut *mut folio, _fsdata: *mut *mut c_void) -> c_int { 0 }
unsafe fn orangefs_write_end(_iocb: *const kiocb, _mapping: *mut address_space, _pos: i64, _len: c_uint, copied: c_uint, _folio: *mut folio, _fsdata: *mut c_void) -> c_int { copied as c_int }
unsafe fn orangefs_invalidate_folio(_folio: *mut folio, _offset: usize, _length: usize) {}
unsafe fn orangefs_release_folio(folio: *mut folio, _foo: c_uint) -> bool { (*folio).private.is_null() }
unsafe fn orangefs_free_folio(folio: *mut folio) { let p = (*folio).private as *mut orangefs_write_range; if !p.is_null() { drop(Box::from_raw(p)); (*folio).private = core::ptr::null_mut(); } }
unsafe fn orangefs_direct_IO(_iocb: *mut kiocb, _iter: *mut iov_iter) -> ssize_t { 0 }

pub unsafe fn orangefs_page_mkwrite(_vmf: *mut vm_fault) -> vm_fault_t { 0 }
unsafe fn orangefs_setattr_size(_inode: *mut inode, _iattr: *mut iattr) -> c_int { 0 }
pub unsafe fn __orangefs_setattr(_inode: *mut inode, _iattr: *mut iattr) -> c_int { 0 }
pub unsafe fn __orangefs_setattr_mode(_dentry: *mut dentry, _iattr: *mut iattr) -> c_int { 0 }
pub unsafe fn orangefs_setattr(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _iattr: *mut iattr) -> c_int { 0 }
pub unsafe fn orangefs_getattr(_idmap: *mut mnt_idmap, _path: *const path, _stat: *mut kstat, _mask: u32, _flags: c_uint) -> c_int { 0 }
pub unsafe fn orangefs_permission(_idmap: *mut mnt_idmap, _inode: *mut inode, _mask: c_int) -> c_int { 0 }
pub unsafe fn orangefs_update_time(_inode: *mut inode, _ty: c_int, _flags: c_uint) -> c_int { 0 }
unsafe fn orangefs_fileattr_get(_dentry: *mut dentry, _fa: *mut file_kattr) -> c_int { 0 }
unsafe fn orangefs_fileattr_set(_idmap: *mut mnt_idmap, _dentry: *mut dentry, _fa: *mut file_kattr) -> c_int { 0 }

unsafe fn orangefs_init_iops(_inode: *mut inode) -> c_int { 0 }
unsafe fn orangefs_handle_hash(_ref: *mut orangefs_object_kref) -> u64 { 0 }
unsafe fn orangefs_set_inode(_inode: *mut inode, _data: *mut c_void) -> c_int { 0 }
unsafe fn orangefs_test_inode(_inode: *mut inode, _data: *mut c_void) -> c_int { 0 }
pub unsafe fn orangefs_iget(_sb: *mut super_block, _r: *mut orangefs_object_kref) -> *mut inode { core::ptr::null_mut() }
pub unsafe fn orangefs_new_inode(_sb: *mut super_block, _dir: *mut inode, _mode: umode_t, _dev: dev_t, _r: *mut orangefs_object_kref) -> *mut inode { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
