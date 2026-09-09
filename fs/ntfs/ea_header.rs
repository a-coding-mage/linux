/* SPDX-License-Identifier: GPL-2.0-or-later */

pub const NTFS_EA_UID: u32 = 1 << 1;
pub const NTFS_EA_GID: u32 = 1 << 2;
pub const NTFS_EA_MODE: u32 = 1 << 3;

extern "C" {
    pub static ntfs_xattr_handlers: *const *const xattr_handler;

    pub fn ntfs_ea_set_wsl_not_symlink(ni: *mut ntfs_inode, mode: mode_t, dev: dev_t) -> c_int;
    pub fn ntfs_ea_get_wsl_inode(
        inode: *mut inode,
        rdevp: *mut dev_t,
        flags: c_uint,
        has_lxmod: *mut bool,
    ) -> c_int;
    pub fn ntfs_ea_set_wsl_inode(
        inode: *mut inode,
        rdev: dev_t,
        ea_size: *mut __le16,
        flags: c_uint,
    ) -> c_int;
    pub fn ntfs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: size_t) -> ssize_t;
}

#[cfg(CONFIG_NTFS_FS_POSIX_ACL)]
extern "C" {
    pub fn ntfs_get_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        type_: c_int,
    ) -> *mut posix_acl;
    pub fn ntfs_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: c_int,
    ) -> c_int;
    pub fn ntfs_init_acl(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        dir: *mut inode,
    ) -> c_int;
}

#[cfg(not(CONFIG_NTFS_FS_POSIX_ACL))]
pub const ntfs_get_acl: *const () = core::ptr::null();
#[cfg(not(CONFIG_NTFS_FS_POSIX_ACL))]
pub const ntfs_set_acl: *const () = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
