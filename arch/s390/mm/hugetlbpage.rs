// SPDX-License-Identifier: GPL-2.0
/*
 *  IBM System z Huge TLB Page Support for Kernel.
 *
 *    Copyright IBM Corp. 2007,2020
 *    Author(s): Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

// #define pr_fmt(fmt) "hugetlb: " fmt
// Kernel dependencies supplied by the surrounding translation unit.

/*
 * If the bit selected by single-bit bitmask "a" is set within "x", move
 * it to the position indicated by single-bit bitmask "b".
 */
#[inline]
fn move_set_bit(x: c_ulong, a: c_ulong, b: c_ulong) -> c_ulong {
    ((x & a) >> ilog2(a) << ilog2(b))
}

#[inline]
unsafe fn __pte_to_rste(pte: pte_t) -> c_ulong {
    let arch_entry: swp_entry_t;
    let mut rste: c_ulong;

    /*
     * Convert encoding          pte bits  pmd / pud bits
     *                             lIR.uswrdy.p  dy..R...I...wr
     * empty                     010.000000.0 -> 00..0...1...00
     * prot-none, clean, old     111.000000.1 -> 00..1...1...00
     * prot-none, clean, young   111.000001.1 -> 01..1...1...00
     * prot-none, dirty, old     111.000010.1 -> 10..1...1...00
     * prot-none, dirty, young   111.000011.1 -> 11..1...1...00
     * read-only, clean, old     111.000100.1 -> 00..1...1...01
     * read-only, clean, young   101.000101.1 -> 01..1...0...01
     * read-only, dirty, old     111.000110.1 -> 10..1...1...01
     * read-only, dirty, young   101.000111.1 -> 11..1...0...01
     * read-write, clean, old    111.001100.1 -> 00..1...1...11
     * read-write, clean, young  101.001101.1 -> 01..1...0...11
     * read-write, dirty, old    110.001110.1 -> 10..0...1...11
     * read-write, dirty, young  100.001111.1 -> 11..0...0...11
     * HW-bits: R read-only, I invalid
     * SW-bits: p present, y young, d dirty, r read, w write, s special,
     *          u unused, l large
     */
    if pte_present(pte) {
        rste = pte_val(pte) & PAGE_MASK;
        rste |= _SEGMENT_ENTRY_PRESENT;
        rste |= move_set_bit(pte_val(pte), _PAGE_READ, _SEGMENT_ENTRY_READ);
        rste |= move_set_bit(pte_val(pte), _PAGE_WRITE, _SEGMENT_ENTRY_WRITE);
        rste |= move_set_bit(pte_val(pte), _PAGE_INVALID, _SEGMENT_ENTRY_INVALID);
        rste |= move_set_bit(pte_val(pte), _PAGE_PROTECT, _SEGMENT_ENTRY_PROTECT);
        rste |= move_set_bit(pte_val(pte), _PAGE_DIRTY, _SEGMENT_ENTRY_DIRTY);
        rste |= move_set_bit(pte_val(pte), _PAGE_YOUNG, _SEGMENT_ENTRY_YOUNG);
        #[cfg(CONFIG_MEM_SOFT_DIRTY)]
        { rste |= move_set_bit(pte_val(pte), _PAGE_SOFT_DIRTY, _SEGMENT_ENTRY_SOFT_DIRTY); }
        rste |= move_set_bit(pte_val(pte), _PAGE_NOEXEC, _SEGMENT_ENTRY_NOEXEC);
    } else if !pte_none(pte) {
        /* swap pte */
        arch_entry = __pte_to_swp_entry(pte);
        rste = mk_swap_rste(__swp_type(arch_entry), __swp_offset(arch_entry));
    } else {
        rste = _SEGMENT_ENTRY_EMPTY;
    }
    rste
}

#[inline]
unsafe fn __rste_to_pte(rste: c_ulong) -> pte_t {
    let arch_entry: swp_entry_t;
    let mut pteval: c_ulong;
    let present: c_int;
    let none: c_int;
    let pte: pte_t;

    if (rste & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3 {
        present = pud_present(__pud(rste));
        none = pud_none(__pud(rste));
    } else {
        present = pmd_present(__pmd(rste));
        none = pmd_none(__pmd(rste));
    }

    /* Encoding conversion; see the corresponding table in __pte_to_rste. */
    if present != 0 {
        pteval = rste & _SEGMENT_ENTRY_ORIGIN_LARGE;
        pteval |= _PAGE_LARGE | _PAGE_PRESENT;
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_READ, _PAGE_READ);
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_WRITE, _PAGE_WRITE);
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_INVALID, _PAGE_INVALID);
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_PROTECT, _PAGE_PROTECT);
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_DIRTY, _PAGE_DIRTY);
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_YOUNG, _PAGE_YOUNG);
        #[cfg(CONFIG_MEM_SOFT_DIRTY)]
        { pteval |= move_set_bit(rste, _SEGMENT_ENTRY_SOFT_DIRTY, _PAGE_SOFT_DIRTY); }
        pteval |= move_set_bit(rste, _SEGMENT_ENTRY_NOEXEC, _PAGE_NOEXEC);
    } else if none == 0 {
        /* swap rste */
        arch_entry = __rste_to_swp_entry(rste);
        pte = mk_swap_pte(__swp_type_rste(arch_entry), __swp_offset_rste(arch_entry));
        pteval = pte_val(pte);
    } else {
        pteval = _PAGE_INVALID;
    }
    __pte(pteval)
}

pub unsafe fn __set_huge_pte_at(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pte: pte_t) {
    let mut rste = __pte_to_rste(pte);
    if (pte_val(ptep_get(ptep)) & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3 {
        if likely(pte_present(pte)) { rste |= _REGION3_ENTRY_LARGE; }
        rste |= _REGION_ENTRY_TYPE_R3;
        set_pud(ptep as *mut pud_t, __pud(rste));
    } else {
        if likely(pte_present(pte)) { rste |= _SEGMENT_ENTRY_LARGE; }
        set_pmd(ptep as *mut pmd_t, __pmd(rste));
    }
}

pub unsafe fn set_huge_pte_at(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pte: pte_t, sz: c_ulong) {
    __set_huge_pte_at(mm, addr, ptep, pte);
}

pub unsafe fn huge_ptep_get(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t {
    __rste_to_pte(pte_val(ptep_get(ptep)))
}

pub unsafe fn __huge_ptep_get_and_clear(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t {
    let pte = huge_ptep_get(mm, addr, ptep);
    let pmdp = ptep as *mut pmd_t;
    let pudp = ptep as *mut pud_t;
    if (pte_val(ptep_get(ptep)) & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3 {
        pudp_xchg_direct(mm, addr, pudp, __pud(_REGION3_ENTRY_EMPTY));
    } else {
        pmdp_xchg_direct(mm, addr, pmdp, __pmd(_SEGMENT_ENTRY_EMPTY));
    }
    pte
}

pub unsafe fn huge_pte_alloc(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: c_ulong, sz: c_ulong) -> *mut pte_t {
    let pgdp = pgd_offset(mm, addr);
    let p4dp = p4d_alloc(mm, pgdp, addr);
    let mut pmdp: *mut pmd_t = core::ptr::null_mut();
    if !p4dp.is_null() {
        let pudp = pud_alloc(mm, p4dp, addr);
        if !pudp.is_null() {
            if sz == PUD_SIZE { return pudp as *mut pte_t; }
            if sz == PMD_SIZE { pmdp = pmd_alloc(mm, pudp, addr); }
        }
    }
    pmdp as *mut pte_t
}

pub unsafe fn huge_pte_offset(mm: *mut mm_struct, addr: c_ulong, sz: c_ulong) -> *mut pte_t {
    let pgdp = pgd_offset(mm, addr);
    let mut pmdp: *mut pmd_t = core::ptr::null_mut();
    if pgd_present(pgdp_get(pgdp)) {
        let p4dp = p4d_offset(pgdp, addr);
        if p4d_present(p4dp_get(p4dp)) {
            let pudp = pud_offset(p4dp, addr);
            if sz == PUD_SIZE { return pudp as *mut pte_t; }
            if pud_present(pudp_get(pudp)) { pmdp = pmd_offset(pudp, addr); }
        }
    }
    pmdp as *mut pte_t
}

pub unsafe fn arch_hugetlb_valid_size(size: c_ulong) -> bool {
    if cpu_has_edat1() && size == PMD_SIZE { true }
    else if cpu_has_edat2() && size == PUD_SIZE { true }
    else { false }
}

pub unsafe fn arch_hugetlb_cma_order() -> c_uint {
    if cpu_has_edat2() { PUD_SHIFT - PAGE_SHIFT } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
