// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) 2003, 04, 11 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2011 Wind River Systems,
 *   written by Ralf Baechle (ralf@linux-mips.org)
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_int, c_uint, c_ulong};

pub type resource_size_t = u64;
pub type phys_addr_t = u64;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    _rest: [u8; 0],
}

extern "C" {
    fn cpu_dcache_line_size() -> c_uint;
    fn cpu_scache_line_size() -> c_uint;
    fn cpu_tcache_line_size() -> c_uint;
    fn resource_size(rsrc: *const resource) -> resource_size_t;
    fn fixup_bigphys_addr(addr: resource_size_t, size: resource_size_t) -> phys_addr_t;
    static mut pci_dfl_cache_line_size: u8;
}

#[no_mangle]
pub static mut PCIBIOS_MIN_IO: c_ulong = 0;

#[no_mangle]
pub static mut PCIBIOS_MIN_MEM: c_ulong = 0;

unsafe fn pcibios_set_cache_line_size() -> c_int {
    let mut lsize: c_uint;

    /*
     * Set PCI cacheline size to that of the highest level in the
     * cache hierarchy.
     */
    lsize = cpu_dcache_line_size();
    let scache_line_size = cpu_scache_line_size();
    lsize = if scache_line_size != 0 {
        scache_line_size
    } else {
        lsize
    };
    let tcache_line_size = cpu_tcache_line_size();
    lsize = if tcache_line_size != 0 {
        tcache_line_size
    } else {
        lsize
    };

    assert!(lsize != 0);

    pci_dfl_cache_line_size = (lsize >> 2) as u8;

    // pr_debug("PCI: pci_cache_line_size set to %d bytes\n", lsize);
    0
}

// arch_initcall(pcibios_set_cache_line_size);

pub unsafe fn pci_resource_to_user(
    _dev: *const pci_dev,
    _bar: c_int,
    rsrc: *const resource,
    start: *mut resource_size_t,
    end: *mut resource_size_t,
) {
    let size: phys_addr_t = resource_size(rsrc) as phys_addr_t;

    *start = fixup_bigphys_addr((*rsrc).start, size as resource_size_t) as resource_size_t;
    *end = (*rsrc).start + size as resource_size_t - 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
