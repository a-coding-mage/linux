/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally: asm/cmpxchg.h

/* PTE level */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte: __be64,
}

#[inline]
pub fn __pte(x: u64) -> pte_t {
    pte_t { pte: cpu_to_be64(x) }
}

#[inline]
pub fn __pte_raw(x: __be64) -> pte_t {
    pte_t { pte: x }
}

#[inline]
pub fn pte_val(x: pte_t) -> usize {
    be64_to_cpu(x.pte) as usize
}

#[inline]
pub fn pte_raw(x: pte_t) -> __be64 {
    x.pte
}

/* PMD level */
// This section is conditional on CONFIG_PPC64.
#[cfg(CONFIG_PPC64)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t {
    pub pmd: __be64,
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn __pmd(x: u64) -> pmd_t {
    pmd_t { pmd: cpu_to_be64(x) }
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn __pmd_raw(x: __be64) -> pmd_t {
    pmd_t { pmd: x }
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn pmd_val(x: pmd_t) -> usize {
    be64_to_cpu(x.pmd) as usize
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn pmd_raw(x: pmd_t) -> __be64 {
    x.pmd
}

/* 64 bit always use 4 level table. */
#[cfg(CONFIG_PPC64)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_t {
    pub pud: __be64,
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn __pud(x: u64) -> pud_t {
    pud_t { pud: cpu_to_be64(x) }
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn __pud_raw(x: __be64) -> pud_t {
    pud_t { pud: x }
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn pud_val(x: pud_t) -> usize {
    be64_to_cpu(x.pud) as usize
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub fn pud_raw(x: pud_t) -> __be64 {
    x.pud
}

/* PGD level */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: __be64,
}

#[inline]
pub fn __pgd(x: u64) -> pgd_t {
    pgd_t { pgd: cpu_to_be64(x) }
}

#[inline]
pub fn __pgd_raw(x: __be64) -> pgd_t {
    pgd_t { pgd: x }
}

#[inline]
pub fn pgd_val(x: pgd_t) -> usize {
    be64_to_cpu(x.pgd) as usize
}

#[inline]
pub fn pgd_raw(x: pgd_t) -> __be64 {
    x.pgd
}

/* Page protection bits */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: usize,
}

#[inline]
pub fn pgprot_val(x: pgprot_t) -> usize {
    x.pgprot
}

#[inline]
pub fn __pgprot(x: usize) -> pgprot_t {
    pgprot_t { pgprot: x }
}

/*
 * With hash config 64k pages additionally define a bigger "real PTE" type that
 * gathers the "second half" part of the PTE for pseudo 64k pages
 */
// This section is conditional on CONFIG_PPC_64K_PAGES.
#[cfg(CONFIG_PPC_64K_PAGES)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_pte_t {
    pub pte: pte_t,
    pub hidx: usize,
}

#[cfg(not(CONFIG_PPC_64K_PAGES))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_pte_t {
    pub pte: pte_t,
}

#[inline]
pub unsafe fn pte_xchg(ptep: *mut pte_t, old: pte_t, new: pte_t) -> bool {
    let p = ptep as *mut usize;
    let prev: __be64 = __cmpxchg_u64(
        p,
        pte_raw(old) as usize,
        pte_raw(new) as usize,
    ) as __be64;

    /* See comment in switch_mm_irqs_off() */
    pte_raw(old) == prev
}

#[cfg(CONFIG_PPC64)]
#[inline]
pub unsafe fn pmd_xchg(pmdp: *mut pmd_t, old: pmd_t, new: pmd_t) -> bool {
    let p = pmdp as *mut usize;
    let prev: __be64 = __cmpxchg_u64(
        p,
        pmd_raw(old) as usize,
        pmd_raw(new) as usize,
    ) as __be64;

    pmd_raw(old) == prev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
