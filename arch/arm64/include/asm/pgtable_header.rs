/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arm64/include/asm/pgtable.h. */

// The included C header supplies the architecture-specific types, constants,
// and operations referenced below.  Configuration guards are retained as
// comments because their values are selected by the kernel build.

pub const HUGE_MAX_HSTATE: usize = 4;
pub const __SWP_TYPE_SHIFT: usize = 6;
pub const __SWP_TYPE_BITS: usize = 5;
pub const __SWP_TYPE_MASK: usize = (1usize << __SWP_TYPE_BITS) - 1;
pub const __SWP_OFFSET_SHIFT: usize = 12;
pub const __SWP_OFFSET_BITS: usize = 50;
pub const __SWP_OFFSET_MASK: usize = (1usize << __SWP_OFFSET_BITS) - 1;

extern "C" {
    pub fn emit_pte_barriers();
    pub fn queue_pte_barriers();
    pub fn __sync_icache_dcache(pteval: pteval_t);
    pub fn pgattr_change_is_safe(old: pteval_t, new: pteval_t) -> bool;
    pub fn arch_prepare_to_swap(folio: *mut folio) -> i32;
    pub fn arch_swap_restore(entry: swp_entry_t, folio: *mut folio);
}

#[inline]
pub unsafe fn pte_pfn(pte: pte_t) -> usize { __pte_to_phys(pte) >> PAGE_SHIFT }
#[inline]
pub unsafe fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t {
    __pte(__phys_to_pte_val((pfn << PAGE_SHIFT) as phys_addr_t) | pgprot_val(prot))
}
#[inline] pub unsafe fn pte_none(pte: pte_t) -> bool { pte_val(pte) == 0 }
#[inline] pub unsafe fn pte_valid(pte: pte_t) -> bool { pte_val(pte) & PTE_VALID != 0 }
#[inline] pub unsafe fn pte_present_invalid(pte: pte_t) -> bool {
    pte_val(pte) & (PTE_VALID | PTE_PRESENT_INVALID) == PTE_PRESENT_INVALID
}
#[inline] pub unsafe fn pte_present(pte: pte_t) -> bool { pte_valid(pte) || pte_present_invalid(pte) }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> bool { pte_val(pte) & PTE_AF != 0 }
#[inline] pub unsafe fn pte_special(pte: pte_t) -> bool { pte_val(pte) & PTE_SPECIAL != 0 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> bool { pte_val(pte) & PTE_WRITE != 0 }
#[inline] pub unsafe fn pte_rdonly(pte: pte_t) -> bool { pte_val(pte) & PTE_RDONLY != 0 }
#[inline] pub unsafe fn pte_user(pte: pte_t) -> bool { pte_val(pte) & PTE_USER != 0 }
#[inline] pub unsafe fn pte_user_exec(pte: pte_t) -> bool { pte_val(pte) & PTE_UXN == 0 }
#[inline] pub unsafe fn pte_cont(pte: pte_t) -> bool { pte_val(pte) & PTE_CONT != 0 }
#[inline] pub unsafe fn pte_sw_dirty(pte: pte_t) -> bool { pte_val(pte) & PTE_DIRTY != 0 }
#[inline] pub unsafe fn pte_hw_dirty(pte: pte_t) -> bool { pte_write(pte) && !pte_rdonly(pte) }
#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> bool { pte_sw_dirty(pte) || pte_hw_dirty(pte) }
#[inline] pub unsafe fn pte_valid_not_user(pte: pte_t) -> bool {
    pte_val(pte) & (PTE_VALID | PTE_USER | PTE_UXN) == (PTE_VALID | PTE_UXN)
}

#[inline] pub unsafe fn clear_pte_bit(mut pte: pte_t, prot: pgprot_t) -> pte_t {
    pte_val(pte) &= !pgprot_val(prot); pte
}
#[inline] pub unsafe fn set_pte_bit(mut pte: pte_t, prot: pgprot_t) -> pte_t {
    pte_val(pte) |= pgprot_val(prot); pte
}
#[inline] pub unsafe fn pte_mkold(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(PTE_AF)) }
#[inline] pub unsafe fn pte_mkyoung(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(PTE_AF)) }
#[inline] pub unsafe fn pte_mkspecial(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(PTE_SPECIAL)) }
#[inline] pub unsafe fn pte_mkcont(pte: pte_t) -> pte_t { set_pte_bit(pte, __pgprot(PTE_CONT)) }
#[inline] pub unsafe fn pte_mknoncont(pte: pte_t) -> pte_t { clear_pte_bit(pte, __pgprot(PTE_CONT)) }
#[inline] pub unsafe fn pte_mkvalid_k(mut pte: pte_t) -> pte_t {
    pte = clear_pte_bit(pte, __pgprot(PTE_PRESENT_INVALID));
    set_pte_bit(pte, __pgprot(PTE_PRESENT_VALID_KERNEL))
}
#[inline] pub unsafe fn pte_mkinvalid(mut pte: pte_t) -> pte_t {
    pte = set_pte_bit(pte, __pgprot(PTE_PRESENT_INVALID));
    clear_pte_bit(pte, __pgprot(PTE_VALID))
}

#[inline] pub unsafe fn __set_pte_nosync(ptep: *mut pte_t, pte: pte_t) { core::ptr::write_volatile(ptep, pte); }
#[inline] pub unsafe fn __ptep_get(ptep: *const pte_t) -> pte_t { core::ptr::read_volatile(ptep) }
#[inline] pub unsafe fn __set_pte(ptep: *mut pte_t, pte: pte_t) { __set_pte_nosync(ptep, pte); if pte_valid_not_user(pte) { queue_pte_barriers(); } }

#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> bool { pte_present(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_valid(pmd: pmd_t) -> bool { pte_valid(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_dirty(pmd: pmd_t) -> bool { pte_dirty(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_young(pmd: pmd_t) -> bool { pte_young(pmd_pte(pmd)) }
#[inline] pub unsafe fn pmd_write(pmd: pmd_t) -> bool { pte_write(pmd_pte(pmd)) }

#[inline] pub unsafe fn __pte_to_phys(pte: pte_t) -> phys_addr_t { pte_val(pte) & PTE_ADDR_LOW }
#[inline] pub unsafe fn __phys_to_pte_val(phys: phys_addr_t) -> pteval_t { phys }
#[inline] pub unsafe fn __pmd_to_phys(pmd: pmd_t) -> phys_addr_t { __pte_to_phys(pmd_pte(pmd)) }
#[inline] pub unsafe fn __pud_to_phys(pud: pud_t) -> phys_addr_t { __pte_to_phys(pud_pte(pud)) }
#[inline] pub unsafe fn __pgd_to_phys(pgd: pgd_t) -> phys_addr_t { __pte_to_phys(pgd_pte(pgd)) }

#[inline] pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { (__pmd_to_phys(pmd) & PMD_MASK) >> PAGE_SHIFT }
#[inline] pub unsafe fn pud_pfn(pud: pud_t) -> usize { (__pud_to_phys(pud) & PUD_MASK) >> PAGE_SHIFT }
#[inline] pub unsafe fn pfn_pmd(pfn: usize, prot: pgprot_t) -> pmd_t { __pmd(__phys_to_pte_val((pfn << PAGE_SHIFT) as phys_addr_t) | pgprot_val(prot)) }
#[inline] pub unsafe fn pfn_pud(pfn: usize, prot: pgprot_t) -> pud_t { __pud(__phys_to_pte_val((pfn << PAGE_SHIFT) as phys_addr_t) | pgprot_val(prot)) }

// The remaining architecture hooks are declaration-only interfaces from the
// original header; their implementations and configuration-dependent aliases
// are supplied by the surrounding kernel translation units.
extern "C" {
    pub fn __ptep_set_access_flags_anysz(vma: *mut vm_area_struct, address: usize, ptep: *mut pte_t, entry: pte_t, dirty: i32, pgsize: usize) -> i32;
    pub fn ptep_modify_prot_start(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t) -> pte_t;
    pub fn ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: usize, ptep: *mut pte_t, old_pte: pte_t, new_pte: pte_t);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
