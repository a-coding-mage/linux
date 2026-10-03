// SPDX-License-Identifier: GPL-2.0-only
/*
 * Very basic string functions. This unit is also part of compressed/string.c.
 * Keep the original early-boot semantics, including Boolean memcmp, bounded
 * hexadecimal digits, C long widths, and output-only-on-success conversions.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::ptr::null_mut;

const KSTRTOX_OVERFLOW: c_uint = 1 << 31;
const ERANGE: c_int = 34;
const EINVAL: c_int = 22;

// ctype.h supplies these as inline predicates, not externally linked symbols.
#[inline]
fn isdigit(c: c_int) -> bool {
    c >= b'0' as c_int && c <= b'9' as c_int
}

#[inline]
fn isxdigit(c: c_int) -> bool {
    isdigit(c)
        || (c >= b'a' as c_int && c <= b'f' as c_int)
        || (c >= b'A' as c_int && c <= b'F' as c_int)
}

/// Compare bytes, returning only zero or one, as required by boot/string.c.
///
/// # Safety
/// The two pointers cover `len` readable bytes; they may overlap. The boot
/// calling convention has the x86 direction flag clear.
#[no_mangle]
pub(crate) unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, len: usize) -> c_int {
    let different: u8;
    // SAFETY: the caller supplies readable memory and DF=0. TEST establishes
    // ZF even when RCX/ECX is zero and REPE CMPSB therefore does not execute.
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "test rcx, rcx",
            "repe cmpsb",
            "setne {different}",
            different = lateout(reg_byte) different,
            inout("rdi") s1 => _,
            inout("rsi") s2 => _,
            inout("rcx") len => _,
            options(nostack),
        );
        #[cfg(target_arch = "x86")]
        asm!(
            "push esi",
            "mov esi, {source:e}",
            "test ecx, ecx",
            "repe cmpsb",
            "setne {different}",
            "pop esi",
            source = in(reg) s2,
            different = lateout(reg_byte) different,
            inout("edi") s1 => _,
            inout("ecx") len => _,
        );
    }
    different as c_int
}

/// # Safety
/// The same readable-memory and direction-flag contract as `memcmp` applies.
#[no_mangle]
pub(crate) unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, len: usize) -> c_int {
    // SAFETY: this is the original forwarding entry point.
    unsafe { memcmp(s1, s2, len) }
}

/// # Safety
/// Both pointers refer to readable NUL-terminated byte strings.
#[no_mangle]
pub(crate) unsafe extern "C" fn strcmp(str1: *const c_char, str2: *const c_char) -> c_int {
    // SAFETY: unsigned-byte reads and the original short-circuit order are
    // valid for the caller's terminated strings.
    unsafe {
        let mut s1 = str1.cast::<u8>();
        let mut s2 = str2.cast::<u8>();
        while *s1 != 0 || *s2 != 0 {
            let delta = *s1 as c_int - *s2 as c_int;
            if delta != 0 {
                return delta;
            }
            s1 = s1.add(1);
            s2 = s2.add(1);
        }
        0
    }
}

/// # Safety
/// Both pointers are readable through `count` bytes or the first NUL or
/// differing pair, whichever terminates the comparison first.
#[no_mangle]
pub(crate) unsafe extern "C" fn strncmp(
    mut cs: *const c_char,
    mut ct: *const c_char,
    mut count: usize,
) -> c_int {
    // SAFETY: the caller supplies all bytes visited by the C loop.
    unsafe {
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
}

/// # Safety
/// The pointer is readable up to the first NUL or through `maxlen + 1`
/// bytes. The C condition dereferences the byte before testing `maxlen`.
#[no_mangle]
pub(crate) unsafe extern "C" fn strnlen(mut s: *const c_char, mut maxlen: usize) -> usize {
    // SAFETY: retain the original condition order, including the final read.
    unsafe {
        let start = s;
        while *s != 0 && maxlen != 0 {
            s = s.add(1);
            maxlen -= 1;
        }
        s.offset_from(start) as usize
    }
}

#[inline]
unsafe fn simple_guess_base(cp: *const c_char) -> c_uint {
    // SAFETY: the caller supplies a terminated string, so each lookahead is
    // reached only after the preceding bytes have proved nonzero.
    unsafe {
        if *cp == b'0' as c_char {
            if (*cp.add(1) as c_int | 0x20) == b'x' as c_int && isxdigit(*cp.add(2) as c_int) {
                16
            } else {
                8
            }
        } else {
            10
        }
    }
}

/// # Safety
/// `cp` is a readable NUL-terminated string. `endp` is null or writable pointer
/// storage, which may alias input storage as in the original C interface.
#[no_mangle]
pub(crate) unsafe extern "C" fn simple_strtoull(
    mut cp: *const c_char,
    endp: *mut *mut c_char,
    mut base: c_uint,
) -> c_ulonglong {
    // SAFETY: all reads precede the final optional end-pointer store.
    unsafe {
        let mut result: c_ulonglong = 0;
        if base == 0 {
            base = simple_guess_base(cp);
        }
        if base == 16 && *cp == b'0' as c_char && (*cp.add(1) as c_int | 0x20) == b'x' as c_int {
            cp = cp.add(2);
        }
        while isxdigit(*cp as c_int) {
            let value = if isdigit(*cp as c_int) {
                *cp as c_int - b'0' as c_int
            } else {
                (*cp as c_int | 0x20) - b'a' as c_int + 10
            } as c_uint;
            if value >= base {
                break;
            }
            result = result
                .wrapping_mul(base as c_ulonglong)
                .wrapping_add(value as c_ulonglong);
            cp = cp.add(1);
        }
        if !endp.is_null() {
            *endp = cp.cast_mut();
        }
        result
    }
}

/// # Safety
/// The input and optional end-pointer storage satisfy `simple_strtoull`.
#[no_mangle]
pub(crate) unsafe extern "C" fn simple_strtol(
    cp: *const c_char,
    endp: *mut *mut c_char,
    base: c_uint,
) -> c_long {
    // SAFETY: the minus sign, if present, is consumed before parsing. C
    // negates the unsigned long long before conversion to the native long.
    unsafe {
        if *cp == b'-' as c_char {
            simple_strtoull(cp.add(1), endp, base).wrapping_neg() as c_long
        } else {
            simple_strtoull(cp, endp, base) as c_long
        }
    }
}

/// # Safety
/// `s` points to a readable NUL-terminated string.
#[no_mangle]
pub(crate) unsafe extern "C" fn strlen(mut s: *const c_char) -> usize {
    // SAFETY: the string terminator bounds the scan.
    unsafe {
        let start = s;
        while *s != 0 {
            s = s.add(1);
        }
        s.offset_from(start) as usize
    }
}

/// # Safety
/// Both pointers refer to readable NUL-terminated strings.
#[no_mangle]
pub(crate) unsafe extern "C" fn strstr(mut s1: *const c_char, s2: *const c_char) -> *mut c_char {
    // SAFETY: length checks bound every byte comparison.
    unsafe {
        let l2 = strlen(s2);
        if l2 == 0 {
            return s1.cast_mut();
        }
        let mut l1 = strlen(s1);
        while l1 >= l2 {
            l1 -= 1;
            if memcmp(s1.cast(), s2.cast(), l2) == 0 {
                return s1.cast_mut();
            }
            s1 = s1.add(1);
        }
        null_mut()
    }
}

/// # Safety
/// `s` points to a readable NUL-terminated string.
#[no_mangle]
pub(crate) unsafe extern "C" fn strchr(mut s: *const c_char, c: c_int) -> *mut c_char {
    // SAFETY: C converts the searched int to char before comparison. A NUL
    // search returns the terminator itself, including when c truncates to 0.
    unsafe {
        while *s != c as c_char {
            let old = s;
            s = s.add(1);
            if *old == 0 {
                return null_mut();
            }
        }
        s.cast_mut()
    }
}

#[inline]
unsafe fn __div_u64_rem(dividend: u64, divisor: u32, remainder: *mut u32) -> u64 {
    let mut upper = (dividend >> 32) as u32;
    let mut high_quotient: u32 = 0;
    let mut low_quotient = dividend as u32;
    // SAFETY: this is the source's split unsigned DIVL algorithm. A zero
    // divisor faults as it does in C; parsing only reaches it with base > 0.
    // Assembly avoids a compiler runtime u64 division helper on x86-32.
    unsafe {
        if upper >= divisor {
            high_quotient = upper;
            asm!(
                "div {divisor:e}",
                divisor = in(reg) divisor,
                inout("eax") high_quotient,
                inout("edx") 0u32 => upper,
                options(nomem, nostack),
            );
        }
        asm!(
            "div {divisor:e}",
            divisor = in(reg) divisor,
            inout("eax") low_quotient,
            inout("edx") upper,
            options(nomem, nostack),
        );
        *remainder = upper;
    }
    ((high_quotient as u64) << 32) | low_quotient as u64
}

#[inline]
unsafe fn __div_u64(dividend: u64, divisor: u32) -> u64 {
    let mut remainder = 0;
    // SAFETY: the local remainder is writable; the caller supplies divisor.
    unsafe { __div_u64_rem(dividend, divisor, &mut remainder) }
}

#[inline]
fn _tolower(c: c_char) -> c_char {
    (c as c_int | 0x20) as c_char
}

unsafe fn _parse_integer_fixup_radix(mut s: *const c_char, base: *mut c_uint) -> *const c_char {
    // SAFETY: base points to private writable storage and s is terminated.
    unsafe {
        if *base == 0 {
            if *s == b'0' as c_char {
                if _tolower(*s.add(1)) == b'x' as c_char && isxdigit(*s.add(2) as c_int) {
                    *base = 16;
                } else {
                    *base = 8;
                }
            } else {
                *base = 10;
            }
        }
        if *base == 16 && *s == b'0' as c_char && _tolower(*s.add(1)) == b'x' as c_char {
            s = s.add(2);
        }
        s
    }
}

unsafe fn _parse_integer(mut s: *const c_char, base: c_uint, p: *mut c_ulonglong) -> c_uint {
    // SAFETY: the caller supplies a readable string and private writable p.
    unsafe {
        let mut res: c_ulonglong = 0;
        let mut rv: c_uint = 0;
        loop {
            // Match promotion from signed char to unsigned int, not u8.
            let c = *s as c_uint;
            let lc = c | 0x20;
            let val = if c >= b'0' as c_uint && c <= b'9' as c_uint {
                c - b'0' as c_uint
            } else if lc >= b'a' as c_uint && lc <= b'f' as c_uint {
                lc - b'a' as c_uint + 10
            } else {
                break;
            };
            if val >= base {
                break;
            }
            // Preserve the original high-nibble gate and sticky overflow bit.
            if res & (!0u64 << 60) != 0 && res > __div_u64(u64::MAX - val as u64, base) {
                rv |= KSTRTOX_OVERFLOW;
            }
            res = res
                .wrapping_mul(base as c_ulonglong)
                .wrapping_add(val as c_ulonglong);
            rv = rv.wrapping_add(1);
            s = s.add(1);
        }
        *p = res;
        rv
    }
}

unsafe fn _kstrtoull(mut s: *const c_char, mut base: c_uint, res: *mut c_ulonglong) -> c_int {
    // SAFETY: parse into private storage; never write caller output on error.
    unsafe {
        if *s == b'+' as c_char {
            s = s.add(1);
        }
        s = _parse_integer_fixup_radix(s, &mut base);
        let mut parsed = 0;
        let rv = _parse_integer(s, base, &mut parsed);
        if rv & KSTRTOX_OVERFLOW != 0 {
            return -ERANGE;
        }
        if rv == 0 {
            return -EINVAL;
        }
        s = s.add(rv as usize);
        if *s == b'\n' as c_char {
            s = s.add(1);
        }
        if *s != 0 {
            return -EINVAL;
        }
        *res = parsed;
        0
    }
}

unsafe fn _kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int {
    // SAFETY: the intermediate is private and output is written on success.
    unsafe {
        let mut tmp = 0;
        let rv = _kstrtoull(s, base, &mut tmp);
        if rv < 0 {
            return rv;
        }
        if tmp != tmp as c_ulong as c_ulonglong {
            return -ERANGE;
        }
        *res = tmp as c_ulong;
        0
    }
}

/// # Safety
/// `s` is readable and NUL-terminated; `res` is writable native unsigned-long
/// storage. Failed conversions leave it untouched, even when it aliases input.
#[no_mangle]
pub(crate) unsafe extern "C" fn boot_kstrtoul(
    s: *const c_char,
    base: c_uint,
    res: *mut c_ulong,
) -> c_int {
    // SAFETY: the source's size/alignment test controls the direct u64 store.
    unsafe {
        if core::mem::size_of::<c_ulong>() == core::mem::size_of::<c_ulonglong>()
            && core::mem::align_of::<c_ulong>() == core::mem::align_of::<c_ulonglong>()
        {
            _kstrtoull(s, base, res.cast())
        } else {
            _kstrtoul(s, base, res)
        }
    }
}

// SOURCE-COMMIT: 8e8505218ff400546323d71109207c781769e54e
