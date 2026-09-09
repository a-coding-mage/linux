/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <asm-generic/pgtable-nop4d.h> */

pub const __PAGETABLE_PUD_FOLDED: usize = 1;

/*
 * Having the pud type consist of a p4d gets the size right, and allows
 * us to conceptually access the p4d entry that this pud is folded into
 * without casting.
 */
#[repr(C)]
pub struct pud_t {
    pub p4d: p4d_t,
}

pub const PUD_SHIFT: usize = P4D_SHIFT;
pub const PTRS_PER_PUD: usize = 1;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);

/*
 * The "p4d_xxx()" functions here are trivial for a folded two-level
 * setup: the pud is never bad, and a pud always exists (as it's folded
 * into the p4d entry)
 */
#[inline]
pub fn p4d_none(_p4d: p4d_t) -> i32 {
    0
}

#[inline]
pub fn p4d_bad(_p4d: p4d_t) -> i32 {
    0
}

#[inline]
pub fn p4d_present(_p4d: p4d_t) -> i32 {
    1
}

#[inline]
pub fn p4d_clear(_p4d: *mut p4d_t) {
}

#[inline]
pub fn pud_ERROR(pud: pud_t) -> p4d_t {
    p4d_ERROR(pud.p4d)
}

#[inline]
pub fn p4d_populate(_mm: *mut core::ffi::c_void, _p4d: *mut p4d_t, _pud: *mut pud_t) {
}

#[inline]
pub fn p4d_populate_safe(_mm: *mut core::ffi::c_void, _p4d: *mut p4d_t, _pud: *mut pud_t) {
}

/*
 * (puds are folded into p4ds so this doesn't get actually called,
 * but the define is needed for a generic inline function.)
 */
#[inline]
pub unsafe fn set_p4d(p4dptr: *mut p4d_t, p4dval: p4d_t) {
    set_pud(p4dptr as *mut pud_t, pud_t { p4d: p4dval });
}

#[inline]
pub unsafe fn pud_offset(p4d: *mut p4d_t, _address: usize) -> *mut pud_t {
    p4d as *mut pud_t
}

#[inline]
pub fn pud_val(x: pud_t) -> p4d_t {
    p4d_val(x.p4d)
}

#[inline]
pub fn __pud(x: p4d_t) -> pud_t {
    pud_t { p4d: __p4d(x) }
}

#[inline]
pub fn p4d_page(p4d: p4d_t) -> *mut core::ffi::c_void {
    pud_page(pud_t { p4d })
}

#[inline]
pub fn p4d_pgtable(p4d: p4d_t) -> *mut pud_t {
    pud_pgtable(pud_t { p4d }) as *mut pud_t
}

/*
 * allocating and freeing a pud is trivial: the 1-entry pud is
 * inside the p4d, so has no extra memory associated with it.
 */
#[inline]
pub fn pud_alloc_one(_mm: *mut core::ffi::c_void, _address: usize) -> *mut pud_t {
    core::ptr::null_mut()
}

#[inline]
pub fn pud_free(_mm: *mut core::ffi::c_void, _x: *mut pud_t) {
}

#[inline]
pub fn pud_free_tlb(_tlb: *mut core::ffi::c_void, _x: *mut pud_t, _a: usize) {
}

#[inline]
pub fn pud_addr_end(_addr: usize, end: usize) -> usize {
    end
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
