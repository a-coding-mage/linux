// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/mm/kmap.c
 *
 *  Copyright (C) 1997 Roman Hodek
 *
 *  10/01/99 cleaned up the code and changing to the same interface
 *           used by other architectures        /Roman Zippel
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/*
 * For 040/060 we can use the virtual memory area like other architectures,
 * but for 020/030 we want to use early termination page descriptors and we
 * can't mix this with normal page descriptors, so we have to copy that code
 * (mm/vmalloc.c) and return appropriately aligned addresses.
 */

#[cfg(CPU_M68040_OR_M68060_ONLY)]
const IO_SIZE: usize = PAGE_SIZE;

#[cfg(CPU_M68040_OR_M68060_ONLY)]
#[inline]
unsafe fn get_io_area(size: c_ulong) -> *mut vm_struct {
    get_vm_area(size, VM_IOREMAP)
}

#[cfg(CPU_M68040_OR_M68060_ONLY)]
#[inline]
unsafe fn free_io_area(addr: *mut c_void) {
    vfree((PAGE_MASK & addr as c_ulong) as *mut c_void);
}

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
const IO_SIZE: usize = PMD_SIZE;

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
static mut iolist: *mut vm_struct = core::ptr::null_mut();

/*
 * __free_io_area unmaps nearly everything, so be careful
 * Currently it doesn't free pointer/page tables anymore but this
 * wasn't used anyway and might be added later.
 */
#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
unsafe fn __free_io_area(addr: *mut c_void, mut size: c_ulong) {
    let mut virtaddr = addr as c_ulong;
    let (mut pgd_dir, mut p4d_dir, mut pud_dir, mut pmd_dir, mut pte_dir):
        (*mut pgd_t, *mut p4d_t, *mut pud_t, *mut pmd_t, *mut pte_t);

    while (size as c_long) > 0 {
        pgd_dir = pgd_offset_k(virtaddr);
        p4d_dir = p4d_offset(pgd_dir, virtaddr);
        pud_dir = pud_offset(p4d_dir, virtaddr);
        if pud_bad(*pud_dir) {
            printk(b"iounmap: bad pud(%08lx)\n\0".as_ptr(), pud_val(*pud_dir));
            pud_clear(pud_dir);
            return;
        }
        pmd_dir = pmd_offset(pud_dir, virtaddr);

        #[cfg(CONFIG_PGTABLE_LEVELS = "3")]
        if CPU_IS_020_OR_030 {
            let pmd_type = pmd_val(*pmd_dir) & _DESCTYPE_MASK;
            if pmd_type == _PAGE_PRESENT {
                pmd_clear(pmd_dir);
                virtaddr += PMD_SIZE;
                size -= PMD_SIZE;
            } else if pmd_type == 0 {
                continue;
            }
        }

        if pmd_bad(*pmd_dir) {
            printk(b"iounmap: bad pmd (%08lx)\n\0".as_ptr(), pmd_val(*pmd_dir));
            pmd_clear(pmd_dir);
            return;
        }
        pte_dir = pte_offset_kernel(pmd_dir, virtaddr);
        pte_val(*pte_dir) = 0;
        virtaddr += PAGE_SIZE;
        size -= PAGE_SIZE;
    }
    flush_tlb_all();
}

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
unsafe fn get_io_area(size: c_ulong) -> *mut vm_struct {
    let mut addr: c_ulong;
    let mut p: *mut *mut vm_struct;
    let mut tmp: *mut vm_struct;
    let area = kmalloc_obj::<vm_struct>();
    if area.is_null() { return core::ptr::null_mut(); }
    addr = KMAP_START;
    p = &raw mut iolist;
    while { tmp = *p; !tmp.is_null() } {
        if size + addr < (*tmp).addr as c_ulong { break; }
        if addr > KMAP_END - size {
            kfree(area as *mut c_void);
            return core::ptr::null_mut();
        }
        addr = (*tmp).size + (*tmp).addr as c_ulong;
        p = &raw mut (*tmp).next;
    }
    (*area).addr = addr as *mut c_void;
    (*area).size = size + IO_SIZE;
    (*area).next = *p;
    *p = area;
    area
}

#[cfg(not(CPU_M68040_OR_M68060_ONLY))]
#[inline]
unsafe fn free_io_area(mut addr: *mut c_void) {
    if addr.is_null() { return; }
    addr = ((addr as c_ulong) & (-(IO_SIZE as c_long) as c_ulong)) as *mut c_void;
    let mut p = &raw mut iolist;
    while { let tmp = *p; !tmp.is_null() } {
        let tmp = *p;
        if (*tmp).addr == addr {
            *p = (*tmp).next;
            /* remove gap added in get_io_area() */
            __free_io_area((*tmp).addr, (*tmp).size - IO_SIZE);
            kfree(tmp as *mut c_void);
            return;
        }
        p = &raw mut (*tmp).next;
    }
}

/* Map some physical address range into the kernel address space. */
/* Rewritten by Andreas Schwab to remove all races. */
pub unsafe fn __ioremap(mut physaddr: c_ulong, mut size: c_ulong, cacheflag: c_int) -> *mut c_void {
    let mut offset: c_long;
    if size == 0 || physaddr > (!size + 1) { return core::ptr::null_mut(); }

    #[cfg(CONFIG_AMIGA)]
    if MACH_IS_AMIGA && physaddr >= 0x40000000 && physaddr + size < 0x60000000 && cacheflag == IOMAP_NOCACHE_SER { return physaddr as *mut c_void; }
    #[cfg(CONFIG_VIRT)]
    if MACH_IS_VIRT && physaddr >= 0xff000000 && cacheflag == IOMAP_NOCACHE_SER { return physaddr as *mut c_void; }
    #[cfg(CONFIG_COLDFIRE)]
    if __cf_internalio(physaddr) { return physaddr as *mut c_void; }

    offset = (physaddr & (IO_SIZE as c_ulong - 1)) as c_long;
    physaddr &= -(IO_SIZE as c_long) as c_ulong;
    size = (size + offset as c_ulong + IO_SIZE as c_ulong - 1) & -(IO_SIZE as c_long) as c_ulong;
    let area = get_io_area(size);
    if area.is_null() { return core::ptr::null_mut(); }
    let mut virtaddr = (*area).addr as c_ulong;
    let retaddr = virtaddr + offset as c_ulong;

    if CPU_IS_040_OR_060 {
        physaddr |= _PAGE_PRESENT | _PAGE_GLOBAL040 | _PAGE_ACCESSED | _PAGE_DIRTY;
        match cacheflag { IOMAP_FULL_CACHING => physaddr |= _PAGE_CACHE040, IOMAP_NOCACHE_NONSER => physaddr |= _PAGE_NOCACHE, IOMAP_WRITETHROUGH => physaddr |= _PAGE_CACHE040W, _ => physaddr |= _PAGE_NOCACHE_S }
    } else {
        physaddr |= _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_DIRTY | _PAGE_READWRITE;
        match cacheflag { IOMAP_FULL_CACHING | IOMAP_WRITETHROUGH => {}, _ => physaddr |= _PAGE_NOCACHE030 }
    }

    while (size as c_long) > 0 {
        let pgd_dir = pgd_offset_k(virtaddr);
        let p4d_dir = p4d_offset(pgd_dir, virtaddr);
        let pud_dir = pud_offset(p4d_dir, virtaddr);
        let pmd_dir = pmd_alloc(&mut init_mm, pud_dir, virtaddr);
        if pmd_dir.is_null() { printk(b"ioremap: no mem for pmd_dir\n\0".as_ptr()); return core::ptr::null_mut(); }
        #[cfg(CONFIG_PGTABLE_LEVELS = "3")]
        if CPU_IS_020_OR_030 { pmd_val(*pmd_dir) = physaddr; physaddr += PMD_SIZE; virtaddr += PMD_SIZE; size -= PMD_SIZE; } else
        {
            let pte_dir = pte_alloc_kernel(pmd_dir, virtaddr);
            if pte_dir.is_null() { printk(b"ioremap: no mem for pte_dir\n\0".as_ptr()); return core::ptr::null_mut(); }
            pte_val(*pte_dir) = physaddr; virtaddr += PAGE_SIZE; physaddr += PAGE_SIZE; size -= PAGE_SIZE;
        }
    }
    flush_tlb_all();
    retaddr as *mut c_void
}

/* Unmap an ioremap()ed region again */
pub unsafe fn iounmap(addr: *mut c_void) {
    #[cfg(CONFIG_AMIGA)]
    if MACH_IS_AMIGA && addr as c_ulong >= 0x40000000 && addr as c_ulong < 0x60000000 { return; }
    #[cfg(CONFIG_VIRT)]
    if MACH_IS_VIRT && addr as c_ulong >= 0xff000000 { return; }
    #[cfg(CONFIG_COLDFIRE)]
    if cf_internalio(addr) { return; }
    free_io_area(addr);
}

/* Set new cache mode for some kernel address space.
 * The caller must push data for that range itself, if such data may already
 * be in the cache.
 */
pub unsafe fn kernel_set_cachemode(addr: *mut c_void, mut size: c_ulong, mut cmode: c_int) {
    let mut virtaddr = addr as c_ulong;
    if CPU_IS_040_OR_060 { cmode = match cmode { IOMAP_FULL_CACHING => _PAGE_CACHE040, IOMAP_NOCACHE_NONSER => _PAGE_NOCACHE, IOMAP_WRITETHROUGH => _PAGE_CACHE040W, _ => _PAGE_NOCACHE_S }; }
    else { cmode = match cmode { IOMAP_FULL_CACHING | IOMAP_WRITETHROUGH => 0, _ => _PAGE_NOCACHE030 }; }
    while (size as c_long) > 0 {
        let pgd_dir = pgd_offset_k(virtaddr); let p4d_dir = p4d_offset(pgd_dir, virtaddr); let pud_dir = pud_offset(p4d_dir, virtaddr);
        if pud_bad(*pud_dir) { printk(b"iocachemode: bad pud(%08lx)\n\0".as_ptr(), pud_val(*pud_dir)); pud_clear(pud_dir); return; }
        let pmd_dir = pmd_offset(pud_dir, virtaddr);
        #[cfg(CONFIG_PGTABLE_LEVELS = "3")]
        if CPU_IS_020_OR_030 { let pmd = pmd_val(*pmd_dir); if (pmd & _DESCTYPE_MASK) == _PAGE_PRESENT { *pmd_dir = __pmd((pmd & _CACHEMASK040) | cmode); virtaddr += PMD_SIZE; size -= PMD_SIZE; continue; } }
        if pmd_bad(*pmd_dir) { printk(b"iocachemode: bad pmd (%08lx)\n\0".as_ptr(), pmd_val(*pmd_dir)); pmd_clear(pmd_dir); return; }
        let pte_dir = pte_offset_kernel(pmd_dir, virtaddr); pte_val(*pte_dir) = (pte_val(*pte_dir) & _CACHEMASK040) | cmode; virtaddr += PAGE_SIZE; size -= PAGE_SIZE;
    }
    flush_tlb_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
