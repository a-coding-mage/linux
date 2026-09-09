/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm-generic/pgalloc.h. */

/* #ifdef CONFIG_MMU */

macro_rules! GFP_PGTABLE_KERNEL { () => { GFP_KERNEL | __GFP_ZERO | __GFP_SKIP_KASAN }; }
macro_rules! GFP_PGTABLE_USER { () => { GFP_PGTABLE_KERNEL!() | __GFP_ACCOUNT }; }

/// Allocate memory for a PTE-level kernel page table.
#[inline]
pub unsafe fn __pte_alloc_one_kernel_noprof(mm: *mut mm_struct) -> *mut pte_t {
    let ptdesc = pagetable_alloc_noprof(GFP_PGTABLE_KERNEL!(), 0);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pte_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }
    ptdesc_set_kernel(ptdesc);
    ptdesc_address(ptdesc)
}
macro_rules! __pte_alloc_one_kernel { ($($args:tt)*) => { alloc_hooks!(__pte_alloc_one_kernel_noprof!($($args)*)) }; }

/* #ifndef __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL */
#[inline]
pub unsafe fn pte_alloc_one_kernel_noprof(mm: *mut mm_struct) -> *mut pte_t {
    __pte_alloc_one_kernel_noprof(mm)
}
macro_rules! pte_alloc_one_kernel { ($($args:tt)*) => { alloc_hooks!(pte_alloc_one_kernel_noprof!($($args)*)) }; }
/* #endif */

#[inline]
pub unsafe fn pte_free_kernel(_mm: *mut mm_struct, pte: *mut pte_t) {
    pagetable_dtor_free(virt_to_ptdesc(pte));
}

#[inline]
pub unsafe fn __pte_alloc_one_noprof(mm: *mut mm_struct, gfp: gfp_t) -> pgtable_t {
    let ptdesc = pagetable_alloc_noprof(gfp, 0);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pte_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }
    ptdesc_page(ptdesc)
}
macro_rules! __pte_alloc_one { ($($args:tt)*) => { alloc_hooks!(__pte_alloc_one_noprof!($($args)*)) }; }

/* #ifndef __HAVE_ARCH_PTE_ALLOC_ONE */
#[inline]
pub unsafe fn pte_alloc_one_noprof(mm: *mut mm_struct) -> pgtable_t {
    __pte_alloc_one_noprof(mm, GFP_PGTABLE_USER!())
}
macro_rules! pte_alloc_one { ($($args:tt)*) => { alloc_hooks!(pte_alloc_one_noprof!($($args)*)) }; }
/* #endif */

/* Should really implement gc for free page table pages. */
#[inline]
pub unsafe fn pte_free(_mm: *mut mm_struct, pte_page: *mut page) {
    let ptdesc = page_ptdesc(pte_page);
    pagetable_dtor_free(ptdesc);
}

/* #if CONFIG_PGTABLE_LEVELS > 2 */
/* #ifndef __HAVE_ARCH_PMD_ALLOC_ONE */
#[inline]
pub unsafe fn pmd_alloc_one_noprof(mm: *mut mm_struct, _addr: c_ulong) -> *mut pmd_t {
    let mut gfp = GFP_PGTABLE_USER!();
    if mm == &raw mut init_mm { gfp = GFP_PGTABLE_KERNEL!(); }
    let ptdesc = pagetable_alloc_noprof(gfp, 0);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    if !pagetable_pmd_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }
    if mm == &raw mut init_mm { ptdesc_set_kernel(ptdesc); }
    ptdesc_address(ptdesc)
}
macro_rules! pmd_alloc_one { ($($args:tt)*) => { alloc_hooks!(pmd_alloc_one_noprof!($($args)*)) }; }
/* #endif */

/* #ifndef __HAVE_ARCH_PMD_FREE */
#[inline]
pub unsafe fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) {
    let ptdesc = virt_to_ptdesc(pmd);
    BUG_ON!((pmd as c_ulong) & (PAGE_SIZE - 1));
    pagetable_dtor_free(ptdesc);
}
/* #endif */
/* #endif */

/* #if CONFIG_PGTABLE_LEVELS > 3 */
#[inline]
pub unsafe fn __pud_alloc_one_noprof(mm: *mut mm_struct, _addr: c_ulong) -> *mut pud_t {
    let mut gfp = GFP_PGTABLE_USER!();
    if mm == &raw mut init_mm { gfp = GFP_PGTABLE_KERNEL!(); }
    let ptdesc = pagetable_alloc_noprof(gfp, 0);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    pagetable_pud_ctor(ptdesc);
    if mm == &raw mut init_mm { ptdesc_set_kernel(ptdesc); }
    ptdesc_address(ptdesc)
}
macro_rules! __pud_alloc_one { ($($args:tt)*) => { alloc_hooks!(__pud_alloc_one_noprof!($($args)*)) }; }
/* #ifndef __HAVE_ARCH_PUD_ALLOC_ONE */
#[inline]
pub unsafe fn pud_alloc_one_noprof(mm: *mut mm_struct, addr: c_ulong) -> *mut pud_t { __pud_alloc_one_noprof(mm, addr) }
macro_rules! pud_alloc_one { ($($args:tt)*) => { alloc_hooks!(pud_alloc_one_noprof!($($args)*)) }; }
/* #endif */
#[inline]
pub unsafe fn __pud_free(_mm: *mut mm_struct, pud: *mut pud_t) {
    let ptdesc = virt_to_ptdesc(pud);
    BUG_ON!((pud as c_ulong) & (PAGE_SIZE - 1));
    pagetable_dtor_free(ptdesc);
}
/* #ifndef __HAVE_ARCH_PUD_FREE */
#[inline]
pub unsafe fn pud_free(mm: *mut mm_struct, pud: *mut pud_t) { __pud_free(mm, pud); }
/* #endif */
/* #endif */

/* #if CONFIG_PGTABLE_LEVELS > 4 */
#[inline]
pub unsafe fn __p4d_alloc_one_noprof(mm: *mut mm_struct, _addr: c_ulong) -> *mut p4d_t {
    let mut gfp = GFP_PGTABLE_USER!();
    if mm == &raw mut init_mm { gfp = GFP_PGTABLE_KERNEL!(); }
    let ptdesc = pagetable_alloc_noprof(gfp, 0);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    pagetable_p4d_ctor(ptdesc);
    if mm == &raw mut init_mm { ptdesc_set_kernel(ptdesc); }
    ptdesc_address(ptdesc)
}
macro_rules! __p4d_alloc_one { ($($args:tt)*) => { alloc_hooks!(__p4d_alloc_one_noprof!($($args)*)) }; }
/* #ifndef __HAVE_ARCH_P4D_ALLOC_ONE */
#[inline]
pub unsafe fn p4d_alloc_one_noprof(mm: *mut mm_struct, addr: c_ulong) -> *mut p4d_t { __p4d_alloc_one_noprof(mm, addr) }
macro_rules! p4d_alloc_one { ($($args:tt)*) => { alloc_hooks!(p4d_alloc_one_noprof!($($args)*)) }; }
/* #endif */
#[inline]
pub unsafe fn __p4d_free(_mm: *mut mm_struct, p4d: *mut p4d_t) {
    let ptdesc = virt_to_ptdesc(p4d);
    BUG_ON!((p4d as c_ulong) & (PAGE_SIZE - 1));
    pagetable_dtor_free(ptdesc);
}
/* #ifndef __HAVE_ARCH_P4D_FREE */
#[inline]
pub unsafe fn p4d_free(mm: *mut mm_struct, p4d: *mut p4d_t) { if !mm_p4d_folded(mm) { __p4d_free(mm, p4d); } }
/* #endif */
/* #endif */

#[inline]
pub unsafe fn __pgd_alloc_noprof(mm: *mut mm_struct, order: c_uint) -> *mut pgd_t {
    let mut gfp = GFP_PGTABLE_USER!();
    if mm == &raw mut init_mm { gfp = GFP_PGTABLE_KERNEL!(); }
    let ptdesc = pagetable_alloc_noprof(gfp, order);
    if ptdesc.is_null() { return core::ptr::null_mut(); }
    pagetable_pgd_ctor(ptdesc);
    if mm == &raw mut init_mm { ptdesc_set_kernel(ptdesc); }
    ptdesc_address(ptdesc)
}
macro_rules! __pgd_alloc { ($($args:tt)*) => { alloc_hooks!(__pgd_alloc_noprof!($($args)*)) }; }

#[inline]
pub unsafe fn __pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    let ptdesc = virt_to_ptdesc(pgd);
    BUG_ON!((pgd as c_ulong) & (PAGE_SIZE - 1));
    pagetable_dtor_free(ptdesc);
}
/* #ifndef __HAVE_ARCH_PGD_FREE */
#[inline]
pub unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) { __pgd_free(mm, pgd); }
/* #endif */

/* #endif CONFIG_MMU */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
