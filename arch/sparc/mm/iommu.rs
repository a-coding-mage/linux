// SPDX-License-Identifier: GPL-2.0
/*
 * iommu.c:  IOMMU specific routines for memory management.
 *
 * Copyright (C) 1995 David S. Miller  (davem@caip.rutgers.edu)
 * Copyright (C) 1995,2002 Pete Zaitcev     (zaitcev@yahoo.com)
 * Copyright (C) 1996 Eddie C. Dost    (ecd@skynet.be)
 * Copyright (C) 1997,1998 Jakub Jelinek    (jj@sunsite.mff.cuni.cz)
 */

// Kernel and architecture dependencies supplied externally.

const IOMMU_RNGE: u32 = IOMMU_RNGE_256MB;
const IOMMU_START: usize = 0xF0000000;
const IOMMU_WINSIZE: usize = 256 * 1024 * 1024;
const IOMMU_NPTES: usize = IOMMU_WINSIZE / PAGE_SIZE; // 64K PTEs, 256KB
const IOMMU_ORDER: u32 = 6; // 4096 * (1<<6)

static mut viking_flush: i32 = 0;
extern "C" {
    fn viking_flush_page(page: usize);
    fn viking_mxcc_flush_page(page: usize);
}

static mut ioperm_noc: u32 = 0;
static mut dvma_prot: pgprot_t = pgprot_t::default();

const IOPERM: u32 = IOPTE_CACHE | IOPTE_WRITE | IOPTE_VALID;

#[inline]
unsafe fn MKIOPTE(pfn: usize, perm: u32) -> u32 {
    ((((pfn << 8) & IOPTE_PAGE) | perm) & !IOPTE_WAZ) as u32
}

static sbus_iommu_dma_gflush_ops: dma_map_ops = dma_map_ops {
    #[cfg(CONFIG_SBUS)]
    alloc: Some(sbus_iommu_alloc),
    #[cfg(CONFIG_SBUS)]
    free: Some(sbus_iommu_free),
    map_phys: Some(sbus_iommu_map_phys_gflush),
    unmap_phys: Some(sbus_iommu_unmap_phys),
    map_sg: Some(sbus_iommu_map_sg_gflush),
    unmap_sg: Some(sbus_iommu_unmap_sg),
};

static sbus_iommu_dma_pflush_ops: dma_map_ops = dma_map_ops {
    #[cfg(CONFIG_SBUS)]
    alloc: Some(sbus_iommu_alloc),
    #[cfg(CONFIG_SBUS)]
    free: Some(sbus_iommu_free),
    map_phys: Some(sbus_iommu_map_phys_pflush),
    unmap_phys: Some(sbus_iommu_unmap_phys),
    map_sg: Some(sbus_iommu_map_sg_pflush),
    unmap_sg: Some(sbus_iommu_unmap_sg),
};

unsafe fn sbus_iommu_init(op: *mut platform_device) {
    let iommu = kmalloc_obj::<iommu_struct>();
    if iommu.is_null() { prom_printf("Unable to allocate iommu structure\n"); prom_halt(); }
    (*iommu).regs = of_ioremap(&(*op).resource[0], 0, PAGE_SIZE * 3, "iommu_regs");
    if (*iommu).regs.is_null() { prom_printf("Cannot map IOMMU registers\n"); prom_halt(); }

    let mut control = sbus_readl(&(*(*iommu).regs).control);
    let impl_ = (control & IOMMU_CTRL_IMPL) >> 28;
    let vers = (control & IOMMU_CTRL_VERS) >> 24;
    control &= !IOMMU_CTRL_RNGE;
    control |= IOMMU_RNGE_256MB | IOMMU_CTRL_ENAB;
    sbus_writel(control, &mut (*(*iommu).regs).control);
    iommu_invalidate((*iommu).regs);
    (*iommu).start = IOMMU_START;
    (*iommu).end = 0xffffffff;

    let tmp = __get_free_pages(GFP_KERNEL, IOMMU_ORDER);
    if tmp == 0 { prom_printf("Unable to allocate iommu table [0x%lx]\n", IOMMU_NPTES * size_of::<iopte_t>()); prom_halt(); }
    (*iommu).page_table = tmp as *mut iopte_t;
    memset((*iommu).page_table as *mut _, 0, IOMMU_NPTES * size_of::<iopte_t>());
    flush_cache_all(); flush_tlb_all();
    let base = __pa((*iommu).page_table as usize) >> 4;
    sbus_writel(base as u32, &mut (*(*iommu).regs).base);
    iommu_invalidate((*iommu).regs);
    let bitmap = kmalloc(IOMMU_NPTES >> 3, GFP_KERNEL);
    if bitmap.is_null() { prom_printf("Unable to allocate iommu bitmap [%d]\n", (IOMMU_NPTES >> 3) as i32); prom_halt(); }
    bit_map_init(&mut (*iommu).usemap, bitmap as *mut _, IOMMU_NPTES);
    (*iommu).usemap.num_colors = if srmmu_modtype == HyperSparc { vac_cache_size >> PAGE_SHIFT } else { 1 };
    printk!(KERN_INFO, "IOMMU: impl %d vers %d table 0x%p[%d B] map [%d b]\n", impl_, vers, (*iommu).page_table, IOMMU_NPTES * size_of::<iopte_t>(), IOMMU_NPTES);
    (*op).dev.archdata.iommu = iommu;
    (*op).dev.dma_ops = if flush_page_for_dma_global { &sbus_iommu_dma_gflush_ops } else { &sbus_iommu_dma_pflush_ops };
}

unsafe fn iommu_init() -> i32 {
    for_each_node_by_name!(dp, "iommu") {
        let op = of_find_device_by_node(dp);
        sbus_iommu_init(op);
        of_propagate_archdata(op);
    }
    0
}

subsys_initcall!(iommu_init);

unsafe fn iommu_flush_iotlb(iopte: *mut iopte_t, niopte: u32) {
    let mut start = iopte as usize;
    let end = PAGE_ALIGN(start + niopte as usize * size_of::<iopte_t>());
    start &= PAGE_MASK;
    while start < end {
        if viking_mxcc_present { viking_mxcc_flush_page(start); }
        else if viking_flush != 0 { viking_flush_page(start); }
        else { __flush_page_to_ram(start); }
        start += PAGE_SIZE;
    }
}

unsafe fn __sbus_iommu_map_phys(dev: *mut device, paddr: phys_addr_t, len: usize, per_page_flush: bool, attrs: usize) -> dma_addr_t {
    let iommu = (*dev).archdata.iommu;
    let off = offset_in_page(paddr);
    let npages = (off + len + PAGE_SIZE - 1) >> PAGE_SHIFT;
    let mut pfn = __phys_to_pfn(paddr);
    if unlikely(attrs & DMA_ATTR_MMIO != 0) || len == 0 || len > 256 * 1024 { return DMA_MAPPING_ERROR; }
    if per_page_flush && !PhysHighMem(paddr) {
        let vaddr = phys_to_virt(paddr);
        let mut p = vaddr & PAGE_MASK;
        while p < vaddr + len { flush_page_for_dma(p); p += PAGE_SIZE; }
    }
    let ioptex = bit_map_string_get(&mut (*iommu).usemap, npages, pfn);
    if ioptex < 0 { panic!("iommu out"); }
    let busa0 = (*iommu).start + ((ioptex as usize) << PAGE_SHIFT);
    let iopte0 = (*iommu).page_table.add(ioptex as usize);
    let mut busa = busa0; let mut iopte = iopte0;
    for _ in 0..npages { iopte_val(*iopte) = MKIOPTE(pfn, IOPERM); iommu_invalidate_page((*iommu).regs, busa); busa += PAGE_SIZE; iopte = iopte.add(1); pfn += 1; }
    iommu_flush_iotlb(iopte0, npages as u32);
    busa0 + off
}

unsafe fn sbus_iommu_map_phys_gflush(dev: *mut device, phys: phys_addr_t, len: usize, _dir: dma_data_direction, attrs: usize) -> dma_addr_t { flush_page_for_dma(0); __sbus_iommu_map_phys(dev, phys, len, false, attrs) }
unsafe fn sbus_iommu_map_phys_pflush(dev: *mut device, phys: phys_addr_t, len: usize, _dir: dma_data_direction, attrs: usize) -> dma_addr_t { __sbus_iommu_map_phys(dev, phys, len, true, attrs) }

unsafe fn __sbus_iommu_map_sg(dev: *mut device, sgl: *mut scatterlist, nents: i32, dir: dma_data_direction, attrs: usize, per_page_flush: bool) -> i32 {
    let mut sg = sgl; for j in 0..nents { let _ = j; (*sg).dma_address = __sbus_iommu_map_phys(dev, sg_phys(sg), (*sg).length, per_page_flush, attrs); if (*sg).dma_address == DMA_MAPPING_ERROR { return -EIO; } (*sg).dma_length = (*sg).length; sg = sg_next(sg); } let _ = dir; nents
}
unsafe fn sbus_iommu_map_sg_gflush(dev: *mut device, sgl: *mut scatterlist, nents: i32, dir: dma_data_direction, attrs: usize) -> i32 { flush_page_for_dma(0); __sbus_iommu_map_sg(dev, sgl, nents, dir, attrs, false) }
unsafe fn sbus_iommu_map_sg_pflush(dev: *mut device, sgl: *mut scatterlist, nents: i32, dir: dma_data_direction, attrs: usize) -> i32 { __sbus_iommu_map_sg(dev, sgl, nents, dir, attrs, true) }

unsafe fn sbus_iommu_unmap_phys(dev: *mut device, dma_addr: dma_addr_t, len: usize, _dir: dma_data_direction, _attrs: usize) {
    let iommu = (*dev).archdata.iommu; let mut busa = dma_addr & PAGE_MASK; let off = dma_addr & !PAGE_MASK; let npages = (off + len + PAGE_SIZE - 1) >> PAGE_SHIFT; let ioptex = (busa - (*iommu).start) >> PAGE_SHIFT;
    BUG_ON(busa < (*iommu).start); for i in 0..npages { iopte_val(*(*iommu).page_table.add(ioptex+i)) = 0; iommu_invalidate_page((*iommu).regs, busa); busa += PAGE_SIZE; } bit_map_clear(&mut (*iommu).usemap, ioptex, npages);
}
unsafe fn sbus_iommu_unmap_sg(dev: *mut device, sgl: *mut scatterlist, nents: i32, dir: dma_data_direction, attrs: usize) { let mut sg=sgl; for _ in 0..nents { sbus_iommu_unmap_phys(dev, (*sg).dma_address, (*sg).length, dir, attrs); (*sg).dma_address=0x21212121; sg=sg_next(sg); } }

#[cfg(CONFIG_SBUS)]
unsafe fn sbus_iommu_alloc(dev: *mut device, len_: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t, attrs: usize) -> *mut core::ffi::c_void {
    let iommu=(*dev).archdata.iommu; if len_==0 || len_>256*1024 { return core::ptr::null_mut(); } let len=PAGE_ALIGN(len_); let va=__get_free_pages(gfp|__GFP_ZERO,get_order(len)); if va==0{return core::ptr::null_mut();} let mut addr=sparc_dma_alloc_resource(dev,len); let ret=addr; if addr==0 {free_pages(va,get_order(len));return core::ptr::null_mut();} BUG_ON((va&!PAGE_MASK)!=0); BUG_ON((addr&!PAGE_MASK)!=0); BUG_ON((len&!PAGE_MASK)!=0); let ioptex=bit_map_string_get(&mut (*iommu).usemap,len>>PAGE_SHIFT,addr>>PAGE_SHIFT); if ioptex<0{panic!("iommu out");} let mut iopte=(*iommu).page_table.add(ioptex as usize); let first=iopte; let end=addr+len; let mut v=va; while addr<end { let page=v; if viking_mxcc_present{viking_mxcc_flush_page(page)}else if viking_flush!=0{viking_flush_page(page)}else{__flush_page_to_ram(page)} let pmdp=pmd_off_k(addr); let ptep=pte_offset_kernel(pmdp,addr); set_pte(ptep,mk_pte(virt_to_page(page),dvma_prot)); iopte_val(*iopte)=MKIOPTE(page_to_pfn(virt_to_page(page)),ioperm_noc);iopte=iopte.add(1);addr+=PAGE_SIZE;v+=PAGE_SIZE;} flush_cache_all();iommu_flush_iotlb(first,(len>>PAGE_SHIFT) as u32);flush_tlb_all();iommu_invalidate((*iommu).regs);*dma_handle=(*iommu).start+((ioptex as usize)<<PAGE_SHIFT);ret as *mut _
}

#[cfg(CONFIG_SBUS)]
unsafe fn sbus_iommu_free(dev:*mut device,len:usize,cpu_addr:*mut core::ffi::c_void,busa:dma_addr_t,_attrs:usize){let iommu=(*dev).archdata.iommu;let page=virt_to_page(cpu_addr);let ioptex=(busa-(*iommu).start)>>PAGE_SHIFT;if sparc_dma_free_resource(cpu_addr,len)==0{return;}BUG_ON((busa&!PAGE_MASK)!=0);BUG_ON((len&!PAGE_MASK)!=0);let mut p=(*iommu).page_table.add(ioptex);let mut b=busa;while b<busa+len{iopte_val(*p)=0;p=p.add(1);b+=PAGE_SIZE;}flush_tlb_all();iommu_invalidate((*iommu).regs);bit_map_clear(&mut (*iommu).usemap,ioptex,len>>PAGE_SHIFT);__free_pages(page,get_order(len));}

pub unsafe fn ld_mmu_iommu() { if viking_mxcc_present || srmmu_modtype == HyperSparc { dvma_prot=__pgprot(SRMMU_CACHE|SRMMU_ET_PTE|SRMMU_PRIV);ioperm_noc=IOPTE_CACHE|IOPTE_WRITE|IOPTE_VALID;}else{dvma_prot=__pgprot(SRMMU_ET_PTE|SRMMU_PRIV);ioperm_noc=IOPTE_WRITE|IOPTE_VALID;} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
