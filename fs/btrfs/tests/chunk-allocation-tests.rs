// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026 Meta.  All rights reserved.
 */

// Dependencies supplied by the surrounding Btrfs implementation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct PendingExtent {
    pub start: u64,
    pub len: u64,
}

#[repr(C)]
pub struct PendingExtentTestCase {
    pub name: *const core::ffi::c_char,
    pub hole_start: u64,
    pub hole_len: u64,
    pub min_hole_size: u64,
    pub pending_extents: [PendingExtent; 2],
    pub expected_found: bool,
    pub expected_start: u64,
    pub expected_len: u64,
}

const SZ_1G: u64 = 1u64 << 30;
const SZ_2G: u64 = 2u64 << 30;

static FIND_HOLE_TESTS: &[PendingExtentTestCase] = &[
    PendingExtentTestCase { name: c"no pending extents".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 0, len: 0 }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: 0, expected_len: 10 * SZ_1G },
    PendingExtentTestCase { name: c"pending extent at start of range".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 0, len: SZ_1G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: SZ_1G, expected_len: 9 * SZ_1G },
    PendingExtentTestCase { name: c"pending extent overlapping start of range".as_ptr(), hole_start: SZ_1G, hole_len: 9 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 0, len: SZ_2G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: SZ_2G, expected_len: 8 * SZ_1G },
    PendingExtentTestCase { name: c"two holes; first hole is exactly big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: 0, expected_len: SZ_1G },
    PendingExtentTestCase { name: c"two holes; first hole is big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: SZ_2G, len: SZ_1G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: 0, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"two holes; second hole is big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_2G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: SZ_2G, expected_len: 8 * SZ_1G },
    PendingExtentTestCase { name: c"three holes; first hole big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_2G, pending_extents: [PendingExtent { start: SZ_2G, len: SZ_1G }, PendingExtent { start: 4 * SZ_1G, len: SZ_1G }], expected_found: true, expected_start: 0, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"three holes; second hole big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_2G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 5 * SZ_1G, len: SZ_1G }], expected_found: true, expected_start: SZ_2G, expected_len: 3 * SZ_1G },
    PendingExtentTestCase { name: c"three holes; third hole big enough".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_2G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 3 * SZ_1G, len: 5 * SZ_1G }], expected_found: true, expected_start: 8 * SZ_1G, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"three holes; all holes too small".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_2G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 3 * SZ_1G, len: 6 * SZ_1G }], expected_found: false, expected_start: 0, expected_len: SZ_1G },
    PendingExtentTestCase { name: c"three holes; all holes too small; first biggest".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: 3 * SZ_1G, pending_extents: [PendingExtent { start: SZ_2G, len: SZ_1G }, PendingExtent { start: 4 * SZ_1G, len: 5 * SZ_1G }], expected_found: false, expected_start: 0, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"three holes; all holes too small; second biggest".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: 3 * SZ_1G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 4 * SZ_1G, len: 5 * SZ_1G }], expected_found: false, expected_start: SZ_2G, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"three holes; all holes too small; third biggest".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: 3 * SZ_1G, pending_extents: [PendingExtent { start: SZ_1G, len: SZ_1G }, PendingExtent { start: 3 * SZ_1G, len: 5 * SZ_1G }], expected_found: false, expected_start: 8 * SZ_1G, expected_len: SZ_2G },
    PendingExtentTestCase { name: c"hole entirely allocated by pending".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 0, len: 10 * SZ_1G }, PendingExtent { start: 0, len: 0 }], expected_found: false, expected_start: 10 * SZ_1G, expected_len: 0 },
    PendingExtentTestCase { name: c"pending extent at end of range".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 9 * SZ_1G, len: SZ_2G }, PendingExtent { start: 0, len: 0 }], expected_found: true, expected_start: 0, expected_len: 9 * SZ_1G },
    PendingExtentTestCase { name: c"zero length input".as_ptr(), hole_start: SZ_1G, hole_len: 0, min_hole_size: SZ_1G, pending_extents: [PendingExtent { start: 0, len: 0 }, PendingExtent { start: 0, len: 0 }], expected_found: false, expected_start: SZ_1G, expected_len: 0 },
];

#[repr(C)]
pub struct FirstPendingTestCase {
    pub name: *const core::ffi::c_char,
    pub hole_start: u64,
    pub hole_len: u64,
    pub pending_extent: PendingExtent,
    pub expected_found: bool,
    pub expected_pending_start: u64,
    pub expected_pending_end: u64,
}

static FIRST_PENDING_TESTS: &[FirstPendingTestCase] = &[
    FirstPendingTestCase { name: c"no pending extent".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, pending_extent: PendingExtent { start: 0, len: 0 }, expected_found: false, expected_pending_start: 0, expected_pending_end: 0 },
    FirstPendingTestCase { name: c"pending extent at search start".as_ptr(), hole_start: SZ_1G, hole_len: 9 * SZ_1G, pending_extent: PendingExtent { start: SZ_1G, len: SZ_1G }, expected_found: true, expected_pending_start: SZ_1G, expected_pending_end: SZ_2G - 1 },
    FirstPendingTestCase { name: c"pending extent overlapping search start".as_ptr(), hole_start: SZ_1G, hole_len: 9 * SZ_1G, pending_extent: PendingExtent { start: 0, len: SZ_2G }, expected_found: true, expected_pending_start: 0, expected_pending_end: SZ_2G - 1 },
    FirstPendingTestCase { name: c"pending extent inside search range".as_ptr(), hole_start: 0, hole_len: 10 * SZ_1G, pending_extent: PendingExtent { start: SZ_2G, len: SZ_1G }, expected_found: true, expected_pending_start: SZ_2G, expected_pending_end: 3 * SZ_1G - 1 },
    FirstPendingTestCase { name: c"pending extent outside search range".as_ptr(), hole_start: 0, hole_len: SZ_1G, pending_extent: PendingExtent { start: SZ_2G, len: SZ_1G }, expected_found: false, expected_pending_start: 0, expected_pending_end: 0 },
    FirstPendingTestCase { name: c"pending extent overlapping end of search range".as_ptr(), hole_start: 0, hole_len: SZ_2G, pending_extent: PendingExtent { start: SZ_1G, len: SZ_2G }, expected_found: true, expected_pending_start: SZ_1G, expected_pending_end: 3 * SZ_1G - 1 },
];

#[repr(C)] pub struct BtrfsFsInfo { _private: [u8; 0] }
#[repr(C)] pub struct BtrfsDevice { pub fs_info: *mut BtrfsFsInfo, pub alloc_state: [u8; 0] }
extern "C" {
    fn test_msg(message: *const core::ffi::c_char);
    fn test_std_err(error: i32);
    fn test_err(message: *const core::ffi::c_char);
    fn btrfs_alloc_dummy_fs_info(nodesize: u32, sectorsize: u32) -> *mut BtrfsFsInfo;
    fn btrfs_alloc_dummy_device(fs_info: *mut BtrfsFsInfo) -> *mut BtrfsDevice;
    fn btrfs_free_dummy_fs_info(fs_info: *mut BtrfsFsInfo);
    fn btrfs_set_extent_bit(state: *mut core::ffi::c_void, start: u64, end: u64, bit: u32, cached_state: *mut core::ffi::c_void);
    fn btrfs_clear_extent_bit(state: *mut core::ffi::c_void, start: u64, end: u64, bit: u32, cached_state: *mut core::ffi::c_void);
    fn mutex_lock(mutex: *mut core::ffi::c_void);
    fn mutex_unlock(mutex: *mut core::ffi::c_void);
    fn btrfs_find_hole_in_pending_extents(device: *mut BtrfsDevice, start: *mut u64, len: *mut u64, min_hole_size: u64) -> bool;
    fn btrfs_first_pending_extent(device: *mut BtrfsDevice, start: u64, len: u64, pending_start: *mut u64, pending_end: *mut u64) -> bool;
}

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const TEST_ALLOC_FS_INFO: i32 = 0;
const CHUNK_ALLOCATED: u32 = 0;

unsafe fn test_find_hole_in_pending(sectorsize: u32, nodesize: u32) -> i32 {
    test_msg(c"running find_hole_in_pending_extents tests".as_ptr());
    let fs_info = btrfs_alloc_dummy_fs_info(nodesize, sectorsize);
    if fs_info.is_null() { test_std_err(TEST_ALLOC_FS_INFO); return -ENOMEM; }
    let device = btrfs_alloc_dummy_device(fs_info);
    if device.is_null() { test_err(c"failed to allocate dummy device".as_ptr()); btrfs_free_dummy_fs_info(fs_info); return -EINVAL; }
    (*device).fs_info = fs_info;
    let mut ret = 0;
    for test_case in FIND_HOLE_TESTS {
        let mut hole_start = test_case.hole_start;
        let mut hole_len = test_case.hole_len;
        for extent in &test_case.pending_extents {
            if extent.len != 0 { btrfs_set_extent_bit(core::ptr::null_mut(), extent.start, extent.start + extent.len - 1, CHUNK_ALLOCATED, core::ptr::null_mut()); }
        }
        let found = btrfs_find_hole_in_pending_extents(device, &mut hole_start, &mut hole_len, test_case.min_hole_size);
        if found != test_case.expected_found || hole_start != test_case.expected_start || hole_len != test_case.expected_len { ret = -EINVAL; }
        btrfs_clear_extent_bit(core::ptr::null_mut(), 0, u64::MAX, CHUNK_ALLOCATED, core::ptr::null_mut());
        if ret != 0 { break; }
    }
    btrfs_free_dummy_fs_info(fs_info); ret
}

unsafe fn test_first_pending_extent(sectorsize: u32, nodesize: u32) -> i32 {
    test_msg(c"running first_pending_extent tests".as_ptr());
    let fs_info = btrfs_alloc_dummy_fs_info(nodesize, sectorsize);
    if fs_info.is_null() { test_std_err(TEST_ALLOC_FS_INFO); return -ENOMEM; }
    let device = btrfs_alloc_dummy_device(fs_info);
    if device.is_null() { test_err(c"failed to allocate dummy device".as_ptr()); btrfs_free_dummy_fs_info(fs_info); return -EINVAL; }
    (*device).fs_info = fs_info;
    let mut ret = 0;
    for test_case in FIRST_PENDING_TESTS {
        let mut pending_start = 0; let mut pending_end = 0;
        let found = btrfs_first_pending_extent(device, test_case.hole_start, test_case.hole_len, &mut pending_start, &mut pending_end);
        if found != test_case.expected_found || (found && (pending_start != test_case.expected_pending_start || pending_end != test_case.expected_pending_end)) { ret = -EINVAL; }
        btrfs_clear_extent_bit(core::ptr::null_mut(), 0, u64::MAX, CHUNK_ALLOCATED, core::ptr::null_mut());
        if ret != 0 { break; }
    }
    btrfs_free_dummy_fs_info(fs_info); ret
}

#[no_mangle]
pub unsafe extern "C" fn btrfs_test_chunk_allocation(sectorsize: u32, nodesize: u32) -> i32 {
    let ret = test_first_pending_extent(sectorsize, nodesize);
    if ret != 0 { return ret; }
    let ret = test_find_hole_in_pending(sectorsize, nodesize);
    if ret != 0 { return ret; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
