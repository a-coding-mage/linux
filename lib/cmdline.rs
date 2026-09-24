// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/lib/cmdline.c
 * Helper functions generally used for parsing kernel command line
 * and module options.
 *
 * Code and copyrights come from init/main.c and arch/i386/kernel/setup.c.
 *
 * GNU Indent formatting options for this file: -kr -i8 -npsl -pcs
 */

//! Byte-oriented command-line helpers, retaining C pointer/write ordering.
//! No slices or references to caller storage are created: output pointers can
//! alias where the original C permits it. Arithmetic uses explicit wrapping
//! for the kernel's -fno-strict-overflow domain and never unwinds.

use core::ffi::{c_int, c_uint, c_ulonglong};
#[cfg(not(CONFIG_RUST))]
use core::ffi::c_char;
#[cfg(CONFIG_RUST)]
use kernel::ffi::c_char;
#[cfg(CONFIG_RUST)]
use kernel::bindings::{simple_strtol, simple_strtoull, skip_spaces, strlen, strncmp};

#[cfg(not(CONFIG_RUST))]
extern "C" {
    fn simple_strtol(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> isize;
    fn simple_strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulonglong;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn skip_spaces(str: *const c_char) -> *mut c_char;
}

unsafe fn get_range(str_: *mut *mut c_char, mut pint: *mut c_int, mut n: c_int) -> c_int {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        *str_ = (*str_).add(1);
        let upper_range = simple_strtol(*str_, core::ptr::null_mut(), 0) as c_int;
        let inc_counter = upper_range.wrapping_sub(*pint);
        let mut x = *pint;
        while n != 0 && x < upper_range {
            *pint = x;
            pint = pint.add(1);
            x = x.wrapping_add(1);
            n = n.wrapping_sub(1);
        }
        inc_counter
    }
}

/// Parse one integer, consuming a following comma and reporting range markers.
///
/// # Safety
/// `str_` points to writable pointer storage. Its value may be null or point to
/// a readable NUL-terminated string. `pint` is null or points to a writable int.
/// Storage must stay valid for the ordered C reads and writes, including when
/// the integer output overlaps input bytes.
#[no_mangle]
pub unsafe extern "C" fn get_option(str_: *mut *mut c_char, pint: *mut c_int) -> c_int {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        let mut cur = *str_;
        let value: c_int;
        if cur.is_null() || *cur == 0 {
            return 0;
        }
        if *cur == b'-' as c_char {
            cur = cur.add(1);
            value = simple_strtoull(cur, str_, 0).wrapping_neg() as c_int;
        } else {
            value = simple_strtoull(cur, str_, 0) as c_int;
        }
        if !pint.is_null() {
            *pint = value;
        }
        if cur == *str_ {
            return 0;
        }
        if **str_ == b',' as c_char {
            *str_ = (*str_).add(1);
            return 2;
        }
        if **str_ == b'-' as c_char {
            return 3;
        }
        1
    }
}

/// Parse an integer list and signed ranges, or count values when `nints` is zero.
///
/// # Safety
/// `ints` always has writable element zero and space for every element the C
/// algorithm visits. Input may be null; otherwise it must be a readable
/// NUL-terminated string whenever parsing occurs. A null input stops parsing
/// without reading it. Range/count arithmetic and any overlap must leave all visited
/// addresses valid, as required by C. Counting does not write past element zero.
#[no_mangle]
pub unsafe extern "C" fn get_options(str_: *const c_char, nints: c_int, ints: *mut c_int) -> *mut c_char {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        let mut str_ = str_ as *mut c_char;
        let validate = nints == 0;
        let mut i: c_int = 1;
        while i < nints || validate {
            let pint = if validate { ints } else { ints.offset(i as isize) };
            let res = get_option(&mut str_, pint);
            if res == 0 {
                break;
            }
            if res == 3 {
                let n = if validate { 0 } else { nints.wrapping_sub(i) };
                let range_nums = get_range(&mut str_, pint, n);
                if range_nums < 0 {
                    break;
                }
                i = i.wrapping_add(range_nums.wrapping_sub(1));
            }
            i = i.wrapping_add(1);
            if res == 1 {
                break;
            }
        }
        *ints = i.wrapping_sub(1);
        str_
    }
}

/// Parse a number and optional binary K/M/G/T/P/E suffix, saturating overflow.
///
/// # Safety
/// `ptr` is a readable NUL-terminated string. `retptr` is null or points to
/// writable pointer storage. It is written only after all input reads complete.
#[no_mangle]
pub unsafe extern "C" fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> c_ulonglong {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        let mut endptr: *mut c_char = core::ptr::null_mut();
        let mut ret = simple_strtoull(ptr, &mut endptr, 0);
        let shl: c_uint = match *endptr as u8 {
            b'E' | b'e' => 60,
            b'P' | b'p' => 50,
            b'T' | b't' => 40,
            b'G' | b'g' => 30,
            b'M' | b'm' => 20,
            b'K' | b'k' => 10,
            _ => 0,
        };
        if shl != 0 && ptr != endptr {
            ret = scaled_memory(ret, shl);
            endptr = endptr.add(1);
        }
        if !retptr.is_null() {
            *retptr = endptr;
        }
        ret
    }
}

/// Find an exact option in a comma-separated byte string.
///
/// # Safety
/// `str_` points to a readable NUL-terminated string. When it is nonempty,
/// `option` also points to a readable NUL-terminated string; for empty `str_`,
/// `option` is not read and may be null. The strings may overlap.
#[no_mangle]
pub unsafe extern "C" fn parse_option_str(mut str_: *const c_char, option: *const c_char) -> bool {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        while *str_ != 0 {
            let option_len = strlen(option);
            if strncmp(str_, option, option_len) == 0 {
                str_ = str_.add(option_len);
                if *str_ == 0 || *str_ == b',' as c_char {
                    return true;
                }
            }
            while *str_ != 0 && *str_ != b',' as c_char {
                str_ = str_.add(1);
            }
            if *str_ == b',' as c_char {
                str_ = str_.add(1);
            }
        }
        false
    }
}

/// Split one parameter/value pair in place, preserving the C quoting rules.
///
/// # Safety
/// `args` points to writable NUL-terminated bytes; `param` and `val` point to
/// writable pointer storage (neither may be null). Aliasing follows the C
/// operation order, including when both output pointers name the same slot.
/// All addresses reached after output writes must remain valid.
#[no_mangle]
pub unsafe extern "C" fn next_arg(mut args: *mut c_char, param: *mut *mut c_char, val: *mut *mut c_char) -> *mut c_char {
    // SAFETY: the C ABI contract above (or the calling wrapper) supplies all
    // accessed storage; raw pointers preserve the original aliasing order.
    unsafe {
        let mut i: c_uint = 0;
        let mut equals: c_uint = 0;
        let mut in_quote = false;
        let mut quoted = false;
        if *args == b'"' as c_char {
            args = args.add(1);
            in_quote = true;
            quoted = true;
        }
        while *args.add(i as usize) != 0 {
            if is_space(*args.add(i as usize) as u8) && !in_quote {
                break;
            }
            if equals == 0 && *args.add(i as usize) == b'=' as c_char {
                equals = i;
            }
            if *args.add(i as usize) == b'"' as c_char {
                in_quote = !in_quote;
            }
            i = i.wrapping_add(1);
        }
        *param = args;
        if equals == 0 {
            *val = core::ptr::null_mut();
        } else {
            *args.add(equals as usize) = 0;
            *val = args.add(equals as usize).add(1);
            if **val == b'"' as c_char {
                *val = (*val).add(1);
                if *args.add(i.wrapping_sub(1) as usize) == b'"' as c_char {
                    *args.add(i.wrapping_sub(1) as usize) = 0;
                }
            }
        }
        if quoted && i > 0 && *args.add(i.wrapping_sub(1) as usize) == b'"' as c_char {
            *args.add(i.wrapping_sub(1) as usize) = 0;
        }
        if *args.add(i as usize) != 0 {
            *args.add(i as usize) = 0;
            args = args.add(i.wrapping_add(1) as usize);
        } else {
            args = args.add(i as usize);
        }
        skip_spaces(args)
    }
}

// linux/ctype.h indexes unsigned bytes; lib/ctype.c includes Latin-1 NBSP.
fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t'..=b'\r' | 0xa0)
}

// All callers supply a suffix shift in 10..=60.
fn scaled_memory(value: u64, shift: u32) -> u64 {
    if value > u64::MAX.wrapping_shr(shift) {
        u64::MAX
    } else {
        value.wrapping_shl(shift)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
