/* SPDX-License-Identifier: GPL-2.0-only */
/* Page table support for the Hexagon architecture. */
/* C dependencies: asm/page.h, asm-generic/pgtable-nopmd.h, asm/vm_mmu.h. */

pub const _PAGE_READ: usize = __HVM_PTE_R;
pub const _PAGE_WRITE: usize = __HVM_PTE_W;
pub const _PAGE_EXECUTE: usize = __HVM_PTE_X;
pub const _PAGE_USER: usize = __HVM_PTE_U;
pub const _PAGE_PRESENT: usize = 1 << 0;
pub const _PAGE_DIRTY: usize = 1 << 1;
pub const _PAGE_ACCESSED: usize = 1 << 2;
pub const _PAGE_VALID: usize = _PAGE_PRESENT;
pub const _PAGE_SWP_EXCLUSIVE: usize = 1 << 6;
pub const PGDIR_SHIFT: usize = 22;
pub const PTRS_PER_PGD: usize = 1024;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/* PTRS_PER_PTE is selected by the build-time page-size configuration. */
#[cfg(CONFIG_PAGE_SIZE_4KB)] pub const PTRS_PER_PTE: usize = 1024;
#[cfg(CONFIG_PAGE_SIZE_16KB)] pub const PTRS_PER_PTE: usize = 256;
#[cfg(CONFIG_PAGE_SIZE_64KB)] pub const PTRS_PER_PTE: usize = 64;
#[cfg(CONFIG_PAGE_SIZE_256KB)] pub const PTRS_PER_PTE: usize = 16;
#[cfg(CONFIG_PAGE_SIZE_1MB)] pub const PTRS_PER_PTE: usize = 4;

extern "C" {
    pub static mut _dflt_cache_att: usize;
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub fn sync_icache_dcache(pte: pte_t);
}

pub const CACHEDEF: usize = CACHE_DEFAULT << 6;
pub const _NULL_PMD: usize = 0x7;
pub const _NULL_PTE: usize = 0x0;

#[inline] pub unsafe fn PAGE_NONE() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_USER | _dflt_cache_att) }
#[inline] pub unsafe fn PAGE_READONLY() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | _PAGE_EXECUTE | _dflt_cache_att) }
#[inline] pub unsafe fn PAGE_COPY() -> pgprot_t { PAGE_READONLY() }
#[inline] pub unsafe fn PAGE_EXEC() -> pgprot_t { PAGE_READONLY() }
#[inline] pub unsafe fn PAGE_COPY_EXEC() -> pgprot_t { PAGE_EXEC() }
#[inline] pub unsafe fn PAGE_SHARED() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_USER | _PAGE_READ | _PAGE_EXECUTE | _PAGE_WRITE | _dflt_cache_att) }
#[inline] pub unsafe fn PAGE_KERNEL() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_READ | _PAGE_WRITE | _PAGE_EXECUTE | _dflt_cache_att) }

#[inline]
pub unsafe fn pte_present_exec_user(pte: pte_t) -> bool {
    (pte_val(pte) & (_PAGE_EXECUTE | _PAGE_USER)) == (_PAGE_EXECUTE | _PAGE_USER)
}

#[inline]
pub unsafe fn set_pte(ptep: *mut pte_t, pteval: pte_t) {
    if pte_present_exec_user(pteval) { sync_icache_dcache(pteval); }
    *ptep = pteval;
}

#[inline] pub unsafe fn pmd_clear(p: *mut pmd_t) { *p = __pmd(_NULL_PMD); }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, p: *mut pte_t) { *p = __pte(_NULL_PTE); }
#[inline] pub unsafe fn pmd_none(p: pmd_t) -> i32 { (pmd_val(p) == _NULL_PMD) as i32 }
#[inline] pub unsafe fn pmd_present(p: pmd_t) -> i32 { (pmd_val(p) != _NULL_PMD) as i32 }
#[inline] pub fn pmd_bad(_p: pmd_t) -> i32 { 0 }
#[inline] pub unsafe fn pmd_pfn(p: pmd_t) -> usize { pmd_val(p) >> PAGE_SHIFT }
#[inline] pub unsafe fn pmd_page(p: pmd_t) -> *mut page { pfn_to_page(pmd_pfn(p)) }
#[inline] pub unsafe fn pte_none(p: pte_t) -> i32 { (pte_val(p) == _NULL_PTE) as i32 }
#[inline] pub unsafe fn pte_present(p: pte_t) -> usize { pte_val(p) & _PAGE_PRESENT }
#[inline] pub unsafe fn pte_page(p: pte_t) -> *mut page { pfn_to_page(pte_pfn(p)) }

#[inline] pub unsafe fn pte_mkold(mut p: pte_t) -> pte_t { p = __pte(pte_val(p) & !_PAGE_ACCESSED); p }
#[inline] pub unsafe fn pte_mkyoung(mut p: pte_t) -> pte_t { p = __pte(pte_val(p) | _PAGE_ACCESSED); p }
#[inline] pub unsafe fn pte_mkclean(mut p: pte_t) -> pte_t { p = __pte(pte_val(p) & !_PAGE_DIRTY); p }
#[inline] pub unsafe fn pte_mkdirty(mut p: pte_t) -> pte_t { p = __pte(pte_val(p) | _PAGE_DIRTY); p }
#[inline] pub unsafe fn pte_young(p: pte_t) -> usize { pte_val(p) & _PAGE_ACCESSED }
#[inline] pub unsafe fn pte_dirty(p: pte_t) -> usize { pte_val(p) & _PAGE_DIRTY }
#[inline] pub unsafe fn pte_modify(p: pte_t, prot: pgprot_t) -> pte_t { __pte((pte_val(p) & PAGE_MASK) | pgprot_val(prot)) }
#[inline] pub unsafe fn pte_wrprotect(p: pte_t) -> pte_t { __pte(pte_val(p) & !_PAGE_WRITE) }
#[inline] pub unsafe fn pte_mkwrite_novma(p: pte_t) -> pte_t { __pte(pte_val(p) | _PAGE_WRITE) }
#[inline] pub unsafe fn pte_mkexec(p: pte_t) -> pte_t { __pte(pte_val(p) | _PAGE_EXECUTE) }
#[inline] pub unsafe fn pte_read(p: pte_t) -> usize { pte_val(p) & _PAGE_READ }
#[inline] pub unsafe fn pte_write(p: pte_t) -> usize { pte_val(p) & _PAGE_WRITE }
#[inline] pub unsafe fn pte_exec(p: pte_t) -> usize { pte_val(p) & _PAGE_EXECUTE }

#[inline] pub unsafe fn __pte_to_swp_entry(p: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(p) } }
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }
pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;
#[inline] pub unsafe fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { __pte((pfn << PAGE_SHIFT) | pgprot_val(prot)) }
#[inline] pub unsafe fn pte_pfn(p: pte_t) -> usize { pte_val(p) >> PAGE_SHIFT }
#[inline] pub unsafe fn set_pmd(p: *mut pmd_t, v: pmd_t) { *p = v; }
#[inline] pub unsafe fn pmd_page_vaddr(p: pmd_t) -> usize { __va(pmd_val(p) & PAGE_MASK) as usize }

#[inline] pub fn __swp_type(p: swp_entry_t) -> usize { (p.val >> 1) & 0x1f }
#[inline] pub fn __swp_offset(p: swp_entry_t) -> usize { ((p.val >> 7) & 0x7) | ((p.val >> 10) & 0x3ffff8) }
#[inline] pub fn __swp_entry(ty: usize, off: usize) -> swp_entry_t { swp_entry_t { val: ((ty & 0x1f) << 1) | ((off & 0x3ffff8) << 10) | ((off & 0x7) << 7) } }
#[inline] pub unsafe fn pte_swp_exclusive(p: pte_t) -> bool { pte_val(p) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub unsafe fn pte_swp_mkexclusive(p: pte_t) -> pte_t { __pte(pte_val(p) | _PAGE_SWP_EXCLUSIVE) }
#[inline] pub unsafe fn pte_swp_clear_exclusive(p: pte_t) -> pte_t { __pte(pte_val(p) & !_PAGE_SWP_EXCLUSIVE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
