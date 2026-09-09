/* SPDX-License-Identifier: GPL-2.0 */

// Declarations corresponding to the C header's external kernel dependencies.

#[repr(C)]
pub struct fscrypt_str {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btrfs_inode {
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
pub struct btrfs_dir_item {
    _private: [u8; 0],
}

extern "C" {
    pub fn crc32c(crc: u32, address: *const core::ffi::c_void, length: usize) -> u32;

    pub fn btrfs_check_dir_item_collision(
        root: *mut btrfs_root,
        dir_ino: u64,
        name: *const fscrypt_str,
    ) -> i32;
    pub fn btrfs_insert_dir_item(
        trans: *mut btrfs_trans_handle,
        name: *const fscrypt_str,
        dir: *mut btrfs_inode,
        location: *const btrfs_key,
        type_: u8,
        index: u64,
    ) -> i32;
    pub fn btrfs_lookup_dir_item(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        dir: u64,
        name: *const fscrypt_str,
        mod_: i32,
    ) -> *mut btrfs_dir_item;
    pub fn btrfs_lookup_dir_index_item(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        dir: u64,
        index: u64,
        name: *const fscrypt_str,
        mod_: i32,
    ) -> *mut btrfs_dir_item;
    pub fn btrfs_search_dir_index_item(
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        dirid: u64,
        name: *const fscrypt_str,
    ) -> *mut btrfs_dir_item;
    pub fn btrfs_delete_one_dir_name(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        di: *const btrfs_dir_item,
    ) -> i32;
    pub fn btrfs_insert_xattr_item(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        objectid: u64,
        name: *const core::ffi::c_char,
        name_len: u16,
        data: *const core::ffi::c_void,
        data_len: u16,
    ) -> i32;
    pub fn btrfs_lookup_xattr(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        path: *mut btrfs_path,
        dir: u64,
        name: *const core::ffi::c_char,
        name_len: u16,
        mod_: i32,
    ) -> *mut btrfs_dir_item;
    pub fn btrfs_match_dir_item_name(
        path: *const btrfs_path,
        name: *const core::ffi::c_char,
        name_len: i32,
    ) -> *mut btrfs_dir_item;
}

#[inline]
pub unsafe fn btrfs_name_hash(name: *const core::ffi::c_char, len: i32) -> u64 {
    crc32c((!1u32), name.cast::<core::ffi::c_void>(), len as usize) as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
