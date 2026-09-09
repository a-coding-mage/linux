/* SPDX-License-Identifier: GPL-2.0 */
/*
  File: linux/xattr.h

  Extended attributes handling.

  Copyright (C) 2001 by Andreas Gruenbacher <a.gruenbacher@computer.org>
  Copyright (c) 2001-2002 Silicon Graphics, Inc.  All Rights Reserved.
  Copyright (c) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
*/

// Dependencies supplied by the corresponding Linux kernel headers:
// linux/slab.h, linux/types.h, linux/spinlock.h, linux/mm.h,
// linux/rhashtable-types.h, linux/user_namespace.h, and uapi/linux/xattr.h.

pub const XATTR_ARGS_SIZE_VER0: usize = 16; /* sizeof first published struct */
pub const XATTR_ARGS_SIZE_LATEST: usize = XATTR_ARGS_SIZE_VER0;

#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct dentry;

#[inline]
pub unsafe fn is_posix_acl_xattr(name: *const ::core::ffi::c_char) -> bool {
    (strcmp(name, XATTR_NAME_POSIX_ACL_ACCESS) == 0)
        || (strcmp(name, XATTR_NAME_POSIX_ACL_DEFAULT) == 0)
}

#[repr(C)]
pub struct xattr_handler {
    pub name: *const ::core::ffi::c_char,
    pub prefix: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int, /* fs private flags */
    pub list: Option<unsafe extern "C" fn(dentry: *mut dentry) -> bool>,
    pub get: Option<unsafe extern "C" fn(
        handler: *const xattr_handler,
        dentry: *mut dentry,
        inode: *mut inode,
        name: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_void,
        size: usize,
    ) -> ssize_t>,
    pub set: Option<unsafe extern "C" fn(
        handler: *const xattr_handler,
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        inode: *mut inode,
        name: *const ::core::ffi::c_char,
        buffer: *const ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
}

#[inline]
pub unsafe fn xattr_handler_can_list(
    handler: *const xattr_handler,
    dentry: *mut dentry,
) -> bool {
    !handler.is_null() && ((*handler).list.is_none() || ((*handler).list.unwrap())(dentry))
}

unsafe extern "C" {
    pub fn xattr_full_name(
        handler: *const xattr_handler,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}

#[repr(C)]
pub struct xattr {
    pub name: *const ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_void,
    pub value_len: usize,
}

unsafe extern "C" {
    pub fn __vfs_getxattr(dentry: *mut dentry, inode: *mut inode, name: *const ::core::ffi::c_char, buffer: *mut ::core::ffi::c_void, size: usize) -> ssize_t;
    pub fn vfs_getxattr(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, buffer: *mut ::core::ffi::c_void, size: usize) -> ssize_t;
    pub fn vfs_listxattr(d: *mut dentry, list: *mut ::core::ffi::c_char, size: usize) -> ssize_t;
    pub fn __vfs_setxattr(idmap: *mut mnt_idmap, dentry: *mut dentry, inode: *mut inode, name: *const ::core::ffi::c_char, buffer: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __vfs_setxattr_noperm(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, buffer: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __vfs_setxattr_locked(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, buffer: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int, delegated_inode: *mut delegated_inode) -> ::core::ffi::c_int;
    pub fn vfs_setxattr(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, buffer: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __vfs_removexattr(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn __vfs_removexattr_locked(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, delegated_inode: *mut delegated_inode) -> ::core::ffi::c_int;
    pub fn vfs_removexattr(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn generic_listxattr(dentry: *mut dentry, buffer: *mut ::core::ffi::c_char, buffer_size: usize) -> ssize_t;
    pub fn vfs_getxattr_alloc(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const ::core::ffi::c_char, xattr_value: *mut *mut ::core::ffi::c_char, size: usize, flags: gfp_t) -> ssize_t;
    pub fn xattr_supports_user_prefix(inode: *mut inode) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn xattr_prefix(handler: *const xattr_handler) -> *const ::core::ffi::c_char {
    if !(*handler).prefix.is_null() { (*handler).prefix } else { (*handler).name }
}

#[repr(C)]
pub struct simple_xattr_cache { pub ht: *mut rhashtable }

#[repr(C)]
pub struct simple_xattr {
    pub hash_node: rhash_head,
    pub parent: *mut list_head,
    pub node: list_head,
    pub rcu: rcu_head,
    pub name: *mut ::core::ffi::c_char,
    pub size: usize,
    pub value: [::core::ffi::c_char; 0],
}

pub const SIMPLE_XATTR_MAX_NR: usize = 128;
pub const SIMPLE_XATTR_MAX_SIZE: usize = 128usize << 10;

#[repr(C)]
pub struct simple_xattr_limits {
    pub nr_xattrs: atomic_t, /* current user.* xattr count */
    pub xattr_size: atomic_t, /* current total user.* value bytes */
}

#[inline]
pub unsafe fn simple_xattr_limits_init(limits: *mut simple_xattr_limits) {
    atomic_set(&mut (*limits).nr_xattrs, 0);
    atomic_set(&mut (*limits).xattr_size, 0);
}

unsafe extern "C" {
    pub fn simple_xattrs_free(cache: *mut simple_xattr_cache, xattrs: *mut list_head, freed_space: *mut usize);
    pub fn simple_xattr_space(name: *const ::core::ffi::c_char, size: usize) -> usize;
    pub fn simple_xattr_alloc(value: *const ::core::ffi::c_void, size: usize) -> *mut simple_xattr;
    pub fn simple_xattr_free(xattr: *mut simple_xattr);
    pub fn simple_xattr_free_rcu(xattr: *mut simple_xattr);
    pub fn simple_xattr_get(cache: *mut simple_xattr_cache, xattrs: *mut list_head, name: *const ::core::ffi::c_char, buffer: *mut ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int;
    pub fn simple_xattr_set(cache: *mut simple_xattr_cache, xattrs: *mut list_head, name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> *mut simple_xattr;
    pub fn simple_xattr_set_limited(cache: *mut simple_xattr_cache, xattrs: *mut list_head, limits: *mut simple_xattr_limits, name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn simple_xattr_list(inode: *mut inode, xattrs: *mut list_head, buffer: *mut ::core::ffi::c_char, size: usize) -> ssize_t;
    pub fn simple_xattr_add(cache: *mut simple_xattr_cache, xattrs: *mut list_head, new_xattr: *mut simple_xattr) -> ::core::ffi::c_int;
    pub fn simple_xattr_add_limited(cache: *mut simple_xattr_cache, xattrs: *mut list_head, limits: *mut simple_xattr_limits, new_xattr: *mut simple_xattr) -> ::core::ffi::c_int;
    pub fn xattr_list_one(buffer: *mut *mut ::core::ffi::c_char, remaining_size: *mut ssize_t, name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn simple_xattr_cache_cleanup(cache: *mut simple_xattr_cache);
}

// The kernel DEFINE_CLASS(simple_xattr, ...) declaration is retained as the
// following source-level intent; its helper types and cleanup machinery are
// supplied by the surrounding kernel translation.
// DEFINE_CLASS(simple_xattr, struct simple_xattr *,
//     if (!IS_ERR_OR_NULL(_T)) simple_xattr_free(_T),
//     simple_xattr_alloc(value, size), const void *value, size_t size)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
