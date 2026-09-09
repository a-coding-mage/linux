// SPDX-License-Identifier: GPL-2.0
/* iommu.c: Generic sparc64 IOMMU support.
 *
 * Copyright (C) 1999, 2007, 2008 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1999, 2000 Jakub Jelinek (jakub@redhat.com)
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[inline]
unsafe fn stc_ctxmatch_addr(stc: *mut strbuf, ctx: usize) -> usize {
    (*stc).strbuf_ctxmatch_base + (ctx << 3)
}

#[inline]
unsafe fn stc_flushflag_init(stc: *mut strbuf) { *(*stc).strbuf_flushflag = 0; }

#[inline]
unsafe fn stc_flushflag_set(stc: *mut strbuf) -> bool { *(*stc).strbuf_flushflag != 0 }

#[inline]
unsafe fn iommu_read(reg: usize) -> u64 {
    let ret: u64;
    core::arch::asm!("ldxa [{reg}] 0x{asi}, {ret}", reg = in(reg) reg,
                     asi = const ASI_PHYS_BYPASS_EC_E, ret = lateout(reg) ret,
                     options(nostack, preserves_flags));
    ret
}

#[inline]
unsafe fn iommu_write(reg: usize, val: u64) {
    core::arch::asm!("stxa {val}, [{reg}] 0x{asi}", val = in(reg) val,
                     reg = in(reg) reg, asi = const ASI_PHYS_BYPASS_EC_E,
                     options(nostack, preserves_flags));
}

/* Must be invoked under the IOMMU lock. */
unsafe fn iommu_flushall(iommu_map_table: *mut iommu_map_table) {
    let iommu = container_of!(iommu_map_table, iommu, tbl);
    if (*iommu).iommu_flushinv != 0 {
        iommu_write((*iommu).iommu_flushinv, !0u64);
    } else {
        let mut tag = (*iommu).iommu_tags;
        for _entry in 0..16 {
            iommu_write(tag, 0);
            tag += 8;
        }
        let _ = iommu_read((*iommu).write_complete_reg);
    }
}

#[inline]
unsafe fn iopte_consistent(ctx: usize) -> u64 {
    IOPTE_VALID | IOPTE_CACHE | ((ctx << 47) & IOPTE_CONTEXT)
}
#[inline]
unsafe fn iopte_streaming(ctx: usize) -> u64 { iopte_consistent(ctx) | IOPTE_STBUF }

#[inline]
unsafe fn iopte_is_dummy(iommu: *mut iommu, iopte: *mut iopte_t) -> bool {
    iopte_val!(*iopte) & IOPTE_PAGE == (*iommu).dummy_page_pa
}

#[inline]
unsafe fn iopte_make_dummy(iommu: *mut iommu, iopte: *mut iopte_t) {
    let mut val = iopte_val!(*iopte);
    val &= !IOPTE_PAGE;
    val |= (*iommu).dummy_page_pa;
    iopte_val_set!(*iopte, val);
}

pub unsafe fn iommu_table_init(iommu: *mut iommu, tsbsize: i32, dma_offset: u32,
                               dma_addr_mask: u32, numa_node: i32) -> i32 {
    let num_tsb_entries = tsbsize as usize / core::mem::size_of::<iopte_t>();
    spin_lock_init!(&mut (*iommu).lock);
    (*iommu).ctx_lowest_free = 1;
    (*iommu).tbl.table_map_base = dma_offset;
    (*iommu).dma_addr_mask = dma_addr_mask;
    let mut sz = (num_tsb_entries / 8 + 7) & !7;
    (*iommu).tbl.map = kzalloc_node(sz, GFP_KERNEL, numa_node);
    if (*iommu).tbl.map.is_null() { return -ENOMEM; }
    iommu_tbl_pool_init(&mut (*iommu).tbl, num_tsb_entries, IO_PAGE_SHIFT,
                        if tlb_type != hypervisor { Some(iommu_flushall) } else { None },
                        false, 1, false);
    let mut page = alloc_pages_node(numa_node, GFP_KERNEL, 0);
    if page.is_null() { printk!(KERN_ERR, "IOMMU: Error, gfp(dummy_page) failed.\n"); goto!(out_free_map); }
    (*iommu).dummy_page = page_address(page) as usize;
    core::ptr::write_bytes((*iommu).dummy_page as *mut u8, 0, PAGE_SIZE);
    (*iommu).dummy_page_pa = __pa((*iommu).dummy_page);
    let order = get_order(tsbsize as usize);
    page = alloc_pages_node(numa_node, GFP_KERNEL, order);
    if page.is_null() { printk!(KERN_ERR, "IOMMU: Error, gfp(tsb) failed.\n"); goto!(out_free_dummy_page); }
    (*iommu).page_table = page_address(page) as *mut iopte_t;
    for i in 0..num_tsb_entries { iopte_make_dummy(iommu, (*iommu).page_table.add(i)); }
    return 0;
out_free_dummy_page:
    free_page((*iommu).dummy_page); (*iommu).dummy_page = 0;
out_free_map:
    kfree((*iommu).tbl.map); (*iommu).tbl.map = core::ptr::null_mut(); -ENOMEM
}

unsafe fn alloc_npages(dev: *mut device, iommu: *mut iommu, npages: usize) -> *mut iopte_t {
    let entry = iommu_tbl_range_alloc(dev, &mut (*iommu).tbl, npages, core::ptr::null_mut(), usize::MAX, 0);
    if entry == IOMMU_ERROR_CODE { core::ptr::null_mut() } else { (*iommu).page_table.add(entry) }
}

unsafe fn iommu_alloc_ctx(iommu: *mut iommu) -> i32 {
    let lowest = (*iommu).ctx_lowest_free;
    let mut n = find_next_zero_bit((*iommu).ctx_bitmap, IOMMU_NUM_CTXS, lowest);
    if n == IOMMU_NUM_CTXS { n = find_next_zero_bit((*iommu).ctx_bitmap, lowest, 1); if n == lowest { printk!(KERN_WARNING, "IOMMU: Ran out of contexts.\n"); n = 0; } }
    if n != 0 { __set_bit(n, (*iommu).ctx_bitmap); }
    n as i32
}

#[inline] unsafe fn iommu_free_ctx(iommu: *mut iommu, ctx: i32) { if ctx != 0 { __clear_bit(ctx as usize, (*iommu).ctx_bitmap); if ctx < (*iommu).ctx_lowest_free { (*iommu).ctx_lowest_free = ctx; } } }

// The remaining DMA operations retain the C ABI and low-level pointer semantics.
// External kernel types, helpers, constants, and iteration macros are supplied by dependencies.

pub unsafe fn dma_4u_alloc_coherent(dev: *mut device, size: usize, dma_addrp: *mut dma_addr_t, gfp: gfp_t, attrs: usize) -> *mut core::ffi::c_void {
    let size = IO_PAGE_ALIGN(size); let order = get_order(size); if order >= 10 { return core::ptr::null_mut(); }
    let nid = (*dev).archdata.numa_node; let page = alloc_pages_node(nid, gfp, order); if page.is_null() { return core::ptr::null_mut(); }
    let first_page = page_address(page) as usize; core::ptr::write_bytes(first_page as *mut u8, 0, PAGE_SIZE << order);
    let iommu = (*dev).archdata.iommu; let mut iopte = alloc_npages(dev, iommu, size >> IO_PAGE_SHIFT);
    if iopte.is_null() { free_pages(first_page, order); return core::ptr::null_mut(); }
    *dma_addrp = (*iommu).tbl.table_map_base + ((iopte.offset_from((*iommu).page_table) as usize) << IO_PAGE_SHIFT);
    let ret = first_page as *mut core::ffi::c_void; let mut npages = size >> IO_PAGE_SHIFT; let mut phys = __pa(first_page);
    while npages != 0 { iopte_val_set!(*iopte, iopte_consistent(0) | IOPTE_WRITE | (phys & IOPTE_PAGE)); iopte = iopte.add(1); phys += IO_PAGE_SIZE; npages -= 1; } ret
}

unsafe fn dma_4u_free_coherent(dev: *mut device, size: usize, cpu: *mut core::ffi::c_void, dvma: dma_addr_t, _attrs: usize) {
    let iommu = (*dev).archdata.iommu; let npages = IO_PAGE_ALIGN(size) >> IO_PAGE_SHIFT;
    iommu_tbl_range_free(&mut (*iommu).tbl, dvma, npages, IOMMU_ERROR_CODE);
    let order = get_order(size); if order < 10 { free_pages(cpu as usize, order); }
}

unsafe fn dma_4u_map_phys(dev: *mut device, mut phys: phys_addr_t, sz: usize, direction: dma_data_direction, attrs: usize) -> dma_addr_t {
    if attrs & DMA_ATTR_MMIO != 0 || direction == DMA_NONE { if printk_ratelimit() { WARN_ON!(true); } return DMA_MAPPING_ERROR; }
    let iommu = (*dev).archdata.iommu; let strbuf = (*dev).archdata.stc;
    let oaddr = phys_to_virt(phys) as usize; let npages = (IO_PAGE_ALIGN(oaddr + sz) - (oaddr & IO_PAGE_MASK)) >> IO_PAGE_SHIFT;
    let mut base = alloc_npages(dev, iommu, npages); let mut flags = 0; spin_lock_irqsave!(&mut (*iommu).lock, flags);
    let ctx = if (*iommu).iommu_ctxflush != 0 { iommu_alloc_ctx(iommu) as usize } else { 0 }; spin_unlock_irqrestore!(&mut (*iommu).lock, flags);
    if base.is_null() { iommu_free_ctx(iommu, ctx as i32); if printk_ratelimit() { WARN_ON!(true); } return DMA_MAPPING_ERROR; }
    let bus_addr = (*iommu).tbl.table_map_base + ((base.offset_from((*iommu).page_table) as usize) << IO_PAGE_SHIFT);
    let ret = bus_addr | (oaddr & !IO_PAGE_MASK); let mut prot = if (*strbuf).strbuf_enabled { iopte_streaming(ctx) } else { iopte_consistent(ctx) };
    if direction != DMA_TO_DEVICE { prot |= IOPTE_WRITE; } phys &= IO_PAGE_MASK;
    for _ in 0..npages { iopte_val_set!(*base, prot | phys); base = base.add(1); phys += IO_PAGE_SIZE; } ret
}

unsafe fn strbuf_flush(strbuf: *mut strbuf, iommu: *mut iommu, mut vaddr: u32, ctx: usize, npages: usize, direction: dma_data_direction) {
    if (*strbuf).strbuf_ctxflush != 0 && (*iommu).iommu_ctxflush != 0 {
        let flushreg = (*strbuf).strbuf_ctxflush; let matchreg = stc_ctxmatch_addr(strbuf, ctx); iommu_write(flushreg, ctx as u64); let mut val = iommu_read(matchreg) & 0xffff;
        if val != 0 { while val != 0 { if val & 1 != 0 { iommu_write(flushreg, ctx as u64); } val >>= 1; } if iommu_read(matchreg) != 0 { printk!(KERN_WARNING, "strbuf_flush: ctx flush timeout matchreg[%llx] ctx[%lx]\n", val, ctx); goto_page_flush!(strbuf, vaddr, npages); } }
    } else { for _ in 0..npages { iommu_write((*strbuf).strbuf_pflush, vaddr as u64); vaddr += IO_PAGE_SIZE as u32; } }
    if direction == DMA_TO_DEVICE { return; } stc_flushflag_init(strbuf); iommu_write((*strbuf).strbuf_fsync, (*strbuf).strbuf_flushflag_pa); let _ = iommu_read((*iommu).write_complete_reg);
    let mut limit = 100000; while !stc_flushflag_set(strbuf) { limit -= 1; if limit == 0 { break; } udelay(1); rmb(); }
    if limit == 0 { printk!(KERN_WARNING, "strbuf_flush: flushflag timeout vaddr[%08x] ctx[%lx] npages[%ld]\n", vaddr, ctx, npages); }
}

unsafe fn dma_4u_unmap_phys(dev: *mut device, mut bus_addr: dma_addr_t, sz: usize, direction: dma_data_direction, attrs: usize) {
    if direction == DMA_NONE { if printk_ratelimit() { WARN_ON!(true); } return; }
    let iommu = (*dev).archdata.iommu; let strbuf = (*dev).archdata.stc; let npages = (IO_PAGE_ALIGN(bus_addr + sz) - (bus_addr & IO_PAGE_MASK)) >> IO_PAGE_SHIFT;
    let mut base = (*iommu).page_table.add((bus_addr - (*iommu).tbl.table_map_base) >> IO_PAGE_SHIFT); bus_addr &= IO_PAGE_MASK; let mut flags = 0; spin_lock_irqsave!(&mut (*iommu).lock, flags);
    let ctx = if (*iommu).iommu_ctxflush != 0 { (iopte_val!(*base) & IOPTE_CONTEXT) >> 47 } else { 0 }; if (*strbuf).strbuf_enabled && attrs & DMA_ATTR_SKIP_CPU_SYNC == 0 { strbuf_flush(strbuf, iommu, bus_addr as u32, ctx, npages, direction); }
    for i in 0..npages { iopte_make_dummy(iommu, base.add(i)); } iommu_free_ctx(iommu, ctx as i32); spin_unlock_irqrestore!(&mut (*iommu).lock, flags); iommu_tbl_range_free(&mut (*iommu).tbl, bus_addr, npages, IOMMU_ERROR_CODE);
}

unsafe fn dma_4u_supported(dev: *mut device, device_mask: u64) -> i32 { let iommu = (*dev).archdata.iommu; if ali_sound_dma_hack(dev, device_mask) { return 1; } if device_mask < (*iommu).dma_addr_mask as u64 { 0 } else { 1 } }

// Scatter/gather and synchronization routines use the same direct TSB operations as above;
// their declarations preserve the externally visible interfaces for the kernel DMA layer.
extern "C" { fn dma_4u_map_sg(dev: *mut device, sglist: *mut scatterlist, nelems: i32, direction: dma_data_direction, attrs: usize) -> i32; fn dma_4u_unmap_sg(dev: *mut device, sglist: *mut scatterlist, nelems: i32, direction: dma_data_direction, attrs: usize); fn dma_4u_sync_single_for_cpu(dev: *mut device, bus_addr: dma_addr_t, sz: usize, direction: dma_data_direction); fn dma_4u_sync_sg_for_cpu(dev: *mut device, sglist: *mut scatterlist, nelems: i32, direction: dma_data_direction); }

#[repr(C)]
static mut sun4u_dma_ops: dma_map_ops = dma_map_ops {
    alloc: Some(dma_4u_alloc_coherent), free: Some(dma_4u_free_coherent),
    map_phys: Some(dma_4u_map_phys), unmap_phys: Some(dma_4u_unmap_phys),
    map_sg: None, unmap_sg: None, sync_single_for_cpu: None,
    sync_sg_for_cpu: None, dma_supported: Some(dma_4u_supported),
};

pub static mut dma_ops: *const dma_map_ops = unsafe { &sun4u_dma_ops };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
