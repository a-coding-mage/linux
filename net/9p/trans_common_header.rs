/* SPDX-License-Identifier: LGPL-2.1 */
/*
 * Copyright IBM Corporation, 2010
 * Author Venkateswararao Jujjuri <jvrao@linux.vnet.ibm.com>
 */

// Opaque declaration supplied by the corresponding dependency.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    pub fn p9_release_pages(pages: *mut *mut page, nr_pages: ::std::os::raw::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
