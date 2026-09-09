/* SPDX-License-Identifier: GPL-2.0 */
/******************************************************************************
 * Xen balloon functionality
 */

use core::ffi::{c_int, c_uint, c_ulong};

pub const RETRY_UNLIMITED: c_ulong = 0;

#[repr(C)]
pub struct balloon_stats {
    /* We aim for 'current allocation' == 'target allocation'. */
    pub current_pages: c_ulong,
    pub target_pages: c_ulong,
    pub target_unpopulated: c_ulong,
    /* Number of pages in high- and low-memory balloons. */
    pub balloon_low: c_ulong,
    pub balloon_high: c_ulong,
    pub total_pages: c_ulong,
    pub schedule_delay: c_ulong,
    pub max_schedule_delay: c_ulong,
    pub retry_count: c_ulong,
    pub max_retry_count: c_ulong,
}

pub struct page;

unsafe extern "C" {
    pub static mut balloon_stats: balloon_stats;

    pub fn balloon_set_new_target(target: c_ulong);

    pub fn xen_alloc_ballooned_pages(
        nr_pages: c_uint,
        pages: *mut *mut page,
    ) -> c_int;
    pub fn xen_free_ballooned_pages(nr_pages: c_uint, pages: *mut *mut page);
}

/* CONFIG_XEN_BALLOON controls whether the external initializer is available. */
#[cfg(feature = "CONFIG_XEN_BALLOON")]
unsafe extern "C" {
    pub fn xen_balloon_init();
}

#[cfg(not(feature = "CONFIG_XEN_BALLOON"))]
#[inline]
pub fn xen_balloon_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
