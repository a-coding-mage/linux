/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * xattr.h
 *
 * Copyright (C) 2004, 2008 Oracle.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum ocfs2_xattr_type {
    OCFS2_XATTR_INDEX_USER = 1,
    OCFS2_XATTR_INDEX_POSIX_ACL_ACCESS,
    OCFS2_XATTR_INDEX_POSIX_ACL_DEFAULT,
    OCFS2_XATTR_INDEX_TRUSTED,
    OCFS2_XATTR_INDEX_SECURITY,
    OCFS2_XATTR_MAX,
}

#[repr(C)]
pub struct ocfs2_security_xattr_info {
    pub enable: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_void,
    pub value_len: usize,
}

#[repr(C)]
pub struct ocfs2_acl_state;

extern "C" {
    pub static ocfs2_xattr_user_handler: xattr_handler;
    pub static ocfs2_xattr_trusted_handler: xattr_handler;
    pub static ocfs2_xattr_security_handler: xattr_handler;
    pub static ocfs2_xattr_handlers: *const *const xattr_handler;

    pub fn ocfs2_listxattr(dentry: *mut dentry, buffer: *mut ::core::ffi::c_char,
                           size: usize) -> isize;
    pub fn ocfs2_xattr_get_nolock(inode: *mut inode, bh: *mut buffer_head,
                                  name_index: ::core::ffi::c_int,
                                  name: *const ::core::ffi::c_char,
                                  value: *mut ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int;
    pub fn ocfs2_xattr_set(inode: *mut inode, name_index: ::core::ffi::c_int,
                           name: *const ::core::ffi::c_char,
                           value: *const ::core::ffi::c_void, size: usize,
                           flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ocfs2_xattr_set_handle(handle: *mut handle_t, inode: *mut inode,
                                  bh: *mut buffer_head, name_index: ::core::ffi::c_int,
                                  name: *const ::core::ffi::c_char,
                                  value: *const ::core::ffi::c_void, size: usize,
                                  flags: ::core::ffi::c_int,
                                  xattr_alloc: *mut ocfs2_alloc_context,
                                  meta_alloc: *mut ocfs2_alloc_context) -> ::core::ffi::c_int;
    pub fn ocfs2_has_inline_xattr_value_outside(inode: *mut inode,
                                                di: *mut ocfs2_dinode) -> ::core::ffi::c_int;
    pub fn ocfs2_validate_inode_xattr(sb: *mut super_block, blkno: u64,
                                      di: *mut ocfs2_dinode) -> ::core::ffi::c_int;
    pub fn ocfs2_xattr_remove(inode: *mut inode, bh: *mut buffer_head) -> ::core::ffi::c_int;
    pub fn ocfs2_init_security_get(inode: *mut inode, dir: *mut inode,
                                   qstr: *const qstr,
                                   si: *mut ocfs2_security_xattr_info) -> ::core::ffi::c_int;
    pub fn ocfs2_init_security_set(handle: *mut handle_t, inode: *mut inode,
                                   bh: *mut buffer_head,
                                   si: *mut ocfs2_security_xattr_info,
                                   data_alloc: *mut ocfs2_alloc_context,
                                   meta_alloc: *mut ocfs2_alloc_context) -> ::core::ffi::c_int;
    pub fn ocfs2_calc_security_init(inode: *mut inode,
                                    si: *mut ocfs2_security_xattr_info,
                                    want_clusters: *mut ::core::ffi::c_int,
                                    xattr_credits: *mut ::core::ffi::c_int,
                                    meta_alloc: *mut *mut ocfs2_alloc_context) -> ::core::ffi::c_int;

    pub fn ocfs2_calc_xattr_init(dir: *mut inode, mode: umode_t,
                                 si: *mut ocfs2_security_xattr_info,
                                 want_clusters: *mut ::core::ffi::c_int,
                                 xattr_credits: *mut ::core::ffi::c_int,
                                 want_meta: *mut ::core::ffi::c_int,
                                 acl_state: *mut ocfs2_acl_state) -> ::core::ffi::c_int;

    pub fn ocfs2_xattr_attach_refcount_tree(inode: *mut inode, fe_bh: *mut buffer_head,
                                            ref_ci: *mut ocfs2_caching_info,
                                            ref_root_bh: *mut buffer_head,
                                            dealloc: *mut ocfs2_cached_dealloc_ctxt) -> ::core::ffi::c_int;
    pub fn ocfs2_reflink_xattrs(old_inode: *mut inode, old_bh: *mut buffer_head,
                                new_inode: *mut inode, new_bh: *mut buffer_head,
                                preserve_security: bool) -> ::core::ffi::c_int;
    pub fn ocfs2_init_security_and_acl(dir: *mut inode, inode: *mut inode,
                                       qstr: *const qstr) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct ocfs2_xattr_value_buf {
    pub vb_bh: *mut buffer_head,
    pub vb_access: ocfs2_journal_access_func,
    pub vb_xv: *mut ocfs2_xattr_value_root,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
