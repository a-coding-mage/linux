// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026 Western Digital.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel test environment.

const WP_MISSING_DEV: u64 = u64::MAX;
const WP_CONVENTIONAL: u64 = u64::MAX - 1;
const ZONE_SIZE: u64 = SZ_256M;
const HALF_STRIPE_LEN: u64 = BTRFS_STRIPE_LEN >> 1;

#[repr(C)]
struct LoadZoneInfoTestVector {
    raid_type: u64,
    num_stripes: u64,
    alloc_offsets: [u64; 8],
    last_alloc: u64,
    bg_length: u64,
    degraded: bool,
    expected_result: i32,
    expected_alloc_offset: u64,
    description: *const core::ffi::c_char,
}

#[repr(C)]
struct ZoneInfo { physical: u64, capacity: u64, alloc_offset: u64 }

unsafe fn test_load_zone_info(fs_info: *mut BtrfsFsInfo,
                              test: *const LoadZoneInfoTestVector) -> i32 {
    let mut bg: *mut BtrfsBlockGroup = core::ptr::null_mut();
    let mut map: *mut BtrfsChunkMap = core::ptr::null_mut();
    let t = &*test;
    bg = btrfs_alloc_dummy_block_group(fs_info, t.bg_length);
    if bg.is_null() { test_std_err(TEST_ALLOC_BLOCK_GROUP); return -ENOMEM; }
    map = btrfs_alloc_chunk_map(t.num_stripes, GFP_KERNEL);
    if map.is_null() { test_std_err(TEST_ALLOC_EXTENT_MAP); return -ENOMEM; }
    let mut zone_info: *mut ZoneInfo = kzalloc_objs(t.num_stripes, GFP_KERNEL);
    if zone_info.is_null() { test_err(c"cannot allocate zone info".as_ptr()); return -ENOMEM; }
    let active = bitmap_zalloc(t.num_stripes, GFP_KERNEL);
    if active.is_null() { test_err(c"cannot allocate active bitmap".as_ptr()); return -ENOMEM; }
    (*map).type_ = t.raid_type;
    (*map).num_stripes = t.num_stripes;
    if t.raid_type == BTRFS_BLOCK_GROUP_RAID10 { (*map).sub_stripes = 2; }
    for i in 0..t.num_stripes as usize {
        (*zone_info.add(i)).physical = 0;
        (*zone_info.add(i)).alloc_offset = t.alloc_offsets[i];
        (*zone_info.add(i)).capacity = ZONE_SIZE;
        if (*zone_info.add(i)).alloc_offset != 0 && (*zone_info.add(i)).alloc_offset < ZONE_SIZE { __set_bit(i, active); }
    }
    if t.degraded { btrfs_set_opt((*fs_info).mount_opt, DEGRADED); }
    else { btrfs_clear_opt((*fs_info).mount_opt, DEGRADED); }
    let ret = btrfs_load_block_group_by_raid_type(bg, map, zone_info, active, t.last_alloc);
    if ret != t.expected_result { test_err(c"unexpected return value".as_ptr()); return -EINVAL; }
    if ret == 0 && (*bg).alloc_offset != t.expected_alloc_offset { test_err(c"unexpected alloc_offset".as_ptr()); return -EINVAL; }
    0
}

// The test vectors retain the complete source ordering and values.
static LOAD_ZONE_INFO_TESTS: &[LoadZoneInfoTestVector] = &[
    LoadZoneInfoTestVector { description: c"SINGLE: load write pointer from sequential zone".as_ptr(), raid_type: 0, num_stripes: 1, alloc_offsets: [SZ_1M,0,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"DUP: having matching write pointers".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_DUP, num_stripes: 2, alloc_offsets: [SZ_1M,SZ_1M,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"DUP: seq zone and conv zone, matching last_alloc".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_DUP, num_stripes: 2, alloc_offsets: [SZ_1M,WP_CONVENTIONAL,0,0,0,0,0,0], last_alloc: SZ_1M, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"DUP: seq zone and conv zone, smaller last_alloc".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_DUP, num_stripes: 2, alloc_offsets: [SZ_1M,WP_CONVENTIONAL,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"DUP: fail: different write pointers".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_DUP, num_stripes: 2, alloc_offsets: [SZ_1M,SZ_2M,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: -EIO, expected_alloc_offset: 0 },
    LoadZoneInfoTestVector { description: c"DUP: fail: missing device".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_DUP, num_stripes: 2, alloc_offsets: [SZ_1M,WP_MISSING_DEV,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: -EIO, expected_alloc_offset: 0 },
    LoadZoneInfoTestVector { description: c"RAID1: having matching write pointers".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_RAID1, num_stripes: 2, alloc_offsets: [SZ_1M,SZ_1M,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"RAID1: fail: missing device on DEGRADED".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_RAID1, num_stripes: 2, alloc_offsets: [SZ_1M,WP_MISSING_DEV,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: true, expected_result: 0, expected_alloc_offset: SZ_1M },
    LoadZoneInfoTestVector { description: c"RAID0: initial partial write".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_RAID0, num_stripes: 4, alloc_offsets: [HALF_STRIPE_LEN,0,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: HALF_STRIPE_LEN },
    LoadZoneInfoTestVector { description: c"RAID0: fail: disordered stripes".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_RAID0, num_stripes: 4, alloc_offsets: [BTRFS_STRIPE_LEN,BTRFS_STRIPE_LEN*2,BTRFS_STRIPE_LEN,BTRFS_STRIPE_LEN,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: -EIO, expected_alloc_offset: 0 },
    LoadZoneInfoTestVector { description: c"RAID10: initial partial write".as_ptr(), raid_type: BTRFS_BLOCK_GROUP_RAID10, num_stripes: 4, alloc_offsets: [HALF_STRIPE_LEN,HALF_STRIPE_LEN,0,0,0,0,0,0], last_alloc: 0, bg_length: 0, degraded: false, expected_result: 0, expected_alloc_offset: HALF_STRIPE_LEN },
];

unsafe fn btrfs_test_zoned() -> i32 {
    test_msg(c"running zoned tests (error messages are expected)".as_ptr());
    let fs_info = btrfs_alloc_dummy_fs_info(PAGE_SIZE, PAGE_SIZE);
    if fs_info.is_null() { test_std_err(TEST_ALLOC_FS_INFO); return -ENOMEM; }
    for test in LOAD_ZONE_INFO_TESTS { let ret = test_load_zone_info(fs_info, test); if ret != 0 { test_err(c"test case failed".as_ptr()); return ret; } }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
