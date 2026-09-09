/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted; this file is the translated header body.
// The contents correspond to the non-assembler configuration.

pub const __PAGETABLE_P4D_FOLDED: i32 = 1;

#[repr(C)]
pub struct p4d_t {
    pub pgd: pgd_t,
}

pub const P4D_SHIFT: usize = PGDIR_SHIFT;
pub const PTRS_PER_P4D: usize = 1;
pub const P4D_SIZE: usize = 1usize << P4D_SHIFT;
pub const P4D_MASK: usize = !(P4D_SIZE - 1);

/*
 * The "pgd_xxx()" functions here are trivial for a folded two-level
 * setup: the p4d is never bad, and a p4d always exists (as it's folded
 * into the pgd entry)
 */
#[inline]
pub fn pgd_none(_pgd: pgd_t) -> i32 {
    0
}

#[inline]
pub fn pgd_bad(_pgd: pgd_t) -> i32 {
    0
}

#[inline]
pub fn pgd_present(_pgd: pgd_t) -> i32 {
    1
}

#[inline]
pub unsafe fn pgd_clear(_pgd: *mut pgd_t) {}

#[inline]
pub fn p4d_ERROR(p4d: p4d_t) {
    pgd_ERROR(p4d.pgd)
}

#[macro_export]
macro_rules! pgd_populate {
    ($mm:expr, $pgd:expr, $p4d:expr) => {{
        let _ = ($mm, $pgd, $p4d);
    }};
}

#[macro_export]
macro_rules! pgd_populate_safe {
    ($mm:expr, $pgd:expr, $p4d:expr) => {{
        let _ = ($mm, $pgd, $p4d);
    }};
}

/*
 * (p4ds are folded into pgds so this doesn't get actually called,
 * but the define is needed for a generic inline function.)
 */
#[macro_export]
macro_rules! set_pgd {
    ($pgdptr:expr, $pgdval:expr) => {{
        set_p4d($pgdptr as *mut p4d_t, p4d_t { pgd: $pgdval });
    }};
}

#[inline]
pub unsafe fn p4d_offset(pgd: *mut pgd_t, _address: c_ulong) -> *mut p4d_t {
    pgd as *mut p4d_t
}

#[inline]
pub fn p4d_val(x: p4d_t) -> pgd_val_t {
    pgd_val(x.pgd)
}

#[inline]
pub fn __p4d(x: pgd_val_t) -> p4d_t {
    p4d_t { pgd: __pgd(x) }
}

#[inline]
pub fn pgd_page(pgd: pgd_t) -> _p4d_page_t {
    p4d_page(p4d_t { pgd })
}

#[inline]
pub fn pgd_page_vaddr(pgd: pgd_t) -> c_ulong {
    p4d_pgtable(p4d_t { pgd }) as c_ulong
}

/*
 * allocating and freeing a p4d is trivial: the 1-entry p4d is
 * inside the pgd, so has no extra memory associated with it.
 */
#[inline]
pub fn p4d_alloc_one(_mm: *mut core::ffi::c_void, _address: c_ulong) -> *mut p4d_t {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn p4d_free(_mm: *mut core::ffi::c_void, _x: *mut p4d_t) {}

#[inline]
pub unsafe fn p4d_free_tlb(_tlb: *mut core::ffi::c_void, _x: *mut p4d_t, _a: c_ulong) {}

#[inline]
pub fn p4d_addr_end(_addr: c_ulong, end: c_ulong) -> c_ulong {
    end
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
