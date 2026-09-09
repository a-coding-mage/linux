// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2024 Oracle.
 * All Rights Reserved.
 */

/* Metadata validators */
extern "C" {
    pub fn xfs_parent_namecheck(attr_flags: ::std::os::raw::c_uint, name: *const ::std::ffi::c_void,
        length: usize) -> bool;
    pub fn xfs_parent_valuecheck(mp: *mut xfs_mount, value: *const ::std::ffi::c_void,
        valuelen: usize) -> bool;

    pub fn xfs_parent_hashval(mp: *mut xfs_mount, name: *const u8,
        namelen: ::std::os::raw::c_int, parent_ino: xfs_ino_t) -> xfs_dahash_t;
    pub fn xfs_parent_hashattr(mp: *mut xfs_mount, name: *const u8,
        namelen: ::std::os::raw::c_int, value: *const ::std::ffi::c_void,
        valuelen: ::std::os::raw::c_int) -> xfs_dahash_t;

    pub static mut xfs_parent_args_cache: *mut kmem_cache;

    pub fn xfs_parent_addname(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args,
        dp: *mut xfs_inode, parent_name: *const xfs_name,
        child: *mut xfs_inode) -> ::std::os::raw::c_int;
    pub fn xfs_parent_removename(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args,
        dp: *mut xfs_inode, parent_name: *const xfs_name,
        child: *mut xfs_inode) -> ::std::os::raw::c_int;
    pub fn xfs_parent_replacename(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args,
        old_dp: *mut xfs_inode, old_name: *const xfs_name,
        new_dp: *mut xfs_inode, new_name: *const xfs_name,
        child: *mut xfs_inode) -> ::std::os::raw::c_int;

    pub fn xfs_parent_from_attr(mp: *mut xfs_mount, attr_flags: ::std::os::raw::c_uint,
        name: *const u8, namelen: ::std::os::raw::c_uint,
        value: *const ::std::ffi::c_void, valuelen: ::std::os::raw::c_uint,
        parent_ino: *mut xfs_ino_t, parent_gen: *mut u32) -> ::std::os::raw::c_int;

    /* Repair functions */
    pub fn xfs_parent_lookup(tp: *mut xfs_trans, ip: *mut xfs_inode,
        name: *const xfs_name, pptr: *mut xfs_parent_rec,
        scratch: *mut xfs_da_args) -> ::std::os::raw::c_int;
    pub fn xfs_parent_set(ip: *mut xfs_inode, owner: xfs_ino_t,
        name: *const xfs_name, pptr: *mut xfs_parent_rec,
        scratch: *mut xfs_da_args) -> ::std::os::raw::c_int;
    pub fn xfs_parent_unset(ip: *mut xfs_inode, owner: xfs_ino_t,
        name: *const xfs_name, pptr: *mut xfs_parent_rec,
        scratch: *mut xfs_da_args) -> ::std::os::raw::c_int;
}

/* Initializes a xfs_parent_rec to be stored as an attribute name. */
#[inline]
pub unsafe fn xfs_parent_rec_init(rec: *mut xfs_parent_rec, ino: xfs_ino_t, gen: u32) {
    (*rec).p_ino = ino.to_be();
    (*rec).p_gen = gen.to_be();
}

/* Initializes a xfs_parent_rec to be stored as an attribute name. */
#[inline]
pub unsafe fn xfs_inode_to_parent_rec(rec: *mut xfs_parent_rec, dp: *const xfs_inode) {
    xfs_parent_rec_init(rec, I_INO(dp), (*VFS_IC(dp)).i_generation);
}

/*
 * Parent pointer information needed to pass around the deferred xattr update
 * machinery.
 */
#[repr(C)]
pub struct xfs_parent_args {
    pub rec: xfs_parent_rec,
    pub new_rec: xfs_parent_rec,
    pub args: xfs_da_args,
}

/*
 * Start a parent pointer update by allocating the context object we need to
 * perform a parent pointer update.
 */
#[inline]
pub unsafe fn xfs_parent_start(mp: *mut xfs_mount, ppargsp: *mut *mut xfs_parent_args) -> ::std::os::raw::c_int {
    if !xfs_has_parent(mp) {
        *ppargsp = ::std::ptr::null_mut();
        return 0;
    }
    *ppargsp = kmem_cache_zalloc(xfs_parent_args_cache, GFP_KERNEL);
    if (*ppargsp).is_null() { return -ENOMEM; }
    0
}

/* Finish a parent pointer update by freeing the context object. */
#[inline]
pub unsafe fn xfs_parent_finish(_mp: *mut xfs_mount, ppargs: *mut xfs_parent_args) {
    if !ppargs.is_null() { kmem_cache_free(xfs_parent_args_cache, ppargs as *mut _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
