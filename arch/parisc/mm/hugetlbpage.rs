// SPDX-License-Identifier: GPL-2.0
/*
 * PARISC64 Huge TLB page support.
 *
 * This parisc implementation is heavily based on the SPARC and x86 code.
 *
 * Copyright (C) 2015 Helge Deller <deller@gmx.de>
 */

// Linux and architecture header dependencies are supplied by other files.

pub unsafe fn huge_pte_alloc(
    mm: *mut mm_struct,
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    sz: c_ulong,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();

    /* We must align the address, because our caller will run
     * set_huge_pte_at() on whatever we return, which writes out
     * all of the sub-ptes for the hugepage range.  So we have
     * to give it the first such sub-pte.
     */
    addr &= HPAGE_MASK;

    pgd = pgd_offset(mm, addr);
    p4d = p4d_offset(pgd, addr);
    pud = pud_alloc(mm, p4d, addr);
    if !pud.is_null() {
        pmd = pmd_alloc(mm, pud, addr);
        if !pmd.is_null() {
            pte = pte_alloc_huge(mm, pmd, addr);
        }
    }
    pte
}

pub unsafe fn huge_pte_offset(
    mm: *mut mm_struct,
    mut addr: c_ulong,
    sz: c_ulong,
) -> *mut pte_t {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let mut pte: *mut pte_t = core::ptr::null_mut();

    addr &= HPAGE_MASK;

    pgd = pgd_offset(mm, addr);
    if !pgd_none(*pgd) {
        p4d = p4d_offset(pgd, addr);
        if !p4d_none(*p4d) {
            pud = pud_offset(p4d, addr);
            if !pud_none(*pud) {
                pmd = pmd_offset(pud, addr);
                if !pmd_none(*pmd) {
                    pte = pte_offset_huge(pmd, addr);
                }
            }
        }
    }
    pte
}

/* Purge data and instruction TLB entries.  Must be called holding
 * the pa_tlb_lock.  The TLB purge instructions are slow on SMP
 * machines since the purge must be broadcast to all CPUs.
 */
#[inline]
unsafe fn purge_tlb_entries_huge(mm: *mut mm_struct, mut addr: c_ulong) {
    /* We may use multiple physical huge pages (e.g. 2x1 MB) to emulate
     * Linux standard huge pages (e.g. 2 MB) */
    build_bug_on!(REAL_HPAGE_SHIFT > HPAGE_SHIFT);

    addr &= HPAGE_MASK;
    addr |= _HUGE_PAGE_SIZE_ENCODING_DEFAULT;

    let mut i = 0;
    while i < (1 << (HPAGE_SHIFT - REAL_HPAGE_SHIFT)) {
        purge_tlb_entries(mm, addr);
        addr = addr.wrapping_add(1u64 << REAL_HPAGE_SHIFT);
        i += 1;
    }
}

/* __set_huge_pte_at() must be called holding the pa_tlb_lock. */
unsafe fn __set_huge_pte_at(
    mm: *mut mm_struct,
    mut addr: c_ulong,
    mut ptep: *mut pte_t,
    mut entry: pte_t,
) {
    addr &= HPAGE_MASK;
    let addr_start = addr;

    let mut i = 0;
    while i < (1 << HUGETLB_PAGE_ORDER) {
        set_pte(ptep, entry);
        ptep = ptep.add(1);

        addr = addr.wrapping_add(PAGE_SIZE);
        pte_val(entry) = pte_val(entry).wrapping_add(PAGE_SIZE);
        i += 1;
    }

    purge_tlb_entries_huge(mm, addr_start);
}

pub unsafe fn set_huge_pte_at(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    entry: pte_t,
    sz: c_ulong,
) {
    __set_huge_pte_at(mm, addr, ptep, entry);
}

pub unsafe fn huge_ptep_get_and_clear(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    sz: c_ulong,
) -> pte_t {
    let entry = *ptep;
    __set_huge_pte_at(mm, addr, ptep, __pte(0));
    entry
}

pub unsafe fn huge_ptep_set_wrprotect(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
) {
    let old_pte = *ptep;
    __set_huge_pte_at(mm, addr, ptep, pte_wrprotect(old_pte));
}

pub unsafe fn huge_ptep_set_access_flags(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    pte: pte_t,
    dirty: c_int,
) -> c_int {
    let mm = (*vma).vm_mm;
    let changed = (!pte_same(*ptep, pte)) as c_int;
    if changed != 0 {
        __set_huge_pte_at(mm, addr, ptep, pte);
    }
    changed
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
