// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/prctl.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include <linux/prctl.h>
// #include "trace/beauty/generated/prctl_option_array.c"

use core::ffi::{c_char, c_int, c_ulong};

pub type size_t = usize;
pub type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub mask: u8,
    pub show_string_prefix: bool,
}

extern "C" {
    static strarray__prctl_options: strarray;

    fn strarray__scnprintf(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        fmt: *const c_char,
        show_prefix: bool,
        val: c_int,
    ) -> size_t;

    fn syscall_arg__val(arg: *mut syscall_arg, idx: c_int) -> c_ulong;
    fn syscall_arg__scnprintf_hex(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t;
    fn syscall_arg__scnprintf_long(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t;
}

// From <linux/prctl.h>.
pub const PR_SET_PDEATHSIG: usize = 1;
pub const PR_GET_PDEATHSIG: usize = 2;
pub const PR_GET_DUMPABLE: usize = 3;
pub const PR_SET_DUMPABLE: usize = 4;
pub const PR_SET_NAME: usize = 15;
pub const PR_GET_SECUREBITS: usize = 27;
pub const PR_SET_SECUREBITS: usize = 28;
pub const PR_SET_MM: usize = 35;
pub const PR_SET_CHILD_SUBREAPER: usize = 36;
pub const PR_GET_CHILD_SUBREAPER: usize = 37;

// DEFINE_STRARRAY(prctl_options, "PR_");
// Translated as use of the generated external strarray__prctl_options above.

unsafe fn prctl__scnprintf_option(
    option: c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    strarray__scnprintf(
        &strarray__prctl_options,
        bf,
        size,
        b"%d\0".as_ptr() as *const c_char,
        show_prefix,
        option,
    )
}

unsafe fn prctl__scnprintf_set_mm(
    option: c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // static DEFINE_STRARRAY(prctl_set_mm_options, "PR_SET_MM_");
    extern "C" {
        static strarray__prctl_set_mm_options: strarray;
    }

    strarray__scnprintf(
        &strarray__prctl_set_mm_options,
        bf,
        size,
        b"%d\0".as_ptr() as *const c_char,
        show_prefix,
        option,
    )
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_arg2(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let option: c_int = syscall_arg__val(arg, 0) as c_int;

    if option == PR_SET_MM as c_int {
        return prctl__scnprintf_set_mm((*arg).val as c_int, bf, size, (*arg).show_string_prefix);
    }
    /*
     * We still don't grab the contents of pointers on entry or exit,
     * so just print them as hex numbers
     */
    if option == PR_SET_NAME as c_int {
        return syscall_arg__scnprintf_hex(bf, size, arg);
    }

    syscall_arg__scnprintf_long(bf, size, arg)
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_arg3(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let option: c_int = syscall_arg__val(arg, 0) as c_int;

    if option == PR_SET_MM as c_int {
        return syscall_arg__scnprintf_hex(bf, size, arg);
    }

    syscall_arg__scnprintf_long(bf, size, arg)
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_option(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let option: c_ulong = (*arg).val;
    const SPO_ARG2: u8 = 1 << 1;
    const SPO_ARG3: u8 = 1 << 2;
    const SPO_ARG4: u8 = 1 << 3;
    const SPO_ARG5: u8 = 1 << 4;
    const SPO_ARG6: u8 = 1 << 5;

    let all_but2: u8 = SPO_ARG3 | SPO_ARG4 | SPO_ARG5 | SPO_ARG6;
    let all: u8 = SPO_ARG2 | all_but2;
    let mut masks: [u8; PR_GET_CHILD_SUBREAPER + 1] = [0; PR_GET_CHILD_SUBREAPER + 1];

    masks[PR_GET_DUMPABLE] = all;
    masks[PR_SET_DUMPABLE] = all_but2;
    masks[PR_SET_NAME] = all_but2;
    masks[PR_GET_CHILD_SUBREAPER] = all_but2;
    masks[PR_SET_CHILD_SUBREAPER] = all_but2;
    masks[PR_GET_SECUREBITS] = all;
    masks[PR_SET_SECUREBITS] = all_but2;
    masks[PR_SET_MM] = SPO_ARG4 | SPO_ARG5 | SPO_ARG6;
    masks[PR_GET_PDEATHSIG] = all;
    masks[PR_SET_PDEATHSIG] = all_but2;

    if (option as usize) < masks.len() {
        (*arg).mask |= masks[option as usize];
    }

    prctl__scnprintf_option(option as c_int, bf, size, (*arg).show_string_prefix)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
