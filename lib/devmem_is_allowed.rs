// SPDX-License-Identifier: GPL-2.0-only
/*
 * A generic version of devmem_is_allowed.
 *
 * Based on arch/arm64/mm/mmap.c
 *
 * Copyright (C) 2020 Google, Inc.
 * Copyright (C) 2012 ARM Ltd.
 */

// Declarations supplied by the Linux memory-management and I/O dependencies.
unsafe extern "C" {
    fn iomem_is_exclusive(addr: usize) -> i32;
    fn page_is_ram(pfn: usize) -> i32;
}

// PFN_PHYS is a preprocessor macro supplied by the platform headers.
extern "C" {
    fn PFN_PHYS(pfn: usize) -> usize;
}

/*
 * devmem_is_allowed() checks to see if /dev/mem access to a certain address
 * is valid. The argument is a physical page number.  We mimic x86 here by
 * disallowing access to system RAM as well as device-exclusive MMIO regions.
 * This effectively disable read()/write() on /dev/mem.
 */
pub unsafe fn devmem_is_allowed(pfn: usize) -> i32 {
    if iomem_is_exclusive(PFN_PHYS(pfn)) != 0 {
        return 0;
    }
    if page_is_ram(pfn) == 0 {
        return 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
