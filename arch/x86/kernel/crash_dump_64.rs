// SPDX-License-Identifier: GPL-2.0
/*
 *	Memory preserving reboot related code.
 *
 *	Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
 *	Copyright (C) IBM Corporation, 2004. All rights reserved
 */

// #include <linux/errno.h>
// #include <linux/crash_dump.h>
// #include <linux/uio.h>
// #include <linux/io.h>
// #include <linux/cc_platform.h>

use core::ffi::c_void;

// External types and symbols are supplied by the surrounding kernel translation.
pub enum iov_iter {}
pub enum kvec {}

type ssize_t = isize;
type size_t = usize;
type u64 = u64;

const ENOMEM: ssize_t = 12;
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1usize << PAGE_SHIFT;
const ITER_DEST: usize = 1;
const CC_ATTR_GUEST_MEM_ENCRYPT: usize = 0;

extern "C" {
    fn ioremap_encrypted(addr: usize, size: usize) -> *mut c_void;
    fn ioremap_cache(addr: usize, size: usize) -> *mut c_void;
    fn copy_to_iter(from: *const c_void, count: usize, iter: *mut iov_iter) -> usize;
    fn iounmap(addr: *mut c_void);
    fn iov_iter_kvec(
        iter: *mut iov_iter,
        direction: usize,
        kvec: *const kvec,
        nr_segs: usize,
        count: usize,
    );
    fn read_from_oldmem(iter: *mut iov_iter, count: usize, ppos: *mut u64, encrypted: bool) -> ssize_t;
    fn cc_platform_has(attribute: usize) -> bool;
}

#[repr(C)]
struct Kvec {
    iov_base: *mut c_void,
    iov_len: usize,
}

unsafe fn __copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    mut csize: usize,
    offset: usize,
    encrypted: bool,
) -> ssize_t {
    let vaddr: *mut c_void;

    if csize == 0 {
        return 0;
    }

    if encrypted {
        vaddr = ioremap_encrypted(pfn << PAGE_SHIFT, PAGE_SIZE);
    } else {
        vaddr = ioremap_cache(pfn << PAGE_SHIFT, PAGE_SIZE);
    }

    if vaddr.is_null() {
        return -ENOMEM;
    }

    csize = copy_to_iter(vaddr.add(offset), csize, iter);

    iounmap(vaddr);
    csize as ssize_t
}

pub unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    csize: usize,
    offset: usize,
) -> ssize_t {
    __copy_oldmem_page(iter, pfn, csize, offset, false)
}

/*
 * copy_oldmem_page_encrypted - same as copy_oldmem_page() above but ioremap the
 * memory with the encryption mask set to accommodate kdump on SME-enabled
 * machines.
 */
pub unsafe fn copy_oldmem_page_encrypted(
    iter: *mut iov_iter,
    pfn: usize,
    csize: usize,
    offset: usize,
) -> ssize_t {
    __copy_oldmem_page(iter, pfn, csize, offset, true)
}

pub unsafe fn elfcorehdr_read(buf: *mut u8, count: usize, ppos: *mut u64) -> ssize_t {
    let kvec = Kvec {
        iov_base: buf as *mut c_void,
        iov_len: count,
    };
    let mut iter = core::mem::MaybeUninit::<iov_iter>::uninit();

    iov_iter_kvec(iter.as_mut_ptr(), ITER_DEST, &kvec as *const Kvec as *const kvec, 1, count);

    read_from_oldmem(
        iter.as_mut_ptr(),
        count,
        ppos,
        cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
