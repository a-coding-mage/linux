/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm-generic/pgtable-nopud.h>.
// This header is excluded when building assembler code in the C source.
use core::ffi::c_ulong;

pub struct mm_struct;

pub const __PAGETABLE_PMD_FOLDED: usize = 1;

/*
 * Having the pmd type consist of a pud gets the size right, and allows
 * us to conceptually access the pud entry that this pmd is folded into
 * without casting.
 */
#[repr(C)]
pub struct pmd_t {
    pub pud: pud_t,
}

pub const PMD_SHIFT: usize = PUD_SHIFT;
pub const PTRS_PER_PMD: usize = 1;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

/*
 * The "pud_xxx()" functions here are trivial for a folded two-level
 * setup: the pmd is never bad, and a pmd always exists (as it's folded
 * into the pud entry)
 */
#[inline]
pub fn pud_none(_pud: pud_t) -> i32 { 0 }

#[inline]
pub fn pud_bad(_pud: pud_t) -> i32 { 0 }

#[inline]
pub fn pud_present(_pud: pud_t) -> i32 { 1 }

#[inline]
pub fn pud_user(_pud: pud_t) -> i32 { 0 }

#[inline]
pub fn pud_leaf(_pud: pud_t) -> i32 { 0 }

#[inline]
pub unsafe fn pud_clear(_pud: *mut pud_t) {}

#[inline]
pub fn pmd_ERROR(pmd: pmd_t) -> i32 {
    pud_ERROR(pmd.pud)
}

// #define pud_populate(mm, pmd, pte) do { } while (0)
#[inline]
pub fn pud_populate<T, U, V>(_mm: T, _pmd: U, _pte: V) {}

/*
 * (pmds are folded into puds so this doesn't get actually called,
 * but the define is needed for a generic inline function.)
 */
#[inline]
pub unsafe fn set_pud(pudptr: *mut pud_t, pudval: pud_t) {
    set_pmd(pudptr as *mut pmd_t, pmd_t { pud: pudval });
}

#[inline]
pub unsafe fn pmd_offset(pud: *mut pud_t, _address: c_ulong) -> *mut pmd_t {
    pud as *mut pmd_t
}

#[inline]
pub fn pmd_val(x: pmd_t) -> c_ulong {
    pud_val(x.pud)
}

#[inline]
pub fn __pmd(x: c_ulong) -> pmd_t {
    pmd_t { pud: __pud(x) }
}

#[inline]
pub fn pud_page(pud: pud_t) -> c_ulong {
    pmd_page(pmd_t { pud })
}

#[inline]
pub unsafe fn pud_pgtable(pud: pud_t) -> *mut pmd_t {
    pmd_page_vaddr(pmd_t { pud }) as *mut pmd_t
}

/*
 * allocating and freeing a pmd is trivial: the 1-entry pmd is
 * inside the pud, so has no extra memory associated with it.
 */
pub unsafe fn pmd_alloc_one<T>(_mm: T, _address: c_ulong) -> *mut pmd_t {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn pmd_free(_mm: *mut mm_struct, _pmd: *mut pmd_t) {}

// #define pmd_free_tlb(tlb, x, a) do { } while (0)
#[inline]
pub fn pmd_free_tlb<T, U, V>(_tlb: T, _x: U, _a: V) {}

#[inline]
pub fn pmd_addr_end(_addr: c_ulong, end: c_ulong) -> c_ulong { end }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
