// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source. Original dependencies:
// <stdbool.h>, <stdlib.h>, <stdint.h>, <string.h>, <stdio.h>,
// "util/debug.h", <dwarf-regs.h>, <subcmd/parse-options.h>,
// "util/perf_regs.h", "util/parse-regs-options.h"

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};

type uint64_t = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn perf_reg_name(reg: c_int, e_machine: c_uint, flags: c_uint) -> *const c_char;
    fn perf_intr_reg_mask(e_machine: c_uint) -> uint64_t;
    fn perf_user_reg_mask(e_machine: c_uint) -> uint64_t;
    fn ui__warning(format: *const c_char, ...) -> c_int;
}

// Provided by dwarf/perf architecture headers in the original C translation unit.
extern "C" {
    static EM_HOST: c_uint;
    static EF_HOST: c_uint;
}

unsafe fn list_perf_regs(fp: *mut FILE, mask: uint64_t) {
    let mut last_name: *const c_char = std::ptr::null();

    fprintf(fp, b"available registers: \0".as_ptr() as *const c_char);
    for reg in 0..64 {
        let name: *const c_char;

        if (((1u64 << reg) & mask) == 0) {
            continue;
        }

        name = perf_reg_name(reg as c_int, EM_HOST, EF_HOST);
        if !name.is_null()
            && (last_name.is_null() || strcmp(last_name, name) != 0)
        {
            fprintf(
                fp,
                b"%s%s\0".as_ptr() as *const c_char,
                if reg > 0 {
                    b" \0".as_ptr() as *const c_char
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                name,
            );
        }
        last_name = name;
    }
    fputc('\n' as c_int, fp);
}

unsafe fn name_to_perf_reg_mask(to_match: *const c_char, mask: uint64_t) -> uint64_t {
    let mut reg_mask: uint64_t = 0;

    for reg in 0..64 {
        let name: *const c_char;

        if (((1u64 << reg) & mask) == 0) {
            continue;
        }

        name = perf_reg_name(reg as c_int, EM_HOST, EF_HOST);
        if name.is_null() {
            continue;
        }

        if strcasecmp(to_match, name) == 0 {
            reg_mask |= 1u64 << reg;
        }
    }
    reg_mask
}

unsafe fn __parse_regs(
    opt: *const option,
    str_: *const c_char,
    unset: c_int,
    intr: bool,
) -> c_int {
    let mode = (*opt).value as *mut uint64_t;
    let mut s: *mut c_char;
    let os: *mut c_char;
    let mut p: *mut c_char;
    let mut ret: c_int = -1;
    let mask: uint64_t;

    if unset != 0 {
        return 0;
    }

    /*
     * cannot set it twice
     */
    if *mode != 0 {
        return -1;
    }

    mask = if intr {
        perf_intr_reg_mask(EM_HOST)
    } else {
        perf_user_reg_mask(EM_HOST)
    };

    /* str may be NULL in case no arg is passed to -I */
    if str_.is_null() {
        *mode = mask;
        return 0;
    }

    /* because str is read-only */
    s = strdup(str_);
    os = s;
    if s.is_null() {
        return -1;
    }

    loop {
        let reg_mask: uint64_t;

        p = strchr(s, ',' as c_int);
        if !p.is_null() {
            *p = '\0' as c_char;
        }

        if strcmp(s, b"?\0".as_ptr() as *const c_char) == 0 {
            list_perf_regs(stderr, mask);
            goto_error_free(os, ret);
            return ret;
        }

        reg_mask = name_to_perf_reg_mask(s, mask);
        if reg_mask == 0 {
            ui__warning(
                b"Unknown register \"%s\", check man page or run \"perf record %s?\"\n\0"
                    .as_ptr() as *const c_char,
                s,
                if intr {
                    b"-I\0".as_ptr() as *const c_char
                } else {
                    b"--user-regs=\0".as_ptr() as *const c_char
                },
            );
            goto_error_free(os, ret);
            return ret;
        }
        *mode |= reg_mask;

        if p.is_null() {
            break;
        }

        s = p.add(1);
    }
    ret = 0;

    free(os as *mut c_void);
    ret
}

unsafe fn goto_error_free(os: *mut c_char, ret: c_int) {
    free(os as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn parse_user_regs(
    opt: *const option,
    str_: *const c_char,
    unset: c_int,
) -> c_int {
    __parse_regs(opt, str_, unset, false)
}

#[no_mangle]
pub unsafe extern "C" fn parse_intr_regs(
    opt: *const option,
    str_: *const c_char,
    unset: c_int,
) -> c_int {
    __parse_regs(opt, str_, unset, true)
}
