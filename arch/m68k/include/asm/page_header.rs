/* SPDX-License-Identifier: GPL-2.0 */

// PAGE_OFFSET is defined from PAGE_OFFSET_RAW in asm/page_offset.h.
pub const PAGE_OFFSET: usize = PAGE_OFFSET_RAW;

/*
 * These are used to make use of C type-checking..
 *
 * The CONFIG_MMU and CONFIG_PGTABLE_LEVELS conditions are retained here as
 * Rust configuration conditions; the corresponding build configuration is
 * supplied by the surrounding kernel translation.
 */
#[cfg(any(not(feature = "mmu"), feature = "pgtable_levels_3"))]
#[repr(C)]
pub struct pmd_t {
    pub pmd: usize,
}

#[cfg(any(not(feature = "mmu"), feature = "pgtable_levels_3"))]
#[macro_export]
macro_rules! pmd_val {
    ($x:expr) => {
        $x.pmd
    };
}

#[cfg(any(not(feature = "mmu"), feature = "pgtable_levels_3"))]
#[inline]
pub const fn __pmd(x: usize) -> pmd_t {
    pmd_t { pmd: x }
}

#[repr(C)]
pub struct pte_t {
    pub pte: usize,
}

#[repr(C)]
pub struct pgd_t {
    pub pgd: usize,
}

#[repr(C)]
pub struct pgprot_t {
    pub pgprot: usize,
}

// CONFIG_SUN3 selects `struct page *`; other configurations use `pte_t *`.
#[cfg(feature = "sun3")]
pub type pgtable_t = *mut page;

#[cfg(not(feature = "sun3"))]
pub type pgtable_t = *mut pte_t;

#[macro_export]
macro_rules! pte_val {
    ($x:expr) => {
        $x.pte
    };
}

#[macro_export]
macro_rules! pgd_val {
    ($x:expr) => {
        $x.pgd
    };
}

#[macro_export]
macro_rules! pgprot_val {
    ($x:expr) => {
        $x.pgprot
    };
}

#[inline]
pub const fn __pte(x: usize) -> pte_t {
    pte_t { pte: x }
}

#[inline]
pub const fn __pgd(x: usize) -> pgd_t {
    pgd_t { pgd: x }
}

#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t {
    pgprot_t { pgprot: x }
}

unsafe extern "C" {
    pub static mut _rambase: usize;
    pub static mut _ramstart: usize;
    pub static mut _ramend: usize;
}

// CONFIG_MMU selects asm/page_mm.h; otherwise asm/page_no.h.
// The asm-generic/getorder.h and asm-generic/memory_model.h declarations are
// supplied by their corresponding translated dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
