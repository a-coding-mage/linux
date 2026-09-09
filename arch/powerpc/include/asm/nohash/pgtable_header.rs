/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PPC64 selects asm/nohash/64/pgtable.h; otherwise asm/nohash/32/pgtable.h. */

#[cfg(not(assembler))]
extern "C" {
    static mut icache_44x_need_flush: ::core::ffi::c_int;
}

pub const _PAGE_CHG_MASK: pte_basic_t = PTE_RPN_MASK | _PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_SPECIAL;

pub const PAGE_KERNEL: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_KERNEL_RW);
pub const PAGE_KERNEL_NC: pgprot_t = __pgprot(_PAGE_BASE_NC | _PAGE_KERNEL_RW | _PAGE_NO_CACHE);
pub const PAGE_KERNEL_NCG: pgprot_t = __pgprot(_PAGE_BASE_NC | _PAGE_KERNEL_RW | _PAGE_NO_CACHE | _PAGE_GUARDED);
pub const PAGE_KERNEL_X: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_KERNEL_RWX);
pub const PAGE_KERNEL_RO: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_KERNEL_RO);
pub const PAGE_KERNEL_ROX: pgprot_t = __pgprot(_PAGE_BASE | _PAGE_KERNEL_ROX);

#[inline]
pub unsafe fn pte_huge_size(_pte: pte_t) -> ::core::ffi::c_ulong {
    PAGE_SIZE
}

#[inline]
pub unsafe fn pte_update(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, p: *mut pte_t,
                         clr: ::core::ffi::c_ulong, set: ::core::ffi::c_ulong,
                         huge: ::core::ffi::c_int) -> pte_basic_t {
    let old = pte_val(*p);
    let mut new = (old & !(clr as pte_basic_t)) | (set as pte_basic_t);
    let sz: ::core::ffi::c_ulong;
    let pdsize: ::core::ffi::c_ulong;
    if new == old { return old; }
    if huge != 0 { sz = pte_huge_size(__pte(old)); } else { sz = PAGE_SIZE; }
    if sz < PMD_SIZE { pdsize = PAGE_SIZE; }
    else if sz < PUD_SIZE { pdsize = PMD_SIZE; }
    else if sz < P4D_SIZE { pdsize = PUD_SIZE; }
    else if sz < PGDIR_SIZE { pdsize = P4D_SIZE; }
    else { pdsize = PGDIR_SIZE; }
    let mut i = 0;
    while i < sz / pdsize {
        *p = __pte(new);
        if new != 0 { new = new.wrapping_add(((pdsize / PAGE_SIZE) << PTE_RPN_SHIFT) as pte_basic_t); }
        p = p.add(1);
        i += 1;
    }
    if cfg!(CONFIG_44x) && !is_kernel_addr(addr) && (old & _PAGE_EXEC) != 0 {
        icache_44x_need_flush = 1;
    }
    if huge == 0 { assert_pte_locked(mm, addr); }
    old
}

#[inline]
pub unsafe fn ptep_test_and_clear_young(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong,
                                        ptep: *mut pte_t) -> bool {
    (pte_update((*vma).vm_mm, addr, ptep, _PAGE_ACCESSED, 0, 0) & _PAGE_ACCESSED) != 0
}

#[inline]
pub unsafe fn ptep_set_wrprotect(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t) {
    pte_update(mm, addr, ptep, _PAGE_WRITE, 0, 0);
}

#[inline]
pub unsafe fn ptep_get_and_clear(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t) -> pte_t {
    let old_pte = __pte(pte_update(mm, addr, ptep, !0, 0, 0));
    page_table_check_pte_clear(mm, addr, old_pte);
    old_pte
}

#[inline]
pub unsafe fn pte_clear(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t) {
    pte_update(mm, addr, ptep, !0, 0, 0);
}

#[inline]
pub unsafe fn __ptep_set_access_flags(vma: *mut vm_area_struct, ptep: *mut pte_t, entry: pte_t,
                                      address: ::core::ffi::c_ulong, psize: ::core::ffi::c_int) {
    let set = pte_val(entry) & (_PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_RW | _PAGE_EXEC);
    let huge = if psize > mmu_virtual_psize { 1 } else { 0 };
    pte_update((*vma).vm_mm, address, ptep, 0, set, huge);
    flush_tlb_page(vma, address);
}

#[inline] pub fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_RW) }
#[inline] pub fn pte_mkdirty(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_DIRTY) }
#[inline] pub fn pte_mkyoung(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_ACCESSED) }
#[inline] pub fn pte_wrprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_WRITE) }
#[inline] pub fn pte_mkexec(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_EXEC) }
#[inline] pub fn pte_write(pte: pte_t) -> ::core::ffi::c_int { (pte_val(pte) & _PAGE_WRITE) as _ }
#[inline] pub fn pte_dirty(pte: pte_t) -> ::core::ffi::c_int { (pte_val(pte) & _PAGE_DIRTY) as _ }
#[inline] pub fn pte_special(pte: pte_t) -> ::core::ffi::c_int { (pte_val(pte) & _PAGE_SPECIAL) as _ }
#[inline] pub fn pte_none(pte: pte_t) -> ::core::ffi::c_int { (((pte_val(pte) & !_PTE_NONE_MASK) == 0) as ::core::ffi::c_int) }
#[inline] pub fn pte_hashpte(_pte: pte_t) -> bool { false }
#[inline] pub fn pte_ci(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_NO_CACHE) != 0 }
#[inline] pub fn pte_exec(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_EXEC) != 0 }
#[inline] pub fn pte_present(pte: pte_t) -> ::core::ffi::c_int { (pte_val(pte) & _PAGE_PRESENT) as _ }
#[inline] pub fn pte_hw_valid(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_PRESENT) != 0 }
#[inline] pub fn pte_young(pte: pte_t) -> ::core::ffi::c_int { (pte_val(pte) & _PAGE_ACCESSED) as _ }
#[inline] pub fn pte_read(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_READ) == _PAGE_READ }

#[inline]
pub unsafe fn pte_access_permitted(pte: pte_t, write: bool) -> bool {
    if pte_present(pte) == 0 || !pte_read(pte) { return false; }
    if write && pte_write(pte) == 0 { return false; }
    true
}

#[inline] pub unsafe fn pte_user_accessible_page(_mm: *mut mm_struct, addr: ::core::ffi::c_ulong, pte: pte_t) -> bool { pte_present(pte) != 0 && !is_kernel_addr(addr) }
#[inline] pub fn pfn_pte(pfn: ::core::ffi::c_ulong, pgprot: pgprot_t) -> pte_t { __pte(((pfn as pte_basic_t) << PTE_RPN_SHIFT) | pgprot_val(pgprot)) }
#[inline] pub fn pte_exprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_EXEC) }
#[inline] pub fn pte_mkclean(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_DIRTY) }
#[inline] pub fn pte_mkold(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_ACCESSED) }
#[inline] pub fn pte_mkspecial(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SPECIAL) }
#[inline] pub fn pte_mkhuge(pte: pte_t) -> pte_t { __pte(pte_val(pte)) }
#[inline] pub fn pte_modify(pte: pte_t, newprot: pgprot_t) -> pte_t { __pte((pte_val(pte) & _PAGE_CHG_MASK) | pgprot_val(newprot)) }
#[inline] pub fn pte_swp_exclusive(pte: pte_t) -> bool { (pte_val(pte) & _PAGE_SWP_EXCLUSIVE) != 0 }
#[inline] pub fn pte_swp_mkexclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_SWP_EXCLUSIVE) }
#[inline] pub fn pte_swp_clear_exclusive(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !_PAGE_SWP_EXCLUSIVE) }

#[inline]
pub unsafe fn __set_pte_at(mm: *mut mm_struct, addr: ::core::ffi::c_ulong, ptep: *mut pte_t, pte: pte_t, percpu: ::core::ffi::c_int) {
    let _ = mm;
    /* The PPC32/PTE_64BIT non-percpu case uses the source's ordered two-half assembly store. */
    if cfg!(CONFIG_PPC32) && cfg!(CONFIG_PTE_64BIT) && percpu == 0 {
        *ptep = pte;
        return;
    }
    *ptep = pte;
    if cfg!(CONFIG_PPC_BOOK3E_64) && is_kernel_addr(addr) { mb(); }
}

pub const _PAGE_CACHE_CTL: pte_basic_t = _PAGE_COHERENT | _PAGE_GUARDED | _PAGE_NO_CACHE | _PAGE_WRITETHRU;
#[inline] pub fn pgprot_noncached(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE | _PAGE_GUARDED) }
#[inline] pub fn pgprot_noncached_wc(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_NO_CACHE) }
#[inline] pub fn pgprot_cached(prot: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_COHERENT) }
#[inline] pub fn pgprot_cached_wthru(prot: pgprot_t) -> pgprot_t { if _PAGE_WRITETHRU != 0 { __pgprot((pgprot_val(prot) & !_PAGE_CACHE_CTL) | _PAGE_COHERENT | _PAGE_WRITETHRU) } else { pgprot_noncached(prot) } }
#[inline] pub fn pgprot_cached_noncoherent(prot: pgprot_t) -> pgprot_t { __pgprot(pgprot_val(prot) & !_PAGE_CACHE_CTL) }
pub use pgprot_noncached_wc as pgprot_writecombine;

extern "C" {
    fn map_kernel_page(va: ::core::ffi::c_ulong, pa: phys_addr_t, prot: pgprot_t) -> ::core::ffi::c_int;
    fn unmap_kernel_page(va: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
