/* SPDX-License-Identifier: GPL-2.0 */

/* Definitions for MMU descriptors. */
pub const _PAGE_PRESENT: usize = 0x001;
pub const _PAGE_SHORT: usize = 0x002;
pub const _PAGE_RONLY: usize = 0x004;
pub const _PAGE_READWRITE: usize = 0x000;
pub const _PAGE_ACCESSED: usize = 0x008;
pub const _PAGE_DIRTY: usize = 0x010;
pub const _PAGE_SUPER: usize = 0x080; /* 68040 supervisor only */
pub const _PAGE_GLOBAL040: usize = 0x400; /* 68040 global bit, used for kva descs */
pub const _PAGE_NOCACHE030: usize = 0x040; /* 68030 no-cache mode */
pub const _PAGE_NOCACHE: usize = 0x060; /* 68040 cache mode, non-serialized */
pub const _PAGE_NOCACHE_S: usize = 0x040; /* 68040 no-cache mode, serialized */
pub const _PAGE_CACHE040: usize = 0x020; /* 68040 cache mode, cachable, copyback */
pub const _PAGE_CACHE040W: usize = 0x000; /* 68040 cache mode, cachable, write-through */
pub const _DESCTYPE_MASK: usize = 0x003;
pub const _CACHEMASK040: usize = !0x060usize;
pub const _TABLE_MASK: usize = 0xffffff00;
pub const _PAGE_TABLE: usize = _PAGE_SHORT;
pub const _PAGE_PROTNONE: usize = 0x004;
pub const _PAGE_SWP_EXCLUSIVE: usize = 0x800;

/* These build-time selections mirror the corresponding C preprocessor conditions. */
extern "C" {
    pub static mut m68k_pgtable_cachemode: i32;
    pub static mut m68k_supervisor_cachemode: i32;
    pub static mut mm_cachebits: libc::c_ulong;
    pub static mut kernel_pg_dir: [pgd_t; 128];
    pub static mut mem_map: *mut page;
}

pub const fn page_none() -> pgprot_t { __pgprot(_PAGE_PROTNONE | _PAGE_ACCESSED | mm_cachebits) }
pub const fn page_shared() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_ACCESSED | mm_cachebits) }
pub const fn page_copy() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_RONLY | _PAGE_ACCESSED | mm_cachebits) }
pub const fn page_readonly() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_RONLY | _PAGE_ACCESSED | mm_cachebits) }
pub const fn page_kernel() -> pgprot_t { __pgprot(_PAGE_PRESENT | _PAGE_DIRTY | _PAGE_ACCESSED | mm_cachebits) }

/* PAGE_MASK, PAGE_SHIFT, PAGE_OFFSET and the MMU types/accessors are supplied by dependencies. */
pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte.val = (pte.val & _PAGE_CHG_MASK) | newprot.val;
    pte
}

pub unsafe fn pmd_set(pmdp: *mut pmd_t, ptep: *mut pte_t) {
    (*pmdp).val = virt_to_phys(ptep) | _PAGE_TABLE | _PAGE_ACCESSED;
}
pub unsafe fn pud_set(pudp: *mut pud_t, pmdp: *mut pmd_t) {
    (*pudp).val = _PAGE_TABLE | _PAGE_ACCESSED | __pa(pmdp);
}

pub unsafe fn __pte_page(pte: pte_t) -> libc::c_ulong { __va(pte.val & PAGE_MASK) as libc::c_ulong }
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> libc::c_ulong { __va(pmd.val & _TABLE_MASK) as libc::c_ulong }
pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t { __va((pud.val & _TABLE_MASK) as usize) as *mut pmd_t }

pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;
pub unsafe fn pte_page(pte: pte_t) -> *mut page { virt_to_page(__va(pte.val)) }
pub fn pte_pfn(pte: pte_t) -> usize { pte.val >> PAGE_SHIFT }
pub fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { __pte((pfn << PAGE_SHIFT) | prot.val) }

pub fn pte_none(pte: pte_t) -> bool { pte.val == 0 }
pub fn pte_present(pte: pte_t) -> usize { pte.val & (_PAGE_PRESENT | _PAGE_PROTNONE) }
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) { (*ptep).val = 0; }
pub fn pmd_none(pmd: pmd_t) -> bool { pmd.val == 0 }
pub fn pmd_bad(pmd: pmd_t) -> bool { (pmd.val & _DESCTYPE_MASK) != _PAGE_TABLE }
pub fn pmd_present(pmd: pmd_t) -> usize { pmd.val & _PAGE_TABLE }
pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { (*pmdp).val = 0; }
pub fn pmd_pfn(pmd: pmd_t) -> usize { (pmd.val & _TABLE_MASK) >> PAGE_SHIFT }
pub fn pmd_page(_pmd: pmd_t) -> *mut page { core::ptr::null_mut() }
pub fn pud_none(pud: pud_t) -> bool { pud.val == 0 }
pub fn pud_bad(pud: pud_t) -> bool { (pud.val & _DESCTYPE_MASK) != _PAGE_TABLE }
pub fn pud_present(pud: pud_t) -> usize { pud.val & _PAGE_TABLE }
pub unsafe fn pud_clear(pudp: *mut pud_t) { (*pudp).val = 0; }
pub unsafe fn pud_page(pud: pud_t) -> *mut page { mem_map.add(((__va(pud.val) as usize - PAGE_OFFSET) >> PAGE_SHIFT)) }

pub fn pte_write(pte: pte_t) -> i32 { if pte.val & _PAGE_RONLY == 0 { 1 } else { 0 } }
pub fn pte_dirty(pte: pte_t) -> usize { pte.val & _PAGE_DIRTY }
pub fn pte_young(pte: pte_t) -> usize { pte.val & _PAGE_ACCESSED }
pub fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_RONLY; pte }
pub fn pte_mkclean(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_DIRTY; pte }
pub fn pte_mkold(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_ACCESSED; pte }
pub fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_RONLY; pte }
pub fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_DIRTY; pte }
pub fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_ACCESSED; pte }
pub unsafe fn pte_mknocache(mut pte: pte_t) -> pte_t { pte.val = (pte.val & _CACHEMASK040) | m68k_pgtable_cachemode as usize; pte }
pub unsafe fn pte_mkcache(mut pte: pte_t) -> pte_t { pte.val = (pte.val & _CACHEMASK040) | m68k_supervisor_cachemode as usize; pte }

pub const SWAPPER_PG_DIR: &str = "kernel_pg_dir";
pub fn __swp_type(x: swp_entry_t) -> usize { (x.val >> 4) & 0x7f }
pub fn __swp_offset(x: swp_entry_t) -> usize { x.val >> 12 }
pub fn __swp_entry(ty: usize, offset: usize) -> swp_entry_t { swp_entry_t { val: ((ty & 0x7f) << 4) | (offset << 12) } }
pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte.val } }
pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { pte_t { val: x.val } }
pub fn pte_swp_exclusive(pte: pte_t) -> bool { pte.val & _PAGE_SWP_EXCLUSIVE != 0 }
pub fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte.val |= _PAGE_SWP_EXCLUSIVE; pte }
pub fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte.val &= !_PAGE_SWP_EXCLUSIVE; pte }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
