/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/string.h.  Included Linux headers provide the types
// and helper macros referenced below.

extern "C" {
    pub fn strndup_user(src: *const core::ffi::c_char, len: isize) -> *mut core::ffi::c_char;
    pub fn memdup_user(src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn vmemdup_user(src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn memdup_user_nul(src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn sized_strscpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, size: usize) -> isize;
    pub fn sized_strscpy_pad(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, size: usize) -> isize;
    pub fn strcpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strcat(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strncat(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, n: usize) -> *mut core::ffi::c_char;
    pub fn strlcat(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, n: usize) -> usize;
    pub fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    pub fn strncmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> i32;
    pub fn strcasecmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    pub fn strncasecmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> i32;
    pub fn strchr(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
    pub fn strchrnul(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
    pub fn strnchrnul(s: *const core::ffi::c_char, n: usize, c: i32) -> *mut core::ffi::c_char;
    pub fn strnchr(s: *const core::ffi::c_char, n: usize, c: i32) -> *mut core::ffi::c_char;
    pub fn strrchr(s: *const core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
    pub fn skip_spaces(s: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strim(s: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strstr(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strnstr(a: *const core::ffi::c_char, b: *const core::ffi::c_char, n: usize) -> *mut core::ffi::c_char;
    pub fn strlen(s: *const core::ffi::c_char) -> usize;
    pub fn strnlen(s: *const core::ffi::c_char, n: usize) -> usize;
    pub fn strpbrk(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strsep(s: *mut *mut core::ffi::c_char, ct: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn strspn(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> usize;
    pub fn strcspn(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> usize;
    pub fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn memset16(dst: *mut u16, v: u16, n: usize) -> *mut core::ffi::c_void;
    pub fn memset32(dst: *mut u32, v: u32, n: usize) -> *mut core::ffi::c_void;
    pub fn memset64(dst: *mut u64, v: u64, n: usize) -> *mut core::ffi::c_void;
    pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn memscan(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    pub fn bcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    pub fn memchr(s: *const core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn memchr_inv(s: *const core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn strreplace(s: *mut core::ffi::c_char, old: i8, new: i8) -> *mut core::ffi::c_char;
    pub fn kfree_const(x: *const core::ffi::c_void);
    pub fn kstrdup(s: *const core::ffi::c_char, gfp: usize) -> *mut core::ffi::c_char;
    pub fn kstrdup_const(s: *const core::ffi::c_char, gfp: usize) -> *const core::ffi::c_char;
    pub fn kstrndup(s: *const core::ffi::c_char, len: usize, gfp: usize) -> *mut core::ffi::c_char;
    pub fn kmemdup_noprof(src: *const core::ffi::c_void, len: usize, gfp: usize) -> *mut core::ffi::c_void;
    pub fn kvmemdup(src: *const core::ffi::c_void, len: usize, gfp: usize) -> *mut core::ffi::c_void;
    pub fn kmemdup_nul(s: *const core::ffi::c_char, len: usize, gfp: usize) -> *mut core::ffi::c_char;
    pub fn argv_split(gfp: usize, s: *const core::ffi::c_char, argc: *mut i32) -> *mut *mut core::ffi::c_char;
    pub fn argv_free(argv: *mut *mut core::ffi::c_char);
    pub fn sysfs_streq(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> bool;
    pub fn match_string(array: *const *const core::ffi::c_char, n: usize, s: *const core::ffi::c_char) -> i32;
    pub fn __sysfs_match_string(array: *const *const core::ffi::c_char, n: usize, s: *const core::ffi::c_char) -> i32;
    pub fn memory_read_from_buffer(to: *mut core::ffi::c_void, count: usize, pos: *mut i64, from: *const core::ffi::c_void, available: usize) -> isize;
    pub fn ptr_to_hashval(ptr: *const core::ffi::c_void, out: *mut usize) -> i32;
    pub fn memweight(ptr: *const core::ffi::c_void, bytes: usize) -> usize;
    pub fn memcpy_and_pad(dest: *mut core::ffi::c_void, dest_len: usize, src: *const core::ffi::c_void, count: usize, pad: i32);
}

#[inline]
pub unsafe fn strstrip(str_: *mut core::ffi::c_char) -> *mut core::ffi::c_char { strim(str_) }

#[inline]
pub unsafe fn mem_is_zero(s: *const core::ffi::c_void, n: usize) -> bool { memchr_inv(s, 0, n).is_null() }

#[inline]
pub unsafe fn memcpy_flushcache(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize) { memcpy(dst, src, cnt); }

#[inline]
pub unsafe fn memzero_explicit(s: *mut core::ffi::c_void, count: usize) { memset(s, 0, count); barrier_data(s); }

#[inline]
pub unsafe fn kbasename(path: *const core::ffi::c_char) -> *const core::ffi::c_char {
    let tail = strrchr(path, b'/' as i32);
    if tail.is_null() { path } else { tail.add(1) }
}

#[inline(always)]
pub unsafe fn str_has_prefix(str_: *const core::ffi::c_char, prefix: *const core::ffi::c_char) -> usize {
    let len = strlen(prefix); if strncmp(str_, prefix, len) == 0 { len } else { 0 }
}

#[inline]
pub unsafe fn strstarts(str_: *const core::ffi::c_char, prefix: *const core::ffi::c_char) -> bool { strncmp(str_, prefix, strlen(prefix)) == 0 }

#[inline]
pub unsafe fn strends(str_: *const core::ffi::c_char, suffix: *const core::ffi::c_char) -> bool {
    let str_len = strlen(str_); let suffix_len = strlen(suffix);
    if str_len < suffix_len { false } else { strcmp(str_.add(str_len - suffix_len), suffix) == 0 }
}

// C preprocessor interfaces retained as Rust macro placeholders; their
// compile-time array/type checking is supplied by the surrounding kernel port.
#[macro_export] macro_rules! strscpy { ($dst:expr, $src:expr $(, $size:expr)?) => { unsafe { sized_strscpy($dst, $src, $($size)?) } }; }
#[macro_export] macro_rules! strscpy_pad { ($dst:expr, $src:expr $(, $size:expr)?) => { unsafe { sized_strscpy_pad($dst, $src, $($size)?) } }; }

extern "C" { fn barrier_data(ptr: *mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
