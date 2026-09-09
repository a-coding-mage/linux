// SPDX-License-Identifier: GPL-2.0+
/*
 * test_free_pages.c: Check that free_pages() doesn't leak memory
 * Copyright (c) 2020 Oracle
 * Author: Matthew Wilcox <willy@infradead.org>
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the Linux kernel headers and implementation.
type gfp_t = u32;

#[repr(C)]
struct page {
    _private: [u8; 0],
}

unsafe extern "C" {
    static GFP_KERNEL: gfp_t;
    static __GFP_COMP: gfp_t;

    fn __get_free_pages(gfp: gfp_t, order: c_int) -> usize;
    fn virt_to_page(addr: *mut c_void) -> *mut page;
    fn get_page(page: *mut page);
    fn free_pages(addr: usize, order: c_int);
    fn put_page(page: *mut page);
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn test_free_pages(gfp: gfp_t) {
    let mut i: u32;

    i = 0;
    while i < 1000 * 1000 {
        let addr: usize = __get_free_pages(gfp, 3);
        let page: *mut page = virt_to_page(addr as *mut c_void);

        /* Simulate page cache getting a speculative reference */
        get_page(page);
        free_pages(addr, 3);
        put_page(page);

        i += 1;
    }
}

unsafe fn m_in() -> c_int {
    pr_info(c"Testing with GFP_KERNEL\n".as_ptr());
    test_free_pages(GFP_KERNEL);
    pr_info(c"Testing with GFP_KERNEL | __GFP_COMP\n".as_ptr());
    test_free_pages(GFP_KERNEL | __GFP_COMP);
    pr_info(c"Test completed\n".as_ptr());

    0
}

unsafe fn m_ex() {
}

// module_init(m_in);
// module_exit(m_ex);
// MODULE_AUTHOR("Matthew Wilcox <willy@infradead.org>");
// MODULE_DESCRIPTION("Check that free_pages() doesn't leak memory");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
