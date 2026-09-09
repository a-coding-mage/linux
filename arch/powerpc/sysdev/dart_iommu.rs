// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/sysdev/dart_iommu.c
 *
 * Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
 * Copyright (C) 2005 Benjamin Herrenschmidt <benh@kernel.crashing.org>,
 *                    IBM Corporation
 *
 * Based on pSeries_iommu.c:
 * Copyright (C) 2001 Mike Corrigan & Dave Engebretsen, IBM Corporation
 * Copyright (C) 2004 Olof Johansson <olof@lixom.net>, IBM Corporation
 *
 * Dynamic DMA mapping support, Apple U3, U4 & IBM CPC925 "DART" iommu.
 */

// Kernel and architecture dependencies supplied externally.

/* DART table address and size */
static mut dart_tablebase: *mut u32 = core::ptr::null_mut();
static mut dart_tablesize: usize = 0;

/* Mapped base address for the dart */
static mut dart: *mut u32 = core::ptr::null_mut();

/* Dummy val that entries are set to when unused */
static mut dart_emptyval: u32 = 0;

static mut iommu_table_dart: iommu_table = unsafe { core::mem::zeroed() };
static mut iommu_table_dart_inited: i32 = 0;
static mut dart_dirty: i32 = 0;
static mut dart_is_u4: i32 = 0;

const DART_U4_BYPASS_BASE: u64 = 0x8000000000;

static mut invalidate_lock: spinlock_t = unsafe { core::mem::zeroed() };

unsafe fn dart_tlb_invalidate_all() {
    let mut l: usize = 0;
    let mut reg: u32;
    let inv_bit: u32;
    let mut limit: usize = 0;
    let mut flags: ulong = 0;

    spin_lock_irqsave(&raw mut invalidate_lock, &mut flags);
    inv_bit = if dart_is_u4 != 0 { DART_CNTL_U4_FLUSHTLB } else { DART_CNTL_U3_FLUSHTLB };
retry:
    l = 0;
    reg = DART_IN(DART_CNTL);
    reg |= inv_bit;
    DART_OUT(DART_CNTL, reg);
    while (DART_IN(DART_CNTL) & inv_bit) != 0 && l < (1usize << limit) { l += 1; }
    if l == (1usize << limit) {
        if limit < 4 {
            limit += 1;
            reg = DART_IN(DART_CNTL);
            reg &= !inv_bit;
            DART_OUT(DART_CNTL, reg);
            goto_retry!();
        } else { panic!("DART: TLB did not flush after waiting a long time. Buggy U3 ?"); }
    }
    spin_unlock_irqrestore(&raw mut invalidate_lock, flags);
}

unsafe fn dart_tlb_invalidate_one(mut bus_rpn: ulong) {
    let mut reg: u32;
    let mut l: u32;
    let mut limit: u32 = 0;
    let mut flags: ulong = 0;
    spin_lock_irqsave(&raw mut invalidate_lock, &mut flags);
    reg = DART_CNTL_U4_ENABLE | DART_CNTL_U4_IONE | (bus_rpn as u32 & DART_CNTL_U4_IONE_MASK);
    DART_OUT(DART_CNTL, reg);
wait_more:
    l = 0;
    while (DART_IN(DART_CNTL) & DART_CNTL_U4_IONE) != 0 && l < (1u32 << limit) { rmb(); l += 1; }
    if l == (1u32 << limit) {
        if limit < 4 { limit += 1; goto_wait_more!(); }
        else { panic!("DART: TLB did not flush after waiting a long time. Buggy U4 ?"); }
    }
    spin_unlock_irqrestore(&raw mut invalidate_lock, flags);
}

unsafe fn dart_cache_sync(base: *mut u32, count: u32) {
    let start = base as ulong;
    let end = start + ((count as ulong + 1) * core::mem::size_of::<u32>() as ulong);
    let mut tmp: u32;
    flush_dcache_range(start, end);
    core::arch::asm!("sync; isync; dcbf 0,{1}; sync; isync; lwz {0},0({1}); isync", out(reg) tmp, in(reg) end, options(nostack, preserves_flags));
}

unsafe fn dart_flush(_tbl: *mut iommu_table) {
    mb();
    if dart_dirty != 0 { dart_tlb_invalidate_all(); dart_dirty = 0; }
}

unsafe fn dart_build(tbl: *mut iommu_table, index: long, mut npages: long, mut uaddr: ulong, _direction: dma_data_direction, _attrs: ulong) -> i32 {
    let orig_npages = npages;
    let mut dp = ((*tbl).it_base as *mut u32).offset(index as isize);
    let orig_dp = dp;
    while npages > 0 {
        let rpn = __pa(uaddr) >> DART_PAGE_SHIFT;
        *dp = DARTMAP_VALID | (rpn as u32 & DARTMAP_RPNMASK);
        dp = dp.add(1); uaddr += DART_PAGE_SIZE as ulong; npages -= 1;
    }
    dart_cache_sync(orig_dp, orig_npages as u32);
    if dart_is_u4 != 0 { let mut rpn = index as ulong; let mut n = orig_npages; while n > 0 { dart_tlb_invalidate_one(rpn); rpn += 1; n -= 1; } } else { dart_dirty = 1; }
    0
}

unsafe fn dart_free(tbl: *mut iommu_table, index: long, mut npages: long) {
    let orig_npages = npages;
    let mut dp = ((*tbl).it_base as *mut u32).offset(index as isize);
    let orig_dp = dp;
    while npages > 0 { *dp = dart_emptyval; dp = dp.add(1); npages -= 1; }
    dart_cache_sync(orig_dp, orig_npages as u32);
}

unsafe fn allocate_dart() {
    dart_tablesize = 1usize << 21;
    dart_tablebase = memblock_alloc_try_nid_raw(SZ_16M, SZ_16M, MEMBLOCK_LOW_LIMIT, SZ_2G, NUMA_NO_NODE) as *mut u32;
    if dart_tablebase.is_null() { panic!("Failed to allocate 16MB below 2GB for DART table\n"); }
    let tmp = memblock_phys_alloc(DART_PAGE_SIZE, DART_PAGE_SIZE);
    if tmp == 0 { panic!("DART: table allocation failed\n"); }
    dart_emptyval = DARTMAP_VALID | ((tmp >> DART_PAGE_SHIFT) as u32 & DARTMAP_RPNMASK);
    printk(KERN_INFO, "DART table allocated at: %p\n", dart_tablebase);
}

unsafe fn iommu_table_dart_setup() { /* fields and helpers supplied by kernel bindings */ }
unsafe fn pci_dma_bus_setup_dart(_bus: *mut pci_bus) { if iommu_table_dart_inited == 0 { iommu_table_dart_inited = 1; iommu_table_dart_setup(); } }
unsafe fn dart_device_on_pcie(_dev: *mut device) -> bool { false }
unsafe fn pci_dma_dev_setup_dart(_dev: *mut pci_dev) {}
unsafe fn iommu_bypass_supported_dart(_dev: *mut pci_dev, mask: u64) -> bool { dart_is_u4 != 0 && mask >= (1u64 << 40) }
unsafe fn iommu_init_early_dart(_controller_ops: *mut pci_controller_ops) {}

#[cfg(feature = "CONFIG_PM")]
unsafe fn iommu_dart_restore() {
    dart_cache_sync(dart_tablebase, (dart_tablesize / core::mem::size_of::<u32>()) as u32);
    dart_tlb_invalidate_all();
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn iommu_init_late_dart() -> i32 {
    if dart_tablebase.is_null() { return 0; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
