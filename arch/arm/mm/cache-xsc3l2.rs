// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mm/cache-xsc3l2.c - XScale3 L2 cache controller support
 *
 * Copyright (C) 2007 ARM Limited
 */

const CR_L2: usize = 1 << 26;

const CACHE_LINE_SIZE: usize = 32;
const CACHE_LINE_SHIFT: usize = 5;
const CACHE_WAY_PER_SET: usize = 8;

const fn cache_way_size(l2ctype: usize) -> usize {
    8192 << ((l2ctype >> 8) & 0xf)
}

const fn cache_set_size(l2ctype: usize) -> usize {
    cache_way_size(l2ctype) >> CACHE_LINE_SHIFT
}

#[inline]
unsafe fn xsc3_l2_present() -> bool {
    let l2ctype: usize;
    core::arch::asm!("mrc p15, 1, {0}, c0, c0, 1", out(reg) l2ctype);
    (l2ctype & 0xf8) != 0
}

#[inline]
unsafe fn xsc3_l2_clean_mva(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c7, c11, 1", in(reg) addr);
}

#[inline]
unsafe fn xsc3_l2_inv_mva(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c7, c7, 1", in(reg) addr);
}

#[inline]
unsafe fn xsc3_l2_inv_all() {
    let l2ctype: usize;
    core::arch::asm!("mrc p15, 1, {0}, c0, c0, 1", out(reg) l2ctype);

    for set in 0..cache_set_size(l2ctype) {
        for way in 0..CACHE_WAY_PER_SET {
            let set_way = (way << 29) | (set << 5);
            core::arch::asm!("mcr p15, 1, {0}, c7, c11, 2", in(reg) set_way);
        }
    }

    dsb();
}

#[inline]
unsafe fn l2_unmap_va(va: usize) {
    // CONFIG_HIGHMEM conditional from the C source.
    #[cfg(CONFIG_HIGHMEM)]
    if va != usize::MAX {
        kunmap_atomic(va as *mut core::ffi::c_void);
    }
}

#[inline]
unsafe fn l2_map_va(pa: usize, prev_va: usize) -> usize {
    // CONFIG_HIGHMEM conditional from the C source.
    #[cfg(CONFIG_HIGHMEM)]
    {
        let mut va = prev_va & PAGE_MASK;
        let pa_offset = pa << (32 - PAGE_SHIFT);
        if pa_offset < (prev_va << (32 - PAGE_SHIFT)) {
            l2_unmap_va(prev_va);
            va = kmap_atomic_pfn(pa >> PAGE_SHIFT) as usize;
        }
        return va + (pa_offset >> (32 - PAGE_SHIFT));
    }
    #[cfg(not(CONFIG_HIGHMEM))]
    {
        __phys_to_virt(pa)
    }
}

unsafe fn xsc3_l2_inv_range(mut start: usize, end: usize) {
    if start == 0 && end == usize::MAX {
        xsc3_l2_inv_all();
        return;
    }

    let mut vaddr = usize::MAX; // to force the first mapping

    // Clean and invalidate partial first cache line.
    if start & (CACHE_LINE_SIZE - 1) != 0 {
        vaddr = l2_map_va(start & !(CACHE_LINE_SIZE - 1), vaddr);
        xsc3_l2_clean_mva(vaddr);
        xsc3_l2_inv_mva(vaddr);
        start = (start | (CACHE_LINE_SIZE - 1)) + 1;
    }

    // Invalidate all full cache lines between 'start' and 'end'.
    while start < (end & !(CACHE_LINE_SIZE - 1)) {
        vaddr = l2_map_va(start, vaddr);
        xsc3_l2_inv_mva(vaddr);
        start += CACHE_LINE_SIZE;
    }

    // Clean and invalidate partial last cache line.
    if start < end {
        vaddr = l2_map_va(start, vaddr);
        xsc3_l2_clean_mva(vaddr);
        xsc3_l2_inv_mva(vaddr);
    }

    l2_unmap_va(vaddr);
    dsb();
}

unsafe fn xsc3_l2_clean_range(mut start: usize, end: usize) {
    let mut vaddr = usize::MAX; // to force the first mapping

    start &= !(CACHE_LINE_SIZE - 1);
    while start < end {
        vaddr = l2_map_va(start, vaddr);
        xsc3_l2_clean_mva(vaddr);
        start += CACHE_LINE_SIZE;
    }

    l2_unmap_va(vaddr);
    dsb();
}

/*
 * optimize L2 flush all operation by set/way format
 */
#[inline]
unsafe fn xsc3_l2_flush_all() {
    let l2ctype: usize;
    core::arch::asm!("mrc p15, 1, {0}, c0, c0, 1", out(reg) l2ctype);

    for set in 0..cache_set_size(l2ctype) {
        for way in 0..CACHE_WAY_PER_SET {
            let set_way = (way << 29) | (set << 5);
            core::arch::asm!("mcr p15, 1, {0}, c7, c15, 2", in(reg) set_way);
        }
    }

    dsb();
}

unsafe fn xsc3_l2_flush_range(mut start: usize, end: usize) {
    if start == 0 && end == usize::MAX {
        xsc3_l2_flush_all();
        return;
    }

    let mut vaddr = usize::MAX; // to force the first mapping

    start &= !(CACHE_LINE_SIZE - 1);
    while start < end {
        vaddr = l2_map_va(start, vaddr);
        xsc3_l2_clean_mva(vaddr);
        xsc3_l2_inv_mva(vaddr);
        start += CACHE_LINE_SIZE;
    }

    l2_unmap_va(vaddr);
    dsb();
}

unsafe fn xsc3_l2_init() -> i32 {
    if !cpu_is_xsc3() || !xsc3_l2_present() {
        return 0;
    }

    if get_cr() & CR_L2 != 0 {
        pr_info("XScale3 L2 cache enabled.\n");
        xsc3_l2_inv_all();

        outer_cache.inv_range = Some(xsc3_l2_inv_range);
        outer_cache.clean_range = Some(xsc3_l2_clean_range);
        outer_cache.flush_range = Some(xsc3_l2_flush_range);
    }

    0
}

core_initcall!(xsc3_l2_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
