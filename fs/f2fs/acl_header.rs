/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs/f2fs/acl.h
 *
 * Copyright (c) 2012 Samsung Electronics Co., Ltd.
 *             http://www.samsung.com/
 *
 * Portions of this code from linux/fs/ext2/acl.h
 *
 * Copyright (C) 2001-2003 Andreas Gruenbacher, <agruen@suse.de>
 */

// Dependency supplied by the Linux POSIX ACL extended-attribute definitions.

pub const F2FS_ACL_VERSION: u32 = 0x0001;

#[repr(C)]
pub struct f2fs_acl_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
pub struct f2fs_acl_entry_short {
    pub e_tag: __le16,
    pub e_perm: __le16,
}

#[repr(C)]
pub struct f2fs_acl_header {
    pub a_version: __le32,
}

// CONFIG_F2FS_FS_POSIX_ACL controls whether the POSIX ACL declarations are enabled.
#[cfg(CONFIG_F2FS_FS_POSIX_ACL)]
extern "C" {
    pub fn f2fs_get_acl(
        inode: *mut inode,
        type_: core::ffi::c_int,
        r#default: bool,
    ) -> *mut posix_acl;

    pub fn f2fs_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn f2fs_init_acl(
        inode: *mut inode,
        dir: *mut inode,
        ifolio: *mut folio,
        dfolio: *mut folio,
    ) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_F2FS_FS_POSIX_ACL))]
pub const f2fs_get_acl: Option<unsafe extern "C" fn(*mut inode, core::ffi::c_int, bool) -> *mut posix_acl> = None;

#[cfg(not(CONFIG_F2FS_FS_POSIX_ACL))]
pub const f2fs_set_acl: Option<
    unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut posix_acl, core::ffi::c_int) -> core::ffi::c_int,
> = None;

#[cfg(not(CONFIG_F2FS_FS_POSIX_ACL))]
pub unsafe fn f2fs_init_acl(
    _inode: *mut inode,
    _dir: *mut inode,
    _ifolio: *mut folio,
    _dfolio: *mut folio,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
