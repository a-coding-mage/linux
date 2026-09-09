// SPDX-License-Identifier: GPL-2.0
/*
 * arch/parisc/mm/ioremap.c
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 * (C) Copyright 2001-2019 Helge Deller <deller@gmx.de>
 * (C) Copyright 2005 Kyle McMartin <kyle@parisc-linux.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct pgprot_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

type PhysAddr = usize;

extern "C" {
    static high_memory: *mut core::ffi::c_void;

    fn virt_to_phys(address: *const core::ffi::c_void) -> PhysAddr;
    fn __va(address: PhysAddr) -> *mut core::ffi::c_char;
    fn virt_to_page(address: *const core::ffi::c_char) -> *mut page;
    fn PageReserved(page: *const page) -> bool;
    fn generic_ioremap_prot(
        phys_addr: PhysAddr,
        size: usize,
        prot: pgprot_t,
    ) -> *mut core::ffi::c_void;
}

extern "C" {
    fn F_EXTEND(value: u32) -> PhysAddr;
}

pub unsafe fn ioremap_prot(
    mut phys_addr: PhysAddr,
    size: usize,
    prot: pgprot_t,
) -> *mut core::ffi::c_void {
    #[cfg(feature = "CONFIG_EISA")]
    {
        let end = phys_addr.wrapping_add(size).wrapping_sub(1);
        /* Support EISA addresses */
        if (phys_addr >= 0x0008_0000 && end < 0x000f_ffff)
            || (phys_addr >= 0x0050_0000 && end < 0x03bf_ffff)
        {
            phys_addr |= F_EXTEND(0xfc00_0000);
        }
    }

    /*
     * Don't allow anybody to remap normal RAM that we're using..
     */
    if phys_addr < virt_to_phys(high_memory as *const core::ffi::c_void) {
        let t_addr = __va(phys_addr);
        let t_end = t_addr.add(size.wrapping_sub(1));

        let mut page_ptr = virt_to_page(t_addr);
        let end_page = virt_to_page(t_end);
        while (page_ptr as usize) <= (end_page as usize) {
            if !PageReserved(page_ptr) {
                return core::ptr::null_mut();
            }
            page_ptr = page_ptr.add(1);
        }
    }

    generic_ioremap_prot(phys_addr, size, prot)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
