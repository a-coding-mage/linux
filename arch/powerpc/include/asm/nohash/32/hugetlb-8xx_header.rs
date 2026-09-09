/* SPDX-License-Identifier: GPL-2.0 */

pub const PAGE_SHIFT_8M: usize = 23;

pub unsafe fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    flush_tlb_page(vma, vmaddr);
}

pub unsafe fn check_and_get_huge_psize(shift: i32) -> i32 {
    shift_to_mmu_psize(shift)
}

pub const __HAVE_ARCH_HUGE_SET_HUGE_PTE_AT: bool = true;

unsafe extern "C" {
    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: usize,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: usize,
    );
}

pub const __HAVE_ARCH_HUGE_PTEP_GET: bool = true;

pub unsafe fn huge_ptep_get(
    mm: *mut mm_struct,
    addr: usize,
    mut ptep: *mut pte_t,
) -> pte_t {
    if ptep_is_8m_pmdp(mm, addr, ptep) {
        ptep = pte_offset_kernel(
            ptep as *mut pmd_t,
            ALIGN_DOWN(addr, SZ_8M),
        );
    }
    ptep_get(ptep)
}

pub const __HAVE_ARCH_HUGE_PTE_CLEAR: bool = true;

pub unsafe fn huge_pte_clear(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
    _sz: usize,
) {
    pte_update(mm, addr, ptep, usize::MAX, 0, 1);
}

pub const __HAVE_ARCH_HUGE_PTEP_SET_WRPROTECT: bool = true;

pub unsafe fn huge_ptep_set_wrprotect(
    mm: *mut mm_struct,
    addr: usize,
    ptep: *mut pte_t,
) {
    let clr: usize = !pte_val(pte_wrprotect(__pte(usize::MAX)));
    let set: usize = pte_val(pte_wrprotect(__pte(0)));

    pte_update(mm, addr, ptep, clr, set, 1);
}

/* Corresponds to #ifdef CONFIG_PPC_4K_PAGES. */
#[cfg(CONFIG_PPC_4K_PAGES)]
pub unsafe fn arch_make_huge_pte(
    entry: pte_t,
    shift: u32,
    _flags: vm_flags_t,
) -> pte_t {
    let size: usize = 1usize << shift;

    if size == SZ_16K {
        __pte(pte_val(entry) | _PAGE_SPS)
    } else {
        __pte(pte_val(entry) | _PAGE_SPS | _PAGE_HUGE)
    }
}

#[cfg(CONFIG_PPC_4K_PAGES)]
pub use arch_make_huge_pte as arch_make_huge_pte;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
