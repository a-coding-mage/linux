/*
 * MIPS Huge TLB Page Support for Kernel.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
 * Copyright 2005, Embedded Alley Solutions, Inc.
 * Matt Porter <mporter@embeddedalley.com>
 * Copyright (C) 2008, 2009 Cavium Networks, Inc.
 */

// Dependencies supplied by the kernel's Linux and MIPS headers:
// linux/fs.h, linux/mm.h, linux/hugetlb.h, linux/pagemap.h, linux/err.h,
// linux/sysctl.h, asm/mman.h, asm/tlb.h, asm/tlbflush.h

use core::ptr;

pub unsafe fn huge_pte_alloc(
    mm: *mut mm_struct,
    _vma: *mut vm_area_struct,
    addr: usize,
    _sz: usize,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let mut pte: *mut pte_t = ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    p4d = p4d_alloc(mm, pgd, addr);
    pud = pud_alloc(mm, p4d, addr);
    if !pud.is_null() {
        pte = pmd_alloc(mm, pud, addr) as *mut pte_t;
    }

    pte
}

pub unsafe fn huge_pte_offset(
    mm: *mut mm_struct,
    addr: usize,
    _sz: usize,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let mut pmd: *mut pmd_t = ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    if pgd_present(*pgd) {
        p4d = p4d_offset(pgd, addr);
        if p4d_present(*p4d) {
            pud = pud_offset(p4d, addr);
            if pud_present(*pud) {
                pmd = pmd_offset(pud, addr);
            }
        }
    }
    pmd as *mut pte_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
