/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * acl.h
 *
 * Copyright (C) 2004, 2008 Oracle.  All rights reserved.
 */

// Dependency supplied by linux/posix_acl_xattr.h in the C source.

pub type __le16 = u16;
pub type __le32 = u32;
pub type umode_t = u32;

// Opaque declarations for types supplied by other headers.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct posix_acl {
    _private: [u8; 0],
}
#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ocfs2_alloc_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct handle_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocfs2_acl_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
pub struct ocfs2_acl_state {
    pub default_acl: *mut posix_acl,
    pub acl: *mut posix_acl,
    pub mode: umode_t,
}

unsafe extern "C" {
    pub fn ocfs2_iop_get_acl(
        inode: *mut inode,
        type_: core::ffi::c_int,
        rcu: bool,
    ) -> *mut posix_acl;

    pub fn ocfs2_iop_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn ocfs2_acl_chmod(
        inode: *mut inode,
        bh: *mut buffer_head,
    ) -> core::ffi::c_int;

    pub fn ocfs2_acl_init_prepare(
        inode: *mut inode,
        dir: *mut inode,
        dir_bh: *mut buffer_head,
        state: *mut ocfs2_acl_state,
    ) -> core::ffi::c_int;

    pub fn ocfs2_acl_init_release(state: *mut ocfs2_acl_state);

    pub fn ocfs2_init_acl(
        handle: *mut handle_t,
        inode: *mut inode,
        di_bh: *mut buffer_head,
        meta_ac: *mut ocfs2_alloc_context,
        data_ac: *mut ocfs2_alloc_context,
        state: *mut ocfs2_acl_state,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
