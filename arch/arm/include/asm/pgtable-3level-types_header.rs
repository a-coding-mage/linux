/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/pgtable-3level-types.h
 *
 * Copyright (C) 2011 ARM Ltd.
 * Author: Catalin Marinas <catalin.marinas@arm.com>
 */

// C dependency: <asm/types.h>

pub type PtevalT = u64;
pub type PmdvalT = u64;
pub type PgdvalT = u64;

// STRICT_MM_TYPECHECKS is a C build-time condition. Enable the feature of
// the same name to select the type-checking representation.
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
pub struct PteT {
    pub pte: PtevalT,
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
pub struct PmdT {
    pub pmd: PmdvalT,
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
pub struct PgdT {
    pub pgd: PgdvalT,
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
pub struct PgprotT {
    pub pgprot: PtevalT,
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! pte_val {
    ($x:expr) => {{ $x.pte }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! pmd_val {
    ($x:expr) => {{ $x.pmd }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! pgd_val {
    ($x:expr) => {{ $x.pgd }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! pgprot_val {
    ($x:expr) => {{ $x.pgprot }};
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! __pte {
    ($x:expr) => {{ $crate::PteT { pte: $x } }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! __pmd {
    ($x:expr) => {{ $crate::PmdT { pmd: $x } }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! __pgd {
    ($x:expr) => {{ $crate::PgdT { pgd: $x } }};
}
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[macro_export]
macro_rules! __pgprot {
    ($x:expr) => {{ $crate::PgprotT { pgprot: $x } }};
}

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PteT = PtevalT;
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PmdT = PmdvalT;
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PgdT = PgdvalT;
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PgprotT = PtevalT;

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! pte_val {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! pmd_val {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! pgd_val {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! pgprot_val {
    ($x:expr) => {{ $x }};
}

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! __pte {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! __pmd {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! __pgd {
    ($x:expr) => {{ $x }};
}
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[macro_export]
macro_rules! __pgprot {
    ($x:expr) => {{ $x }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
