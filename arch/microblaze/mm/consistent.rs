// SPDX-License-Identifier: GPL-2.0-only
/*
 * Microblaze support for cache consistent memory.
 * Copyright (C) 2010 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2010 PetaLogix
 * Copyright (C) 2005 John Williams <jwilliams@itee.uq.edu.au>
 */

use core::ffi::c_ulong;

// Types and functions supplied by the surrounding kernel environment.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub type phys_addr_t = usize;

unsafe extern "C" {
    fn page_to_phys(page: *mut page) -> phys_addr_t;
    fn flush_dcache_range(start: phys_addr_t, end: phys_addr_t);
}

#[no_mangle]
pub unsafe extern "C" fn arch_dma_prep_coherent(page: *mut page, size: c_ulong) {
    let paddr: phys_addr_t = page_to_phys(page);

    flush_dcache_range(paddr, paddr.wrapping_add(size as phys_addr_t));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
