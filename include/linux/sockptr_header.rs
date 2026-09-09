/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 Christoph Hellwig.
 *
 * Support for "universal" pointers that can point to either kernel or userspace
 * memory.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

// Dependencies supplied by the kernel's slab and uaccess headers.
#[repr(C)]
pub union SockptrUnion {
    pub kernel: *mut c_void,
    pub user: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockptr_t {
    pub ptr: SockptrUnion,
    pub is_kernel: bool,
}

extern "C" {
    fn copy_from_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn copy_struct_from_user(dst: *mut c_void, ksize: usize, src: *const c_void, usize_: usize) -> c_int;
    fn copy_struct_from_bounce_buffer(dst: *mut c_void, ksize: usize, src: *const c_void, usize_: usize) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
    fn copy_struct_to_user(dst: *mut c_void, usize_: usize, src: *const c_void, ksize: usize,
                           ignored_trailing: *mut bool) -> c_int;
    fn copy_struct_to_bounce_buffer(dst: *mut c_void, usize_: usize, src: *const c_void,
                                    ksize: usize, ignored_trailing: *mut bool) -> c_int;
    fn kmalloc_track_caller_noprof(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn err_ptr(error: c_long) -> *mut c_void;
    fn strnlen(src: *const c_char, maxlen: usize) -> usize;
    fn strncpy_from_user(dst: *mut c_char, src: *const c_void, count: usize) -> c_long;
    fn memchr_inv(src: *const c_void, value: c_int, size: usize) -> *mut c_void;
}

pub const EINVAL: c_int = 22;
pub const EFAULT: c_int = 14;
pub const ENOMEM: c_long = 12;
pub const GFP_USER: c_uint = 0;
pub const GFP_KERNEL: c_uint = 0;
pub const __GFP_NOWARN: c_uint = 0;

#[inline]
pub unsafe fn sockptr_is_kernel(sockptr: sockptr_t) -> bool { sockptr.is_kernel }

#[inline]
pub unsafe fn KERNEL_SOCKPTR(p: *mut c_void) -> sockptr_t {
    sockptr_t { ptr: SockptrUnion { kernel: p }, is_kernel: true }
}

#[inline]
pub unsafe fn USER_SOCKPTR(p: *mut c_void) -> sockptr_t {
    sockptr_t { ptr: SockptrUnion { user: p }, is_kernel: false }
}

#[inline]
pub unsafe fn sockptr_is_null(sockptr: sockptr_t) -> bool {
    if sockptr_is_kernel(sockptr) { sockptr.ptr.kernel.is_null() } else { sockptr.ptr.user.is_null() }
}

#[inline]
pub unsafe fn copy_from_sockptr_offset(dst: *mut c_void, src: sockptr_t, offset: usize, size: usize) -> c_int {
    if !sockptr_is_kernel(src) { return copy_from_user(dst, (src.ptr.user as *mut u8).add(offset) as *const c_void, size); }
    memcpy(dst, (src.ptr.kernel as *mut u8).add(offset) as *const c_void, size);
    0
}

#[inline]
pub unsafe fn copy_from_sockptr(dst: *mut c_void, src: sockptr_t, size: usize) -> c_int { copy_from_sockptr_offset(dst, src, 0, size) }

#[inline]
pub unsafe fn copy_safe_from_sockptr(dst: *mut c_void, ksize: usize, optval: sockptr_t, optlen: c_uint) -> c_int {
    if (optlen as usize) < ksize { return -EINVAL; }
    if copy_from_sockptr(dst, optval, ksize) != 0 { return -EFAULT; }
    0
}

#[inline]
pub unsafe fn copy_struct_from_sockptr(dst: *mut c_void, ksize: usize, src: sockptr_t, usize_: usize) -> c_int {
    if !sockptr_is_kernel(src) { copy_struct_from_user(dst, ksize, src.ptr.user, usize_) }
    else { copy_struct_from_bounce_buffer(dst, ksize, src.ptr.kernel, usize_) }
}

#[inline]
pub unsafe fn copy_to_sockptr_offset(dst: sockptr_t, offset: usize, src: *const c_void, size: usize) -> c_int {
    if !sockptr_is_kernel(dst) { return copy_to_user((dst.ptr.user as *mut u8).add(offset) as *mut c_void, src, size); }
    memcpy((dst.ptr.kernel as *mut u8).add(offset) as *mut c_void, src, size); 0
}

#[inline]
pub unsafe fn copy_to_sockptr(dst: sockptr_t, src: *const c_void, size: usize) -> c_int { copy_to_sockptr_offset(dst, 0, src, size) }

#[inline]
pub unsafe fn copy_struct_to_sockptr(dst: sockptr_t, usize_: usize, src: *const c_void, ksize: usize, ignored_trailing: *mut bool) -> c_int {
    if !sockptr_is_kernel(dst) { copy_struct_to_user(dst.ptr.user, usize_, src, ksize, ignored_trailing) }
    else { copy_struct_to_bounce_buffer(dst.ptr.kernel, usize_, src, ksize, ignored_trailing) }
}

#[inline]
pub unsafe fn memdup_sockptr_noprof(src: sockptr_t, len: usize) -> *mut c_void {
    let p = kmalloc_track_caller_noprof(len, GFP_USER | __GFP_NOWARN);
    if p.is_null() { return err_ptr(-ENOMEM); }
    if copy_from_sockptr(p, src, len) != 0 { kfree(p); return err_ptr(-EFAULT); }
    p
}

#[inline]
pub unsafe fn memdup_sockptr_nul_noprof(src: sockptr_t, len: usize) -> *mut c_char {
    let p = kmalloc_track_caller_noprof(len + 1, GFP_KERNEL) as *mut c_char;
    if p.is_null() { return err_ptr(-ENOMEM) as *mut c_char; }
    if copy_from_sockptr(p as *mut c_void, src, len) != 0 { kfree(p as *mut c_void); return err_ptr(-EFAULT) as *mut c_char; }
    *p.add(len) = 0; p
}

#[inline]
pub unsafe fn strncpy_from_sockptr(dst: *mut c_char, src: sockptr_t, count: usize) -> c_long {
    if sockptr_is_kernel(src) {
        let len = core::cmp::min(strnlen(src.ptr.kernel as *const c_char, count - 1) + 1, count);
        memcpy(dst as *mut c_void, src.ptr.kernel, len); return len as c_long;
    }
    strncpy_from_user(dst, src.ptr.user, count)
}

#[inline]
pub unsafe fn check_zeroed_sockptr(src: sockptr_t, offset: usize, size: usize) -> c_int {
    if !sockptr_is_kernel(src) { return memchr_inv((src.ptr.user as *mut u8).add(offset), 0, size).is_null() as c_int; }
    memchr_inv((src.ptr.kernel as *mut u8).add(offset), 0, size).is_null() as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
