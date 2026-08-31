// SPDX-License-Identifier: GPL-2.0
// C dependencies from the original file:
// #include <string.h>
// #include <stdlib.h>
// #include "util/string2.h"
// #include "demangle-ocaml.h"
// #include <linux/ctype.h>

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

unsafe extern "C" {
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn isupper(c: c_int) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn hex(c: c_char) -> c_int;
}

const caml_prefix: *const c_char = b"caml\0".as_ptr() as *const c_char;
const caml_prefix_len: usize = 4;

/* mangled OCaml symbols start with "caml" followed by an upper-case letter */
unsafe fn ocaml_is_mangled(sym: *const c_char) -> bool {
    unsafe {
        0 == strncmp(sym, caml_prefix, caml_prefix_len)
            && isupper(*sym.add(caml_prefix_len) as c_int) != 0
    }
}

/*
 * input:
 *     sym: a symbol which may have been mangled by the OCaml compiler
 * return:
 *     if the input doesn't look like a mangled OCaml symbol, NULL is returned
 *     otherwise, a newly allocated string containing the demangled symbol is returned
 */
#[no_mangle]
pub unsafe extern "C" fn ocaml_demangle_sym(sym: *const c_char) -> *mut c_char {
    let result: *mut c_char;
    let mut j: c_int = 0;
    let mut i: c_int;
    let len: c_int;

    unsafe {
        if !ocaml_is_mangled(sym) {
            return ptr::null_mut();
        }

        len = strlen(sym) as c_int;

        /* the demangled symbol is always smaller than the mangled symbol */
        result = malloc(len as usize + 1) as *mut c_char;
        if result.is_null() {
            return ptr::null_mut();
        }

        /* skip "caml" prefix */
        i = caml_prefix_len as c_int;

        while i < len {
            if *sym.add(i as usize) == b'_' as c_char
                && *sym.add(i as usize + 1) == b'_' as c_char
            {
                /* "__" -> "." */
                *result.add(j as usize) = b'.' as c_char;
                j += 1;
                i += 2;
            } else if *sym.add(i as usize) == b'$' as c_char
                && isxdigit(*sym.add(i as usize + 1) as c_int) != 0
                && isxdigit(*sym.add(i as usize + 2) as c_int) != 0
            {
                /* "$xx" is a hex-encoded character */
                *result.add(j as usize) =
                    ((hex(*sym.add(i as usize + 1)) << 4) | hex(*sym.add(i as usize + 2)))
                        as c_char;
                j += 1;
                i += 3;
            } else {
                *result.add(j as usize) = *sym.add(i as usize);
                j += 1;
                i += 1;
            }
        }
        *result.add(j as usize) = b'\0' as c_char;

        result
    }
}
