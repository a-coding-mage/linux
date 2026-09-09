/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The original include of <linux/types.h>
// supplies u64 and bool.

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_scrub_progress {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn btrfs_scrub_dev(
        fs_info: *mut btrfs_fs_info,
        devid: u64,
        start: u64,
        end: u64,
        progress: *mut btrfs_scrub_progress,
        readonly: bool,
        is_dev_replace: bool,
    ) -> i32;
    pub fn btrfs_scrub_pause(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_scrub_continue(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_scrub_cancel(info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_scrub_cancel_dev(dev: *mut btrfs_device) -> i32;
    pub fn btrfs_scrub_progress(
        fs_info: *mut btrfs_fs_info,
        devid: u64,
        progress: *mut btrfs_scrub_progress,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
