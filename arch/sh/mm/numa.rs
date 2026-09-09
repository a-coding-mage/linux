/*
 * arch/sh/mm/numa.c - Multiple node support for SH machines
 *
 *  Copyright (C) 2007  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Declarations supplied by the kernel headers in the original translation unit.
extern "C" {
    fn pmb_bolt_mapping(virt: usize, phys: usize, size: usize, prot: usize);
    fn memblock_add(base: usize, size: usize);
    fn __add_active_range(nid: i32, start_pfn: usize, end_pfn: usize);
    fn memblock_alloc_node(size: usize, align: usize, nid: i32) -> *mut pglist_data;
    fn node_set_online(nid: i32);
    fn __va(addr: usize) -> usize;
}

#[repr(C)]
pub struct pglist_data {
    pub node_start_pfn: usize,
    pub node_spanned_pages: usize,
}

extern "C" {
    static mut NODE_DATA: [*mut pglist_data; MAX_NUMNODES];
}

const MAX_NUMNODES: usize = 1; // Supplied by the build configuration.
const SMP_CACHE_BYTES: usize = 0; // Supplied by the architecture headers.
const PAGE_KERNEL: usize = 0; // Supplied by the architecture headers.

#[inline]
fn pfn_down(value: usize) -> usize {
    value >> PAGE_SHIFT
}

const PAGE_SHIFT: usize = 0; // Supplied by the architecture headers.

/*
 * On SH machines the conventional approach is to stash system RAM
 * in node 0, and other memory blocks in to node 1 and up, ordered by
 * latency. Each node's pgdat is node-local at the beginning of the node,
 * immediately followed by the node mem map.
 */
pub unsafe extern "C" fn setup_bootmem_node(nid: i32, start: usize, end: usize) {
    let start_pfn: usize;
    let end_pfn: usize;

    /* Don't allow bogus node assignment */
    assert!(nid < MAX_NUMNODES as i32 && nid > 0);

    start_pfn = pfn_down(start);
    end_pfn = pfn_down(end);

    pmb_bolt_mapping(__va(start), start, end.wrapping_sub(start), PAGE_KERNEL);

    memblock_add(start, end.wrapping_sub(start));

    __add_active_range(nid, start_pfn, end_pfn);

    /* Node-local pgdat */
    NODE_DATA[nid as usize] = memblock_alloc_node(
        core::mem::size_of::<pglist_data>(),
        SMP_CACHE_BYTES,
        nid,
    );
    if NODE_DATA[nid as usize].is_null() {
        panic!(
            "setup_bootmem_node: Failed to allocate {} bytes align=0x{:x} nid={}\n",
            core::mem::size_of::<pglist_data>(),
            SMP_CACHE_BYTES,
            nid,
        );
    }

    (*NODE_DATA[nid as usize]).node_start_pfn = start_pfn;
    (*NODE_DATA[nid as usize]).node_spanned_pages = end_pfn.wrapping_sub(start_pfn);

    /* It's up */
    node_set_online(nid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
