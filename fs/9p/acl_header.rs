/* SPDX-License-Identifier: LGPL-2.1 */
/*
 * Copyright IBM Corporation, 2010
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// C header guard: FS_9P_ACL_H

// CONFIG_9P_FS_POSIX_ACL selects the declarations below at build time.
#[cfg(feature = "CONFIG_9P_FS_POSIX_ACL")]
extern "C" {
    pub fn v9fs_get_acl(inode: *mut inode, fid: *mut p9_fid) -> ::core::ffi::c_int;
    pub fn v9fs_iop_get_inode_acl(
        inode: *mut inode,
        type_: ::core::ffi::c_int,
        rcu: bool,
    ) -> *mut posix_acl;
    pub fn v9fs_iop_get_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        type_: ::core::ffi::c_int,
    ) -> *mut posix_acl;
    pub fn v9fs_iop_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn v9fs_acl_chmod(inode: *mut inode, fid: *mut p9_fid) -> ::core::ffi::c_int;
    pub fn v9fs_set_create_acl(
        inode: *mut inode,
        fid: *mut p9_fid,
        dacl: *mut posix_acl,
        acl: *mut posix_acl,
    ) -> ::core::ffi::c_int;
    pub fn v9fs_acl_mode(
        dir: *mut inode,
        modep: *mut umode_t,
        dpacl: *mut *mut posix_acl,
        pacl: *mut *mut posix_acl,
    ) -> ::core::ffi::c_int;
    pub fn v9fs_put_acl(dacl: *mut posix_acl, acl: *mut posix_acl);
}

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub const v9fs_iop_get_inode_acl: Option<unsafe extern "C" fn()> = None;
#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub const v9fs_iop_get_acl: Option<unsafe extern "C" fn()> = None;
#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub const v9fs_iop_set_acl: Option<unsafe extern "C" fn()> = None;

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub unsafe extern "C" fn v9fs_get_acl(_inode: *mut inode, _fid: *mut p9_fid) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub unsafe extern "C" fn v9fs_acl_chmod(_inode: *mut inode, _fid: *mut p9_fid) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub unsafe extern "C" fn v9fs_set_create_acl(
    _inode: *mut inode,
    _fid: *mut p9_fid,
    _dacl: *mut posix_acl,
    _acl: *mut posix_acl,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub unsafe extern "C" fn v9fs_put_acl(_dacl: *mut posix_acl, _acl: *mut posix_acl) {}

#[cfg(not(feature = "CONFIG_9P_FS_POSIX_ACL"))]
pub unsafe extern "C" fn v9fs_acl_mode(
    _dir: *mut inode,
    _modep: *mut umode_t,
    _dpacl: *mut *mut posix_acl,
    _pacl: *mut *mut posix_acl,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
