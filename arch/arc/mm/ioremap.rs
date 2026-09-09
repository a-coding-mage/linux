// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel dependencies supplied by other translated files.
use core::ffi::c_void;

pub type phys_addr_t = u64;
pub type pgprot_t = usize;

unsafe extern "C" {
    fn is_isa_arcompact() -> bool;
    static perip_base: phys_addr_t;
    static perip_end: phys_addr_t;
    fn ioremap_prot(paddr: phys_addr_t, size: usize, prot: pgprot_t) -> *mut c_void;
    fn pgprot_noncached(prot: pgprot_t) -> pgprot_t;
    fn generic_ioremap_prot(paddr: phys_addr_t, size: usize, prot: pgprot_t) -> *mut c_void;
    fn generic_iounmap(addr: *const c_void);
}

// Build-time architecture constant supplied by the surrounding kernel.
const ARC_UNCACHED_ADDR_SPACE: phys_addr_t = 0;

#[inline]
unsafe fn arc_uncached_addr_space(paddr: phys_addr_t) -> bool {
    if is_isa_arcompact() {
        if paddr >= ARC_UNCACHED_ADDR_SPACE {
            return true;
        }
    } else if paddr >= perip_base && paddr <= perip_end {
        return true;
    }

    false
}

pub unsafe fn ioremap(paddr: phys_addr_t, size: u64) -> *mut c_void {
    /*
     * If the region is h/w uncached, MMU mapping can be elided as optim
     * The cast to u32 is fine as this region can only be inside 4GB
     */
    if arc_uncached_addr_space(paddr) {
        return (paddr as u32) as usize as *mut c_void;
    }

    ioremap_prot(paddr, size as usize, pgprot_noncached(PAGE_KERNEL))
}

/*
 * ioremap with access flags
 * Cache semantics wise it is same as ioremap - "forced" uncached.
 * However unlike vanilla ioremap which bypasses ARC MMU for addresses in
 * ARC hardware uncached region, this one still goes thru the MMU as caller
 * might need finer access control (R/W/X)
 */
pub unsafe fn ioremap_prot(paddr: phys_addr_t, size: usize, prot: pgprot_t) -> *mut c_void {
    /* force uncached */
    generic_ioremap_prot(paddr, size, pgprot_noncached(prot))
}

pub unsafe fn iounmap(addr: *const c_void) {
    /* weird double cast to handle phys_addr_t > 32 bits */
    if arc_uncached_addr_space((addr as usize as u32) as phys_addr_t) {
        return;
    }

    generic_iounmap(addr);
}

// PAGE_KERNEL is supplied by the surrounding memory-management translation.
const PAGE_KERNEL: pgprot_t = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
