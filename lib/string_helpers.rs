// SPDX-License-Identifier: GPL-2.0-only
/*
 * Helpers for formatting and printing strings
 *
 * Copyright 31 August 2008 James Bottomley
 * Copyright (C) 2013, Intel Corporation
 */

// Dependencies: linux/bug.h, linux/math64.h, linux/ctype.h, linux/device.h,
// linux/errno.h, linux/fs.h, linux/hex.h, linux/limits.h, linux/mm.h,
// linux/slab.h, linux/string.h, linux/string_helpers.h.
//
// The CONFIG_FORTIFY_SOURCE reporters keep the original C provider: lib/Kconfig
// only offers RUST_STRING_HELPERS without FORTIFY_SOURCE.

use core::ffi::CStr;
use core::mem::size_of;
use core::ptr::{addr_of, copy_nonoverlapping, null_mut, write_bytes};
use kernel::bindings::{
    _ctype, device, devres_add, devres_free, file, file_path, get_cmdline,
    get_options, gfp_t, kasprintf, kfree, kstrdup, memdup_user_nul, snprintf, strchr, strcmp, strlen,
    task_struct, EINVAL, ENOENT, ENOMEM, GFP_KERNEL, PAGE_SIZE, PATH_MAX,
};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// `struct device` embeds a zero-sized lockdep map; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    fn hex_to_bin(ch: u8) -> c_int;
    /// `const char hex_asc[]` ("0123456789abcdef"), owned by the hexdump unit.
    static hex_asc: [c_char; 0];

    fn lupos_string_helpers_kmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn lupos_string_helpers_kcalloc(n: usize, size: usize, gfp: gfp_t) -> *mut c_void;
    fn lupos_string_helpers_kzalloc_ints(count: c_int) -> *mut c_int;
    fn lupos_string_helpers_devres_alloc(
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        size: usize,
    ) -> *mut c_void;
    fn lupos_string_helpers_warn_quotable();
}

/// `enum string_size_units` values; lib/string_helpers_helpers.c asserts them.
const STRING_UNITS_2: c_uint = 1;
const STRING_UNITS_MASK: c_uint = 1 << 0;
const STRING_UNITS_NO_SPACE: c_uint = 1 << 30;
const STRING_UNITS_NO_BYTES: c_uint = 1 << 31;

const UNESCAPE_SPACE: c_uint = 1 << 0;
const UNESCAPE_OCTAL: c_uint = 1 << 1;
const UNESCAPE_HEX: c_uint = 1 << 2;
const UNESCAPE_SPECIAL: c_uint = 1 << 3;

const ESCAPE_SPACE: c_uint = 1 << 0;
const ESCAPE_SPECIAL: c_uint = 1 << 1;
const ESCAPE_NULL: c_uint = 1 << 2;
const ESCAPE_OCTAL: c_uint = 1 << 3;
const ESCAPE_NP: c_uint = 1 << 4;
const ESCAPE_HEX: c_uint = 1 << 5;
const ESCAPE_NA: c_uint = 1 << 6;
const ESCAPE_NAP: c_uint = 1 << 7;
const ESCAPE_APPEND: c_uint = 1 << 8;

/// ctype classes from linux/ctype.h; lib/string_helpers_helpers.c asserts them.
const CTYPE_U: u8 = 0x01;
const CTYPE_L: u8 = 0x02;
const CTYPE_D: u8 = 0x04;
const CTYPE_P: u8 = 0x10;
const CTYPE_S: u8 = 0x20;
const CTYPE_SP: u8 = 0x80;

/// MAX_ERRNO from linux/err.h, for IS_ERR().
const MAX_ERRNO: c_ulong = 4095;

/// __ismask(c): the shared _ctype classification of the unsigned char value.
fn ismask(c: u8) -> u8 {
    // SAFETY: the C declaration is an incomplete array; the definition has
    // 256 entries indexed by the unsigned char value.
    unsafe { *addr_of!(_ctype).cast::<u8>().add(usize::from(c)) }
}

fn isspace(c: u8) -> bool {
    ismask(c) & CTYPE_S != 0
}

fn isprint(c: u8) -> bool {
    ismask(c) & (CTYPE_P | CTYPE_U | CTYPE_L | CTYPE_D | CTYPE_SP) != 0
}

fn isascii(c: u8) -> bool {
    c <= 0x7f
}

fn isodigit(c: u8) -> bool {
    (b'0'..=b'7').contains(&c)
}

/// hex_asc[x & 0x0f].
fn hex_asc_at(nibble: u8) -> c_char {
    // SAFETY: hex_asc holds 16 digits; the index is a nibble.
    unsafe { *addr_of!(hex_asc).cast::<c_char>().add(usize::from(nibble & 0x0f)) }
}

/// IS_ERR() on a returned pointer.
fn is_err<T>(ptr: *const T) -> bool {
    ptr as c_ulong >= MAX_ERRNO.wrapping_neg()
}

/// ERR_PTR(error).
fn err_ptr<T>(error: c_long) -> *mut T {
    error as *mut T
}

/// The do_div() macro: divides `n` in place and returns the remainder.
#[inline(always)]
fn do_div(n: &mut u64, base: c_uint) -> u32 {
    let remainder = (*n % u64::from(base)) as u32;
    *n /= u64::from(base);
    remainder
}

static UNITS_10: [&CStr; 9] = [c"", c"k", c"M", c"G", c"T", c"P", c"E", c"Z", c"Y"];
static UNITS_2: [&CStr; 9] = [c"", c"Ki", c"Mi", c"Gi", c"Ti", c"Pi", c"Ei", c"Zi", c"Yi"];
static UNITS_STR: [&[&CStr; 9]; 2] = [&UNITS_10, &UNITS_2];
static ROUNDING: [c_uint; 3] = [500, 50, 5];

/**
 * string_get_size - get the size in the specified units
 * @size:	The size to be converted in blocks
 * @blk_size:	Size of the block (use 1 for size in bytes)
 * @units:	Units to use (powers of 1000 or 1024), whether to include space separator
 * @buf:	buffer to format to
 * @len:	length of buffer
 *
 * This function returns a string formatted to 3 significant figures
 * giving the size in the required units.  @buf should have room for
 * at least 9 bytes and will always be zero terminated.
 *
 * Return value: number of characters of output that would have been written
 * (which may be greater than len, if output was truncated).
 */
/// # Safety
/// `buf` is writable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn string_get_size(
    mut size: u64,
    mut blk_size: u64,
    units: c_uint,
    buf: *mut c_char,
    len: c_int,
) -> c_int {
    let units_base = (units & STRING_UNITS_MASK) as usize;
    // divisor[] = { [STRING_UNITS_10] = 1000, [STRING_UNITS_2] = 1024 }.
    let divisor: c_uint = if units_base == STRING_UNITS_2 as usize { 1024 } else { 1000 };
    let mut i: usize = 0;
    let mut remainder: u32 = 0;
    let mut tmp = [0 as c_char; 12];

    tmp[0] = 0;

    if blk_size == 0 {
        size = 0;
    }
    'out: {
        if size == 0 {
            break 'out;
        }

        /* This is Napier's algorithm.  Reduce the original block size to
         *
         * coefficient * divisor[units_base]^i
         *
         * we do the reduction so both coefficients are just under 32 bits so
         * that multiplying them together won't overflow 64 bits and we keep
         * as much precision as possible in the numbers.
         *
         * Note: it's safe to throw away the remainders here because all the
         * precision is in the coefficients.
         */
        while blk_size >> 32 != 0 {
            do_div(&mut blk_size, divisor);
            i = i.wrapping_add(1);
        }

        while size >> 32 != 0 {
            do_div(&mut size, divisor);
            i = i.wrapping_add(1);
        }

        /* now perform the actual multiplication keeping i as the sum of the
         * two logarithms */
        size = size.wrapping_mul(blk_size);

        /* and logarithmically reduce it until it's just under the divisor */
        while size >= u64::from(divisor) {
            remainder = do_div(&mut size, divisor);
            i = i.wrapping_add(1);
        }

        /* work out in j how many digits of precision we need from the
         * remainder */
        let mut sf_cap = size as u32;
        let mut j: usize = 0;
        while sf_cap.wrapping_mul(10) < 1000 {
            sf_cap = sf_cap.wrapping_mul(10);
            j = j.wrapping_add(1);
        }

        if units_base == STRING_UNITS_2 as usize {
            /* express the remainder as a decimal.  It's currently the
             * numerator of a fraction whose denominator is
             * divisor[units_base], which is 1 << 10 for STRING_UNITS_2 */
            remainder = remainder.wrapping_mul(1000);
            remainder >>= 10;
        }

        /* add a 5 to the digit below what will be printed to ensure
         * an arithmetical round up and carry it through to size */
        // SAFETY: j <= 2 because sf_cap = size is in [1, divisor), as in C.
        remainder = remainder.wrapping_add(unsafe { *ROUNDING.as_ptr().add(j) });
        if remainder >= 1000 {
            remainder -= 1000;
            size = size.wrapping_add(1);
        }

        if j != 0 {
            // SAFETY: tmp is 12 bytes, which bounds the formatted output.
            unsafe { snprintf(tmp.as_mut_ptr(), tmp.len(), c".%03u".as_ptr().cast(), remainder) };
            // SAFETY: j + 1 <= 3 lies inside tmp.
            unsafe { *tmp.as_mut_ptr().add(j.wrapping_add(1)) = 0 };
        }
    }

    // out:
    let unit: *const c_char = if i >= UNITS_2.len() {
        c"UNK".as_ptr().cast()
    } else {
        UNITS_STR[units_base][i].as_ptr().cast()
    };

    let space: &CStr = if units & STRING_UNITS_NO_SPACE != 0 { c"" } else { c" " };
    let bytes: &CStr = if units & STRING_UNITS_NO_BYTES != 0 { c"" } else { c"B" };
    // SAFETY: snprintf() is bounded by the caller's buffer length.
    unsafe {
        snprintf(
            buf,
            len as usize,
            c"%u%s%s%s%s".as_ptr().cast(),
            size as u32,
            tmp.as_ptr(),
            space.as_ptr(),
            unit,
            bytes.as_ptr(),
        )
    }
}
// EXPORT_SYMBOL(string_get_size);

/// Splits `buf` into a newly allocated, length-prefixed integer array.
///
/// # Safety
/// `buf` is a NUL-terminated string and `array` points to writable storage.
#[no_mangle]
pub unsafe extern "C" fn parse_int_array(buf: *const c_char, _count: usize, array: *mut *mut c_int) -> c_int {
    let mut nints: c_int = 0;

    // SAFETY: get_options() writes at most nints + 1 entries to ints.
    unsafe {
        get_options(buf, 0, &mut nints);
        if nints == 0 {
            return -(ENOENT as c_int);
        }

        let ints = lupos_string_helpers_kzalloc_ints(nints.wrapping_add(1));
        if ints.is_null() {
            return -(ENOMEM as c_int);
        }

        get_options(buf, nints.wrapping_add(1), ints);
        *array = ints;
    }

    0
}
// EXPORT_SYMBOL(parse_int_array);

/**
 * parse_int_array_user - Split string into a sequence of integers
 * @from:	The user space buffer to read from
 * @count:	The maximum number of bytes to read
 * @array:	Returned pointer to sequence of integers
 *
 * On success @array is allocated and initialized with a sequence of
 * integers extracted from the @from plus an additional element that
 * begins the sequence and specifies the integers count.
 *
 * Caller takes responsibility for freeing @array when it is no longer
 * needed.
 */
/// # Safety
/// `from` is a user pointer checked by memdup_user_nul(); `array` points to
/// writable storage.
#[no_mangle]
pub unsafe extern "C" fn parse_int_array_user(from: *const c_char, count: usize, array: *mut *mut c_int) -> c_int {
    // SAFETY: the user copy is validated by memdup_user_nul() and freed here.
    unsafe {
        let buf = memdup_user_nul(from.cast::<c_void>(), count).cast::<c_char>();
        if is_err(buf) {
            return buf as c_long as c_int;
        }

        let ret = parse_int_array(buf, count, array);
        kfree(buf.cast::<c_void>());
        ret
    }
}
// EXPORT_SYMBOL(parse_int_array_user);

/// # Safety
/// `*src` and `*dst` are valid cursors into the unescape buffers.
unsafe fn unescape_space(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let p = *dst;
    let q = *src;

    // SAFETY: q points at the escaped character and p at the output slot.
    unsafe {
        *p = match *q {
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'v' => 0x0b,
            b'f' => 0x0c,
            _ => return false,
        };
    }
    *dst = dst.wrapping_add(1);
    *src = src.wrapping_add(1);
    true
}

/// # Safety
/// `*src` and `*dst` are valid cursors into the unescape buffers.
unsafe fn unescape_octal(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let p = *dst;
    let mut q = *src;

    // SAFETY: q only advances over octal digits of the NUL-terminated source.
    unsafe {
        if !isodigit(*q) {
            return false;
        }

        let mut num: u8 = *q & 7;
        q = q.add(1);
        while num < 32 && isodigit(*q) && q.offset_from(*src) < 3 {
            num <<= 3;
            num = num.wrapping_add(*q & 7);
            q = q.add(1);
        }
        *p = num;
    }
    *dst = dst.wrapping_add(1);
    *src = q;
    true
}

/// # Safety
/// `*src` and `*dst` are valid cursors into the unescape buffers.
unsafe fn unescape_hex(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let p = *dst;
    let mut q = *src;

    // SAFETY: q only advances over 'x' and hex digits of the NUL-terminated
    // source.
    unsafe {
        let x = *q;
        q = q.add(1);
        if x != b'x' {
            return false;
        }

        let mut digit = hex_to_bin(*q);
        q = q.add(1);
        let mut num = digit as u8;
        if digit < 0 {
            return false;
        }

        digit = hex_to_bin(*q);
        if digit >= 0 {
            q = q.add(1);
            num = ((c_int::from(num) << 4) | digit) as u8;
        }
        *p = num;
    }
    *dst = dst.wrapping_add(1);
    *src = q;
    true
}

/// # Safety
/// `*src` and `*dst` are valid cursors into the unescape buffers.
unsafe fn unescape_special(src: &mut *mut c_char, dst: &mut *mut c_char) -> bool {
    let p = *dst;
    let q = *src;

    // SAFETY: q points at the escaped character and p at the output slot.
    unsafe {
        *p = match *q {
            b'"' => b'"',
            b'\\' => b'\\',
            b'a' => 0x07,
            b'e' => 0x1b,
            _ => return false,
        };
    }
    *dst = dst.wrapping_add(1);
    *src = src.wrapping_add(1);
    true
}

/**
 * string_unescape - unquote characters in the given string
 * @src:	source buffer (escaped)
 * @dst:	destination buffer (unescaped)
 * @size:	size of the destination buffer (0 to unlimit)
 * @flags:	combination of the flags.
 *
 * Description:
 * The function unquotes characters in the given string.
 *
 * Because the size of the output will be the same as or less than the size of
 * the input, the transformation may be performed in place.
 *
 * Caller must provide valid source and destination pointers. Be aware that
 * destination buffer will always be NULL-terminated. Source string must be
 * NULL-terminated as well.  The supported flags are::
 *
 *	UNESCAPE_SPACE:
 *		'\f' - form feed
 *		'\n' - new line
 *		'\r' - carriage return
 *		'\t' - horizontal tab
 *		'\v' - vertical tab
 *	UNESCAPE_OCTAL:
 *		'\NNN' - byte with octal value NNN (1 to 3 digits)
 *	UNESCAPE_HEX:
 *		'\xHH' - byte with hexadecimal value HH (1 to 2 digits)
 *	UNESCAPE_SPECIAL:
 *		'\"' - double quote
 *		'\\' - backslash
 *		'\a' - alert (BEL)
 *		'\e' - escape
 *	UNESCAPE_ANY:
 *		all previous together
 *
 * Return:
 * The amount of the characters processed to the destination buffer excluding
 * trailing '\0' is returned.
 */
/// # Safety
/// `src` is NUL-terminated and `dst` writable for `size` bytes (or as many
/// as `src` produces when `size` is 0).
#[no_mangle]
pub unsafe extern "C" fn string_unescape(mut src: *mut c_char, dst: *mut c_char, mut size: usize, flags: c_uint) -> c_int {
    let mut out = dst;

    if size == 0 {
        size = usize::MAX;
    }

    // SAFETY: the loop consumes the NUL-terminated source and writes at most
    // size - 1 bytes plus the terminator.
    unsafe {
        while *src != 0 && {
            size = size.wrapping_sub(1);
            size != 0
        } {
            if *src == b'\\' && *src.add(1) != 0 && size > 1 {
                src = src.add(1);
                size -= 1;

                if flags & UNESCAPE_SPACE != 0 && unescape_space(&mut src, &mut out) {
                    continue;
                }

                if flags & UNESCAPE_OCTAL != 0 && unescape_octal(&mut src, &mut out) {
                    continue;
                }

                if flags & UNESCAPE_HEX != 0 && unescape_hex(&mut src, &mut out) {
                    continue;
                }

                if flags & UNESCAPE_SPECIAL != 0 && unescape_special(&mut src, &mut out) {
                    continue;
                }

                *out = b'\\';
                out = out.add(1);
            }
            *out = *src;
            out = out.add(1);
            src = src.add(1);
        }
        *out = 0;

        out.offset_from(dst) as c_int
    }
}
// EXPORT_SYMBOL(string_unescape);

/// Writes `c` at `*dst` when it lies before `end`, then advances the cursor.
fn put(dst: &mut *mut c_char, end: *mut c_char, c: u8) {
    if *dst < end {
        // SAFETY: the cursor lies inside the caller's [dst, end) buffer.
        unsafe { **dst = c };
    }
    *dst = dst.wrapping_add(1);
}

fn escape_passthrough(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    put(dst, end, c);
    true
}

fn escape_space(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    let to = match c {
        b'\n' => b'n',
        b'\r' => b'r',
        b'\t' => b't',
        0x0b => b'v',
        0x0c => b'f',
        _ => return false,
    };

    put(dst, end, b'\\');
    put(dst, end, to);
    true
}

fn escape_special(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    let to = match c {
        b'\\' => b'\\',
        0x07 => b'a',
        0x1b => b'e',
        b'"' => b'"',
        _ => return false,
    };

    put(dst, end, b'\\');
    put(dst, end, to);
    true
}

fn escape_null(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    if c != 0 {
        return false;
    }

    put(dst, end, b'\\');
    put(dst, end, b'0');
    true
}

fn escape_octal(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    put(dst, end, b'\\');
    put(dst, end, ((c >> 6) & 0x07) + b'0');
    put(dst, end, ((c >> 3) & 0x07) + b'0');
    put(dst, end, (c & 0x07) + b'0');
    true
}

fn escape_hex(c: u8, dst: &mut *mut c_char, end: *mut c_char) -> bool {
    put(dst, end, b'\\');
    put(dst, end, b'x');
    put(dst, end, hex_asc_at((c & 0xf0) >> 4));
    put(dst, end, hex_asc_at(c & 0x0f));
    true
}

/**
 * string_escape_mem - quote characters in the given memory buffer
 * @src:	source buffer (unescaped)
 * @isz:	source buffer size
 * @dst:	destination buffer (escaped)
 * @osz:	destination buffer size
 * @flags:	combination of the flags
 * @only:	NULL-terminated string containing characters used to limit
 *		the selected escape class. If characters are included in @only
 *		that would not normally be escaped by the classes selected
 *		in @flags, they will be copied to @dst unescaped.
 *
 * Description:
 * The process of escaping byte buffer includes several parts. They are applied
 * in the following sequence.
 *
 *	1. The character is not matched to the one from @only string and thus
 *	   must go as-is to the output.
 *	2. The character is matched to the printable and ASCII classes, if asked,
 *	   and in case of match it passes through to the output.
 *	3. The character is matched to the printable or ASCII class, if asked,
 *	   and in case of match it passes through to the output.
 *	4. The character is checked if it falls into the class given by @flags.
 *	   %ESCAPE_OCTAL and %ESCAPE_HEX are going last since they cover any
 *	   character. Note that they actually can't go together, otherwise
 *	   %ESCAPE_HEX will be ignored.
 *
 * Caller must provide valid source and destination pointers. Be aware that
 * destination buffer will not be NULL-terminated, thus caller have to append
 * it if needs. The supported flags are::
 *
 *	%ESCAPE_SPACE: (special white space, not space itself)
 *		'\f' - form feed
 *		'\n' - new line
 *		'\r' - carriage return
 *		'\t' - horizontal tab
 *		'\v' - vertical tab
 *	%ESCAPE_SPECIAL:
 *		'\"' - double quote
 *		'\\' - backslash
 *		'\a' - alert (BEL)
 *		'\e' - escape
 *	%ESCAPE_NULL:
 *		'\0' - null
 *	%ESCAPE_OCTAL:
 *		'\NNN' - byte with octal value NNN (3 digits)
 *	%ESCAPE_ANY:
 *		all previous together
 *	%ESCAPE_NP:
 *		escape only non-printable characters, checked by isprint()
 *	%ESCAPE_ANY_NP:
 *		all previous together
 *	%ESCAPE_HEX:
 *		'\xHH' - byte with hexadecimal value HH (2 digits)
 *	%ESCAPE_NA:
 *		escape only non-ascii characters, checked by isascii()
 *	%ESCAPE_NAP:
 *		escape only non-printable or non-ascii characters
 *	%ESCAPE_APPEND:
 *		append characters from @only to be escaped by the given classes
 *
 * %ESCAPE_APPEND would help to pass additional characters to the escaped, when
 * one of %ESCAPE_NP, %ESCAPE_NA, or %ESCAPE_NAP is provided.
 *
 * One notable caveat, the %ESCAPE_NAP, %ESCAPE_NP and %ESCAPE_NA have the
 * higher priority than the rest of the flags (%ESCAPE_NAP is the highest).
 * It doesn't make much sense to use either of them without %ESCAPE_OCTAL
 * or %ESCAPE_HEX, because they cover most of the other character classes.
 * %ESCAPE_NAP can utilize %ESCAPE_SPACE or %ESCAPE_SPECIAL in addition to
 * the above.
 *
 * Return:
 * The total size of the escaped output that would be generated for
 * the given input and flags. To check whether the output was
 * truncated, compare the return value to osz. There is room left in
 * dst for a '\0' terminator if and only if ret < osz.
 */
/// # Safety
/// `src` is readable for `isz` bytes, `dst` writable for `osz` bytes (or
/// NULL with `osz` 0) and `only` NULL or NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn string_escape_mem(
    mut src: *const c_char,
    mut isz: usize,
    dst: *mut c_char,
    osz: usize,
    flags: c_uint,
    only: *const c_char,
) -> c_int {
    let mut p = dst;
    let end = p.wrapping_add(osz);
    // SAFETY: only is NULL or NUL-terminated.
    let is_dict = !only.is_null() && unsafe { *only } != 0;
    let is_append = flags & ESCAPE_APPEND != 0;

    while isz != 0 {
        isz -= 1;
        // SAFETY: src is readable for the remaining isz bytes.
        let c = unsafe { *src };
        src = src.wrapping_add(1);
        // SAFETY: strchr() scans the NUL-terminated dictionary.
        let in_dict = is_dict && !unsafe { strchr(only, c_int::from(c)) }.is_null();

        /*
         * Apply rules in the following sequence:
         *	- the @only string is supplied and does not contain a
         *	  character under question
         *	- the character is printable and ASCII, when @flags has
         *	  %ESCAPE_NAP bit set
         *	- the character is printable, when @flags has
         *	  %ESCAPE_NP bit set
         *	- the character is ASCII, when @flags has
         *	  %ESCAPE_NA bit set
         *	- the character doesn't fall into a class of symbols
         *	  defined by given @flags
         * In these cases we just pass through a character to the
         * output buffer.
         *
         * When %ESCAPE_APPEND is passed, the characters from @only
         * have been excluded from the %ESCAPE_NAP, %ESCAPE_NP, and
         * %ESCAPE_NA cases.
         */
        if !(is_append || in_dict) && is_dict && escape_passthrough(c, &mut p, end) {
            continue;
        }

        if !(is_append && in_dict)
            && isascii(c)
            && isprint(c)
            && flags & ESCAPE_NAP != 0
            && escape_passthrough(c, &mut p, end)
        {
            continue;
        }

        if !(is_append && in_dict) && isprint(c) && flags & ESCAPE_NP != 0 && escape_passthrough(c, &mut p, end) {
            continue;
        }

        if !(is_append && in_dict) && isascii(c) && flags & ESCAPE_NA != 0 && escape_passthrough(c, &mut p, end) {
            continue;
        }

        if flags & ESCAPE_SPACE != 0 && escape_space(c, &mut p, end) {
            continue;
        }

        if flags & ESCAPE_SPECIAL != 0 && escape_special(c, &mut p, end) {
            continue;
        }

        if flags & ESCAPE_NULL != 0 && escape_null(c, &mut p, end) {
            continue;
        }

        /* ESCAPE_OCTAL and ESCAPE_HEX always go last */
        if flags & ESCAPE_OCTAL != 0 && escape_octal(c, &mut p, end) {
            continue;
        }

        if flags & ESCAPE_HEX != 0 && escape_hex(c, &mut p, end) {
            continue;
        }

        escape_passthrough(c, &mut p, end);
    }

    (p as usize).wrapping_sub(dst as usize) as c_int
}
// EXPORT_SYMBOL(string_escape_mem);

/*
 * Return an allocated string that has been escaped of special characters
 * and double quotes, making it safe to log in quotes.
 */
/// # Safety
/// `src` is NULL or a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn kstrdup_quotable(src: *const c_char, gfp: gfp_t) -> *mut c_char {
    let flags: c_uint = ESCAPE_HEX;
    let esc: &CStr = c"\x0c\n\r\t\x0b\x07\x1b\\\"";

    if src.is_null() {
        return null_mut();
    }

    // SAFETY: src is NUL-terminated; dst is allocated for dlen + 1 bytes.
    unsafe {
        let slen = strlen(src);

        let dlen = string_escape_mem(src, slen, null_mut(), 0, flags, esc.as_ptr().cast()) as usize;
        let dst = lupos_string_helpers_kmalloc(dlen.wrapping_add(1), gfp).cast::<c_char>();
        if dst.is_null() {
            return null_mut();
        }

        if string_escape_mem(src, slen, dst, dlen, flags, esc.as_ptr().cast()) as usize != dlen {
            lupos_string_helpers_warn_quotable();
        }
        *dst.add(dlen) = 0;

        dst
    }
}
// EXPORT_SYMBOL_GPL(kstrdup_quotable);

/*
 * Returns allocated NULL-terminated string containing process
 * command line, with inter-argument NULLs replaced with spaces,
 * and other special characters escaped.
 */
/// # Safety
/// `task` points to a valid task.
#[no_mangle]
pub unsafe extern "C" fn kstrdup_quotable_cmdline(task: *mut task_struct, gfp: gfp_t) -> *mut c_char {
    // SAFETY: get_cmdline() fills at most PAGE_SIZE - 1 bytes of buffer.
    unsafe {
        let buffer = lupos_string_helpers_kmalloc(PAGE_SIZE, GFP_KERNEL).cast::<c_char>();
        if buffer.is_null() {
            return null_mut();
        }

        let mut res = get_cmdline(task, buffer, (PAGE_SIZE - 1) as c_int);
        *buffer.offset(res as isize) = 0;

        /* Collapse trailing NULLs, leave res pointing to last non-NULL. */
        loop {
            res = res.wrapping_sub(1);
            if !(res >= 0 && *buffer.offset(res as isize) == 0) {
                break;
            }
        }

        /* Replace inter-argument NULLs. */
        let mut i: c_int = 0;
        while i <= res {
            if *buffer.offset(i as isize) == 0 {
                *buffer.offset(i as isize) = b' ';
            }
            i = i.wrapping_add(1);
        }

        /* Make sure result is printable. */
        let quoted = kstrdup_quotable(buffer, gfp);
        kfree(buffer.cast::<c_void>());
        quoted
    }
}
// EXPORT_SYMBOL_GPL(kstrdup_quotable_cmdline);

/*
 * Returns allocated NULL-terminated string containing pathname,
 * with special characters escaped, able to be safely logged. If
 * there is an error, the leading character will be "<".
 */
/// # Safety
/// `file` is NULL or a valid file.
#[no_mangle]
pub unsafe extern "C" fn kstrdup_quotable_file(file: *mut file, gfp: gfp_t) -> *mut c_char {
    // SAFETY: file_path() writes inside the PATH_MAX + 11 byte buffer.
    unsafe {
        if file.is_null() {
            return kstrdup(c"<unknown>".as_ptr().cast(), gfp);
        }

        /* We add 11 spaces for ' (deleted)' to be appended */
        let temp = lupos_string_helpers_kmalloc(PATH_MAX as usize + 11, GFP_KERNEL).cast::<c_char>();
        if temp.is_null() {
            return kstrdup(c"<no_memory>".as_ptr().cast(), gfp);
        }

        let mut pathname = file_path(file, temp, PATH_MAX as c_int + 11);
        if is_err(pathname) {
            pathname = kstrdup(c"<too_long>".as_ptr().cast(), gfp);
        } else {
            pathname = kstrdup_quotable(pathname, gfp);
        }

        kfree(temp.cast::<c_void>());
        pathname
    }
}
// EXPORT_SYMBOL_GPL(kstrdup_quotable_file);

/*
 * Returns duplicate string in which the @old characters are replaced by @new.
 */
/// # Safety
/// `src` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn kstrdup_and_replace(src: *const c_char, old: c_char, new: c_char, gfp: gfp_t) -> *mut c_char {
    // SAFETY: the duplicate is a NUL-terminated string owned by the caller.
    unsafe {
        let dst = kstrdup(src, gfp);
        if dst.is_null() {
            return null_mut();
        }

        strreplace(dst, old, new)
    }
}
// EXPORT_SYMBOL_GPL(kstrdup_and_replace);

/**
 * kasprintf_strarray - allocate and fill array of sequential strings
 * @gfp: flags for the slab allocator
 * @prefix: prefix to be used
 * @n: number of strings to be allocated and filled
 *
 * Allocates and fills @n strings using pattern "%s-%zu", where prefix
 * is provided by caller. The caller is responsible to free them with
 * kfree_strarray() after use.
 *
 * Returns array of strings or NULL when memory can't be allocated.
 */
/// # Safety
/// `prefix` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn kasprintf_strarray(gfp: gfp_t, prefix: *const c_char, n: usize) -> *mut *mut c_char {
    // SAFETY: names has n zeroed slots; each is filled before the next.
    unsafe {
        let names = lupos_string_helpers_kcalloc(n, size_of::<*mut c_char>(), gfp).cast::<*mut c_char>();
        if names.is_null() {
            return null_mut();
        }

        for i in 0..n {
            let name = kasprintf(gfp, c"%s-%zu".as_ptr().cast(), prefix, i);
            *names.add(i) = name;
            if name.is_null() {
                kfree_strarray(names, i);
                return null_mut();
            }
        }

        names
    }
}
// EXPORT_SYMBOL_GPL(kasprintf_strarray);

/**
 * kfree_strarray - free a number of dynamically allocated strings contained
 *                  in an array and the array itself
 *
 * @array: Dynamically allocated array of strings to free.
 * @n: Number of strings (starting from the beginning of the array) to free.
 *
 * Passing a non-NULL @array and @n == 0 as well as NULL @array are valid
 * use-cases. If @array is NULL, the function does nothing.
 */
/// # Safety
/// `array` is NULL or holds at least `n` kfree()-able strings.
#[no_mangle]
pub unsafe extern "C" fn kfree_strarray(array: *mut *mut c_char, n: usize) {
    if array.is_null() {
        return;
    }

    // The unsigned int index of the C loop.
    let mut i: c_uint = 0;
    // SAFETY: the first n entries and the array are kfree()-able.
    unsafe {
        while (i as usize) < n {
            kfree((*array.add(i as usize)).cast::<c_void>());
            i = i.wrapping_add(1);
        }
        kfree(array.cast::<c_void>());
    }
}
// EXPORT_SYMBOL_GPL(kfree_strarray);

/// `struct strarray`, private to the original translation unit.
#[repr(C)]
struct Strarray {
    array: *mut *mut c_char,
    n: usize,
}

/// The devres release callback of devm_kasprintf_strarray().
///
/// # Safety
/// `res` is the devres payload allocated for a `struct strarray`.
unsafe extern "C" fn devm_kfree_strarray(_dev: *mut device, res: *mut c_void) {
    let array = res.cast::<Strarray>();

    // SAFETY: the payload was filled by devm_kasprintf_strarray().
    unsafe { kfree_strarray((*array).array, (*array).n) };
}

/// # Safety
/// `dev` is a valid device and `prefix` a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn devm_kasprintf_strarray(dev: *mut device, prefix: *const c_char, n: usize) -> *mut *mut c_char {
    // SAFETY: the devres payload is a private struct strarray.
    unsafe {
        let ptr = lupos_string_helpers_devres_alloc(devm_kfree_strarray, size_of::<Strarray>()).cast::<Strarray>();
        if ptr.is_null() {
            return err_ptr(-(ENOMEM as c_long));
        }

        (*ptr).n = n;
        (*ptr).array = kasprintf_strarray(GFP_KERNEL, prefix, n);
        if (*ptr).array.is_null() {
            devres_free(ptr.cast::<c_void>());
            return err_ptr(-(ENOMEM as c_long));
        }

        devres_add(dev, ptr.cast::<c_void>());

        (*ptr).array
    }
}
// EXPORT_SYMBOL_GPL(devm_kasprintf_strarray);

/**
 * skip_spaces - Removes leading whitespace from @str.
 * @str: The string to be stripped.
 *
 * Returns a pointer to the first non-whitespace character in @str.
 */
/// # Safety
/// `str` is a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn skip_spaces(mut str: *const c_char) -> *mut c_char {
    // SAFETY: the scan stops at the NUL terminator, which is not a space.
    unsafe {
        while isspace(*str) {
            str = str.add(1);
        }
    }
    str.cast_mut()
}
// EXPORT_SYMBOL(skip_spaces);

/**
 * strim - Removes leading and trailing whitespace from @s.
 * @s: The string to be stripped.
 *
 * Note that the first trailing whitespace is replaced with a %NUL-terminator
 * in the given string @s. Returns a pointer to the first non-whitespace
 * character in @s.
 */
/// # Safety
/// `s` is a writable NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strim(s: *mut c_char) -> *mut c_char {
    // SAFETY: end only addresses bytes of s, or s - 1 when it is reached.
    unsafe {
        let size = strlen(s);
        if size == 0 {
            return s;
        }

        let mut end = s.add(size - 1);
        while end >= s && isspace(*end) {
            end = end.wrapping_sub(1);
        }
        *end.wrapping_add(1) = 0;

        skip_spaces(s)
    }
}
// EXPORT_SYMBOL(strim);

/**
 * sysfs_streq - return true if strings are equal, modulo trailing newline
 * @s1: one string
 * @s2: another string
 *
 * This routine returns true iff two strings are equal, treating both
 * NUL and newline-then-NUL as equivalent string terminations.  It's
 * geared for use with sysfs input strings, which generally terminate
 * with newlines but are compared against values without newlines.
 */
/// # Safety
/// `s1` and `s2` are NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn sysfs_streq(mut s1: *const c_char, mut s2: *const c_char) -> bool {
    // SAFETY: both scans stop at the first difference or NUL terminator.
    unsafe {
        while *s1 != 0 && *s1 == *s2 {
            s1 = s1.add(1);
            s2 = s2.add(1);
        }

        if *s1 == *s2 {
            return true;
        }
        if *s1 == 0 && *s2 == b'\n' && *s2.add(1) == 0 {
            return true;
        }
        if *s1 == b'\n' && *s1.add(1) == 0 && *s2 == 0 {
            return true;
        }
    }
    false
}
// EXPORT_SYMBOL(sysfs_streq);

/**
 * match_string - matches given string in an array
 * @array:	array of strings
 * @n:		number of strings in the array or -1 for NULL terminated arrays
 * @string:	string to match with
 *
 * This routine will look for a string in an array of strings up to the
 * n-th element in the array or until the first NULL element.
 *
 * Historically the value of -1 for @n, was used to search in arrays that
 * are NULL terminated. However, the function does not make a distinction
 * when finishing the search: either @n elements have been compared OR
 * the first NULL element was found.
 *
 * Return:
 * index of a @string in the @array if matches, or %-EINVAL otherwise.
 */
/// # Safety
/// `array` holds `n` entries or a NULL terminator; `string` is NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn match_string(array: *const *const c_char, n: usize, string: *const c_char) -> c_int {
    let mut index: c_int = 0;

    // SAFETY: entries are read up to n or the first NULL.
    unsafe {
        while (index as usize) < n {
            let item = *array.offset(index as isize);
            if item.is_null() {
                break;
            }
            if strcmp(item, string) == 0 {
                return index;
            }
            index = index.wrapping_add(1);
        }
    }

    -(EINVAL as c_int)
}
// EXPORT_SYMBOL(match_string);

/**
 * __sysfs_match_string - matches given string in an array
 * @array: array of strings
 * @n: number of strings in the array or -1 for NULL terminated arrays
 * @str: string to match with
 *
 * Returns index of @str in the @array or -EINVAL, just like match_string().
 * Uses sysfs_streq instead of strcmp for matching.
 *
 * This routine will look for a string in an array of strings up to the
 * n-th element in the array or until the first NULL element.
 *
 * Historically the value of -1 for @n, was used to search in arrays that
 * are NULL terminated. However, the function does not make a distinction
 * when finishing the search: either @n elements have been compared OR
 * the first NULL element was found.
 */
/// # Safety
/// `array` holds `n` entries or a NULL terminator; `str` is NUL-terminated.
#[no_mangle]
pub unsafe extern "C" fn __sysfs_match_string(array: *const *const c_char, n: usize, str: *const c_char) -> c_int {
    let mut index: c_int = 0;

    // SAFETY: entries are read up to n or the first NULL.
    unsafe {
        while (index as usize) < n {
            let item = *array.offset(index as isize);
            if item.is_null() {
                break;
            }
            if sysfs_streq(item, str) {
                return index;
            }
            index = index.wrapping_add(1);
        }
    }

    -(EINVAL as c_int)
}
// EXPORT_SYMBOL(__sysfs_match_string);

/**
 * strreplace - Replace all occurrences of character in string.
 * @str: The string to operate on.
 * @old: The character being replaced.
 * @new: The character @old is replaced with.
 *
 * Replaces the each @old character with a @new one in the given string @str.
 *
 * Return: pointer to the string @str itself.
 */
/// # Safety
/// `str` is a writable NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn strreplace(str: *mut c_char, old: c_char, new: c_char) -> *mut c_char {
    let mut s = str;

    // SAFETY: the scan stops at the NUL terminator.
    unsafe {
        while *s != 0 {
            if *s == old {
                *s = new;
            }
            s = s.add(1);
        }
    }
    str
}
// EXPORT_SYMBOL(strreplace);

/**
 * memcpy_and_pad - Copy one buffer to another with padding
 * @dest: Where to copy to
 * @dest_len: The destination buffer size
 * @src: Where to copy from
 * @count: The number of bytes to copy
 * @pad: Character to use for padding if space is left in destination.
 */
/// # Safety
/// `dest` is writable for `dest_len` bytes and `src` readable for the bytes
/// copied; the buffers do not overlap.
#[no_mangle]
pub unsafe extern "C" fn memcpy_and_pad(dest: *mut c_void, dest_len: usize, src: *const c_void, count: usize, pad: c_int) {
    // SAFETY: forwarded caller contract.
    unsafe {
        if dest_len > count {
            copy_nonoverlapping(src.cast::<u8>(), dest.cast::<u8>(), count);
            write_bytes(dest.cast::<u8>().add(count), pad as u8, dest_len - count);
        } else {
            copy_nonoverlapping(src.cast::<u8>(), dest.cast::<u8>(), dest_len);
        }
    }
}
// EXPORT_SYMBOL(memcpy_and_pad);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
