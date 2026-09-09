// SPDX-License-Identifier: GPL-2.0
/*
 * SPARC64 Huge TLB page support.
 *
 * Copyright (C) 2002, 2003, 2006 David S. Miller (davem@davemloft.net)
 */

// Dependencies supplied by the Linux kernel headers are intentionally external.

unsafe fn sun4u_hugepage_shift_to_tte(mut entry: pte_t, shift: c_uint) -> pte_t {
    let mut hugepage_size: c_ulong = _PAGE_SZ4MB_4U;

    pte_val(entry) = pte_val(entry) & !_PAGE_SZALL_4U;

    match shift {
        HPAGE_256MB_SHIFT => {
            hugepage_size = _PAGE_SZ256MB_4U;
            pte_val(entry) |= _PAGE_PMD_HUGE;
        }
        HPAGE_SHIFT => {
            pte_val(entry) |= _PAGE_PMD_HUGE;
        }
        HPAGE_64K_SHIFT => {
            hugepage_size = _PAGE_SZ64K_4U;
        }
        _ => {
            WARN_ONCE(1, "unsupported hugepage shift=%u\n", shift);
        }
    }

    pte_val(entry) = pte_val(entry) | hugepage_size;
    entry
}

unsafe fn sun4v_hugepage_shift_to_tte(mut entry: pte_t, shift: c_uint) -> pte_t {
    let mut hugepage_size: c_ulong = _PAGE_SZ4MB_4V;

    pte_val(entry) = pte_val(entry) & !_PAGE_SZALL_4V;

    match shift {
        HPAGE_16GB_SHIFT => {
            hugepage_size = _PAGE_SZ16GB_4V;
            pte_val(entry) |= _PAGE_PUD_HUGE;
        }
        HPAGE_2GB_SHIFT => {
            hugepage_size = _PAGE_SZ2GB_4V;
            pte_val(entry) |= _PAGE_PMD_HUGE;
        }
        HPAGE_256MB_SHIFT => {
            hugepage_size = _PAGE_SZ256MB_4V;
            pte_val(entry) |= _PAGE_PMD_HUGE;
        }
        HPAGE_SHIFT => {
            pte_val(entry) |= _PAGE_PMD_HUGE;
        }
        HPAGE_64K_SHIFT => {
            hugepage_size = _PAGE_SZ64K_4V;
        }
        _ => {
            WARN_ONCE(1, "unsupported hugepage shift=%u\n", shift);
        }
    }

    pte_val(entry) = pte_val(entry) | hugepage_size;
    entry
}

unsafe fn hugepage_shift_to_tte(entry: pte_t, shift: c_uint) -> pte_t {
    if tlb_type == hypervisor {
        sun4v_hugepage_shift_to_tte(entry, shift)
    } else {
        sun4u_hugepage_shift_to_tte(entry, shift)
    }
}

pub unsafe fn arch_make_huge_pte(mut entry: pte_t, shift: c_uint, flags: vm_flags_t) -> pte_t {
    entry = pte_mkhuge(entry);
    let pte = hugepage_shift_to_tte(entry, shift);

    // If this vma has ADI enabled on it, turn on TTE.mcd.
    #[cfg(CONFIG_SPARC64)]
    {
        if flags & VM_SPARC_ADI != 0 {
            return pte_mkmcd(pte);
        } else {
            return pte_mknotmcd(pte);
        }
    }
    #[cfg(not(CONFIG_SPARC64))]
    {
        let _ = flags;
        pte
    }
}

unsafe fn sun4v_huge_tte_to_shift(entry: pte_t) -> c_uint {
    let tte_szbits = pte_val(entry) & _PAGE_SZALL_4V;
    match tte_szbits {
        _PAGE_SZ16GB_4V => HPAGE_16GB_SHIFT,
        _PAGE_SZ2GB_4V => HPAGE_2GB_SHIFT,
        _PAGE_SZ256MB_4V => HPAGE_256MB_SHIFT,
        _PAGE_SZ4MB_4V => REAL_HPAGE_SHIFT,
        _PAGE_SZ64K_4V => HPAGE_64K_SHIFT,
        _ => PAGE_SHIFT,
    }
}

unsafe fn sun4u_huge_tte_to_shift(entry: pte_t) -> c_uint {
    let tte_szbits = pte_val(entry) & _PAGE_SZALL_4U;
    match tte_szbits {
        _PAGE_SZ256MB_4U => HPAGE_256MB_SHIFT,
        _PAGE_SZ4MB_4U => REAL_HPAGE_SHIFT,
        _PAGE_SZ64K_4U => HPAGE_64K_SHIFT,
        _ => PAGE_SHIFT,
    }
}

unsafe fn tte_to_shift(entry: pte_t) -> c_ulong {
    if tlb_type == hypervisor {
        sun4v_huge_tte_to_shift(entry) as c_ulong
    } else {
        sun4u_huge_tte_to_shift(entry) as c_ulong
    }
}

unsafe fn huge_tte_to_shift(entry: pte_t) -> c_uint {
    let shift = tte_to_shift(entry);
    if shift == PAGE_SHIFT as c_ulong {
        WARN_ONCE(1, "tto_to_shift: invalid hugepage tte=0x%lx\n", pte_val(entry));
    }
    shift as c_uint
}

unsafe fn huge_tte_to_size(pte: pte_t) -> c_ulong {
    let mut size = 1 as c_ulong << huge_tte_to_shift(pte);
    if size == REAL_HPAGE_SIZE {
        size = HPAGE_SIZE;
    }
    size
}

pub unsafe fn pud_leaf_size(pud: pud_t) -> c_ulong {
    1 as c_ulong << tte_to_shift(*( &pud as *const pud_t as *const pte_t))
}
pub unsafe fn pmd_leaf_size(pmd: pmd_t) -> c_ulong {
    1 as c_ulong << tte_to_shift(*( &pmd as *const pmd_t as *const pte_t))
}
pub unsafe fn pte_leaf_size(pte: pte_t) -> c_ulong {
    1 as c_ulong << tte_to_shift(pte)
}

pub unsafe fn huge_pte_alloc(mm: *mut mm_struct, vma: *mut vm_area_struct, addr: c_ulong, sz: c_ulong) -> *mut pte_t {
    let pgd = pgd_offset(mm, addr);
    let p4d = p4d_offset(pgd, addr);
    let pud = pud_alloc(mm, p4d, addr);
    if pud.is_null() { return core::ptr::null_mut(); }
    if sz >= PUD_SIZE { return pud as *mut pte_t; }
    let pmd = pmd_alloc(mm, pud, addr);
    if pmd.is_null() { return core::ptr::null_mut(); }
    if sz >= PMD_SIZE { return pmd as *mut pte_t; }
    pte_alloc_huge(mm, pmd, addr)
}

pub unsafe fn huge_pte_offset(mm: *mut mm_struct, addr: c_ulong, _sz: c_ulong) -> *mut pte_t {
    let pgd = pgd_offset(mm, addr);
    if pgd_none(*pgd) { return core::ptr::null_mut(); }
    let p4d = p4d_offset(pgd, addr);
    if p4d_none(*p4d) { return core::ptr::null_mut(); }
    let pud = pud_offset(p4d, addr);
    if pud_none(*pud) { return core::ptr::null_mut(); }
    if is_hugetlb_pud(*pud) { return pud as *mut pte_t; }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(*pmd) { return core::ptr::null_mut(); }
    if is_hugetlb_pmd(*pmd) { return pmd as *mut pte_t; }
    pte_offset_huge(pmd, addr)
}

pub unsafe fn __set_huge_pte_at(mm: *mut mm_struct, mut addr: c_ulong, ptep: *mut pte_t, entry: pte_t) {
    let size = huge_tte_to_size(entry);
    let shift = if size >= PUD_SIZE { PUD_SHIFT } else if size >= PMD_SIZE { PMD_SHIFT } else { PAGE_SHIFT };
    let nptes = size >> shift;
    if !pte_present(*ptep) && pte_present(entry) { (*mm).context.hugetlb_pte_count += nptes; }
    addr &= !(size - 1);
    let orig = *ptep;
    let orig_shift = if pte_none(orig) { PAGE_SHIFT } else { huge_tte_to_shift(orig) };
    for i in 0..nptes { *ptep.add(i as usize) = __pte(pte_val(entry) + (i << shift)); }
    maybe_tlb_batch_add(mm, addr, ptep, orig, 0, orig_shift);
    if size == HPAGE_SIZE { maybe_tlb_batch_add(mm, addr + REAL_HPAGE_SIZE, ptep, orig, 0, orig_shift); }
}

pub unsafe fn set_huge_pte_at(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, entry: pte_t, _sz: c_ulong) {
    __set_huge_pte_at(mm, addr, ptep, entry);
}

pub unsafe fn huge_ptep_get_and_clear(mm: *mut mm_struct, mut addr: c_ulong, ptep: *mut pte_t, _sz: c_ulong) -> pte_t {
    let entry = *ptep;
    let size = huge_tte_to_size(entry);
    let shift = if size >= PUD_SIZE { PUD_SHIFT } else if size >= PMD_SIZE { PMD_SHIFT } else { PAGE_SHIFT };
    let nptes = size >> shift;
    let orig_shift = if pte_none(entry) { PAGE_SHIFT } else { huge_tte_to_shift(entry) };
    if pte_present(entry) { (*mm).context.hugetlb_pte_count -= nptes; }
    addr &= !(size - 1);
    for i in 0..nptes { *ptep.add(i as usize) = __pte(0 as c_ulong); }
    maybe_tlb_batch_add(mm, addr, ptep, entry, 0, orig_shift);
    if size == HPAGE_SIZE { maybe_tlb_batch_add(mm, addr + REAL_HPAGE_SIZE, ptep, entry, 0, orig_shift); }
    entry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
