// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to Linux kernel headers:
// linux/highmem.h, linux/crash_dump.h, linux/uio.h

use core::ffi::c_void;

// External declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

extern "C" {
    fn kmap_local_pfn(pfn: c_ulong) -> *mut c_void;
    fn copy_to_iter(from: *const c_void, count: usize, iter: *mut iov_iter) -> usize;
    fn kunmap_local(vaddr: *mut c_void);
}

type c_ulong = usize;

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: c_ulong,
    mut csize: usize,
    offset: c_ulong,
) -> isize {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = kmap_local_pfn(pfn);
    csize = copy_to_iter(vaddr.add(offset) as *const c_void, csize, iter);
    kunmap_local(vaddr);

    csize as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
