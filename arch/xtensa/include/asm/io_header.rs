/*
 * include/asm-xtensa/io.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/byteorder.h, asm/page.h, asm/vectors.h, linux/bug.h,
// linux/kernel.h, linux/pgtable.h, linux/types.h, and asm-generic/io.h.

#[allow(non_snake_case)]
pub const fn IOADDR(x: usize) -> usize {
    XCHAL_KIO_BYPASS_VADDR.wrapping_add(x)
}
pub const IO_SPACE_LIMIT: usize = !0;
pub const PCI_IOBASE: *mut core::ffi::c_void = XCHAL_KIO_BYPASS_VADDR as *mut core::ffi::c_void;

// CONFIG_MMU
extern "C" {
    pub fn ioremap_prot(
        phys_addr: phys_addr_t,
        size: usize,
        prot: pgprot_t,
    ) -> *mut core::ffi::c_void;
}

// #define ioremap_prot ioremap_prot
// #define iounmap iounmap

#[inline]
pub unsafe fn ioremap(offset: core::ffi::c_ulong, size: core::ffi::c_ulong) -> *mut core::ffi::c_void {
    if offset >= XCHAL_KIO_PADDR
        && offset.wrapping_sub(XCHAL_KIO_PADDR) < XCHAL_KIO_SIZE
    {
        (offset
            .wrapping_sub(XCHAL_KIO_PADDR)
            .wrapping_add(XCHAL_KIO_BYPASS_VADDR)) as *mut core::ffi::c_void
    } else {
        ioremap_prot(offset as phys_addr_t, size as usize, pgprot_noncached(PAGE_KERNEL))
    }
}

// #define ioremap ioremap

#[inline]
pub unsafe fn ioremap_cache(
    offset: core::ffi::c_ulong,
    size: core::ffi::c_ulong,
) -> *mut core::ffi::c_void {
    if offset >= XCHAL_KIO_PADDR
        && offset.wrapping_sub(XCHAL_KIO_PADDR) < XCHAL_KIO_SIZE
    {
        (offset
            .wrapping_sub(XCHAL_KIO_PADDR)
            .wrapping_add(XCHAL_KIO_CACHED_VADDR)) as *mut core::ffi::c_void
    } else {
        ioremap_prot(offset as phys_addr_t, size as usize, PAGE_KERNEL)
    }
}

// #define ioremap_cache ioremap_cache

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
