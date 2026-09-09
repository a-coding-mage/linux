// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the kernel headers are intentionally left external.

extern "C" {
    static mut ioremap_bot: ::core::ffi::c_ulong;

    fn __ioremap_caller(
        addr: phys_addr_t,
        size: ::core::ffi::c_ulong,
        prot: pgprot_t,
        caller: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn pgprot_noncached(prot: pgprot_t) -> pgprot_t;
    fn pgprot_noncached_wc(prot: pgprot_t) -> pgprot_t;
    fn pgprot_cached(prot: pgprot_t) -> pgprot_t;
    fn pgprot_val(prot: pgprot_t) -> ::core::ffi::c_ulong;
    fn __pte(val: ::core::ffi::c_ulong) -> pte_t;
    fn pte_write(pte: pte_t) -> bool;
    fn pte_mkdirty(pte: pte_t) -> pte_t;
    fn pte_pgprot(pte: pte_t) -> pgprot_t;
    fn pgprot_nx(prot: pgprot_t) -> pgprot_t;
    fn map_kernel_page(
        ea: ::core::ffi::c_ulong,
        pa: phys_addr_t,
        prot: pgprot_t,
    ) -> ::core::ffi::c_int;
    fn WARN_ON_ONCE(condition: ::core::ffi::c_int) -> bool;
}

// EXPORT_SYMBOL(ioremap_bot);

pub unsafe fn ioremap(addr: phys_addr_t, size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    let prot: pgprot_t = pgprot_noncached(PAGE_KERNEL);
    // Rust has no direct stable equivalent of __builtin_return_address(0).
    let caller: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

    __ioremap_caller(addr, size, prot, caller)
}

// EXPORT_SYMBOL(ioremap);

pub unsafe fn ioremap_wc(addr: phys_addr_t, size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    let prot: pgprot_t = pgprot_noncached_wc(PAGE_KERNEL);
    // Rust has no direct stable equivalent of __builtin_return_address(0).
    let caller: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

    __ioremap_caller(addr, size, prot, caller)
}

// EXPORT_SYMBOL(ioremap_wc);

pub unsafe fn ioremap_coherent(
    addr: phys_addr_t,
    size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    let prot: pgprot_t = pgprot_cached(PAGE_KERNEL);
    // Rust has no direct stable equivalent of __builtin_return_address(0).
    let caller: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

    __ioremap_caller(addr, size, prot, caller)
}

pub unsafe fn ioremap_prot(
    addr: phys_addr_t,
    size: usize,
    prot: pgprot_t,
) -> *mut ::core::ffi::c_void {
    let mut pte: pte_t = __pte(pgprot_val(prot));
    // Rust has no direct stable equivalent of __builtin_return_address(0).
    let caller: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

    /* writeable implies dirty for kernel addresses */
    if pte_write(pte) {
        pte = pte_mkdirty(pte);
    }

    __ioremap_caller(addr, size as ::core::ffi::c_ulong, pte_pgprot(pte), caller)
}

// EXPORT_SYMBOL(ioremap_prot);

pub unsafe fn early_ioremap_range(
    ea: ::core::ffi::c_ulong,
    pa: phys_addr_t,
    size: ::core::ffi::c_ulong,
    prot: pgprot_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_ulong = 0;

    while i < size {
        let err: ::core::ffi::c_int = map_kernel_page(ea + i, pa + i, pgprot_nx(prot));

        if WARN_ON_ONCE(err) { /* Should clean up */
            return err;
        }
        i += PAGE_SIZE;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
