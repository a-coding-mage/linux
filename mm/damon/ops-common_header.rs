/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common Code for Data Access Monitoring
 */

// Dependency supplied by the Linux DAMON headers.

extern "C" {
    pub fn damon_get_folio(pfn: core::ffi::c_ulong) -> *mut folio;

    pub fn damon_ptep_mkold(
        pte: *mut pte_t,
        vma: *mut vm_area_struct,
        addr: core::ffi::c_ulong,
    );
    pub fn damon_pmdp_mkold(
        pmd: *mut pmd_t,
        vma: *mut vm_area_struct,
        addr: core::ffi::c_ulong,
    );
    pub fn damon_folio_mkold(folio: *mut folio);
    pub fn damon_folio_young(folio: *mut folio) -> bool;

    pub fn damon_cold_score(
        c: *mut damon_ctx,
        r: *mut damon_region,
        s: *mut damos,
    ) -> core::ffi::c_int;
    pub fn damon_hot_score(
        c: *mut damon_ctx,
        r: *mut damon_region,
        s: *mut damos,
    ) -> core::ffi::c_int;

    pub fn damos_folio_filter_match(
        filter: *mut damos_filter,
        folio: *mut folio,
    ) -> bool;
    pub fn damon_migrate_pages(
        folio_list: *mut list_head,
        target_nid: core::ffi::c_int,
    ) -> core::ffi::c_ulong;

    pub fn damos_ops_has_filter(s: *mut damos) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
