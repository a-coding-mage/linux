/* SPDX-License-Identifier: GPL-2.0 */
// Translated from alpha/include/asm/page.h.
// C header dependencies are supplied by other translated files.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub const STRICT_MM_TYPECHECKS: bool = true;

extern "C" {
    pub fn clear_page(page: *mut c_void);
    pub fn copy_page(_to: *mut c_void, _from: *mut c_void);
}

macro_rules! vma_alloc_zeroed_movable_folio {
    ($vma:expr, $vaddr:expr) => {
        vma_alloc_folio(GFP_HIGHUSER_MOVABLE | __GFP_ZERO, 0, $vma, $vaddr)
    };
}

macro_rules! copy_user_page {
    ($to:expr, $from:expr, $vaddr:expr, $pg:expr) => {
        copy_page($to, $from)
    };
}

/* These are used to make use of C type-checking. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    pub pte: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_t {
    pub pmd: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t {
    pub pgd: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t {
    pub pgprot: c_ulong,
}

pub type c_ulong = usize;

macro_rules! pte_val {
    ($x:expr) => { $x.pte };
}
macro_rules! pmd_val {
    ($x:expr) => { $x.pmd };
}
macro_rules! pgd_val {
    ($x:expr) => { $x.pgd };
}
macro_rules! pgprot_val {
    ($x:expr) => { $x.pgprot };
}

macro_rules! __pte {
    ($x:expr) => { pte_t { pte: $x } };
}
macro_rules! __pmd {
    ($x:expr) => { pmd_t { pmd: $x } };
}
macro_rules! __pgd {
    ($x:expr) => { pgd_t { pgd: $x } };
}
macro_rules! __pgprot {
    ($x:expr) => { pgprot_t { pgprot: $x } };
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub type pgtable_t = *mut page;

// Build-time condition preserved from USE_48_BIT_KSEG.
#[cfg(feature = "USE_48_BIT_KSEG")]
pub const PAGE_OFFSET: usize = 0xffff800000000000usize;
#[cfg(not(feature = "USE_48_BIT_KSEG"))]
pub const PAGE_OFFSET: usize = 0xfffffc0000000000usize;

macro_rules! __pa {
    ($x:expr) => { (($x as usize).wrapping_sub(PAGE_OFFSET)) };
}

macro_rules! __va {
    ($x:expr) => { (($x as usize).wrapping_add(PAGE_OFFSET) as *mut c_void) };
}

macro_rules! virt_to_page {
    ($kaddr:expr) => { pfn_to_page(__pa!($kaddr) >> PAGE_SHIFT) };
}

macro_rules! virt_addr_valid {
    ($kaddr:expr) => { pfn_valid(__pa!($kaddr) >> PAGE_SHIFT) };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
