/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations corresponding to the C header's opaque kernel types.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_kattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_cmd {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_ioctl_balance_args {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btrfs_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize;
    pub fn btrfs_compat_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize;
    pub fn btrfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> i32;
    pub fn btrfs_fileattr_set(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        fa: *mut file_kattr,
    ) -> i32;
    pub fn btrfs_ioctl_get_supported_features(arg: *mut core::ffi::c_void) -> i32;
    pub fn btrfs_sync_inode_flags_to_i_flags(inode: *mut btrfs_inode);
    pub fn btrfs_update_ioctl_balance_args(
        fs_info: *mut btrfs_fs_info,
        bargs: *mut btrfs_ioctl_balance_args,
    );
    pub fn btrfs_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32;
    pub fn btrfs_uring_read_extent_endio(ctx: *mut core::ffi::c_void, err: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
