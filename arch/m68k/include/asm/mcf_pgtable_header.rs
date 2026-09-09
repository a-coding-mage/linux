/* SPDX-License-Identifier: GPL-2.0 */
// Translated from mcf_pgtable.h.
// Dependencies supplied by asm/mcfmmu.h and asm/page.h remain external.

/* MMUDR bits, in proper place. We write these directly into the MMUDR
 * after masking from the pte. */
pub const CF_PAGE_LOCKED: usize = MMUDR_LK;
pub const CF_PAGE_EXEC: usize = MMUDR_X;
pub const CF_PAGE_WRITABLE: usize = MMUDR_W;
pub const CF_PAGE_READABLE: usize = MMUDR_R;
pub const CF_PAGE_SYSTEM: usize = MMUDR_SP;
pub const CF_PAGE_COPYBACK: usize = MMUDR_CM_CCB;
pub const CF_PAGE_NOCACHE: usize = MMUDR_CM_NCP;

pub const CF_CACHEMASK: usize = !MMUDR_CM_CCB;
pub const CF_PAGE_MMUDR_MASK: usize = 0x000000fe;
pub const _PAGE_NOCACHE030: usize = CF_PAGE_NOCACHE;

/* MMUTR bits, need shifting down. */
pub const CF_PAGE_MMUTR_MASK: usize = 0x00000c00;
pub const CF_PAGE_MMUTR_SHIFT: usize = 10;
pub const CF_PAGE_VALID: usize = MMUTR_V << CF_PAGE_MMUTR_SHIFT;
pub const CF_PAGE_SHARED: usize = MMUTR_SG << CF_PAGE_MMUTR_SHIFT;

/* Fake bits, not implemented in CF, will get masked out before
 * hitting hardware. */
pub const CF_PAGE_DIRTY: usize = 0x00000001;
pub const CF_PAGE_ACCESSED: usize = 0x00001000;

pub const _PAGE_CACHE040: usize = 0x020;
pub const _PAGE_NOCACHE_S: usize = 0x040;
pub const _PAGE_NOCACHE: usize = 0x060;
pub const _PAGE_CACHE040W: usize = 0x000;
pub const _DESCTYPE_MASK: usize = 0x003;
pub const _CACHEMASK040: usize = !0x060usize;
pub const _PAGE_GLOBAL040: usize = 0x400;

/* We borrow bit 7 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: usize = CF_PAGE_NOCACHE;

pub const _PAGE_PRESENT: usize = CF_PAGE_VALID;
pub const _PAGE_ACCESSED: usize = CF_PAGE_ACCESSED;
pub const _PAGE_DIRTY: usize = CF_PAGE_DIRTY;
pub const _PAGE_READWRITE: usize = CF_PAGE_READABLE | CF_PAGE_WRITABLE | CF_PAGE_SYSTEM | CF_PAGE_SHARED;

pub const PAGE_NONE: pgprot_t = __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED);
pub const PAGE_SHARED: pgprot_t = __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_SHARED);
pub const PAGE_INIT: pgprot_t = __pgprot(CF_PAGE_VALID | CF_PAGE_READABLE | CF_PAGE_WRITABLE | CF_PAGE_EXEC | CF_PAGE_SYSTEM);
pub const PAGE_KERNEL: pgprot_t = __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_WRITABLE | CF_PAGE_EXEC | CF_PAGE_SYSTEM | CF_PAGE_SHARED);
pub const PAGE_COPY: pgprot_t = __pgprot(CF_PAGE_VALID | CF_PAGE_ACCESSED | CF_PAGE_READABLE | CF_PAGE_DIRTY);

pub const PTE_MASK: usize = PAGE_MASK;
pub const CF_PAGE_CHG_MASK: usize = PTE_MASK | CF_PAGE_ACCESSED | CF_PAGE_DIRTY;

#[inline]
pub unsafe fn pmd_pgtable(pmd: pmd_t) -> *mut core::ffi::c_void {
    pfn_to_virt(pmd_val(pmd) >> PAGE_SHIFT)
}

#[inline]
pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte_val(pte) = (pte_val(pte) & CF_PAGE_CHG_MASK) | pgprot_val(newprot);
    pte
}

#[inline]
pub unsafe fn pmd_set(_pmdp: *mut pmd_t, _ptep: *mut pte_t) {}

#[inline]
pub unsafe fn pgd_set(pgdp: *mut pgd_t, pmdp: *mut pmd_t) {
    pgd_val(*pgdp) = virt_to_phys(pmdp);
}

#[inline]
pub unsafe fn __pte_page(pte: pte_t) -> *mut core::ffi::c_void { (pte_val(pte) & PAGE_MASK) as *mut core::ffi::c_void }
#[inline]
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> usize { pmd_val(pmd) }

#[inline] pub unsafe fn pte_none(pte: pte_t) -> i32 { (!pte_val(pte)) as i32 }
#[inline] pub unsafe fn pte_present(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_VALID) as i32 }
#[inline] pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: usize, ptep: *mut pte_t) { pte_val(*ptep) = 0; }
#[inline] pub unsafe fn pte_page(pte: pte_t) -> *mut page { virt_to_page(__pte_page(pte)) }

#[inline] pub unsafe fn pmd_none2(pmd: *mut pmd_t) -> i32 { (!pmd_val(*pmd)) as i32 }
#[inline] pub unsafe fn pmd_none(pmd: pmd_t) -> i32 { pmd_none2(&mut { pmd }) }
#[inline] pub unsafe fn pmd_bad2(_pmd: *mut pmd_t) -> i32 { 0 }
#[inline] pub unsafe fn pmd_bad(pmd: pmd_t) -> i32 { pmd_bad2(&mut { pmd }) }
#[inline] pub unsafe fn pmd_present(pmd: pmd_t) -> i32 { (!pmd_none2(pmd as *mut pmd_t)) as i32 }
#[inline] pub unsafe fn pmd_clear(pmdp: *mut pmd_t) { pmd_val(*pmdp) = 0; }

/* The following only work if pte_present() is true. */
#[inline] pub unsafe fn pte_read(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_READABLE) as i32 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_WRITABLE) as i32 }
#[inline] pub unsafe fn pte_exec(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_EXEC) as i32 }
#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_DIRTY) as i32 }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> i32 { (pte_val(pte) & CF_PAGE_ACCESSED) as i32 }

#[inline] pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_WRITABLE; pte }
#[inline] pub unsafe fn pte_rdprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_READABLE; pte }
#[inline] pub unsafe fn pte_exprotect(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_EXEC; pte }
#[inline] pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_DIRTY; pte }
#[inline] pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_ACCESSED; pte }
#[inline] pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_val(pte) |= CF_PAGE_WRITABLE; pte }
#[inline] pub unsafe fn pte_mkread(mut pte: pte_t) -> pte_t { pte_val(pte) |= CF_PAGE_READABLE; pte }
#[inline] pub unsafe fn pte_mkexec(mut pte: pte_t) -> pte_t { pte_val(pte) |= CF_PAGE_EXEC; pte }
#[inline] pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_val(pte) |= CF_PAGE_DIRTY; pte }
#[inline] pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_val(pte) |= CF_PAGE_ACCESSED; pte }
#[inline] pub unsafe fn pte_mknocache(mut pte: pte_t) -> pte_t { pte_val(pte) |= 0x80 | (pte_val(pte) & !0x40); pte }
#[inline] pub unsafe fn pte_mkcache(mut pte: pte_t) -> pte_t { pte_val(pte) &= !CF_PAGE_NOCACHE; pte }

pub use kernel_pg_dir as swapper_pg_dir;
extern "C" { pub static mut kernel_pg_dir: [pgd_t; PTRS_PER_PGD]; }

#[inline] pub fn __swp_type(x: swp_entry_t) -> usize { x.val & 0x7f }
#[inline] pub fn __swp_offset(x: swp_entry_t) -> usize { x.val >> 11 }
#[inline] pub fn __swp_entry(typ: usize, off: usize) -> swp_entry_t { swp_entry_t { val: (typ & 0x7f) | (off << 11) } }
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }

#[inline] pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { pte_val(pte) & _PAGE_SWP_EXCLUSIVE != 0 }
#[inline] pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_val(pte) |= _PAGE_SWP_EXCLUSIVE; pte }
#[inline] pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_val(pte) &= !_PAGE_SWP_EXCLUSIVE; pte }

pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;
#[inline] pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { pmd_val(pmd) >> PAGE_SHIFT }
#[inline] pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd_val(pmd) >> PAGE_SHIFT) }
#[inline] pub unsafe fn pfn_pte(pfn: usize, prot: pgprot_t) -> pte_t { __pte((pfn << PAGE_SHIFT) | pgprot_val(prot)) }
#[inline] pub unsafe fn pte_pfn(pte: pte_t) -> usize { pte_val(pte) >> PAGE_SHIFT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
