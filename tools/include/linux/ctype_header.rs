/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Original C header included <linux/compiler.h>.
 *
 * NOTE! This ctype does not handle EOF like the standard C
 * library is required to.
 */

pub const _U: u8 = 0x01; /* upper */
pub const _L: u8 = 0x02; /* lower */
pub const _D: u8 = 0x04; /* digit */
pub const _C: u8 = 0x08; /* cntrl */
pub const _P: u8 = 0x10; /* punct */
pub const _S: u8 = 0x20; /* white space (space/lf/tab) */
pub const _X: u8 = 0x40; /* hex digit */
pub const _SP: u8 = 0x80; /* hard space (0x20) */

unsafe extern "C" {
    pub static _ctype: [u8; 0];
}

#[inline]
pub unsafe fn __ismask(x: i32) -> u8 {
    unsafe { *_ctype.as_ptr().add((x as u8) as usize) }
}

#[inline]
pub unsafe fn isalnum(c: i32) -> bool {
    unsafe { (__ismask(c) & (_U | _L | _D)) != 0 }
}

#[inline]
pub unsafe fn isalpha(c: i32) -> bool {
    unsafe { (__ismask(c) & (_U | _L)) != 0 }
}

#[inline]
pub unsafe fn iscntrl(c: i32) -> bool {
    unsafe { (__ismask(c) & _C) != 0 }
}

#[inline]
pub unsafe fn isgraph(c: i32) -> bool {
    unsafe { (__ismask(c) & (_P | _U | _L | _D)) != 0 }
}

#[inline]
pub unsafe fn islower(c: i32) -> bool {
    unsafe { (__ismask(c) & _L) != 0 }
}

#[inline]
pub unsafe fn isprint(c: i32) -> bool {
    unsafe { (__ismask(c) & (_P | _U | _L | _D | _SP)) != 0 }
}

#[inline]
pub unsafe fn ispunct(c: i32) -> bool {
    unsafe { (__ismask(c) & _P) != 0 }
}

/* Note: isspace() must return false for %NUL-terminator */
#[inline]
pub unsafe fn isspace(c: i32) -> bool {
    unsafe { (__ismask(c) & _S) != 0 }
}

#[inline]
pub unsafe fn isupper(c: i32) -> bool {
    unsafe { (__ismask(c) & _U) != 0 }
}

#[inline]
pub unsafe fn isxdigit(c: i32) -> bool {
    unsafe { (__ismask(c) & (_D | _X)) != 0 }
}

#[inline]
pub fn isascii(c: i32) -> bool {
    (c as u8) <= 0x7f
}

#[inline]
pub fn toascii(c: i32) -> u8 {
    (c as u8) & 0x7f
}

/*
 * Original C condition:
 * #if __has_builtin(__builtin_isdigit)
 * #define isdigit(c) __builtin_isdigit(c)
 * #else
 */
#[inline]
pub fn __isdigit(c: i32) -> i32 {
    (b'0' as i32 <= c && c <= b'9' as i32) as i32
}

#[inline]
pub fn isdigit(c: i32) -> i32 {
    __isdigit(c)
}

#[inline]
pub unsafe fn __tolower(mut c: u8) -> u8 {
    if unsafe { isupper(c as i32) } {
        c = c.wrapping_sub((b'A' as i32 - b'a' as i32) as u8);
    }
    c
}

#[inline]
pub unsafe fn __toupper(mut c: u8) -> u8 {
    if unsafe { islower(c as i32) } {
        c = c.wrapping_sub((b'a' as i32 - b'A' as i32) as u8);
    }
    c
}

#[inline]
pub unsafe fn tolower(c: u8) -> u8 {
    unsafe { __tolower(c) }
}

#[inline]
pub unsafe fn toupper(c: u8) -> u8 {
    unsafe { __toupper(c) }
}

/*
 * Fast implementation of tolower() for internal usage. Do not use in your
 * code.
 */
#[inline]
pub fn _tolower(c: i8) -> i8 {
    c | 0x20
}

/* Fast check for octal digit */
#[inline]
pub fn isodigit(c: i8) -> i32 {
    (c >= b'0' as i8 && c <= b'7' as i8) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
