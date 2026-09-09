/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2006  NEC Corporation
 *
 * Created by KaiGai Kohei <kaigai@ak.jp.nec.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const JFFS2_XFLAGS_HOT: u8 = 0x01; // This datum is HOT
pub const JFFS2_XFLAGS_BIND: u8 = 0x02; // This datum is not reclaimed
pub const JFFS2_XFLAGS_DEAD: u8 = 0x40; // This datum is already dead
pub const JFFS2_XFLAGS_INVALID: u8 = 0x80; // This datum contains crc error

#[repr(C)]
pub struct jffs2_xattr_datum {
    pub always_null: *mut core::ffi::c_void,
    pub node: *mut jffs2_raw_node_ref,
    pub class: u8,
    pub flags: u8,
    pub xprefix: u16, // see JFFS2_XATTR_PREFIX_*
    pub xindex: list_head, // chained from c->xattrindex[n]
    pub refcnt: atomic_t, // # of xattr_ref refers this
    pub xid: u32,
    pub version: u32,
    pub data_crc: u32,
    pub hashkey: u32,
    pub xname: *mut core::ffi::c_char, // XATTR name without prefix
    pub name_len: u32, // length of xname
    pub xvalue: *mut core::ffi::c_char, // XATTR value
    pub value_len: u32, // length of xvalue
}

#[repr(C)]
pub struct jffs2_xattr_ref {
    pub always_null: *mut core::ffi::c_void,
    pub node: *mut jffs2_raw_node_ref,
    pub class: u8,
    pub flags: u8, // Currently unused
    pub unused: u16,
    pub xseqno: u32,
    pub target: jffs2_xattr_ref_target,
    pub datum: jffs2_xattr_ref_datum,
    pub next: *mut jffs2_xattr_ref, // chained from ic->xref_list
}

#[repr(C)]
pub union jffs2_xattr_ref_target {
    pub ic: *mut jffs2_inode_cache, // reference to jffs2_inode_cache
    pub ino: u32, // only used in scanning/building
}

#[repr(C)]
pub union jffs2_xattr_ref_datum {
    pub xd: *mut jffs2_xattr_datum, // reference to jffs2_xattr_datum
    pub xid: u32, // only used in sccanning/building
}

pub const XREF_DELETE_MARKER: u32 = 0x00000001;

#[inline]
pub unsafe fn is_xattr_ref_dead(ref_: *mut jffs2_xattr_ref) -> i32 {
    ((*ref_).xseqno & XREF_DELETE_MARKER != 0) as i32
}

// CONFIG_JFFS2_FS_XATTR declarations are enabled when the corresponding
// build-time kernel configuration is selected.
#[cfg(feature = "CONFIG_JFFS2_FS_XATTR")]
extern "C" {
    pub fn jffs2_init_xattr_subsystem(c: *mut jffs2_sb_info);
    pub fn jffs2_build_xattr_subsystem(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_clear_xattr_subsystem(c: *mut jffs2_sb_info);
    pub fn jffs2_setup_xattr_datum(c: *mut jffs2_sb_info, xid: u32, version: u32) -> *mut jffs2_xattr_datum;
    pub fn jffs2_xattr_do_crccheck_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
    pub fn jffs2_xattr_delete_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
    pub fn jffs2_xattr_free_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
    pub fn jffs2_garbage_collect_xattr_datum(c: *mut jffs2_sb_info, xd: *mut jffs2_xattr_datum, raw: *mut jffs2_raw_node_ref) -> i32;
    pub fn jffs2_garbage_collect_xattr_ref(c: *mut jffs2_sb_info, ref_: *mut jffs2_xattr_ref, raw: *mut jffs2_raw_node_ref) -> i32;
    pub fn jffs2_verify_xattr(c: *mut jffs2_sb_info) -> i32;
    pub fn jffs2_release_xattr_datum(c: *mut jffs2_sb_info, xd: *mut jffs2_xattr_datum);
    pub fn jffs2_release_xattr_ref(c: *mut jffs2_sb_info, ref_: *mut jffs2_xattr_ref);
    pub fn do_jffs2_getxattr(inode: *mut inode, xprefix: i32, xname: *const core::ffi::c_char, buffer: *mut core::ffi::c_char, size: usize) -> i32;
    pub fn do_jffs2_setxattr(inode: *mut inode, xprefix: i32, xname: *const core::ffi::c_char, buffer: *const core::ffi::c_char, size: usize, flags: i32) -> i32;
    pub static jffs2_xattr_handlers: *const *const xattr_handler;
    pub static jffs2_user_xattr_handler: xattr_handler;
    pub static jffs2_trusted_xattr_handler: xattr_handler;
    pub fn jffs2_listxattr(dentry: *mut dentry, buffer: *mut core::ffi::c_char, size: usize) -> isize;
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_XATTR"))]
pub const jffs2_xattr_handlers: *const *const xattr_handler = core::ptr::null();
#[cfg(not(feature = "CONFIG_JFFS2_FS_XATTR"))]
pub const jffs2_listxattr: Option<unsafe extern "C" fn(*mut dentry, *mut core::ffi::c_char, usize) -> isize> = None;

#[cfg(feature = "CONFIG_JFFS2_FS_SECURITY")]
extern "C" {
    pub fn jffs2_init_security(inode: *mut inode, dir: *mut inode, qstr: *const qstr) -> i32;
    pub static jffs2_security_xattr_handler: xattr_handler;
}

#[cfg(not(feature = "CONFIG_JFFS2_FS_SECURITY"))]
#[inline]
pub unsafe fn jffs2_init_security(_inode: *mut inode, _dir: *mut inode, _qstr: *const qstr) -> i32 { 0 }

// Opaque types and kernel-provided layout types.
pub enum jffs2_raw_node_ref {}
pub enum jffs2_inode_cache {}
pub enum jffs2_sb_info {}
pub enum inode {}
pub enum dentry {}
pub enum qstr {}
pub enum xattr_handler {}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
