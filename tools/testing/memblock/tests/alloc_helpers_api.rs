// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from alloc_helpers_api.c. Original dependency intent:
// #include "alloc_helpers_api.h"

use core::ffi::{c_char, c_int, c_void};

type PhysAddrT = u64;

const SZ_2: PhysAddrT = 2;
const SZ_8: PhysAddrT = 8;
const SZ_16: PhysAddrT = 16;
const SZ_32: PhysAddrT = 32;
const SZ_64: PhysAddrT = 64;
const SZ_128: PhysAddrT = 128;

unsafe extern "C" {
    static mut memblock: Memblock;
    static SMP_CACHE_BYTES: PhysAddrT;
    static MEM_SIZE: PhysAddrT;

    fn setup_memblock();
    fn memblock_end_of_DRAM() -> PhysAddrT;
    fn memblock_start_of_DRAM() -> PhysAddrT;
    fn memblock_alloc_from(size: PhysAddrT, align: PhysAddrT, min_addr: PhysAddrT) -> *mut c_void;
    fn memblock_reserve_kern(base: PhysAddrT, size: PhysAddrT);
    fn memblock_reserve(base: PhysAddrT, size: PhysAddrT);
    fn memblock_set_bottom_up(bottom_up: bool);

    fn prefix_reset();
    fn prefix_push(prefix: *const c_char);
    fn prefix_pop();
    fn test_pass_pop();
    fn test_print(fmt: *const c_char, ...);

    fn reset_memblock_attributes();
    fn dummy_physical_memory_init();
    fn dummy_physical_memory_cleanup();

    fn run_top_down(test: unsafe fn() -> c_int);
    fn run_bottom_up(test: unsafe fn() -> c_int);

    fn ASSERT_NE(actual: *const c_void, expected: *const c_void);
    fn ASSERT_MEM_EQ(ptr: *const c_void, value: c_int, size: PhysAddrT);
    fn ASSERT_EQ_U64(actual: PhysAddrT, expected: PhysAddrT);
    fn ASSERT_EQ_INT(actual: c_int, expected: c_int);
}

#[repr(C)]
struct MemblockRegion {
    base: PhysAddrT,
    size: PhysAddrT,
}

#[repr(C)]
struct MemblockType {
    cnt: c_int,
    total_size: PhysAddrT,
    regions: *mut MemblockRegion,
}

#[repr(C)]
struct Memblock {
    reserved: MemblockType,
}

unsafe fn prefix_push_empty() {
    unsafe { prefix_push(c"".as_ptr()) };
}

/*
 * A simple test that tries to allocate a memory region above a specified,
 * aligned address:
 *
 *             +
 *  |          +-----------+         |
 *  |          |    rgn    |         |
 *  +----------+-----------+---------+
 *             ^
 *             |
 *             Aligned min_addr
 *
 * Expect to allocate a cleared region at the minimal memory address.
 */
unsafe fn alloc_from_simple_generic_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let size: PhysAddrT = SZ_16;
    let min_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    min_addr = unsafe { memblock_end_of_DRAM() - SMP_CACHE_BYTES };

    allocated_ptr = unsafe { memblock_alloc_from(size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_MEM_EQ(allocated_ptr, 0, size) };

    unsafe { ASSERT_EQ_U64((*rgn).size, size) };
    unsafe { ASSERT_EQ_U64((*rgn).base, min_addr) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region above a certain address.
 * The minimal address here is not aligned:
 *
 *         +      +
 *  |      +      +---------+            |
 *  |      |      |   rgn   |            |
 *  +------+------+---------+------------+
 *         ^      ^------.
 *         |             |
 *       min_addr        Aligned address
 *                       boundary
 *
 * Expect to allocate a cleared region at the closest aligned memory address.
 */
unsafe fn alloc_from_misaligned_generic_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let size: PhysAddrT = SZ_32;
    let min_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    /* A misaligned address */
    min_addr = unsafe { memblock_end_of_DRAM() - (SMP_CACHE_BYTES * 2 - 1) };

    allocated_ptr = unsafe { memblock_alloc_from(size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_MEM_EQ(allocated_ptr, 0, size) };

    unsafe { ASSERT_EQ_U64((*rgn).size, size) };
    unsafe { ASSERT_EQ_U64((*rgn).base, memblock_end_of_DRAM() - SMP_CACHE_BYTES) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region above an address that is too
 * close to the end of the memory:
 *
 *              +        +
 *  |           +--------+---+      |
 *  |           |   rgn  +   |      |
 *  +-----------+--------+---+------+
 *              ^        ^
 *              |        |
 *              |        min_addr
 *              |
 *              Aligned address
 *              boundary
 *
 * Expect to prioritize granting memory over satisfying the minimal address
 * requirement.
 */
unsafe fn alloc_from_top_down_high_addr_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let size: PhysAddrT = SZ_32;
    let min_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    /* The address is too close to the end of the memory */
    min_addr = unsafe { memblock_end_of_DRAM() - SZ_16 };

    allocated_ptr = unsafe { memblock_alloc_from(size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).size, size) };
    unsafe { ASSERT_EQ_U64((*rgn).base, memblock_end_of_DRAM() - SMP_CACHE_BYTES) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region when there is no space
 * available above the minimal address above a certain address:
 *
 *                     +
 *  |        +---------+-------------|
 *  |        |   rgn   |             |
 *  +--------+---------+-------------+
 *                     ^
 *                     |
 *                     min_addr
 *
 * Expect to prioritize granting memory over satisfying the minimal address
 * requirement and to allocate next to the previously reserved region. The
 * regions get merged into one.
 */
unsafe fn alloc_from_top_down_no_space_above_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let r1_size: PhysAddrT = SZ_64;
    let r2_size: PhysAddrT = SZ_2;
    let total_size: PhysAddrT = r1_size + r2_size;
    let min_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    min_addr = unsafe { memblock_end_of_DRAM() - SMP_CACHE_BYTES * 2 };

    /* No space above this address */
    unsafe { memblock_reserve_kern(min_addr, r2_size) };

    allocated_ptr = unsafe { memblock_alloc_from(r1_size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).base, min_addr - r1_size) };
    unsafe { ASSERT_EQ_U64((*rgn).size, total_size) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, total_size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region with a minimal address below
 * the start address of the available memory. As the allocation is top-down,
 * first reserve a region that will force allocation near the start.
 * Expect successful allocation and merge of both regions.
 */
unsafe fn alloc_from_top_down_min_addr_cap_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let r1_size: PhysAddrT = SZ_64;
    let min_addr: PhysAddrT;
    let start_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    start_addr = unsafe { memblock_start_of_DRAM() as PhysAddrT };
    min_addr = unsafe { start_addr - SMP_CACHE_BYTES * 3 };

    unsafe { memblock_reserve_kern(start_addr + r1_size, MEM_SIZE - r1_size) };

    allocated_ptr = unsafe { memblock_alloc_from(r1_size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).base, start_addr) };
    unsafe { ASSERT_EQ_U64((*rgn).size, MEM_SIZE) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, MEM_SIZE) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region above an address that is too
 * close to the end of the memory:
 *
 *                             +
 *  |-----------+              +     |
 *  |    rgn    |              |     |
 *  +-----------+--------------+-----+
 *  ^                          ^
 *  |                          |
 *  Aligned address            min_addr
 *  boundary
 *
 * Expect to prioritize granting memory over satisfying the minimal address
 * requirement. Allocation happens at beginning of the available memory.
 */
unsafe fn alloc_from_bottom_up_high_addr_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let size: PhysAddrT = SZ_32;
    let min_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    /* The address is too close to the end of the memory */
    min_addr = unsafe { memblock_end_of_DRAM() - SZ_8 };

    allocated_ptr = unsafe { memblock_alloc_from(size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).size, size) };
    unsafe { ASSERT_EQ_U64((*rgn).base, memblock_start_of_DRAM()) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region when there is no space
 * available above the minimal address above a certain address:
 *
 *                   +
 *  |-----------+    +-------------------|
 *  |    rgn    |    |                   |
 *  +-----------+----+-------------------+
 *                   ^
 *                   |
 *                   min_addr
 *
 * Expect to prioritize granting memory over satisfying the minimal address
 * requirement and to allocate at the beginning of the available memory.
 */
unsafe fn alloc_from_bottom_up_no_space_above_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let r1_size: PhysAddrT = SZ_64;
    let min_addr: PhysAddrT;
    let r2_size: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    min_addr = unsafe { memblock_start_of_DRAM() + SZ_128 };
    r2_size = unsafe { memblock_end_of_DRAM() - min_addr };

    /* No space above this address */
    unsafe { memblock_reserve(min_addr - SMP_CACHE_BYTES, r2_size) };

    allocated_ptr = unsafe { memblock_alloc_from(r1_size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).base, memblock_start_of_DRAM()) };
    unsafe { ASSERT_EQ_U64((*rgn).size, r1_size) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 2) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, r1_size + r2_size) };

    unsafe { test_pass_pop() };

    0
}

/*
 * A test that tries to allocate a memory region with a minimal address below
 * the start address of the available memory. Expect to allocate a region
 * at the beginning of the available memory.
 */
unsafe fn alloc_from_bottom_up_min_addr_cap_check() -> c_int {
    let rgn: *mut MemblockRegion = unsafe { memblock.reserved.regions.add(0) };
    let mut allocated_ptr: *mut c_void = core::ptr::null_mut();
    let r1_size: PhysAddrT = SZ_64;
    let min_addr: PhysAddrT;
    let start_addr: PhysAddrT;

    unsafe { prefix_push_empty() };
    unsafe { setup_memblock() };

    start_addr = unsafe { memblock_start_of_DRAM() as PhysAddrT };
    min_addr = unsafe { start_addr - SMP_CACHE_BYTES * 3 };

    allocated_ptr = unsafe { memblock_alloc_from(r1_size, SMP_CACHE_BYTES, min_addr) };

    unsafe { ASSERT_NE(allocated_ptr, core::ptr::null()) };
    unsafe { ASSERT_EQ_U64((*rgn).base, start_addr) };
    unsafe { ASSERT_EQ_U64((*rgn).size, r1_size) };

    unsafe { ASSERT_EQ_INT(memblock.reserved.cnt, 1) };
    unsafe { ASSERT_EQ_U64(memblock.reserved.total_size, r1_size) };

    unsafe { test_pass_pop() };

    0
}

/* Test case wrappers */
unsafe fn alloc_from_simple_check() -> c_int {
    unsafe { test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_from_simple_check".as_ptr()) };
    unsafe { run_top_down(alloc_from_simple_generic_check) };
    unsafe { run_bottom_up(alloc_from_simple_generic_check) };

    0
}

unsafe fn alloc_from_misaligned_check() -> c_int {
    unsafe { test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_from_misaligned_check".as_ptr()) };
    unsafe { run_top_down(alloc_from_misaligned_generic_check) };
    unsafe { run_bottom_up(alloc_from_misaligned_generic_check) };

    0
}

unsafe fn alloc_from_high_addr_check() -> c_int {
    unsafe { test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_from_high_addr_check".as_ptr()) };
    unsafe { memblock_set_bottom_up(false) };
    unsafe { alloc_from_top_down_high_addr_check() };
    unsafe { memblock_set_bottom_up(true) };
    unsafe { alloc_from_bottom_up_high_addr_check() };

    0
}

unsafe fn alloc_from_no_space_above_check() -> c_int {
    unsafe { test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_from_no_space_above_check".as_ptr()) };
    unsafe { memblock_set_bottom_up(false) };
    unsafe { alloc_from_top_down_no_space_above_check() };
    unsafe { memblock_set_bottom_up(true) };
    unsafe { alloc_from_bottom_up_no_space_above_check() };

    0
}

unsafe fn alloc_from_min_addr_cap_check() -> c_int {
    unsafe { test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_from_min_addr_cap_check".as_ptr()) };
    unsafe { memblock_set_bottom_up(false) };
    unsafe { alloc_from_top_down_min_addr_cap_check() };
    unsafe { memblock_set_bottom_up(true) };
    unsafe { alloc_from_bottom_up_min_addr_cap_check() };

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memblock_alloc_helpers_checks() -> c_int {
    let func_testing: *const c_char = c"memblock_alloc_from".as_ptr();

    unsafe { prefix_reset() };
    unsafe { prefix_push(func_testing) };
    unsafe { test_print(c"Running %s tests...\n".as_ptr(), func_testing) };

    unsafe { reset_memblock_attributes() };
    unsafe { dummy_physical_memory_init() };

    unsafe { alloc_from_simple_check() };
    unsafe { alloc_from_misaligned_check() };
    unsafe { alloc_from_high_addr_check() };
    unsafe { alloc_from_no_space_above_check() };
    unsafe { alloc_from_min_addr_cap_check() };

    unsafe { dummy_physical_memory_cleanup() };

    unsafe { prefix_pop() };

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
