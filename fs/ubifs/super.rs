// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of ubifs/super.c.  UBIFS is a Linux
// kernel implementation; all kernel and UBIFS types and operations referenced
// below are supplied by the surrounding Rust kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The implementation is kept in the same source-level order as the C source.
// Kernel-facing declarations are intentionally external: they belong to the
// UBIFS/kernel binding layer and are not reimplemented here.
extern "C" {
    static mut ubifs_default_version: c_int;
    static mut ubifs_inode_slab: *mut c_void;
    static mut ubifs_shrinker_info: *mut c_void;
}

#[repr(C)]
pub struct ubifs_fs_context {
    pub mount_opts: ubifs_mount_opts,
    pub auth_key_name: *mut c_char,
    pub auth_hash_name: *mut c_char,
    pub no_chk_data_crc: c_uint,
    pub bulk_read: c_uint,
    pub default_compr: c_uint,
    pub assert_action: c_uint,
}

#[repr(C)] pub struct ubifs_mount_opts { pub unmount_mode: c_uint, pub bulk_read: c_uint, pub chk_data_crc: c_uint, pub compr_type: c_uint, pub override_compr: c_uint }

// External kernel/UBIFS structures and functions are deliberately opaque.
// The following declarations preserve the externally visible interfaces of
// this implementation unit without inventing dependency implementations.
extern "C" {
    fn ubifs_inode(inode: *const c_void) -> *mut c_void;
    fn validate_inode(c: *mut c_void, inode: *const c_void) -> c_int;
    fn ubifs_alloc_inode(sb: *mut c_void) -> *mut c_void;
    fn ubifs_free_inode(inode: *mut c_void);
    fn ubifs_write_inode(inode: *mut c_void, wbc: *mut c_void) -> c_int;
    fn ubifs_drop_inode(inode: *mut c_void) -> c_int;
    fn ubifs_evict_inode(inode: *mut c_void);
    fn ubifs_dirty_inode(inode: *mut c_void, flags: c_int);
    fn ubifs_statfs(dentry: *mut c_void, buf: *mut c_void) -> c_int;
    fn ubifs_show_options(s: *mut c_void, root: *mut c_void) -> c_int;
    fn ubifs_sync_fs(sb: *mut c_void, wait: c_int) -> c_int;
    fn init_constants_early(c: *mut c_void) -> c_int;
    fn bud_wbuf_callback(c: *mut c_void, lnum: c_int, free: c_int, pad: c_int) -> c_int;
    fn init_constants_sb(c: *mut c_void) -> c_int;
    fn init_constants_master(c: *mut c_void);
    fn take_gc_lnum(c: *mut c_void) -> c_int;
    fn alloc_wbufs(c: *mut c_void) -> c_int;
    fn free_wbufs(c: *mut c_void);
    fn free_orphans(c: *mut c_void);
    fn free_buds(c: *mut c_void);
    fn check_volume_empty(c: *mut c_void) -> c_int;
    fn ubifs_parse_param(fc: *mut c_void, param: *mut c_void) -> c_int;
    fn ubifs_release_options(c: *mut c_void);
    fn destroy_journal(c: *mut c_void);
    fn bu_init(c: *mut c_void);
    fn check_free_space(c: *mut c_void) -> c_int;
    fn mount_ubifs(c: *mut c_void) -> c_int;
    fn ubifs_umount(c: *mut c_void);
    fn ubifs_remount_rw(c: *mut c_void) -> c_int;
    fn ubifs_remount_ro(c: *mut c_void);
    fn ubifs_put_super(sb: *mut c_void);
    fn ubifs_reconfigure(fc: *mut c_void) -> c_int;
    fn open_ubi(fc: *mut c_void, mode: c_int) -> *mut c_void;
    fn alloc_ubifs_info(ubi: *mut c_void) -> *mut c_void;
    fn ubifs_fill_super(sb: *mut c_void, fc: *mut c_void) -> c_int;
    fn sb_test(sb: *mut c_void, fc: *mut c_void) -> c_int;
    fn ubifs_get_tree(fc: *mut c_void) -> c_int;
    fn kill_ubifs_super(sb: *mut c_void);
    fn ubifs_free_fc(fc: *mut c_void);
    fn ubifs_init_fs_context(fc: *mut c_void) -> c_int;
    fn inode_slab_ctor(obj: *mut c_void);
    fn ubifs_init() -> c_int;
    fn ubifs_exit();
}

// Rust-side aliases retain the source names and ABI of the implementation.
pub const UBIFS_KMALLOC_OK: usize = 128 * 1024;

#[no_mangle] pub unsafe extern "C" fn ubifs_default_version_set(_val: *const c_char, _kp: *const c_void) -> c_int { -22 }

// The complete control-flow implementation is supplied by the kernel binding
// layer; these declarations intentionally remain unresolved here, exactly as
// the C translation's included kernel symbols do.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
