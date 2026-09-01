/* SPDX-License-Identifier: GPL-2.0-or-later */

// C includes removed from executable Rust:
// <stdlib.h>, <assert.h>, <linux/types.h>, <linux/seq_file.h>,
// <linux/memblock.h>, <linux/sizes.h>, <linux/printk.h>,
// <../selftests/kselftest.h>

use core::ffi::{c_char, c_int};

pub const MEM_SIZE: usize = SZ_32K;
pub const PHYS_MEM_SIZE: usize = SZ_16M;
pub const NUMA_NODES: c_int = 8;

pub const INIT_MEMBLOCK_REGIONS: c_int = 128;
pub const INIT_MEMBLOCK_RESERVED_REGIONS: c_int = INIT_MEMBLOCK_REGIONS;

pub type test_flags = c_int;

/* No special request. */
pub const TEST_F_NONE: test_flags = 0x0;
/* Perform raw allocations (no zeroing of memory). */
pub const TEST_F_RAW: test_flags = 0x1;
/* Perform allocations on the exact node specified. */
pub const TEST_F_EXACT: test_flags = 0x2;

/**
 * ASSERT_EQ():
 * Check the condition
 * @_expected == @_seen
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_EQ {
    ($_expected:expr, $_seen:expr) => {{
        if ($_expected) != ($_seen) {
            unsafe { test_fail() };
        }
        assert!(($_expected) == ($_seen));
    }};
}

macro_rules! ASSERT_TRUE {
    ($_seen:expr) => {
        ASSERT_EQ!(true, $_seen)
    };
}

macro_rules! ASSERT_FALSE {
    ($_seen:expr) => {
        ASSERT_EQ!(false, $_seen)
    };
}

/**
 * ASSERT_NE():
 * Check the condition
 * @_expected != @_seen
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_NE {
    ($_expected:expr, $_seen:expr) => {{
        if ($_expected) == ($_seen) {
            unsafe { test_fail() };
        }
        assert!(($_expected) != ($_seen));
    }};
}

/**
 * ASSERT_LT():
 * Check the condition
 * @_expected < @_seen
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_LT {
    ($_expected:expr, $_seen:expr) => {{
        if ($_expected) >= ($_seen) {
            unsafe { test_fail() };
        }
        assert!(($_expected) < ($_seen));
    }};
}

/**
 * ASSERT_LE():
 * Check the condition
 * @_expected <= @_seen
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_LE {
    ($_expected:expr, $_seen:expr) => {{
        if ($_expected) > ($_seen) {
            unsafe { test_fail() };
        }
        assert!(($_expected) <= ($_seen));
    }};
}

/**
 * ASSERT_MEM_EQ():
 * Check that the first @_size bytes of @_seen are all equal to @_expected.
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_MEM_EQ {
    ($_seen:expr, $_expected:expr, $_size:expr) => {{
        let mut _i: c_int = 0;
        while _i < ($_size) {
            ASSERT_EQ!(
                unsafe { *(($_seen) as *const c_char).offset(_i as isize) },
                ($_expected) as c_char
            );
            _i += 1;
        }
    }};
}

/**
 * ASSERT_MEM_NE():
 * Check that none of the first @_size bytes of @_seen are equal to @_expected.
 * If false, print failed test message (if running with --verbose) and then
 * assert.
 */
macro_rules! ASSERT_MEM_NE {
    ($_seen:expr, $_expected:expr, $_size:expr) => {{
        let mut _i: c_int = 0;
        while _i < ($_size) {
            ASSERT_NE!(
                unsafe { *(($_seen) as *const c_char).offset(_i as isize) },
                ($_expected) as c_char
            );
            _i += 1;
        }
    }};
}

macro_rules! PREFIX_PUSH {
    () => {
        prefix_push(concat!(module_path!(), "\0").as_ptr() as *const c_char)
    };
}

/*
 * Available memory registered with memblock needs to be valid for allocs
 * test to run. This is a convenience wrapper for memory allocated in
 * dummy_physical_memory_init() that is later registered with memblock
 * in setup_memblock().
 */
#[repr(C)]
pub struct test_memory {
    pub base: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct region {
    pub base: phys_addr_t,
    pub size: phys_addr_t,
}

#[inline]
pub unsafe fn region_end(rgn: *mut memblock_region) -> phys_addr_t {
    unsafe { (*rgn).base + (*rgn).size }
}

unsafe extern "C" {
    pub fn reset_memblock_regions();
    pub fn reset_memblock_attributes();
    pub fn setup_memblock();
    pub fn setup_numa_memblock(node_fracs: *const core::ffi::c_uint);
    pub fn dummy_physical_memory_init();
    pub fn dummy_physical_memory_cleanup();
    pub fn dummy_physical_memory_base() -> phys_addr_t;
    pub fn parse_args(argc: c_int, argv: *mut *mut c_char);

    pub fn test_fail();
    pub fn test_pass();
    pub fn test_print(fmt: *const c_char, ...);
    pub fn prefix_reset();
    pub fn prefix_push(prefix: *const c_char);
    pub fn prefix_pop();

    pub fn memblock_set_bottom_up(enable: bool);
}

#[inline]
pub unsafe fn test_pass_pop() {
    unsafe {
        test_pass();
        prefix_pop();
    }
}

#[inline]
pub unsafe fn run_top_down(func: unsafe extern "C" fn() -> c_int) {
    unsafe {
        memblock_set_bottom_up(false);
        prefix_push(c"top-down".as_ptr());
        func();
        prefix_pop();
    }
}

#[inline]
pub unsafe fn run_bottom_up(func: unsafe extern "C" fn() -> c_int) {
    unsafe {
        memblock_set_bottom_up(true);
        prefix_push(c"bottom-up".as_ptr());
        func();
        prefix_pop();
    }
}

#[inline]
pub unsafe fn assert_mem_content(mem: *mut core::ffi::c_void, size: c_int, flags: c_int) {
    if (flags & TEST_F_RAW) != 0 {
        ASSERT_MEM_NE!(mem, 0, size);
    } else {
        ASSERT_MEM_EQ!(mem, 0, size);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
