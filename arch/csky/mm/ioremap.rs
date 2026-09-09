// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Declarations supplied by the Linux kernel headers.
use crate::{file, pgprot_t};

extern "C" {
    fn pfn_valid(pfn: ::core::ffi::c_ulong) -> bool;
    fn pgprot_noncached(vma_prot: pgprot_t) -> pgprot_t;
    fn pgprot_writecombine(vma_prot: pgprot_t) -> pgprot_t;
}

// O_SYNC is supplied by the Linux kernel headers.
extern "C" {
    static O_SYNC: ::core::ffi::c_uint;
}

pub unsafe fn phys_mem_access_prot(
    file: *mut file,
    pfn: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
    vma_prot: pgprot_t,
) -> pgprot_t {
    let _ = size;

    if !pfn_valid(pfn) {
        return pgprot_noncached(vma_prot);
    } else if ((*file).f_flags & O_SYNC as _) != 0 {
        return pgprot_writecombine(vma_prot);
    }

    vma_prot
}

// EXPORT_SYMBOL(phys_mem_access_prot);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
