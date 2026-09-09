/* SPDX-License-Identifier: GPL-2.0 */

// Translated from btrfs/acl.h.

use core::ffi::c_int;

#[repr(C)]
pub struct posix_acl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_BTRFS_FS_POSIX_ACL")]
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_BTRFS_FS_POSIX_ACL")]
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[cfg(not(feature = "CONFIG_BTRFS_FS_POSIX_ACL"))]
#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[cfg(not(feature = "CONFIG_BTRFS_FS_POSIX_ACL"))]
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_BTRFS_FS_POSIX_ACL")]
extern "C" {
    pub fn btrfs_get_acl(inode: *mut inode, type_: c_int, rcu: bool) -> *mut posix_acl;
    pub fn btrfs_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: c_int,
    ) -> c_int;
    pub fn __btrfs_set_acl(
        trans: *mut btrfs_trans_handle,
        inode: *mut inode,
        acl: *mut posix_acl,
        type_: c_int,
    ) -> c_int;
}

#[cfg(not(feature = "CONFIG_BTRFS_FS_POSIX_ACL"))]
pub const btrfs_get_acl: Option<
    unsafe extern "C" fn(*mut inode, c_int, bool) -> *mut posix_acl,
> = None;

#[cfg(not(feature = "CONFIG_BTRFS_FS_POSIX_ACL"))]
pub const btrfs_set_acl: Option<
    unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut posix_acl, c_int) -> c_int,
> = None;

#[cfg(not(feature = "CONFIG_BTRFS_FS_POSIX_ACL"))]
pub unsafe fn __btrfs_set_acl(
    _trans: *mut btrfs_trans_handle,
    _inode: *mut inode,
    _acl: *mut posix_acl,
    _type_: c_int,
) -> c_int {
    // EOPNOTSUPP from <linux/errno.h>.
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
