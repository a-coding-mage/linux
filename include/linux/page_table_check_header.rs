/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2021, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

/* CONFIG_PAGE_TABLE_CHECK is a build-time condition from the C header. */
#[cfg(CONFIG_PAGE_TABLE_CHECK)]
extern "C" {
    pub static mut page_table_check_disabled: static_key_true;
    pub static mut page_table_check_ops: page_ext_operations;

    pub fn __page_table_check_zero(page: *mut page, order: core::ffi::c_uint);
    pub fn __page_table_check_pte_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pte: pte_t);
    pub fn __page_table_check_pmd_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pmd: pmd_t);
    pub fn __page_table_check_pud_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pud: pud_t);
    pub fn __page_table_check_ptes_set(
        mm: *mut mm_struct,
        addr: core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        nr: core::ffi::c_uint,
    );
    pub fn __page_table_check_pmds_set(
        mm: *mut mm_struct,
        addr: core::ffi::c_ulong,
        pmdp: *mut pmd_t,
        pmd: pmd_t,
        nr: core::ffi::c_uint,
    );
    pub fn __page_table_check_puds_set(
        mm: *mut mm_struct,
        addr: core::ffi::c_ulong,
        pudp: *mut pud_t,
        pud: pud_t,
        nr: core::ffi::c_uint,
    );
    pub fn __page_table_check_pte_clear_range(
        mm: *mut mm_struct,
        addr: core::ffi::c_ulong,
        pmd: pmd_t,
    );
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_alloc(page: *mut page, order: core::ffi::c_uint) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) {
        return;
    }
    __page_table_check_zero(page, order);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_free(page: *mut page, order: core::ffi::c_uint) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_zero(page, order);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_pte_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pte: pte_t) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_pte_clear(mm, addr, pte);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_pmd_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pmd: pmd_t) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_pmd_clear(mm, addr, pmd);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_pud_clear(mm: *mut mm_struct, addr: core::ffi::c_ulong, pud: pud_t) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_pud_clear(mm, addr, pud);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_ptes_set(mm: *mut mm_struct, addr: core::ffi::c_ulong, ptep: *mut pte_t, pte: pte_t, nr: core::ffi::c_uint) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_ptes_set(mm, addr, ptep, pte, nr);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_pmds_set(mm: *mut mm_struct, addr: core::ffi::c_ulong, pmdp: *mut pmd_t, pmd: pmd_t, nr: core::ffi::c_uint) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_pmds_set(mm, addr, pmdp, pmd, nr);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_puds_set(mm: *mut mm_struct, addr: core::ffi::c_ulong, pudp: *mut pud_t, pud: pud_t, nr: core::ffi::c_uint) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_puds_set(mm, addr, pudp, pud, nr);
}

#[cfg(CONFIG_PAGE_TABLE_CHECK)]
#[inline]
pub unsafe fn page_table_check_pte_clear_range(mm: *mut mm_struct, addr: core::ffi::c_ulong, pmd: pmd_t) {
    if static_branch_likely(unsafe { &page_table_check_disabled }) { return; }
    __page_table_check_pte_clear_range(mm, addr, pmd);
}

#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_alloc(_page: *mut page, _order: core::ffi::c_uint) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_free(_page: *mut page, _order: core::ffi::c_uint) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_pte_clear(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pte: pte_t) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_pmd_clear(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pmd: pmd_t) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_pud_clear(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pud: pud_t) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_ptes_set(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _ptep: *mut pte_t, _pte: pte_t, _nr: core::ffi::c_uint) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_pmds_set(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pmdp: *mut pmd_t, _pmd: pmd_t, _nr: core::ffi::c_uint) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_puds_set(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pudp: *mut pud_t, _pud: pud_t, _nr: core::ffi::c_uint) {}
#[cfg(not(CONFIG_PAGE_TABLE_CHECK))]
#[inline]
pub unsafe fn page_table_check_pte_clear_range(_mm: *mut mm_struct, _addr: core::ffi::c_ulong, _pmd: pmd_t) {}

#[macro_export]
macro_rules! page_table_check_pmd_set { ($mm:expr, $addr:expr, $pmdp:expr, $pmd:expr) => { $crate::page_table_check_pmds_set($mm, $addr, $pmdp, $pmd, 1) }; }
#[macro_export]
macro_rules! page_table_check_pud_set { ($mm:expr, $addr:expr, $pudp:expr, $pud:expr) => { $crate::page_table_check_puds_set($mm, $addr, $pudp, $pud, 1) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
