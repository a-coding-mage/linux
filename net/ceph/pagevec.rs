// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and libceph headers.

use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub type gfp_t = u32;
pub type loff_t = i64;

extern "C" {
    fn __free_pages(page: *mut page, order: u32);
    fn kfree(ptr: *mut c_void);
    fn kmalloc_objs<T>(count: usize, flags: gfp_t) -> *mut T;
    fn __page_cache_alloc(flags: gfp_t) -> *mut page;
    fn page_address(page: *mut page) -> *mut u8;
    fn zero_user_segment(page: *mut page, start: usize, end: usize);
}

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: u32 = 12;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const ENOMEM: isize = 12;

#[inline]
unsafe fn err_ptr<T>(err: isize) -> *mut T {
    err as *mut T
}

pub unsafe fn ceph_release_page_vector(pages: *mut *mut page, num_pages: i32) {
    let mut i: i32 = 0;

    while i < num_pages {
        __free_pages(*pages.add(i as usize), 0);
        i += 1;
    }
    kfree(pages as *mut c_void);
}

pub unsafe fn ceph_alloc_page_vector(
    num_pages: i32,
    flags: gfp_t,
) -> *mut *mut page {
    let pages = kmalloc_objs::<*mut page>(num_pages as usize, flags);
    if pages.is_null() {
        return err_ptr(-ENOMEM);
    }

    let mut i: i32 = 0;
    while i < num_pages {
        *pages.add(i as usize) = __page_cache_alloc(flags);
        if (*pages.add(i as usize)).is_null() {
            ceph_release_page_vector(pages, i);
            return err_ptr(-ENOMEM);
        }
        i += 1;
    }
    pages
}

pub unsafe fn ceph_copy_from_page_vector(
    pages: *mut *mut page,
    mut data: *mut c_void,
    off: loff_t,
    len: usize,
) {
    let mut i: usize = 0;
    let mut po: usize = (off as usize) & !PAGE_MASK;
    let mut left: usize = len;

    while left > 0 {
        let l = core::cmp::min(PAGE_SIZE - po, left);
        core::ptr::copy_nonoverlapping(
            page_address(*pages.add(i)).add(po),
            data as *mut u8,
            l,
        );
        data = (data as *mut u8).add(l) as *mut c_void;
        left -= l;
        po += l;
        if po == PAGE_SIZE {
            po = 0;
            i += 1;
        }
    }
}

pub unsafe fn ceph_zero_page_vector_range(
    mut off: i32,
    mut len: i32,
    pages: *mut *mut page,
) {
    let mut i: i32 = off >> PAGE_SHIFT;

    off &= !(PAGE_MASK as i32);

    // dout("zero_page_vector_page %u~%u\n", off, len);

    // leading partial page?
    if off != 0 {
        let end = core::cmp::min(PAGE_SIZE as i32, off + len);
        // dout("zeroing %d %p head from %d\n", i, pages[i], (int)off);
        zero_user_segment(*pages.add(i as usize), off as usize, end as usize);
        len -= end - off;
        i += 1;
    }
    while len >= PAGE_SIZE as i32 {
        // dout("zeroing %d %p len=%d\n", i, pages[i], len);
        zero_user_segment(*pages.add(i as usize), 0, PAGE_SIZE);
        len -= PAGE_SIZE as i32;
        i += 1;
    }
    // trailing partial page?
    if len != 0 {
        // dout("zeroing %d %p tail to %d\n", i, pages[i], (int)len);
        zero_user_segment(*pages.add(i as usize), 0, len as usize);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
