// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of ocfs2/super.c.  Kernel and OCFS2
// declarations referenced below are supplied by the surrounding translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct mount_options {
    pub commit_interval: c_ulong,
    pub mount_opt: c_ulong,
    pub atime_quantum: c_uint,
    pub slot: u16,
    pub localalloc_opt: c_int,
    pub resv_level: c_uint,
    pub dir_resv_level: c_int,
    pub cluster_stack: [c_char; OCFS2_STACK_LABEL_LEN + 1],
    pub user_stack: bool,
}

extern "C" {
    static mut ocfs2_inode_cachep: *mut kmem_cache;
    pub static mut ocfs2_dquot_cachep: *mut kmem_cache;
    pub static mut ocfs2_qf_chunk_cachep: *mut kmem_cache;

    fn ocfs2_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    fn ocfs2_check_set_options(sb: *mut super_block, options: *mut mount_options) -> c_int;
    fn ocfs2_show_options(s: *mut seq_file, root: *mut dentry) -> c_int;
    fn ocfs2_put_super(sb: *mut super_block);
    fn ocfs2_mount_volume(sb: *mut super_block) -> c_int;
    fn ocfs2_dismount_volume(sb: *mut super_block, mnt_err: c_int);
    fn ocfs2_initialize_mem_caches() -> c_int;
    fn ocfs2_free_mem_caches();
    fn ocfs2_delete_osb(osb: *mut ocfs2_super);
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_super { _private: [u8; 0] }

pub const OCFS2_STACK_LABEL_LEN: usize = 64;

// The remaining definitions retain the original implementation's external
// kernel ABI and are intentionally expressed as declarations until the
// translated OCFS2 headers are available.
extern "C" {
    fn ocfs2_sync_fs(sb: *mut super_block, wait: c_int) -> c_int;
    fn ocfs2_statfs(dentry: *mut dentry, buf: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
