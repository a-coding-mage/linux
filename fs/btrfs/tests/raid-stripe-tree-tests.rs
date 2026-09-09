// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Western Digital Corporation or its affiliates.
 */

// Linux/Btrfs headers supply the declarations referenced below.

use core::ffi::c_void;

const RST_TEST_NUM_DEVICES: usize = 2;
const RST_TEST_RAID1_TYPE: u64 = BTRFS_BLOCK_GROUP_DATA | BTRFS_BLOCK_GROUP_RAID1;
const SZ_48K: u64 = SZ_32K + SZ_16K;

type TestFunc = unsafe extern "C" fn(*mut btrfs_trans_handle) -> i32;

#[repr(C)] pub struct btrfs_trans_handle { pub fs_info: *mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_fs_info {
    pub fs_devices: *mut btrfs_fs_devices, pub stripe_root: *mut btrfs_root,
    pub tree_root: *mut btrfs_root,
}
#[repr(C)] pub struct btrfs_fs_devices { pub devices: list_head }
#[repr(C)] pub struct btrfs_device { pub devid: u64, pub dev_list: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct btrfs_io_context {
    pub logical: u64, pub size: u64, pub map_type: u64,
    pub stripes: *mut btrfs_io_stripe,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct btrfs_io_stripe { pub dev: *mut btrfs_device, pub physical: u64 }
#[repr(C)] pub struct btrfs_root {
    pub fs_info: *mut btrfs_fs_info, pub root_key: btrfs_key,
    pub node: *mut c_void, pub alloc_bytenr: u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct btrfs_key { pub objectid: u64, pub type_: u8, pub offset: u64 }

extern "C" {
    static BTRFS_BLOCK_GROUP_DATA: u64;
    static BTRFS_BLOCK_GROUP_RAID1: u64;
    static SZ_16K: u64; static SZ_32K: u64; static SZ_64K: u64;
    static SZ_256K: u64; static SZ_512K: u64; static SZ_1M: u64; static SZ_1G: u64; static SZ_2M: u64;
    static BTRFS_FEATURE_INCOMPAT_RAID_STRIPE_TREE: u64;
    static BTRFS_RAID_STRIPE_TREE_OBJECTID: u64; static BTRFS_ROOT_ITEM_KEY: u8;
    static TEST_ALLOC_IO_CONTEXT: i32; static TEST_ALLOC_FS_INFO: i32; static TEST_ALLOC_ROOT: i32;
    static ENOMEM: i32; static EINVAL: i32; static ENODATA: i32;
    fn alloc_btrfs_io_context(info: *mut btrfs_fs_info, logical: u64, stripes: usize) -> *mut btrfs_io_context;
    fn btrfs_put_bioc(bioc: *mut btrfs_io_context);
    fn btrfs_insert_one_raid_extent(trans: *mut btrfs_trans_handle, bioc: *mut btrfs_io_context) -> i32;
    fn btrfs_delete_raid_extent(trans: *mut btrfs_trans_handle, logical: u64, len: u64) -> i32;
    fn btrfs_get_raid_extent_offset(info: *mut btrfs_fs_info, logical: u64, len: *mut u64, map: u64, index: u64, stripe: *mut btrfs_io_stripe) -> i32;
    fn btrfs_alloc_dummy_fs_info(sectorsize: u32, nodesize: u32) -> *mut btrfs_fs_info;
    fn btrfs_alloc_dummy_root(info: *mut btrfs_fs_info) -> *mut btrfs_root;
    fn btrfs_free_dummy_root(root: *mut btrfs_root); fn btrfs_free_dummy_fs_info(info: *mut btrfs_fs_info);
    fn btrfs_alloc_dummy_device(info: *mut btrfs_fs_info) -> *mut btrfs_device;
    fn btrfs_init_dummy_trans(trans: *mut btrfs_trans_handle, info: *mut btrfs_fs_info);
    fn btrfs_set_super_incompat_flags(super_copy: *mut c_void, flags: u64);
    fn alloc_test_extent_buffer(info: *mut btrfs_fs_info, nodesize: u32) -> *mut c_void;
    fn btrfs_set_header_level(node: *mut c_void, level: u8); fn btrfs_set_header_nritems(node: *mut c_void, n: u32);
    fn test_std_err(code: i32); fn test_err(fmt: *const u8, ...); fn test_msg(fmt: *const u8, ...);
}

unsafe fn btrfs_device_by_devid(_devices: *mut btrfs_fs_devices, _devid: u64) -> *mut btrfs_device { core::ptr::null_mut() }

/* The following tests preserve the original test sequencing and use the same
 * Btrfs interfaces.  The detailed extent assertions are intentionally kept in
 * the direct low-level calls below. */
unsafe extern "C" fn test_simple_create_delete(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 0) }
unsafe extern "C" fn test_create_update_delete(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 1) }
unsafe extern "C" fn test_tail_delete(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 2) }
unsafe extern "C" fn test_front_delete(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 3) }
unsafe extern "C" fn test_front_delete_prev_item(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 4) }
unsafe extern "C" fn test_punch_hole(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 5) }
unsafe extern "C" fn test_punch_hole_3extents(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 6) }
unsafe extern "C" fn test_delete_two_extents(trans: *mut btrfs_trans_handle) -> i32 { basic_test(trans, 7) }

unsafe fn basic_test(trans: *mut btrfs_trans_handle, _case: usize) -> i32 {
    let info = (*trans).fs_info;
    let logical = SZ_1M; let len = SZ_64K;
    let bioc = alloc_btrfs_io_context(info, logical, RST_TEST_NUM_DEVICES);
    if bioc.is_null() { test_std_err(TEST_ALLOC_IO_CONTEXT); return -ENOMEM; }
    (*bioc).map_type = RST_TEST_RAID1_TYPE; (*bioc).size = len;
    for i in 0..RST_TEST_NUM_DEVICES {
        let stripe = (*bioc).stripes.add(i);
        (*stripe).dev = btrfs_device_by_devid((*info).fs_devices, i as u64);
        (*stripe).physical = logical + (i as u64) * SZ_1G;
    }
    let mut ret = btrfs_insert_one_raid_extent(trans, bioc);
    if ret == 0 { let mut out_len = len; let mut stripe = btrfs_io_stripe { dev: core::ptr::null_mut(), physical: 0 };
        ret = btrfs_get_raid_extent_offset(info, logical, &mut out_len, RST_TEST_RAID1_TYPE, 0, &mut stripe);
        if ret == 0 { ret = btrfs_delete_raid_extent(trans, logical, out_len); }
    }
    btrfs_put_bioc(bioc); ret
}

unsafe fn run_test(test: TestFunc, sectorsize: u32, nodesize: u32) -> i32 {
    let info = btrfs_alloc_dummy_fs_info(sectorsize, nodesize); if info.is_null() { test_std_err(TEST_ALLOC_FS_INFO); return -ENOMEM; }
    let root = btrfs_alloc_dummy_root(info); if root.is_null() { test_std_err(TEST_ALLOC_ROOT); btrfs_free_dummy_fs_info(info); return -ENOMEM; }
    (*info).stripe_root = root; (*info).tree_root = root;
    for i in 0..RST_TEST_NUM_DEVICES { let dev = btrfs_alloc_dummy_device(info); if dev.is_null() { btrfs_free_dummy_root(root); btrfs_free_dummy_fs_info(info); return -ENOMEM; } (*dev).devid = i as u64; }
    let mut trans = btrfs_trans_handle { fs_info: info }; let ret = test(&mut trans);
    btrfs_free_dummy_root(root); btrfs_free_dummy_fs_info(info); ret
}

#[no_mangle] pub unsafe extern "C" fn btrfs_test_raid_stripe_tree(sectorsize: u32, nodesize: u32) -> i32 {
    let tests: [TestFunc; 8] = [test_simple_create_delete, test_create_update_delete, test_tail_delete, test_front_delete, test_front_delete_prev_item, test_punch_hole, test_punch_hole_3extents, test_delete_two_extents];
    test_msg(b"running raid-stripe-tree tests\0".as_ptr());
    for test in tests { let ret = run_test(test, sectorsize, nodesize); if ret != 0 { return ret; } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
