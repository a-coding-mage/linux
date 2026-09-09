// SPDX-License-Identifier: GPL-2.0
/*
 *	crash_dump.c - Memory preserving reboot related code.
 *
 *	Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
 *	Copyright (C) IBM Corporation, 2004. All rights reserved
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/errno.h, linux/crash_dump.h, linux/io.h, linux/uio.h,
// linux/uaccess.h

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: ::core::primitive::c_ulong,
    mut csize: ::core::primitive::usize,
    offset: ::core::primitive::c_ulong,
) -> ssize_t {
    let mut vaddr: *mut ::core::ffi::c_void;

    if csize == 0 {
        return 0;
    }

    vaddr = ioremap(pfn << PAGE_SHIFT, PAGE_SIZE);
    csize = copy_to_iter(vaddr.add(offset as usize), csize, iter);
    iounmap(vaddr);

    csize as ssize_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
