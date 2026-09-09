// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/parser.c - simple parser for mount, etc. options.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

// Kernel-provided types and functions are supplied by the surrounding build.
pub const NUMBER_BUF_LEN: usize = 24;

#[repr(C)]
pub struct substring_t {
    pub from: *mut c_char,
    pub to: *mut c_char,
}

#[repr(C)]
pub struct match_token {
    pub pattern: *const c_char,
    pub token: c_int,
}

pub type match_table_t = *const match_token;

extern "C" {
    static MAX_OPT_ARGS: c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn isdigit(c: c_int) -> c_int;
    fn simple_strtol(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_long;
    fn simple_strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn kstrtoull(s: *const c_char, base: c_uint, result: *mut c_ulonglong) -> c_int;
    fn kstrtouint(s: *const c_char, base: c_uint, result: *mut c_uint) -> c_int;
    fn kmemdup_nul(src: *const c_void, len: usize, flags: c_ulong) -> *mut c_char;
}

type c_long = isize;
const GFP_KERNEL: c_ulong = 0;
const INT_MIN: c_long = -2147483648;
const INT_MAX: c_long = 2147483647;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;

unsafe fn match_one(mut s: *mut c_char, mut p: *const c_char, args: *mut substring_t) -> c_int {
    let mut argc: c_int = 0;
    if p.is_null() { return 1; }
    loop {
        let mut len: isize = -1;
        let meta = strchr(p, '%' as c_int);
        if meta.is_null() { return (strcmp(p, s) == 0) as c_int; }
        if strncmp(p, s, meta.offset_from(p) as usize) != 0 { return 0; }
        s = s.offset(meta.offset_from(p));
        p = meta.add(1);
        if isdigit(*p as c_int) != 0 {
            let mut end: *mut c_char = core::ptr::null_mut();
            len = simple_strtoul(p, &mut end, 10) as isize;
            p = end;
        } else if *p == b'%' as c_char {
            if *s != b'%' as c_char { return 0; }
            s = s.add(1); p = p.add(1); continue;
        }
        if argc >= MAX_OPT_ARGS { return 0; }
        let arg = &mut *args.add(argc as usize);
        arg.from = s;
        let kind = *p; p = p.add(1);
        match kind {
            b's' as c_char => {
                let str_len = strlen(s) as isize;
                if str_len == 0 { return 0; }
                if len == -1 || len > str_len { len = str_len; }
                arg.to = s.offset(len);
            }
            b'd' as c_char => { simple_strtol(s, &mut arg.to, 0); }
            b'u' as c_char => { simple_strtoul(s, &mut arg.to, 0); }
            b'o' as c_char => { simple_strtoul(s, &mut arg.to, 8); }
            b'x' as c_char => { simple_strtoul(s, &mut arg.to, 16); }
            _ => return 0,
        }
        if kind != b's' as c_char && arg.to == arg.from { return 0; }
        s = arg.to;
        argc += 1;
    }
}

pub unsafe extern "C" fn match_token(s: *mut c_char, table: match_table_t, args: *mut substring_t) -> c_int {
    let mut p = table;
    while match_one(s, (*p).pattern, args) == 0 { p = p.add(1); }
    (*p).token
}

unsafe fn match_number(s: *mut substring_t, result: *mut c_int, base: c_int) -> c_int {
    let mut buf = [0 as c_char; NUMBER_BUF_LEN];
    if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN { return -ERANGE; }
    let mut endp = core::ptr::null_mut();
    let val = simple_strtol(buf.as_ptr(), &mut endp, base as c_uint);
    if endp == buf.as_mut_ptr() { -EINVAL } else if val < INT_MIN || val > INT_MAX { -ERANGE } else { *result = val as c_int; 0 }
}

unsafe fn match_u64int(s: *mut substring_t, result: *mut c_ulonglong, base: c_int) -> c_int {
    let mut buf = [0 as c_char; NUMBER_BUF_LEN];
    if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN { return -ERANGE; }
    let mut val = 0;
    let ret = kstrtoull(buf.as_ptr(), base as c_uint, &mut val);
    if ret == 0 { *result = val; }
    ret
}

pub unsafe extern "C" fn match_int(s: *mut substring_t, result: *mut c_int) -> c_int { match_number(s, result, 0) }
pub unsafe extern "C" fn match_uint(s: *mut substring_t, result: *mut c_uint) -> c_int {
    let mut buf = [0 as c_char; NUMBER_BUF_LEN];
    if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN { return -ERANGE; }
    kstrtouint(buf.as_ptr(), 10, result)
}
pub unsafe extern "C" fn match_u64(s: *mut substring_t, result: *mut c_ulonglong) -> c_int { match_u64int(s, result, 0) }
pub unsafe extern "C" fn match_octal(s: *mut substring_t, result: *mut c_int) -> c_int { match_number(s, result, 8) }
pub unsafe extern "C" fn match_hex(s: *mut substring_t, result: *mut c_int) -> c_int { match_number(s, result, 16) }

pub unsafe extern "C" fn match_wildcard(pattern: *const c_char, str_: *const c_char) -> bool {
    let (mut s, mut p) = (str_, pattern); let mut star = false;
    let (mut saved_s, mut saved_p) = (str_, pattern);
    while *s != 0 {
        match *p {
            b'?' as c_char => { s = s.add(1); p = p.add(1); }
            b'*' as c_char => { star = true; saved_s = s; p = p.add(1); if *p == 0 { return true; } saved_p = p; }
            _ => if *s == *p { s = s.add(1); p = p.add(1); } else if !star { return false; } else { saved_s = saved_s.add(1); s = saved_s; p = saved_p; }
        }
    }
    while *p == b'*' as c_char { p = p.add(1); }
    *p == 0
}

pub unsafe extern "C" fn match_strlcpy(dest: *mut c_char, src: *const substring_t, size: usize) -> usize {
    let ret = (*src).to.offset_from((*src).from) as usize;
    if size != 0 { let len = if ret >= size { size - 1 } else { ret }; memcpy(dest, (*src).from as *const c_void, len); *dest.add(len) = 0; }
    ret
}

pub unsafe extern "C" fn match_strdup(s: *const substring_t) -> *mut c_char {
    kmemdup_nul((*s).from as *const c_void, (*s).to.offset_from((*s).from) as usize, GFP_KERNEL)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
