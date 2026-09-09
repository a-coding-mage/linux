/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The Linux type declarations and included
// definitions are supplied by the surrounding translation unit.

#[repr(C)]
pub struct fscrypt_str {
    _private: [u8; 0],
}

#[repr(C)]
pub struct extent_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_root_item {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_block_rsv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}

extern "C" {
    pub fn btrfs_subvolume_reserve_metadata(
        root: *mut btrfs_root,
        rsv: *mut btrfs_block_rsv,
        nitems: i32,
        use_global_rsv: bool,
    ) -> i32;
    pub fn btrfs_add_root_ref(
        trans: *mut btrfs_trans_handle,
        root_id: u64,
        ref_id: u64,
        dirid: u64,
        sequence: u64,
        name: *const fscrypt_str,
    ) -> i32;
    pub fn btrfs_del_root_ref(
        trans: *mut btrfs_trans_handle,
        root_id: u64,
        ref_id: u64,
        dirid: u64,
        sequence: *mut u64,
        name: *const fscrypt_str,
    ) -> i32;
    pub fn btrfs_del_root(trans: *mut btrfs_trans_handle, key: *const btrfs_key) -> i32;
    pub fn btrfs_insert_root(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        key: *const btrfs_key,
        item: *mut btrfs_root_item,
    ) -> i32;
    pub fn btrfs_update_root(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        key: *mut btrfs_key,
        item: *mut btrfs_root_item,
    ) -> i32;
    pub fn btrfs_find_root(
        root: *mut btrfs_root,
        search_key: *const btrfs_key,
        path: *mut btrfs_path,
        root_item: *mut btrfs_root_item,
        root_key: *mut btrfs_key,
    ) -> i32;
    pub fn btrfs_find_orphan_roots(fs_info: *mut btrfs_fs_info) -> i32;
    pub fn btrfs_set_root_node(item: *mut btrfs_root_item, node: *mut extent_buffer);
    pub fn btrfs_check_and_init_root_item(item: *mut btrfs_root_item);
    pub fn btrfs_update_root_times(trans: *mut btrfs_trans_handle, root: *mut btrfs_root);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
