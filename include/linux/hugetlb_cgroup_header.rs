/*
 * Copyright IBM Corporation, 2012
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 */

// Dependency: linux/mmdebug.h

pub struct hugetlb_cgroup;
pub struct resv_map;
pub struct file_region;

// The following items are enabled when CONFIG_CGROUP_HUGETLB is defined.
#[cfg(CONFIG_CGROUP_HUGETLB)]
#[repr(C)]
pub enum hugetlb_memory_event {
    HUGETLB_MAX,
    HUGETLB_NR_MEMORY_EVENTS,
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[repr(C)]
pub struct hugetlb_cgroup_per_node {
    /* hugetlb usage in pages over all hstates. */
    pub usage: [::core::ffi::c_ulong; HUGE_MAX_HSTATE],
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[repr(C)]
pub struct hugetlb_cgroup {
    pub css: cgroup_subsys_state,
    /* the counter to account for hugepages from hugetlb. */
    pub hugepage: [page_counter; HUGE_MAX_HSTATE],
    /* the counter to account for hugepage reservations from hugetlb. */
    pub rsvd_hugepage: [page_counter; HUGE_MAX_HSTATE],
    pub events: [[atomic_long_t; HUGETLB_NR_MEMORY_EVENTS]; HUGE_MAX_HSTATE],
    pub events_local: [[atomic_long_t; HUGETLB_NR_MEMORY_EVENTS]; HUGE_MAX_HSTATE],
    /* Handle for "hugetlb.events" */
    pub events_file: [cgroup_file; HUGE_MAX_HSTATE],
    /* Handle for "hugetlb.events.local" */
    pub events_local_file: [cgroup_file; HUGE_MAX_HSTATE],
    pub nodeinfo: [*mut hugetlb_cgroup_per_node; 0],
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn __hugetlb_cgroup_from_folio(folio: *mut folio, rsvd: bool) -> *mut hugetlb_cgroup {
    VM_BUG_ON_FOLIO(!folio_test_hugetlb(folio), folio);
    if rsvd { (*folio)._hugetlb_cgroup_rsvd } else { (*folio)._hugetlb_cgroup }
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn hugetlb_cgroup_from_folio(folio: *mut folio) -> *mut hugetlb_cgroup {
    __hugetlb_cgroup_from_folio(folio, false)
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn hugetlb_cgroup_from_folio_rsvd(folio: *mut folio) -> *mut hugetlb_cgroup {
    __hugetlb_cgroup_from_folio(folio, true)
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn __set_hugetlb_cgroup(folio: *mut folio, h_cg: *mut hugetlb_cgroup, rsvd: bool) {
    VM_BUG_ON_FOLIO(!folio_test_hugetlb(folio), folio);
    if rsvd { (*folio)._hugetlb_cgroup_rsvd = h_cg; } else { (*folio)._hugetlb_cgroup = h_cg; }
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn set_hugetlb_cgroup(folio: *mut folio, h_cg: *mut hugetlb_cgroup) {
    __set_hugetlb_cgroup(folio, h_cg, false);
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn set_hugetlb_cgroup_rsvd(folio: *mut folio, h_cg: *mut hugetlb_cgroup) {
    __set_hugetlb_cgroup(folio, h_cg, true);
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn hugetlb_cgroup_disabled() -> bool { !cgroup_subsys_enabled(hugetlb_cgrp_subsys) }

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn hugetlb_cgroup_put_rsvd_cgroup(h_cg: *mut hugetlb_cgroup) { css_put(&mut (*h_cg).css); }

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn resv_map_dup_hugetlb_cgroup_uncharge_info(resv_map: *mut resv_map) {
    if !(*resv_map).css.is_null() { css_get((*resv_map).css); }
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
#[inline]
pub unsafe fn resv_map_put_hugetlb_cgroup_uncharge_info(resv_map: *mut resv_map) {
    if !(*resv_map).css.is_null() { css_put((*resv_map).css); }
}

#[cfg(CONFIG_CGROUP_HUGETLB)]
extern "C" {
    pub fn hugetlb_cgroup_charge_cgroup(idx: i32, nr_pages: ::core::ffi::c_ulong, ptr: *mut *mut hugetlb_cgroup) -> i32;
    pub fn hugetlb_cgroup_charge_cgroup_rsvd(idx: i32, nr_pages: ::core::ffi::c_ulong, ptr: *mut *mut hugetlb_cgroup) -> i32;
    pub fn hugetlb_cgroup_commit_charge(idx: i32, nr_pages: ::core::ffi::c_ulong, h_cg: *mut hugetlb_cgroup, folio: *mut folio);
    pub fn hugetlb_cgroup_commit_charge_rsvd(idx: i32, nr_pages: ::core::ffi::c_ulong, h_cg: *mut hugetlb_cgroup, folio: *mut folio);
    pub fn hugetlb_cgroup_uncharge_folio(idx: i32, nr_pages: ::core::ffi::c_ulong, folio: *mut folio);
    pub fn hugetlb_cgroup_uncharge_folio_rsvd(idx: i32, nr_pages: ::core::ffi::c_ulong, folio: *mut folio);
    pub fn hugetlb_cgroup_uncharge_cgroup(idx: i32, nr_pages: ::core::ffi::c_ulong, h_cg: *mut hugetlb_cgroup);
    pub fn hugetlb_cgroup_uncharge_cgroup_rsvd(idx: i32, nr_pages: ::core::ffi::c_ulong, h_cg: *mut hugetlb_cgroup);
    pub fn hugetlb_cgroup_uncharge_counter(resv: *mut resv_map, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn hugetlb_cgroup_uncharge_file_region(resv: *mut resv_map, rg: *mut file_region, nr_pages: ::core::ffi::c_ulong, region_del: bool);
    pub fn hugetlb_cgroup_file_init(); // C declaration has __init.
    pub fn hugetlb_cgroup_migrate(old_folio: *mut folio, new_folio: *mut folio);
}

// CONFIG_CGROUP_HUGETLB disabled: direct translations of the C no-op stubs.
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_file_region(_: *mut resv_map, _: *mut file_region, _: ::core::ffi::c_ulong, _: bool) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_from_folio(_: *mut folio) -> *mut hugetlb_cgroup { core::ptr::null_mut() }
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_from_folio_rsvd(_: *mut folio) -> *mut hugetlb_cgroup { core::ptr::null_mut() }
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn set_hugetlb_cgroup(_: *mut folio, _: *mut hugetlb_cgroup) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn set_hugetlb_cgroup_rsvd(_: *mut folio, _: *mut hugetlb_cgroup) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_disabled() -> bool { true }
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_put_rsvd_cgroup(_: *mut hugetlb_cgroup) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn resv_map_dup_hugetlb_cgroup_uncharge_info(_: *mut resv_map) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn resv_map_put_hugetlb_cgroup_uncharge_info(_: *mut resv_map) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_charge_cgroup(_: i32, _: ::core::ffi::c_ulong, _: *mut *mut hugetlb_cgroup) -> i32 { 0 }
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_charge_cgroup_rsvd(_: i32, _: ::core::ffi::c_ulong, _: *mut *mut hugetlb_cgroup) -> i32 { 0 }
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_commit_charge(_: i32, _: ::core::ffi::c_ulong, _: *mut hugetlb_cgroup, _: *mut folio) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_commit_charge_rsvd(_: i32, _: ::core::ffi::c_ulong, _: *mut hugetlb_cgroup, _: *mut folio) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_folio(_: i32, _: ::core::ffi::c_ulong, _: *mut folio) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_folio_rsvd(_: i32, _: ::core::ffi::c_ulong, _: *mut folio) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_cgroup(_: i32, _: ::core::ffi::c_ulong, _: *mut hugetlb_cgroup) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_cgroup_rsvd(_: i32, _: ::core::ffi::c_ulong, _: *mut hugetlb_cgroup) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_uncharge_counter(_: *mut resv_map, _: ::core::ffi::c_ulong, _: ::core::ffi::c_ulong) {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_file_init() {}
#[cfg(not(CONFIG_CGROUP_HUGETLB))]
#[inline] pub unsafe fn hugetlb_cgroup_migrate(_: *mut folio, _: *mut folio) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
