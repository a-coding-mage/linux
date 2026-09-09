/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/types.h and linux/sizes.h.

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_discard_ctl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_block_group {
    _private: [u8; 0],
}

/* Discard size limits */
pub const BTRFS_ASYNC_DISCARD_DEFAULT_MAX_SIZE: u64 = 64 * 1024 * 1024;
pub const BTRFS_ASYNC_DISCARD_MAX_FILTER: u64 = 1024 * 1024;
pub const BTRFS_ASYNC_DISCARD_MIN_FILTER: u64 = 32 * 1024;

/* List operations */
unsafe extern "C" {
    pub fn btrfs_discard_check_filter(
        block_group: *mut btrfs_block_group,
        bytes: u64,
    );

    /* Work operations */
    pub fn btrfs_discard_cancel_work(
        discard_ctl: *mut btrfs_discard_ctl,
        block_group: *mut btrfs_block_group,
    );
    pub fn btrfs_discard_queue_work(
        discard_ctl: *mut btrfs_discard_ctl,
        block_group: *mut btrfs_block_group,
    );
    pub fn btrfs_discard_schedule_work(
        discard_ctl: *mut btrfs_discard_ctl,
        override_: bool,
    );

    /* Update operations */
    pub fn btrfs_discard_calc_delay(discard_ctl: *mut btrfs_discard_ctl);
    pub fn btrfs_discard_update_discardable(block_group: *mut btrfs_block_group);

    /* Setup/cleanup operations */
    pub fn btrfs_discard_punt_unused_bgs_list(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_discard_resume(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_discard_stop(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_discard_init(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_discard_cleanup(fs_info: *mut btrfs_fs_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
