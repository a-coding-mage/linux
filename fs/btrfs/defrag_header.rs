/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the Linux types and compiler annotations
// included by the original header are supplied by other translation units.

#[repr(C)]
pub struct file_ra_state {
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
pub struct btrfs_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_ioctl_defrag_range_args {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btrfs_defrag_file(
        inode: *mut btrfs_inode,
        ra: *mut file_ra_state,
        range: *mut btrfs_ioctl_defrag_range_args,
        newer_than: u64,
        max_to_defrag: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;

    // The C __init and __cold annotations affect section placement only.
    pub fn btrfs_auto_defrag_init() -> kernel::ffi::c_int;
    pub fn btrfs_auto_defrag_exit();
    pub fn btrfs_add_inode_defrag(inode: *mut btrfs_inode, extent_thresh: u32);
    pub fn btrfs_run_defrag_inodes(fs_info: *mut btrfs_fs_info) -> kernel::ffi::c_int;
    pub fn btrfs_cleanup_defrag_inodes(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_defrag_root(root: *mut btrfs_root) -> kernel::ffi::c_int;

    // `current` is a kernel-provided task pointer used by signal_pending.
    pub static mut current: *mut kernel::ffi::c_void;
    pub fn signal_pending(task: *mut kernel::ffi::c_void) -> kernel::ffi::c_int;
}

#[inline]
pub unsafe fn btrfs_defrag_cancelled(_fs_info: *mut btrfs_fs_info) -> kernel::ffi::c_int {
    signal_pending(current)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
