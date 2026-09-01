// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from alloc_api.c. Original C dependency: "alloc_api.h".

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type phys_addr_t = u64;

const TEST_F_NONE: c_int = 0;
const TEST_F_RAW: c_int = 1;

const SZ_2: phys_addr_t = 2;
const SZ_8: phys_addr_t = 8;
const SZ_16: phys_addr_t = 16;
const SZ_64: phys_addr_t = 64;
const SZ_128: phys_addr_t = 128;
const SZ_256: phys_addr_t = 256;
const SZ_512: phys_addr_t = 512;
const SZ_1K: phys_addr_t = 1024;

extern "C" {
    static SMP_CACHE_BYTES: phys_addr_t;
    static MEM_SIZE: phys_addr_t;
}

#[repr(C)]
struct region {
    base: phys_addr_t,
    size: phys_addr_t,
}

#[repr(C)]
struct memblock_region {
    base: phys_addr_t,
    size: phys_addr_t,
}

#[repr(C)]
struct memblock_type {
    cnt: c_int,
    total_size: phys_addr_t,
    regions: *mut memblock_region,
}

#[repr(C)]
struct memblock {
    reserved: memblock_type,
}

extern "C" {
    static mut memblock: memblock;

    fn memblock_alloc_raw(size: phys_addr_t, align: phys_addr_t) -> *mut c_void;
    fn memblock_alloc(size: phys_addr_t, align: phys_addr_t) -> *mut c_void;
    fn memblock_end_of_DRAM() -> phys_addr_t;
    fn memblock_start_of_DRAM() -> phys_addr_t;
    fn memblock_reserve(base: phys_addr_t, size: phys_addr_t);
    fn memblock_reserve_kern(base: phys_addr_t, size: phys_addr_t);
    fn memblock_set_bottom_up(enable: bool);

    fn setup_memblock();
    fn reset_memblock_regions();
    fn reset_memblock_attributes();
    fn dummy_physical_memory_init();
    fn dummy_physical_memory_cleanup();

    fn assert_mem_content(ptr: *mut c_void, size: phys_addr_t, flags: c_int);
    fn test_pass_pop();
    fn test_print(fmt: *const c_char, ...);
    fn prefix_reset();
    fn prefix_push(prefix: *const c_char);
    fn prefix_pop();
    fn run_top_down(test: unsafe extern "C" fn() -> c_int);
    fn run_bottom_up(test: unsafe extern "C" fn() -> c_int);
}

macro_rules! PREFIX_PUSH {
    () => {
        prefix_push(concat!(module_path!(), "::", line!(), "\0").as_ptr() as *const c_char)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

static mut alloc_test_flags: c_int = TEST_F_NONE;

#[inline]
unsafe fn get_memblock_alloc_name(flags: c_int) -> *const c_char {
    if (flags & TEST_F_RAW) != 0 {
        return b"memblock_alloc_raw\0".as_ptr() as *const c_char;
    }
    b"memblock_alloc\0".as_ptr() as *const c_char
}

#[inline]
unsafe fn run_memblock_alloc(size: phys_addr_t, align: phys_addr_t) -> *mut c_void {
    if (alloc_test_flags & TEST_F_RAW) != 0 {
        return memblock_alloc_raw(size, align);
    }
    memblock_alloc(size, align)
}

/*
 * A simple test that tries to allocate a small memory region.
 * Expect to allocate an aligned region near the end of the available memory.
 */
unsafe extern "C" fn alloc_top_down_simple_check() -> c_int {
    let rgn = (*memblock.reserved.regions.add(0)) as *mut memblock_region;
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let size: phys_addr_t = SZ_2;
    let expected_start: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    expected_start = memblock_end_of_DRAM() - SMP_CACHE_BYTES;

    allocated_ptr = run_memblock_alloc(size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, size);
    ASSERT_EQ!((*rgn).base, expected_start);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory next to a reserved region that starts at
 * the misaligned address. Expect to create two separate entries, with the new
 * entry aligned to the provided alignment:
 *
 *              +
 * |            +--------+         +--------|
 * |            |  rgn2  |         |  rgn1  |
 * +------------+--------+---------+--------+
 *              ^
 *              |
 *              Aligned address boundary
 *
 * The allocation direction is top-down and region arrays are sorted from lower
 * to higher addresses, so the new region will be the first entry in
 * memory.reserved array. The previously reserved region does not get modified.
 * Region counter and total size get updated.
 */
unsafe extern "C" fn alloc_top_down_disjoint_check() -> c_int {
    /* After allocation, this will point to the "old" region */
    let rgn1 = memblock.reserved.regions.add(1);
    let rgn2 = memblock.reserved.regions.add(0);
    let mut r1 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r2_size: phys_addr_t = SZ_16;
    /* Use custom alignment */
    let alignment: phys_addr_t = SMP_CACHE_BYTES * 2;
    let total_size: phys_addr_t;
    let expected_start: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    r1.base = memblock_end_of_DRAM() - SZ_2;
    r1.size = SZ_2;

    total_size = r1.size + r2_size;
    expected_start = memblock_end_of_DRAM() - alignment;

    memblock_reserve(r1.base, r1.size);

    allocated_ptr = run_memblock_alloc(r2_size, alignment);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r2_size, alloc_test_flags);

    ASSERT_EQ!((*rgn1).size, r1.size);
    ASSERT_EQ!((*rgn1).base, r1.base);

    ASSERT_EQ!((*rgn2).size, r2_size);
    ASSERT_EQ!((*rgn2).base, expected_start);

    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there is enough space at the end
 * of the previously reserved block (i.e. first fit):
 *
 *  |              +--------+--------------|
 *  |              |   r1   |      r2      |
 *  +--------------+--------+--------------+
 *
 * Expect a merge of both regions. Only the region size gets updated.
 */
unsafe extern "C" fn alloc_top_down_before_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    /*
     * The first region ends at the aligned address to test region merging
     */
    let r1_size: phys_addr_t = SMP_CACHE_BYTES;
    let r2_size: phys_addr_t = SZ_512;
    let total_size: phys_addr_t = r1_size + r2_size;

    PREFIX_PUSH!();
    setup_memblock();

    memblock_reserve_kern(memblock_end_of_DRAM() - total_size, r1_size);

    allocated_ptr = run_memblock_alloc(r2_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r2_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, total_size);
    ASSERT_EQ!((*rgn).base, memblock_end_of_DRAM() - total_size);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there is not enough space at the
 * end of the previously reserved block (i.e. second fit):
 *
 *  |            +-----------+------+     |
 *  |            |     r2    |  r1  |     |
 *  +------------+-----------+------+-----+
 *
 * Expect a merge of both regions. Both the base address and size of the region
 * get updated.
 */
unsafe extern "C" fn alloc_top_down_after_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut r1 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r2_size: phys_addr_t = SZ_512;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    /*
     * The first region starts at the aligned address to test region merging
     */
    r1.base = memblock_end_of_DRAM() - SMP_CACHE_BYTES;
    r1.size = SZ_8;

    total_size = r1.size + r2_size;

    memblock_reserve_kern(r1.base, r1.size);

    allocated_ptr = run_memblock_alloc(r2_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r2_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, total_size);
    ASSERT_EQ!((*rgn).base, r1.base - r2_size);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there are two reserved regions with
 * a gap too small to fit the new region:
 *
 *  |       +--------+----------+   +------|
 *  |       |   r3   |    r2    |   |  r1  |
 *  +-------+--------+----------+---+------+
 *
 * Expect to allocate a region before the one that starts at the lower address,
 * and merge them into one. The region counter and total size fields get
 * updated.
 */
unsafe extern "C" fn alloc_top_down_second_fit_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut r1 = region { base: 0, size: 0 };
    let mut r2 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r3_size: phys_addr_t = SZ_1K;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    r1.base = memblock_end_of_DRAM() - SZ_512;
    r1.size = SZ_512;

    r2.base = r1.base - SZ_512;
    r2.size = SZ_256;

    total_size = r1.size + r2.size + r3_size;

    memblock_reserve_kern(r1.base, r1.size);
    memblock_reserve_kern(r2.base, r2.size);

    allocated_ptr = run_memblock_alloc(r3_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r3_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, r2.size + r3_size);
    ASSERT_EQ!((*rgn).base, r2.base - r3_size);

    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there are two reserved regions with
 * a gap big enough to accommodate the new region:
 *
 *  |     +--------+--------+--------+     |
 *  |     |   r2   |   r3   |   r1   |     |
 *  +-----+--------+--------+--------+-----+
 *
 * Expect to merge all of them, creating one big entry in memblock.reserved
 * array. The region counter and total size fields get updated.
 */
unsafe extern "C" fn alloc_in_between_generic_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut r1 = region { base: 0, size: 0 };
    let mut r2 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let gap_size: phys_addr_t = SMP_CACHE_BYTES;
    let r3_size: phys_addr_t = SZ_64;
    /*
     * Calculate regions size so there's just enough space for the new entry
     */
    let rgn_size: phys_addr_t = (MEM_SIZE - (2 * gap_size + r3_size)) / 2;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    r1.size = rgn_size;
    r1.base = memblock_end_of_DRAM() - (gap_size + rgn_size);

    r2.size = rgn_size;
    r2.base = memblock_start_of_DRAM() + gap_size;

    total_size = r1.size + r2.size + r3_size;

    memblock_reserve_kern(r1.base, r1.size);
    memblock_reserve_kern(r2.base, r2.size);

    allocated_ptr = run_memblock_alloc(r3_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r3_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, total_size);
    ASSERT_EQ!((*rgn).base, r1.base - r2.size - r3_size);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when the memory is filled with reserved
 * regions with memory gaps too small to fit the new region:
 *
 * +-------+
 * |  new  |
 * +--+----+
 *    |    +-----+    +-----+    +-----+    |
 *    |    | res |    | res |    | res |    |
 *    +----+-----+----+-----+----+-----+----+
 *
 * Expect no allocation to happen.
 */
unsafe extern "C" fn alloc_small_gaps_generic_check() -> c_int {
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let region_size: phys_addr_t = SZ_1K;
    let gap_size: phys_addr_t = SZ_256;
    let mut region_end: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    region_end = memblock_start_of_DRAM();

    while region_end < memblock_end_of_DRAM() {
        memblock_reserve(region_end + gap_size, region_size);
        region_end += gap_size + region_size;
    }

    allocated_ptr = run_memblock_alloc(region_size, SMP_CACHE_BYTES);

    ASSERT_EQ!(allocated_ptr, ptr::null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when all memory is reserved.
 * Expect no allocation to happen.
 */
unsafe extern "C" fn alloc_all_reserved_generic_check() -> c_int {
    let mut allocated_ptr: *mut c_void = ptr::null_mut();

    PREFIX_PUSH!();
    setup_memblock();

    /* Simulate full memory */
    memblock_reserve(memblock_start_of_DRAM(), MEM_SIZE);

    allocated_ptr = run_memblock_alloc(SZ_256, SMP_CACHE_BYTES);

    ASSERT_EQ!(allocated_ptr, ptr::null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when the memory is almost full,
 * with not enough space left for the new region:
 *
 *                                +-------+
 *                                |  new  |
 *                                +-------+
 *  |-----------------------------+   |
 *  |          reserved           |   |
 *  +-----------------------------+---+
 *
 * Expect no allocation to happen.
 */
unsafe extern "C" fn alloc_no_space_generic_check() -> c_int {
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let available_size: phys_addr_t = SZ_256;
    let reserved_size: phys_addr_t = MEM_SIZE - available_size;

    PREFIX_PUSH!();
    setup_memblock();

    /* Simulate almost-full memory */
    memblock_reserve(memblock_start_of_DRAM(), reserved_size);

    allocated_ptr = run_memblock_alloc(SZ_1K, SMP_CACHE_BYTES);

    ASSERT_EQ!(allocated_ptr, ptr::null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when the memory is almost full,
 * but there is just enough space left:
 *
 *  |---------------------------+---------|
 *  |          reserved         |   new   |
 *  +---------------------------+---------+
 *
 * Expect to allocate memory and merge all the regions. The total size field
 * gets updated.
 */
unsafe extern "C" fn alloc_limited_space_generic_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let available_size: phys_addr_t = SZ_256;
    let reserved_size: phys_addr_t = MEM_SIZE - available_size;

    PREFIX_PUSH!();
    setup_memblock();

    /* Simulate almost-full memory */
    memblock_reserve_kern(memblock_start_of_DRAM(), reserved_size);

    allocated_ptr = run_memblock_alloc(available_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, available_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, MEM_SIZE);
    ASSERT_EQ!((*rgn).base, memblock_start_of_DRAM());

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, MEM_SIZE);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there is no available memory
 * registered (i.e. memblock.memory has only a dummy entry).
 * Expect no allocation to happen.
 */
unsafe extern "C" fn alloc_no_memory_generic_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();

    PREFIX_PUSH!();

    reset_memblock_regions();

    allocated_ptr = run_memblock_alloc(SZ_1K, SMP_CACHE_BYTES);

    ASSERT_EQ!(allocated_ptr, ptr::null_mut());
    ASSERT_EQ!((*rgn).size, 0);
    ASSERT_EQ!((*rgn).base, 0);
    ASSERT_EQ!(memblock.reserved.total_size, 0);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a region that is larger than the total size of
 * available memory (memblock.memory):
 *
 *  +-----------------------------------+
 *  |                 new               |
 *  +-----------------------------------+
 *  |                                 |
 *  |                                 |
 *  +---------------------------------+
 *
 * Expect no allocation to happen.
 */
unsafe extern "C" fn alloc_too_large_generic_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();

    PREFIX_PUSH!();
    setup_memblock();

    allocated_ptr = run_memblock_alloc(MEM_SIZE + SZ_2, SMP_CACHE_BYTES);

    ASSERT_EQ!(allocated_ptr, ptr::null_mut());
    ASSERT_EQ!((*rgn).size, 0);
    ASSERT_EQ!((*rgn).base, 0);
    ASSERT_EQ!(memblock.reserved.total_size, 0);

    test_pass_pop();

    0
}

/*
 * A simple test that tries to allocate a small memory region.
 * Expect to allocate an aligned region at the beginning of the available
 * memory.
 */
unsafe extern "C" fn alloc_bottom_up_simple_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();

    PREFIX_PUSH!();
    setup_memblock();

    allocated_ptr = run_memblock_alloc(SZ_2, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, SZ_2, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, SZ_2);
    ASSERT_EQ!((*rgn).base, memblock_start_of_DRAM());

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, SZ_2);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory next to a reserved region that starts at
 * the misaligned address. Expect to create two separate entries, with the new
 * entry aligned to the provided alignment:
 *
 *                      +
 *  |    +----------+   +----------+     |
 *  |    |   rgn1   |   |   rgn2   |     |
 *  +----+----------+---+----------+-----+
 *                      ^
 *                      |
 *                      Aligned address boundary
 *
 * The allocation direction is bottom-up, so the new region will be the second
 * entry in memory.reserved array. The previously reserved region does not get
 * modified. Region counter and total size get updated.
 */
unsafe extern "C" fn alloc_bottom_up_disjoint_check() -> c_int {
    let rgn1 = memblock.reserved.regions.add(0);
    let rgn2 = memblock.reserved.regions.add(1);
    let mut r1 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r2_size: phys_addr_t = SZ_16;
    /* Use custom alignment */
    let alignment: phys_addr_t = SMP_CACHE_BYTES * 2;
    let total_size: phys_addr_t;
    let expected_start: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    r1.base = memblock_start_of_DRAM() + SZ_2;
    r1.size = SZ_2;

    total_size = r1.size + r2_size;
    expected_start = memblock_start_of_DRAM() + alignment;

    memblock_reserve(r1.base, r1.size);

    allocated_ptr = run_memblock_alloc(r2_size, alignment);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r2_size, alloc_test_flags);

    ASSERT_EQ!((*rgn1).size, r1.size);
    ASSERT_EQ!((*rgn1).base, r1.base);

    ASSERT_EQ!((*rgn2).size, r2_size);
    ASSERT_EQ!((*rgn2).base, expected_start);

    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there is enough space at
 * the beginning of the previously reserved block (i.e. first fit):
 *
 *  |------------------+--------+         |
 *  |        r1        |   r2   |         |
 *  +------------------+--------+---------+
 *
 * Expect a merge of both regions. Only the region size gets updated.
 */
unsafe extern "C" fn alloc_bottom_up_before_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r1_size: phys_addr_t = SZ_512;
    let r2_size: phys_addr_t = SZ_128;
    let total_size: phys_addr_t = r1_size + r2_size;

    PREFIX_PUSH!();
    setup_memblock();

    memblock_reserve_kern(memblock_start_of_DRAM() + r1_size, r2_size);

    allocated_ptr = run_memblock_alloc(r1_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r1_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, total_size);
    ASSERT_EQ!((*rgn).base, memblock_start_of_DRAM());

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there is not enough space at
 * the beginning of the previously reserved block (i.e. second fit):
 *
 *  |    +--------+--------------+         |
 *  |    |   r1   |      r2      |         |
 *  +----+--------+--------------+---------+
 *
 * Expect a merge of both regions. Only the region size gets updated.
 */
unsafe extern "C" fn alloc_bottom_up_after_check() -> c_int {
    let rgn = memblock.reserved.regions.add(0);
    let mut r1 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r2_size: phys_addr_t = SZ_512;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    /*
     * The first region starts at the aligned address to test region merging
     */
    r1.base = memblock_start_of_DRAM() + SMP_CACHE_BYTES;
    r1.size = SZ_64;

    total_size = r1.size + r2_size;

    memblock_reserve_kern(r1.base, r1.size);

    allocated_ptr = run_memblock_alloc(r2_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r2_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, total_size);
    ASSERT_EQ!((*rgn).base, r1.base);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory when there are two reserved regions, the
 * first one starting at the beginning of the available memory, with a gap too
 * small to fit the new region:
 *
 *  |------------+     +--------+--------+  |
 *  |     r1     |     |   r2   |   r3   |  |
 *  +------------+-----+--------+--------+--+
 *
 * Expect to allocate after the second region, which starts at the higher
 * address, and merge them into one. The region counter and total size fields
 * get updated.
 */
unsafe extern "C" fn alloc_bottom_up_second_fit_check() -> c_int {
    let rgn = memblock.reserved.regions.add(1);
    let mut r1 = region { base: 0, size: 0 };
    let mut r2 = region { base: 0, size: 0 };
    let mut allocated_ptr: *mut c_void = ptr::null_mut();
    let r3_size: phys_addr_t = SZ_1K;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_memblock();

    r1.base = memblock_start_of_DRAM();
    r1.size = SZ_512;

    r2.base = r1.base + r1.size + SZ_512;
    r2.size = SZ_256;

    total_size = r1.size + r2.size + r3_size;

    memblock_reserve_kern(r1.base, r1.size);
    memblock_reserve_kern(r2.base, r2.size);

    allocated_ptr = run_memblock_alloc(r3_size, SMP_CACHE_BYTES);

    ASSERT_NE!(allocated_ptr, ptr::null_mut());
    assert_mem_content(allocated_ptr, r3_size, alloc_test_flags);

    ASSERT_EQ!((*rgn).size, r2.size + r3_size);
    ASSERT_EQ!((*rgn).base, r2.base);

    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/* Test case wrappers */
unsafe extern "C" fn alloc_simple_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_simple_check\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    alloc_top_down_simple_check();
    memblock_set_bottom_up(true);
    alloc_bottom_up_simple_check();

    0
}

unsafe extern "C" fn alloc_disjoint_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_disjoint_check\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    alloc_top_down_disjoint_check();
    memblock_set_bottom_up(true);
    alloc_bottom_up_disjoint_check();

    0
}

unsafe extern "C" fn alloc_before_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_before_check\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    alloc_top_down_before_check();
    memblock_set_bottom_up(true);
    alloc_bottom_up_before_check();

    0
}

unsafe extern "C" fn alloc_after_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_after_check\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    alloc_top_down_after_check();
    memblock_set_bottom_up(true);
    alloc_bottom_up_after_check();

    0
}

unsafe extern "C" fn alloc_in_between_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_in_between_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_in_between_generic_check);
    run_bottom_up(alloc_in_between_generic_check);

    0
}

unsafe extern "C" fn alloc_second_fit_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_second_fit_check\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    alloc_top_down_second_fit_check();
    memblock_set_bottom_up(true);
    alloc_bottom_up_second_fit_check();

    0
}

unsafe extern "C" fn alloc_small_gaps_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_small_gaps_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_small_gaps_generic_check);
    run_bottom_up(alloc_small_gaps_generic_check);

    0
}

unsafe extern "C" fn alloc_all_reserved_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_all_reserved_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_all_reserved_generic_check);
    run_bottom_up(alloc_all_reserved_generic_check);

    0
}

unsafe extern "C" fn alloc_no_space_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_no_space_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_no_space_generic_check);
    run_bottom_up(alloc_no_space_generic_check);

    0
}

unsafe extern "C" fn alloc_limited_space_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_limited_space_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_limited_space_generic_check);
    run_bottom_up(alloc_limited_space_generic_check);

    0
}

unsafe extern "C" fn alloc_no_memory_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_no_memory_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_no_memory_generic_check);
    run_bottom_up(alloc_no_memory_generic_check);

    0
}

unsafe extern "C" fn alloc_too_large_check() -> c_int {
    test_print(b"\tRunning %s...\n\0".as_ptr() as *const c_char, b"alloc_too_large_check\0".as_ptr() as *const c_char);
    run_top_down(alloc_too_large_generic_check);
    run_bottom_up(alloc_too_large_generic_check);

    0
}

unsafe fn memblock_alloc_checks_internal(flags: c_int) -> c_int {
    let func: *const c_char = get_memblock_alloc_name(flags);

    alloc_test_flags = flags;
    prefix_reset();
    prefix_push(func);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, func);

    reset_memblock_attributes();
    dummy_physical_memory_init();

    alloc_simple_check();
    alloc_disjoint_check();
    alloc_before_check();
    alloc_after_check();
    alloc_second_fit_check();
    alloc_small_gaps_check();
    alloc_in_between_check();
    alloc_all_reserved_check();
    alloc_no_space_check();
    alloc_limited_space_check();
    alloc_no_memory_check();
    alloc_too_large_check();

    dummy_physical_memory_cleanup();

    prefix_pop();

    0
}

#[no_mangle]
pub unsafe extern "C" fn memblock_alloc_checks() -> c_int {
    memblock_alloc_checks_internal(TEST_F_NONE);
    memblock_alloc_checks_internal(TEST_F_RAW);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
