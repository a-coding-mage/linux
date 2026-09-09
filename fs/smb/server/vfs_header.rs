/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

/* Translated from vfs.h. Kernel and project headers provide the referenced types. */

use core::ffi::{c_char, c_void};

pub type __le32 = u32;
pub type umode_t = u16;
pub type loff_t = i64;
pub type u64 = core::primitive::u64;
pub type u16 = core::primitive::u16;
pub type size_t = usize;
pub type ssize_t = isize;

/* Enumeration for stream type. */
pub const DATA_STREAM: u32 = 1; /* type $DATA */
pub const DIR_STREAM: u32 = 2; /* type $INDEX_ALLOCATION */

/* CreateOptions */
pub const CREATE_TREE_CONNECTION: __le32 = 0x00000080;
pub const FILE_RESERVE_OPFILTER_LE: __le32 = 0x00100000;
pub const CREATE_OPTION_READONLY: u32 = 0x10000000;
/* system. NB not sent over wire */
pub const CREATE_OPTION_SPECIAL: u32 = 0x20000000;

#[repr(C)]
pub struct ksmbd_work { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_file { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_conn { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)]
pub struct path { _private: [u8; 0] }
#[repr(C)]
pub struct kstat { _private: [u8; 0] }
#[repr(C)]
pub struct srv_copychunk { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }
#[repr(C)]
pub struct file_lock { _private: [u8; 0] }
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct dir_context { _private: [u8; 0] }
#[repr(C)]
pub struct unicode_map { _private: [u8; 0] }
#[repr(C)]
pub struct file_allocated_range_buffer { _private: [u8; 0] }
#[repr(C)]
pub struct smb_ntsd { _private: [u8; 0] }
#[repr(C)]
pub struct xattr_dos_attrib { _private: [u8; 0] }

#[repr(C)]
pub struct ksmbd_dir_info {
    pub name: *const c_char,
    pub wptr: *mut c_char,
    pub rptr: *mut c_char,
    pub name_len: i32,
    pub out_buf_len: i32,
    pub num_scan: i32,
    pub num_entry: i32,
    pub data_count: i32,
    pub last_entry_offset: i32,
    pub hide_dot_file: bool,
    pub flags: i32,
    pub last_entry_off_align: i32,
}

#[repr(C)]
pub union ksmbd_readdir_data_private {
    pub private: *mut c_void,
    pub dirent: *mut c_char,
}

#[repr(C)]
pub struct ksmbd_readdir_data {
    pub ctx: dir_context,
    pub data: ksmbd_readdir_data_private,
    pub used: u32,
    pub dirent_count: u32,
    pub file_attr: u32,
    pub um: *mut unicode_map,
}

/* ksmbd kstat wrapper to get valid create time when reading dir entry */
#[repr(C)]
pub struct ksmbd_kstat {
    pub kstat: *mut kstat,
    pub create_time: u64,
    pub file_attributes: __le32,
    pub has_ads_stream: bool, /* AAPL READDIR_ATTR V2 xattr-presence flag */
}

extern "C" {
    pub fn ksmbd_vfs_lock_parent(parent: *mut dentry, child: *mut dentry) -> i32;
    pub fn ksmbd_vfs_query_maximal_access(idmap: *mut mnt_idmap, dentry: *mut dentry, daccess: *mut __le32);
    pub fn ksmbd_vfs_create(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> i32;
    pub fn ksmbd_vfs_mkdir(work: *mut ksmbd_work, name: *const c_char, mode: umode_t) -> i32;
    pub fn ksmbd_vfs_read(work: *mut ksmbd_work, fp: *mut ksmbd_file, count: size_t, pos: *mut loff_t, rbuf: *mut c_char) -> i32;
    pub fn ksmbd_vfs_write(work: *mut ksmbd_work, fp: *mut ksmbd_file, buf: *mut c_char, count: size_t, pos: *mut loff_t, sync: bool, written: *mut ssize_t) -> i32;
    pub fn ksmbd_vfs_fsync(work: *mut ksmbd_work, fid: u64, p_id: u64) -> i32;
    pub fn ksmbd_vfs_remove_file(work: *mut ksmbd_work, path: *const path) -> i32;
    pub fn ksmbd_vfs_link(work: *mut ksmbd_work, oldname: *const c_char, newname: *const c_char) -> i32;
    pub fn ksmbd_vfs_getattr(path: *const path, stat: *mut kstat) -> i32;
    pub fn ksmbd_vfs_rename(work: *mut ksmbd_work, old_fp: *mut ksmbd_file, newname: *mut c_char, flags: i32) -> i32;
    pub fn ksmbd_vfs_check_rename_share(work: *mut ksmbd_work, old_path: *const path) -> i32;
    pub fn ksmbd_vfs_truncate(work: *mut ksmbd_work, fp: *mut ksmbd_file, size: loff_t) -> i32;
    pub fn ksmbd_vfs_copy_file_ranges(work: *mut ksmbd_work, src_fp: *mut ksmbd_file, dst_fp: *mut ksmbd_file, chunks: *mut srv_copychunk, chunk_count: u32, chunk_count_written: *mut u32, chunk_size_written: *mut u32, total_size_written: *mut loff_t) -> i32;
    pub fn ksmbd_vfs_listxattr(dentry: *mut dentry, list: *mut *mut c_char) -> ssize_t;
    pub fn ksmbd_vfs_getxattr(idmap: *mut mnt_idmap, dentry: *mut dentry, xattr_name: *mut c_char, xattr_buf: *mut *mut c_char) -> ssize_t;
    pub fn ksmbd_vfs_casexattr_len(idmap: *mut mnt_idmap, dentry: *mut dentry, attr_name: *mut c_char, attr_name_len: i32) -> ssize_t;
    pub fn ksmbd_vfs_setxattr(idmap: *mut mnt_idmap, path: *const path, attr_name: *const c_char, attr_value: *mut c_void, attr_size: size_t, flags: i32, get_write: bool) -> i32;
    pub fn ksmbd_vfs_xattr_stream_name(stream_name: *mut c_char, xattr_stream_name: *mut *mut c_char, xattr_stream_name_size: *mut size_t, s_type: i32) -> i32;
    pub fn ksmbd_vfs_remove_xattr(idmap: *mut mnt_idmap, path: *const path, attr_name: *mut c_char, get_write: bool) -> i32;
    pub fn ksmbd_vfs_kern_path(work: *mut ksmbd_work, name: *mut c_char, flags: u32, path: *mut path, caseless: bool) -> i32;
    pub fn ksmbd_vfs_kern_path_start_removing(work: *mut ksmbd_work, name: *mut c_char, flags: u32, path: *mut path, caseless: bool) -> i32;
    pub fn ksmbd_vfs_kern_path_end_removing(path: *const path);
    pub fn ksmbd_vfs_kern_path_create(work: *mut ksmbd_work, name: *const c_char, flags: u32, path: *mut path) -> *mut dentry;
    pub fn ksmbd_vfs_empty_dir(fp: *mut ksmbd_file) -> i32;
    pub fn ksmbd_vfs_set_fadvise(filp: *mut file, option: __le32);
    pub fn ksmbd_vfs_zero_data(work: *mut ksmbd_work, fp: *mut ksmbd_file, off: loff_t, len: loff_t) -> i32;
    pub fn ksmbd_vfs_zero_holes(fp: *mut ksmbd_file) -> i32;
    pub fn ksmbd_vfs_trim_data(work: *mut ksmbd_work, fp: *mut ksmbd_file, off: loff_t, len: loff_t) -> i32;
    pub fn ksmbd_vfs_query_allocated_ranges(fp: *mut ksmbd_file, start: loff_t, length: loff_t, ranges: *mut file_allocated_range_buffer, in_count: u32, out_count: *mut u32) -> i32;
    pub fn ksmbd_vfs_unlink(filp: *mut file) -> i32;
    pub fn ksmbd_vfs_init_kstat(p: *mut *mut c_char, ksmbd_kstat: *mut ksmbd_kstat) -> *mut c_void;
    pub fn ksmbd_vfs_fill_dentry_attrs(work: *mut ksmbd_work, idmap: *mut mnt_idmap, dentry: *mut dentry, ksmbd_kstat: *mut ksmbd_kstat) -> i32;
    pub fn ksmbd_vfs_posix_lock_wait(flock: *mut file_lock);
    pub fn ksmbd_vfs_posix_lock_unblock(flock: *mut file_lock);
    pub fn ksmbd_vfs_remove_acl_xattrs(idmap: *mut mnt_idmap, path: *const path) -> i32;
    pub fn ksmbd_vfs_remove_sd_xattrs(idmap: *mut mnt_idmap, path: *const path) -> i32;
    pub fn ksmbd_vfs_set_sd_xattr(conn: *mut ksmbd_conn, idmap: *mut mnt_idmap, path: *const path, pntsd: *mut smb_ntsd, len: i32, get_write: bool) -> i32;
    pub fn ksmbd_vfs_get_sd_xattr(conn: *mut ksmbd_conn, idmap: *mut mnt_idmap, dentry: *mut dentry, pntsd: *mut *mut smb_ntsd) -> i32;
    pub fn ksmbd_vfs_set_dos_attrib_xattr(idmap: *mut mnt_idmap, path: *const path, da: *mut xattr_dos_attrib, get_write: bool) -> i32;
    pub fn ksmbd_vfs_get_dos_attrib_xattr(idmap: *mut mnt_idmap, dentry: *mut dentry, da: *mut xattr_dos_attrib) -> i32;
    pub fn ksmbd_vfs_set_init_posix_acl(idmap: *mut mnt_idmap, path: *const path) -> i32;
    pub fn ksmbd_vfs_inherit_posix_acl(idmap: *mut mnt_idmap, path: *const path, parent_inode: *mut inode) -> i32;
    pub fn ksmbd_vfs_update_compressed_fattr(dentry: *mut dentry, fattr: *mut __le32);
    pub fn ksmbd_vfs_get_compression(fp: *mut ksmbd_file, fmt: *mut u16) -> i32;
    pub fn ksmbd_vfs_set_compression(work: *mut ksmbd_work, fp: *mut ksmbd_file, fmt: u16) -> i32;
    pub fn ksmbd_vfs_set_compression_create(work: *mut ksmbd_work, fp: *mut ksmbd_file, fmt: u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
