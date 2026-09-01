// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/arch_prctl.c
 *
 *  Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_ulong};

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include "trace/beauty/generated/x86_arch_prctl_code_array.c"

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strarrays {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
}

unsafe extern "C" {
    // static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_1, "ARCH_", x86_arch_prctl_codes_1_offset);
    static mut strarray__x86_arch_prctl_codes_1: strarray;

    // static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_2, "ARCH_", x86_arch_prctl_codes_2_offset);
    static mut strarray__x86_arch_prctl_codes_2: strarray;

    // static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_3, "ARCH_", x86_arch_prctl_codes_3_offset);
    static mut strarray__x86_arch_prctl_codes_3: strarray;

    // static DEFINE_STRARRAYS(x86_arch_prctl_codes);
    static mut strarrays__x86_arch_prctl_codes: strarrays;

    fn strarrays__scnprintf(
        strarrays: *mut strarrays,
        bf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        show_prefix: bool,
        value: c_int,
    ) -> usize;
}

#[used]
static mut x86_arch_prctl_codes: [*mut strarray; 3] = [
    &raw mut strarray__x86_arch_prctl_codes_1,
    &raw mut strarray__x86_arch_prctl_codes_2,
    &raw mut strarray__x86_arch_prctl_codes_3,
];

unsafe fn x86_arch_prctl__scnprintf_code(
    option: c_int,
    bf: *mut c_char,
    size: usize,
    show_prefix: bool,
) -> usize {
    unsafe {
        strarrays__scnprintf(
            &raw mut strarrays__x86_arch_prctl_codes,
            bf,
            size,
            c"%#x".as_ptr(),
            show_prefix,
            option,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_x86_arch_prctl_code(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let code: c_ulong = unsafe { (*arg).val };

    unsafe {
        x86_arch_prctl__scnprintf_code(
            code as c_int,
            bf,
            size,
            (*arg).show_string_prefix,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
