/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Copyright © 2008 Ingo Molnar
 */

/*
 * Dependencies supplied by the surrounding kernel translation:
 * linux/fs.h, linux/mm.h, linux/uaccess.h, linux/highmem.h,
 * asm/cacheflush.h, and asm/tlbflush.h.
 */

unsafe extern "C" {
    pub fn __iomap_local_pfn_prot(
        pfn: core::ffi::c_ulong,
        prot: pgprot_t,
    ) -> *mut core::ffi::c_void;

    pub fn iomap_create_wc(
        base: resource_size_t,
        size: core::ffi::c_ulong,
        prot: *mut pgprot_t,
    ) -> core::ffi::c_int;

    pub fn iomap_free(base: resource_size_t, size: core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
