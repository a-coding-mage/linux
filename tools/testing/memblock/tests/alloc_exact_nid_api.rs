// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from alloc_exact_nid_api.c.
// Depends on Rust equivalents of alloc_exact_nid_api.h and alloc_nid_api.h.

use core::ptr::null_mut;

const FUNC_NAME: &str = "memblock_alloc_exact_nid_raw";

/*
 * contains the fraction of MEM_SIZE contained in each node in basis point
 * units (one hundredth of 1% or 1/10000)
 */
static NODE_FRACTIONS: [u32; 8] = [
    2500, /* 1/4  */
     625, /* 1/16 */
    1250, /* 1/8  */
    1250, /* 1/8  */
     625, /* 1/16 */
     625, /* 1/16 */
    2500, /* 1/4  */
     625, /* 1/16 */
];

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * has enough memory to allocate a region of the requested size.
 * Expect to allocate an aligned region at the end of the requested node.
 */
unsafe fn alloc_exact_nid_top_down_numa_simple_check() -> i32 {
    let nid_req: i32 = 3;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    ASSERT_LE!(SZ_4, (*req_node).size);
    size = (*req_node).size / SZ_4;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, region_end(req_node) - size);
    ASSERT_LE!((*req_node).base, (*new_rgn).base);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * is partially reserved but has enough memory for the allocated region:
 *
 *  |           +---------------------------------------+          |
 *  |           |               requested               |          |
 *  +-----------+---------------------------------------+----------+
 *
 *  |           +------------------+              +-----+          |
 *  |           |     reserved     |              | new |          |
 *  +-----------+------------------+--------------+-----+----------+
 *
 * Expect to allocate an aligned region at the end of the requested node. The
 * region count and total size get updated.
 */
unsafe fn alloc_exact_nid_top_down_numa_part_reserved_check() -> i32 {
    let nid_req: i32 = 4;
    let new_rgn = &mut memblock.reserved.regions[1] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let mut r1: region = core::mem::zeroed();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    ASSERT_LE!(SZ_8, (*req_node).size);
    r1.base = (*req_node).base;
    r1.size = (*req_node).size / SZ_2;
    size = r1.size / SZ_4;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    memblock_reserve(r1.base, r1.size);
    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, region_end(req_node) - size);
    ASSERT_LE!((*req_node).base, (*new_rgn).base);

    ASSERT_EQ!(memblock.reserved.cnt, 2);
    ASSERT_EQ!(memblock.reserved.total_size, size + r1.size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region that spans over the min_addr
 * and max_addr range and overlaps with two different nodes, where the first
 * node is the requested node:
 *
 *                                min_addr
 *                                |           max_addr
 *                                |           |
 *                                v           v
 *  |           +-----------------------+-----------+              |
 *  |           |       requested       |   node3   |              |
 *  +-----------+-----------------------+-----------+--------------+
 *                                +           +
 *  |                       +-----------+                          |
 *  |                       |    rgn    |                          |
 *  +-----------------------+-----------+--------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region that ends at
 * the end of the requested node.
 */
unsafe fn alloc_exact_nid_top_down_numa_split_range_low_check() -> i32 {
    let nid_req: i32 = 2;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_512;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;
    let req_node_end: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    req_node_end = region_end(req_node);
    min_addr = req_node_end - SZ_256;
    max_addr = min_addr + size;

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, req_node_end - size);
    ASSERT_LE!((*req_node).base, (*new_rgn).base);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region that spans over the min_addr
 * and max_addr range and overlaps with two different nodes, where the requested
 * node ends before min_addr:
 *
 *                                         min_addr
 *                                         |         max_addr
 *                                         |         |
 *                                         v         v
 *  |    +---------------+        +-------------+---------+          |
 *  |    |   requested   |        |    node1    |  node2  |          |
 *  +----+---------------+--------+-------------+---------+----------+
 *                                         +         +
 *  |          +---------+                                           |
 *  |          |   rgn   |                                           |
 *  +----------+---------+-------------------------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region that ends at
 * the end of the requested node.
 */
unsafe fn alloc_exact_nid_top_down_numa_no_overlap_split_check() -> i32 {
    let nid_req: i32 = 2;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let node2 = &mut memblock.memory.regions[6] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    size = SZ_512;
    min_addr = (*node2).base - SZ_256;
    max_addr = min_addr + size;

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, region_end(req_node) - size);
    ASSERT_LE!((*req_node).base, (*new_rgn).base);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory within min_addr and max_add range when
 * the requested node and the range do not overlap, and requested node ends
 * before min_addr. The range overlaps with multiple nodes along node
 * boundaries:
 *
 *                          min_addr
 *                          |                                 max_addr
 *                          |                                 |
 *                          v                                 v
 *  |-----------+           +----------+----...----+----------+      |
 *  | requested |           | min node |    ...    | max node |      |
 *  +-----------+-----------+----------+----...----+----------+------+
 *                          +                                 +
 *  |     +-----+                                                    |
 *  |     | rgn |                                                    |
 *  +-----+-----+----------------------------------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region that ends at
 * the end of the requested node.
 */
unsafe fn alloc_exact_nid_top_down_numa_no_overlap_low_check() -> i32 {
    let nid_req: i32 = 0;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let min_node = &mut memblock.memory.regions[2] as *mut memblock_region;
    let max_node = &mut memblock.memory.regions[5] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_64;
    let max_addr: phys_addr_t;
    let min_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    min_addr = (*min_node).base;
    max_addr = region_end(max_node);

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, region_end(req_node) - size);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * has enough memory to allocate a region of the requested size.
 * Expect to allocate an aligned region at the beginning of the requested node.
 */
unsafe fn alloc_exact_nid_bottom_up_numa_simple_check() -> i32 {
    let nid_req: i32 = 3;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    ASSERT_LE!(SZ_4, (*req_node).size);
    size = (*req_node).size / SZ_4;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(new_rgn), region_end(req_node));

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * is partially reserved but has enough memory for the allocated region:
 *
 *  |           +---------------------------------------+         |
 *  |           |               requested               |         |
 *  +-----------+---------------------------------------+---------+
 *
 *  |           +------------------+-----+                        |
 *  |           |     reserved     | new |                        |
 *  +-----------+------------------+-----+------------------------+
 *
 * Expect to allocate an aligned region in the requested node that merges with
 * the existing reserved region. The total size gets updated.
 */
unsafe fn alloc_exact_nid_bottom_up_numa_part_reserved_check() -> i32 {
    let nid_req: i32 = 4;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let mut r1: region = core::mem::zeroed();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;
    let total_size: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    ASSERT_LE!(SZ_8, (*req_node).size);
    r1.base = (*req_node).base;
    r1.size = (*req_node).size / SZ_2;
    size = r1.size / SZ_4;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();
    total_size = size + r1.size;

    __memblock_reserve(r1.base, r1.size, nid_req, MEMBLOCK_RSRV_KERN);
    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, total_size);
    ASSERT_EQ!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(new_rgn), region_end(req_node));

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region that spans over the min_addr
 * and max_addr range and overlaps with two different nodes, where the first
 * node is the requested node:
 *
 *                                min_addr
 *                                |           max_addr
 *                                |           |
 *                                v           v
 *  |           +-----------------------+-----------+              |
 *  |           |       requested       |   node3   |              |
 *  +-----------+-----------------------+-----------+--------------+
 *                                +           +
 *  |           +-----------+                                      |
 *  |           |    rgn    |                                      |
 *  +-----------+-----------+--------------------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region at the beginning
 * of the requested node.
 */
unsafe fn alloc_exact_nid_bottom_up_numa_split_range_low_check() -> i32 {
    let nid_req: i32 = 2;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_512;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;
    let req_node_end: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    req_node_end = region_end(req_node);
    min_addr = req_node_end - SZ_256;
    max_addr = min_addr + size;

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(new_rgn), req_node_end);

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region that spans over the min_addr
 * and max_addr range and overlaps with two different nodes, where the requested
 * node ends before min_addr:
 *
 *                                          min_addr
 *                                         |         max_addr
 *                                         |         |
 *                                         v         v
 *  |    +---------------+        +-------------+---------+         |
 *  |    |   requested   |        |    node1    |  node2  |         |
 *  +----+---------------+--------+-------------+---------+---------+
 *                                         +         +
 *  |    +---------+                                                |
 *  |    |   rgn   |                                                |
 *  +----+---------+------------------------------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region that starts at
 * the beginning of the requested node.
 */
unsafe fn alloc_exact_nid_bottom_up_numa_no_overlap_split_check() -> i32 {
    let nid_req: i32 = 2;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let node2 = &mut memblock.memory.regions[6] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    size = SZ_512;
    min_addr = (*node2).base - SZ_256;
    max_addr = min_addr + size;

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(new_rgn), region_end(req_node));

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory within min_addr and max_add range when
 * the requested node and the range do not overlap, and requested node ends
 * before min_addr. The range overlaps with multiple nodes along node
 * boundaries:
 *
 *                          min_addr
 *                          |                                 max_addr
 *                          |                                 |
 *                          v                                 v
 *  |-----------+           +----------+----...----+----------+      |
 *  | requested |           | min node |    ...    | max node |      |
 *  +-----------+-----------+----------+----...----+----------+------+
 *                          +                                 +
 *  |-----+                                                          |
 *  | rgn |                                                          |
 *  +-----+----------------------------------------------------------+
 *
 * Expect to drop the lower limit and allocate a memory region that starts at
 * the beginning of the requested node.
 */
unsafe fn alloc_exact_nid_bottom_up_numa_no_overlap_low_check() -> i32 {
    let nid_req: i32 = 0;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let min_node = &mut memblock.memory.regions[2] as *mut memblock_region;
    let max_node = &mut memblock.memory.regions[5] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_64;
    let max_addr: phys_addr_t;
    let min_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    min_addr = (*min_node).base;
    max_addr = region_end(max_node);

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, size);
    ASSERT_EQ!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(new_rgn), region_end(req_node));

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * does not have enough memory to allocate a region of the requested size:
 *
 *  |   +-----+                            |
 *  |   | req |                            |
 *  +---+-----+----------------------------+
 *
 *  +---------+
 *  |   rgn   |
 *  +---------+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_small_node_generic_check() -> i32 {
    let nid_req: i32 = 1;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    size = SZ_2 * (*req_node).size;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * is fully reserved:
 *
 *  |              +---------+             |
 *  |              |requested|             |
 *  +--------------+---------+-------------+
 *
 *  |              +---------+             |
 *  |              | reserved|             |
 *  +--------------+---------+-------------+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_node_reserved_generic_check() -> i32 {
    let nid_req: i32 = 2;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    size = (*req_node).size;
    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    memblock_reserve((*req_node).base, (*req_node).size);
    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * is partially reserved and does not have enough contiguous memory for the
 * allocated region:
 *
 *  |           +-----------------------+    |
 *  |           |       requested       |    |
 *  +-----------+-----------------------+----+
 *
 *  |                 +----------+           |
 *  |                 | reserved |           |
 *  +-----------------+----------+-----------+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_part_reserved_fail_generic_check() -> i32 {
    let nid_req: i32 = 4;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let mut r1: region = core::mem::zeroed();
    let size: phys_addr_t;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    ASSERT_LE!(SZ_4, (*req_node).size);
    size = (*req_node).size / SZ_2;
    r1.base = (*req_node).base + (size / SZ_2);
    r1.size = size;

    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    memblock_reserve(r1.base, r1.size);
    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region that spans over the min_addr
 * and max_addr range and overlaps with two different nodes, where the second
 * node is the requested node:
 *
 *                               min_addr
 *                               |         max_addr
 *                               |         |
 *                               v         v
 *  |      +--------------------------+---------+                |
 *  |      |        first node        |requested|                |
 *  +------+--------------------------+---------+----------------+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_split_range_high_generic_check() -> i32 {
    let nid_req: i32 = 3;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_512;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    min_addr = (*req_node).base - SZ_256;
    max_addr = min_addr + size;

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory within min_addr and max_add range when
 * the requested node and the range do not overlap, and requested node starts
 * after max_addr. The range overlaps with multiple nodes along node
 * boundaries:
 *
 *        min_addr
 *        |                                 max_addr
 *        |                                 |
 *        v                                 v
 *  |     +----------+----...----+----------+        +-----------+   |
 *  |     | min node |    ...    | max node |        | requested |   |
 *  +-----+----------+----...----+----------+--------+-----------+---+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_no_overlap_high_generic_check() -> i32 {
    let nid_req: i32 = 7;
    let min_node = &mut memblock.memory.regions[2] as *mut memblock_region;
    let max_node = &mut memblock.memory.regions[5] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = SZ_64;
    let max_addr: phys_addr_t;
    let min_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    min_addr = (*min_node).base;
    max_addr = region_end(max_node);

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate a memory region in a specific NUMA node that
 * does not have enough memory to allocate a region of the requested size.
 * Additionally, none of the nodes have enough memory to allocate the region:
 *
 * +-----------------------------------+
 * |                new                |
 * +-----------------------------------+
 *     |-------+-------+-------+-------+-------+-------+-------+-------|
 *     | node0 | node1 | node2 | node3 | node4 | node5 | node6 | node7 |
 *     +-------+-------+-------+-------+-------+-------+-------+-------+
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_large_region_generic_check() -> i32 {
    let nid_req: i32 = 3;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let size: phys_addr_t = MEM_SIZE / SZ_2;
    let min_addr: phys_addr_t;
    let max_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    min_addr = memblock_start_of_DRAM();
    max_addr = memblock_end_of_DRAM();

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);
    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory within min_addr and max_addr range when
 * there are two reserved regions at the borders. The requested node starts at
 * min_addr and ends at max_addr and is the same size as the region to be
 * allocated:
 *
 *                     min_addr
 *                     |                       max_addr
 *                     |                       |
 *                     v                       v
 *  |      +-----------+-----------------------+-----------------------|
 *  |      |   node5   |       requested       |         node7         |
 *  +------+-----------+-----------------------+-----------------------+
 *                     +                       +
 *  |             +----+-----------------------+----+                  |
 *  |             | r2 |          new          | r1 |                  |
 *  +-------------+----+-----------------------+----+------------------+
 *
 * Expect to merge all of the regions into one. The region counter and total
 * size fields get updated.
 */
unsafe fn alloc_exact_nid_numa_reserved_full_merge_generic_check() -> i32 {
    let nid_req: i32 = 6;
    let nid_next: i32 = nid_req + 1;
    let new_rgn = &mut memblock.reserved.regions[0] as *mut memblock_region;
    let req_node = &mut memblock.memory.regions[nid_req as usize] as *mut memblock_region;
    let next_node = &mut memblock.memory.regions[nid_next as usize] as *mut memblock_region;
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let mut r1: region = core::mem::zeroed();
    let mut r2: region = core::mem::zeroed();
    let size: phys_addr_t = (*req_node).size;
    let total_size: phys_addr_t;
    let max_addr: phys_addr_t;
    let min_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    r1.base = (*next_node).base;
    r1.size = SZ_128;

    r2.size = SZ_128;
    r2.base = r1.base - (size + r2.size);

    total_size = r1.size + r2.size + size;
    min_addr = r2.base + r2.size;
    max_addr = r1.base;

    __memblock_reserve(r1.base, r1.size, nid_req, MEMBLOCK_RSRV_KERN);
    __memblock_reserve(r2.base, r2.size, nid_req, MEMBLOCK_RSRV_KERN);

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, nid_req);

    ASSERT_NE!(allocated_ptr, null_mut());
    ASSERT_MEM_NE!(allocated_ptr, 0, size);

    ASSERT_EQ!((*new_rgn).size, total_size);
    ASSERT_EQ!((*new_rgn).base, r2.base);

    ASSERT_LE!((*new_rgn).base, (*req_node).base);
    ASSERT_LE!(region_end(req_node), region_end(new_rgn));

    ASSERT_EQ!(memblock.reserved.cnt, 1);
    ASSERT_EQ!(memblock.reserved.total_size, total_size);

    test_pass_pop();

    0
}

/*
 * A test that tries to allocate memory within min_addr and max_add range,
 * where the total range can fit the region, but it is split between two nodes
 * and everything else is reserved. Additionally, nid is set to NUMA_NO_NODE
 * instead of requesting a specific node:
 *
 *                         +-----------+
 *                         |    new    |
 *                         +-----------+
 *  |      +---------------------+-----------|
 *  |      |      prev node      | next node |
 *  +------+---------------------+-----------+
 *                         +           +
 *  |----------------------+           +-----|
 *  |          r1          |           |  r2 |
 *  +----------------------+-----------+-----+
 *                         ^           ^
 *                         |           |
 *                         |           max_addr
 *                         |
 *                         min_addr
 *
 * Expect no allocation to happen.
 */
unsafe fn alloc_exact_nid_numa_split_all_reserved_generic_check() -> i32 {
    let mut allocated_ptr: *mut core::ffi::c_void = null_mut();
    let next_node = &mut memblock.memory.regions[7] as *mut memblock_region;
    let mut r1: region = core::mem::zeroed();
    let mut r2: region = core::mem::zeroed();
    let size: phys_addr_t = SZ_256;
    let max_addr: phys_addr_t;
    let min_addr: phys_addr_t;

    PREFIX_PUSH!();
    setup_numa_memblock(NODE_FRACTIONS.as_ptr());

    r2.base = (*next_node).base + SZ_128;
    r2.size = memblock_end_of_DRAM() - r2.base;

    r1.size = MEM_SIZE - (r2.size + size);
    r1.base = memblock_start_of_DRAM();

    min_addr = r1.base + r1.size;
    max_addr = r2.base;

    memblock_reserve(r1.base, r1.size);
    memblock_reserve(r2.base, r2.size);

    allocated_ptr = memblock_alloc_exact_nid_raw(size, SMP_CACHE_BYTES, min_addr, max_addr, NUMA_NO_NODE);

    ASSERT_EQ!(allocated_ptr, null_mut());

    test_pass_pop();

    0
}

/* Test case wrappers for NUMA tests */
unsafe fn alloc_exact_nid_numa_simple_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_simple_check".as_ptr());
    memblock_set_bottom_up(false);
    alloc_exact_nid_top_down_numa_simple_check();
    memblock_set_bottom_up(true);
    alloc_exact_nid_bottom_up_numa_simple_check();

    0
}

unsafe fn alloc_exact_nid_numa_part_reserved_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_part_reserved_check".as_ptr());
    memblock_set_bottom_up(false);
    alloc_exact_nid_top_down_numa_part_reserved_check();
    memblock_set_bottom_up(true);
    alloc_exact_nid_bottom_up_numa_part_reserved_check();

    0
}

unsafe fn alloc_exact_nid_numa_split_range_low_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_split_range_low_check".as_ptr());
    memblock_set_bottom_up(false);
    alloc_exact_nid_top_down_numa_split_range_low_check();
    memblock_set_bottom_up(true);
    alloc_exact_nid_bottom_up_numa_split_range_low_check();

    0
}

unsafe fn alloc_exact_nid_numa_no_overlap_split_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_no_overlap_split_check".as_ptr());
    memblock_set_bottom_up(false);
    alloc_exact_nid_top_down_numa_no_overlap_split_check();
    memblock_set_bottom_up(true);
    alloc_exact_nid_bottom_up_numa_no_overlap_split_check();

    0
}

unsafe fn alloc_exact_nid_numa_no_overlap_low_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_no_overlap_low_check".as_ptr());
    memblock_set_bottom_up(false);
    alloc_exact_nid_top_down_numa_no_overlap_low_check();
    memblock_set_bottom_up(true);
    alloc_exact_nid_bottom_up_numa_no_overlap_low_check();

    0
}

unsafe fn alloc_exact_nid_numa_small_node_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_small_node_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_small_node_generic_check);
    run_bottom_up(alloc_exact_nid_numa_small_node_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_node_reserved_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_node_reserved_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_node_reserved_generic_check);
    run_bottom_up(alloc_exact_nid_numa_node_reserved_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_part_reserved_fail_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_part_reserved_fail_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_part_reserved_fail_generic_check);
    run_bottom_up(alloc_exact_nid_numa_part_reserved_fail_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_split_range_high_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_split_range_high_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_split_range_high_generic_check);
    run_bottom_up(alloc_exact_nid_numa_split_range_high_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_no_overlap_high_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_no_overlap_high_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_no_overlap_high_generic_check);
    run_bottom_up(alloc_exact_nid_numa_no_overlap_high_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_large_region_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_large_region_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_large_region_generic_check);
    run_bottom_up(alloc_exact_nid_numa_large_region_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_reserved_full_merge_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_reserved_full_merge_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_reserved_full_merge_generic_check);
    run_bottom_up(alloc_exact_nid_numa_reserved_full_merge_generic_check);

    0
}

unsafe fn alloc_exact_nid_numa_split_all_reserved_check() -> i32 {
    test_print(c"\tRunning %s...\n".as_ptr(), c"alloc_exact_nid_numa_split_all_reserved_check".as_ptr());
    run_top_down(alloc_exact_nid_numa_split_all_reserved_generic_check);
    run_bottom_up(alloc_exact_nid_numa_split_all_reserved_generic_check);

    0
}

pub unsafe fn __memblock_alloc_exact_nid_numa_checks() -> i32 {
    test_print(c"Running %s NUMA tests...\n".as_ptr(), c"memblock_alloc_exact_nid_raw".as_ptr());

    alloc_exact_nid_numa_simple_check();
    alloc_exact_nid_numa_part_reserved_check();
    alloc_exact_nid_numa_split_range_low_check();
    alloc_exact_nid_numa_no_overlap_split_check();
    alloc_exact_nid_numa_no_overlap_low_check();

    alloc_exact_nid_numa_small_node_check();
    alloc_exact_nid_numa_node_reserved_check();
    alloc_exact_nid_numa_part_reserved_fail_check();
    alloc_exact_nid_numa_split_range_high_check();
    alloc_exact_nid_numa_no_overlap_high_check();
    alloc_exact_nid_numa_large_region_check();
    alloc_exact_nid_numa_reserved_full_merge_check();
    alloc_exact_nid_numa_split_all_reserved_check();

    0
}

pub unsafe fn memblock_alloc_exact_nid_checks() -> i32 {
    prefix_reset();
    prefix_push(FUNC_NAME);

    reset_memblock_attributes();
    dummy_physical_memory_init();

    memblock_alloc_exact_nid_range_checks();
    memblock_alloc_exact_nid_numa_checks();

    dummy_physical_memory_cleanup();

    prefix_pop();

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
