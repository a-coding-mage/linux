// SPDX-License-Identifier: GPL-2.0
/*
 * Re-map IO memory to kernel address space so that we can access it.
 * This is needed for high PCI addresses that aren't mapped in the
 * 640k-1MB IO memory area on PC's
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

pub unsafe fn generic_ioremap_prot(
    mut phys_addr: phys_addr_t,
    mut size: usize,
    prot: pgprot_t,
) -> *mut c_void {
    let offset: usize;
    let vaddr: usize;
    let last_addr: phys_addr_t;
    let area: *mut vm_struct;

    /* An early platform driver might end up here */
    if WARN_ON_ONCE(!slab_is_available()) {
        return core::ptr::null_mut();
    }

    /* Disallow wrap-around or zero size */
    last_addr = phys_addr
        .wrapping_add(size as phys_addr_t)
        .wrapping_sub(1);
    if size == 0 || last_addr < phys_addr {
        return core::ptr::null_mut();
    }

    /* Page-align mappings */
    offset = (phys_addr as usize) & (!PAGE_MASK);
    phys_addr = phys_addr.wrapping_sub(offset as phys_addr_t);
    size = PAGE_ALIGN(size.wrapping_add(offset));

    area = __get_vm_area_caller(
        size,
        VM_IOREMAP,
        IOREMAP_START,
        IOREMAP_END,
        builtin_return_address(0),
    );
    if area.is_null() {
        return core::ptr::null_mut();
    }
    vaddr = (*area).addr as usize;
    (*area).phys_addr = phys_addr;

    if ioremap_page_range(vaddr, vaddr.wrapping_add(size), phys_addr, prot) != 0 {
        free_vm_area(area);
        return core::ptr::null_mut();
    }

    (vaddr.wrapping_add(offset)) as *mut c_void
}

// The C source conditionally defines this function when no platform
// definition of ioremap_prot is available.
#[cfg(not(ioremap_prot))]
pub unsafe fn ioremap_prot(
    phys_addr: phys_addr_t,
    size: usize,
    prot: pgprot_t,
) -> *mut c_void {
    generic_ioremap_prot(phys_addr, size, prot)
}

#[cfg(not(ioremap_prot))]
pub const EXPORT_SYMBOL_ioremap_prot: &str = "ioremap_prot";

pub unsafe fn generic_iounmap(addr: *const c_void) {
    let vaddr = ((addr as usize) & PAGE_MASK) as *mut c_void;

    if is_ioremap_addr(vaddr) {
        vunmap(vaddr);
    }
}

// The C source conditionally defines this function when no platform
// definition of iounmap is available.
#[cfg(not(iounmap))]
pub unsafe fn iounmap(addr: *const c_void) {
    generic_iounmap(addr);
}

#[cfg(not(iounmap))]
pub const EXPORT_SYMBOL_iounmap: &str = "iounmap";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
