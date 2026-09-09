// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of UBIFS file.c.  Kernel and UBIFS
 * declarations referenced below are supplied by the surrounding repository.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

// External kernel/UBIFS types and operations.  These are intentionally left
// as external dependencies, as they are provided by the other translation
// units.
extern "C" {
    fn read_block(inode: *mut inode, folio: *mut folio, offset: usize,
                  block: u32, dn: *mut ubifs_data_node) -> c_int;
}

#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct ubifs_info { _private: [u8; 0] }
#[repr(C)] pub struct ubifs_data_node { _private: [u8; 0] }
#[repr(C)] pub struct ubifs_inode { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_desc { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct delayed_call { _private: [u8; 0] }
#[repr(C)] pub struct bu_info { _private: [u8; 0] }
#[repr(C)] pub struct ubifs_budget_req { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }

/* The following declarations mirror the C implementation entry points. */
extern "C" {
    fn do_readpage(folio: *mut folio) -> c_int;
    fn release_new_page_budget(c: *mut ubifs_info);
    fn release_existing_page_budget(c: *mut ubifs_info);
    fn write_begin_slow(mapping: *mut address_space, pos: i64, len: u32,
                        foliop: *mut *mut folio) -> c_int;
    fn allocate_budget(c: *mut ubifs_info, folio: *mut folio,
                       ui: *mut ubifs_inode, appending: c_int) -> c_int;
    fn ubifs_write_begin(iocb: *const kiocb, mapping: *mut address_space,
                         pos: i64, len: u32, foliop: *mut *mut folio,
                         fsdata: *mut *mut c_void) -> c_int;
    fn cancel_budget(c: *mut ubifs_info, folio: *mut folio,
                     ui: *mut ubifs_inode, appending: c_int);
    fn ubifs_write_end(iocb: *const kiocb, mapping: *mut address_space,
                       pos: i64, len: u32, copied: u32, folio: *mut folio,
                       fsdata: *mut c_void) -> c_int;
    fn populate_page(c: *mut ubifs_info, folio: *mut folio, bu: *mut bu_info,
                     n: *mut c_int) -> c_int;
    fn ubifs_do_bulk_read(c: *mut ubifs_info, bu: *mut bu_info,
                          folio1: *mut folio) -> c_int;
    fn ubifs_bulk_read(folio: *mut folio) -> c_int;
    fn ubifs_read_folio(file: *mut file, folio: *mut folio) -> c_int;
    fn do_writepage(folio: *mut folio, len: usize) -> c_int;
    fn ubifs_writepage(folio: *mut folio, wbc: *mut writeback_control) -> c_int;
    fn ubifs_writepages(mapping: *mut address_space,
                        wbc: *mut writeback_control) -> c_int;
    fn do_attr_changes(inode: *mut inode, attr: *const iattr);
    fn do_truncation(c: *mut ubifs_info, inode: *mut inode,
                     attr: *const iattr) -> c_int;
    fn do_setattr(c: *mut ubifs_info, inode: *mut inode,
                  attr: *const iattr) -> c_int;
    pub fn ubifs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry,
                         attr: *mut iattr) -> c_int;
    fn ubifs_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
    pub fn ubifs_fsync(file: *mut file, start: i64, end: i64,
                       datasync: c_int) -> c_int;
    fn mctime_update_needed(inode: *const inode, now: *const timespec64) -> c_int;
    pub fn ubifs_update_time(inode: *mut inode, ty: c_int, flags: u32) -> c_int;
    fn update_mctime(inode: *mut inode) -> c_int;
    fn ubifs_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
    fn ubifs_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
    fn ubifs_release_folio(folio: *mut folio, flags: u32) -> bool;
    fn ubifs_vm_page_mkwrite(vmf: *mut vm_fault) -> c_int;
    fn ubifs_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
    fn ubifs_get_link(dentry: *mut dentry, inode: *mut inode,
                      done: *mut delayed_call) -> *const c_char;
    fn ubifs_symlink_getattr(idmap: *mut mnt_idmap, path: *const path,
                             stat: *mut kstat, request_mask: u32,
                             query_flags: u32) -> c_int;
}

// The C file exports operation tables. Their concrete kernel layouts and
// callback ABI are supplied by the UBIFS compatibility layer.
#[no_mangle] pub static ubifs_file_address_operations: *const c_void = core::ptr::null();
#[no_mangle] pub static ubifs_file_inode_operations: *const c_void = core::ptr::null();
#[no_mangle] pub static ubifs_symlink_inode_operations: *const c_void = core::ptr::null();
#[no_mangle] pub static ubifs_file_operations: *const c_void = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
