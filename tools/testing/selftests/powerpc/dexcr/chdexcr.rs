// SPDX-License-Identifier: GPL-2.0-or-later

use core::ffi::{c_char, c_int, c_ulong};
use core::ptr;

// C dependencies originally supplied by:
// <errno.h>, <stddef.h>, <stdio.h>, <stdlib.h>, <string.h>, <sys/prctl.h>,
// "dexcr.h", and "utils.h".

#[repr(C)]
pub struct dexcr_aspect {
    pub opt: *const c_char,
    pub desc: *const c_char,
    pub prctl: c_int,
}

unsafe extern "C" {
    static aspects: dexcr_aspect;
    static ARRAY_SIZE_aspects: usize;

    static PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC: c_ulong;
    static PR_PPC_DEXCR_CTRL_SET_ONEXEC: c_ulong;

    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
    fn perror(s: *const c_char);
    fn __errno_location() -> *mut c_int;

    fn dexcr_exists() -> c_int;
    fn pr_set_dexcr(aspect: c_int, ctrl: c_ulong) -> c_int;
}

unsafe fn aspect_at(i: usize) -> *const dexcr_aspect {
    (&aspects as *const dexcr_aspect).add(i)
}

unsafe fn die(msg: *const c_char) -> ! {
    printf(b"%s\n\0".as_ptr() as *const c_char, msg);
    exit(1);
}

unsafe fn help() {
    printf(
        b"Invoke a provided program with a custom DEXCR on-exec reset value\n\
          \n\
          usage: chdexcr [CHDEXCR OPTIONS] -- PROGRAM [ARGS...]\n\
          \n\
          Each configurable DEXCR aspect is exposed as an option.\n\
          \n\
          The normal option sets the aspect in the DEXCR. The --no- variant\n\
          clears that aspect. For example, --ibrtpd sets the IBRTPD aspect bit,\n\
          so indirect branch prediction will be disabled in the provided program.\n\
          Conversely, --no-ibrtpd clears the aspect bit, so indirect branch\n\
          prediction may occur.\n\
          \n\
          CHDEXCR OPTIONS:\n\0"
            .as_ptr() as *const c_char,
    );

    let mut i = 0usize;
    while i < ARRAY_SIZE_aspects {
        let aspect = aspect_at(i);

        if (*aspect).prctl == -1 {
            i += 1;
            continue;
        }

        printf(
            b"  --%-6s / --no-%-6s : %s\n\0".as_ptr() as *const c_char,
            (*aspect).opt,
            (*aspect).opt,
            (*aspect).desc,
        );
        i += 1;
    }
}

unsafe fn opt_to_aspect(opt: *const c_char) -> *const dexcr_aspect {
    let mut i = 0usize;
    while i < ARRAY_SIZE_aspects {
        let aspect = aspect_at(i);
        if (*aspect).prctl != -1 && strcmp((*aspect).opt, opt) == 0 {
            return aspect;
        }
        i += 1;
    }

    ptr::null()
}

unsafe fn apply_option(option: *const c_char) -> c_int {
    let mut aspect: *const dexcr_aspect;
    let mut opt: *const c_char = ptr::null();
    let set_prefix = b"--\0".as_ptr() as *const c_char;
    let clear_prefix = b"--no-\0".as_ptr() as *const c_char;
    let mut ctrl: c_ulong = 0;
    let err: c_int;

    if strcmp(option, b"-h\0".as_ptr() as *const c_char) == 0
        || strcmp(option, b"--help\0".as_ptr() as *const c_char) == 0
    {
        help();
        exit(0);
    }

    /* Strip out --(no-) prefix and determine ctrl value */
    if strncmp(option, clear_prefix, strlen(clear_prefix)) == 0 {
        opt = option.add(strlen(clear_prefix));
        ctrl |= PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC;
    } else if strncmp(option, set_prefix, strlen(set_prefix)) == 0 {
        opt = option.add(strlen(set_prefix));
        ctrl |= PR_PPC_DEXCR_CTRL_SET_ONEXEC;
    }

    if opt.is_null() || *opt == 0 {
        return 1;
    }

    aspect = opt_to_aspect(opt);
    if aspect.is_null() {
        die(b"unknown aspect\0".as_ptr() as *const c_char);
    }

    err = pr_set_dexcr((*aspect).prctl, ctrl);
    if err != 0 {
        die(b"failed to apply option\0".as_ptr() as *const c_char);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;

    if dexcr_exists() == 0 {
        die(b"DEXCR not detected on this hardware\0".as_ptr() as *const c_char);
    }

    i = 1;
    while i < argc {
        if apply_option(*argv.add(i as usize) as *const c_char) != 0 {
            break;
        }
        i += 1;
    }

    if i < argc
        && strcmp(
            *argv.add(i as usize) as *const c_char,
            b"--\0".as_ptr() as *const c_char,
        ) == 0
    {
        i += 1;
    }

    if i >= argc {
        die(b"missing command\0".as_ptr() as *const c_char);
    }

    execvp(
        *argv.add(i as usize) as *const c_char,
        argv.add(i as usize) as *const *mut c_char,
    );
    perror(b"execve\0".as_ptr() as *const c_char);

    *__errno_location()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
