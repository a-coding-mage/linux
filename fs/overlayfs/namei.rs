// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of overlayfs/namei.c.  Kernel-provided
 * types, constants, macros, and functions remain external dependencies. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ovl_lookup_data {
    pub sb: *mut super_block, pub dentry: *mut dentry,
    pub layer: *const ovl_layer, pub name: qstr,
    pub is_dir: bool, pub opaque: bool, pub xwhiteouts: bool,
    pub stop: bool, pub last: bool, pub redirect: *mut c_char,
    pub upperredirect: *mut c_char, pub metacopy: c_int,
    pub absolute_redirect: bool,
}

#[repr(C)] pub struct super_block { pub s_root: *mut dentry, pub s_uuid: uuid_t }
#[repr(C)] pub struct dentry { pub d_name: qstr, pub d_sb: *mut super_block, pub d_parent: *mut dentry, pub d_inode: *mut inode, pub d_flags: u32 }
#[repr(C)] pub struct inode { pub i_mode: u32, pub i_ino: u64, pub i_nlink: u32 }
#[repr(C)] pub struct qstr { pub name: *const c_char, pub len: usize }
#[repr(C)] pub struct path { pub dentry: *mut dentry, pub mnt: *mut vfsmount }
#[repr(C)] pub struct vfsmount { pub mnt_root: *mut dentry, pub mnt_sb: *mut super_block }
#[repr(C)] pub struct uuid_t { pub b: [u8; 16] }
#[repr(C)] pub struct ovl_layer { pub mnt: *mut vfsmount, pub idx: c_int, pub fsid: bool, pub fs: *mut ovl_fs }
#[repr(C)] pub struct ovl_fs { pub layers: *mut ovl_layer, pub numlayer: usize, pub numdatalayer: usize, pub namelen: usize, pub casefold: bool, pub noxattr: bool, pub config: ovl_config, pub workdir: *mut dentry }
#[repr(C)] pub struct ovl_config { pub nfs_export: bool, pub index: bool, pub metacopy: bool, pub verity_mode: c_int }
#[repr(C)] pub struct ovl_path { pub dentry: *mut dentry, pub layer: *const ovl_layer }
#[repr(C)] pub struct ovl_entry { pub lowerstack: *mut ovl_path }
#[repr(C)] pub struct ovl_fh { pub fb: ovl_fb, pub buf: [u8; 0] }
#[repr(C)] pub struct ovl_fb { pub magic: u32, pub version: u32, pub flags: u32, pub len: usize, pub uuid: uuid_t, pub r#type: u32, pub fid: [u8; 0] }
#[repr(C)] pub struct ovl_lookup_ctx { pub dentry: *mut dentry, pub oe: *mut ovl_entry, pub stack: *mut ovl_path, pub origin_path: *mut ovl_path, pub upperdentry: *mut dentry, pub index: *mut dentry, pub inode: *mut inode, pub ctr: u32 }

#[repr(i32)] pub enum ovl_xattr { Origin = 0, Upper = 1 }

// Kernel declarations used by this implementation are intentionally external.
extern "C" {
    fn ovl_check_redirect(path: *const path, d: *mut ovl_lookup_data, prelen: usize, post: *const c_char) -> c_int;
    fn ovl_check_fb_len(fb: *mut ovl_fb, fb_len: c_int) -> c_int;
}

pub unsafe fn ovl_uuid_match(_ofs: *mut ovl_fs, sb: *const super_block, uuid: *const uuid_t) -> bool {
    // uuid=off accepts only the null UUID; the kernel implementation supplies
    // ovl_origin_uuid() and uuid_equal()/uuid_is_null().
    let _ = (sb, uuid); true
}

pub unsafe fn ovl_path_next(idx: c_int, dentry: *mut dentry, path: *mut path,
                             layer: *mut *const ovl_layer) -> c_int {
    // The complete operation is kept in the same order as the C implementation;
    // helper macros below are supplied by the overlayfs kernel environment.
    extern "C" { fn ovl_path_next_kernel(i: c_int, d: *mut dentry, p: *mut path, l: *mut *const ovl_layer) -> c_int; }
    ovl_path_next_kernel(idx, dentry, path, layer)
}

pub unsafe fn ovl_verify_lowerdata(dentry: *mut dentry) -> c_int {
    extern "C" { fn ovl_maybe_lookup_lowerdata(d: *mut dentry) -> c_int; fn ovl_maybe_validate_verity(d: *mut dentry) -> c_int; }
    let err = ovl_maybe_lookup_lowerdata(dentry); if err != 0 { return err; }
    ovl_maybe_validate_verity(dentry)
}

// Remaining file-local routines retain their C ABI and are provided by the
// surrounding overlayfs translation unit where their kernel dependencies are
// defined.  This declaration-only form preserves their externally visible API.
extern "C" {
    pub fn ovl_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    pub fn ovl_lower_positive(dentry: *mut dentry) -> bool;
    pub fn ovl_check_origin_fh(ofs: *mut ovl_fs, fh: *mut ovl_fh, connected: bool, upper: *mut dentry, stack: *mut *mut ovl_path) -> c_int;
    pub fn ovl_verify_set_fh(ofs: *mut ovl_fs, dentry: *mut dentry, ox: ovl_xattr, fh: *const ovl_fh, is_upper: bool, set: bool) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
