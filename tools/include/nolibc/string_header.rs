/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * string function definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C header dependencies removed: "nolibc.h", "arch.h", "std.h". */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

unsafe extern "C" {
    fn malloc(len: usize) -> *mut c_void;
}

/*
 * As much as possible, please keep functions alphabetically sorted.
 */

pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int {
    let mut ofs: usize = 0;
    let mut c1: c_int = 0;

    while ofs < n {
        c1 = unsafe { *(s1 as *const u8).add(ofs) as c_int - *(s2 as *const u8).add(ofs) as c_int };
        if c1 != 0 {
            break;
        }
        ofs += 1;
    }
    c1
}

/* C condition preserved: #ifndef NOLIBC_ARCH_HAS_MEMMOVE */
#[cfg(not(NOLIBC_ARCH_HAS_MEMMOVE))]
/* might be ignored by the compiler without -ffreestanding, then found as
 * missing.
 */
/* C weak/unused/section attributes have no exact file-local Rust equivalent. */
pub unsafe extern "C" fn memmove(dst: *mut c_void, src: *const c_void, mut len: usize) -> *mut c_void {
    let mut dir: usize;
    let mut pos: usize;

    pos = len;
    dir = usize::MAX;

    if (dst as usize) < (src as usize) {
        pos = usize::MAX;
        dir = 1;
    }

    while len != 0 {
        pos = pos.wrapping_add(dir);
        unsafe {
            *(dst as *mut c_char).add(pos) = *(src as *const c_char).add(pos);
        }
        len -= 1;
    }
    dst
}

/* C condition preserved: #ifndef NOLIBC_ARCH_HAS_MEMCPY */
#[cfg(not(NOLIBC_ARCH_HAS_MEMCPY))]
/* must be exported, as it's used by libgcc on ARM */
/* C weak/unused/section attributes have no exact file-local Rust equivalent. */
pub unsafe extern "C" fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void {
    let mut pos: usize = 0;

    while pos < len {
        unsafe {
            *(dst as *mut c_char).add(pos) = *(src as *const c_char).add(pos);
        }
        pos += 1;
    }
    dst
}

/* C condition preserved: #ifndef NOLIBC_ARCH_HAS_MEMSET */
#[cfg(not(NOLIBC_ARCH_HAS_MEMSET))]
/* might be ignored by the compiler without -ffreestanding, then found as
 * missing.
 */
/* C weak/unused/section attributes have no exact file-local Rust equivalent. */
pub unsafe extern "C" fn memset(dst: *mut c_void, b: c_int, mut len: usize) -> *mut c_void {
    let mut p: *mut c_char = dst as *mut c_char;

    while len != 0 {
        len -= 1;
        /* prevent gcc from recognizing memset() here */
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe {
            *p = b as c_char;
            p = p.add(1);
        }
    }
    dst
}

/* C condition preserved: #ifndef NOLIBC_ARCH_HAS_MEMCHR */
#[cfg(not(NOLIBC_ARCH_HAS_MEMCHR))]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, mut len: usize) -> *mut c_void {
    let mut p: *mut c_char = s as *mut c_char;

    while len != 0 {
        len -= 1;
        if unsafe { *p == c as c_char } {
            return p as *mut c_void;
        }
        p = unsafe { p.add(1) };
    }
    ptr::null_mut()
}

pub unsafe extern "C" fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    while unsafe { *s != 0 } {
        if unsafe { *s == c as c_char } {
            return s as *mut c_char;
        }
        s = unsafe { s.add(1) };
    }
    ptr::null_mut()
}

pub unsafe extern "C" fn strcmp(mut a: *const c_char, mut b: *const c_char) -> c_int {
    let mut c: u32;
    let mut diff: c_int;

    loop {
        let av = unsafe { *(a as *const u8) };
        a = unsafe { a.add(1) };
        let bv = unsafe { *(b as *const u8) };
        b = unsafe { b.add(1) };
        c = bv as u32;
        diff = av as c_int - c as c_int;
        if diff != 0 || c == 0 {
            break;
        }
    }
    diff
}

pub unsafe extern "C" fn strcpy(mut dst: *mut c_char, mut src: *const c_char) -> *mut c_char {
    let ret: *mut c_char = dst;

    loop {
        let ch = unsafe { *src };
        unsafe {
            *dst = ch;
            dst = dst.add(1);
            src = src.add(1);
        }
        if ch == 0 {
            break;
        }
    }
    ret
}

/* this function is only used with arguments that are not constants or when
 * it's not known because optimizations are disabled. Note that gcc 12
 * recognizes an strlen() pattern and replaces it with a jump to strlen(),
 * thus itself, hence the asm() statement below that's meant to disable this
 * confusing practice.
 */
/* C weak/unused/section attributes have no exact file-local Rust equivalent. */
pub unsafe extern "C" fn strlen(str_: *const c_char) -> usize {
    let mut len: usize = 0;

    while unsafe { *str_.add(len) != 0 } {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        len += 1;
    }
    len
}

/* do not trust __builtin_constant_p() at -O0, as clang will emit a test and
 * the two branches, then will rely on an external definition of strlen().
 */
/* C condition preserved: #if defined(__OPTIMIZE__)
 * #define nolibc_strlen(x) strlen(x)
 * #define strlen(str) choose __builtin_strlen(str) for compile-time constants,
 * otherwise nolibc_strlen(str).
 */

pub unsafe extern "C" fn strnlen(str_: *const c_char, maxlen: usize) -> usize {
    let mut len: usize = 0;

    while (len < maxlen) && unsafe { *str_.add(len) != 0 } {
        len += 1;
    }
    len
}

pub unsafe extern "C" fn strdup(str_: *const c_char) -> *mut c_char {
    let len: usize;
    let ret: *mut c_char;

    len = unsafe { strlen(str_) };
    ret = unsafe { malloc(len + 1) as *mut c_char };
    if !ret.is_null() {
        unsafe {
            memcpy(ret as *mut c_void, str_ as *const c_void, len + 1);
        }
    }

    ret
}

pub unsafe extern "C" fn strndup(str_: *const c_char, maxlen: usize) -> *mut c_char {
    let len: usize;
    let ret: *mut c_char;

    len = unsafe { strnlen(str_, maxlen) };
    ret = unsafe { malloc(len + 1) as *mut c_char };
    if !ret.is_null() {
        unsafe {
            memcpy(ret as *mut c_void, str_ as *const c_void, len);
            *ret.add(len) = b'\0' as c_char;
        }
    }

    ret
}

pub unsafe extern "C" fn strlcat(mut dst: *mut c_char, mut src: *const c_char, size: usize) -> usize {
    let mut len: usize = unsafe { strnlen(dst, size) };

    /*
     * We want len < size-1. But as size is unsigned and can wrap
     * around, we use len + 1 instead.
     */
    while len.wrapping_add(1) < size {
        unsafe {
            *dst.add(len) = *src;
        }
        if unsafe { *src == b'\0' as c_char } {
            break;
        }
        len += 1;
        src = unsafe { src.add(1) };
    }

    if len < size {
        unsafe {
            *dst.add(len) = b'\0' as c_char;
        }
    }

    while unsafe { *src != 0 } {
        src = unsafe { src.add(1) };
        len += 1;
    }
    src = unsafe { src.add(1) };
    let _ = src;

    len
}

pub unsafe extern "C" fn strlcpy(dst: *mut c_char, src: *const c_char, size: usize) -> usize {
    let mut len: usize;

    len = 0;
    while len < size {
        unsafe {
            *dst.add(len) = *src.add(len);
        }
        if unsafe { *dst.add(len) == 0 } {
            return len;
        }
        len += 1;
    }
    if size != 0 {
        unsafe {
            *dst.add(size - 1) = b'\0' as c_char;
        }
    }

    while unsafe { *src.add(len) != 0 } {
        len += 1;
    }

    len
}

pub unsafe extern "C" fn strncat(mut dst: *mut c_char, mut src: *const c_char, mut size: usize) -> *mut c_char {
    let orig: *mut c_char = dst;

    while unsafe { *dst != 0 } {
        dst = unsafe { dst.add(1) };
    }

    while size != 0 {
        let ch = unsafe { *src };
        unsafe {
            *dst = ch;
        }
        if ch == 0 {
            break;
        }
        src = unsafe { src.add(1) };
        dst = unsafe { dst.add(1) };
        size -= 1;
    }

    unsafe {
        *dst = 0;
    }
    orig
}

pub unsafe extern "C" fn strncmp(mut a: *const c_char, mut b: *const c_char, mut size: usize) -> c_int {
    let mut c: u32;
    let mut diff: c_int = 0;

    while size != 0 {
        size -= 1;
        let av = unsafe { *(a as *const u8) };
        a = unsafe { a.add(1) };
        let bv = unsafe { *(b as *const u8) };
        b = unsafe { b.add(1) };
        c = bv as u32;
        diff = av as c_int - c as c_int;
        if diff != 0 || c == 0 {
            break;
        }
    }

    diff
}

pub unsafe extern "C" fn strncpy(dst: *mut c_char, mut src: *const c_char, size: usize) -> *mut c_char {
    let mut len: usize;

    len = 0;
    while len < size {
        let ch = unsafe { *src };
        unsafe {
            *dst.add(len) = ch;
        }
        if ch != 0 {
            src = unsafe { src.add(1) };
        }
        len += 1;
    }
    dst
}

pub unsafe extern "C" fn strrchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    let mut ret: *const c_char = ptr::null();

    while unsafe { *s != 0 } {
        if unsafe { *s == c as c_char } {
            ret = s;
        }
        s = unsafe { s.add(1) };
    }
    ret as *mut c_char
}

pub unsafe extern "C" fn strstr(mut haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    let mut len_haystack: usize;
    let len_needle: usize;

    len_needle = unsafe { strlen(needle) };
    if len_needle == 0 {
        return ptr::null_mut();
    }

    len_haystack = unsafe { strlen(haystack) };
    while len_haystack >= len_needle {
        if unsafe { memcmp(haystack as *const c_void, needle as *const c_void, len_needle) } == 0 {
            return haystack as *mut c_char;
        }
        haystack = unsafe { haystack.add(1) };
        len_haystack -= 1;
    }

    ptr::null_mut()
}

pub unsafe extern "C" fn tolower(c: c_int) -> c_int {
    if c >= b'A' as c_int && c <= b'Z' as c_int {
        return c - b'A' as c_int + b'a' as c_int;
    }
    c
}

pub unsafe extern "C" fn toupper(c: c_int) -> c_int {
    if c >= b'a' as c_int && c <= b'z' as c_int {
        return c - b'a' as c_int + b'A' as c_int;
    }
    c
}
