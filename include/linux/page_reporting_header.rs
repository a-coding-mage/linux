/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/mmzone.h and linux/scatterlist.h

pub const PAGE_REPORTING_CAPACITY: u32 = 32;
pub const PAGE_REPORTING_ORDER_UNSPECIFIED: i32 = -1;

#[repr(C)]
pub struct page_reporting_dev_info {
    /* function that alters pages to make them "reported" */
    pub report: Option<
        unsafe extern "C" fn(
            prdev: *mut page_reporting_dev_info,
            sg: *mut scatterlist,
            nents: u32,
        ) -> i32,
    >,

    /* work struct for processing reports */
    pub work: delayed_work,

    /* Current state of page reporting */
    pub state: atomic_t,

    /* Minimal order of page reporting */
    pub order: u32,

    /* Max pages per report batch; 0 (default) means PAGE_REPORTING_CAPACITY */
    pub capacity: u32,
}

/* Tear-down and bring-up for page reporting devices */
extern "C" {
    pub fn page_reporting_unregister(prdev: *mut page_reporting_dev_info);
    pub fn page_reporting_register(prdev: *mut page_reporting_dev_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
