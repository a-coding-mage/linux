// SPDX-License-Identifier: GPL-2.0
// The C source includes Linux ctype, kernel, errno, and the common lib/string.c.
// Those dependencies are supplied by the surrounding translation unit.

unsafe extern "C" {
    fn isspace(c: i32) -> i32;
    fn isxdigit(c: i32) -> i32;
    fn isdigit(c: i32) -> i32;
    fn strnlen(s: *const core::ffi::c_char, count: usize) -> usize;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        count: usize,
    ) -> *mut core::ffi::c_void;
    fn __memset64(s: *mut u64, v: u64, count: usize) -> *mut core::ffi::c_void;
    fn strlen(s: *const core::ffi::c_char) -> usize;
}

pub unsafe fn strncmp(
    mut cs: *const core::ffi::c_char,
    mut ct: *const core::ffi::c_char,
    mut count: usize,
) -> i32 {
    let mut c1: u8;
    let mut c2: u8;

    while count != 0 {
        c1 = *cs as u8;
        cs = cs.add(1);
        c2 = *ct as u8;
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

pub unsafe fn sized_strscpy(
    dst: *mut core::ffi::c_char,
    src: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let len: usize;

    if count == 0 {
        return -7; // -E2BIG
    }
    len = strnlen(src, count - 1);
    memcpy(dst as *mut core::ffi::c_void, src as *const core::ffi::c_void, len);
    *dst.add(len) = 0;
    if *src.add(len) != 0 { -7 } else { len as isize }
}

pub unsafe fn memset64(s: *mut u64, v: u64, count: usize) -> *mut core::ffi::c_void {
    __memset64(s, v, count.wrapping_mul(core::mem::size_of::<u64>()))
}

pub unsafe fn skip_spaces(mut str_: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    while isspace(*str_ as u8 as i32) != 0 {
        str_ = str_.add(1);
    }
    str_ as *mut core::ffi::c_char
}

pub unsafe fn strim(s: *mut core::ffi::c_char) -> *mut core::ffi::c_char {
    let size = strlen(s);
    if size == 0 {
        return s;
    }

    let mut end = s.add(size - 1);
    while (end as usize) >= (s as usize) && isspace(*end as u8 as i32) != 0 {
        end = end.sub(1);
    }
    *end.add(1) = 0;

    skip_spaces(s)
}

/* Works only for digits and letters, but small and fast */
#[inline]
fn tolower(x: u8) -> u8 {
    x | 0x20
}

unsafe fn simple_guess_base(cp: *const core::ffi::c_char) -> u32 {
    if *cp == b'0' as core::ffi::c_char {
        if tolower(*cp.add(1) as u8) == b'x' && isxdigit(*cp.add(2) as u8 as i32) != 0 {
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
pub unsafe fn simple_strtoull(
    mut cp: *const core::ffi::c_char,
    endp: *mut *mut core::ffi::c_char,
    mut base: u32,
) -> u64 {
    let mut result: u64 = 0;

    if base == 0 {
        base = simple_guess_base(cp);
    }
    if base == 16 && *cp == b'0' as core::ffi::c_char && tolower(*cp.add(1) as u8) == b'x' {
        cp = cp.add(2);
    }

    while isxdigit(*cp as u8 as i32) != 0 {
        let value: u32 = if isdigit(*cp as u8 as i32) != 0 {
            (*cp as u8 - b'0') as u32
        } else {
            (tolower(*cp as u8) - b'a' + 10) as u32
        };
        if value >= base {
            break;
        }
        result = result.wrapping_mul(base as u64).wrapping_add(value as u64);
        cp = cp.add(1);
    }
    if !endp.is_null() {
        *endp = cp as *mut core::ffi::c_char;
    }
    result
}

pub unsafe fn simple_strtol(
    cp: *const core::ffi::c_char,
    endp: *mut *mut core::ffi::c_char,
    base: u32,
) -> i64 {
    if *cp == b'-' as core::ffi::c_char {
        return -(simple_strtoull(cp.add(1), endp, base) as i64);
    }
    simple_strtoull(cp, endp, base) as i64
}

pub unsafe fn kstrtobool(s: *const core::ffi::c_char, res: *mut bool) -> i32 {
    if s.is_null() {
        return -22; // -EINVAL
    }

    match *s as u8 {
        b'y' | b'Y' | b'1' => {
            *res = true;
            0
        }
        b'n' | b'N' | b'0' => {
            *res = false;
            0
        }
        b'o' | b'O' => match *s.add(1) as u8 {
            b'n' | b'N' => {
                *res = true;
                0
            }
            b'f' | b'F' => {
                *res = false;
                0
            }
            _ => -22,
        },
        _ => -22,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
