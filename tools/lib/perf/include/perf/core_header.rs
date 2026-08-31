/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from lib/perf/include/perf/core.h. */
/* C dependency intent: #include <stdarg.h> for va_list. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libperf_print_level {
    LIBPERF_ERR = 0,
    LIBPERF_WARN = 1,
    LIBPERF_INFO = 2,
    LIBPERF_DEBUG = 3,
    LIBPERF_DEBUG2 = 4,
    LIBPERF_DEBUG3 = 5,
}

pub type va_list = *mut core::ffi::c_void;

pub type libperf_print_fn_t = Option<
    unsafe extern "C" fn(
        level: libperf_print_level,
        arg1: *const core::ffi::c_char,
        ap: va_list,
    ) -> core::ffi::c_int,
>;

unsafe extern "C" {
    /* LIBPERF_API: extern symbol with default visibility in C. */
    pub fn libperf_init(fn_: libperf_print_fn_t);
}
