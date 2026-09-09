/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header BTRFS_DELALLOC_SPACE_H.

#[repr(C)]
pub struct extent_changeset {
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

extern "C" {
    pub fn btrfs_alloc_data_chunk_ondemand(inode: *const btrfs_inode, bytes: u64) -> i32;
    pub fn btrfs_check_data_free_space(
        inode: *mut btrfs_inode,
        reserved: *mut *mut extent_changeset,
        start: u64,
        len: u64,
        noflush: bool,
    ) -> i32;
    pub fn btrfs_free_reserved_data_space(
        inode: *mut btrfs_inode,
        reserved: *mut extent_changeset,
        start: u64,
        len: u64,
    );
    pub fn btrfs_delalloc_release_space(
        inode: *mut btrfs_inode,
        reserved: *mut extent_changeset,
        start: u64,
        len: u64,
        qgroup_free: bool,
    );
    pub fn btrfs_free_reserved_data_space_noquota(inode: *mut btrfs_inode, len: u64);
    pub fn btrfs_delalloc_release_metadata(
        inode: *mut btrfs_inode,
        num_bytes: u64,
        qgroup_free: bool,
    );
    pub fn btrfs_delalloc_reserve_space(
        inode: *mut btrfs_inode,
        reserved: *mut *mut extent_changeset,
        start: u64,
        len: u64,
    ) -> i32;
    pub fn btrfs_delalloc_reserve_metadata(
        inode: *mut btrfs_inode,
        num_bytes: u64,
        disk_num_bytes: u64,
        noflush: bool,
    ) -> i32;
    pub fn btrfs_delalloc_release_extents(inode: *mut btrfs_inode, num_bytes: u64);
    pub fn btrfs_delalloc_shrink_extents(
        inode: *mut btrfs_inode,
        reserved_len: u64,
        new_len: u64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
