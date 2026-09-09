// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mm/cache-feroceon-l2.c - Feroceon L2 cache controller support
 *
 * Copyright (C) 2008 Marvell Semiconductor
 *
 * References:
 * - Unified Layer 2 Cache for Feroceon CPU Cores,
 *   Document ID MV-S104858-00, Rev. A, October 23 2007.
 */

// C dependencies: linux/init.h, linux/of*.h, linux/highmem.h, linux/io.h,
// asm/cacheflush.h, asm/cp15.h, and asm/hardware/cache-feroceon-l2.h.

const L2_WRITETHROUGH_KIRKWOOD: u32 = 1 << 4;

/*
 * Low-level cache maintenance operations.
 *
 * As well as the regular 'clean/invalidate/flush L2 cache line by
 * MVA' instructions, the Feroceon L2 cache controller also features
 * 'clean/invalidate L2 range by MVA' operations.
 *
 * Cache range operations are initiated by writing the start and
 * end addresses to successive cp15 registers, and process every
 * cache line whose first byte address lies in the inclusive range
 * [start:end].
 *
 * The cache range operations stall the CPU pipeline until completion.
 *
 * The range operations require two successive cp15 writes, in
 * between which we don't want to be preempted.
 */

#[inline]
unsafe fn l2_get_va(paddr: usize) -> usize {
    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        /* Because range ops can't be done on physical addresses, install a
         * virtual mapping only for the TLB lookup to occur. */
        let vaddr = kmap_atomic_pfn(paddr >> PAGE_SHIFT);
        return vaddr as usize + (paddr & !PAGE_MASK);
    }
    #[cfg(not(feature = "CONFIG_HIGHMEM"))]
    {
        __phys_to_virt(paddr)
    }
}

#[inline]
unsafe fn l2_put_va(vaddr: usize) {
    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        kunmap_atomic(vaddr as *mut core::ffi::c_void);
    }
}

#[inline]
unsafe fn l2_clean_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c15, c9, 3", in(reg) addr);
}

#[inline]
unsafe fn l2_clean_pa_range(start: usize, end: usize) {
    /* Make sure start and end reference the same page. */
    BUG_ON(((start ^ end) >> PAGE_SHIFT) != 0);
    let va_start = l2_get_va(start);
    let va_end = va_start + (end - start);
    let mut flags: usize = 0;
    raw_local_irq_save(&mut flags);
    core::arch::asm!("mcr p15, 1, {0}, c15, c9, 4\n\tmcr p15, 1, {1}, c15, c9, 5", in(reg) va_start, in(reg) va_end);
    raw_local_irq_restore(flags);
    l2_put_va(va_start);
}

#[inline]
unsafe fn l2_clean_inv_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c15, c10, 3", in(reg) addr);
}

#[inline]
unsafe fn l2_inv_pa(addr: usize) {
    core::arch::asm!("mcr p15, 1, {0}, c15, c11, 3", in(reg) addr);
}

#[inline]
unsafe fn l2_inv_pa_range(start: usize, end: usize) {
    BUG_ON(((start ^ end) >> PAGE_SHIFT) != 0);
    let va_start = l2_get_va(start);
    let va_end = va_start + (end - start);
    let mut flags: usize = 0;
    raw_local_irq_save(&mut flags);
    core::arch::asm!("mcr p15, 1, {0}, c15, c11, 4\n\tmcr p15, 1, {1}, c15, c11, 5", in(reg) va_start, in(reg) va_end);
    raw_local_irq_restore(flags);
    l2_put_va(va_start);
}

#[inline]
unsafe fn l2_inv_all() {
    core::arch::asm!("mcr p15, 1, {0}, c15, c11, 0", in(reg) 0usize);
}

const CACHE_LINE_SIZE: usize = 32;
const MAX_RANGE_SIZE: usize = 1024;
static mut l2_wt_override: i32 = 0;

unsafe fn calc_range_end(start: usize, end: usize) -> usize {
    BUG_ON((start & (CACHE_LINE_SIZE - 1)) != 0);
    BUG_ON((end & (CACHE_LINE_SIZE - 1)) != 0);
    let mut range_end = end;
    if range_end > start + MAX_RANGE_SIZE { range_end = start + MAX_RANGE_SIZE; }
    if range_end > (start | (PAGE_SIZE - 1)) + 1 { range_end = (start | (PAGE_SIZE - 1)) + 1; }
    range_end
}

unsafe fn feroceon_l2_inv_range(mut start: usize, mut end: usize) {
    if (start & (CACHE_LINE_SIZE - 1)) != 0 {
        l2_clean_inv_pa(start & !(CACHE_LINE_SIZE - 1));
        start = (start | (CACHE_LINE_SIZE - 1)) + 1;
    }
    if start < end && (end & (CACHE_LINE_SIZE - 1)) != 0 {
        l2_clean_inv_pa(end & !(CACHE_LINE_SIZE - 1));
        end &= !(CACHE_LINE_SIZE - 1);
    }
    while start < end {
        let range_end = calc_range_end(start, end);
        l2_inv_pa_range(start, range_end - CACHE_LINE_SIZE);
        start = range_end;
    }
    dsb();
}

unsafe fn feroceon_l2_clean_range(mut start: usize, mut end: usize) {
    if l2_wt_override == 0 {
        start &= !(CACHE_LINE_SIZE - 1);
        end = (end + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);
        while start != end {
            let range_end = calc_range_end(start, end);
            l2_clean_pa_range(start, range_end - CACHE_LINE_SIZE);
            start = range_end;
        }
    }
    dsb();
}

unsafe fn feroceon_l2_flush_range(mut start: usize, mut end: usize) {
    start &= !(CACHE_LINE_SIZE - 1);
    end = (end + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);
    while start != end {
        let range_end = calc_range_end(start, end);
        if l2_wt_override == 0 { l2_clean_pa_range(start, range_end - CACHE_LINE_SIZE); }
        l2_inv_pa_range(start, range_end - CACHE_LINE_SIZE);
        start = range_end;
    }
    dsb();
}

unsafe fn flush_and_disable_dcache() -> i32 {
    let cr = get_cr();
    if cr & CR_C != 0 {
        let mut flags: usize = 0;
        raw_local_irq_save(&mut flags);
        flush_cache_all();
        set_cr(cr & !CR_C);
        raw_local_irq_restore(flags);
        return 1;
    }
    0
}

unsafe fn enable_dcache() { let cr = get_cr(); set_cr(cr | CR_C); }

unsafe fn __invalidate_icache() { core::arch::asm!("mcr p15, 0, {0}, c7, c5, 0", in(reg) 0usize); }

unsafe fn invalidate_and_disable_icache() -> i32 {
    let cr = get_cr();
    if cr & CR_I != 0 { set_cr(cr & !CR_I); __invalidate_icache(); return 1; }
    0
}

unsafe fn enable_icache() { let cr = get_cr(); set_cr(cr | CR_I); }

#[inline]
unsafe fn read_extra_features() -> u32 {
    let u: u32;
    core::arch::asm!("mrc p15, 1, {0}, c15, c1, 0", out(reg) u);
    u
}

#[inline]
unsafe fn write_extra_features(u: u32) { core::arch::asm!("mcr p15, 1, {0}, c15, c1, 0", in(reg) u); }

unsafe fn disable_l2_prefetch() {
    let u = read_extra_features();
    if u & 0x01000000 == 0 { pr_info!("Feroceon L2: Disabling L2 prefetch.\n"); write_extra_features(u | 0x01000000); }
}

unsafe fn enable_l2() {
    let u = read_extra_features();
    if u & 0x00400000 == 0 {
        pr_info!("Feroceon L2: Enabling L2\n");
        let d = flush_and_disable_dcache();
        let i = invalidate_and_disable_icache();
        l2_inv_all();
        write_extra_features(u | 0x00400000);
        if i != 0 { enable_icache(); }
        if d != 0 { enable_dcache(); }
    } else { pr_err!(concat!(FW_BUG, "Feroceon L2: bootloader left the L2 cache on!\n")); }
}

pub unsafe fn feroceon_l2_init(l2_wt_override_arg: i32) {
    l2_wt_override = l2_wt_override_arg;
    disable_l2_prefetch();
    outer_cache.inv_range = Some(feroceon_l2_inv_range);
    outer_cache.clean_range = Some(feroceon_l2_clean_range);
    outer_cache.flush_range = Some(feroceon_l2_flush_range);
    enable_l2();
    pr_info!("Feroceon L2: Cache support initialised{} .\n", if l2_wt_override != 0 { ", in WT override mode" } else { "" });
}

// CONFIG_OF declarations and feroceon_of_init are preserved below.
#[cfg(feature = "CONFIG_OF")]
unsafe fn feroceon_of_init() -> i32 {
    let mut l2_wt_override_of = false;
    #[cfg(feature = "CONFIG_CACHE_FEROCEON_L2_WRITETHROUGH")]
    { l2_wt_override_of = true; }
    let node = of_find_matching_node(core::ptr::null_mut(), feroceon_ids.as_ptr());
    if !node.is_null() && of_device_is_compatible(node, b"marvell,kirkwood-cache\0".as_ptr()) {
        let base = of_iomap(node, 0);
        if base.is_null() { return -12; }
        let value = readl(base);
        if l2_wt_override_of { writel(value | L2_WRITETHROUGH_KIRKWOOD, base); }
        else { writel(value & !L2_WRITETHROUGH_KIRKWOOD, base); }
    }
    feroceon_l2_init(if l2_wt_override_of { 1 } else { 0 });
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
