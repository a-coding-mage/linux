// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source. Dependencies originally came from:
// "string2.h", <linux/kernel.h>, <linux/string.h>, <stdlib.h>, <linux/ctype.h>.

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};

pub type s64 = i64;
pub type size_t = c_ulong;

unsafe extern "C" {
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

#[inline]
unsafe fn isdigit(c: c_char) -> bool {
    c >= b'0' as c_char && c <= b'9' as c_char
}

#[inline]
unsafe fn islower(c: c_char) -> bool {
    c >= b'a' as c_char && c <= b'z' as c_char
}

#[inline]
unsafe fn isspace(c: c_char) -> bool {
    matches!(c as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

#[inline]
unsafe fn tolower(c: c_char) -> c_char {
    if c >= b'A' as c_char && c <= b'Z' as c_char {
        c + (b'a' - b'A') as c_char
    } else {
        c
    }
}

static GRAPH_DOTTED_LINE_BYTES: &[u8] =
    b"---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\0";
static DOTS_BYTES: &[u8] =
    b".....................................................................................................................................................................................................................\0";

#[no_mangle]
pub static graph_dotted_line: *const c_char = GRAPH_DOTTED_LINE_BYTES.as_ptr() as *const c_char;

#[no_mangle]
pub static dots: *const c_char = DOTS_BYTES.as_ptr() as *const c_char;

/*
 * perf_atoll()
 * Parse (\d+)(b|B|kb|KB|mb|MB|gb|GB|tb|TB) (e.g. "256MB")
 * and return its numeric value
 */
#[no_mangle]
pub unsafe extern "C" fn perf_atoll(str_: *const c_char) -> s64 {
    let mut length: s64;
    let mut p: *mut c_char = core::ptr::null_mut();
    let c: c_char;

    if !isdigit(*str_.offset(0)) {
        return -1;
    }

    length = strtoll(str_, &mut p, 10) as s64;
    c = *p;
    p = p.offset(1);
    match c {
        b'b' | b'B' => {
            if *p != 0 {
                return -1;
            }
            return length;
        }
        0 => return length,
        b'k' | b'K' => length <<= 10,
        b'm' | b'M' => length <<= 20,
        b'g' | b'G' => length <<= 30,
        b't' | b'T' => length <<= 40,
        _ => return -1,
    }
    /* we want the cases to match */
    if islower(c) {
        if strcmp(p, c"b".as_ptr()) != 0 {
            return -1;
        }
    } else if strcmp(p, c"B".as_ptr()) != 0 {
        return -1;
    }
    length
}

/* Character class matching */
unsafe fn __match_charclass(mut pat: *const c_char, c: c_char, npat: *mut *const c_char) -> bool {
    let mut complement = false;
    let mut ret = true;

    if *pat == b'!' as c_char {
        complement = true;
        pat = pat.offset(1);
    }
    if {
        let old = pat;
        pat = pat.offset(1);
        *old == c
    } {
        /* First character is special */
        while *pat != 0 && *pat != b']' as c_char {
            pat = pat.offset(1);
        }
        if *pat == 0 {
            return false;
        }
        *npat = pat.offset(1);
        return if complement { !ret } else { ret };
    }

    while *pat != 0 && *pat != b']' as c_char {
        /* Matching */
        if *pat == b'-' as c_char && *pat.offset(1) != b']' as c_char {
            /* Range */
            if *pat.offset(-1) <= c && c <= *pat.offset(1) {
                while *pat != 0 && *pat != b']' as c_char {
                    pat = pat.offset(1);
                }
                if *pat == 0 {
                    return false;
                }
                *npat = pat.offset(1);
                return if complement { !ret } else { ret };
            }
            if *pat.offset(-1) > *pat.offset(1) {
                return false;
            }
            pat = pat.offset(2);
        } else {
            let old = pat;
            pat = pat.offset(1);
            if *old == c {
                while *pat != 0 && *pat != b']' as c_char {
                    pat = pat.offset(1);
                }
                if *pat == 0 {
                    return false;
                }
                *npat = pat.offset(1);
                return if complement { !ret } else { ret };
            }
        }
    }
    if *pat == 0 {
        return false;
    }
    ret = false;

    while *pat != 0 && *pat != b']' as c_char {
        /* Searching closing */
        pat = pat.offset(1);
    }
    if *pat == 0 {
        return false;
    }
    *npat = pat.offset(1);
    if complement {
        !ret
    } else {
        ret
    }
}

/* Glob/lazy pattern matching */
unsafe fn __match_glob(
    mut str_: *const c_char,
    mut pat: *const c_char,
    ignore_space: bool,
    case_ins: bool,
) -> bool {
    while *str_ != 0 && *pat != 0 && *pat != b'*' as c_char {
        if ignore_space {
            /* Ignore spaces for lazy matching */
            if isspace(*str_) {
                str_ = str_.offset(1);
                continue;
            }
            if isspace(*pat) {
                pat = pat.offset(1);
                continue;
            }
        }
        if *pat == b'?' as c_char {
            /* Matches any single character */
            str_ = str_.offset(1);
            pat = pat.offset(1);
            continue;
        } else if *pat == b'[' as c_char {
            /* Character classes/Ranges */
            if __match_charclass(pat.offset(1), *str_, &mut pat) {
                str_ = str_.offset(1);
                continue;
            } else {
                return false;
            }
        } else if *pat == b'\\' as c_char {
            /* Escaped char match as normal char */
            pat = pat.offset(1);
        }
        if case_ins {
            if tolower(*str_) != tolower(*pat) {
                return false;
            }
        } else if *str_ != *pat {
            return false;
        }
        str_ = str_.offset(1);
        pat = pat.offset(1);
    }
    /* Check wild card */
    if *pat == b'*' as c_char {
        while *pat == b'*' as c_char {
            pat = pat.offset(1);
        }
        if *pat == 0 {
            /* Tail wild card matches all */
            return true;
        }
        while *str_ != 0 {
            let cur = str_;
            str_ = str_.offset(1);
            if __match_glob(cur, pat, ignore_space, case_ins) {
                return true;
            }
        }
    }
    *str_ == 0 && *pat == 0
}

/**
 * strglobmatch - glob expression pattern matching
 * @str: the target string to match
 * @pat: the pattern string to match
 *
 * This returns true if the @str matches @pat. @pat can includes wildcards
 * ('*','?') and character classes ([CHARS], complementation and ranges are
 * also supported). Also, this supports escape character ('\') to use special
 * characters as normal character.
 *
 * Note: if @pat syntax is broken, this always returns false.
 */
#[no_mangle]
pub unsafe extern "C" fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool {
    __match_glob(str_, pat, false, false)
}

#[no_mangle]
pub unsafe extern "C" fn strglobmatch_nocase(str_: *const c_char, pat: *const c_char) -> bool {
    __match_glob(str_, pat, false, true)
}

/**
 * strlazymatch - matching pattern strings lazily with glob pattern
 * @str: the target string to match
 * @pat: the pattern string to match
 *
 * This is similar to strglobmatch, except this ignores spaces in
 * the target string.
 */
#[no_mangle]
pub unsafe extern "C" fn strlazymatch(str_: *const c_char, pat: *const c_char) -> bool {
    __match_glob(str_, pat, true, false)
}

/**
 * strtailcmp - Compare the tail of two strings
 * @s1: 1st string to be compared
 * @s2: 2nd string to be compared
 *
 * Return 0 if whole of either string is same as another's tail part.
 */
#[no_mangle]
pub unsafe extern "C" fn strtailcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let mut i1 = strlen(s1) as c_int;
    let mut i2 = strlen(s2) as c_int;
    loop {
        i1 -= 1;
        i2 -= 1;
        if !(i1 >= 0 && i2 >= 0) {
            break;
        }
        if *s1.offset(i1 as isize) != *s2.offset(i2 as isize) {
            return *s1.offset(i1 as isize) as c_int - *s2.offset(i2 as isize) as c_int;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn asprintf_expr_inout_ints(
    var: *const c_char,
    in_: bool,
    nints: size_t,
    ints: *mut c_int,
) -> *mut c_char {
    /*
     * FIXME: replace this with an expression using log10() when we
     * find a suitable implementation, maybe the one in the dvb drivers...
     *
     * "%s == %d || " = log10(MAXINT) * 2 + 8 chars for the operators
     */
    let size: size_t = nints.wrapping_mul(28).wrapping_add(1); /* \0 */
    let mut i: size_t;
    let mut printed: size_t = 0;
    let expr = malloc(size) as *mut c_char;

    if !expr.is_null() {
        let mut or_and = c"||".as_ptr();
        let mut eq_neq = c"==".as_ptr();
        let e = expr;

        if !in_ {
            or_and = c"&&".as_ptr();
            eq_neq = c"!=".as_ptr();
        }

        i = 0;
        while i < nints {
            if printed == size {
                free(expr as *mut c_void);
                return core::ptr::null_mut();
            }

            if i > 0 {
                printed = printed.wrapping_add(
                    scnprintf(
                        e.offset(printed as isize),
                        size.wrapping_sub(printed),
                        c" %s ".as_ptr(),
                        or_and,
                    ) as size_t,
                );
            }
            printed = printed.wrapping_add(
                scnprintf(
                    e.offset(printed as isize),
                    size.wrapping_sub(printed),
                    c"%s %s %d".as_ptr(),
                    var,
                    eq_neq,
                    *ints.offset(i as isize),
                ) as size_t,
            );
            i += 1;
        }
    }

    expr
}

/* Like strpbrk(), but not break if it is right after a backslash (escaped) */
#[no_mangle]
pub unsafe extern "C" fn strpbrk_esc(mut str_: *mut c_char, stopset: *const c_char) -> *mut c_char {
    let mut ptr: *mut c_char;

    loop {
        ptr = strpbrk(str_, stopset);
        if ptr.is_null() {
            /* stopset not in str. */
            break;
        }
        if ptr == str_ {
            /* stopset character is first in str. */
            break;
        }
        if ptr == str_.offset(1) && *str_.offset(0) != b'\\' as c_char {
            /* stopset chacter is second and wasn't preceded by a '\'. */
            break;
        }
        str_ = ptr.offset(1);
        if !(*ptr.offset(-1) == b'\\' as c_char && *ptr.offset(-2) != b'\\' as c_char) {
            break;
        }
    }

    ptr
}

/* Like strpbrk_esc(), but not break if it is quoted with single/double quotes */
#[no_mangle]
pub unsafe extern "C" fn strpbrk_esq(mut str_: *mut c_char, stopset: *const c_char) -> *mut c_char {
    let mut _stopset: *mut c_char = core::ptr::null_mut();
    let mut ptr: *mut c_char;
    let squote = c"'".as_ptr();
    let dquote = c"\"".as_ptr();

    if asprintf(&mut _stopset, c"%s%c%c".as_ptr(), stopset, *squote, *dquote) < 0 {
        return core::ptr::null_mut();
    }

    loop {
        ptr = strpbrk_esc(str_, _stopset);
        if ptr.is_null() {
            break;
        }
        if *ptr == *squote {
            ptr = strpbrk_esc(ptr.offset(1), squote);
        } else if *ptr == *dquote {
            ptr = strpbrk_esc(ptr.offset(1), dquote);
        } else {
            break;
        }
        str_ = ptr.offset(1);
        if ptr.is_null() {
            break;
        }
    }

    free(_stopset as *mut c_void);
    ptr
}

/* Like strdup, but do not copy a single backslash */
#[no_mangle]
pub unsafe extern "C" fn strdup_esc(str_: *const c_char) -> *mut c_char {
    let mut s: *mut c_char;
    let mut d: *mut c_char;
    let mut p: *mut c_char;
    let ret = strdup(str_);

    if ret.is_null() {
        return core::ptr::null_mut();
    }

    d = strchr(ret, b'\\' as c_int);
    if d.is_null() {
        return ret;
    }

    s = d.offset(1);
    loop {
        if *s == 0 {
            *d = 0;
            break;
        }
        p = strchr(s.offset(1), b'\\' as c_int);
        if !p.is_null() {
            memmove(d as *mut c_void, s as *const c_void, p.offset_from(s) as size_t);
            d = d.offset(p.offset_from(s));
            s = p.offset(1);
        } else {
            memmove(
                d as *mut c_void,
                s as *const c_void,
                strlen(s).wrapping_add(1),
            );
        }
        if p.is_null() {
            break;
        }
    }

    ret
}

/* Remove backslash right before quote and return next quote address. */
unsafe fn remove_consumed_esc(str_: *mut c_char, len: c_int, quote: c_int) -> *mut c_char {
    let mut ptr = str_;
    let mut end = str_.offset(len as isize);

    while *ptr != quote as c_char && ptr < end {
        if *ptr == b'\\' as c_char && *ptr.offset(1) == quote as c_char {
            memmove(
                ptr as *mut c_void,
                ptr.offset(1) as *const c_void,
                end.offset_from(ptr.offset(1)) as size_t,
            );
            /* now *ptr is `quote`. */
            end = end.offset(-1);
        }
        ptr = ptr.offset(1);
    }

    if *ptr == quote as c_char {
        ptr
    } else {
        core::ptr::null_mut()
    }
}

/*
 * Like strdup_esc, but keep quoted string as it is (and single backslash
 * before quote is removed). If there is no closed quote, return NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn strdup_esq(str_: *const c_char) -> *mut c_char {
    let mut d: *mut c_char;
    let ret: *mut c_char;

    /* If there is no quote, return normal strdup_esc() */
    d = strpbrk_esc(str_ as *mut c_char, c"\"'".as_ptr());
    if d.is_null() {
        return strdup_esc(str_);
    }

    ret = strdup(str_);
    if ret.is_null() {
        return core::ptr::null_mut();
    }

    d = ret;
    loop {
        d = strpbrk(d, c"\\\"'".as_ptr());
        if d.is_null() {
            break;
        }

        if *d == b'"' as c_char || *d == b'\'' as c_char {
            /* This is non-escaped quote */
            let quote = *d as c_int;
            let len = strlen(d.offset(1)).wrapping_add(1) as c_int;

            /*
             * Remove the start quote and remove consumed escape (backslash
             * before quote) and remove the end quote. If there is no end
             * quote, it is the input error.
             */
            memmove(d as *mut c_void, d.offset(1) as *const c_void, len as size_t);
            d = remove_consumed_esc(d, len, quote);
            if d.is_null() {
                free(ret as *mut c_void);
                return core::ptr::null_mut();
            }
            memmove(
                d as *mut c_void,
                d.offset(1) as *const c_void,
                strlen(d.offset(1)).wrapping_add(1),
            );
        }
        if *d == b'\\' as c_char {
            memmove(
                d as *mut c_void,
                d.offset(1) as *const c_void,
                strlen(d.offset(1)).wrapping_add(1),
            );
            if *d == b'\\' as c_char {
                /* double backslash -- keep the second one. */
                d = d.offset(1);
            }
        }
        if *d == 0 {
            break;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn hex(c: c_char) -> c_uint {
    if c >= b'0' as c_char && c <= b'9' as c_char {
        return (c - b'0' as c_char) as c_uint;
    }
    if c >= b'a' as c_char && c <= b'f' as c_char {
        return (c - b'a' as c_char + 10) as c_uint;
    }
    (c - b'A' as c_char + 10) as c_uint
}

/*
 * Replace all occurrences of character 'needle' in string 'haystack' with
 * string 'replace'
 *
 * The new string could be longer so a new string is returned which must be
 * freed.
 */
#[no_mangle]
pub unsafe extern "C" fn strreplace_chars(
    needle: c_char,
    haystack: *const c_char,
    replace: *const c_char,
) -> *mut c_char {
    let replace_len = strlen(replace) as c_int;
    let mut new_s: *mut c_char;
    let mut to: *mut c_char;
    let mut loc = strchr(haystack, needle as c_int) as *const c_char;
    let mut from = haystack;
    let mut num: c_int = 0;

    /* Count occurrences */
    while !loc.is_null() {
        loc = strchr(loc.offset(1), needle as c_int) as *const c_char;
        num += 1;
    }

    /* Allocate enough space for replacements and reset first location */
    new_s = malloc(
        strlen(haystack).wrapping_add((num * (replace_len - 1) + 1) as size_t),
    ) as *mut c_char;
    if new_s.is_null() {
        return core::ptr::null_mut();
    }
    loc = strchr(haystack, needle as c_int) as *const c_char;
    to = new_s;

    while !loc.is_null() {
        /* Copy original string up to found char and update positions */
        memcpy(
            to as *mut c_void,
            from as *const c_void,
            (1 + loc.offset_from(from)) as size_t,
        );
        to = to.offset(loc.offset_from(from));
        from = loc.offset(1);

        /* Copy replacement string and update positions */
        memcpy(to as *mut c_void, replace as *const c_void, replace_len as size_t);
        to = to.offset(replace_len as isize);

        /* needle next occurrence or end of string */
        loc = strchr(from, needle as c_int) as *const c_char;
    }

    /* Copy any remaining chars + null */
    strcpy(to, from);

    new_s
}
