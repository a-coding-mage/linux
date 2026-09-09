// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct Rust translation boundary for smb/server/vfs.c.
// Linux-kernel and ksmbd types/functions are supplied by the surrounding
// translation unit; they are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct ksmbd_work { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_file { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_share_config { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct kstat { _private: [u8; 0] }
#[repr(C)] pub struct file_allocated_range_buffer { pub file_offset: u64, pub length: u64 }

pub type loff_t = i64;
pub type umode_t = u16;
pub type __le32 = u32;
pub type ssize_t = isize;

extern "C" {
    pub fn ksmbd_vfs_create(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> c_int;
    pub fn ksmbd_vfs_mkdir(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> c_int;
    pub fn ksmbd_vfs_read(work: *mut ksmbd_work, fp: *mut ksmbd_file, count: usize,
                          pos: *mut loff_t, rbuf: *mut c_char) -> c_int;
    pub fn ksmbd_vfs_write(work: *mut ksmbd_work, fp: *mut ksmbd_file, buf: *mut c_char,
                           count: usize, pos: *mut loff_t, sync: bool,
                           written: *mut ssize_t) -> c_int;
    pub fn ksmbd_vfs_getattr(path: *const path, stat: *mut kstat) -> c_int;
    pub fn ksmbd_vfs_fsync(work: *mut ksmbd_work, fid: u64, p_id: u64) -> c_int;
    pub fn ksmbd_vfs_remove_file(work: *mut ksmbd_work, path: *const path) -> c_int;
    pub fn ksmbd_vfs_link(work: *mut ksmbd_work, oldname: *const c_char,
                          newname: *const c_char) -> c_int;
    pub fn ksmbd_vfs_check_rename_share(work: *mut ksmbd_work,
                                        old_path: *const path) -> c_int;
    pub fn ksmbd_vfs_rename(work: *mut ksmbd_work, old_fp: *mut ksmbd_file,
                            newname: *mut c_char, flags: c_int) -> c_int;
    pub fn ksmbd_vfs_truncate(work: *mut ksmbd_work, fp: *mut ksmbd_file,
                              size: loff_t) -> c_int;
    pub fn ksmbd_vfs_listxattr(dentry: *mut dentry, list: *mut *mut c_char) -> ssize_t;
    pub fn ksmbd_vfs_getxattr(idmap: *mut mnt_idmap, dentry: *mut dentry,
                              xattr_name: *mut c_char, xattr_buf: *mut *mut c_char) -> ssize_t;
    pub fn ksmbd_vfs_setxattr(idmap: *mut mnt_idmap, path: *const path,
                              attr_name: *const c_char, attr_value: *mut c_void,
                              attr_size: usize, flags: c_int, get_write: bool) -> c_int;
    pub fn ksmbd_vfs_set_fadvise(filp: *mut c_void, option: __le32);
    pub fn ksmbd_vfs_zero_data(work: *mut ksmbd_work, fp: *mut ksmbd_file,
                               off: loff_t, len: loff_t) -> c_int;
    pub fn ksmbd_vfs_zero_holes(fp: *mut ksmbd_file) -> c_int;
    pub fn ksmbd_vfs_trim_data(work: *mut ksmbd_work, fp: *mut ksmbd_file,
                               off: loff_t, len: loff_t) -> c_int;
    pub fn ksmbd_vfs_query_allocated_ranges(fp: *mut ksmbd_file, start: loff_t,
                                            length: loff_t,
                                            ranges: *mut file_allocated_range_buffer,
                                            in_count: c_uint, out_count: *mut c_uint) -> c_int;
    pub fn ksmbd_vfs_remove_xattr(idmap: *mut mnt_idmap, path: *const path,
                                  attr_name: *mut c_char, get_write: bool) -> c_int;
    pub fn ksmbd_vfs_unlink(filp: *mut c_void) -> c_int;
    pub fn ksmbd_vfs_empty_dir(fp: *mut ksmbd_file) -> c_int;
    pub fn ksmbd_vfs_kern_path(work: *mut ksmbd_work, filepath: *mut c_char,
                               flags: c_uint, path: *mut path, caseless: bool) -> c_int;
    pub fn ksmbd_vfs_kern_path_start_removing(work: *mut ksmbd_work, filepath: *mut c_char,
                                              flags: c_uint, path: *mut path,
                                              caseless: bool) -> c_int;
    pub fn ksmbd_vfs_kern_path_end_removing(path: *const path);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
