/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of alpha/include/asm/pgtable.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub unsafe fn set_pte(pteptr: *mut pte_t, pteval: pte_t) { *pteptr = pteval; }

pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - 3);
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);
pub const PGDIR_SHIFT: usize = PAGE_SHIFT + 2 * (PAGE_SHIFT - 3);
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);
pub const PTRS_PER_PTE: usize = 1usize << (PAGE_SHIFT - 3);
pub const PTRS_PER_PMD: usize = 1usize << (PAGE_SHIFT - 3);
pub const PTRS_PER_PGD: usize = 1usize << (PAGE_SHIFT - 3);
pub const USER_PTRS_PER_PGD: usize = TASK_SIZE / PGDIR_SIZE;
pub const PTRS_PER_PAGE: usize = 1usize << (PAGE_SHIFT - 3);

/* CONFIG_ALPHA_LARGE_VMALLOC selects the alternate VMALLOC_START value. */
#[cfg(CONFIG_ALPHA_LARGE_VMALLOC)]
pub const VMALLOC_START: u64 = 0xfffffe0000000000;
#[cfg(not(CONFIG_ALPHA_LARGE_VMALLOC))]
pub const VMALLOC_START: usize = (-2isize * PGDIR_SIZE as isize) as usize;
pub const VMALLOC_END: usize = (-PGDIR_SIZE as isize) as usize;

pub const _PAGE_VALID: u64 = 0x0001;
pub const _PAGE_FOR: u64 = 0x0002;
pub const _PAGE_FOW: u64 = 0x0004;
pub const _PAGE_FOE: u64 = 0x0008;
pub const _PAGE_ASM: u64 = 0x0010;
pub const _PAGE_KRE: u64 = 0x0100;
pub const _PAGE_URE: u64 = 0x0200;
pub const _PAGE_KWE: u64 = 0x1000;
pub const _PAGE_UWE: u64 = 0x2000;
pub const _PAGE_DIRTY: u64 = 0x20000;
pub const _PAGE_ACCESSED: u64 = 0x40000;
pub const _PAGE_SWP_EXCLUSIVE: u64 = 0x8000000000;
pub const __DIRTY_BITS: u64 = _PAGE_DIRTY | _PAGE_KWE | _PAGE_UWE;
pub const __ACCESS_BITS: u64 = _PAGE_ACCESSED | _PAGE_KRE | _PAGE_URE;
pub const _PFN_MASK: u64 = 0xFFFFFFFF00000000;
pub const _PAGE_TABLE: u64 = _PAGE_VALID | __DIRTY_BITS | __ACCESS_BITS;
pub const _PAGE_CHG_MASK: u64 = _PFN_MASK | __DIRTY_BITS | __ACCESS_BITS;

pub const fn PAGE_NONE() -> pgprot_t { __pgprot(_PAGE_VALID | __ACCESS_BITS | _PAGE_FOR | _PAGE_FOW | _PAGE_FOE) }
pub const fn PAGE_SHARED() -> pgprot_t { __pgprot(_PAGE_VALID | __ACCESS_BITS) }
pub const fn PAGE_COPY() -> pgprot_t { __pgprot(_PAGE_VALID | __ACCESS_BITS | _PAGE_FOW) }
pub const fn PAGE_READONLY() -> pgprot_t { __pgprot(_PAGE_VALID | __ACCESS_BITS | _PAGE_FOW) }
pub const fn PAGE_KERNEL() -> pgprot_t { __pgprot(_PAGE_VALID | _PAGE_ASM | _PAGE_KRE | _PAGE_KWE) }
pub const fn _PAGE_NORMAL(x: u64) -> pgprot_t { __pgprot(_PAGE_VALID | __ACCESS_BITS | x) }
pub const fn _PAGE_P(x: u64) -> pgprot_t { _PAGE_NORMAL(x | _PAGE_FOW) }
pub const fn _PAGE_S(x: u64) -> pgprot_t { _PAGE_NORMAL(x) }

pub const fn pgprot_noncached(prot: pgprot_t) -> pgprot_t { prot }
pub fn pgprot_modify(_oldprot: pgprot_t, newprot: pgprot_t) -> pgprot_t { newprot }

/* The generic/EV6 configuration selects the physical-address twiddle. */
#[cfg(any(CONFIG_ALPHA_GENERIC, all(CONFIG_ALPHA_EV6, not(USE_48_BIT_KSEG))))]
pub const KSEG_PFN: u64 = 0xc0000000000u64 >> PAGE_SHIFT;
#[cfg(any(CONFIG_ALPHA_GENERIC, all(CONFIG_ALPHA_EV6, not(USE_48_BIT_KSEG))))]
pub fn PHYS_TWIDDLE(mut pfn: u64) -> u64 {
    if (pfn & KSEG_PFN) == (0x40000000000u64 >> PAGE_SHIFT) { pfn ^= KSEG_PFN; }
    pfn
}
#[cfg(not(any(CONFIG_ALPHA_GENERIC, all(CONFIG_ALPHA_EV6, not(USE_48_BIT_KSEG)))))]
pub const fn PHYS_TWIDDLE(pfn: u64) -> u64 { pfn }

pub fn page_to_pa(page: *const core::ffi::c_void) -> u64 { page_to_pfn(page) << PAGE_SHIFT }
pub const PFN_PTE_SHIFT: u32 = 32;
pub fn pte_pfn(pte: pte_t) -> u64 { pte_val(pte) >> PFN_PTE_SHIFT }
pub fn pte_page(pte: pte_t) -> *mut page { pfn_to_page(pte_pfn(pte)) }

pub fn pfn_pte(physpfn: u64, pgprot: pgprot_t) -> pte_t {
    pte_t { val: (PHYS_TWIDDLE(physpfn) << 32) | pgprot_val(pgprot) }
}
pub fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte.val = (pte.val & _PAGE_CHG_MASK) | pgprot_val(newprot); pte
}
pub unsafe fn pmd_set(pmdp: *mut pmd_t, ptep: *mut pte_t) {
    (*pmdp).val = _PAGE_TABLE | (((ptep as u64) - PAGE_OFFSET) << (32 - PAGE_SHIFT));
}
pub unsafe fn pud_set(pudp: *mut pud_t, pmdp: *mut pmd_t) {
    (*pudp).val = _PAGE_TABLE | (((pmdp as u64) - PAGE_OFFSET) << (32 - PAGE_SHIFT));
}

extern "C" { pub fn migrate_flush_tlb_page(vma: *mut vm_area_struct, addr: u64); }

pub fn pmd_page_vaddr(pmd: pmd_t) -> u64 { ((pmd.val & _PFN_MASK) >> (32 - PAGE_SHIFT)) + PAGE_OFFSET }
pub fn pmd_pfn(pmd: pmd_t) -> u64 { pmd.val >> 32 }
pub fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd.val >> 32) }
pub fn pud_page(pud: pud_t) -> *mut page { pfn_to_page(pud.val >> 32) }
pub fn pud_pgtable(pgd: pud_t) -> *mut pmd_t { (PAGE_OFFSET + ((pgd.val & _PFN_MASK) >> (32 - PAGE_SHIFT))) as *mut pmd_t }

pub fn pte_none(pte: pte_t) -> i32 { (pte.val == 0) as i32 }
pub fn pte_present(pte: pte_t) -> i32 { (pte.val & _PAGE_VALID) as i32 }
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: u64, ptep: *mut pte_t) { core::ptr::write_volatile(&mut (*ptep).val, 0); }
pub fn pmd_none(pmd: pmd_t) -> i32 { (pmd.val == 0) as i32 }
pub fn pmd_bad(pmd: pmd_t) -> i32 { ((pmd.val & !_PFN_MASK) != _PAGE_TABLE) as i32 }
pub fn pmd_present(pmd: pmd_t) -> i32 { (pmd.val & _PAGE_VALID) as i32 }
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { (*pmdp).val = 0; }
pub fn pud_none(pud: pud_t) -> i32 { (pud.val == 0) as i32 }
pub fn pud_bad(pud: pud_t) -> i32 { ((pud.val & !_PFN_MASK) != _PAGE_TABLE) as i32 }
pub fn pud_present(pud: pud_t) -> i32 { (pud.val & _PAGE_VALID) as i32 }
pub unsafe fn pud_clear(pudp: *mut pud_t) { (*pudp).val = 0; }

pub fn pte_write(pte: pte_t) -> i32 { (!(pte.val & _PAGE_FOW) != 0) as i32 }
pub fn pte_dirty(pte: pte_t) -> i32 { (pte.val & _PAGE_DIRTY) as i32 }
pub fn pte_young(pte: pte_t) -> i32 { (pte.val & _PAGE_ACCESSED) as i32 }
pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_FOW; pte }
pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte.val &= !__DIRTY_BITS; pte }
pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte.val &= !__ACCESS_BITS; pte }
pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_FOW; pte }
pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte.val |= __DIRTY_BITS; pte }
pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte.val |= __ACCESS_BITS; pte }

pub unsafe fn pmd_offset(dir: *mut pud_t, address: u64) -> *mut pmd_t {
    let ret = pud_pgtable(*dir).add(((address >> PMD_SHIFT) as usize) & (PTRS_PER_PAGE - 1)); smp_rmb(); ret
}
pub unsafe fn pte_offset_kernel(dir: *mut pmd_t, address: u64) -> *mut pte_t {
    let ret = (pmd_page_vaddr(*dir) as *mut pte_t).add(((address >> PAGE_SHIFT) as usize) & (PTRS_PER_PAGE - 1)); smp_rmb(); ret
}

extern "C" { pub static mut swapper_pg_dir: [pgd_t; 1024]; }

pub fn mk_swap_pte(typ: u64, offset: u64) -> pte_t { pte_t { val: ((typ & 0x7f) << 32) | (offset << 40) } }
pub fn __swp_type(x: swp_entry_t) -> u64 { (x.val >> 32) & 0x7f }
pub fn __swp_offset(x: swp_entry_t) -> u64 { x.val >> 40 }
pub fn __swp_entry(typ: u64, off: u64) -> swp_entry_t { swp_entry_t { val: mk_swap_pte(typ, off).val } }
pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte.val } }
pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { pte_t { val: x.val } }
pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte.val & _PAGE_SWP_EXCLUSIVE != 0 }
pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_SWP_EXCLUSIVE; pte }
pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_SWP_EXCLUSIVE; pte }

extern "C" { pub fn paging_init(); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
