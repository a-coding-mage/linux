// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/lib/string.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

/*
 * This file should be used only for "library" routines that may have
 * alternative implementations on specific architectures (generally
 * found in <asm-xx/string.h>), or get overloaded by FORTIFY_SOURCE.
 * (Specifically, this file is built with __NO_FORTIFY.)
 *
 * Other helper functions should live in string_helpers.c.
 */

// Dependencies: linux/bits.h, linux/bug.h, linux/ctype.h, linux/errno.h,
// linux/limits.h, linux/string.h, asm/page.h, linux/unaligned.h,
// asm/word-at-a-time.h.
//
// lib/Kconfig limits RUST_STRING to x86-64, whose asm/string_64.h supplies
// memcpy(), memset(), memset16/32/64() and memmove(); every other routine of
// this unit is defined here. The crate is #![no_builtins] (lib/string_rust.rs),
// the counterpart of the C unit's -ffreestanding: LLVM must not turn these
// loops back into calls to the routines being defined.

use core::ptr::{addr_of, copy_nonoverlapping, null_mut, read_unaligned, write_unaligned};
use kernel::bindings::{_ctype, E2BIG};
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    fn lupos_string_load_unaligned_zeropad(addr: *const c_void) -> c_ulong;
    fn lupos_string_warn_strscpy_count();
    fn lupos_string_strlcat_bug() -> !;
}

/// `_U` from linux/ctype.h; lib/string_native.c asserts it.
const CTYPE_U: u8 = 0x01;
const INT_MAX: usize = c_int::MAX as usize;
const ULONG_SIZE: usize = core::mem::size_of::<c_ulong>();

// The configured paths of sized_strscpy(), memcmp() and memchr_inv() below.
#[cfg(not(CONFIG_DCACHE_WORD_ACCESS))]
compile_error!("RUST_STRING requires CONFIG_DCACHE_WORD_ACCESS");
#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
compile_error!("RUST_STRING requires CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS");
#[cfg(not(CONFIG_ARCH_HAS_FAST_MULTIPLIER))]
compile_error!("RUST_STRING requires CONFIG_ARCH_HAS_FAST_MULTIPLIER");

/// __tolower(): the _U class of the shared _ctype table, then `c -= 'A'-'a'`.
fn tolower(c: u8) -> u8 {
    // SAFETY: the C declaration is an incomplete array; the definition has
    // 256 entries indexed by the unsigned char value.
    let mask = unsafe { *addr_of!(_ctype).cast::<u8>().add(usize::from(c)) };
    if mask & CTYPE_U != 0 {
        c.wrapping_add(b'a'.wrapping_sub(b'A'))
    } else {
        c
    }
}

/**
 * strncasecmp - Case insensitive, length-limited string comparison
 * @s1: One string
 * @s2: The other string
 * @len: the maximum number of characters to compare
 */
/// # Safety
/// `s1` and `s2` are NUL-terminated or readable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn strncasecmp(mut s1: *const c_char, mut s2: *const c_char, mut len: usize) -> c_int {
    /* Yes, Virginia, it had better be unsigned */
    let mut c1: u8;
    let mut c2: u8;

    if len == 0 {
        return 0;
    }

    // SAFETY: reads stop at a NUL in either string or after len bytes.
    unsafe {
        loop {
            c1 = *s1;
            s1 = s1.add(1);
            c2 = *s2;
            s2 = s2.add(1);
            if c1 == 0 || c2 == 0 {
                break;
            }
            if c1 != c2 {
                c1 = tolower(c1);
                c2 = tolower(c2);
                if c1 != c2 {
                    break;
                }
            }
            len -= 1;
            if len == 0 {
                break;
            }
        }
    }
    c_int::from(c1) - c_int::from(c2)
}
// EXPORT_SYMBOL(strncasecmp);

/// # Safety
/// `s1` and `s2` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strcasecmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    let mut c1: c_int;
    let mut c2: c_int;

    // SAFETY: reads stop at the first difference or NUL.
    unsafe {
        loop {
            c1 = c_int::from(tolower(*s1));
            s1 = s1.add(1);
            c2 = c_int::from(tolower(*s2));
            s2 = s2.add(1);
            if !(c1 == c2 && c1 != 0) {
                break;
            }
        }
    }
    c1 - c2
}
// EXPORT_SYMBOL(strcasecmp);

/// # Safety
/// `src` is NUL-terminated and `dest` large enough for it.
#[no_mangle]
pub unsafe extern "C" fn strcpy(mut dest: *mut c_char, mut src: *const c_char) -> *mut c_char {
    let tmp = dest;

    // SAFETY: the copy stops after the NUL terminator.
    unsafe {
        loop {
            let c = *src;
            *dest = c;
            dest = dest.add(1);
            src = src.add(1);
            if c == 0 {
                break;
            }
        }
    }
    tmp
}
// EXPORT_SYMBOL(strcpy);

#[cfg(target_endian = "big")]
const ALLBUTLAST_BYTE_MASK: c_ulong = !255;
#[cfg(target_endian = "little")]
const ALLBUTLAST_BYTE_MASK: c_ulong = !0 >> 8;

/// REPEAT_BYTE(0x01) and REPEAT_BYTE(0x80): WORD_AT_A_TIME_CONSTANTS.
const ONE_BITS: c_ulong = c_ulong::MAX / 0xff;
const HIGH_BITS: c_ulong = ONE_BITS * 0x80;

/// has_zero(): the 64-bit little-endian word-at-a-time zero-byte detector.
fn has_zero(a: c_ulong, bits: &mut c_ulong) -> c_ulong {
    let mask = (a.wrapping_sub(ONE_BITS) & !a) & HIGH_BITS;
    *bits = mask;
    mask
}

/// zero_bytemask(): bytes before the first zero byte.
fn zero_bytemask(bits: c_ulong) -> c_ulong {
    (bits.wrapping_sub(1) & !bits) >> 7
}

/// find_zero(): `__ffs(bits) >> 3`, the index of the first zero byte.
fn find_zero(bits: c_ulong) -> c_ulong {
    (bits.trailing_zeros() >> 3) as c_ulong
}

/// Copies a C string into a sized buffer, returning its length or -E2BIG.
///
/// # Safety
/// `dest` is writable for `count` bytes and `src` NUL-terminated or readable
/// for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn sized_strscpy(dest: *mut c_char, src: *const c_char, mut count: usize) -> isize {
    let mut max = count;
    let mut res: c_long = 0;

    if count == 0 || (count > INT_MAX && {
        // SAFETY: the native WARN_ON_ONCE() has no preconditions.
        unsafe { lupos_string_warn_strscpy_count() };
        true
    }) {
        return -(E2BIG as isize);
    }

    // CONFIG_DCACHE_WORD_ACCESS: load_unaligned_zeropad() may cross pages,
    // so no page-boundary limit applies to `max` (enforced at crate level).

    /*
     * load_unaligned_zeropad() or read_word_at_a_time() below may read
     * uninitialized bytes after the trailing zero and use them in
     * comparisons. Disable this optimization under KMSAN to prevent
     * false positive reports.
     */
    if cfg!(CONFIG_KMSAN) {
        max = 0;
    }

    // SAFETY: word loads use the native zero-padding fixup; stores and byte
    // accesses stay within the first `count` bytes of dest and src.
    unsafe {
        while max >= ULONG_SIZE {
            let mut data: c_ulong = 0;

            let mut c = lupos_string_load_unaligned_zeropad(src.offset(res as isize).cast::<c_void>());
            if has_zero(c, &mut data) != 0 {
                // prep_zero_mask() and create_zero_mask() are the identity.
                write_unaligned(dest.offset(res as isize).cast::<c_ulong>(), c & zero_bytemask(data));
                return (res as c_ulong).wrapping_add(find_zero(data)) as isize;
            }
            count -= ULONG_SIZE;
            if count == 0 {
                c &= ALLBUTLAST_BYTE_MASK;
                write_unaligned(dest.offset(res as isize).cast::<c_ulong>(), c);
                return -(E2BIG as isize);
            }
            write_unaligned(dest.offset(res as isize).cast::<c_ulong>(), c);
            res = res.wrapping_add(ULONG_SIZE as c_long);
            max -= ULONG_SIZE;
        }

        while count > 1 {
            let c = *src.offset(res as isize);
            *dest.offset(res as isize) = c;
            if c == 0 {
                return res as isize;
            }
            res = res.wrapping_add(1);
            count -= 1;
        }

        /* Force NUL-termination. */
        *dest.offset(res as isize) = 0;

        /* Return E2BIG if the source didn't stop */
        if *src.offset(res as isize) != 0 {
            -(E2BIG as isize)
        } else {
            res as isize
        }
    }
}
// EXPORT_SYMBOL(sized_strscpy);

/**
 * stpcpy - copy a string from src to dest returning a pointer to the new end
 *          of dest, including src's %NUL-terminator. May overrun dest.
 * @dest: pointer to end of string being copied into. Must be large enough
 *        to receive copy.
 * @src: pointer to the beginning of string being copied from. Must not overlap
 *       dest.
 *
 * stpcpy differs from strcpy in a key way: the return value is a pointer
 * to the new %NUL-terminating character in @dest. (For strcpy, the return
 * value is a pointer to the start of @dest). This interface is considered
 * unsafe as it doesn't perform bounds checking of the inputs. As such it's
 * not recommended for usage. Instead, its definition is provided in case
 * the compiler lowers other libcalls to stpcpy.
 */
/// # Safety
/// `src` is NUL-terminated and `dest` large enough for it.
#[no_mangle]
pub unsafe extern "C" fn stpcpy(mut dest: *mut c_char, mut src: *const c_char) -> *mut c_char {
    // SAFETY: the copy stops after the NUL terminator.
    unsafe {
        loop {
            let c = *src;
            *dest = c;
            dest = dest.add(1);
            src = src.add(1);
            if c == 0 {
                break;
            }
        }
        dest.sub(1)
    }
}
// EXPORT_SYMBOL(stpcpy);

/// # Safety
/// `dest` and `src` are NUL-terminated and `dest` has room for both.
#[no_mangle]
pub unsafe extern "C" fn strcat(mut dest: *mut c_char, mut src: *const c_char) -> *mut c_char {
    let tmp = dest;

    // SAFETY: dest's end is found first; the copy stops after src's NUL.
    unsafe {
        while *dest != 0 {
            dest = dest.add(1);
        }
        loop {
            let c = *src;
            *dest = c;
            dest = dest.add(1);
            src = src.add(1);
            if c == 0 {
                break;
            }
        }
    }
    tmp
}
// EXPORT_SYMBOL(strcat);

/// # Safety
/// `dest` is NUL-terminated with room for `count` more bytes and a NUL;
/// `src` is NUL-terminated or readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn strncat(mut dest: *mut c_char, mut src: *const c_char, mut count: usize) -> *mut c_char {
    let tmp = dest;

    // SAFETY: at most count bytes and a NUL are appended.
    unsafe {
        if count != 0 {
            while *dest != 0 {
                dest = dest.add(1);
            }
            loop {
                let c = *src;
                *dest = c;
                dest = dest.add(1);
                src = src.add(1);
                if c == 0 {
                    break;
                }
                count -= 1;
                if count == 0 {
                    *dest = 0;
                    break;
                }
            }
        }
    }
    tmp
}
// EXPORT_SYMBOL(strncat);

/// # Safety
/// `dest` is a NUL-terminated string in a `count`-byte buffer and `src` is
/// NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn strlcat(mut dest: *mut c_char, src: *const c_char, mut count: usize) -> usize {
    // SAFETY: the appended bytes and NUL stay inside the count-byte buffer.
    unsafe {
        let dsize = strlen(dest);
        let mut len = strlen(src);
        let res = dsize.wrapping_add(len);

        /* This would be a bug */
        if dsize >= count {
            lupos_string_strlcat_bug();
        }

        dest = dest.add(dsize);
        count -= dsize;
        if len >= count {
            len = count - 1;
        }
        copy_nonoverlapping(src, dest, len);
        *dest.add(len) = 0;
        res
    }
}
// EXPORT_SYMBOL(strlcat);

/**
 * strcmp - Compare two strings
 * @cs: One string
 * @ct: Another string
 */
/// # Safety
/// `cs` and `ct` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strcmp(mut cs: *const c_char, mut ct: *const c_char) -> c_int {
    // SAFETY: reads stop at the first difference or NUL.
    unsafe {
        loop {
            let c1: u8 = *cs;
            cs = cs.add(1);
            let c2: u8 = *ct;
            ct = ct.add(1);
            if c1 != c2 {
                return if c1 < c2 { -1 } else { 1 };
            }
            if c1 == 0 {
                break;
            }
        }
    }
    0
}
// EXPORT_SYMBOL(strcmp);

/**
 * strncmp - Compare two length-limited strings
 * @cs: One string
 * @ct: Another string
 * @count: The maximum number of bytes to compare
 */
/// # Safety
/// `cs` and `ct` are NUL-terminated or readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn strncmp(mut cs: *const c_char, mut ct: *const c_char, mut count: usize) -> c_int {
    // SAFETY: reads stop at the first difference, NUL or after count bytes.
    unsafe {
        while count != 0 {
            let c1: u8 = *cs;
            cs = cs.add(1);
            let c2: u8 = *ct;
            ct = ct.add(1);
            if c1 != c2 {
                return if c1 < c2 { -1 } else { 1 };
            }
            if c1 == 0 {
                break;
            }
            count -= 1;
        }
    }
    0
}
// EXPORT_SYMBOL(strncmp);

/**
 * strchr - Find the first occurrence of a character in a string
 * @s: The string to be searched
 * @c: The character to search for
 *
 * Note that the %NUL-terminator is considered part of the string, and can
 * be searched for.
 */
/// # Safety
/// `s` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    // SAFETY: the scan stops at the match or the NUL terminator.
    unsafe {
        while *s != c as c_char {
            if *s == 0 {
                return null_mut();
            }
            s = s.add(1);
        }
    }
    s.cast_mut()
}
// EXPORT_SYMBOL(strchr);

/**
 * strchrnul - Find and return a character in string, or end of string
 * @s: The string to be searched
 * @c: The character to search for
 *
 * Returns pointer to first occurrence of 'c' in s. If c is not found, then
 * return a pointer to the null byte at the end of s.
 */
/// # Safety
/// `s` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strchrnul(mut s: *const c_char, c: c_int) -> *mut c_char {
    // SAFETY: the scan stops at the match or the NUL terminator.
    unsafe {
        while *s != 0 && *s != c as c_char {
            s = s.add(1);
        }
    }
    s.cast_mut()
}
// EXPORT_SYMBOL(strchrnul);

/**
 * strnchrnul - Find and return a character in a length limited string,
 * or end of string
 * @s: The string to be searched
 * @count: The number of characters to be searched
 * @c: The character to search for
 *
 * Returns pointer to the first occurrence of 'c' in s. If c is not found,
 * then return a pointer to the last character of the string.
 */
/// # Safety
/// `s` is NUL-terminated or readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn strnchrnul(mut s: *const c_char, mut count: usize, c: c_int) -> *mut c_char {
    // SAFETY: the scan stops after count bytes, a match or the NUL.
    unsafe {
        while {
            let remaining = count;
            count = count.wrapping_sub(1);
            remaining != 0
        } && *s != 0
            && *s != c as c_char
        {
            s = s.add(1);
        }
    }
    s.cast_mut()
}

/**
 * strrchr - Find the last occurrence of a character in a string
 * @s: The string to be searched
 * @c: The character to search for
 */
/// # Safety
/// `s` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strrchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    let mut last: *const c_char = core::ptr::null();

    // SAFETY: the scan includes and stops at the NUL terminator.
    unsafe {
        loop {
            if *s == c as c_char {
                last = s;
            }
            let cur = *s;
            s = s.add(1);
            if cur == 0 {
                break;
            }
        }
    }
    last.cast_mut()
}
// EXPORT_SYMBOL(strrchr);

/**
 * strnchr - Find a character in a length limited string
 * @s: The string to be searched
 * @count: The number of characters to be searched
 * @c: The character to search for
 *
 * Note that the %NUL-terminator is considered part of the string, and can
 * be searched for.
 */
/// # Safety
/// `s` is NUL-terminated or readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn strnchr(mut s: *const c_char, mut count: usize, c: c_int) -> *mut c_char {
    // SAFETY: the scan stops after count bytes, a match or the NUL.
    unsafe {
        while count != 0 {
            count -= 1;
            if *s == c as c_char {
                return s.cast_mut();
            }
            let cur = *s;
            s = s.add(1);
            if cur == 0 {
                break;
            }
        }
    }
    null_mut()
}
// EXPORT_SYMBOL(strnchr);

/// # Safety
/// `s` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    let mut sc = s;

    // SAFETY: the scan stops at the NUL terminator.
    unsafe {
        while *sc != 0 {
            sc = sc.add(1);
        }
        sc.offset_from(s) as usize
    }
}
// EXPORT_SYMBOL(strlen);

/// # Safety
/// `s` is NUL-terminated or readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const c_char, mut count: usize) -> usize {
    let mut sc = s;

    // SAFETY: the scan stops after count bytes or at the NUL.
    unsafe {
        while {
            let remaining = count;
            count = count.wrapping_sub(1);
            remaining != 0
        } && *sc != 0
        {
            sc = sc.add(1);
        }
        sc.offset_from(s) as usize
    }
}
// EXPORT_SYMBOL(strnlen);

/**
 * strspn - Calculate the length of the initial substring of @s which only contain letters in @accept
 * @s: The string to be searched
 * @accept: The string to search for
 */
/// # Safety
/// `s` and `accept` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strspn(s: *const c_char, accept: *const c_char) -> usize {
    let mut p = s;

    // SAFETY: both strings are NUL-terminated.
    unsafe {
        while *p != 0 {
            if strchr(accept, c_int::from(*p)).is_null() {
                break;
            }
            p = p.add(1);
        }
        p.offset_from(s) as usize
    }
}
// EXPORT_SYMBOL(strspn);

/**
 * strcspn - Calculate the length of the initial substring of @s which does not contain letters in @reject
 * @s: The string to be searched
 * @reject: The string to avoid
 */
/// # Safety
/// `s` and `reject` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strcspn(s: *const c_char, reject: *const c_char) -> usize {
    let mut p = s;

    // SAFETY: both strings are NUL-terminated.
    unsafe {
        while *p != 0 {
            if !strchr(reject, c_int::from(*p)).is_null() {
                break;
            }
            p = p.add(1);
        }
        p.offset_from(s) as usize
    }
}
// EXPORT_SYMBOL(strcspn);

/**
 * strpbrk - Find the first occurrence of a set of characters
 * @cs: The string to be searched
 * @ct: The characters to search for
 */
/// # Safety
/// `cs` and `ct` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char {
    let mut sc = cs;

    // SAFETY: both strings are NUL-terminated.
    unsafe {
        while *sc != 0 {
            if !strchr(ct, c_int::from(*sc)).is_null() {
                return sc.cast_mut();
            }
            sc = sc.add(1);
        }
    }
    null_mut()
}
// EXPORT_SYMBOL(strpbrk);

/**
 * strsep - Split a string into tokens
 * @s: The string to be searched
 * @ct: The characters to search for
 *
 * strsep() updates @s to point after the token, ready for the next call.
 *
 * It returns empty tokens, too, behaving exactly like the libc function
 * of that name. In fact, it was stolen from glibc2 and de-fancy-fied.
 * Same semantics, slimmer shape. ;)
 */
/// # Safety
/// `s` points to NULL or a writable NUL-terminated string; `ct` is
/// NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn strsep(s: *mut *mut c_char, ct: *const c_char) -> *mut c_char {
    // SAFETY: the token end lies inside the NUL-terminated string.
    unsafe {
        let sbegin = *s;

        if sbegin.is_null() {
            return null_mut();
        }

        let mut end = strpbrk(sbegin, ct);
        if !end.is_null() {
            *end = 0;
            end = end.add(1);
        }
        *s = end;
        sbegin
    }
}
// EXPORT_SYMBOL(strsep);

/**
 * memcmp - Compare two areas of memory
 * @cs: One area of memory
 * @ct: Another area of memory
 * @count: The size of the area.
 */
/// # Safety
/// `cs` and `ct` are readable for `count` bytes.
#[no_mangle]
pub unsafe extern "C" fn memcmp(mut cs: *const c_void, mut ct: *const c_void, mut count: usize) -> c_int {
    let mut res: c_int = 0;

    // SAFETY: every load lies within the first count bytes of both areas.
    unsafe {
        // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS
        if count >= ULONG_SIZE {
            let mut u1 = cs.cast::<c_ulong>();
            let mut u2 = ct.cast::<c_ulong>();
            loop {
                if read_unaligned(u1) != read_unaligned(u2) {
                    break;
                }
                u1 = u1.add(1);
                u2 = u2.add(1);
                count -= ULONG_SIZE;
                if count < ULONG_SIZE {
                    break;
                }
            }
            cs = u1.cast::<c_void>();
            ct = u2.cast::<c_void>();
        }
        let mut su1 = cs.cast::<u8>();
        let mut su2 = ct.cast::<u8>();
        while 0 < count {
            res = c_int::from(*su1) - c_int::from(*su2);
            if res != 0 {
                break;
            }
            su1 = su1.add(1);
            su2 = su2.add(1);
            count -= 1;
        }
    }
    res
}
// EXPORT_SYMBOL(memcmp);

/**
 * bcmp - returns 0 if and only if the buffers have identical contents.
 * @a: pointer to first buffer.
 * @b: pointer to second buffer.
 * @len: size of buffers.
 *
 * The sign or magnitude of a non-zero return value has no particular
 * meaning, and architectures may implement their own more efficient bcmp(). So
 * while this particular implementation is a simple (tail) call to memcmp, do
 * not rely on anything but whether the return value is zero or non-zero.
 */
/// # Safety
/// `a` and `b` are readable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, len: usize) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe { memcmp(a, b, len) }
}
// EXPORT_SYMBOL(bcmp);

/**
 * memscan - Find a character in an area of memory.
 * @addr: The memory area
 * @c: The byte to search for
 * @size: The size of the area.
 *
 * returns the address of the first occurrence of @c, or 1 byte past
 * the area if @c is not found
 */
/// # Safety
/// `addr` is readable for `size` bytes.
#[no_mangle]
pub unsafe extern "C" fn memscan(addr: *mut c_void, c: c_int, mut size: usize) -> *mut c_void {
    let mut p = addr.cast::<u8>();

    // SAFETY: the scan stops after size bytes.
    unsafe {
        while size != 0 {
            if *p == c as u8 {
                return p.cast::<c_void>();
            }
            p = p.add(1);
            size -= 1;
        }
    }
    p.cast::<c_void>()
}
// EXPORT_SYMBOL(memscan);

/**
 * strstr - Find the first substring in a %NUL terminated string
 * @s1: The string to be searched
 * @s2: The string to search for
 */
/// # Safety
/// `s1` and `s2` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn strstr(mut s1: *const c_char, s2: *const c_char) -> *mut c_char {
    // SAFETY: each comparison reads l2 bytes of s1, within its l1 bytes.
    unsafe {
        let l2 = strlen(s2);
        if l2 == 0 {
            return s1.cast_mut();
        }
        let mut l1 = strlen(s1);
        while l1 >= l2 {
            l1 -= 1;
            if memcmp(s1.cast::<c_void>(), s2.cast::<c_void>(), l2) == 0 {
                return s1.cast_mut();
            }
            s1 = s1.add(1);
        }
    }
    null_mut()
}
// EXPORT_SYMBOL(strstr);

/**
 * strnstr - Find the first substring in a length-limited string
 * @s1: The string to be searched
 * @s2: The string to search for
 * @len: the maximum number of characters to search
 */
/// # Safety
/// `s1` is readable for `len` bytes and `s2` NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn strnstr(mut s1: *const c_char, s2: *const c_char, mut len: usize) -> *mut c_char {
    // SAFETY: each comparison reads l2 bytes of s1, within its len bytes.
    unsafe {
        let l2 = strlen(s2);
        if l2 == 0 {
            return s1.cast_mut();
        }
        while len >= l2 {
            len -= 1;
            if memcmp(s1.cast::<c_void>(), s2.cast::<c_void>(), l2) == 0 {
                return s1.cast_mut();
            }
            s1 = s1.add(1);
        }
    }
    null_mut()
}
// EXPORT_SYMBOL(strnstr);

/**
 * memchr - Find a character in an area of memory.
 * @s: The memory area
 * @c: The byte to search for
 * @n: The size of the area.
 *
 * returns the address of the first occurrence of @c, or %NULL
 * if @c is not found
 */
/// # Safety
/// `s` is readable for `n` bytes.
#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const c_void, c: c_int, mut n: usize) -> *mut c_void {
    let mut p = s.cast::<u8>();

    // SAFETY: the scan stops after n bytes.
    unsafe {
        while n != 0 {
            n -= 1;
            let cur = *p;
            p = p.add(1);
            if c as u8 == cur {
                return p.sub(1).cast_mut().cast::<c_void>();
            }
        }
    }
    null_mut()
}
// EXPORT_SYMBOL(memchr);

/// # Safety
/// `start` is readable for `bytes` bytes.
unsafe fn check_bytes8(mut start: *const u8, value: u8, mut bytes: u32) -> *mut c_void {
    // SAFETY: the scan stops after bytes bytes.
    unsafe {
        while bytes != 0 {
            if *start != value {
                return start.cast_mut().cast::<c_void>();
            }
            start = start.add(1);
            bytes -= 1;
        }
    }
    null_mut()
}

/**
 * memchr_inv - Find an unmatching character in an area of memory.
 * @start: The memory area
 * @c: Find a character other than c
 * @bytes: The size of the area.
 *
 * returns the address of the first character other than @c, or %NULL
 * if the whole buffer contains just @c.
 */
/// # Safety
/// `start` is readable for `bytes` bytes.
#[no_mangle]
pub unsafe extern "C" fn memchr_inv(start: *const c_void, c: c_int, mut bytes: usize) -> *mut c_void {
    let value = c as u8;
    let mut start = start.cast::<u8>();

    // SAFETY: every access lies within the first `bytes` bytes; the u64
    // loads are 8-byte aligned after the prefix.
    unsafe {
        if bytes <= 16 {
            return check_bytes8(start, value, bytes as u32);
        }

        // CONFIG_ARCH_HAS_FAST_MULTIPLIER && BITS_PER_LONG == 64
        let value64 = u64::from(value).wrapping_mul(0x0101010101010101);

        let mut prefix = (start as c_ulong % 8) as u32;
        if prefix != 0 {
            prefix = 8 - prefix;
            let r = check_bytes8(start, value, prefix);
            if !r.is_null() {
                return r;
            }
            start = start.add(prefix as usize);
            bytes -= prefix as usize;
        }

        let mut words = bytes / 8;

        while words != 0 {
            if *start.cast::<u64>() != value64 {
                return check_bytes8(start, value, 8);
            }
            start = start.add(8);
            words -= 1;
        }

        check_bytes8(start, value, (bytes % 8) as u32)
    }
}
// EXPORT_SYMBOL(memchr_inv);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
