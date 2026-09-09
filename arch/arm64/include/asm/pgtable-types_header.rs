/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Page table types definitions.
 *
 * Copyright (C) 2014 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 */

// Dependency supplied by the surrounding translation unit: <asm/types.h>.

/*
 * Page Table Descriptor
 *
 * Generic page table descriptor format from which
 * all level specific descriptors can be derived.
 */
pub type ptval_t = u64;

pub type pteval_t = ptval_t;
pub type pmdval_t = ptval_t;
pub type pudval_t = ptval_t;
pub type p4dval_t = ptval_t;
pub type pgdval_t = ptval_t;

/*
 * These are used to make use of C type-checking..
 */
#[repr(C)]
pub struct pte_t {
    pub pte: pteval_t,
}

#[macro_export]
macro_rules! pte_val {
    ($x:expr) => { $x.pte };
}

#[macro_export]
macro_rules! __pte {
    ($x:expr) => { $crate::pte_t { pte: $x } };
}

// C condition: CONFIG_PGTABLE_LEVELS > 2.
#[cfg(feature = "config_pgtable_levels_gt_2")]
#[repr(C)]
pub struct pmd_t {
    pub pmd: pmdval_t,
}

#[cfg(feature = "config_pgtable_levels_gt_2")]
#[macro_export]
macro_rules! pmd_val {
    ($x:expr) => { $x.pmd };
}

#[cfg(feature = "config_pgtable_levels_gt_2")]
#[macro_export]
macro_rules! __pmd {
    ($x:expr) => { $crate::pmd_t { pmd: $x } };
}

// C condition: CONFIG_PGTABLE_LEVELS > 3.
#[cfg(feature = "config_pgtable_levels_gt_3")]
#[repr(C)]
pub struct pud_t {
    pub pud: pudval_t,
}

#[cfg(feature = "config_pgtable_levels_gt_3")]
#[macro_export]
macro_rules! pud_val {
    ($x:expr) => { $x.pud };
}

#[cfg(feature = "config_pgtable_levels_gt_3")]
#[macro_export]
macro_rules! __pud {
    ($x:expr) => { $crate::pud_t { pud: $x } };
}

// C condition: CONFIG_PGTABLE_LEVELS > 4.
#[cfg(feature = "config_pgtable_levels_gt_4")]
#[repr(C)]
pub struct p4d_t {
    pub p4d: p4dval_t,
}

#[cfg(feature = "config_pgtable_levels_gt_4")]
#[macro_export]
macro_rules! p4d_val {
    ($x:expr) => { $x.p4d };
}

#[cfg(feature = "config_pgtable_levels_gt_4")]
#[macro_export]
macro_rules! __p4d {
    ($x:expr) => { $crate::p4d_t { p4d: $x } };
}

#[repr(C)]
pub struct pgd_t {
    pub pgd: pgdval_t,
}

#[macro_export]
macro_rules! pgd_val {
    ($x:expr) => { $x.pgd };
}

#[macro_export]
macro_rules! __pgd {
    ($x:expr) => { $crate::pgd_t { pgd: $x } };
}

#[repr(C)]
pub struct pgprot_t {
    pub pgprot: ptval_t,
}

#[macro_export]
macro_rules! pgprot_val {
    ($x:expr) => { $x.pgprot };
}

#[macro_export]
macro_rules! __pgprot {
    ($x:expr) => { $crate::pgprot_t { pgprot: $x } };
}

// Conditional dependencies supplied by the surrounding translation unit:
// CONFIG_PGTABLE_LEVELS == 2 -> <asm-generic/pgtable-nopmd.h>
// CONFIG_PGTABLE_LEVELS == 3 -> <asm-generic/pgtable-nopud.h>
// CONFIG_PGTABLE_LEVELS == 4 -> <asm-generic/pgtable-nop4d.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
