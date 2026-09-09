// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding architecture and generic headers
// are intentionally referenced here rather than reimplemented.

unsafe extern "C" {
    fn early_memremap(phys_addr: resource_size_t, size: ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    fn TO_CACHE(phys_addr: phys_addr_t) -> phys_addr_t;
}

pub unsafe fn early_ioremap(
    phys_addr: phys_addr_t,
    size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    let _ = size;
    TO_CACHE(phys_addr) as *mut ::core::ffi::c_void
}

pub unsafe fn early_iounmap(
    addr: *mut ::core::ffi::c_void,
    size: ::core::ffi::c_ulong,
) {
    let _ = addr;
    let _ = size;
}

pub unsafe fn early_memremap_ro(
    phys_addr: resource_size_t,
    size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    early_memremap(phys_addr, size)
}

pub unsafe fn early_memremap_prot(
    phys_addr: resource_size_t,
    size: ::core::ffi::c_ulong,
    prot_val: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    let _ = prot_val;
    early_memremap(phys_addr, size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
