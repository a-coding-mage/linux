/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

#[cfg(feature = "CONFIG_PAGE_REPORTING")]
extern "C" {
    pub static page_reporting_enabled: StaticKey;
    pub static mut page_reporting_order: ::core::ffi::c_uint;
    pub fn __page_reporting_notify();
}

#[cfg(feature = "CONFIG_PAGE_REPORTING")]
#[inline]
pub unsafe fn page_reported(page: *mut page) -> bool {
    static_branch_unlikely(&page_reporting_enabled) && PageReported(page)
}

/**
 * page_reporting_notify_free - Free page notification to start page processing
 *
 * This function is meant to act as a screener for __page_reporting_notify
 * which will determine if a give zone has crossed over the high-water mark
 * that will justify us beginning page treatment. If we have crossed that
 * threshold then it will start the process of pulling some pages and
 * placing them in the batch list for treatment.
 */
#[cfg(feature = "CONFIG_PAGE_REPORTING")]
#[inline]
pub unsafe fn page_reporting_notify_free(order: ::core::ffi::c_uint) {
    /* Called from hot path in __free_one_page() */
    if !static_branch_unlikely(&page_reporting_enabled) {
        return;
    }

    /* Determine if we have crossed reporting threshold */
    if order < page_reporting_order {
        return;
    }

    /* This will add a few cycles, but should be called infrequently */
    __page_reporting_notify();
}

#[cfg(not(feature = "CONFIG_PAGE_REPORTING"))]
#[inline]
pub unsafe fn page_reported(_page: *mut page) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_PAGE_REPORTING"))]
#[inline]
pub unsafe fn page_reporting_notify_free(_order: ::core::ffi::c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
