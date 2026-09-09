// SPDX-License-Identifier: GPL-2.0-only
/* Very basic string functions */

use core::ffi::{c_char, c_int, c_void};

const KSTRTOX_OVERFLOW: u32 = 1u32 << 31;
const ERANGE: c_int = 34;
const EINVAL: c_int = 22;

extern "C" {
    fn isxdigit(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
}

pub unsafe fn memcmp(mut s1: *const c_void, mut s2: *const c_void, len: usize) -> c_int {
    // The assembly sets the result flag even when len is zero.
    let mut different = false;
    for _ in 0..len {
        if *(s1 as *const u8) != *(s2 as *const u8) {
            different = true;
            break;
        }
        s1 = (s1 as *const u8).add(1) as *const c_void;
        s2 = (s2 as *const u8).add(1) as *const c_void;
    }
    different as c_int
}

pub unsafe fn bcmp(s1: *const c_void, s2: *const c_void, len: usize) -> c_int {
    memcmp(s1, s2, len)
}

pub unsafe fn strcmp(mut str1: *const c_char, mut str2: *const c_char) -> c_int {
    let mut s1 = str1 as *const u8;
    let mut s2 = str2 as *const u8;
    while *s1 != 0 || *s2 != 0 {
        let delta = *s1 as c_int - *s2 as c_int;
        if delta != 0 { return delta; }
        s1 = s1.add(1);
        s2 = s2.add(1);
    }
    0
}

pub unsafe fn strncmp(mut cs: *const c_char, mut ct: *const c_char, mut count: usize) -> c_int {
    while count != 0 {
        let c1 = *cs as u8;
        let c2 = *ct as u8;
        cs = cs.add(1); ct = ct.add(1);
        if c1 != c2 { return if c1 < c2 { -1 } else { 1 }; }
        if c1 == 0 { break; }
        count -= 1;
    }
    0
}

pub unsafe fn strnlen(mut s: *const c_char, mut maxlen: usize) -> usize {
    let start = s;
    while *s != 0 && maxlen != 0 { s = s.add(1); maxlen -= 1; }
    s.offset_from(start) as usize
}

#[inline]
unsafe fn simple_guess_base(cp: *const c_char) -> u32 {
    if *cp == b'0' as c_char {
        if ((*cp.add(1) as u8) | 0x20) == b'x' && isxdigit(*cp.add(2) as c_int) != 0 { 16 } else { 8 }
    } else { 10 }
}

pub unsafe fn simple_strtoull(mut cp: *const c_char, endp: *mut *mut c_char, mut base: u32) -> u64 {
    let mut result = 0u64;
    if base == 0 { base = simple_guess_base(cp); }
    if base == 16 && *cp == b'0' as c_char && ((*cp.add(1) as u8) | 0x20) == b'x' { cp = cp.add(2); }
    while isxdigit(*cp as c_int) != 0 {
        let value = if isdigit(*cp as c_int) != 0 { *cp as u8 - b'0' } else { ((*cp as u8) | 0x20) - b'a' + 10 } as u32;
        if value >= base { break; }
        result = result.wrapping_mul(base as u64).wrapping_add(value as u64);
        cp = cp.add(1);
    }
    if !endp.is_null() { *endp = cp as *mut c_char; }
    result
}

pub unsafe fn simple_strtol(cp: *const c_char, endp: *mut *mut c_char, base: u32) -> i64 {
    if *cp == b'-' as c_char { -(simple_strtoull(cp.add(1), endp, base) as i64) } else { simple_strtoull(cp, endp, base) as i64 }
}

pub unsafe fn strlen(mut s: *const c_char) -> usize {
    let start = s;
    while *s != 0 { s = s.add(1); }
    s.offset_from(start) as usize
}

pub unsafe fn strstr(mut s1: *const c_char, s2: *const c_char) -> *mut c_char {
    let l2 = strlen(s2);
    if l2 == 0 { return s1 as *mut c_char; }
    let mut l1 = strlen(s1);
    while l1 >= l2 { l1 -= 1; if memcmp(s1 as *const c_void, s2 as *const c_void, l2) == 0 { return s1 as *mut c_char; } s1 = s1.add(1); }
    core::ptr::null_mut()
}

pub unsafe fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    while *s as c_int != c { if *s == 0 { return core::ptr::null_mut(); } s = s.add(1); }
    s as *mut c_char
}

#[inline]
unsafe fn __div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u32) -> u64 {
    let q = dividend / divisor as u64;
    *remainder = (dividend % divisor as u64) as u32;
    q
}

#[inline]
unsafe fn __div_u64(dividend: u64, divisor: u32) -> u64 { let mut remainder = 0; __div_u64_rem(dividend, divisor, &mut remainder) }

#[inline] unsafe fn _tolower(c: c_char) -> c_char { (c as u8 | 0x20) as c_char }

unsafe fn _parse_integer_fixup_radix(mut s: *const c_char, base: *mut u32) -> *const c_char {
    if *base == 0 { if *s == b'0' as c_char { if _tolower(*s.add(1)) == b'x' as c_char && isxdigit(*s.add(2) as c_int) != 0 { *base = 16; } else { *base = 8; } } else { *base = 10; } }
    if *base == 16 && *s == b'0' as c_char && _tolower(*s.add(1)) == b'x' as c_char { s = s.add(2); }
    s
}

unsafe fn _parse_integer(mut s: *const c_char, base: u32, p: *mut u64) -> u32 {
    let mut res = 0u64; let mut rv = 0u32;
    loop { let c = *s as u8; let lc = c | 0x20; let val = if c >= b'0' && c <= b'9' { c - b'0' } else if lc >= b'a' && lc <= b'f' { lc - b'a' + 10 } else { break } as u8; let val = val as u32; if val >= base { break; } if res & (!0u64 << 60) != 0 && res > __div_u64(u64::MAX - val as u64, base) { rv |= KSTRTOX_OVERFLOW; } res = res.wrapping_mul(base as u64).wrapping_add(val as u64); rv += 1; s = s.add(1); }
    *p = res; rv
}

unsafe fn _kstrtoull(mut s: *const c_char, mut base: u32, res: *mut u64) -> c_int {
    if *s == b'+' as c_char { s = s.add(1); } s = _parse_integer_fixup_radix(s, &mut base); let mut tmp = 0; let rv = _parse_integer(s, base, &mut tmp); if rv & KSTRTOX_OVERFLOW != 0 { return -ERANGE; } if rv == 0 { return -EINVAL; } s = s.add(rv as usize); if *s == b'\n' as c_char { s = s.add(1); } if *s != 0 { return -EINVAL; } *res = tmp; 0
}

unsafe fn _kstrtoul(s: *const c_char, base: u32, res: *mut usize) -> c_int { let mut tmp = 0u64; let rv = _kstrtoull(s, base, &mut tmp); if rv < 0 { return rv; } if tmp != tmp as usize as u64 { return -ERANGE; } *res = tmp as usize; 0 }

pub unsafe fn boot_kstrtoul(s: *const c_char, base: u32, res: *mut usize) -> c_int {
    if core::mem::size_of::<usize>() == core::mem::size_of::<u64>() && core::mem::align_of::<usize>() == core::mem::align_of::<u64>() { _kstrtoull(s, base, res as *mut u64) } else { _kstrtoul(s, base, res) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
