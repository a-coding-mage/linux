/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

extern "C" {
    fn __pgprot(value: usize) -> pgprot_t;
    fn pte_none(pte: pte_t) -> bool;
    fn set_pte(pte: *mut pte_t, value: pte_t);
    fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t;
    fn pte_alloc_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t;
    fn pgd_offset(mm: *mut mm_struct, address: usize) -> *mut pgd_t;
    fn flush_cache_all();
    fn p4d_alloc(mm: *mut mm_struct, pgd: *mut pgd_t, address: usize) -> *mut p4d_t;
    fn pud_alloc(mm: *mut mm_struct, p4d: *mut p4d_t, address: usize) -> *mut pud_t;
    fn pmd_alloc(mm: *mut mm_struct, pud: *mut pud_t, address: usize) -> *mut pmd_t;
    fn flush_tlb_all();
    fn virt_to_phys(address: *mut core::ffi::c_void) -> usize;
    fn __va(address: usize) -> *mut core::ffi::c_char;
    fn virt_to_page(address: *mut core::ffi::c_char) -> *mut page;
    fn PageReserved(page: *mut page) -> bool;
    fn get_vm_area(size: usize, flags: usize) -> *mut vm_struct;
    fn vunmap(address: *mut core::ffi::c_void);
    fn remove_vm_area(address: *mut core::ffi::c_void) -> *mut vm_struct;
    fn kfree(ptr: *mut vm_struct);
}

extern "C" {
    static mut init_mm: mm_struct;
    static mut high_memory: *mut core::ffi::c_void;
}

const IS_MAPPABLE_UNCACHEABLE: fn(usize) -> bool = |addr| addr < 0x20000000usize;

unsafe fn remap_area_pte(
    mut pte: *mut pte_t, mut address: usize, size: usize,
    phys_addr: usize, flags: usize,
) {
    let mut end: usize;
    let mut pfn: usize;
    let pgprot = __pgprot(_PAGE_GLOBAL | _PAGE_PRESENT | _PAGE_READ | _PAGE_WRITE | flags);

    address &= !PMD_MASK;
    end = address.wrapping_add(size);
    if end > PMD_SIZE { end = PMD_SIZE; }
    if address >= end { BUG(); }
    pfn = phys_addr / PAGE_SIZE;
    loop {
        if pte_none(*pte) {
            // empty
        } else {
            pr_err("remap_area_pte: page already exists\n");
            BUG();
        }
        set_pte(pte, pfn_pte(pfn, pgprot));
        address = address.wrapping_add(PAGE_SIZE);
        pfn = pfn.wrapping_add(1);
        pte = pte.add(1);
        if address == 0 || address >= end { break; }
    }
}

unsafe fn remap_area_pmd(
    mut pmd: *mut pmd_t, mut address: usize, size: usize,
    mut phys_addr: usize, flags: usize,
) -> i32 {
    let mut end = address.wrapping_add(size);
    address &= !PGDIR_MASK;
    end = address.wrapping_add(size);
    if end > PGDIR_SIZE { end = PGDIR_SIZE; }
    phys_addr = phys_addr.wrapping_sub(address);
    if address >= end { BUG(); }
    loop {
        let pte = pte_alloc_kernel(pmd, address);
        if pte.is_null() { return -12; }
        remap_area_pte(pte, address, end - address, address.wrapping_add(phys_addr), flags);
        address = address.wrapping_add(PMD_SIZE) & PMD_MASK;
        pmd = pmd.add(1);
        if address == 0 || address >= end { break; }
    }
    0
}

unsafe fn remap_area_pages(mut address: usize, mut phys_addr: usize, size: usize, flags: usize) -> i32 {
    let mut error: i32;
    let mut dir: *mut pgd_t;
    let end = address.wrapping_add(size);
    phys_addr = phys_addr.wrapping_sub(address);
    dir = pgd_offset(&mut init_mm, address);
    flush_cache_all();
    if address >= end { BUG(); }
    loop {
        error = -12;
        let p4d = p4d_alloc(&mut init_mm, dir, address);
        if p4d.is_null() { break; }
        let pud = pud_alloc(&mut init_mm, p4d, address);
        if pud.is_null() { break; }
        let pmd = pmd_alloc(&mut init_mm, pud, address);
        if pmd.is_null() { break; }
        if remap_area_pmd(pmd, address, end - address, phys_addr.wrapping_add(address), flags) != 0 { break; }
        error = 0;
        address = address.wrapping_add(PGDIR_SIZE) & PGDIR_MASK;
        dir = dir.add(1);
        if address == 0 || address >= end { break; }
    }
    flush_tlb_all();
    error
}

/* Map some physical address range into the kernel address space. */
#[no_mangle]
pub unsafe extern "C" fn ioremap(mut phys_addr: usize, mut size: usize) -> *mut core::ffi::c_void {
    let last_addr = phys_addr.wrapping_add(size).wrapping_sub(1);
    if size == 0 || last_addr < phys_addr { return core::ptr::null_mut(); }
    if phys_addr > PHYS_OFFSET && phys_addr < virt_to_phys(high_memory) {
        let t_addr = __va(phys_addr);
        let t_end = t_addr.add(size - 1);
        let mut page = virt_to_page(t_addr);
        while page <= virt_to_page(t_end) {
            if !PageReserved(page) { return core::ptr::null_mut(); }
            page = page.add(1);
        }
    }
    if IS_MAPPABLE_UNCACHEABLE(phys_addr) && IS_MAPPABLE_UNCACHEABLE(last_addr) {
        return (CONFIG_NIOS2_IO_REGION_BASE.wrapping_add(phys_addr)) as *mut core::ffi::c_void;
    }
    let offset = phys_addr & !PAGE_MASK;
    phys_addr &= PAGE_MASK;
    size = (last_addr.wrapping_add(1) + PAGE_SIZE - 1) & PAGE_MASK;
    let area = get_vm_area(size, VM_IOREMAP);
    if area.is_null() { return core::ptr::null_mut(); }
    let addr = (*area).addr;
    if remap_area_pages(addr as usize, phys_addr, size, 0) != 0 {
        vunmap(addr);
        return core::ptr::null_mut();
    }
    (addr as usize + offset) as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *mut core::ffi::c_void) {
    if addr as usize > CONFIG_NIOS2_IO_REGION_BASE { return; }
    let p = remove_vm_area((PAGE_MASK & addr as usize) as *mut core::ffi::c_void);
    if p.is_null() { pr_err("iounmap: bad address %p\n", addr); }
    kfree(p);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
