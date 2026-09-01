// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/kernel.h, subcmd/pager.h, string.h, config.h,
// stdlib.h, stdio.h, color.h, math.h, unistd.h

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn isatty(fd: c_int) -> c_int;

    fn pager_in_use() -> c_int;
    fn perf_config_bool(var: *const c_char, value: *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_config_colorbool(
    var: *const c_char,
    value: *const c_char,
    mut stdout_is_tty: c_int,
) -> c_int {
    if !value.is_null() {
        if unsafe { strcasecmp(value, c"never".as_ptr()) } == 0 {
            return 0;
        }
        if unsafe { strcasecmp(value, c"always".as_ptr()) } == 0 {
            return 1;
        }
        if unsafe { strcasecmp(value, c"auto".as_ptr()) } == 0 {
            // any normal truth value defaults to 'auto'
            if stdout_is_tty < 0 {
                stdout_is_tty = unsafe { isatty(1) };
            }
            if stdout_is_tty != 0 || unsafe { pager_in_use() } != 0 {
                let term = unsafe { getenv(c"TERM".as_ptr()) };
                if !term.is_null() && unsafe { strcmp(term, c"dumb".as_ptr()) } != 0 {
                    return 1;
                }
            }
            return 0;
        }
    }

    /* Missing or explicit false to turn off colorization */
    if unsafe { perf_config_bool(var, value) } == 0 {
        return 0;
    }

    /* any normal truth value defaults to 'auto' */
    if stdout_is_tty < 0 {
        stdout_is_tty = unsafe { isatty(1) };
    }
    if stdout_is_tty != 0 || unsafe { pager_in_use() } != 0 {
        let term = unsafe { getenv(c"TERM".as_ptr()) };
        if !term.is_null() && unsafe { strcmp(term, c"dumb".as_ptr()) } != 0 {
            return 1;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
