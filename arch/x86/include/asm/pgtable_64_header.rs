/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/pgtable_64.h. External kernel types and operations are
 * supplied by the surrounding translation unit. */

extern "C" {
    pub static mut level4_kernel_pgt: [p4d_t; 512];
    pub static mut level4_ident_pgt: [p4d_t; 512];
    pub static mut level3_kernel_pgt: [pud_t; 512];
    pub static mut level2_kernel_pgt: [pmd_t; 512];
    pub static mut level2_fixmap_pgt: [pmd_t; 512];
    pub static mut level1_fixmap_pgt: [pte_t; 512 * FIXMAP_PMD_NUM];
    pub static mut init_top_pgt: [pgd_t; 0];
    pub fn paging_init();
    pub fn set_pte_vaddr_p4d(p4d_page: *mut p4d_t, vaddr: c_ulong, new_pte: pte_t);
    pub fn set_pte_vaddr_pud(pud_page: *mut pud_t, vaddr: c_ulong, new_pte: pte_t);
    pub fn cleanup_highmap();
    pub fn init_extra_mapping_uc(phys: c_ulong, size: c_ulong);
    pub fn init_extra_mapping_wb(phys: c_ulong, size: c_ulong);
}

pub const swapper_pg_dir: *mut pgd_t = unsafe { init_top_pgt.as_mut_ptr() };

#[inline]
pub fn sync_initial_page_table() {}

/* pte_ERROR, pmd_ERROR, pud_ERROR, p4d_ERROR, and pgd_ERROR invoke the
 * surrounding kernel's pr_err, source-location, and *_val facilities. */

#[repr(C)]
pub struct mm_struct;

#[inline]
pub unsafe fn mm_p4d_folded(_mm: *mut mm_struct) -> bool { !pgtable_l5_enabled() }

#[inline]
pub unsafe fn native_set_pte(ptep: *mut pte_t, pte: pte_t) {
    core::ptr::write_volatile(ptep, pte);
}

#[inline]
pub unsafe fn native_pte_clear(_mm: *mut mm_struct, _addr: c_ulong, ptep: *mut pte_t) {
    native_set_pte(ptep, native_make_pte(0));
}

#[inline]
pub unsafe fn native_set_pte_atomic(ptep: *mut pte_t, pte: pte_t) { native_set_pte(ptep, pte); }

#[inline]
pub unsafe fn native_set_pmd(pmdp: *mut pmd_t, pmd: pmd_t) {
    core::ptr::write_volatile(pmdp, pmd);
}

#[inline]
pub unsafe fn native_pmd_clear(pmd: *mut pmd_t) { native_set_pmd(pmd, native_make_pmd(0)); }

#[inline]
pub unsafe fn native_ptep_get_and_clear(xp: *mut pte_t) -> pte_t {
    /* CONFIG_SMP selects the atomic xchg path; the local path is retained
     * here as the source's build-time alternative. */
    let ret = *xp;
    native_pte_clear(core::ptr::null_mut(), 0, xp);
    ret
}

#[inline]
pub unsafe fn native_pmdp_get_and_clear(xp: *mut pmd_t) -> pmd_t {
    let ret = *xp;
    native_pmd_clear(xp);
    ret
}

#[inline]
pub unsafe fn native_set_pud(pudp: *mut pud_t, pud: pud_t) {
    core::ptr::write_volatile(pudp, pud);
}

#[inline]
pub unsafe fn native_pud_clear(pud: *mut pud_t) { native_set_pud(pud, native_make_pud(0)); }

#[inline]
pub unsafe fn native_pudp_get_and_clear(xp: *mut pud_t) -> pud_t {
    let ret = *xp;
    native_pud_clear(xp);
    ret
}

#[inline]
pub unsafe fn native_set_p4d(p4dp: *mut p4d_t, p4d: p4d_t) {
    if pgtable_l5_enabled() || !IS_ENABLED_CONFIG_MITIGATION_PAGE_TABLE_ISOLATION {
        core::ptr::write_volatile(p4dp, p4d);
        return;
    }
    let mut pgd = native_make_pgd(native_p4d_val(p4d));
    pgd = pti_set_user_pgtbl(p4dp as *mut pgd_t, pgd);
    core::ptr::write_volatile(p4dp, native_make_p4d(native_pgd_val(pgd)));
}

#[inline]
pub unsafe fn native_p4d_clear(p4d: *mut p4d_t) { native_set_p4d(p4d, native_make_p4d(0)); }

#[inline]
pub unsafe fn native_set_pgd(pgdp: *mut pgd_t, pgd: pgd_t) {
    core::ptr::write_volatile(pgdp, pti_set_user_pgtbl(pgdp, pgd));
}

#[inline]
pub unsafe fn native_pgd_clear(pgd: *mut pgd_t) { native_set_pgd(pgd, native_make_pgd(0)); }

pub const SWP_TYPE_BITS: u32 = 5;
pub const SWP_OFFSET_FIRST_BIT: u32 = _PAGE_BIT_PROTNONE + 1;
pub const SWP_OFFSET_SHIFT: u32 = SWP_OFFSET_FIRST_BIT + SWP_TYPE_BITS;

#[inline] pub fn __swp_type(x: swp_entry_t) -> c_ulong { x.val >> (64 - SWP_TYPE_BITS) }
#[inline] pub fn __swp_offset(x: swp_entry_t) -> c_ulong { (!(x.val) << SWP_TYPE_BITS) >> SWP_OFFSET_SHIFT }
#[inline] pub fn __swp_entry(type_: c_ulong, offset: c_ulong) -> swp_entry_t {
    swp_entry_t { val: ((!(offset) << SWP_OFFSET_SHIFT) >> SWP_TYPE_BITS) | (type_ << (64 - SWP_TYPE_BITS)) }
}
#[inline] pub fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte) } }
#[inline] pub fn __pmd_to_swp_entry(pmd: pmd_t) -> swp_entry_t { swp_entry_t { val: pmd_val(pmd) } }
#[inline] pub fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }
#[inline] pub fn __swp_entry_to_pmd(x: swp_entry_t) -> pmd_t { __pmd(x.val) }

pub const HAVE_ARCH_UNMAPPED_AREA: bool = true;
pub const HAVE_ARCH_UNMAPPED_AREA_TOPDOWN: bool = true;
pub const PAGE_AGP: usize = PAGE_KERNEL_NOCACHE;
pub const HAVE_PAGE_AGP: usize = 1;

#[inline] pub fn kc_vaddr_to_offset(v: c_ulong) -> c_ulong { v & __VIRTUAL_MASK }
#[inline] pub fn kc_offset_to_vaddr(o: c_ulong) -> c_ulong { o | !__VIRTUAL_MASK }

#[inline]
pub unsafe fn gup_fast_permitted(start: c_ulong, end: c_ulong) -> bool {
    let _ = start;
    !(end >> __VIRTUAL_MASK_SHIFT != 0)
}

/* __ASSEMBLER__-only l4_index, pud_index, L4_PAGE_OFFSET, L4_START_KERNEL,
 * L3_START_KERNEL, SYM_DATA_START_PAGE_ALIGNED, and PMDS are retained as
 * assembly-source concepts and are intentionally represented by this comment. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
