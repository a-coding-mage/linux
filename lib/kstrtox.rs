// SPDX-License-Identifier: GPL-2.0
/*
 * Convert integer string representation to an integer.
 * If an integer doesn't fit into specified type, -E is returned.
 *
 * Integer starts with optional sign.
 * kstrtou*() functions do not accept sign "-".
 *
 * Radix 0 means autodetection: leading "0x" implies radix 16,
 * leading "0" implies radix 8, otherwise radix is 10.
 * Autodetection hints work after optional sign, but not before.
 *
 * If -E is returned, result is not touched.
 */

// Dependencies: linux/ctype.h, linux/errno.h, linux/kstrtox.h,
// linux/math64.h, linux/overflow.h, linux/uaccess.h, lib/kstrtox.h.

use core::cmp::min;
use core::mem::{align_of, size_of};
use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings::{_ctype, copy_from_user, int_pow, EFAULT, EINVAL, ERANGE};
use kernel::ffi::{c_char, c_int, c_long, c_longlong, c_uint, c_ulong, c_ulonglong};

/// KSTRTOX_OVERFLOW from lib/kstrtox.h; lib/kstrtox_helpers.c asserts it.
const KSTRTOX_OVERFLOW: c_uint = 1 << 31;
/// The max_chars of `_parse_integer(s, base, res)` (`_parse_integer0`).
const INT_MAX: usize = c_int::MAX as usize;

/// `_D` and `_X` from linux/ctype.h; lib/kstrtox_helpers.c asserts them.
const CTYPE_D: u8 = 0x04;
const CTYPE_X: u8 = 0x40;

const NEG_EINVAL: c_int = -(EINVAL as c_int);
const NEG_ERANGE: c_int = -(ERANGE as c_int);
const NEG_EFAULT: c_int = -(EFAULT as c_int);

/// isxdigit(): the _D|_X classes of the shared _ctype table.
fn isxdigit(c: c_char) -> bool {
    // SAFETY: the C declaration is an incomplete array; the definition has
    // 256 entries indexed by the unsigned char value.
    unsafe { *addr_of!(_ctype).cast::<u8>().add(usize::from(c)) & (CTYPE_D | CTYPE_X) != 0 }
}

/// isdigit(): '0' <= c && c <= '9'.
fn isdigit(c: c_char) -> bool {
    c.is_ascii_digit()
}

/// _tolower(): `c | 0x20`, as the char-returning inline of linux/ctype.h.
fn _tolower(c: c_char) -> c_char {
    c | 0x20
}

/// Detects the radix when `*base` is 0 and skips a "0x" prefix for base 16.
///
/// # Safety
/// `s` is a NUL-terminated string and `base` points to a writable radix.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _parse_integer_fixup_radix(
    mut s: *const c_char,
    base: *mut c_uint,
) -> *const c_char {
    // SAFETY: each later character is read only after its predecessor was
    // found to be a non-NUL prefix character, as in the C expression order.
    unsafe {
        if *base == 0 {
            if *s == b'0' {
                if _tolower(*s.add(1)) == b'x' && isxdigit(*s.add(2)) {
                    *base = 16;
                } else {
                    *base = 8;
                }
            } else {
                *base = 10;
            }
        }
        if *base == 16 && *s == b'0' && _tolower(*s.add(1)) == b'x' {
            s = s.add(2);
        }
        s
    }
}

/**
 * _parse_integer_limit - Convert integer string representation to an integer
 * @s: Integer string representation
 * @base: Radix
 * @p: Where to store result
 * @max_chars: Maximum amount of characters to convert
 * @init: Initial value of the multiply-accumulate result
 *
 * Convert non-negative integer string representation in explicitly given
 * radix to an integer. If overflow occurs, value at @p is set to ULLONG_MAX.
 *
 * This function is the workhorse of other string conversion functions and it
 * is discouraged to use it explicitly. Consider kstrto*() family instead.
 *
 * Return: Number of characters consumed, maybe ORed with overflow bit
 */
/// # Safety
/// `s` is a NUL-terminated string and `p` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _parse_integer_limit(
    mut s: *const c_char,
    base: c_uint,
    p: *mut c_ulonglong,
    max_chars: usize,
    init: c_ulonglong,
) -> c_uint {
    let mut rv: c_uint = 0;
    let mut overflow: c_uint = 0;
    let mut res: c_ulonglong = init;

    // SAFETY: the scan stops at the first non-digit, at the latest at NUL.
    unsafe {
        while (rv as usize) < max_chars {
            let c = c_uint::from(*s);
            let lc = c_uint::from(_tolower(*s));
            let val: c_uint;

            if c_uint::from(b'0') <= c && c <= c_uint::from(b'9') {
                val = c - c_uint::from(b'0');
            } else if c_uint::from(b'a') <= lc && lc <= c_uint::from(b'f') {
                val = lc - c_uint::from(b'a') + 10;
            } else {
                break;
            }

            if val >= base {
                break;
            }
            /*
             * Check for overflow only if we are within range of
             * it in the max base we support (16)
             */
            if res & (!0u64 << 60) != 0 {
                // check_mul_overflow() || check_add_overflow(), each storing
                // its wrapped result like the C builtins.
                let (product, mul_overflow) = res.overflowing_mul(u64::from(base));
                res = product;
                let overflowed = mul_overflow || {
                    let (sum, add_overflow) = res.overflowing_add(u64::from(val));
                    res = sum;
                    add_overflow
                };
                if overflowed {
                    res = c_ulonglong::MAX;
                    overflow = KSTRTOX_OVERFLOW;
                }
            } else {
                res = res.wrapping_mul(u64::from(base)).wrapping_add(u64::from(val));
            }
            rv = rv.wrapping_add(1);
            s = s.add(1);
        }
        *p = res;
    }
    rv | overflow
}

/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
unsafe fn _kstrtoull(mut s: *const c_char, mut base: c_uint, res: *mut c_ulonglong) -> c_int {
    let mut _res: c_ulonglong = 0;

    // SAFETY: forwarded caller contract; `s` advances within the string.
    unsafe {
        s = _parse_integer_fixup_radix(s, &mut base);
        let rv = _parse_integer_limit(s, base, addr_of_mut!(_res), INT_MAX, 0);
        if rv & KSTRTOX_OVERFLOW != 0 {
            return NEG_ERANGE;
        }
        if rv == 0 {
            return NEG_EINVAL;
        }
        s = s.add(rv as usize);
        if *s == b'\n' {
            s = s.add(1);
        }
        if *s != 0 {
            return NEG_EINVAL;
        }
        *res = _res;
    }
    0
}

/**
 * kstrtoull - convert a string to an unsigned long long
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign, but not a minus sign.
 * @base: The number base to use. The maximum supported base is 16. If base is
 *  given as 0, then the base of the string is automatically detected with the
 *  conventional semantics - If it begins with 0x the number will be parsed as a
 *  hexadecimal (case insensitive), if it otherwise begins with 0, it will be
 *  parsed as an octal number. Otherwise it will be parsed as a decimal.
 * @res: Where to write the result of the conversion on success.
 *
 * Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 * Preferred over simple_strtoull(). Return code must be checked.
 */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kstrtoull(mut s: *const c_char, base: c_uint, res: *mut c_ulonglong) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe {
        if *s == b'+' {
            s = s.add(1);
        }
        _kstrtoull(s, base, res)
    }
}
// EXPORT_SYMBOL(kstrtoull);

/**
 * kstrtoll - convert a string to a long long
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign or a minus sign.
 * @base: The number base to use. The maximum supported base is 16. If base is
 *  given as 0, then the base of the string is automatically detected with the
 *  conventional semantics - If it begins with 0x the number will be parsed as a
 *  hexadecimal (case insensitive), if it otherwise begins with 0, it will be
 *  parsed as an octal number. Otherwise it will be parsed as a decimal.
 * @res: Where to write the result of the conversion on success.
 *
 * Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 * Preferred over simple_strtoll(). Return code must be checked.
 */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kstrtoll(s: *const c_char, base: c_uint, res: *mut c_longlong) -> c_int {
    let mut tmp: c_ulonglong = 0;

    // SAFETY: forwarded caller contract.
    unsafe {
        if *s == b'-' {
            let rv = _kstrtoull(s.add(1), base, &mut tmp);
            if rv < 0 {
                return rv;
            }
            if tmp.wrapping_neg() as c_longlong > 0 {
                return NEG_ERANGE;
            }
            *res = tmp.wrapping_neg() as c_longlong;
        } else {
            let rv = kstrtoull(s, base, &mut tmp);
            if rv < 0 {
                return rv;
            }
            if (tmp as c_longlong) < 0 {
                return NEG_ERANGE;
            }
            *res = tmp as c_longlong;
        }
    }
    0
}
// EXPORT_SYMBOL(kstrtoll);

/* Internal, do not use. */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
pub unsafe extern "C" fn _kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int {
    let mut tmp: c_ulonglong = 0;

    // SAFETY: forwarded caller contract.
    unsafe {
        let rv = kstrtoull(s, base, &mut tmp);
        if rv < 0 {
            return rv;
        }
        if tmp != tmp as c_ulong as c_ulonglong {
            return NEG_ERANGE;
        }
        *res = tmp as c_ulong;
    }
    0
}
// EXPORT_SYMBOL(_kstrtoul);

/* Internal, do not use. */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
pub unsafe extern "C" fn _kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> c_int {
    let mut tmp: c_longlong = 0;

    // SAFETY: forwarded caller contract.
    unsafe {
        let rv = kstrtoll(s, base, &mut tmp);
        if rv < 0 {
            return rv;
        }
        if tmp != tmp as c_long as c_longlong {
            return NEG_ERANGE;
        }
        *res = tmp as c_long;
    }
    0
}
// EXPORT_SYMBOL(_kstrtol);

/// The narrowing kstrtou*()/kstrtos*() conversions, each rejecting values
/// that change when cast to the result type, as in the C bodies.
macro_rules! kstrto_narrow {
    ($(#[$doc:meta])* $name:ident, $wide:ident, $tmp:ty, $ty:ty) => {
        $(#[$doc])*
        /// # Safety
        /// `s` is a NUL-terminated string and `res` points to writable storage.
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn $name(s: *const c_char, base: c_uint, res: *mut $ty) -> c_int {
            let mut tmp: $tmp = 0;

            // SAFETY: forwarded caller contract.
            unsafe {
                let rv = $wide(s, base, &mut tmp);
                if rv < 0 {
                    return rv;
                }
                if tmp != tmp as $ty as $tmp {
                    return NEG_ERANGE;
                }
                *res = tmp as $ty;
            }
            0
        }
    };
}

/*
 * kstrtouint - convert a string to an unsigned int
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign, but not a minus sign.
 * @base: The number base to use. The maximum supported base is 16. If base is
 *  given as 0, then the base of the string is automatically detected with the
 *  conventional semantics - If it begins with 0x the number will be parsed as a
 *  hexadecimal (case insensitive), if it otherwise begins with 0, it will be
 *  parsed as an octal number. Otherwise it will be parsed as a decimal.
 * @res: Where to write the result of the conversion on success.
 *
 * Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 * Preferred over simple_strtoul(). Return code must be checked.
 */
kstrto_narrow!(
    /// Converts a string to an unsigned int.
    kstrtouint, kstrtoull, c_ulonglong, c_uint
);
// EXPORT_SYMBOL(kstrtouint);

/*
 * kstrtoint - convert a string to an int
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign or a minus sign.
 * @base: The number base to use. The maximum supported base is 16. If base is
 *  given as 0, then the base of the string is automatically detected with the
 *  conventional semantics - If it begins with 0x the number will be parsed as a
 *  hexadecimal (case insensitive), if it otherwise begins with 0, it will be
 *  parsed as an octal number. Otherwise it will be parsed as a decimal.
 * @res: Where to write the result of the conversion on success.
 *
 * Returns 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 * Preferred over simple_strtol(). Return code must be checked.
 */
kstrto_narrow!(
    /// Converts a string to an int.
    kstrtoint, kstrtoll, c_longlong, c_int
);
// EXPORT_SYMBOL(kstrtoint);

kstrto_narrow!(
    /// Converts a string to a u16.
    kstrtou16, kstrtoull, c_ulonglong, u16
);
// EXPORT_SYMBOL(kstrtou16);

kstrto_narrow!(
    /// Converts a string to an s16.
    kstrtos16, kstrtoll, c_longlong, i16
);
// EXPORT_SYMBOL(kstrtos16);

kstrto_narrow!(
    /// Converts a string to a u8.
    kstrtou8, kstrtoull, c_ulonglong, u8
);
// EXPORT_SYMBOL(kstrtou8);

kstrto_narrow!(
    /// Converts a string to an s8.
    kstrtos8, kstrtoll, c_longlong, i8
);
// EXPORT_SYMBOL(kstrtos8);

/**
 * kstrtobool - convert common user inputs into boolean values
 * @s: input string
 * @res: result
 *
 * This routine returns 0 iff the first character is one of 'EeYyTt1DdNnFf0',
 * or [oO][NnFf] for "on" and "off". Otherwise it will return -EINVAL.  Value
 * pointed to by res is updated upon finding a match.
 */
/// # Safety
/// `s` is NULL or a NUL-terminated string; `res` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kstrtobool(s: *const c_char, res: *mut bool) -> c_int {
    if s.is_null() {
        return NEG_EINVAL;
    }

    // SAFETY: s[1] is read only after s[0] matched a non-NUL character.
    unsafe {
        match *s {
            b'e' | b'E' | b'y' | b'Y' | b't' | b'T' | b'1' => {
                *res = true;
                return 0;
            }
            b'd' | b'D' | b'n' | b'N' | b'f' | b'F' | b'0' => {
                *res = false;
                return 0;
            }
            b'o' | b'O' => match *s.add(1) {
                b'n' | b'N' => {
                    *res = true;
                    return 0;
                }
                b'f' | b'F' => {
                    *res = false;
                    return 0;
                }
                _ => {}
            },
            _ => {}
        }
    }

    NEG_EINVAL
}
// EXPORT_SYMBOL(kstrtobool);

/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
unsafe fn _kstrtoudec64(mut s: *const c_char, scale: c_uint, res: *mut u64) -> c_int {
    let mut _res: u64 = 0;

    // SAFETY: forwarded caller contract; `s` advances within the string.
    unsafe {
        let rv_int = _parse_integer_limit(s, 10, addr_of_mut!(_res), INT_MAX, 0);
        if rv_int & KSTRTOX_OVERFLOW != 0 {
            return NEG_ERANGE;
        }
        s = s.add(rv_int as usize);

        if *s == b'.' {
            s = s.add(1); /* skip decimal point */
        }

        let init = _res;
        let rv_frac = _parse_integer_limit(s, 10, addr_of_mut!(_res), scale as usize, init);
        if rv_frac & KSTRTOX_OVERFLOW != 0 {
            return NEG_ERANGE;
        }
        s = s.add(rv_frac as usize);

        /*
         * Check input beyond rv_int and rv_frac to cover cases like ".5" with
         * scale 0, which is considered a valid input, being parsed as 0.
         */
        if rv_int == 0 && rv_frac == 0 && !isdigit(*s) {
            return NEG_EINVAL;
        }

        while isdigit(*s) {
            /* truncate digits */
            s = s.add(1);
        }

        if *s == b'\n' {
            s = s.add(1);
        }
        if *s != 0 {
            return NEG_EINVAL;
        }

        if _res != 0
            && (scale.wrapping_sub(rv_frac) > 19 /* log10(2^64) = 19.26 */ || {
                let (product, overflow) = _res.overflowing_mul(int_pow(10, scale.wrapping_sub(rv_frac)));
                _res = product;
                overflow
            })
        {
            return NEG_ERANGE;
        }

        *res = _res;
    }
    0
}

/**
 * kstrtoudec64() - Convert a string to an unsigned 64-bit scaled decimal value.
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign, but not a minus sign.
 * @scale: The number of digits to the right of the decimal point.
 * @res: Where to write the result of the conversion on success.
 *
 * For example, a scale of 3 with input "123.45" results in 123450. Note that
 * trailing zeros in the fractional part input to match the scale are not
 * required. Also, digits beyond the specified scale are ignored.
 *
 * Return: 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kstrtoudec64(mut s: *const c_char, scale: c_uint, res: *mut u64) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe {
        if *s == b'+' {
            s = s.add(1);
        }
        _kstrtoudec64(s, scale, res)
    }
}
// EXPORT_SYMBOL(kstrtoudec64);

/**
 * kstrtodec64() - Convert a string to a signed 64-bit scaled decimal value.
 * @s: The start of the string. The string must be null-terminated, and may also
 *  include a single newline before its terminating null. The first character
 *  may also be a plus sign or a minus sign.
 * @scale: The number of digits to the right of the decimal point.
 * @res: Where to write the result of the conversion on success.
 *
 * For example, a scale of 4 with input "-3.141592" results in -31415. Note
 * that digits beyond the specified scale are ignored. Also, trailing zeros in
 * the fractional part input to match the scale are not required.
 *
 * Return: 0 on success, -ERANGE on overflow and -EINVAL on parsing error.
 */
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn kstrtodec64(s: *const c_char, scale: c_uint, res: *mut i64) -> c_int {
    let mut tmp: u64 = 0;

    // SAFETY: forwarded caller contract.
    unsafe {
        if *s == b'-' {
            let rv = _kstrtoudec64(s.add(1), scale, &mut tmp);
            if rv < 0 {
                return rv;
            }
            if tmp.wrapping_neg() as i64 > 0 {
                return NEG_ERANGE;
            }
            *res = tmp.wrapping_neg() as i64;
        } else {
            let rv = kstrtoudec64(s, scale, &mut tmp);
            if rv < 0 {
                return rv;
            }
            if (tmp as i64) < 0 {
                return NEG_ERANGE;
            }
            *res = tmp as i64;
        }
    }
    0
}
// EXPORT_SYMBOL(kstrtodec64);

/// kstrtoul() from linux/kstrtox.h: kstrtoull() when the types match.
///
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
unsafe fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int {
    /*
     * We want to shortcut function call, but
     * __builtin_types_compatible_p(unsigned long, unsigned long long) = 0.
     */
    // SAFETY: forwarded caller contract; the layouts are identical when the
    // shortcut is taken.
    unsafe {
        if size_of::<c_ulong>() == size_of::<c_ulonglong>()
            && align_of::<c_ulong>() == align_of::<c_ulonglong>()
        {
            kstrtoull(s, base, res.cast::<c_ulonglong>())
        } else {
            _kstrtoul(s, base, res)
        }
    }
}

/// kstrtol() from linux/kstrtox.h: kstrtoll() when the types match.
///
/// # Safety
/// `s` is a NUL-terminated string and `res` points to writable storage.
unsafe fn kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> c_int {
    /*
     * We want to shortcut function call, but
     * __builtin_types_compatible_p(long, long long) = 0.
     */
    // SAFETY: forwarded caller contract; the layouts are identical when the
    // shortcut is taken.
    unsafe {
        if size_of::<c_long>() == size_of::<c_longlong>()
            && align_of::<c_long>() == align_of::<c_longlong>()
        {
            kstrtoll(s, base, res.cast::<c_longlong>())
        } else {
            _kstrtol(s, base, res)
        }
    }
}

/*
 * Since "base" would be a nonsense argument, this open-codes the
 * _from_user helper instead of using the helper macro below.
 */
/// # Safety
/// `s` is a user pointer valid for `count` bytes or faulting; `res` points to
/// writable storage.
#[no_mangle]
pub unsafe extern "C" fn kstrtobool_from_user(s: *const c_char, mut count: usize, res: *mut bool) -> c_int {
    /* Longest string needed to differentiate, newline, terminator */
    let mut buf = [0 as c_char; 4];

    count = min(count, buf.len() - 1);
    // SAFETY: copy_from_user() validates the user range; count < buf.len().
    unsafe {
        if copy_from_user(buf.as_mut_ptr().cast(), s.cast(), count as c_ulong) != 0 {
            return NEG_EFAULT;
        }
        *buf.as_mut_ptr().add(count) = 0;
        kstrtobool(buf.as_ptr(), res)
    }
}
// EXPORT_SYMBOL(kstrtobool_from_user);

/// The `kstrto_from_user(f, g, type)` expansion.
macro_rules! kstrto_from_user {
    ($f:ident, $g:ident, $ty:ty) => {
        /// # Safety
        /// `s` is a user pointer valid for `count` bytes or faulting; `res`
        /// points to writable storage.
        #[no_mangle]
        pub unsafe extern "C" fn $f(s: *const c_char, mut count: usize, base: c_uint, res: *mut $ty) -> c_int {
            /* sign, base 2 representation, newline, terminator */
            let mut buf = [0 as c_char; 1 + size_of::<$ty>() * 8 + 1 + 1];

            count = min(count, buf.len() - 1);
            // SAFETY: copy_from_user() validates the user range;
            // count < buf.len().
            unsafe {
                if copy_from_user(buf.as_mut_ptr().cast(), s.cast(), count as c_ulong) != 0 {
                    return NEG_EFAULT;
                }
                *buf.as_mut_ptr().add(count) = 0;
                $g(buf.as_ptr(), base, res)
            }
        }
        // EXPORT_SYMBOL(f)
    };
}

kstrto_from_user!(kstrtoull_from_user, kstrtoull, c_ulonglong);
kstrto_from_user!(kstrtoll_from_user, kstrtoll, c_longlong);
kstrto_from_user!(kstrtoul_from_user, kstrtoul, c_ulong);
kstrto_from_user!(kstrtol_from_user, kstrtol, c_long);
kstrto_from_user!(kstrtouint_from_user, kstrtouint, c_uint);
kstrto_from_user!(kstrtoint_from_user, kstrtoint, c_int);
kstrto_from_user!(kstrtou16_from_user, kstrtou16, u16);
kstrto_from_user!(kstrtos16_from_user, kstrtos16, i16);
kstrto_from_user!(kstrtou8_from_user, kstrtou8, u8);
kstrto_from_user!(kstrtos8_from_user, kstrtos8, i8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
