// SPDX-License-Identifier: LGPL-2.1
/*
 * Copyright IBM Corporation, 2010
 * Author Venkateswararao Jujjuri <jvrao@linux.vnet.ibm.com>
 */

// Linux kernel dependencies supplied by other translated units.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    fn put_page(page: *mut page);
}

/**
 * p9_release_pages - Release pages after the transaction.
 * @pages: array of pages to be put
 * @nr_pages: size of array
 */
#[no_mangle]
pub unsafe extern "C" fn p9_release_pages(pages: *mut *mut page, nr_pages: i32) {
    let mut i: i32 = 0;

    while i < nr_pages {
        if *pages.offset(i as isize) != core::ptr::null_mut() {
            put_page(*pages.offset(i as isize));
        }
        i += 1;
    }
}

// EXPORT_SYMBOL(p9_release_pages);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
