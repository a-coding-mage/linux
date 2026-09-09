/* SPDX-License-Identifier: GPL-2.0 */

/*
 * pte_ERROR(e): pr_err("%s:%d: bad pte %08lx\\n", __FILE__, __LINE__, (e).pte_low)
 * pgd_ERROR(e): pr_err("%s:%d: bad pgd %08lx\\n", __FILE__, __LINE__, pgd_val(e))
 *
 * These are retained as macros because their logging dependencies are supplied
 * by the surrounding kernel translation unit.
 */
macro_rules! pte_ERROR {
    ($e:expr) => {
        pr_err!("{}:{}: bad pte {:08x}\\n", file!(), line!(), $e.pte_low)
    };
}

macro_rules! pgd_ERROR {
    ($e:expr) => {
        pr_err!("{}:{}: bad pgd {:08x}\\n", file!(), line!(), pgd_val($e))
    };
}

/*
 * Certain architectures need to do special things when PTEs
 * within a page table are directly modified.  Thus, the following
 * hook is made available.
 */
#[inline]
pub unsafe fn native_set_pte(ptep: *mut pte_t, pte: pte_t) {
    *ptep = pte;
}

#[inline]
pub unsafe fn native_set_pmd(pmdp: *mut pmd_t, pmd: pmd_t) {
    *pmdp = pmd;
}

#[inline]
pub unsafe fn native_set_pud(_pudp: *mut pud_t, _pud: pud_t) {}

#[inline]
pub unsafe fn native_set_pte_atomic(ptep: *mut pte_t, pte: pte_t) {
    native_set_pte(ptep, pte);
}

#[inline]
pub unsafe fn native_pmd_clear(pmdp: *mut pmd_t) {
    native_set_pmd(pmdp, __pmd(0));
}

#[inline]
pub unsafe fn native_pud_clear(_pudp: *mut pud_t) {}

#[inline]
pub unsafe fn native_pte_clear(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong, xp: *mut pte_t) {
    *xp = native_make_pte(0);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn native_ptep_get_and_clear(xp: *mut pte_t) -> pte_t {
    __pte(xchg(&mut (*xp).pte_low, 0))
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! native_ptep_get_and_clear {
    ($xp:expr) => { native_local_ptep_get_and_clear($xp) };
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn native_pmdp_get_and_clear(xp: *mut pmd_t) -> pmd_t {
    __pmd(xchg(xp as *mut pmdval_t, 0))
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! native_pmdp_get_and_clear {
    ($xp:expr) => { native_local_pmdp_get_and_clear($xp) };
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn native_pudp_get_and_clear(xp: *mut pud_t) -> pud_t {
    __pud(xchg(xp as *mut pudval_t, 0))
}

#[cfg(not(feature = "CONFIG_SMP"))]
macro_rules! native_pudp_get_and_clear {
    ($xp:expr) => { native_local_pudp_get_and_clear($xp) };
}

/* Bit manipulation helper on pte/pgoff entry */
#[inline]
pub fn pte_bitop(value: ::core::ffi::c_ulong, rightshift: ::core::ffi::c_uint,
                 mask: ::core::ffi::c_ulong, leftshift: ::core::ffi::c_uint) -> ::core::ffi::c_ulong {
    ((value >> rightshift) & mask) << leftshift
}

/*
 * Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
 * are !pte_none() && !pte_present().
 *
 * Format of swap PTEs:
 *
 *   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
 *   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
 *   <----------------- offset ------------------> 0 E <- type --> 0
 *
 *   E is the exclusive marker that is not stored in swap entries.
 */
pub const SWP_TYPE_BITS: u32 = 5;
pub const _SWP_TYPE_MASK: u32 = (1u32 << SWP_TYPE_BITS) - 1;
pub const _SWP_TYPE_SHIFT: u32 = _PAGE_BIT_PRESENT + 1;
pub const SWP_OFFSET_SHIFT: u32 = _PAGE_BIT_PROTNONE + 1;

macro_rules! MAX_SWAPFILES_CHECK {
    () => { BUILD_BUG_ON!(MAX_SWAPFILES_SHIFT > 5) };
}

macro_rules! __swp_type {
    ($x:expr) => { (($x.val >> _SWP_TYPE_SHIFT) & _SWP_TYPE_MASK) };
}
macro_rules! __swp_offset {
    ($x:expr) => { ($x.val >> SWP_OFFSET_SHIFT) };
}
macro_rules! __swp_entry {
    ($type:expr, $offset:expr) => {
        swp_entry_t { val: (($type & _SWP_TYPE_MASK) << _SWP_TYPE_SHIFT) | ($offset << SWP_OFFSET_SHIFT) }
    };
}
macro_rules! __pte_to_swp_entry {
    ($pte:expr) => { swp_entry_t { val: $pte.pte_low } };
}
macro_rules! __swp_entry_to_pte {
    ($x:expr) => { pte_t { pte: $x.val } };
}

/* We borrow bit 7 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: u64 = _PAGE_PSE;

/* No inverted PFNs on 2 level page tables */
#[inline]
pub fn protnone_mask(_val: u64) -> u64 { 0 }

#[inline]
pub fn flip_protnone_guard(_oldval: u64, val: u64, _mask: u64) -> u64 { val }

#[inline]
pub fn __pte_needs_invert(_val: u64) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
