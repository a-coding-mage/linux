// SPDX-License-Identifier: GPL-2.0
/*
 * Mips Jazz DMA controller support
 * Copyright (C) 1995, 1996 by Andreas Busse
 *
 * NOTE: Some of the argument checking could be removed when
 * things have settled down. Also, instead of returning 0xffffffff
 * on failure of vdma_alloc() one could leave page #0 unused
 * and return the more usual NULL pointer as logical address.
 */

/* Kernel and architecture dependencies are supplied externally. */

const CONF_DEBUG_VDMA: i32 = 0;

static mut pgtbl: *mut VDMA_PGTBL_ENTRY = core::ptr::null_mut();
static mut vdma_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK {};
static mut debuglvl: i32 = 3;

#[inline]
unsafe fn vdma_pgtbl_init() {
    let mut paddr: libc::c_ulong = 0;
    let mut i: i32 = 0;
    while i < VDMA_PGTBL_ENTRIES {
        (*pgtbl.add(i as usize)).frame = paddr;
        (*pgtbl.add(i as usize)).owner = VDMA_PAGE_EMPTY;
        paddr = paddr.wrapping_add(VDMA_PAGESIZE);
        i += 1;
    }
}

unsafe fn vdma_init() -> i32 {
    pgtbl = __get_free_pages(GFP_KERNEL | GFP_DMA, get_order(VDMA_PGTBL_SIZE)) as *mut VDMA_PGTBL_ENTRY;
    BUG_ON(pgtbl.is_null());
    dma_cache_wback_inv(pgtbl as libc::c_ulong, VDMA_PGTBL_SIZE);
    pgtbl = CKSEG1ADDR(pgtbl as libc::c_ulong) as *mut VDMA_PGTBL_ENTRY;
    vdma_pgtbl_init();
    r4030_write_reg32(JAZZ_R4030_TRSTBL_BASE, CPHYSADDR(pgtbl as libc::c_ulong));
    r4030_write_reg32(JAZZ_R4030_TRSTBL_LIM, VDMA_PGTBL_SIZE);
    r4030_write_reg32(JAZZ_R4030_TRSTBL_INV, 0);
    printk(KERN_INFO "VDMA: R4030 DMA pagetables initialized.\n");
    0
}

pub unsafe fn vdma_alloc(paddr: libc::c_ulong, size: libc::c_ulong) -> libc::c_ulong {
    if paddr > 0x1fffffff { if debuglvl != 0 { printk("vdma_alloc: Invalid physical address: %08lx\n", paddr); } return DMA_MAPPING_ERROR; }
    if size > 0x400000 || size == 0 { if debuglvl != 0 { printk("vdma_alloc: Invalid size: %08lx\n", size); } return DMA_MAPPING_ERROR; }
    let mut flags: libc::c_ulong = 0;
    spin_lock_irqsave(&raw mut vdma_lock, &mut flags);
    let pages = (VDMA_PAGE(paddr + size) - VDMA_PAGE(paddr) + 1) as i32;
    let mut first: i32 = 0;
    let (last, laddr);
    loop {
        while first < VDMA_PGTBL_ENTRIES && (*pgtbl.add(first as usize)).owner != VDMA_PAGE_EMPTY { first += 1; }
        if first + pages > VDMA_PGTBL_ENTRIES { spin_unlock_irqrestore(&raw mut vdma_lock, flags); return DMA_MAPPING_ERROR; }
        let mut last0 = first + 1;
        while last0 - first < pages && (*pgtbl.add(last0 as usize)).owner == VDMA_PAGE_EMPTY { last0 += 1; }
        if last0 - first == pages { last = last0; break; }
        first = last0 + 1;
    }
    laddr = ((first as libc::c_ulong) << 12) + (paddr & (VDMA_PAGESIZE - 1));
    let mut frame = paddr & !(VDMA_PAGESIZE - 1);
    let mut i = first;
    while i < last { (*pgtbl.add(i as usize)).frame = frame; (*pgtbl.add(i as usize)).owner = laddr; frame += VDMA_PAGESIZE; i += 1; }
    r4030_write_reg32(JAZZ_R4030_TRSTBL_INV, 0);
    if debuglvl > 1 { printk("vdma_alloc: Allocated %d pages starting from %08lx\n", pages, laddr); }
    if debuglvl > 2 {
        printk("LADDR: "); i = first; while i < last { printk("%08x ", i << 12); i += 1; }
        printk("\nPADDR: "); i = first; while i < last { printk("%08x ", (*pgtbl.add(i as usize)).frame); i += 1; }
        printk("\nOWNER: "); i = first; while i < last { printk("%08x ", (*pgtbl.add(i as usize)).owner); i += 1; } printk("\n");
    }
    spin_unlock_irqrestore(&raw mut vdma_lock, flags); laddr
}

pub unsafe fn vdma_free(laddr: libc::c_ulong) -> i32 {
    let mut i = (laddr >> 12) as i32;
    if (*pgtbl.add(i as usize)).owner != laddr { printk("vdma_free: trying to free other's dma pages, laddr=%8lx\n", laddr); return -1; }
    while i < VDMA_PGTBL_ENTRIES && (*pgtbl.add(i as usize)).owner == laddr { (*pgtbl.add(i as usize)).owner = VDMA_PAGE_EMPTY; i += 1; }
    if debuglvl > 1 { printk("vdma_free: freed %ld pages starting from %08lx\n", i - ((laddr >> 12) as i32), laddr); } 0
}

pub unsafe fn vdma_phys2log(paddr: libc::c_ulong) -> libc::c_ulong {
    let frame = paddr & !(VDMA_PAGESIZE - 1); let mut i = 0;
    while i < VDMA_PGTBL_ENTRIES && (*pgtbl.add(i as usize)).frame != frame { i += 1; }
    if i == VDMA_PGTBL_ENTRIES { return !0; } ((i as libc::c_ulong) << 12) + (paddr & (VDMA_PAGESIZE - 1))
}
pub unsafe fn vdma_log2phys(laddr: libc::c_ulong) -> libc::c_ulong { (*pgtbl.add((laddr >> 12) as usize)).frame + (laddr & (VDMA_PAGESIZE - 1)) }

pub unsafe fn vdma_stats() { printk("vdma_stats: CONFIG: %08x\n", r4030_read_reg32(JAZZ_R4030_CONFIG)); printk("R4030 translation table base: %08x\n", r4030_read_reg32(JAZZ_R4030_TRSTBL_BASE)); printk("R4030 translation table limit: %08x\n", r4030_read_reg32(JAZZ_R4030_TRSTBL_LIM)); printk("vdma_stats: INV_ADDR: %08x\n", r4030_read_reg32(JAZZ_R4030_INV_ADDR)); printk("vdma_stats: R_FAIL_ADDR: %08x\n", r4030_read_reg32(JAZZ_R4030_R_FAIL_ADDR)); printk("vdma_stats: M_FAIL_ADDR: %08x\n", r4030_read_reg32(JAZZ_R4030_M_FAIL_ADDR)); printk("vdma_stats: IRQ_SOURCE: %08x\n", r4030_read_reg32(JAZZ_R4030_IRQ_SOURCE)); printk("vdma_stats: I386_ERROR: %08x\n", r4030_read_reg32(JAZZ_R4030_I386_ERROR)); let mut i=0; printk("vdma_chnl_modes:   "); while i<8 { printk("%04x ", r4030_read_reg32(JAZZ_R4030_CHNL_MODE+(i<<5)) as libc::c_uint); i+=1; } printk("\nvdma_chnl_enables: "); i=0; while i<8 { printk("%04x ", r4030_read_reg32(JAZZ_R4030_CHNL_ENABLE+(i<<5)) as libc::c_uint); i+=1; } printk("\n"); }

pub unsafe fn vdma_enable(channel: i32) { if debuglvl != 0 { printk("vdma_enable: channel %d\n", channel); } let a=JAZZ_R4030_CHNL_ENABLE+(channel<<5); let s=r4030_read_reg32(a); if s&0x400!=0 { printk("VDMA: Channel %d: Address error!\n",channel); } if s&0x200!=0 { printk("VDMA: Channel %d: Memory error!\n",channel); } r4030_write_reg32(a,r4030_read_reg32(a)|R4030_TC_INTR|R4030_MEM_INTR|R4030_ADDR_INTR); r4030_write_reg32(a,r4030_read_reg32(a)|R4030_CHNL_ENABLE); }
pub unsafe fn vdma_disable(channel: i32) { let a=JAZZ_R4030_CHNL_ENABLE+(channel<<5); if debuglvl!=0 { let s=r4030_read_reg32(a); printk("vdma_disable: channel %d\n",channel); printk("VDMA: channel %d status: %04x (%s) mode: %02x addr: %06x count: %06x\n",channel,s,if s&0x600!=0{"ERROR"}else{"OK"},r4030_read_reg32(JAZZ_R4030_CHNL_MODE+(channel<<5)) as libc::c_uint,r4030_read_reg32(JAZZ_R4030_CHNL_ADDR+(channel<<5)) as libc::c_uint,r4030_read_reg32(JAZZ_R4030_CHNL_COUNT+(channel<<5)) as libc::c_uint); } r4030_write_reg32(a,r4030_read_reg32(a)&!R4030_CHNL_ENABLE); core::ptr::read_volatile(JAZZ_DUMMY_DEVICE as *const u32); }
pub unsafe fn vdma_set_mode(channel:i32, mode:i32) { if debuglvl!=0 { printk("vdma_set_mode: channel %d, mode 0x%x\n",channel,mode); } match channel { JAZZ_SCSI_DMA=>r4030_write_reg32(JAZZ_R4030_CHNL_MODE+(channel<<5),R4030_MODE_INTR_EN|R4030_MODE_WIDTH_16|R4030_MODE_ATIME_80), JAZZ_FLOPPY_DMA=>r4030_write_reg32(JAZZ_R4030_CHNL_MODE+(channel<<5),R4030_MODE_INTR_EN|R4030_MODE_WIDTH_8|R4030_MODE_ATIME_120), JAZZ_AUDIOL_DMA|JAZZ_AUDIOR_DMA=>printk("VDMA: Audio DMA not supported yet.\n"), _=>printk("VDMA: vdma_set_mode() called with unsupported channel %d!\n",channel) }; let a=JAZZ_R4030_CHNL_ENABLE+(channel<<5); match mode { DMA_MODE_READ=>r4030_write_reg32(a,r4030_read_reg32(a)&!R4030_CHNL_WRITE), DMA_MODE_WRITE=>r4030_write_reg32(a,r4030_read_reg32(a)|R4030_CHNL_WRITE), _=>printk("VDMA: vdma_set_mode() called with unknown dma mode 0x%x\n",mode) } }
pub unsafe fn vdma_set_addr(channel:i32, addr:i64) { if debuglvl!=0 { printk("vdma_set_addr: channel %d, addr %lx\n",channel,addr); } r4030_write_reg32(JAZZ_R4030_CHNL_ADDR+(channel<<5),addr as u32); }
pub unsafe fn vdma_set_count(channel:i32,count:i32) { if debuglvl!=0 { printk("vdma_set_count: channel %d, count %08x\n",channel,count as u32); } r4030_write_reg32(JAZZ_R4030_CHNL_COUNT+(channel<<5),count as u32); }
pub unsafe fn vdma_get_residue(channel:i32)->i32 { let r=r4030_read_reg32(JAZZ_R4030_CHNL_COUNT+(channel<<5)); if debuglvl!=0 { printk("vdma_get_residual: channel %d: residual=%d\n",channel,r); } r as i32 }
pub unsafe fn vdma_get_enable(channel:i32)->i32 { let e=r4030_read_reg32(JAZZ_R4030_CHNL_ENABLE+(channel<<5)); if debuglvl!=0 { printk("vdma_get_enable: channel %d: enable=%d\n",channel,e); } e as i32 }

unsafe fn jazz_dma_alloc(dev: *mut device, mut size: usize, dma_handle: *mut dma_addr_t, mut gfp: gfp_t, attrs: libc::c_ulong) -> *mut libc::c_void { if attrs & DMA_ATTR_NO_WARN != 0 { gfp |= __GFP_NOWARN; } size=PAGE_ALIGN(size); let page=alloc_pages(gfp,get_order(size)); if page.is_null(){return core::ptr::null_mut();} let ret=page_address(page); memset(ret,0,size); *dma_handle=vdma_alloc(virt_to_phys(ret),size as libc::c_ulong); if *dma_handle==DMA_MAPPING_ERROR { __free_pages(page,get_order(size)); return core::ptr::null_mut(); } arch_dma_prep_coherent(page,size); (UNCAC_BASE+__pa(ret)) as *mut libc::c_void }
unsafe fn jazz_dma_free(dev:*mut device,size:usize,vaddr:*mut libc::c_void,dma_handle:dma_addr_t,attrs:libc::c_ulong){vdma_free(dma_handle);__free_pages(virt_to_page(vaddr),get_order(size));}
unsafe fn jazz_dma_map_phys(dev:*mut device,phys:phys_addr_t,size:usize,dir:dma_data_direction,attrs:libc::c_ulong)->dma_addr_t{if attrs&DMA_ATTR_MMIO!=0{return DMA_MAPPING_ERROR;}if attrs&DMA_ATTR_SKIP_CPU_SYNC==0{arch_sync_dma_for_device(phys,size,dir);}vdma_alloc(phys,size as libc::c_ulong)}
unsafe fn jazz_dma_unmap_phys(dev:*mut device,dma_addr:dma_addr_t,size:usize,dir:dma_data_direction,attrs:libc::c_ulong){if attrs&DMA_ATTR_SKIP_CPU_SYNC==0{arch_sync_dma_for_cpu(vdma_log2phys(dma_addr),size,dir);}vdma_free(dma_addr);}
unsafe fn jazz_dma_map_sg(dev:*mut device,sglist:*mut scatterlist,nents:i32,dir:dma_data_direction,attrs:libc::c_ulong)->i32{let mut i=0;let mut sg=sglist;while i<nents{if attrs&DMA_ATTR_SKIP_CPU_SYNC==0{arch_sync_dma_for_device(sg_phys(sg),(*sg).length,dir);}(*sg).dma_address=vdma_alloc(sg_phys(sg),(*sg).length as libc::c_ulong);if (*sg).dma_address==DMA_MAPPING_ERROR{return -EIO;}sg_dma_len(sg)=(*sg).length;i+=1;sg=sg_next(sg);}nents}
unsafe fn jazz_dma_unmap_sg(dev:*mut device,sglist:*mut scatterlist,nents:i32,dir:dma_data_direction,attrs:libc::c_ulong){let mut i=0;let mut sg=sglist;while i<nents{if attrs&DMA_ATTR_SKIP_CPU_SYNC==0{arch_sync_dma_for_cpu(sg_phys(sg),(*sg).length,dir);}vdma_free((*sg).dma_address);i+=1;sg=sg_next(sg);}}
unsafe fn jazz_dma_sync_single_for_device(dev:*mut device,addr:dma_addr_t,size:usize,dir:dma_data_direction){arch_sync_dma_for_device(vdma_log2phys(addr),size,dir)}
unsafe fn jazz_dma_sync_single_for_cpu(dev:*mut device,addr:dma_addr_t,size:usize,dir:dma_data_direction){arch_sync_dma_for_cpu(vdma_log2phys(addr),size,dir)}
unsafe fn jazz_dma_sync_sg_for_device(dev:*mut device,sgl:*mut scatterlist,nents:i32,dir:dma_data_direction){let mut i=0;let mut sg=sgl;while i<nents{arch_sync_dma_for_device(sg_phys(sg),(*sg).length,dir);i+=1;sg=sg_next(sg);}}
unsafe fn jazz_dma_sync_sg_for_cpu(dev:*mut device,sgl:*mut scatterlist,nents:i32,dir:dma_data_direction){let mut i=0;let mut sg=sgl;while i<nents{arch_sync_dma_for_cpu(sg_phys(sg),(*sg).length,dir);i+=1;sg=sg_next(sg);}}

/* DMA map operations and their external kernel dependencies. */
pub static jazz_dma_ops: dma_map_ops = dma_map_ops { alloc: Some(jazz_dma_alloc), free: Some(jazz_dma_free), map_phys: Some(jazz_dma_map_phys), unmap_phys: Some(jazz_dma_unmap_phys), map_sg: Some(jazz_dma_map_sg), unmap_sg: Some(jazz_dma_unmap_sg), sync_single_for_cpu: Some(jazz_dma_sync_single_for_cpu), sync_single_for_device: Some(jazz_dma_sync_single_for_device), sync_sg_for_cpu: Some(jazz_dma_sync_sg_for_cpu), sync_sg_for_device: Some(jazz_dma_sync_sg_for_device), mmap: Some(dma_common_mmap), get_sgtable: Some(dma_common_get_sgtable), alloc_pages_op: Some(dma_common_alloc_pages), free_pages: Some(dma_common_free_pages) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
