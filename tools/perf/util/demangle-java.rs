// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/demangle-java.c.
// C includes referenced sys/types.h, stdio.h, stdlib.h, string.h, symbol.h,
// demangle-java.h, linux/ctype.h, and linux/kernel.h.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const MODE_PREFIX: c_int = 0;
const MODE_CLASS: c_int = 1;
const MODE_FUNC: c_int = 2;
const MODE_TYPE: c_int = 3;
const MODE_CTYPE: c_int = 4; /* class arg */

// JAVA_DEMANGLE_NORET is supplied by demangle-java.h in the original tree.
const JAVA_DEMANGLE_NORET: c_int = crate::JAVA_DEMANGLE_NORET;

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn isalpha(c: c_int) -> c_int;
}

static mut BASE_TYPES: [*const c_char; ('Z' as usize) - ('A' as usize) + 1] = {
    let mut base_types = [ptr::null(); ('Z' as usize) - ('A' as usize) + 1];
    base_types[('B' as usize) - ('A' as usize)] = b"byte\0".as_ptr() as *const c_char;
    base_types[('C' as usize) - ('A' as usize)] = b"char\0".as_ptr() as *const c_char;
    base_types[('D' as usize) - ('A' as usize)] = b"double\0".as_ptr() as *const c_char;
    base_types[('F' as usize) - ('A' as usize)] = b"float\0".as_ptr() as *const c_char;
    base_types[('I' as usize) - ('A' as usize)] = b"int\0".as_ptr() as *const c_char;
    base_types[('J' as usize) - ('A' as usize)] = b"long\0".as_ptr() as *const c_char;
    base_types[('S' as usize) - ('A' as usize)] = b"short\0".as_ptr() as *const c_char;
    base_types[('Z' as usize) - ('A' as usize)] = b"boolean\0".as_ptr() as *const c_char;
    base_types
};

/*
 * demangle Java symbol between str and end positions and stores
 * up to maxlen characters into buf. The parser starts in mode.
 *
 * Use MODE_PREFIX to process entire prototype till end position
 * Use MODE_TYPE to process return type if str starts on return type char
 *
 *  Return:
 *	success: buf
 *	error  : NULL
 */
unsafe fn __demangle_java_sym(
    str_: *const c_char,
    mut end: *const c_char,
    buf: *mut c_char,
    maxlen: c_int,
    mut mode: c_int,
) -> *mut c_char {
    let mut rlen: c_int = 0;
    let mut array: c_int = 0;
    let mut narg: c_int = 0;
    let mut q: *const c_char;

    if end.is_null() {
        end = str_.add(strlen(str_));
    }

    q = str_;
    while q != end {
        if rlen == maxlen - 1 {
            break;
        }

        match *q as u8 as char {
            'L' => {
                if mode == MODE_PREFIX || mode == MODE_TYPE {
                    if mode == MODE_TYPE {
                        if narg != 0 {
                            rlen += scnprintf(
                                buf.add(rlen as usize),
                                (maxlen - rlen) as usize,
                                b", \0".as_ptr() as *const c_char,
                            );
                        }
                        narg += 1;
                    }
                    if mode == MODE_PREFIX {
                        mode = MODE_CLASS;
                    } else {
                        mode = MODE_CTYPE;
                    }
                } else {
                    *buf.add(rlen as usize) = *q;
                    rlen += 1;
                }
            }
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => {
                if mode == MODE_TYPE {
                    if narg != 0 {
                        rlen += scnprintf(
                            buf.add(rlen as usize),
                            (maxlen - rlen) as usize,
                            b", \0".as_ptr() as *const c_char,
                        );
                    }
                    rlen += scnprintf(
                        buf.add(rlen as usize),
                        (maxlen - rlen) as usize,
                        b"%s\0".as_ptr() as *const c_char,
                        BASE_TYPES[(*q as usize) - ('A' as usize)],
                    );
                    while array != 0 {
                        array -= 1;
                        rlen += scnprintf(
                            buf.add(rlen as usize),
                            (maxlen - rlen) as usize,
                            b"[]\0".as_ptr() as *const c_char,
                        );
                    }
                    array = 0;
                    narg += 1;
                } else {
                    *buf.add(rlen as usize) = *q;
                    rlen += 1;
                }
            }
            'V' => {
                if mode == MODE_TYPE {
                    rlen += scnprintf(
                        buf.add(rlen as usize),
                        (maxlen - rlen) as usize,
                        b"void\0".as_ptr() as *const c_char,
                    );
                    while array != 0 {
                        array -= 1;
                        rlen += scnprintf(
                            buf.add(rlen as usize),
                            (maxlen - rlen) as usize,
                            b"[]\0".as_ptr() as *const c_char,
                        );
                    }
                    array = 0;
                } else {
                    *buf.add(rlen as usize) = *q;
                    rlen += 1;
                }
            }
            '[' => {
                if mode != MODE_TYPE {
                    return ptr::null_mut();
                }
                array += 1;
            }
            '(' => {
                if mode != MODE_FUNC {
                    return ptr::null_mut();
                }
                *buf.add(rlen as usize) = *q;
                rlen += 1;
                mode = MODE_TYPE;
            }
            ')' => {
                if mode != MODE_TYPE {
                    return ptr::null_mut();
                }
                *buf.add(rlen as usize) = *q;
                rlen += 1;
                narg = 0;
            }
            ';' => {
                if mode != MODE_CLASS && mode != MODE_CTYPE {
                    return ptr::null_mut();
                }
                /* safe because at least one other char to process */
                if isalpha(*q.add(1) as c_int) != 0 && mode == MODE_CLASS {
                    rlen += scnprintf(
                        buf.add(rlen as usize),
                        (maxlen - rlen) as usize,
                        b".\0".as_ptr() as *const c_char,
                    );
                }
                if mode == MODE_CLASS {
                    mode = MODE_FUNC;
                } else if mode == MODE_CTYPE {
                    mode = MODE_TYPE;
                }
            }
            '/' => {
                if mode != MODE_CLASS && mode != MODE_CTYPE {
                    return ptr::null_mut();
                }
                rlen += scnprintf(
                    buf.add(rlen as usize),
                    (maxlen - rlen) as usize,
                    b".\0".as_ptr() as *const c_char,
                );
            }
            _ => {
                *buf.add(rlen as usize) = *q;
                rlen += 1;
            }
        }

        q = q.add(1);
    }
    *buf.add(rlen as usize) = 0;
    buf
}

/*
 * Demangle Java function signature (openJDK, not GCJ)
 * input:
 * 	str: string to parse. String is not modified
 *    flags: combination of JAVA_DEMANGLE_* flags to modify demangling
 * return:
 *	if input can be demangled, then a newly allocated string is returned.
 *	if input cannot be demangled, then NULL is returned
 *
 * Note: caller is responsible for freeing demangled string
 */
#[no_mangle]
pub unsafe extern "C" fn java_demangle_sym(str_: *const c_char, flags: c_int) -> *mut c_char {
    let mut buf: *mut c_char;
    let mut ptr_: *mut c_char;
    let p: *const c_char;
    let len: usize;
    let mut l1: usize = 0;

    if str_.is_null() {
        return ptr::null_mut();
    }

    /* find start of return type */
    p = strrchr(str_, ')' as c_int) as *const c_char;
    if p.is_null() {
        return ptr::null_mut();
    }

    /*
     * expansion factor estimated to 3x
     */
    len = strlen(str_).wrapping_mul(3).wrapping_add(1);
    buf = malloc(len) as *mut c_char;
    if buf.is_null() {
        return ptr::null_mut();
    }

    *buf = 0;
    if flags & JAVA_DEMANGLE_NORET == 0 {
        /*
         * get return type first
         */
        ptr_ = __demangle_java_sym(p.add(1), ptr::null(), buf, len as c_int, MODE_TYPE);
        if ptr_.is_null() {
            free(buf as *mut c_void);
            return ptr::null_mut();
        }

        /* add space between return type and function prototype */
        l1 = strlen(buf);
        *buf.add(l1) = b' ' as c_char;
        l1 += 1;
    }

    /* process function up to return type */
    ptr_ = __demangle_java_sym(str_, p.add(1), buf.add(l1), (len - l1) as c_int, MODE_PREFIX);
    if ptr_.is_null() {
        free(buf as *mut c_void);
        return ptr::null_mut();
    }

    buf
}
