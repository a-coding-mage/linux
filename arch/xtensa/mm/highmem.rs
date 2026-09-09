/*
 * High memory support for Xtensa architecture
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 *
 * Copyright (C) 2014 Cadence Design Systems Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/export.h, linux/highmem.h, asm/tlbflush.h

// The following items are selected when DCACHE_WAY_SIZE > PAGE_SIZE.
#[cfg(dcache_way_size_gt_page_size)]
pub static mut last_pkmap_nr_arr: [u32; DCACHE_N_COLORS] = [0; DCACHE_N_COLORS];

#[cfg(dcache_way_size_gt_page_size)]
pub static mut pkmap_map_wait_arr: [wait_queue_head_t; DCACHE_N_COLORS] =
    [unsafe { core::mem::zeroed() }; DCACHE_N_COLORS];

#[cfg(dcache_way_size_gt_page_size)]
unsafe fn kmap_waitqueues_init() {
    let mut i: u32 = 0;

    while i < core::mem::size_of_val(&pkmap_map_wait_arr) as u32
        / core::mem::size_of::<wait_queue_head_t>() as u32
    {
        init_waitqueue_head(pkmap_map_wait_arr.as_mut_ptr().add(i as usize));
        i = i.wrapping_add(1);
    }
}

#[cfg(dcache_way_size_gt_page_size)]
#[inline]
unsafe fn kmap_idx(type_: i32, color: u64) -> fixed_addresses {
    let idx: i32 = (type_ + KM_MAX_IDX * smp_processor_id()) * DCACHE_N_COLORS;

    /*
     * The fixmap operates top down, so the color offset needs to be
     * reverse as well.
     */
    (idx + DCACHE_N_COLORS - 1 - color as i32) as fixed_addresses
}

#[cfg(dcache_way_size_gt_page_size)]
pub unsafe fn kmap_local_map_idx(type_: i32, pfn: u64) -> fixed_addresses {
    kmap_idx(type_, DCACHE_ALIAS(pfn << PAGE_SHIFT))
}

#[cfg(dcache_way_size_gt_page_size)]
pub unsafe fn kmap_local_unmap_idx(type_: i32, addr: u64) -> fixed_addresses {
    kmap_idx(type_, DCACHE_ALIAS(addr))
}

// When DCACHE_WAY_SIZE <= PAGE_SIZE, the C inline helper is empty.
#[cfg(not(dcache_way_size_gt_page_size))]
#[inline]
unsafe fn kmap_waitqueues_init() {}

pub unsafe fn kmap_init() {
    /* Check if this memory layout is broken because PKMAP overlaps
     * page table.
     */
    BUILD_BUG_ON(PKMAP_BASE < TLBTEMP_BASE_1 + TLBTEMP_SIZE);
    kmap_waitqueues_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
