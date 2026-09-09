/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Filesystem superblock creation and reconfiguration context. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the surrounding kernel translation.
pub enum cred {}
pub enum dentry {}
pub enum file_operations {}
pub enum file_system_type {}
pub enum mnt_namespace {}
pub enum net {}
pub enum pid_namespace {}
pub enum super_block {}
pub enum user_namespace {}
pub enum vfsmount {}
pub enum path {}
pub enum filename {}
pub enum file {}
pub enum qstr {}
pub enum mutex {}
pub enum refcount_t {}
pub enum module {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fs_context_purpose {
    FS_CONTEXT_FOR_MOUNT,
    FS_CONTEXT_FOR_SUBMOUNT,
    FS_CONTEXT_FOR_RECONFIGURE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fs_context_phase {
    FS_CONTEXT_CREATE_PARAMS,
    FS_CONTEXT_CREATING,
    FS_CONTEXT_AWAITING_MOUNT,
    FS_CONTEXT_AWAITING_RECONF,
    FS_CONTEXT_RECONF_PARAMS,
    FS_CONTEXT_RECONFIGURING,
    FS_CONTEXT_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fs_value_type {
    fs_value_is_undefined,
    fs_value_is_flag,
    fs_value_is_string,
    fs_value_is_blob,
    fs_value_is_filename,
    fs_value_is_file,
}

#[repr(C)]
pub union fs_parameter_value {
    pub string: *mut c_char,
    pub blob: *mut c_void,
    pub name: *mut filename,
    pub file: *mut file,
}

#[repr(C)]
pub struct fs_parameter {
    pub key: *const c_char,
    pub type_: fs_value_type,
    pub value: fs_parameter_value,
    pub size: usize,
    pub dirfd: c_int,
}

#[repr(C)]
pub struct p_log {
    pub prefix: *const c_char,
    pub log: *mut fc_log,
}

#[repr(C)]
pub struct fs_context {
    pub ops: *const fs_context_operations,
    pub uapi_mutex: mutex,
    pub fs_type: *mut file_system_type,
    pub fs_private: *mut c_void,
    pub sget_key: *mut c_void,
    pub root: *mut dentry,
    pub user_ns: *mut user_namespace,
    pub net_ns: *mut net,
    pub cred: *const cred,
    pub log: p_log,
    pub source: *const c_char,
    pub security: *mut c_void,
    pub s_fs_info: *mut c_void,
    pub sb_flags: c_uint,
    pub sb_flags_mask: c_uint,
    pub s_iflags: c_uint,
    pub purpose: fs_context_purpose,
    pub phase: fs_context_phase,
    pub need_free: bool,
    pub global: bool,
    pub oldapi: bool,
    pub exclusive: bool,
}

#[repr(C)]
pub struct fs_context_operations {
    pub free: Option<unsafe extern "C" fn(fc: *mut fs_context)>,
    pub dup: Option<unsafe extern "C" fn(fc: *mut fs_context, src_fc: *mut fs_context) -> c_int>,
    pub parse_param: Option<unsafe extern "C" fn(fc: *mut fs_context, param: *mut fs_parameter) -> c_int>,
    pub parse_monolithic: Option<unsafe extern "C" fn(fc: *mut fs_context, data: *mut c_void) -> c_int>,
    pub get_tree: Option<unsafe extern "C" fn(fc: *mut fs_context) -> c_int>,
    pub reconfigure: Option<unsafe extern "C" fn(fc: *mut fs_context) -> c_int>,
}

extern "C" {
    pub fn fs_context_for_mount(fs_type: *mut file_system_type, sb_flags: c_uint) -> *mut fs_context;
    pub fn fs_context_for_reconfigure(dentry: *mut dentry, sb_flags: c_uint, sb_flags_mask: c_uint) -> *mut fs_context;
    pub fn fs_context_for_submount(fs_type: *mut file_system_type, reference: *mut dentry) -> *mut fs_context;
    pub fn vfs_dup_fs_context(fc: *mut fs_context) -> *mut fs_context;
    pub fn vfs_parse_fs_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    pub fn vfs_parse_fs_qstr(fc: *mut fs_context, key: *const c_char, value: *const qstr) -> c_int;
    pub fn vfs_parse_monolithic_sep(fc: *mut fs_context, data: *mut c_void, sep: Option<unsafe extern "C" fn(*mut *mut c_char) -> *mut c_char>) -> c_int;
    pub fn generic_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int;
    pub fn vfs_get_tree(fc: *mut fs_context) -> c_int;
    pub fn put_fs_context(fc: *mut fs_context);
    pub fn vfs_parse_fs_param_source(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
    pub fn fc_drop_locked(fc: *mut fs_context);
    pub fn get_tree_nodev(fc: *mut fs_context, fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>) -> c_int;
    pub fn get_tree_single(fc: *mut fs_context, fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>) -> c_int;
    pub fn get_tree_keyed(fc: *mut fs_context, fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>, key: *mut c_void) -> c_int;
    pub fn setup_bdev_super(sb: *mut super_block, sb_flags: c_int, fc: *mut fs_context) -> c_int;
    pub fn get_tree_bdev_flags(fc: *mut fs_context, fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>, flags: c_uint) -> c_int;
    pub fn get_tree_bdev(fc: *mut fs_context, fill_super: Option<unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int>) -> c_int;
    pub static fscontext_fops: file_operations;
    pub fn logfc(log: *mut fc_log, prefix: *const c_char, level: c_char, fmt: *const c_char, ...);
}

pub const GET_TREE_BDEV_QUIET_LOOKUP: c_uint = 0x0001;

#[repr(C)]
pub struct fc_log {
    pub usage: refcount_t,
    pub head: u8,
    pub tail: u8,
    pub need_free: u8,
    pub owner: *mut module,
    pub buffer: [*mut c_char; 8],
}

#[macro_export]
macro_rules! __logfc { ($fc:expr, $l:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::logfc((*$fc).log.log, core::ptr::null(), $l, $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! __plogp { ($p:expr, $prefix:expr, $l:expr, $fmt:expr $(, $arg:expr)*) => { unsafe { $crate::logfc((*$p).log, $prefix, $l, $fmt $(, $arg)*); } }; }
#[macro_export]
macro_rules! __plog { ($p:expr, $l:expr, $fmt:expr $(, $arg:expr)*) => { $crate::__plogp!($p, (*$p).prefix, $l, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! infof { ($fc:expr, $fmt:expr $(, $arg:expr)*) => { $crate::__logfc!($fc, b'i' as c_char, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! warnf { ($fc:expr, $fmt:expr $(, $arg:expr)*) => { $crate::__logfc!($fc, b'w' as c_char, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! errorf { ($fc:expr, $fmt:expr $(, $arg:expr)*) => { $crate::__logfc!($fc, b'e' as c_char, $fmt $(, $arg)*) }; }
#[macro_export]
macro_rules! invalf { ($fc:expr, $fmt:expr $(, $arg:expr)*) => {{ $crate::errorf!($fc, $fmt $(, $arg)*); -22 }}; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
