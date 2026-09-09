// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/mm/hugetlbpage.c
 *
 * SuperH HugeTLB page support.
 *
 * Cloned from sparc64 by Paul Mundt.
 *
 * Copyright (C) 2002, 2003 David S. Miller (davem@redhat.com)
 */

// Linux and SuperH declarations are supplied by the surrounding kernel.

extern "C" {
    fn pgd_offset(mm: *mut mm_struct, addr: libc::c_ulong) -> *mut pgd_t;
    fn p4d_alloc(
        mm: *mut mm_struct,
        pgd: *mut pgd_t,
        addr: libc::c_ulong,
    ) -> *mut p4d_t;
    fn pud_alloc(
        mm: *mut mm_struct,
        p4d: *mut p4d_t,
        addr: libc::c_ulong,
    ) -> *mut pud_t;
    fn pmd_alloc(
        mm: *mut mm_struct,
        pud: *mut pud_t,
        addr: libc::c_ulong,
    ) -> *mut pmd_t;
    fn pte_alloc_huge(
        mm: *mut mm_struct,
        pmd: *mut pmd_t,
        addr: libc::c_ulong,
    ) -> *mut pte_t;
    fn p4d_offset(pgd: *mut pgd_t, addr: libc::c_ulong) -> *mut p4d_t;
    fn pud_offset(p4d: *mut p4d_t, addr: libc::c_ulong) -> *mut pud_t;
    fn pmd_offset(pud: *mut pud_t, addr: libc::c_ulong) -> *mut pmd_t;
    fn pte_offset_huge(pmd: *mut pmd_t, addr: libc::c_ulong) -> *mut pte_t;
}

pub unsafe fn huge_pte_alloc(
    mm: *mut mm_struct,
    vma: *mut vm_area_struct,
    addr: libc::c_ulong,
    sz: libc::c_ulong,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    if !pgd.is_null() {
        p4d = p4d_alloc(mm, pgd, addr);
        if !p4d.is_null() {
            pud = pud_alloc(mm, p4d, addr);
            if !pud.is_null() {
                pmd = pmd_alloc(mm, pud, addr);
                if !pmd.is_null() {
                    pte = pte_alloc_huge(mm, pmd, addr);
                }
            }
        }
    }

    pte
}

pub unsafe fn huge_pte_offset(
    mm: *mut mm_struct,
    addr: libc::c_ulong,
    sz: libc::c_ulong,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();

    pgd = pgd_offset(mm, addr);
    if !pgd.is_null() {
        p4d = p4d_offset(pgd, addr);
        if !p4d.is_null() {
            pud = pud_offset(p4d, addr);
            if !pud.is_null() {
                pmd = pmd_offset(pud, addr);
                if !pmd.is_null() {
                    pte = pte_offset_huge(pmd, addr);
                }
            }
        }
    }

    pte
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
