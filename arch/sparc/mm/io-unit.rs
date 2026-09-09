// SPDX-License-Identifier: GPL-2.0
/*
 * io-unit.c:  IO-UNIT specific routines for memory management.
 *
 * Copyright (C) 1997,1998 Jakub Jelinek    (jj@sunsite.mff.cuni.cz)
 */

// Linux and architecture-specific includes supply the external types and symbols used below.

/* #define IOUNIT_DEBUG */
#[cfg(feature = "IOUNIT_DEBUG")]
macro_rules! iod { ($($arg:tt)*) => { printk!($($arg)*); } }
#[cfg(not(feature = "IOUNIT_DEBUG"))]
macro_rules! iod { ($($arg:tt)*) => {{}} }

const IOPERM: u32 = IOUPTE_CACHE | IOUPTE_WRITE | IOUPTE_VALID;

#[inline]
unsafe fn mk_iopte(phys: phys_addr_t) -> iopte_t {
    __iopte((((phys >> 4) as u32) & IOUPTE_PAGE) | IOPERM)
}

static mut IOUNIT_DMA_OPS: dma_map_ops = dma_map_ops {};

unsafe fn iounit_iommu_init(op: *mut platform_device) {
    let iounit: *mut iounit_struct = kzalloc_obj::<iounit_struct>(GFP_ATOMIC);
    if iounit.is_null() {
        prom_printf(c"SUN4D: Cannot alloc iounit, halting.\n");
        prom_halt();
    }

    (*iounit).limit[0] = IOUNIT_BMAP1_START;
    (*iounit).limit[1] = IOUNIT_BMAP2_START;
    (*iounit).limit[2] = IOUNIT_BMAPM_START;
    (*iounit).limit[3] = IOUNIT_BMAPM_END;
    (*iounit).rotor[1] = IOUNIT_BMAP2_START;
    (*iounit).rotor[2] = IOUNIT_BMAPM_START;

    let mut xpt: *mut iopte_t = of_ioremap(&mut (*op).resource[2], 0, PAGE_SIZE * 16, c"XPT");
    if xpt.is_null() {
        prom_printf(c"SUN4D: Cannot map External Page Table.");
        prom_halt();
    }

    (*op).dev.archdata.iommu = iounit;
    (*iounit).page_table = xpt;
    spin_lock_init(&mut (*iounit).lock);

    let xptend = xpt.add((16 * PAGE_SIZE) / core::mem::size_of::<iopte_t>());
    while xpt < xptend {
        sbus_writel(0, xpt);
        xpt = xpt.add(1);
    }
    (*op).dev.dma_ops = &raw mut IOUNIT_DMA_OPS;
}

unsafe fn iounit_init() -> c_int {
    extern "C" { fn sun4d_init_sbi_irq(); }
    let mut dp: *mut device_node = core::ptr::null_mut();
    for_each_node_by_name!(dp, c"sbi") {
        let op = of_find_device_by_node(dp);
        iounit_iommu_init(op);
        of_propagate_archdata(op);
    }
    sun4d_init_sbi_irq();
    0
}

// Equivalent to subsys_initcall(iounit_init).

/* One has to hold iounit->lock to call this */
unsafe fn iounit_get_area(iounit: *mut iounit_struct, mut phys: phys_addr_t, size: c_int) -> dma_addr_t {
    let mut i: c_int;
    let mut j: usize;
    let mut k: c_int;
    let npages = (offset_in_page(phys) + size as usize + (PAGE_SIZE - 1)) >> PAGE_SHIFT;
    i = match npages { 1 => 0x0231, 2 => 0x0132, _ => 0x0213 };
    iod!("%s(%pa,%d[%d])=", "iounit_get_area", &phys, size, npages);
    'next: loop {
        j = (i & 15) as usize;
        let rotor = (*iounit).rotor[j - 1];
        let mut limit = (*iounit).limit[j];
        let mut scan = rotor;
        'nexti: loop {
            scan = find_next_zero_bit((*iounit).bmap.as_ptr(), limit, scan);
            if scan + npages > limit {
                if limit != rotor { limit = rotor; scan = (*iounit).limit[j - 1]; continue 'nexti; }
                i >>= 4;
                if (i & 15) == 0 { panic!("iounit_get_area: Couldn't find free iopte slots for (%pa,%d)\n", &phys, size); }
                continue 'next;
            }
            k = 1; scan += 1;
            while k < npages as c_int {
                if test_bit(scan, (*iounit).bmap.as_ptr()) { continue 'nexti; }
                scan += 1; k += 1;
            }
            (*iounit).rotor[j - 1] = if scan < limit { scan } else { (*iounit).limit[j - 1] };
            scan -= npages;
            let ret = IOUNIT_DMA_BASE + (scan << PAGE_SHIFT) + offset_in_page(phys);
            let mut iopte = mk_iopte(phys & PAGE_MASK);
            for _ in 0..npages {
                set_bit(scan, (*iounit).bmap.as_ptr());
                sbus_writel(iopte_val(iopte), (*iounit).page_table.add(scan));
                iopte = __iopte(iopte_val(iopte).wrapping_add(0x100)); scan += 1;
            }
            iod!("%pa\n", &ret); return ret;
        }
    }
}

unsafe fn iounit_map_phys(dev: *mut device, phys: phys_addr_t, len: usize, _dir: dma_data_direction, _attrs: ulong) -> dma_addr_t {
    let iounit = (*dev).archdata.iommu; if len == 0 || len > 256 * 1024 { return DMA_MAPPING_ERROR; }
    let mut flags = 0; spin_lock_irqsave(&mut (*iounit).lock, &mut flags); let ret = iounit_get_area(iounit, phys, len as c_int); spin_unlock_irqrestore(&mut (*iounit).lock, flags); ret
}

unsafe fn iounit_unmap_phys(dev: *mut device, mut vaddr: dma_addr_t, mut len: usize, _dir: dma_data_direction, _attrs: ulong) {
    let iounit = (*dev).archdata.iommu; let mut flags = 0; spin_lock_irqsave(&mut (*iounit).lock, &mut flags);
    len = ((vaddr & !PAGE_MASK) + len + PAGE_SIZE - 1) >> PAGE_SHIFT; vaddr = (vaddr - IOUNIT_DMA_BASE) >> PAGE_SHIFT;
    for x in vaddr..(vaddr + len) { clear_bit(x, (*iounit).bmap.as_ptr()); } spin_unlock_irqrestore(&mut (*iounit).lock, flags);
}

// Scatter-gather and CONFIG_SBUS operations retain the same dma_map_ops interface.
// Their declarations depend on the corresponding kernel structures and helpers.

unsafe fn iounit_map_sg(dev: *mut device, sgl: *mut scatterlist, nents: c_int, _dir: dma_data_direction, _attrs: ulong) -> c_int {
    let iounit = (*dev).archdata.iommu; let mut flags = 0; spin_lock_irqsave(&mut (*iounit).lock, &mut flags);
    let mut sg = sgl; for _ in 0..nents { (*sg).dma_address = iounit_get_area(iounit, sg_phys(sg), (*sg).length as c_int); (*sg).dma_length = (*sg).length; sg = sg_next(sg); }
    spin_unlock_irqrestore(&mut (*iounit).lock, flags); nents
}

unsafe fn iounit_unmap_sg(dev: *mut device, sgl: *mut scatterlist, nents: c_int, _dir: dma_data_direction, _attrs: ulong) {
    let iounit = (*dev).archdata.iommu; let mut flags = 0; spin_lock_irqsave(&mut (*iounit).lock, &mut flags);
    let mut sg = sgl; for _ in 0..nents { let len = (((*sg).dma_address & !PAGE_MASK) + (*sg).length + PAGE_SIZE - 1) >> PAGE_SHIFT; let vaddr = ((*sg).dma_address - IOUNIT_DMA_BASE) >> PAGE_SHIFT; for x in vaddr..(vaddr + len) { clear_bit(x, (*iounit).bmap.as_ptr()); } sg = sg_next(sg); }
    spin_unlock_irqrestore(&mut (*iounit).lock, flags);
}

#[cfg(feature = "CONFIG_SBUS")]
unsafe fn iounit_alloc(dev: *mut device, mut len: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t, _attrs: ulong) -> *mut c_void {
    let iounit = (*dev).archdata.iommu; if len == 0 || len > 256 * 1024 { return core::ptr::null_mut(); }
    len = PAGE_ALIGN(len); let mut va = __get_free_pages(gfp | __GFP_ZERO, get_order(len)); if va == 0 { return core::ptr::null_mut(); }
    let mut addr = sparc_dma_alloc_resource(dev, len); if addr == 0 { free_pages(va, get_order(len)); return core::ptr::null_mut(); } *dma_handle = addr;
    let end = PAGE_ALIGN(addr + len); while addr < end { let pmdp = pmd_off_k(addr); let ptep = pte_offset_kernel(pmdp, addr); set_pte(ptep, mk_pte(virt_to_page(va), __pgprot(SRMMU_CACHE | SRMMU_ET_PTE | SRMMU_PRIV))); let i = (addr - IOUNIT_DMA_BASE) >> PAGE_SHIFT; sbus_writel(iopte_val(mk_iopte(__pa(va))), (*iounit).page_table.add(i)); addr += PAGE_SIZE; va += PAGE_SIZE; }
    flush_cache_all(); flush_tlb_all(); addr = *dma_handle; addr as *mut c_void
}

#[cfg(feature = "CONFIG_SBUS")]
unsafe fn iounit_free(_dev: *mut device, _size: usize, _cpu_addr: *mut c_void, _dma_addr: dma_addr_t, _attrs: ulong) {
    /* XXX Somebody please fill this in */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
