/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Xen memory reservation utilities.
 *
 * Copyright (c) 2003, B Dragovic
 * Copyright (c) 2003-2004, M Williamson, K Fraser
 * Copyright (c) 2005 Dan M. Smith, IBM Corporation
 * Copyright (c) 2010 Daniel Kiper
 * Copyright (c) 2018 Oleksandr Andrushchenko, EPAM Systems Inc.
 */

// Dependency intent from <linux/highmem.h> and <xen/page.h> is preserved by
// the external types and functions referenced below.

use core::ffi::{c_int, c_ulong};

extern "C" {
    pub static mut xen_scrub_pages: bool;

    pub fn clear_highpage(page: *mut page);
    pub fn xen_pv_domain() -> bool;

    #[cfg(CONFIG_XEN_HAVE_PVMMU)]
    pub fn __xenmem_reservation_va_mapping_update(
        count: c_ulong,
        pages: *mut *mut page,
        frames: *mut xen_pfn_t,
    );

    #[cfg(CONFIG_XEN_HAVE_PVMMU)]
    pub fn __xenmem_reservation_va_mapping_reset(
        count: c_ulong,
        pages: *mut *mut page,
    );

    pub fn xenmem_reservation_increase(count: c_int, frames: *mut xen_pfn_t) -> c_int;
    pub fn xenmem_reservation_decrease(count: c_int, frames: *mut xen_pfn_t) -> c_int;
}

// Supplied by the included Xen/Linux headers.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub type xen_pfn_t = c_ulong;

#[inline]
pub unsafe fn xenmem_reservation_scrub_page(page: *mut page) {
    if xen_scrub_pages {
        clear_highpage(page);
    }
}

#[inline]
pub unsafe fn xenmem_reservation_va_mapping_update(
    count: c_ulong,
    pages: *mut *mut page,
    frames: *mut xen_pfn_t,
) {
    #[cfg(CONFIG_XEN_HAVE_PVMMU)]
    {
        if xen_pv_domain() {
            __xenmem_reservation_va_mapping_update(count, pages, frames);
        }
    }
}

#[inline]
pub unsafe fn xenmem_reservation_va_mapping_reset(count: c_ulong, pages: *mut *mut page) {
    #[cfg(CONFIG_XEN_HAVE_PVMMU)]
    {
        if xen_pv_domain() {
            __xenmem_reservation_va_mapping_reset(count, pages);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
