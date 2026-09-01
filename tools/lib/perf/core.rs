// SPDX-License-Identifier: GPL-2.0-only

// C dependencies in the original source:
// #include <stdio.h>
// #include <stdarg.h>
// #include <unistd.h>
// #include <linux/compiler.h>
// #include <perf/core.h>
// #include <internal/lib.h>
// #include "internal.h"

use core::ffi::{c_char, c_int, c_long, VaListImpl};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// Supplied by external headers in the original C translation unit.
pub type libperf_print_level = c_int;
pub type libperf_print_fn_t = Option<
    unsafe extern "C" fn(
        level: libperf_print_level,
        format: *const c_char,
        args: VaListImpl,
    ) -> c_int,
>;

extern "C" {
    static mut stderr: *mut FILE;
    static mut page_size: c_long;

    fn vfprintf(stream: *mut FILE, format: *const c_char, args: VaListImpl) -> c_int;
    fn sysconf(name: c_int) -> c_long;
}

pub const _SC_PAGE_SIZE: c_int = 30;

unsafe extern "C" fn __base_pr(
    level: libperf_print_level,
    format: *const c_char,
    args: VaListImpl,
) -> c_int {
    let _ = level;
    unsafe { vfprintf(stderr, format, args) }
}

static mut __libperf_pr: libperf_print_fn_t = Some(__base_pr);

// Original C declaration is annotated with __printf(2, 3).
pub unsafe extern "C" fn libperf_print(
    level: libperf_print_level,
    format: *const c_char,
    mut args: ...
) {
    unsafe {
        if __libperf_pr.is_none() {
            return;
        }

        if let Some(pr) = __libperf_pr {
            pr(level, format, args.as_va_list());
        }
    }
}

pub unsafe extern "C" fn libperf_init(fn_: libperf_print_fn_t) {
    unsafe {
        page_size = sysconf(_SC_PAGE_SIZE);
        __libperf_pr = fn_;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
