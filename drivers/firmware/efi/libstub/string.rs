// SPDX-License-Identifier: GPL-2.0
/*
 * Taken from:
 *  linux/lib/string.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

unsafe extern "C" {
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
}

#[cfg(not(feature = "EFI_HAVE_STRLEN"))]
/// strlen - Find the length of a string
/// @s: The string to be sized
pub unsafe fn strlen(s: *const c_char) -> usize {
    let mut sc = s;
    while *sc != 0 {
        sc = sc.add(1);
    }
    sc.offset_from(s) as usize
}

#[cfg(not(feature = "EFI_HAVE_STRNLEN"))]
/// strnlen - Find the length of a length-limited string
/// @s: The string to be sized
/// @count: The maximum number of bytes to search
pub unsafe fn strnlen(s: *const c_char, mut count: usize) -> usize {
    let mut sc = s;
    while count != 0 && *sc != 0 {
        count -= 1;
        sc = sc.add(1);
    }
    sc.offset_from(s) as usize
}

/// strstr - Find the first substring in a %NUL terminated string
/// @s1: The string to be searched
/// @s2: The string to search for
pub unsafe fn strstr(mut s1: *const c_char, s2: *const c_char) -> *mut c_char {
    let l2 = strlen(s2);
    if l2 == 0 {
        return s1 as *mut c_char;
    }
    let mut l1 = strlen(s1);
    while l1 >= l2 {
        l1 -= 1;
        if memcmp(s1 as *const c_void, s2 as *const c_void, l2) == 0 {
            return s1 as *mut c_char;
        }
        s1 = s1.add(1);
    }
    core::ptr::null_mut()
}

#[cfg(not(feature = "EFI_HAVE_STRCMP"))]
/// strcmp - Compare two strings
/// @cs: One string
/// @ct: Another string
pub unsafe fn strcmp(mut cs: *const c_char, mut ct: *const c_char) -> c_int {
    loop {
        let c1 = *cs as u8;
        let c2 = *ct as u8;
        cs = cs.add(1);
        ct = ct.add(1);
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
        if c1 == 0 {
            break;
        }
    }
    0
}

/// strncmp - Compare two length-limited strings
/// @cs: One string
/// @ct: Another string
/// @count: The maximum number of bytes to compare
pub unsafe fn strncmp(mut cs: *const c_char, mut ct: *const c_char, mut count: usize) -> c_int {
    while count != 0 {
        let c1 = *cs as u8;
        let c2 = *ct as u8;
        cs = cs.add(1);
        ct = ct.add(1);
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
        if c1 == 0 {
            break;
        }
        count -= 1;
    }
    0
}

/* Works only for digits and letters, but small and fast */
#[inline]
fn tolower(x: c_int) -> c_int {
    x | 0x20
}

unsafe fn simple_guess_base(cp: *const c_char) -> c_uint {
    if *cp == b'0' as c_char {
        if tolower(*cp.add(1) as c_int) == b'x' as c_int && isxdigit(*cp.add(2) as c_int) != 0 {
            16
        } else {
            8
        }
    } else {
        10
    }
}

/// simple_strtoull - convert a string to an unsigned long long
/// @cp: The start of the string
/// @endp: A pointer to the end of the parsed string will be placed here
/// @base: The number base to use
pub unsafe fn simple_strtoull(mut cp: *const c_char, endp: *mut *mut c_char, mut base: c_uint) -> u64 {
    let mut result: u64 = 0;
    if base == 0 {
        base = simple_guess_base(cp);
    }
    if base == 16 && *cp == b'0' as c_char && tolower(*cp.add(1) as c_int) == b'x' as c_int {
        cp = cp.add(2);
    }
    while isxdigit(*cp as c_int) != 0 {
        let value = if isdigit(*cp as c_int) != 0 {
            (*cp as c_int - b'0' as c_int) as c_uint
        } else {
            (tolower(*cp as c_int) - b'a' as c_int + 10) as c_uint
        };
        if value >= base {
            break;
        }
        result = result.wrapping_mul(base as u64).wrapping_add(value as u64);
        cp = cp.add(1);
    }
    if !endp.is_null() {
        *endp = cp as *mut c_char;
    }
    result
}

pub unsafe fn simple_strtol(cp: *const c_char, endp: *mut *mut c_char, base: c_uint) -> i64 {
    if *cp == b'-' as c_char {
        return -(simple_strtoull(cp.add(1), endp, base) as i64);
    }
    simple_strtoull(cp, endp, base) as i64
}

#[cfg(feature = "CONFIG_EFI_PARAMS_FROM_FDT")]
#[cfg(not(feature = "EFI_HAVE_STRRCHR"))]
/// strrchr - Find the last occurrence of a character in a string
/// @s: The string to be searched
/// @c: The character to search for
pub unsafe fn strrchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    let mut last = core::ptr::null();
    loop {
        if *s == c as c_char {
            last = s;
        }
        let current = *s;
        s = s.add(1);
        if current == 0 {
            break;
        }
    }
    last as *mut c_char
}

#[cfg(feature = "CONFIG_EFI_PARAMS_FROM_FDT")]
#[cfg(not(feature = "EFI_HAVE_MEMCHR"))]
/// memchr - Find a character in an area of memory.
/// @s: The memory area
/// @c: The byte to search for
/// @n: The size of the area.
///
/// returns the address of the first occurrence of @c, or %NULL
/// if @c is not found
pub unsafe fn memchr(s: *const c_void, c: c_int, mut n: usize) -> *mut c_void {
    let mut p = s as *const u8;
    while n != 0 {
        n -= 1;
        if c as u8 == *p {
            return p as *mut c_void;
        }
        p = p.add(1);
    }
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
