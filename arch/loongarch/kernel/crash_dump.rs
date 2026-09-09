// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel environment:
// linux/crash_dump.h, linux/io.h, and linux/uio.h.

use core::ffi::c_void;

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn __pfn_to_phys(pfn: usize) -> usize;
    fn memremap(offset: usize, size: usize, flags: usize) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn copy_to_iter(from: *const c_void, count: usize, iter: *mut iov_iter) -> usize;
}

// These values are supplied by the kernel headers/build configuration.
unsafe extern "C" {
    static PAGE_SIZE: usize;
    static MEMREMAP_WB: usize;
    static ENOMEM: isize;
}

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    mut csize: usize,
    offset: usize,
) -> isize {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = memremap(__pfn_to_phys(pfn), PAGE_SIZE, MEMREMAP_WB);
    if vaddr.is_null() {
        return -ENOMEM;
    }

    csize = copy_to_iter(vaddr.add(offset) as *const c_void, csize, iter);

    memunmap(vaddr);

    csize as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
