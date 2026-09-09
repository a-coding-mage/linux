/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C preprocessor conditions are preserved as Rust cfg conditions where
 * they have a direct local equivalent.  pte_basic_t and __cmpxchg_u64 are
 * supplied by the surrounding translation unit.
 */

/* PTE level */
#[cfg(all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES"))]
#[repr(C)]
pub struct pte_t {
    pub pte: pte_basic_t,
    pub pte1: pte_basic_t,
    pub pte2: pte_basic_t,
    pub pte3: pte_basic_t,
}

#[cfg(all(not(all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES")), feature = "STRICT_MM_TYPECHECKS"))]
#[repr(C)]
pub struct pte_t {
    pub pte: pte_basic_t,
}

#[cfg(all(not(all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES")), not(feature = "STRICT_MM_TYPECHECKS")))]
pub type pte_t = pte_basic_t;

#[cfg(any(feature = "STRICT_MM_TYPECHECKS", all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES")))]
#[inline]
pub fn __pte(x: pte_basic_t) -> pte_t {
    pte_t { pte: x }
}

#[cfg(any(feature = "STRICT_MM_TYPECHECKS", all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES")))]
#[inline]
pub fn pte_val(x: pte_t) -> pte_basic_t {
    x.pte
}

#[cfg(all(not(feature = "STRICT_MM_TYPECHECKS"), not(all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES"))))]
#[inline]
pub fn __pte(x: pte_basic_t) -> pte_t {
    x
}

#[cfg(all(not(feature = "STRICT_MM_TYPECHECKS"), not(all(feature = "CONFIG_PPC_8xx", feature = "CONFIG_PPC_16K_PAGES"))))]
#[inline]
pub fn pte_val(x: pte_t) -> pte_basic_t {
    x
}

/* PMD level */
#[cfg(feature = "CONFIG_PPC64")]
#[repr(C)]
pub struct pmd_t {
    pub pmd: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub fn __pmd(x: ::core::ffi::c_ulong) -> pmd_t {
    pmd_t { pmd: x }
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub fn pmd_val(x: pmd_t) -> ::core::ffi::c_ulong {
    x.pmd
}

/* 64 bit always use 4 level table. */
#[cfg(feature = "CONFIG_PPC64")]
#[repr(C)]
pub struct pud_t {
    pub pud: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub fn __pud(x: ::core::ffi::c_ulong) -> pud_t {
    pud_t { pud: x }
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub fn pud_val(x: pud_t) -> ::core::ffi::c_ulong {
    x.pud
}

/* PGD level */
#[cfg(feature = "CONFIG_PPC_85xx")]
#[repr(C)]
pub struct pgd_t {
    pub pgd: ::core::ffi::c_ulonglong,
}

#[cfg(not(feature = "CONFIG_PPC_85xx"))]
#[repr(C)]
pub struct pgd_t {
    pub pgd: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PPC_85xx")]
#[inline]
pub fn pgd_val(x: pgd_t) -> ::core::ffi::c_ulonglong {
    x.pgd
}

#[cfg(not(feature = "CONFIG_PPC_85xx"))]
#[inline]
pub fn pgd_val(x: pgd_t) -> ::core::ffi::c_ulong {
    x.pgd
}

#[inline]
pub fn __pgd(x: ::core::ffi::c_ulong) -> pgd_t {
    pgd_t { pgd: x }
}

/* Page protection bits */
#[repr(C)]
pub struct pgprot_t {
    pub pgprot: ::core::ffi::c_ulong,
}

#[inline]
pub fn pgprot_val(x: pgprot_t) -> ::core::ffi::c_ulong {
    x.pgprot
}

#[inline]
pub fn __pgprot(x: ::core::ffi::c_ulong) -> pgprot_t {
    pgprot_t { pgprot: x }
}

/*
 * With hash config 64k pages additionally define a bigger "real PTE" type
 * that gathers the "second half" part of the PTE for pseudo 64k pages
 */
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
#[repr(C)]
pub struct real_pte_t {
    pub pte: pte_t,
    pub hidx: ::core::ffi::c_ulong,
}

#[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
#[repr(C)]
pub struct real_pte_t {
    pub pte: pte_t,
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn pte_xchg(ptep: *mut pte_t, old: pte_t, new: pte_t) -> bool {
    let p = ptep as *mut ::core::ffi::c_ulong;
    /* See comment in switch_mm_irqs_off() */
    pte_val(old) == __cmpxchg_u64(p, pte_val(old), pte_val(new))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
