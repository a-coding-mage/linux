// SPDX-License-Identifier: GPL-2.0-only
/*
 * ioremap implementation.
 *
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Dependencies supplied by the Linux and Xtensa headers:
// linux/io.h, linux/pgtable.h, asm/cacheflush.h, asm/io.h

extern "C" {
    fn __phys_to_pfn(phys_addr: phys_addr_t) -> ::core::ffi::c_ulong;
    fn pfn_valid(pfn: ::core::ffi::c_ulong) -> bool;
    fn generic_ioremap_prot(
        phys_addr: phys_addr_t,
        size: usize,
        prot: pgprot_t,
    ) -> *mut ::core::ffi::c_void;
    fn generic_iounmap(addr: *const ::core::ffi::c_void);
    fn WARN_ON(condition: bool) -> bool;
}

extern "C" {
    static XCHAL_KIO_CACHED_VADDR: ::core::ffi::c_ulong;
    static XCHAL_KIO_BYPASS_VADDR: ::core::ffi::c_ulong;
    static XCHAL_KIO_SIZE: ::core::ffi::c_ulong;
}

pub unsafe fn ioremap_prot(
    phys_addr: phys_addr_t,
    size: usize,
    prot: pgprot_t,
) -> *mut ::core::ffi::c_void {
    let pfn: ::core::ffi::c_ulong = __phys_to_pfn(phys_addr);
    WARN_ON(pfn_valid(pfn));

    generic_ioremap_prot(phys_addr, size, prot)
}

// EXPORT_SYMBOL(ioremap_prot);

pub unsafe fn iounmap(addr: *const ::core::ffi::c_void) {
    let va: ::core::ffi::c_ulong = addr as ::core::ffi::c_ulong;

    if (va >= XCHAL_KIO_CACHED_VADDR
        && va - XCHAL_KIO_CACHED_VADDR < XCHAL_KIO_SIZE)
        || (va >= XCHAL_KIO_BYPASS_VADDR
            && va - XCHAL_KIO_BYPASS_VADDR < XCHAL_KIO_SIZE)
    {
        return;
    }

    generic_iounmap(addr);
}

// EXPORT_SYMBOL(iounmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
