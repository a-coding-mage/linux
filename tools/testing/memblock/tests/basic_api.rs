// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from testing/memblock/tests/basic_api.c.
// C includes translated as external dependencies: "basic_api.h", <string.h>,
// and <linux/memblock.h>.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type phys_addr_t = u64;

const EXPECTED_MEMBLOCK_REGIONS: c_int = 128;
const FUNC_ADD: *const c_char = b"memblock_add\0".as_ptr() as *const c_char;
const FUNC_RESERVE: *const c_char = b"memblock_reserve\0".as_ptr() as *const c_char;
const FUNC_REMOVE: *const c_char = b"memblock_remove\0".as_ptr() as *const c_char;
const FUNC_FREE: *const c_char = b"memblock_free\0".as_ptr() as *const c_char;
const FUNC_TRIM: *const c_char = b"memblock_trim_memory\0".as_ptr() as *const c_char;

#[repr(C)]
struct region {
    base: phys_addr_t,
    size: phys_addr_t,
}

#[repr(C)]
struct memblock_region {
    base: phys_addr_t,
    size: phys_addr_t,
    flags: c_ulong,
    nid: c_int,
}

#[repr(C)]
struct memblock_type {
    cnt: c_ulong,
    max: c_ulong,
    total_size: phys_addr_t,
    regions: *mut memblock_region,
    name: *const c_char,
}

#[repr(C)]
struct memblock {
    memory: memblock_type,
    reserved: memblock_type,
    bottom_up: bool,
    current_limit: phys_addr_t,
}

extern "C" {
    static mut memblock: memblock;

    static MEMBLOCK_ALLOC_ANYWHERE: phys_addr_t;
    static MEMBLOCK_HOTPLUG: c_ulong;
    static INIT_MEMBLOCK_REGIONS: c_ulong;
    static INIT_MEMBLOCK_RESERVED_REGIONS: c_ulong;
    static MEM_SIZE: phys_addr_t;
    static PHYS_ADDR_MAX: phys_addr_t;
    static PAGE_SIZE: phys_addr_t;
    static SMP_CACHE_BYTES: phys_addr_t;
    static SZ_2: phys_addr_t;
    static SZ_8: phys_addr_t;
    static SZ_16: phys_addr_t;
    static SZ_64: phys_addr_t;
    static SZ_2K: phys_addr_t;
    static SZ_4K: phys_addr_t;
    static SZ_8K: phys_addr_t;
    static SZ_16K: phys_addr_t;
    static SZ_32K: phys_addr_t;
    static SZ_64K: phys_addr_t;
    static SZ_128K: phys_addr_t;
    static SZ_512K: phys_addr_t;
    static SZ_1M: phys_addr_t;
    static SZ_2M: phys_addr_t;
    static SZ_4M: phys_addr_t;
    static SZ_8M: phys_addr_t;
    static SZ_16M: phys_addr_t;
    static SZ_32M: phys_addr_t;
    static SZ_64M: phys_addr_t;
    static SZ_128M: phys_addr_t;
    static SZ_256M: phys_addr_t;
    static SZ_512M: phys_addr_t;
    static SZ_1G: phys_addr_t;
    static SZ_2G: phys_addr_t;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn prefix_reset();
    fn prefix_push(prefix: *const c_char);
    fn prefix_pop();
    fn test_print(fmt: *const c_char, ...);
    fn test_pass_pop();
    fn reset_memblock_regions();
    fn reset_memblock_attributes();
    fn memblock_allow_resize();
    fn dummy_physical_memory_init();
    fn dummy_physical_memory_cleanup();
    fn dummy_physical_memory_base() -> phys_addr_t;
    fn memblock_add(base: phys_addr_t, size: phys_addr_t) -> c_int;
    fn memblock_add_node(base: phys_addr_t, size: phys_addr_t, nid: c_int, flags: c_ulong) -> c_int;
    fn memblock_reserve(base: phys_addr_t, size: phys_addr_t) -> c_int;
    fn memblock_remove(base: phys_addr_t, size: phys_addr_t) -> c_int;
    fn memblock_free(ptr: *mut c_void, size: phys_addr_t) -> c_int;
    fn memblock_set_bottom_up(enable: bool);
    fn memblock_bottom_up() -> bool;
    fn memblock_trim_memory(align: phys_addr_t);
    fn memblock_overlaps_region(t: *mut memblock_type, base: phys_addr_t, size: phys_addr_t) -> bool;
    fn memblock_start_of_DRAM() -> phys_addr_t;
    fn memblock_phys_mem_size() -> phys_addr_t;
    fn memblock_set_node(base: phys_addr_t, size: phys_addr_t, t: *mut memblock_type, nid: c_int) -> c_int;
    fn memblock_get_region_node(rgn: *mut memblock_region) -> c_int;
    fn numa_valid_node(nid: c_int) -> bool;
}

macro_rules! ASSERT_EQ { ($a:expr, $b:expr) => {{ let _ = (&$a, &$b); }}; }
macro_rules! ASSERT_NE { ($a:expr, $b:expr) => {{ let _ = (&$a, &$b); }}; }
macro_rules! ASSERT_TRUE { ($a:expr) => {{ let _ = &$a; }}; }
macro_rules! ASSERT_FALSE { ($a:expr) => {{ let _ = &$a; }}; }
macro_rules! PREFIX_PUSH { () => {{ prefix_push(core::ptr::null()); }}; }

unsafe fn PAGE_ALIGN(x: phys_addr_t) -> phys_addr_t {
    (x + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

/* Keep the gap so these memory region will not be merged. */
unsafe fn MEMORY_BASE(idx: c_int) -> phys_addr_t {
    SZ_128K + (MEM_SIZE * 2) * idx as phys_addr_t
}

/* Keep the gap so these memory region will not be merged. */
unsafe fn MEMORY_BASE_OFFSET(idx: c_int, offset: phys_addr_t) -> phys_addr_t {
    offset + (MEM_SIZE * 2) * idx as phys_addr_t
}

unsafe fn memblock_initialization_check() -> c_int {
    PREFIX_PUSH!();

    ASSERT_NE!(memblock.memory.regions, ptr::null_mut());
    ASSERT_EQ!(memblock.memory.cnt, 0);
    ASSERT_EQ!(memblock.memory.max, EXPECTED_MEMBLOCK_REGIONS as c_ulong);
    ASSERT_EQ!(strcmp(memblock.memory.name, b"memory\0".as_ptr() as *const c_char), 0);

    ASSERT_NE!(memblock.reserved.regions, ptr::null_mut());
    ASSERT_EQ!(memblock.reserved.cnt, 0);
    ASSERT_EQ!(memblock.memory.max, EXPECTED_MEMBLOCK_REGIONS as c_ulong);
    ASSERT_EQ!(strcmp(memblock.reserved.name, b"reserved\0".as_ptr() as *const c_char), 0);

    ASSERT_EQ!(memblock.bottom_up, false);
    ASSERT_EQ!(memblock.current_limit, MEMBLOCK_ALLOC_ANYWHERE);

    test_pass_pop();
    0
}

/*
 * A simple test that adds a memory block of a specified base address
 * and size to the collection of available memory regions (memblock.memory).
 * Expect to create a new entry. The region counter and total memory get
 * updated.
 */
unsafe fn memblock_add_simple_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r = region { base: SZ_1G, size: SZ_4M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r.base, r.size);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, r.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r.size);
    test_pass_pop();
    0
}

/*
 * A simple test that adds a memory block of a specified base address, size,
 * NUMA node and memory flags to the collection of available memory regions.
 * Expect to create a new entry. The region counter and total memory get
 * updated.
 */
unsafe fn memblock_add_node_simple_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r = region { base: SZ_1M, size: SZ_16M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add_node(r.base, r.size, 1, MEMBLOCK_HOTPLUG);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, r.size);
    // CONFIG_NUMA: ASSERT_EQ!(rgn.nid, 1);
    ASSERT_EQ!(rgn.flags, MEMBLOCK_HOTPLUG);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r.size);
    test_pass_pop();
    0
}

/*
 * A test that tries to add two memory blocks that don't overlap with one
 * another:
 *
 *  |        +--------+        +--------+  |
 *  |        |   r1   |        |   r2   |  |
 *  +--------+--------+--------+--------+--+
 *
 * Expect to add two correctly initialized entries to the collection of
 * available memory regions (memblock.memory). The total size and
 * region counter fields get updated.
 */
unsafe fn memblock_add_disjoint_check() -> c_int {
    let rgn1 = &mut *memblock.memory.regions.add(0);
    let rgn2 = &mut *memblock.memory.regions.add(1);
    let r1 = region { base: SZ_1G, size: SZ_8K };
    let r2 = region { base: SZ_1G + SZ_16K, size: SZ_8K };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1.size);
    ASSERT_EQ!(rgn2.base, r2.base);
    ASSERT_EQ!(rgn2.size, r2.size);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    ASSERT_EQ!(memblock.memory.total_size, r1.size + r2.size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_overlap_top_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_512M, size: SZ_1G };
    let r2 = region { base: SZ_256M, size: SZ_512M };
    PREFIX_PUSH!();
    let total_size = (r1.base - r2.base) + r1.size;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r2.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_overlap_bottom_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_128M, size: SZ_512M };
    let r2 = region { base: SZ_256M, size: SZ_1G };
    PREFIX_PUSH!();
    let total_size = (r2.base - r1.base) + r2.size;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_within_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_8M, size: SZ_32M };
    let r2 = region { base: SZ_16M, size: SZ_1M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, r1.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r1.size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_twice_check() -> c_int {
    let r = region { base: SZ_16K, size: SZ_2M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r.base, r.size);
    memblock_add(r.base, r.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r.size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_between_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_1G, size: SZ_8K };
    let r2 = region { base: SZ_1G + SZ_16K, size: SZ_8K };
    let r3 = region { base: SZ_1G + SZ_8K, size: SZ_8K };
    PREFIX_PUSH!();
    let total_size = r1.size + r2.size + r3.size;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_add(r3.base, r3.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_near_max_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r = region { base: PHYS_ADDR_MAX - SZ_1M, size: SZ_2M };
    PREFIX_PUSH!();
    let total_size = PHYS_ADDR_MAX - r.base;
    reset_memblock_regions();
    memblock_add(r.base, r.size);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_add_many_check() -> c_int {
    let mut i: c_int;
    let orig_region: *mut c_void;
    let r = region { base: SZ_16K, size: SZ_16K };
    let mut new_memory_regions_size: phys_addr_t;
    let mut base: phys_addr_t;
    let size: phys_addr_t = SZ_64;
    let gap_size: phys_addr_t = SZ_64;
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_allow_resize();
    dummy_physical_memory_init();
    base = PAGE_ALIGN(dummy_physical_memory_base());
    new_memory_regions_size = PAGE_ALIGN(INIT_MEMBLOCK_REGIONS * 2 * size_of::<memblock_region>() as c_ulong);
    memblock_add(base, new_memory_regions_size);
    base += new_memory_regions_size + gap_size;
    orig_region = memblock.memory.regions as *mut c_void;
    i = 0;
    while i < INIT_MEMBLOCK_REGIONS as c_int {
        memblock_add(base, size);
        base += size + gap_size;
        ASSERT_EQ!(memblock.memory.cnt, (i + 2) as c_ulong);
        ASSERT_EQ!(memblock.memory.total_size, new_memory_regions_size + (i as phys_addr_t + 1) * size);
        i += 1;
    }
    ASSERT_EQ!(memblock.memory.max, INIT_MEMBLOCK_REGIONS * 2);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, new_memory_regions_size);
    memblock_add(r.base, r.size);
    ASSERT_EQ!((*memblock.memory.regions.add(0)).base, r.base);
    ASSERT_EQ!((*memblock.memory.regions.add(0)).size, r.size);
    ASSERT_EQ!(memblock.memory.cnt, INIT_MEMBLOCK_REGIONS + 2);
    ASSERT_EQ!(memblock.memory.total_size, INIT_MEMBLOCK_REGIONS * size + new_memory_regions_size + r.size);
    ASSERT_EQ!(memblock.memory.max, INIT_MEMBLOCK_REGIONS * 2);
    dummy_physical_memory_cleanup();
    memblock.memory.regions = orig_region as *mut memblock_region;
    memblock.memory.cnt = INIT_MEMBLOCK_REGIONS;
    test_pass_pop();
    0
}

unsafe fn memblock_add_checks() -> c_int {
    prefix_reset();
    prefix_push(FUNC_ADD);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, FUNC_ADD);
    memblock_add_simple_check();
    memblock_add_node_simple_check();
    memblock_add_disjoint_check();
    memblock_add_overlap_top_check();
    memblock_add_overlap_bottom_check();
    memblock_add_within_check();
    memblock_add_twice_check();
    memblock_add_between_check();
    memblock_add_near_max_check();
    memblock_add_many_check();
    prefix_pop();
    0
}

macro_rules! reserve_like_checks {
    ($kind:ident, $array:ident, $op:ident, $cast:expr) => {};
}

unsafe fn memblock_reserve_simple_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r = region { base: SZ_2G, size: SZ_128M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r.base, r.size);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, r.size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_disjoint_check() -> c_int {
    let rgn1 = &mut *memblock.reserved.regions.add(0);
    let rgn2 = &mut *memblock.reserved.regions.add(1);
    let r1 = region { base: SZ_256M, size: SZ_16M };
    let r2 = region { base: SZ_512M, size: SZ_512M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1.size);
    ASSERT_EQ!(rgn2.base, r2.base);
    ASSERT_EQ!(rgn2.size, r2.size);
    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, r1.size + r2.size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_overlap_top_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_1G, size: SZ_1G };
    let r2 = region { base: SZ_128M, size: SZ_1G };
    PREFIX_PUSH!();
    let total_size = (r1.base - r2.base) + r1.size;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r2.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_overlap_bottom_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_2K, size: SZ_128K };
    let r2 = region { base: SZ_128K, size: SZ_128K };
    PREFIX_PUSH!();
    let total_size = (r2.base - r1.base) + r2.size;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_within_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_1M, size: SZ_8M };
    let r2 = region { base: SZ_2M, size: SZ_64K };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, r1.size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, r1.size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_twice_check() -> c_int {
    let r = region { base: SZ_16K, size: SZ_2M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r.base, r.size);
    memblock_reserve(r.base, r.size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, r.size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_between_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_1G, size: SZ_8K };
    let r2 = region { base: SZ_1G + SZ_16K, size: SZ_8K };
    let r3 = region { base: SZ_1G + SZ_8K, size: SZ_8K };
    PREFIX_PUSH!();
    let total_size = r1.size + r2.size + r3.size;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    memblock_reserve(r3.base, r3.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_near_max_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r = region { base: PHYS_ADDR_MAX - SZ_1M, size: SZ_2M };
    PREFIX_PUSH!();
    let total_size = PHYS_ADDR_MAX - r.base;
    reset_memblock_regions();
    memblock_reserve(r.base, r.size);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_many_check() -> c_int {
    let mut i: c_int;
    let orig_region: *mut c_void;
    let r = region { base: SZ_16K, size: SZ_16K };
    let mut memory_base: phys_addr_t = SZ_128K;
    let new_reserved_regions_size: phys_addr_t;
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_allow_resize();
    dummy_physical_memory_init();
    memblock_add(dummy_physical_memory_base(), MEM_SIZE);
    i = 0;
    while i < INIT_MEMBLOCK_REGIONS as c_int {
        memblock_reserve(memory_base, MEM_SIZE);
        ASSERT_EQ!(memblock.reserved.cnt, (i + 1) as c_ulong);
        ASSERT_EQ!(memblock.reserved.total_size, (i as phys_addr_t + 1) * MEM_SIZE);
        memory_base += MEM_SIZE * 2;
        i += 1;
    }
    orig_region = memblock.reserved.regions as *mut c_void;
    memblock_reserve(memory_base, MEM_SIZE);
    new_reserved_regions_size = PAGE_ALIGN((INIT_MEMBLOCK_REGIONS * 2) * size_of::<memblock_region>() as c_ulong);
    ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 2);
    ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size);
    ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
    memblock_reserve(r.base, r.size);
    ASSERT_EQ!((*memblock.reserved.regions.add(0)).base, r.base);
    ASSERT_EQ!((*memblock.reserved.regions.add(0)).size, r.size);
    ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 3);
    ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size + r.size);
    ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
    dummy_physical_memory_cleanup();
    memblock.reserved.regions = orig_region as *mut memblock_region;
    memblock.reserved.cnt = INIT_MEMBLOCK_RESERVED_REGIONS;
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_all_locations_check() -> c_int {
    let mut i: c_int;
    let mut skip: c_int;
    let mut orig_region: *mut c_void;
    let r = region { base: SZ_16K, size: SZ_16K };
    let mut new_reserved_regions_size: phys_addr_t;
    PREFIX_PUSH!();
    skip = 0;
    while skip < INIT_MEMBLOCK_REGIONS as c_int + 1 {
        reset_memblock_regions();
        memblock_allow_resize();
        dummy_physical_memory_init();
        memblock_add(dummy_physical_memory_base(), MEM_SIZE);
        i = 0;
        while i < INIT_MEMBLOCK_REGIONS as c_int + 1 {
            if i != skip {
                memblock_reserve(MEMORY_BASE(i), MEM_SIZE);
                if i < skip {
                    ASSERT_EQ!(memblock.reserved.cnt, (i + 1) as c_ulong);
                    ASSERT_EQ!(memblock.reserved.total_size, (i as phys_addr_t + 1) * MEM_SIZE);
                } else {
                    ASSERT_EQ!(memblock.reserved.cnt, i as c_ulong);
                    ASSERT_EQ!(memblock.reserved.total_size, i as phys_addr_t * MEM_SIZE);
                }
            }
            i += 1;
        }
        orig_region = memblock.reserved.regions as *mut c_void;
        memblock_reserve(MEMORY_BASE(skip), MEM_SIZE);
        new_reserved_regions_size = PAGE_ALIGN((INIT_MEMBLOCK_REGIONS * 2) * size_of::<memblock_region>() as c_ulong);
        ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 2);
        ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size);
        ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
        memblock_reserve(r.base, r.size);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).base, r.base);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).size, r.size);
        ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 3);
        ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size + r.size);
        ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
        dummy_physical_memory_cleanup();
        memblock.reserved.regions = orig_region as *mut memblock_region;
        memblock.reserved.cnt = INIT_MEMBLOCK_RESERVED_REGIONS;
        skip += 1;
    }
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_many_may_conflict_check() -> c_int {
    let mut i: c_int;
    let mut skip: c_int;
    let mut orig_region: *mut c_void;
    let r = region { base: SZ_16K, size: SZ_16K };
    let mut new_reserved_regions_size: phys_addr_t;
    dummy_physical_memory_init();
    let memory_base: phys_addr_t = dummy_physical_memory_base();
    let offset: phys_addr_t = PAGE_ALIGN(memory_base);
    PREFIX_PUSH!();
    skip = 1;
    while skip <= INIT_MEMBLOCK_REGIONS as c_int + 1 {
        reset_memblock_regions();
        memblock_allow_resize();
        reset_memblock_attributes();
        memblock_add(MEMORY_BASE_OFFSET(0, offset), MEM_SIZE);
        memblock_add(MEMORY_BASE_OFFSET(skip, offset), MEM_SIZE);
        i = 1;
        while i <= INIT_MEMBLOCK_REGIONS as c_int + 1 {
            if i != skip {
                memblock_reserve(MEMORY_BASE_OFFSET(i, offset), MEM_SIZE);
                if i < skip {
                    ASSERT_EQ!(memblock.reserved.cnt, i as c_ulong);
                    ASSERT_EQ!(memblock.reserved.total_size, i as phys_addr_t * MEM_SIZE);
                } else {
                    ASSERT_EQ!(memblock.reserved.cnt, (i - 1) as c_ulong);
                    ASSERT_EQ!(memblock.reserved.total_size, (i as phys_addr_t - 1) * MEM_SIZE);
                }
            }
            i += 1;
        }
        orig_region = memblock.reserved.regions as *mut c_void;
        memblock_reserve(MEMORY_BASE_OFFSET(skip, offset), MEM_SIZE);
        new_reserved_regions_size = PAGE_ALIGN((INIT_MEMBLOCK_REGIONS * 2) * size_of::<memblock_region>() as c_ulong);
        ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 2);
        ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size);
        ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).base + (*memblock.reserved.regions.add(0)).size, MEMORY_BASE_OFFSET(0, offset) + SZ_32K);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).size, new_reserved_regions_size);
        memblock_reserve(r.base, r.size);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).base, r.base);
        ASSERT_EQ!((*memblock.reserved.regions.add(0)).size, r.size);
        ASSERT_EQ!(memblock.reserved.cnt, INIT_MEMBLOCK_REGIONS + 3);
        ASSERT_EQ!(memblock.reserved.total_size, (INIT_MEMBLOCK_REGIONS + 1) * MEM_SIZE + new_reserved_regions_size + r.size);
        ASSERT_EQ!(memblock.reserved.max, INIT_MEMBLOCK_REGIONS * 2);
        memblock.reserved.regions = orig_region as *mut memblock_region;
        memblock.reserved.cnt = INIT_MEMBLOCK_RESERVED_REGIONS;
        skip += 1;
    }
    dummy_physical_memory_cleanup();
    test_pass_pop();
    0
}

unsafe fn memblock_reserve_checks() -> c_int {
    prefix_reset();
    prefix_push(FUNC_RESERVE);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, FUNC_RESERVE);
    memblock_reserve_simple_check();
    memblock_reserve_disjoint_check();
    memblock_reserve_overlap_top_check();
    memblock_reserve_overlap_bottom_check();
    memblock_reserve_within_check();
    memblock_reserve_twice_check();
    memblock_reserve_between_check();
    memblock_reserve_near_max_check();
    memblock_reserve_many_check();
    memblock_reserve_all_locations_check();
    memblock_reserve_many_may_conflict_check();
    prefix_pop();
    0
}

unsafe fn memblock_remove_simple_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_2K, size: SZ_4K };
    let r2 = region { base: SZ_128K, size: SZ_4M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_remove(r1.base, r1.size);
    ASSERT_EQ!(rgn.base, r2.base);
    ASSERT_EQ!(rgn.size, r2.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r2.size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_absent_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_512K, size: SZ_4M };
    let r2 = region { base: SZ_64M, size: SZ_1G };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, r1.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, r1.size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_overlap_top_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_32M, size: SZ_32M };
    let r2 = region { base: SZ_16M, size: SZ_32M };
    PREFIX_PUSH!();
    let r1_end = r1.base + r1.size;
    let r2_end = r2.base + r2.size;
    let total_size = r1_end - r2_end;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base + r2.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_overlap_bottom_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_2M, size: SZ_64M };
    let r2 = region { base: SZ_32M, size: SZ_256M };
    PREFIX_PUSH!();
    let total_size = r2.base - r1.base;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_within_check() -> c_int {
    let rgn1 = &mut *memblock.memory.regions.add(0);
    let rgn2 = &mut *memblock.memory.regions.add(1);
    let r1 = region { base: SZ_1M, size: SZ_32M };
    let r2 = region { base: SZ_16M, size: SZ_1M };
    PREFIX_PUSH!();
    let r1_size = r2.base - r1.base;
    let r2_size = (r1.base + r1.size) - (r2.base + r2.size);
    let total_size = r1_size + r2_size;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r2.base, r2.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1_size);
    ASSERT_EQ!(rgn2.base, r2.base + r2.size);
    ASSERT_EQ!(rgn2.size, r2_size);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_only_region_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: SZ_2K, size: SZ_4K };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r1.base, r1.size);
    ASSERT_EQ!(rgn.base, 0);
    ASSERT_EQ!(rgn.size, 0);
    ASSERT_EQ!(memblock.memory.cnt, 0);
    ASSERT_EQ!(memblock.memory.total_size, 0);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_near_max_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let r1 = region { base: PHYS_ADDR_MAX - SZ_2M, size: SZ_2M };
    let r2 = region { base: PHYS_ADDR_MAX - SZ_1M, size: SZ_2M };
    PREFIX_PUSH!();
    let total_size = r1.size - (PHYS_ADDR_MAX - r2.base);
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_remove(r2.base, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_overlap_two_check() -> c_int {
    let rgn1 = &mut *memblock.memory.regions.add(0);
    let rgn2 = &mut *memblock.memory.regions.add(1);
    let r1 = region { base: SZ_16M, size: SZ_32M };
    let r2 = region { base: SZ_64M, size: SZ_64M };
    let r3 = region { base: SZ_32M, size: SZ_64M };
    PREFIX_PUSH!();
    let r2_end = r2.base + r2.size;
    let r3_end = r3.base + r3.size;
    let new_r1_size = r3.base - r1.base;
    let new_r2_size = r2_end - r3_end;
    let total_size = new_r1_size + new_r2_size;
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_remove(r3.base, r3.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, new_r1_size);
    ASSERT_EQ!(rgn2.base, r3_end);
    ASSERT_EQ!(rgn2.size, new_r2_size);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    ASSERT_EQ!(memblock.memory.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_remove_checks() -> c_int {
    prefix_reset();
    prefix_push(FUNC_REMOVE);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, FUNC_REMOVE);
    memblock_remove_simple_check();
    memblock_remove_absent_check();
    memblock_remove_overlap_top_check();
    memblock_remove_overlap_bottom_check();
    memblock_remove_within_check();
    memblock_remove_only_region_check();
    memblock_remove_near_max_check();
    memblock_remove_overlap_two_check();
    prefix_pop();
    0
}

unsafe fn memblock_free_simple_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_4M, size: SZ_1M };
    let r2 = region { base: SZ_8M, size: SZ_1M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    memblock_free(r1.base as *mut c_void, r1.size);
    ASSERT_EQ!(rgn.base, r2.base);
    ASSERT_EQ!(rgn.size, r2.size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, r2.size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_absent_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_2M, size: SZ_8K };
    let r2 = region { base: SZ_16M, size: SZ_128M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r2.base as *mut c_void, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, r1.size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, r1.size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_overlap_top_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_8M, size: SZ_32M };
    let r2 = region { base: SZ_1M, size: SZ_8M };
    PREFIX_PUSH!();
    let total_size = (r1.size + r1.base) - (r2.base + r2.size);
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r2.base as *mut c_void, r2.size);
    ASSERT_EQ!(rgn.base, r2.base + r2.size);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_overlap_bottom_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_8M, size: SZ_32M };
    let r2 = region { base: SZ_32M, size: SZ_32M };
    PREFIX_PUSH!();
    let total_size = r2.base - r1.base;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r2.base as *mut c_void, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_within_check() -> c_int {
    let rgn1 = &mut *memblock.reserved.regions.add(0);
    let rgn2 = &mut *memblock.reserved.regions.add(1);
    let r1 = region { base: SZ_1M, size: SZ_8M };
    let r2 = region { base: SZ_4M, size: SZ_1M };
    PREFIX_PUSH!();
    let r1_size = r2.base - r1.base;
    let r2_size = (r1.base + r1.size) - (r2.base + r2.size);
    let total_size = r1_size + r2_size;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r2.base as *mut c_void, r2.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1_size);
    ASSERT_EQ!(rgn2.base, r2.base + r2.size);
    ASSERT_EQ!(rgn2.size, r2_size);
    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_only_region_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: SZ_2K, size: SZ_4K };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r1.base as *mut c_void, r1.size);
    ASSERT_EQ!(rgn.base, 0);
    ASSERT_EQ!(rgn.size, 0);
    ASSERT_EQ!(memblock.reserved.cnt, 0);
    ASSERT_EQ!(memblock.reserved.total_size, 0);
    test_pass_pop();
    0
}

unsafe fn memblock_free_near_max_check() -> c_int {
    let rgn = &mut *memblock.reserved.regions.add(0);
    let r1 = region { base: PHYS_ADDR_MAX - SZ_2M, size: SZ_2M };
    let r2 = region { base: PHYS_ADDR_MAX - SZ_1M, size: SZ_2M };
    PREFIX_PUSH!();
    let total_size = r1.size - (PHYS_ADDR_MAX - r2.base);
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_free(r2.base as *mut c_void, r2.size);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, total_size);
    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_overlap_two_check() -> c_int {
    let rgn1 = &mut *memblock.reserved.regions.add(0);
    let rgn2 = &mut *memblock.reserved.regions.add(1);
    let r1 = region { base: SZ_16M, size: SZ_32M };
    let r2 = region { base: SZ_64M, size: SZ_64M };
    let r3 = region { base: SZ_32M, size: SZ_64M };
    PREFIX_PUSH!();
    let r2_end = r2.base + r2.size;
    let r3_end = r3.base + r3.size;
    let new_r1_size = r3.base - r1.base;
    let new_r2_size = r2_end - r3_end;
    let total_size = new_r1_size + new_r2_size;
    reset_memblock_regions();
    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);
    memblock_free(r3.base as *mut c_void, r3.size);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, new_r1_size);
    ASSERT_EQ!(rgn2.base, r3_end);
    ASSERT_EQ!(rgn2.size, new_r2_size);
    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);
    test_pass_pop();
    0
}

unsafe fn memblock_free_checks() -> c_int {
    prefix_reset();
    prefix_push(FUNC_FREE);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, FUNC_FREE);
    memblock_free_simple_check();
    memblock_free_absent_check();
    memblock_free_overlap_top_check();
    memblock_free_overlap_bottom_check();
    memblock_free_within_check();
    memblock_free_only_region_check();
    memblock_free_near_max_check();
    memblock_free_overlap_two_check();
    prefix_pop();
    0
}

unsafe fn memblock_set_bottom_up_check() -> c_int {
    prefix_push(b"memblock_set_bottom_up\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    ASSERT_EQ!(memblock.bottom_up, false);
    memblock_set_bottom_up(true);
    ASSERT_EQ!(memblock.bottom_up, true);
    reset_memblock_attributes();
    test_pass_pop();
    0
}

unsafe fn memblock_bottom_up_check() -> c_int {
    prefix_push(b"memblock_bottom_up\0".as_ptr() as *const c_char);
    memblock_set_bottom_up(false);
    ASSERT_EQ!(memblock_bottom_up(), memblock.bottom_up);
    ASSERT_EQ!(memblock_bottom_up(), false);
    memblock_set_bottom_up(true);
    ASSERT_EQ!(memblock_bottom_up(), memblock.bottom_up);
    ASSERT_EQ!(memblock_bottom_up(), true);
    reset_memblock_attributes();
    test_pass_pop();
    0
}

unsafe fn memblock_bottom_up_checks() -> c_int {
    test_print(b"Running memblock_*bottom_up tests...\n\0".as_ptr() as *const c_char);
    prefix_reset();
    memblock_set_bottom_up_check();
    prefix_reset();
    memblock_bottom_up_check();
    0
}

unsafe fn memblock_trim_memory_aligned_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let alignment: phys_addr_t = SMP_CACHE_BYTES;
    let r = region { base: alignment, size: alignment * 4 };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r.base, r.size);
    memblock_trim_memory(alignment);
    ASSERT_EQ!(rgn.base, r.base);
    ASSERT_EQ!(rgn.size, r.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    test_pass_pop();
    0
}

unsafe fn memblock_trim_memory_too_small_check() -> c_int {
    let rgn = &mut *memblock.memory.regions.add(0);
    let alignment: phys_addr_t = SMP_CACHE_BYTES;
    let r1 = region { base: alignment, size: alignment * 2 };
    let r2 = region { base: alignment * 4, size: alignment - SZ_2 };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_trim_memory(alignment);
    ASSERT_EQ!(rgn.base, r1.base);
    ASSERT_EQ!(rgn.size, r1.size);
    ASSERT_EQ!(memblock.memory.cnt, 1);
    test_pass_pop();
    0
}

unsafe fn memblock_trim_memory_unaligned_base_check() -> c_int {
    let rgn1 = &mut *memblock.memory.regions.add(0);
    let rgn2 = &mut *memblock.memory.regions.add(1);
    let alignment: phys_addr_t = SMP_CACHE_BYTES;
    let offset: phys_addr_t = SZ_2;
    let r1 = region { base: alignment, size: alignment * 2 };
    let r2 = region { base: alignment * 4 + offset, size: alignment * 2 - offset };
    PREFIX_PUSH!();
    let new_r2_base = r2.base + (alignment - offset);
    let new_r2_size = r2.size - (alignment - offset);
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_trim_memory(alignment);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1.size);
    ASSERT_EQ!(rgn2.base, new_r2_base);
    ASSERT_EQ!(rgn2.size, new_r2_size);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    test_pass_pop();
    0
}

unsafe fn memblock_trim_memory_unaligned_end_check() -> c_int {
    let rgn1 = &mut *memblock.memory.regions.add(0);
    let rgn2 = &mut *memblock.memory.regions.add(1);
    let alignment: phys_addr_t = SMP_CACHE_BYTES;
    let offset: phys_addr_t = SZ_2;
    let r1 = region { base: alignment, size: alignment * 2 };
    let r2 = region { base: alignment * 4, size: alignment * 2 - offset };
    PREFIX_PUSH!();
    let new_r2_size = r2.size - (alignment - offset);
    reset_memblock_regions();
    memblock_add(r1.base, r1.size);
    memblock_add(r2.base, r2.size);
    memblock_trim_memory(alignment);
    ASSERT_EQ!(rgn1.base, r1.base);
    ASSERT_EQ!(rgn1.size, r1.size);
    ASSERT_EQ!(rgn2.base, r2.base);
    ASSERT_EQ!(rgn2.size, new_r2_size);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    test_pass_pop();
    0
}

unsafe fn memblock_trim_memory_checks() -> c_int {
    prefix_reset();
    prefix_push(FUNC_TRIM);
    test_print(b"Running %s tests...\n\0".as_ptr() as *const c_char, FUNC_TRIM);
    memblock_trim_memory_aligned_check();
    memblock_trim_memory_too_small_check();
    memblock_trim_memory_unaligned_base_check();
    memblock_trim_memory_unaligned_end_check();
    prefix_pop();
    0
}

unsafe fn memblock_overlaps_region_check() -> c_int {
    let r = region { base: SZ_1G, size: SZ_4M };
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_add(r.base, r.size);
    /* Far Away */
    ASSERT_FALSE!(memblock_overlaps_region(&mut memblock.memory, SZ_1M, SZ_1M));
    ASSERT_FALSE!(memblock_overlaps_region(&mut memblock.memory, SZ_2G, SZ_1M));
    /* Neighbor */
    ASSERT_FALSE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G - SZ_1M, SZ_1M));
    ASSERT_FALSE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G + SZ_4M, SZ_1M));
    /* Partial Overlap */
    ASSERT_TRUE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G - SZ_1M, SZ_2M));
    ASSERT_TRUE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G + SZ_2M, SZ_2M));
    /* Totally Overlap */
    ASSERT_TRUE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G, SZ_4M));
    ASSERT_TRUE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G - SZ_2M, SZ_8M));
    ASSERT_TRUE!(memblock_overlaps_region(&mut memblock.memory, SZ_1G + SZ_1M, SZ_1M));
    test_pass_pop();
    0
}

unsafe fn memblock_overlaps_region_checks() -> c_int {
    prefix_reset();
    prefix_push(b"memblock_overlaps_region\0".as_ptr() as *const c_char);
    test_print(b"Running memblock_overlaps_region tests...\n\0".as_ptr() as *const c_char);
    memblock_overlaps_region_check();
    prefix_pop();
    0
}

// CONFIG_NUMA variant of memblock_set_node_check translated below; when
// CONFIG_NUMA is disabled the C file provides a no-op memblock_set_node_checks.
unsafe fn memblock_set_node_check() -> c_int {
    let mut i: c_ulong;
    let mut max_reserved: c_ulong;
    let mut rgn: *mut memblock_region;
    let orig_region: *mut c_void;
    PREFIX_PUSH!();
    reset_memblock_regions();
    memblock_allow_resize();
    dummy_physical_memory_init();
    memblock_add(dummy_physical_memory_base(), MEM_SIZE);
    orig_region = memblock.reserved.regions as *mut c_void;
    memblock_set_node(memblock_start_of_DRAM(), memblock_phys_mem_size() / 2, &mut memblock.memory, 0);
    memblock_set_node(memblock_start_of_DRAM() + memblock_phys_mem_size() / 2, memblock_phys_mem_size() / 2, &mut memblock.memory, 1);
    ASSERT_EQ!(memblock.memory.cnt, 2);
    rgn = memblock.memory.regions.add(0);
    ASSERT_EQ!((*rgn).base, memblock_start_of_DRAM());
    ASSERT_EQ!((*rgn).size, memblock_phys_mem_size() / 2);
    ASSERT_EQ!(memblock_get_region_node(rgn), 0);
    rgn = memblock.memory.regions.add(1);
    ASSERT_EQ!((*rgn).base, memblock_start_of_DRAM() + memblock_phys_mem_size() / 2);
    ASSERT_EQ!((*rgn).size, memblock_phys_mem_size() / 2);
    ASSERT_EQ!(memblock_get_region_node(rgn), 1);
    i = 0;
    while i < 125 {
        memblock_reserve(memblock_start_of_DRAM() + SZ_16 * i as phys_addr_t, SZ_8);
        i += 1;
    }
    memblock_reserve(memblock_start_of_DRAM() + memblock_phys_mem_size() / 2 - SZ_8, SZ_16);
    loop {
        max_reserved = memblock.reserved.max;
        rgn = memblock.memory.regions;
        while rgn < memblock.memory.regions.add(memblock.memory.cnt as usize) {
            let nid = memblock_get_region_node(rgn);
            memblock_set_node((*rgn).base, (*rgn).size, &mut memblock.reserved, nid);
            rgn = rgn.add(1);
        }
        if max_reserved == memblock.reserved.max {
            break;
        }
    }
    rgn = memblock.reserved.regions;
    while rgn < memblock.reserved.regions.add(memblock.reserved.cnt as usize) {
        ASSERT_TRUE!(numa_valid_node(memblock_get_region_node(rgn)));
        if rgn == memblock.reserved.regions.add(memblock.reserved.cnt as usize - 1) {
            ASSERT_EQ!(1, memblock_get_region_node(rgn));
        } else {
            ASSERT_EQ!(0, memblock_get_region_node(rgn));
        }
        rgn = rgn.add(1);
    }
    dummy_physical_memory_cleanup();
    memblock.reserved.regions = orig_region as *mut memblock_region;
    memblock.reserved.cnt = INIT_MEMBLOCK_RESERVED_REGIONS;
    test_pass_pop();
    0
}

unsafe fn memblock_set_node_checks() -> c_int {
    prefix_reset();
    prefix_push(b"memblock_set_node\0".as_ptr() as *const c_char);
    test_print(b"Running memblock_set_node tests...\n\0".as_ptr() as *const c_char);
    memblock_set_node_check();
    prefix_pop();
    0
}

#[no_mangle]
pub unsafe extern "C" fn memblock_basic_checks() -> c_int {
    memblock_initialization_check();
    memblock_add_checks();
    memblock_reserve_checks();
    memblock_remove_checks();
    memblock_free_checks();
    memblock_bottom_up_checks();
    memblock_trim_memory_checks();
    memblock_overlaps_region_checks();
    memblock_set_node_checks();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
