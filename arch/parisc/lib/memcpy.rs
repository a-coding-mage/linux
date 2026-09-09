// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *    Optimized memory copy routines.
 *
 *    Copyright (C) 2004 Randolph Chung <tausq@debian.org>
 *    Copyright (C) 2013-2017 Helge Deller <deller@gmx.de>
 *
 *    Portions derived from the GNU C Library
 *    Copyright (C) 1991, 1997, 2003 Free Software Foundation, Inc.
 */

// Kernel declarations supplied by the surrounding build.
unsafe extern "C" {
    fn mfsp(space: u32) -> u32;
    fn mtsp(space: u32, register: u32);
    fn prober_user(space: u32, address: usize) -> bool;
    fn pa_memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> usize;
}

// These constants and PAGE_ALIGN_DOWN are supplied by the kernel headers.
const SR_USER: u32 = 0;
const SR_KERNEL: u32 = 0;
const SR_TEMP1: u32 = 0;
const SR_TEMP2: u32 = 0;
const PAGE_SIZE: usize = 0;

#[inline]
unsafe fn get_user_space() -> u32 {
    mfsp(SR_USER)
}

#[inline]
unsafe fn get_kernel_space() -> u32 {
    SR_KERNEL
}

// Returns 0 for success, otherwise, returns number of bytes not transferred.
#[no_mangle]
pub unsafe extern "C" fn raw_copy_to_user(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) -> usize {
    mtsp(get_kernel_space(), SR_TEMP1);
    mtsp(get_user_space(), SR_TEMP2);
    pa_memcpy(dst, src, len)
}

// EXPORT_SYMBOL(raw_copy_to_user);

#[no_mangle]
pub unsafe extern "C" fn raw_copy_from_user(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: usize,
) -> usize {
    let mut start = src as usize;
    let end = start.wrapping_add(len);
    let mut newlen = len;

    mtsp(get_user_space(), SR_TEMP1);
    mtsp(get_kernel_space(), SR_TEMP2);

    /* Check region is user accessible */
    while start < end {
        if !prober_user(SR_TEMP1, start) {
            newlen = start.wrapping_sub(src as usize);
            break;
        }
        start = start.wrapping_add(PAGE_SIZE);
        /* align to page boundry which may have different permission */
        start &= !(PAGE_SIZE.wrapping_sub(1));
    }
    len.wrapping_sub(newlen).wrapping_add(pa_memcpy(
        dst,
        src,
        newlen,
    ))
}

// EXPORT_SYMBOL(raw_copy_from_user);

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    count: usize,
) -> *mut core::ffi::c_void {
    mtsp(get_kernel_space(), SR_TEMP1);
    mtsp(get_kernel_space(), SR_TEMP2);
    pa_memcpy(dst, src, count);
    dst
}

// EXPORT_SYMBOL(memcpy);

#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    _size: usize,
) -> bool {
    if (unsafe_src as usize) < PAGE_SIZE {
        return false;
    }
    /* check for I/O space F_EXTEND(0xfff00000) access as well? */
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
