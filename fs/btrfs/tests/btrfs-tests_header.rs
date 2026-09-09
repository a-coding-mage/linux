/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2013 Fusion IO.  All rights reserved.
 */

// Translated from btrfs-tests.h.  The Linux type and cleanup headers are
// dependencies supplied by the surrounding translation.

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
extern "C" {
    pub fn btrfs_run_sanity_tests() -> ::core::ffi::c_int;
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
macro_rules! test_msg {
    ($fmt:expr $(, $args:expr)*) => {
        pr_info!("BTRFS: selftest: " $fmt "\n" $(, $args)*);
    };
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
macro_rules! test_err {
    ($fmt:expr $(, $args:expr)*) => {
        pr_err!(
            "BTRFS: selftest: %s:%d " $fmt "\n",
            file!(),
            line!()
            $(, $args)*
        );
    };
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
macro_rules! test_std_err {
    ($index:expr) => {
        test_err!("%s", test_error[$index]);
    };
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(u32)]
pub enum TestAlloc {
    TEST_ALLOC_FS_INFO,
    TEST_ALLOC_ROOT,
    TEST_ALLOC_EXTENT_BUFFER,
    TEST_ALLOC_PATH,
    TEST_ALLOC_INODE,
    TEST_ALLOC_BLOCK_GROUP,
    TEST_ALLOC_EXTENT_MAP,
    TEST_ALLOC_CHUNK_MAP,
    TEST_ALLOC_IO_CONTEXT,
    TEST_ALLOC_TRANSACTION,
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
extern "C" {
    pub static test_error: *const *const ::core::ffi::c_char;
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_root {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_trans_handle {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_transaction {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_fs_info {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_block_group {
    _private: [u8; 0],
}
#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
#[repr(C)]
pub struct btrfs_device {
    _private: [u8; 0],
}

#[cfg(CONFIG_BTRFS_FS_RUN_SANITY_TESTS)]
extern "C" {
    pub fn btrfs_test_extent_buffer_operations(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_free_space_cache(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_extent_io(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_inodes(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_qgroups(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_free_space_tree(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_raid_stripe_tree(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_extent_map() -> ::core::ffi::c_int;
    pub fn btrfs_test_delayed_refs(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_test_chunk_allocation(sectorsize: u32, nodesize: u32) -> ::core::ffi::c_int;
    pub fn btrfs_new_test_inode() -> *mut inode;
    pub fn btrfs_alloc_dummy_fs_info(nodesize: u32, sectorsize: u32) -> *mut btrfs_fs_info;
    pub fn btrfs_free_dummy_fs_info(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_free_dummy_root(root: *mut btrfs_root);
    pub fn btrfs_alloc_dummy_block_group(
        fs_info: *mut btrfs_fs_info,
        length: ::core::ffi::c_ulong,
    ) -> *mut btrfs_block_group;
    pub fn btrfs_free_dummy_block_group(cache: *mut btrfs_block_group);
    pub fn btrfs_init_dummy_trans(trans: *mut btrfs_trans_handle, fs_info: *mut btrfs_fs_info);
    pub fn btrfs_init_dummy_transaction(trans: *mut btrfs_transaction, fs_info: *mut btrfs_fs_info);
    pub fn btrfs_alloc_dummy_device(fs_info: *mut btrfs_fs_info) -> *mut btrfs_device;
}

// DEFINE_FREE cleanup declarations are represented by the corresponding
// externally defined free functions above; the surrounding cleanup facility
// supplies the declaration-specific ownership integration.

#[cfg(all(CONFIG_BTRFS_FS_RUN_SANITY_TESTS, CONFIG_BLK_DEV_ZONED))]
extern "C" {
    pub fn btrfs_test_zoned() -> ::core::ffi::c_int;
}

#[cfg(all(CONFIG_BTRFS_FS_RUN_SANITY_TESTS, not(CONFIG_BLK_DEV_ZONED)))]
#[inline]
pub fn btrfs_test_zoned() -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_BTRFS_FS_RUN_SANITY_TESTS))]
#[inline]
pub fn btrfs_run_sanity_tests() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
