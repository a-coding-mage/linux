/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/pgtable-2level-types.h
 *
 * Copyright (C) 1995-2003 Russell King
 */

// C header dependency: <asm/types.h>

pub type PtevalT = u32;
pub type PmdvalT = u32;

// STRICT_MM_TYPECHECKS is undefined by the source header.  The cfg branches
// preserve the source's alternative type-checking representation.
#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[repr(C)]
pub struct PteT {
    pub pte: PtevalT,
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[repr(C)]
pub struct PmdT {
    pub pmd: PmdvalT,
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[repr(C)]
pub struct PgdT {
    pub pgd: [PmdvalT; 2],
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[repr(C)]
pub struct PgprotT {
    pub pgprot: PtevalT,
}

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn pte_val(x: PteT) -> PtevalT { x.pte }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn pmd_val(x: PmdT) -> PmdvalT { x.pmd }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn pgd_val(x: PgdT) -> PmdvalT { x.pgd[0] }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn pgprot_val(x: PgprotT) -> PtevalT { x.pgprot }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn __pte(x: PtevalT) -> PteT { PteT { pte: x } }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn __pmd(x: PmdvalT) -> PmdT { PmdT { pmd: x } }

#[cfg(feature = "STRICT_MM_TYPECHECKS")]
#[inline]
pub const fn __pgprot(x: PtevalT) -> PgprotT { PgprotT { pgprot: x } }

// Non-STRICT_MM_TYPECHECKS representation (the active source configuration).
#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PteT = PtevalT;

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PmdT = PmdvalT;

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PgdT = [PmdvalT; 2];

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
pub type PgprotT = PtevalT;

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn pte_val(x: PteT) -> PtevalT { x }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn pmd_val(x: PmdT) -> PmdvalT { x }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn pgd_val(x: PgdT) -> PmdvalT { x[0] }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn pgprot_val(x: PgprotT) -> PtevalT { x }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn __pte(x: PtevalT) -> PteT { x }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn __pmd(x: PmdvalT) -> PmdT { x }

#[cfg(not(feature = "STRICT_MM_TYPECHECKS"))]
#[inline]
pub const fn __pgprot(x: PtevalT) -> PgprotT { x }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
