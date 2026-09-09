// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation unit:
// linux/memblock.h, linux/printk.h, linux/numa.h, linux/numa_memblks.h

use core::ffi::c_void;

// External kernel types, constants, functions, and macros are supplied by
// the corresponding translated headers.
extern "C" {
    pub static mut node_data: [*mut pglist_data; MAX_NUMNODES];

    fn memblock_phys_alloc_try_nid(size: usize, align: usize, nid: i32) -> u64;
    fn early_pfn_to_nid(pfn: u64) -> i32;
    fn __va(pa: u64) -> *mut c_void;
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut c_void;
    fn memory_add_physaddr_to_nid(start: u64) -> i32;
    fn phys_to_target_node(start: u64) -> i32;
}

// These declarations represent types and build-time constants from the
// included kernel headers.
#[repr(C)]
pub struct pglist_data {
    _opaque: [u8; 0],
}
pub type pg_data_t = pglist_data;

extern "Rust" {
    static MAX_NUMNODES: usize;
    static SMP_CACHE_BYTES: usize;
    static PAGE_SHIFT: u32;
}

// Allocate NODE_DATA for a node on the local memory
#[no_mangle]
pub unsafe extern "C" fn alloc_node_data(nid: i32) {
    let nd_size: usize = (core::mem::size_of::<pg_data_t>() + SMP_CACHE_BYTES - 1)
        & !(SMP_CACHE_BYTES - 1);
    let nd_pa: u64;
    let tnid: i32;

    // Allocate node data.  Try node-local memory and then any node.
    nd_pa = memblock_phys_alloc_try_nid(nd_size, SMP_CACHE_BYTES, nid);
    if nd_pa == 0 {
        panic!("Cannot allocate {} bytes for node {} data\n", nd_size, nid);
    }

    // report and initialize
    // pr_info!("NODE_DATA({}) allocated [mem {:#010x}-{:#010x}]\n", nid,
    //     nd_pa, nd_pa + nd_size as u64 - 1);
    tnid = early_pfn_to_nid(nd_pa >> PAGE_SHIFT);
    if tnid != nid {
        // pr_info!("    NODE_DATA({}) on node {}\n", nid, tnid);
    }

    node_data[nid as usize] = __va(nd_pa) as *mut pglist_data;
    // NODE_DATA(nid) expands to the node_data entry for nid in the kernel.
    core::ptr::write_bytes(node_data[nid as usize] as *mut u8, 0, core::mem::size_of::<pg_data_t>());
}

#[no_mangle]
pub unsafe extern "C" fn alloc_offline_node_data(nid: i32) {
    let pgdat: *mut pg_data_t;
    node_data[nid as usize] = memblock_alloc_or_panic(
        core::mem::size_of::<pg_data_t>(),
        SMP_CACHE_BYTES,
    ) as *mut pglist_data;
    pgdat = node_data[nid as usize] as *mut pg_data_t;
    let _ = pgdat;
}

/* Stub functions: */

// C preprocessor condition: compile these stubs only when the corresponding
// external declarations are not provided by the build.
#[no_mangle]
pub unsafe extern "C" fn memory_add_physaddr_to_nid_stub(start: u64) -> i32 {
    // pr_info_once!("Unknown online node for memory at 0x{:x}, assuming node 0\n", start);
    let _ = start;
    0
}

#[no_mangle]
pub unsafe extern "C" fn phys_to_target_node_stub(start: u64) -> i32 {
    // pr_info_once!("Unknown target node for memory at 0x{:x}, assuming node 0\n", start);
    let _ = start;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
