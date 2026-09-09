/*
 * arch/sh/mm/ioremap.c
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 * (C) Copyright 2005 - 2010  Paul Mundt
 *
 * Re-map IO memory to kernel address space so that we can access it.
 * This is needed for high PCI addresses that aren't mapped in the
 * 640k-1MB IO memory area on PC's
 */

// Linux and SH dependencies supplied by other translation units.

#[cfg(CONFIG_29BIT)]
unsafe fn __ioremap_29bit(offset: phys_addr_t, size: c_ulong, prot: pgprot_t) -> *mut core::ffi::c_void {
    let last_addr = offset.wrapping_add(size as phys_addr_t).wrapping_sub(1);

    if PXSEG(offset) < P3SEG && PXSEG(last_addr) < P3SEG {
        let flags: u64 = pgprot_val(prot);

        if flags & _PAGE_PCC_MASK != 0 {
            return core::ptr::null_mut();
        }
        if flags & _PAGE_CACHABLE != 0 {
            return P1SEGADDR(offset) as *mut core::ffi::c_void;
        }

        return P2SEGADDR(offset) as *mut core::ffi::c_void;
    }

    /* P4 above the store queues are always mapped. */
    if offset >= P3_ADDR_MAX {
        return P4SEGADDR(offset) as *mut core::ffi::c_void;
    }

    core::ptr::null_mut()
}

#[cfg(not(CONFIG_29BIT))]
unsafe fn __ioremap_29bit(_offset: phys_addr_t, _size: c_ulong, _prot: pgprot_t) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

unsafe extern "C" {
    static mut mem_init_done: bool;

    fn __ioremap_trapped(phys_addr: phys_addr_t, size: c_ulong) -> *mut core::ffi::c_void;
    fn ioremap_fixed(phys_addr: phys_addr_t, size: c_ulong, prot: pgprot_t) -> *mut core::ffi::c_void;
    fn pmb_remap_caller(phys_addr: phys_addr_t, size: c_ulong, prot: pgprot_t, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn generic_ioremap_prot(phys_addr: phys_addr_t, size: c_ulong, prot: pgprot_t) -> *mut core::ffi::c_void;
    fn iounmap_fixed(addr: *mut core::ffi::c_void) -> i32;
    fn pmb_unmap(addr: *mut core::ffi::c_void) -> i32;
    fn generic_iounmap(addr: *mut core::ffi::c_void);
    fn __builtin_return_address(level: i32) -> *mut core::ffi::c_void;
}

pub unsafe fn ioremap_prot(phys_addr: phys_addr_t, size: usize, pgprot: pgprot_t) -> *mut core::ffi::c_void {
    let mut mapped = __ioremap_trapped(phys_addr, size as c_ulong);
    if !mapped.is_null() {
        return mapped;
    }

    mapped = __ioremap_29bit(phys_addr, size as c_ulong, pgprot);
    if !mapped.is_null() {
        return mapped;
    }

    /* If we can't yet use the regular approach, go the fixmap route. */
    if !mem_init_done {
        return ioremap_fixed(phys_addr, size as c_ulong, pgprot);
    }

    /* First try to remap through the PMB. PMB entries are all pre-faulted. */
    mapped = pmb_remap_caller(phys_addr, size as c_ulong, pgprot, __builtin_return_address(0));
    if !mapped.is_null() && !IS_ERR(mapped) {
        return mapped;
    }

    generic_ioremap_prot(phys_addr, size as c_ulong, pgprot)
}

#[cfg(CONFIG_29BIT)]
#[inline]
unsafe fn iomapping_nontranslatable(offset: c_ulong) -> i32 {
    if PXSEG(offset) < P3SEG || offset >= P3_ADDR_MAX {
        return 1;
    }
    0
}

#[cfg(not(CONFIG_29BIT))]
#[inline]
unsafe fn iomapping_nontranslatable(_offset: c_ulong) -> i32 {
    0
}

pub unsafe fn iounmap(addr: *mut core::ffi::c_void) {
    let vaddr = addr as c_ulong;

    if iomapping_nontranslatable(vaddr) != 0 {
        return;
    }
    if iounmap_fixed(addr) == 0 {
        return;
    }
    if pmb_unmap(addr) == 0 {
        return;
    }

    generic_iounmap(addr);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
