/* SPDX-License-Identifier: GPL-2.0 */
/*
 * S390 version. Direct Rust translation of the corresponding C header.
 * C includes and configuration-dependent assembly helpers are represented by
 * comments or macro interfaces; their definitions are supplied externally.
 */

extern "C" {
    pub fn debug_user_asce(exit: ::core::ffi::c_int);
    pub fn _copy_from_user_key(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
                               n: usize, key: usize) -> usize;
    pub fn _copy_to_user_key(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
                             n: usize, key: usize) -> usize;
    pub fn __put_user_bad() -> !;
    pub fn __get_user_bad() -> !;
    pub fn strncpy_from_user(dst: *mut ::core::ffi::c_char,
                             src: *const ::core::ffi::c_char, count: isize) -> isize;
    pub fn strnlen_user(src: *const ::core::ffi::c_char, count: isize) -> isize;
    pub fn __s390_kernel_write(dst: *mut ::core::ffi::c_void,
                               src: *const ::core::ffi::c_void, size: usize)
                               -> *mut ::core::ffi::c_void;
    pub fn __mvc_kernel_nofault_bad() -> !;
    pub fn __cmpxchg_key1(address: *mut ::core::ffi::c_void, uval: *mut u8,
                           old: u8, new: u8, key: usize) -> ::core::ffi::c_int;
    pub fn __cmpxchg_key2(address: *mut ::core::ffi::c_void, uval: *mut u16,
                           old: u16, new: u16, key: usize) -> ::core::ffi::c_int;
    pub fn __cmpxchg_key4(address: *mut ::core::ffi::c_void, uval: *mut u32,
                           old: u32, new: u32, key: usize) -> ::core::ffi::c_int;
    pub fn __cmpxchg_key8(address: *mut ::core::ffi::c_void, uval: *mut usize,
                           old: usize, new: usize, key: usize) -> ::core::ffi::c_int;
    pub fn __cmpxchg_key16(address: *mut ::core::ffi::c_void, uval: *mut u128,
                            old: u128, new: u128, key: usize) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "kmsan"))]
#[inline(always)]
unsafe fn uaccess_kmsan_or_inline<T>(f: impl FnOnce() -> T) -> T { f() }

pub const INLINE_COPY_USER: bool = true;

#[inline(always)]
pub unsafe fn raw_copy_from_user(to: *mut u8, mut from: *const u8, mut size: usize) -> usize {
    let mut osize;
    loop {
        osize = size;
        // C inline assembly: mvcos with user-access exception-table handling.
        todo!("s390 mvcos raw_copy_from_user assembly");
        /*
        if constant(osize) && osize <= 4096 { return osize - size; }
        if likely(CC_TRANSFORM(cc) == 0) { return osize - size; }
        size -= 4096; to = to.add(4096); from = from.add(4096);
        */
    }
}

#[inline(always)]
pub unsafe fn raw_copy_to_user(mut to: *mut u8, mut from: *const u8, mut size: usize) -> usize {
    let mut osize;
    loop {
        osize = size;
        // C inline assembly: mvcos with user-access exception-table handling.
        todo!("s390 mvcos raw_copy_to_user assembly");
        /* if constant(osize) && osize <= 4096 || likely(CC_TRANSFORM(cc) == 0) {
             return osize - size;
           }
           size -= 4096; to = to.add(4096); from = from.add(4096); */
    }
}

#[inline(always)]
pub unsafe fn copy_from_user_key(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
                                mut n: usize, key: usize) -> usize {
    if check_copy_size(to, n, false) { n = _copy_from_user_key(to, from, n, key); }
    n
}

#[inline(always)]
pub unsafe fn copy_to_user_key(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
                              mut n: usize, key: usize) -> usize {
    if check_copy_size(from, n, true) { n = _copy_to_user_key(to, from, n, key); }
    n
}

extern "Rust" {
    fn check_copy_size(ptr: *const ::core::ffi::c_void, n: usize, is_source: bool) -> bool;
    fn might_fault();
    fn instrument_put_user<T>(value: T, to: *mut T, size: usize);
    fn instrument_get_user<T>(value: T);
}

// Assembly-dependent DEFINE_PUT_USER_NOINSTR/DEFINE_GET_USER_NOINSTR and their
// typed wrappers retain the source macro interface for downstream expansion.
#[macro_export]
macro_rules! define_put_user_no_instr { ($($tt:tt)*) => { /* s390 mvcos */ }; }
#[macro_export]
macro_rules! define_get_user_no_instr { ($($tt:tt)*) => { /* s390 mvcos */ }; }

#[inline(always)]
pub unsafe fn __clear_user(mut to: *mut u8, mut size: usize) -> usize {
    let mut osize;
    loop {
        osize = size;
        // C inline assembly: clear user memory through mvcos from empty_zero_page.
        todo!("s390 mvcos __clear_user assembly");
        /* if constant(osize) && osize <= 4096 || CC_TRANSFORM(cc) == 0 {
             return osize - size;
           }
           size -= 4096; to = to.add(4096); */
    }
}

#[inline(always)]
pub unsafe fn clear_user(to: *mut u8, n: usize) -> usize {
    might_fault();
    __clear_user(to, n)
}

#[inline]
pub unsafe fn s390_kernel_write(dst: *mut ::core::ffi::c_void,
                                src: *const ::core::ffi::c_void, size: usize)
                                -> *mut ::core::ffi::c_void {
    if cfg!(feature = "decompressor") {
        // Equivalent to memcpy(dst, src, size), supplied by the platform.
        extern "C" { fn memcpy(dst: *mut ::core::ffi::c_void,
                                 src: *const ::core::ffi::c_void, n: usize)
                                 -> *mut ::core::ffi::c_void; }
        memcpy(dst, src, size)
    } else { __s390_kernel_write(dst, src, size) }
}

// __mvc_kernel_nofault and arch_{get,put}_kernel_nofault are configuration-
// dependent assembly macros; preserve their exported names for consumers.
#[macro_export]
macro_rules! __mvc_kernel_nofault { ($($tt:tt)*) => { todo!("s390 mvc kernel nofault assembly") }; }
pub use crate::__mvc_kernel_nofault as arch_get_kernel_nofault;
pub use crate::__mvc_kernel_nofault as arch_put_kernel_nofault;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
