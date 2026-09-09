/*
 * include/asm-xtensa/highmem.h
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 *
 * Copyright (C) 2003 - 2005 Tensilica Inc.
 * Copyright (C) 2014 Cadence Design Systems Inc.
 */

// The following declarations are present when CONFIG_HIGHMEM is enabled.

pub const PKMAP_BASE: usize =
    (FIXADDR_START - (LAST_PKMAP + 1) * PAGE_SIZE) & PMD_MASK;
pub const LAST_PKMAP: usize = PTRS_PER_PTE * DCACHE_N_COLORS;
pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline]
pub const fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline]
pub const fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

pub const kmap_prot: _ = PAGE_KERNEL_EXEC;

// This section corresponds to: #if DCACHE_WAY_SIZE > PAGE_SIZE.
#[inline]
pub unsafe fn get_pkmap_color(page: *const struct_page) -> i32 {
    DCACHE_ALIAS(page_to_phys(page))
}

extern "C" {
    pub static mut last_pkmap_nr_arr: [u32; DCACHE_N_COLORS];
}

#[inline]
pub unsafe fn get_next_pkmap_nr(color: u32) -> u32 {
    let index = color as usize;
    last_pkmap_nr_arr[index] =
        (last_pkmap_nr_arr[index].wrapping_add(DCACHE_N_COLORS as u32))
            & LAST_PKMAP_MASK as u32;
    last_pkmap_nr_arr[index].wrapping_add(color)
}

#[inline]
pub const fn no_more_pkmaps(pkmap_nr: u32, _color: u32) -> i32 {
    (pkmap_nr < DCACHE_N_COLORS as u32) as i32
}

#[inline]
pub const fn get_pkmap_entries_count(_color: u32) -> u32 {
    (LAST_PKMAP / DCACHE_N_COLORS) as u32
}

extern "C" {
    pub static mut pkmap_map_wait_arr: [wait_queue_head_t; DCACHE_N_COLORS];
}

#[inline]
pub unsafe fn get_pkmap_wait_queue_head(color: u32) -> *mut wait_queue_head_t {
    pkmap_map_wait_arr.as_mut_ptr().add(color as usize)
}

extern "C" {
    pub fn kmap_local_map_idx(type_: i32, pfn: c_ulong) -> fixed_addresses;
    pub fn kmap_local_unmap_idx(type_: i32, addr: c_ulong) -> fixed_addresses;
}

pub use kmap_local_map_idx as arch_kmap_local_map_idx;
pub use kmap_local_unmap_idx as arch_kmap_local_unmap_idx;

extern "C" {
    pub static mut pkmap_page_table: *mut pte_t;
}

#[inline]
pub unsafe fn flush_cache_kmaps() {
    flush_cache_all();
}

#[inline]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: c_ulong) {
    local_flush_tlb_kernel_range(vaddr, vaddr + PAGE_SIZE as c_ulong);
}

extern "C" {
    pub fn kmap_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
