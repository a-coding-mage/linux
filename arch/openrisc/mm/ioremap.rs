// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC ioremap.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

extern "C" {
    static mut mem_init_done: i32;
    fn __pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t;
    fn memblock_alloc_or_panic(size: usize, align: usize) -> *mut core::ffi::c_void;
}

/*
 * OK, this one's a bit tricky... ioremap can get called before memory is
 * initialized (early serial console does this) and will want to alloc a page
 * for its mapping.  No userspace pages will ever get allocated before memory
 * is initialized so this applies only to kernel pages.  In the event that
 * this is called before memory is initialized we allocate the page using the
 * memblock infrastructure.
 */

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let pte: *mut pte_t;

    if mem_init_done != 0 {
        pte = __pte_alloc_one_kernel(mm);
    } else {
        pte = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
    }

    pte
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
