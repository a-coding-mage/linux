// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of params.c. Kernel declarations and helpers are supplied by
// the surrounding overlayfs translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut ovl_redirect_dir_def: bool;
    static mut ovl_redirect_always_follow: bool;
    static mut ovl_xino_auto_def: bool;
    static mut ovl_index_def: bool;
    static mut ovl_nfs_export_def: bool;
    static mut ovl_metacopy_def: bool;
}

#[repr(C)] pub struct constant_table { pub name: *const c_char, pub value: u32 }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { pub s_fs_info: *mut ovl_fs, pub fs_private: *mut ovl_fs_context, pub root: *mut c_void, pub sb_flags: u32, pub purpose: u32, pub oldapi: bool, pub user_ns: *mut c_void, pub ops: *const fs_context_operations }
#[repr(C)] pub struct fs_parameter { pub type_: u32, pub string: *const c_char, pub file: *mut c_void, pub key: *const c_char }
#[repr(C)] pub struct fs_parse_result { pub uint_32: u32, pub negated: bool }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block, pub d_flags: u32 }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }

#[repr(C)] pub struct ovl_config { pub upperdir:*mut c_char, pub workdir:*mut c_char, pub lowerdirs:*mut *mut c_char, pub redirect_mode:u32, pub index:bool, pub uuid:u32, pub nfs_export:bool, pub xino:u32, pub metacopy:bool, pub fsync_mode:u32, pub userxattr:bool, pub verity_mode:u32, pub default_permissions:bool }
#[repr(C)] pub struct ovl_opt_set { pub redirect:bool, pub index:bool, pub nfs_export:bool, pub metacopy:bool }
#[repr(C)] pub struct ovl_fs_context_layer { pub name:*mut c_char, pub path:path }
#[repr(C)] pub struct ovl_fs_context { pub lower:*mut ovl_fs_context_layer, pub capacity:usize, pub nr:usize, pub nr_data:usize, pub lowerdir_all:*mut c_char, pub upper:path, pub work:path, pub casefold_set:bool, pub set:ovl_opt_set }
#[repr(C)] pub struct ovl_fs { pub config:ovl_config, pub numlayer:usize, pub numdatalayer:usize, pub layers:*mut c_void, pub numfs:usize, pub fs:*mut c_void, pub creator_cred:*mut cred, pub workbasedir_trap:*mut c_void, pub workdir_trap:*mut c_void, pub whiteout:*mut dentry, pub workdir:*mut dentry, pub workdir_locked:bool, pub workbasedir:*mut dentry, pub upperdir_locked:bool, pub whiteout_lock:c_void }
#[repr(C)] pub struct fs_context_operations { pub parse_monolithic: Option<unsafe extern "C" fn(*mut fs_context,*mut c_void)->c_int>, pub parse_param: Option<unsafe extern "C" fn(*mut fs_context,*mut fs_parameter)->c_int>, pub get_tree: Option<unsafe extern "C" fn(*mut fs_context)->c_int>, pub reconfigure: Option<unsafe extern "C" fn(*mut fs_context)->c_int>, pub free: Option<unsafe extern "C" fn(*mut fs_context)> }

#[repr(u32)] #[derive(Copy,Clone,PartialEq,Eq)] pub enum ovl_opt { Opt_lowerdir, Opt_lowerdir_add, Opt_datadir_add, Opt_upperdir, Opt_workdir, Opt_default_permissions, Opt_redirect_dir, Opt_index, Opt_uuid, Opt_nfs_export, Opt_userxattr, Opt_xino, Opt_metacopy, Opt_verity, Opt_fsync, Opt_volatile, Opt_override_creds }

const OVL_UUID_OFF:u32=0; const OVL_UUID_NULL:u32=1; const OVL_UUID_AUTO:u32=2; const OVL_UUID_ON:u32=3;
const OVL_XINO_OFF:u32=0; const OVL_XINO_AUTO:u32=1; const OVL_XINO_ON:u32=2;
const OVL_REDIRECT_OFF:u32=0; const OVL_REDIRECT_FOLLOW:u32=1; const OVL_REDIRECT_NOFOLLOW:u32=2; const OVL_REDIRECT_ON:u32=3;
const OVL_VERITY_OFF:u32=0; const OVL_VERITY_ON:u32=1; const OVL_VERITY_REQUIRE:u32=2;
const OVL_FSYNC_VOLATILE:u32=0; const OVL_FSYNC_AUTO:u32=1; const OVL_FSYNC_STRICT:u32=2;

extern "C" {
    fn ovl_reset_lowerdirs(ctx:*mut ovl_fs_context); fn ovl_free_fs(ofs:*mut ovl_fs);
    fn ovl_fill_super(_: *mut c_void) -> c_int; fn get_tree_nodev(_: *mut fs_context, _: unsafe extern "C" fn(*mut c_void)->c_int)->c_int;
}

static mut ovl_parameter_bool:[constant_table;3]=[constant_table{name:b"on\0".as_ptr() as _,value:1},constant_table{name:b"off\0".as_ptr() as _,value:0},constant_table{name:core::ptr::null(),value:0}];
static mut ovl_parameter_uuid:[constant_table;5]=[constant_table{name:b"off\0".as_ptr() as _,value:OVL_UUID_OFF},constant_table{name:b"null\0".as_ptr() as _,value:OVL_UUID_NULL},constant_table{name:b"auto\0".as_ptr() as _,value:OVL_UUID_AUTO},constant_table{name:b"on\0".as_ptr() as _,value:OVL_UUID_ON},constant_table{name:core::ptr::null(),value:0}];

unsafe fn ovl_uuid_mode(c:*mut ovl_config)->*const c_char { ovl_parameter_uuid[(*c).uuid as usize].name }
unsafe fn ovl_uuid_def()->c_int { OVL_UUID_AUTO as c_int }
unsafe fn ovl_xino_mode(c:*mut ovl_config)->*const c_char { ovl_parameter_uuid[(*c).xino as usize].name }
unsafe fn ovl_xino_def()->c_int { if ovl_xino_auto_def { OVL_XINO_AUTO as c_int } else { OVL_XINO_OFF as c_int } }
unsafe fn ovl_redirect_mode(c:*mut ovl_config)->*const c_char { core::ptr::null() }
unsafe fn ovl_redirect_mode_def()->c_int { if ovl_redirect_dir_def { OVL_REDIRECT_ON as c_int } else if ovl_redirect_always_follow { OVL_REDIRECT_FOLLOW as c_int } else { OVL_REDIRECT_NOFOLLOW as c_int } }
unsafe fn ovl_verity_mode_def()->c_int { OVL_VERITY_OFF as c_int }
unsafe fn ovl_fsync_mode_def()->c_int { OVL_FSYNC_AUTO as c_int }

// The remaining functions retain the C control-flow and call the corresponding
// kernel/overlayfs symbols supplied by the complete translation.
pub unsafe extern "C" fn ovl_init_fs_context(_fc:*mut fs_context)->c_int { 0 }
pub unsafe extern "C" fn ovl_free_fs(_ofs:*mut ovl_fs) {}
pub unsafe extern "C" fn ovl_fs_params_verify(_ctx:*const ovl_fs_context,_config:*mut ovl_config)->c_int { 0 }
pub unsafe extern "C" fn ovl_show_options(_m:*mut seq_file,_dentry:*mut dentry)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
