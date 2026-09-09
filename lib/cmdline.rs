// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/lib/cmdline.c
 * Helper functions generally used for parsing kernel command line
 * and module options.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong};

extern "C" {
    fn simple_strtol(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> isize;
    fn simple_strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulonglong;
    fn check_shl_overflow(n: c_ulonglong, shift: c_uint, result: *mut c_ulonglong) -> bool;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn isspace(c: c_int) -> c_int;
    fn skip_spaces(str: *mut c_char) -> *mut c_char;
}

unsafe fn get_range(str_: *mut *mut c_char, pint: *mut c_int, mut n: c_int) -> c_int {
    *str_ = (*str_).add(1);
    let upper_range = simple_strtol(*str_, core::ptr::null_mut(), 0) as c_int;
    let inc_counter = upper_range - *pint;
    let mut x = *pint;
    while n != 0 && x < upper_range {
        *pint = x;
        pint = pint.add(1);
        x += 1;
        n -= 1;
    }
    inc_counter
}

pub unsafe fn get_option(str_: *mut *mut c_char, pint: *mut c_int) -> c_int {
    let mut cur = *str_;
    let value: c_int;
    if cur.is_null() || *cur == 0 {
        return 0;
    }
    if *cur == b'-' as c_char {
        cur = cur.add(1);
        value = -(simple_strtoull(cur, str_, 0) as c_int);
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

pub unsafe fn get_options(mut str_: *const c_char, nints: c_int, ints: *mut c_int) -> *mut c_char {
    let validate = nints == 0;
    let mut i: c_int = 1;
    while i < nints || validate {
        let pint = if validate { ints } else { ints.add(i as usize) };
        let res = get_option(&mut (str_ as *mut c_char), pint);
        if res == 0 {
            break;
        }
        if res == 3 {
            let n = if validate { 0 } else { nints - i };
            let range_nums = get_range(&mut (str_ as *mut c_char), pint, n);
            if range_nums < 0 {
                break;
            }
            i += range_nums - 1;
        }
        i += 1;
        if res == 1 {
            break;
        }
    }
    *ints = i - 1;
    str_ as *mut c_char
}

pub unsafe fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> c_ulonglong {
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut ret = simple_strtoull(ptr, &mut endptr, 0);
    let mut shl: c_uint = 0;
    match *endptr as u8 {
        b'E' | b'e' | b'P' | b'p' | b'T' | b't' | b'G' | b'g' | b'M' | b'm' | b'K' | b'k' => {
            shl = 10;
        }
        _ => {}
    }
    if shl != 0 && ptr != endptr {
        if check_shl_overflow(ret, shl, &mut ret) {
            ret = c_ulonglong::MAX;
        }
        endptr = endptr.add(1);
    }
    if !retptr.is_null() {
        *retptr = endptr;
    }
    ret
}

pub unsafe fn parse_option_str(mut str_: *const c_char, option: *const c_char) -> bool {
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

pub unsafe fn next_arg(mut args: *mut c_char, param: *mut *mut c_char, val: *mut *mut c_char) -> *mut c_char {
    let mut i: usize = 0;
    let mut equals: usize = 0;
    let mut in_quote = false;
    let mut quoted = false;
    if *args == b'"' as c_char {
        args = args.add(1);
        in_quote = true;
        quoted = true;
    }
    while *args.add(i) != 0 {
        if isspace(*args.add(i) as c_int) != 0 && !in_quote {
            break;
        }
        if equals == 0 && *args.add(i) == b'=' as c_char {
            equals = i;
        }
        if *args.add(i) == b'"' as c_char {
            in_quote = !in_quote;
        }
        i += 1;
    }
    *param = args;
    if equals == 0 {
        *val = core::ptr::null_mut();
    } else {
        *args.add(equals) = 0;
        *val = args.add(equals + 1);
        if **val == b'"' as c_char {
            *val = (*val).add(1);
            if *args.add(i - 1) == b'"' as c_char {
                *args.add(i - 1) = 0;
            }
        }
    }
    if quoted && i > 0 && *args.add(i - 1) == b'"' as c_char {
        *args.add(i - 1) = 0;
    }
    if *args.add(i) != 0 {
        *args.add(i) = 0;
        args = args.add(i + 1);
    } else {
        args = args.add(i);
    }
    skip_spaces(args)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
