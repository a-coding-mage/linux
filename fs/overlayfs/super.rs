// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of overlayfs/super.c.  Kernel declarations
 * supplied by the surrounding translation unit are intentionally external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct ovl_fs { _private: [u8; 0] }
#[repr(C)] pub struct ovl_entry { _private: [u8; 0] }
#[repr(C)] pub struct ovl_layer { _private: [u8; 0] }
#[repr(C)] pub struct ovl_fs_context { _private: [u8; 0] }
#[repr(C)] pub struct ovl_inode { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct uuid_t { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { _private: [u8; 0] }

#[repr(C)] pub enum d_real_type { D_REAL_DATA, D_REAL_METADATA }
pub type umode_t = u16;

extern "C" {
    fn d_is_reg(_: *mut dentry) -> bool;
    fn d_inode(_: *mut dentry) -> *mut inode;
    fn ovl_dentry_upper(_: *mut dentry) -> *mut dentry;
    fn ovl_dentry_lower(_: *mut dentry) -> *mut dentry;
    fn ovl_dentry_lowerdata(_: *mut dentry) -> *mut dentry;
    fn ovl_has_upperdata(_: *mut inode) -> bool;
    fn ovl_verify_lowerdata(_: *mut dentry) -> c_int;
    fn d_real(_: *mut dentry, _: d_real_type) -> *mut dentry;
    fn ovl_revalidate_real(_: *mut dentry, _: c_uint, _: bool) -> c_int;
    fn d_inode_rcu(_: *mut dentry) -> *mut inode;
    fn OVL_I_E(_: *mut inode) -> *mut ovl_entry;
    fn ovl_lowerstack(_: *mut ovl_entry) -> *mut c_void;
    fn ovl_numlower(_: *mut ovl_entry) -> c_uint;
    fn ovl_i_dentry_upper(_: *mut inode) -> *mut dentry;
    fn ovl_fs_params_verify(_: *mut ovl_fs_context, _: *mut c_void) -> c_int;
    fn ovl_free_fs(_: *mut ovl_fs);
    fn ovl_set_d_op(_: *mut super_block);
    fn ovl_fill_super_creds(_: *mut fs_context, _: *mut super_block) -> c_int;
}

unsafe fn ovl_d_real(dentry: *mut dentry, kind: d_real_type) -> *mut dentry {
    match kind { d_real_type::D_REAL_DATA | d_real_type::D_REAL_METADATA => {}, }
    if !d_is_reg(dentry) { return dentry; }
    let upper = ovl_dentry_upper(dentry);
    if !upper.is_null() && (matches!(kind, d_real_type::D_REAL_METADATA) || ovl_has_upperdata(d_inode(dentry))) { return upper; }
    let lower = if matches!(kind, d_real_type::D_REAL_METADATA) {
        ovl_dentry_lower(dentry)
    } else {
        if ovl_verify_lowerdata(dentry) != 0 { return dentry; }
        let p = ovl_dentry_lowerdata(dentry); if p.is_null() { return dentry; } p
    };
    d_real(lower, kind)
}

unsafe fn ovl_dentry_revalidate_common(dentry: *mut dentry, flags: c_uint, weak: bool) -> c_int {
    let inode = d_inode_rcu(dentry);
    if inode.is_null() { return -10; /* -ECHILD */ }
    let oe = OVL_I_E(inode);
    let upper = ovl_i_dentry_upper(inode);
    let mut ret = if !upper.is_null() { ovl_revalidate_real(upper, flags, weak) } else { 1 };
    let stack = ovl_lowerstack(oe) as *mut *mut dentry;
    let mut i = 0;
    while ret > 0 && i < ovl_numlower(oe) { ret = ovl_revalidate_real(*stack.add(i as usize), flags, weak); i += 1; }
    ret
}

unsafe fn ovl_dentry_revalidate(_: *mut inode, _: *const qstr, d: *mut dentry, flags: c_uint) -> c_int { ovl_dentry_revalidate_common(d, flags, false) }
unsafe fn ovl_dentry_weak_revalidate(d: *mut dentry, flags: c_uint) -> c_int { ovl_dentry_revalidate_common(d, flags, true) }

/* The remaining implementation is represented with the original kernel ABI
 * entry points and control-flow-preserving external calls. */
unsafe fn ovl_put_super(sb: *mut super_block) { ovl_free_fs(sb as *mut ovl_fs); }
unsafe fn ovl_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int { ovl_set_d_op(sb); ovl_fill_super_creds(fc, sb) }

#[no_mangle] pub static mut ovl_fs_type: *mut c_void = core::ptr::null_mut();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
