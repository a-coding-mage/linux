/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/fortify-string.h. C preprocessor configuration is
 * intentionally preserved in comments where it has no direct Rust analogue. */

pub const FORTIFY_READ: u8 = 0;
pub const FORTIFY_WRITE: u8 = 1;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FortifyFunc {
    Strnlen, Strlen, Strscpy, Strlcat, Strcat, Strncat, Memset, Memcpy,
    Memmove, Memscan, Memcmp, Memchr, MemchrInv, Kmemdup, Strcpy, Unknown,
}

pub const fn fortify_reason_dir(r: u8) -> u8 { r & 1 }
pub const fn fortify_reason_func(r: u8) -> u8 { r >> 1 }
pub const fn fortify_reason(func: u8, write: u8) -> u8 { (func << 1) | write }

extern "C" {
    pub fn __fortify_report(reason: u8, avail: usize, size: usize);
    pub fn __fortify_panic(reason: u8, avail: usize, size: usize) -> !;
    pub fn __read_overflow() -> !;
    pub fn __read_overflow2() -> !;
    pub fn __read_overflow2_field(avail: usize, wanted: usize);
    pub fn __write_overflow() -> !;
    pub fn __write_overflow_field(avail: usize, wanted: usize);
}

/* Build-time object-size and compiler-constant predicates supplied by the
 * surrounding kernel translation. */
extern "C" {
    pub fn __member_size<T>(p: *const T) -> usize;
    pub fn __struct_size<T>(p: *const T) -> usize;
    pub fn __compiletime_strlen(p: *const core::ffi::c_char) -> usize;
}

#[inline(always)]
pub unsafe fn fortify_panic(func: FortifyFunc, write: u8, avail: usize, size: usize, _retfail: isize) -> ! {
    __fortify_panic(fortify_reason(func as u8, write), avail, size)
}

extern "C" {
    pub fn __real_strnlen(p: *const core::ffi::c_char, maxlen: usize) -> usize;
    pub fn __real_strscpy(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, size: usize) -> isize;
    pub fn __real_strlcat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, avail: usize) -> usize;
    pub fn __real_memscan(p: *mut core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void;
    pub fn __real_memchr_inv(p: *const core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn __real_kmemdup(src: *const core::ffi::c_void, len: usize, gfp: usize) -> *mut core::ffi::c_void;
}

extern "C" {
    pub fn __underlying_memcpy(p: *mut core::ffi::c_void, q: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    pub fn __underlying_memmove(p: *mut core::ffi::c_void, q: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    pub fn __underlying_memset(p: *mut core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void;
    pub fn __underlying_memchr(p: *const core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void;
    pub fn __underlying_memcmp(p: *const core::ffi::c_void, q: *const core::ffi::c_void, size: usize) -> i32;
    pub fn __underlying_strcat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn __underlying_strcpy(p: *mut core::ffi::c_char, q: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn __underlying_strlen(p: *const core::ffi::c_char) -> usize;
    pub fn __underlying_strncat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, count: usize) -> *mut core::ffi::c_char;
}

/* The following inline functions retain the header's control flow; object
 * size values are provided by the translated kernel environment. */
#[inline(always)]
pub unsafe fn strnlen(p: *const core::ffi::c_char, maxlen: usize) -> usize {
    let p_size = __member_size(p);
    let p_len = __compiletime_strlen(p);
    if p_len != usize::MAX && maxlen >= p_size { return p_len; }
    let ret = __real_strnlen(p, if maxlen < p_size { maxlen } else { p_size });
    if p_size <= ret && maxlen != ret { fortify_panic(FortifyFunc::Strnlen, FORTIFY_READ, p_size, ret + 1, ret as isize); }
    ret
}

#[inline(always)]
pub unsafe fn __fortify_strlen(p: *const core::ffi::c_char) -> usize {
    let p_size = __member_size(p);
    if p_size == usize::MAX { return __underlying_strlen(p); }
    let ret = strnlen(p, p_size);
    if p_size <= ret { fortify_panic(FortifyFunc::Strlen, FORTIFY_READ, p_size, ret + 1, ret as isize); }
    ret
}

#[inline(always)]
pub unsafe fn sized_strscpy(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, size: usize) -> isize {
    let p_size = __member_size(p); let q_size = __member_size(q);
    if p_size == usize::MAX && q_size == usize::MAX { return __real_strscpy(p, q, size); }
    let len = strnlen(q, size);
    let len = if len == size { size } else { len + 1 };
    if p_size < len { fortify_panic(FortifyFunc::Strscpy, FORTIFY_WRITE, p_size, len, -7); }
    __real_strscpy(p, q, len)
}

#[inline(always)]
pub unsafe fn strlcat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, avail: usize) -> usize {
    let p_size = __member_size(p); let q_size = __member_size(q);
    if p_size == usize::MAX && q_size == usize::MAX { return __real_strlcat(p, q, avail); }
    let p_len = strnlen(p, avail); let mut copy_len = __fortify_strlen(q); let wanted = p_len + copy_len;
    if avail <= p_len { return wanted; }
    if p_size <= p_len { fortify_panic(FortifyFunc::Strlcat, FORTIFY_READ, p_size, p_len + 1, wanted as isize); }
    let mut actual = wanted;
    if actual >= avail { copy_len = avail - p_len - 1; actual = p_len + copy_len; }
    if p_size <= actual { fortify_panic(FortifyFunc::Strlcat, FORTIFY_WRITE, p_size, actual + 1, wanted as isize); }
    __underlying_memcpy(p.add(p_len) as *mut _, q as *const _, copy_len);
    *p.add(actual) = 0; wanted
}

#[inline(always)]
pub unsafe fn strcat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    let p_size = __member_size(p); let wanted = strlcat(p, q, p_size);
    if p_size <= wanted { fortify_panic(FortifyFunc::Strcat, FORTIFY_WRITE, p_size, wanted + 1, 0); } p
}

#[inline(always)]
pub unsafe fn strncat(p: *mut core::ffi::c_char, q: *const core::ffi::c_char, count: usize) -> *mut core::ffi::c_char {
    let p_size = __member_size(p); let q_size = __member_size(q);
    if p_size == usize::MAX && q_size == usize::MAX { return __underlying_strncat(p, q, count); }
    let p_len = __fortify_strlen(p); let copy_len = strnlen(q, count); let total = p_len + copy_len + 1;
    if p_size < total { fortify_panic(FortifyFunc::Strncat, FORTIFY_WRITE, p_size, total, 0); }
    __underlying_memcpy(p.add(p_len) as *mut _, q as *const _, copy_len); *p.add(p_len + copy_len) = 0; p
}

#[inline(always)]
pub unsafe fn fortify_memset_chk(size: usize, p_size: usize, p_size_field: usize) -> bool {
    if p_size != usize::MAX && p_size < size { fortify_panic(FortifyFunc::Memset, FORTIFY_WRITE, p_size, size, 1); }
    let _ = p_size_field; false
}

#[inline(always)]
pub unsafe fn fortify_memcpy_chk(size: usize, p_size: usize, q_size: usize, _p_size_field: usize, _q_size_field: usize, func: FortifyFunc) -> bool {
    if p_size != usize::MAX && p_size < size { fortify_panic(func, FORTIFY_WRITE, p_size, size, 1); }
    if q_size != usize::MAX && q_size < size { fortify_panic(func, FORTIFY_READ, q_size, size, 1); }
    false
}

#[inline(always)]
pub unsafe fn memscan(p: *mut core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void {
    let p_size = __struct_size(p); if p_size < size { fortify_panic(FortifyFunc::Memscan, FORTIFY_READ, p_size, size, 0); } __real_memscan(p, c, size)
}

#[inline(always)]
pub unsafe fn memcmp(p: *const core::ffi::c_void, q: *const core::ffi::c_void, size: usize) -> i32 {
    let p_size = __struct_size(p); let q_size = __struct_size(q);
    if p_size < size { fortify_panic(FortifyFunc::Memcmp, FORTIFY_READ, p_size, size, i32::MIN as isize); }
    if q_size < size { fortify_panic(FortifyFunc::Memcmp, FORTIFY_READ, q_size, size, i32::MIN as isize); }
    __underlying_memcmp(p, q, size)
}

#[inline(always)]
pub unsafe fn memchr(p: *const core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void {
    let p_size = __struct_size(p); if p_size < size { fortify_panic(FortifyFunc::Memchr, FORTIFY_READ, p_size, size, 0); } __underlying_memchr(p, c, size)
}

#[inline(always)]
pub unsafe fn memchr_inv(p: *const core::ffi::c_void, c: i32, size: usize) -> *mut core::ffi::c_void {
    let p_size = __struct_size(p); if p_size < size { fortify_panic(FortifyFunc::MemchrInv, FORTIFY_READ, p_size, size, 0); } __real_memchr_inv(p, c, size)
}

#[inline(always)]
pub unsafe fn strcpy(p: *mut core::ffi::c_char, q: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    let p_size = __member_size(p); let size = __fortify_strlen(q) + 1;
    if p_size < size { fortify_panic(FortifyFunc::Strcpy, FORTIFY_WRITE, p_size, size, 0); }
    __underlying_memcpy(p as *mut _, q as *const _, size); p
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
