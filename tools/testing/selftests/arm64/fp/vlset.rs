// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015-2019 ARM Limited.
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

const NO_ARGUMENT: c_int = 0;
const EOF: c_int = -1;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

const PR_SVE_SET_VL: c_int = 50;
const PR_SVE_GET_VL: c_int = 51;
const PR_SVE_VL_LEN_MASK: c_long = 0xffff;
const PR_SVE_VL_INHERIT: c_long = 1 << 17;
const PR_SVE_SET_VL_ONEXEC: c_long = 1 << 18;
const PR_SME_SET_VL: c_int = 63;
const PR_SME_GET_VL: c_int = 64;

const AT_HWCAP: c_ulong = 16;
const HWCAP_SVE: c_ulong = 1 << 22;
const SVE_VL_MAX: c_ulong = 0xffff;
const ULONG_MAX: c_ulong = c_ulong::MAX;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe extern "C" {
    static mut optind: c_int;
    static mut stderr: *mut c_void;

    fn __errno_location() -> *mut c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, ...) -> c_int;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
}

static mut inherit: c_int = 0;
static mut no_inherit: c_int = 0;
static mut force: c_int = 0;
static mut vl: c_ulong = 0;
static mut set_ctl: c_int = PR_SVE_SET_VL;
static mut get_ctl: c_int = PR_SVE_GET_VL;

static mut options: [option; 7] = [
    option { name: c"force".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'f' as c_int },
    option { name: c"inherit".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'i' as c_int },
    option { name: c"max".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'M' as c_int },
    option { name: c"no-inherit".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::addr_of_mut!(no_inherit), val: 1 },
    option { name: c"sme".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b's' as c_int },
    option { name: c"help".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'?' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

static mut program_name: *const c_char = ptr::null();

unsafe fn parse_options_error() -> c_int {
    fprintf(
        stderr,
        c"Usage: %s [-f | --force] [-i | --inherit | --no-inherit] {-M | --max | <vector length>} <command> [<arguments> ...]\n".as_ptr(),
        program_name,
    );
    -1
}

unsafe fn parse_options(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut rest: *mut c_char = ptr::null_mut();

    program_name = strrchr(*argv.offset(0), b'/' as c_int);
    if !program_name.is_null() {
        program_name = program_name.offset(1);
    } else {
        program_name = *argv.offset(0);
    }

    loop {
        c = getopt_long(
            argc,
            argv,
            c"Mfhi".as_ptr(),
            ptr::addr_of!(options) as *const option,
            ptr::null_mut(),
        );
        if c == EOF {
            break;
        }

        match c {
            x if x == b'M' as c_int => vl = SVE_VL_MAX,
            x if x == b'f' as c_int => force = 1,
            x if x == b'i' as c_int => inherit = 1,
            x if x == b's' as c_int => {
                set_ctl = PR_SME_SET_VL;
                get_ctl = PR_SME_GET_VL;
            }
            0 => {}
            _ => return parse_options_error(),
        }
    }

    if inherit != 0 && no_inherit != 0 {
        return parse_options_error();
    }

    if vl == 0 {
        /* vector length */
        if optind >= argc {
            return parse_options_error();
        }

        *__errno_location() = 0;
        vl = strtoul(*argv.offset(optind as isize), ptr::addr_of_mut!(rest), 0);
        if *rest != 0 {
            vl = ULONG_MAX;
            *__errno_location() = EINVAL;
        }
        if vl == ULONG_MAX && *__errno_location() != 0 {
            fprintf(
                stderr,
                c"%s: %s: %s\n".as_ptr(),
                program_name,
                *argv.offset(optind as isize),
                strerror(*__errno_location()),
            );
            return parse_options_error();
        }

        optind += 1;
    }

    /* command */
    if optind >= argc {
        return parse_options_error();
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 126; /* same as sh(1) command-not-executable error */
    let mut flags: c_long;
    let path: *mut c_char;
    let mut t: c_int;
    let e: c_int;

    if parse_options(argc, argv) != 0 {
        return 2; /* same as sh(1) builtin incorrect-usage */
    }

    if vl & !(vl & PR_SVE_VL_LEN_MASK as c_ulong) != 0 {
        fprintf(
            stderr,
            c"%s: Invalid vector length %lu\n".as_ptr(),
            program_name,
            vl,
        );
        return 2; /* same as sh(1) builtin incorrect-usage */
    }

    if getauxval(AT_HWCAP) & HWCAP_SVE == 0 {
        fprintf(
            stderr,
            c"%s: Scalable Vector Extension not present\n".as_ptr(),
            program_name,
        );

        if force == 0 {
            return ret;
        }

        fputs(
            c"Going ahead anyway (--force):  This is a debug option.  Don't rely on it.\n".as_ptr(),
            stderr,
        );
    }

    flags = PR_SVE_SET_VL_ONEXEC;
    if inherit != 0 {
        flags |= PR_SVE_VL_INHERIT;
    }

    t = prctl(set_ctl, vl | flags as c_ulong);
    if t < 0 {
        fprintf(
            stderr,
            c"%s: PR_SVE_SET_VL: %s\n".as_ptr(),
            program_name,
            strerror(*__errno_location()),
        );
        return ret;
    }

    t = prctl(get_ctl);
    if t == -1 {
        fprintf(
            stderr,
            c"%s: PR_SVE_GET_VL: %s\n".as_ptr(),
            program_name,
            strerror(*__errno_location()),
        );
        return ret;
    }
    flags = PR_SVE_VL_LEN_MASK;
    flags = (t as c_long) & !flags;

    assert!(optind < argc);
    path = *argv.offset(optind as isize);

    execvp(path, argv.offset(optind as isize));
    e = *__errno_location();
    if *__errno_location() == ENOENT {
        ret = 127; /* same as sh(1) not-found error */
    }
    fprintf(
        stderr,
        c"%s: %s: %s\n".as_ptr(),
        program_name,
        path,
        strerror(e),
    );

    ret /* same as sh(1) not-executable error */
}
