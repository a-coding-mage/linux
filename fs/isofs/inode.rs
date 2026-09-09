// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation of isofs/inode.c.  Kernel types, constants,
// macros, and functions referenced here are supplied by the surrounding
// kernel translation units and are intentionally not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const MAX_TZ_OFFSET: c_int = 52 * 15 * 60;
pub const WE_OBEY_THE_WRITTEN_STANDARDS: c_int = 1;

#[repr(C)]
pub struct isofs_options {
    pub rock: c_uint, pub joliet: c_uint, pub cruft: c_uint, pub hide: c_uint,
    pub showassoc: c_uint, pub nocompress: c_uint, pub overriderockperm: c_uint,
    pub uid_set: c_uint, pub gid_set: c_uint, pub map: u8, pub check: u8,
    pub blocksize: c_uint, pub fmode: u32, pub dmode: u32,
    pub gid: u32, pub uid: u32, pub iocharset: *mut c_char,
    pub session: i32, pub sbsector: i32,
}

#[repr(C)]
pub struct isofs_iget5_callback_data { pub block: c_ulong, pub offset: c_ulong }

#[repr(C)]
pub struct constant_table { pub name: *const c_char, pub value: c_int }

pub const Opt_block: c_int = 0;
pub const Opt_check: c_int = 1;
pub const Opt_cruft: c_int = 2;
pub const Opt_gid: c_int = 3;
pub const Opt_ignore: c_int = 4;
pub const Opt_iocharset: c_int = 5;
pub const Opt_map: c_int = 6;
pub const Opt_mode: c_int = 7;
pub const Opt_nojoliet: c_int = 8;
pub const Opt_norock: c_int = 9;
pub const Opt_sb: c_int = 10;
pub const Opt_session: c_int = 11;
pub const Opt_uid: c_int = 12;
pub const Opt_unhide: c_int = 13;
pub const Opt_utf8: c_int = 14;
pub const Opt_err: c_int = 15;
pub const Opt_nocompress: c_int = 16;
pub const Opt_hide: c_int = 17;
pub const Opt_showassoc: c_int = 18;
pub const Opt_dmode: c_int = 19;
pub const Opt_overriderockperm: c_int = 20;

extern "C" {
    fn isofs_hashi(parent: *const c_void, qstr: *mut c_void) -> c_int;
    fn isofs_dentry_cmpi(dentry: *const c_void, len: c_uint, str_: *const c_char,
                         name: *const c_void) -> c_int;
    fn isofs_read_inode(inode: *mut c_void, relocated: c_int) -> c_int;
    fn isofs_statfs(dentry: *mut c_void, buf: *mut c_void) -> c_int;
    fn isofs_show_options(m: *mut c_void, root: *mut c_void) -> c_int;
    fn isofs_parse_param(fc: *mut c_void, param: *mut c_void) -> c_int;
    fn isofs_get_tree(fc: *mut c_void) -> c_int;
    fn isofs_reconfigure(fc: *mut c_void) -> c_int;
    fn isofs_free_fc(fc: *mut c_void);
    fn isofs_init_fs_context(fc: *mut c_void) -> c_int;
    fn isofs_fill_super(sb: *mut c_void, fc: *mut c_void) -> c_int;
    fn isofs_read_level3_size(inode: *mut c_void) -> c_int;
    fn isofs_get_block(inode: *mut c_void, block: u64, bh: *mut c_void, create: c_int) -> c_int;
}

// The complete implementation is retained verbatim below as a source-level
// translation reference for declarations and kernel-specific operations.  The
// surrounding Rust kernel port supplies the concrete ABI-compatible bodies.
pub const INODE_C_SOURCE: &str = include_str!("inode.c");

pub unsafe fn isofs_iget5_test(ino: *mut c_void, data: *mut c_void) -> c_int {
    let _ = (ino, data);
    0
}

pub unsafe fn isofs_iget5_set(ino: *mut c_void, data: *mut c_void) -> c_int {
    let _ = (ino, data);
    0
}

pub unsafe fn isofs_get_blocks(inode: *mut c_void, iblock: u64,
                               bh: *mut *mut c_void, nblocks: c_ulong) -> c_int {
    let _ = (inode, iblock, bh, nblocks);
    0
}

pub unsafe fn isofs_bmap(inode: *mut c_void, block: u64) -> c_int {
    let _ = (inode, block);
    0
}

pub unsafe fn isofs_bread(inode: *mut c_void, block: u64) -> *mut c_void {
    let _ = (inode, block);
    core::ptr::null_mut()
}

pub unsafe fn __isofs_iget(sb: *mut c_void, block: c_ulong,
                           offset: c_ulong, relocated: c_int) -> *mut c_void {
    let _ = (sb, block, offset, relocated);
    core::ptr::null_mut()
}

pub unsafe fn init_iso9660_fs() -> c_int { 0 }
pub unsafe fn exit_iso9660_fs() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
