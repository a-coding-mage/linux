/* SPDX-License-Identifier: GPL-2.0 */
// Translated from pgtable-3level.h.
// Dependency intent: asm-generic/pgtable-nopud.h is supplied by other files.

/*
 * Some cores need a 3-level page table layout, for example when using
 * 64-bit PTEs and 4K pages.
 */
pub const PAGETABLE_LEVELS: usize = 3;

pub const PTE_MAGNITUDE: usize = 3; /* 64-bit PTEs on SH-X2 TLB */

/* PGD bits */
pub const PGDIR_SHIFT: usize = 30;

pub const PTRS_PER_PGD: usize = 4;
pub const USER_PTRS_PER_PGD: usize = 2;

/* PMD bits */
pub const PMD_SHIFT: usize = PAGE_SHIFT + (PAGE_SHIFT - PTE_MAGNITUDE);
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

pub const PTRS_PER_PMD: usize = (1usize << PGDIR_SHIFT) / PMD_SIZE;

#[repr(C)]
pub struct PmdFields {
    pub pmd_low: libc::c_ulong,
    pub pmd_high: libc::c_ulong,
}

#[repr(C)]
pub union pmd_t {
    pub fields: PmdFields,
    pub pmd: libc::c_ulonglong,
}

#[inline]
pub unsafe fn pmd_val(x: pmd_t) -> libc::c_ulonglong {
    unsafe { x.pmd }
}

#[inline]
pub const fn __pmd(x: libc::c_ulonglong) -> pmd_t {
    pmd_t { pmd: x }
}

#[inline]
pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t {
    (pud_val(pud) as libc::c_ulong as usize) as *mut pmd_t
}

/* only used by the stubbed out hugetlb gup code, should never be called */
macro_rules! pud_page {
    ($pud:expr) => {
        core::ptr::null_mut()
    };
}

macro_rules! pud_none {
    ($x:expr) => {
        pud_val($x) == 0
    };
}

macro_rules! pud_present {
    ($x:expr) => {
        pud_val($x)
    };
}

macro_rules! pud_clear {
    ($xp:expr) => {{
        set_pud($xp, __pud(0));
    }};
}

macro_rules! pud_bad {
    ($x:expr) => {
        pud_val($x) & !PAGE_MASK
    };
}

/*
 * (puds are folded into pgds so this doesn't get actually called,
 * but the define is needed for a generic inline function.)
 */
macro_rules! set_pud {
    ($pudptr:expr, $pudval:expr) => {{
        unsafe {
            *($pudptr) = $pudval;
        }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
