// SPDX-License-Identifier: GPL-2.0
/* PARISC 1.1 Dynamic DMA mapping support. */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

static mut PROC_GSC_ROOT: *mut proc_dir_entry = core::ptr::null_mut();
static mut PCXL_USED_BYTES: usize = 0;
static mut PCXL_USED_PAGES: usize = 0;
pub static mut PCXL_DMA_START: usize = 0;
static mut PCXL_RES_MAP: *mut u8 = core::ptr::null_mut();
static mut PCXL_RES_HINT: usize = 0;
static mut PCXL_RES_SIZE: usize = 0;
static mut PCXL_RES_LOCK: spinlock_t = spinlock_t::new();

#[inline]
unsafe fn dump_resmap() {}

#[inline]
unsafe fn map_pte_uncached(mut pte: *mut pte_t, mut vaddr: usize, size: usize,
                           paddr_ptr: *mut usize) -> i32 {
    let orig_vaddr = vaddr;
    vaddr &= !PMD_MASK;
    let mut end = vaddr.wrapping_add(size);
    if end > PMD_SIZE { end = PMD_SIZE; }
    loop {
        let mut flags: usize = 0;
        if !pte_none(*pte) { printk(KERN_ERR, "map_pte_uncached: page already exists\n"); }
        purge_tlb_start(&mut flags);
        set_pte(pte, __mk_pte(*paddr_ptr, PAGE_KERNEL_UNC));
        pdtlb(SR_KERNEL, orig_vaddr);
        purge_tlb_end(flags);
        vaddr = vaddr.wrapping_add(PAGE_SIZE);
        pte = pte.add(1);
        (*paddr_ptr) = (*paddr_ptr).wrapping_add(PAGE_SIZE);
        if vaddr >= end { break; }
    }
    0
}

#[inline]
unsafe fn map_pmd_uncached(mut pmd: *mut pmd_t, mut vaddr: usize, size: usize,
                           paddr_ptr: *mut usize) -> i32 {
    let mut orig_vaddr = vaddr;
    vaddr &= !PGDIR_MASK;
    let mut end = vaddr.wrapping_add(size);
    if end > PGDIR_SIZE { end = PGDIR_SIZE; }
    loop {
        let pte = pte_alloc_kernel(pmd, vaddr);
        if pte.is_null() { return -12; }
        if map_pte_uncached(pte, orig_vaddr, end - vaddr, paddr_ptr) != 0 { return -12; }
        vaddr = vaddr.wrapping_add(PMD_SIZE) & PMD_MASK;
        orig_vaddr = orig_vaddr.wrapping_add(PMD_SIZE);
        pmd = pmd.add(1);
        if vaddr >= end { break; }
    }
    0
}

#[inline]
unsafe fn map_uncached_pages(mut vaddr: usize, size: usize, mut paddr: usize) -> i32 {
    let mut dir = pgd_offset_k(vaddr);
    let end = vaddr.wrapping_add(size);
    loop {
        let p4d = p4d_offset(dir, vaddr);
        let pud = pud_offset(p4d, vaddr);
        let pmd = pmd_alloc(core::ptr::null_mut(), pud, vaddr);
        if pmd.is_null() { return -12; }
        if map_pmd_uncached(pmd, vaddr, end - vaddr, &mut paddr) != 0 { return -12; }
        vaddr = vaddr.wrapping_add(PGDIR_SIZE);
        dir = dir.add(1);
        if vaddr == 0 || vaddr >= end { break; }
    }
    0
}

#[inline]
unsafe fn unmap_uncached_pte(pmd: *mut pmd_t, mut vaddr: usize, size: usize) {
    if pmd_none(*pmd) { return; }
    if pmd_bad(*pmd) { pmd_ERROR(pmd); pmd_clear(pmd); return; }
    let mut pte = pte_offset_kernel(pmd, vaddr);
    let orig_vaddr = vaddr;
    vaddr &= !PMD_MASK;
    let mut end = vaddr.wrapping_add(size);
    if end > PMD_SIZE { end = PMD_SIZE; }
    loop {
        let page = *pte;
        pte_clear(&mut init_mm, vaddr, pte);
        let mut flags = 0usize;
        purge_tlb_start(&mut flags); pdtlb(SR_KERNEL, orig_vaddr); purge_tlb_end(flags);
        vaddr = vaddr.wrapping_add(PAGE_SIZE); pte = pte.add(1);
        if !(pte_none(page) || pte_present(page)) { printk(KERN_CRIT, "Whee.. Swapped out page in kernel page table\n"); }
        if vaddr >= end { break; }
    }
}

#[inline]
unsafe fn unmap_uncached_pmd(dir: *mut pgd_t, mut vaddr: usize, size: usize) {
    if pgd_none(*dir) { return; }
    if pgd_bad(*dir) { pgd_ERROR(dir); pgd_clear(dir); return; }
    let mut pmd = pmd_offset(pud_offset(p4d_offset(dir, vaddr), vaddr), vaddr);
    let orig_vaddr = vaddr; vaddr &= !PGDIR_MASK;
    let mut end = vaddr.wrapping_add(size); if end > PGDIR_SIZE { end = PGDIR_SIZE; }
    loop {
        unmap_uncached_pte(pmd, orig_vaddr, end - vaddr);
        vaddr = vaddr.wrapping_add(PMD_SIZE) & PMD_MASK; pmd = pmd.add(1);
        if vaddr >= end { break; }
    }
}

unsafe fn unmap_uncached_pages(mut vaddr: usize, size: usize) {
    let mut dir = pgd_offset_k(vaddr); let end = vaddr.wrapping_add(size);
    loop { unmap_uncached_pmd(dir, vaddr, end - vaddr); vaddr = vaddr.wrapping_add(PGDIR_SIZE); dir = dir.add(1); if vaddr == 0 || vaddr >= end { break; } }
}

unsafe fn pcxl_alloc_range(size: usize) -> usize {
    let pages = size >> PAGE_SHIFT;
    let mask = usize::MAX >> (usize::BITS as usize - pages);
    spin_lock_irqsave(&mut PCXL_RES_LOCK);
    let mut idx = 0usize;
    let width = if pages <= 8 { 8 } else if pages <= 16 { 16 } else if pages <= 32 { 32 } else { panic!("pcxl_alloc_range: Too many pages to map") };
    let words = PCXL_RES_SIZE / (width / 8);
    let start = PCXL_RES_HINT & !((width / 8) - 1);
    let mut found = false;
    for pass in 0..2 { let begin = if pass == 0 { start } else { 0 }; for i in begin..words { let p = PCXL_RES_MAP.add(i * (width / 8)) as *mut usize; if *p & mask == 0 { *p |= mask; idx = i * (width / 8); PCXL_RES_HINT = idx + width / 8; found = true; break; } } if found { break; } }
    if !found { panic!("pcxl_alloc_range: out of dma mapping resources"); }
    PCXL_USED_PAGES += pages; PCXL_USED_BYTES += if pages >> 3 != 0 { pages >> 3 } else { 1 };
    spin_unlock_irqrestore(&mut PCXL_RES_LOCK);
    PCXL_DMA_START + (idx << (PAGE_SHIFT + 3))
}

unsafe fn pcxl_free_range(vaddr: usize, size: usize) {
    let idx = (vaddr - PCXL_DMA_START) >> (PAGE_SHIFT + 3); let pages = size >> PAGE_SHIFT;
    let mask = usize::MAX >> (usize::BITS as usize - pages);
    spin_lock_irqsave(&mut PCXL_RES_LOCK);
    let width = if pages <= 8 { 8 } else if pages <= 16 { 16 } else if pages <= 32 { 32 } else { panic!("pcxl_free_range: Too many pages to unmap") };
    let p = PCXL_RES_MAP.add(idx + (((width / 8) - 1) & !((width / 8) - 1))) as *mut usize; *p &= !mask;
    PCXL_USED_PAGES -= if pages != 0 { pages } else { 1 }; PCXL_USED_BYTES -= if pages >> 3 != 0 { pages >> 3 } else { 1 };
    spin_unlock_irqrestore(&mut PCXL_RES_LOCK);
}

pub unsafe fn arch_dma_alloc(_dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t, _attrs: usize) -> *mut c_void {
    if boot_cpu_data.cpu_type != pcxl2 && boot_cpu_data.cpu_type != pcxl { return core::ptr::null_mut(); }
    let order = get_order(size); let size = 1usize << (order + PAGE_SHIFT); let vaddr = pcxl_alloc_range(size);
    let mut paddr = __get_free_pages(gfp | __GFP_ZERO, order); flush_kernel_dcache_range(paddr, size); paddr = __pa(paddr); map_uncached_pages(vaddr, size, paddr); *dma_handle = paddr as dma_addr_t; vaddr as *mut c_void
}

pub unsafe fn arch_dma_free(_dev: *mut device, size: usize, vaddr: *mut c_void, dma_handle: dma_addr_t, _attrs: usize) {
    let order = get_order(size); let size = 1usize << (order + PAGE_SHIFT); unmap_uncached_pages(vaddr as usize, size); pcxl_free_range(vaddr as usize, size); free_pages(__va(dma_handle as usize), order);
}

pub unsafe fn arch_sync_dma_for_device(paddr: phys_addr_t, size: usize, _dir: dma_data_direction) { flush_kernel_dcache_range(phys_to_virt(paddr) as usize, size); }
pub unsafe fn arch_sync_dma_for_cpu(paddr: phys_addr_t, size: usize, dir: dma_data_direction) {
    let addr = phys_to_virt(paddr) as usize;
    match dir { DMA_TO_DEVICE | DMA_BIDIRECTIONAL => flush_kernel_dcache_range(addr, size), DMA_FROM_DEVICE => purge_kernel_dcache_range_asm(addr, addr + size), _ => BUG(), }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
