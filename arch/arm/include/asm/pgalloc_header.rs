/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/pgalloc.h
 *
 *  Copyright (C) 2000-2001 Russell King
 */

// Translated from the C header. Included dependencies and configuration
// symbols are supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_MMU")]
pub const _PAGE_USER_TABLE: _ = PMD_TYPE_TABLE | PMD_BIT4 | PMD_DOMAIN(DOMAIN_USER);
#[cfg(feature = "CONFIG_MMU")]
pub const _PAGE_KERNEL_TABLE: _ = PMD_TYPE_TABLE | PMD_BIT4 | PMD_DOMAIN(DOMAIN_KERNEL);

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_ARM_LPAE"))]
pub const PGD_SIZE: usize = PTRS_PER_PGD * core::mem::size_of::<pgd_t>();

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_ARM_LPAE"))]
#[inline]
pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    let _ = mm;
    set_pud(pud, __pud(__pa(pmd) | PMD_TYPE_TABLE));
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_ARM_LPAE")))]
pub const PGD_SIZE: usize = PAGE_SIZE << 2;

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_ARM_LPAE")))]
#[macro_export]
macro_rules! pmd_alloc_one {
    ($mm:expr, $addr:expr) => {{
        let _ = ($mm, $addr);
        BUG();
        2 as *mut pmd_t
    }};
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_ARM_LPAE")))]
#[macro_export]
macro_rules! pmd_free {
    ($mm:expr, $pmd:expr) => {{ let _ = ($mm, $pmd); }};
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_ARM_LPAE", feature = "CONFIG_KASAN")))]
#[macro_export]
macro_rules! pud_populate {
    ($mm:expr, $pmd:expr, $pte:expr) => {{ let _ = ($mm, $pmd, $pte); BUG!(); }};
}

#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_ARM_LPAE"), feature = "CONFIG_KASAN"))]
#[macro_export]
macro_rules! pud_populate {
    ($mm:expr, $pmd:expr, $pte:expr) => {{ let _ = ($mm, $pmd, $pte); }};
}

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
    pub fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn clean_pte_table(pte: *mut pte_t) {
    clean_dcache_area(pte.add(PTE_HWTABLE_PTRS), PTE_HWTABLE_SIZE);
}

// __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL, __HAVE_ARCH_PTE_ALLOC_ONE,
// __HAVE_ARCH_PGD_FREE; asm-generic/pgalloc.h supplies the dependencies.

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let pte = __pte_alloc_one_kernel(mm);
    if !pte.is_null() { clean_pte_table(pte); }
    pte
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    let pte = __pte_alloc_one(mm, GFP_PGTABLE_USER | PGTABLE_HIGHMEM);
    if pte.is_null() { return core::ptr::null_mut(); }
    if !PageHighMem(pte) { clean_pte_table(page_address(pte)); }
    pte
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn __pmd_populate(pmdp: *mut pmd_t, pte: phys_addr_t, prot: pmdval_t) {
    let pmdval = (pte + PTE_HWTABLE_OFF) | prot;
    *pmdp.add(0) = __pmd(pmdval);
    #[cfg(not(feature = "CONFIG_ARM_LPAE"))]
    { *pmdp.add(1) = __pmd(pmdval + 256 * core::mem::size_of::<pte_t>()); }
    flush_pmd_entry(pmdp);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pmd_populate_kernel(_mm: *mut mm_struct, pmdp: *mut pmd_t, ptep: *mut pte_t) {
    __pmd_populate(pmdp, __pa(ptep), _PAGE_KERNEL_TABLE);
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn pmd_populate(_mm: *mut mm_struct, pmdp: *mut pmd_t, ptep: pgtable_t) {
    extern "C" { static mut user_pmd_table: pmdval_t; }
    let prot;
    #[cfg(not(feature = "CONFIG_ARM_LPAE"))]
    { prot = if __LINUX_ARM_ARCH__ >= 6 { user_pmd_table } else { _PAGE_USER_TABLE }; }
    #[cfg(feature = "CONFIG_ARM_LPAE")]
    { prot = _PAGE_USER_TABLE; }
    __pmd_populate(pmdp, page_to_phys(ptep), prot);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
