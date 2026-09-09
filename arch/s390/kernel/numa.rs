// SPDX-License-Identifier: GPL-2.0
/*
 * NUMA support for s390
 *
 * Implement NUMA core code.
 *
 * Copyright IBM Corp. 2015
 */

// Dependencies supplied by the kernel headers and by other translation units.

use core::ffi::c_void;

pub const PAGE_SHIFT: usize = 0; // Supplied by the kernel configuration.
pub const MAX_NUMNODES: i32 = 0; // Supplied by the kernel configuration.

#[repr(C)]
pub struct pg_data_t {
    pub node_spanned_pages: usize,
    pub node_id: i32,
}

unsafe extern "C" {
    static mut node_possible_map: c_void;

    fn nodes_clear(map: *mut c_void);
    fn node_set(nid: i32, map: *mut c_void);
    fn node_set_online(nid: i32);
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut c_void;
    fn memblock_end_of_DRAM() -> usize;

    // C macro NODE_DATA(nid), represented as its underlying accessor.
    fn NODE_DATA(nid: i32) -> *mut *mut pg_data_t;
}

pub unsafe fn numa_setup() {
    let mut nid: i32;

    nodes_clear(&raw mut node_possible_map);
    node_set(0, &raw mut node_possible_map);
    node_set_online(0);
    nid = 0;
    while nid < MAX_NUMNODES {
        *NODE_DATA(nid) = memblock_alloc_or_panic(
            core::mem::size_of::<pg_data_t>(),
            8,
        ) as *mut pg_data_t;
        nid += 1;
    }
    (**NODE_DATA(0)).node_spanned_pages = memblock_end_of_DRAM() >> PAGE_SHIFT;
    (**NODE_DATA(0)).node_id = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
