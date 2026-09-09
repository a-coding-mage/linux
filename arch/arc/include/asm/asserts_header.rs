/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Synopsys, Inc. (www.synopsys.com)
 *
 * Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>
 */

use core::ffi::c_char;

/* Helpers to sanitize config options. */

extern "C" {
    pub fn chk_opt_strict(opt_name: *mut c_char, hw_exists: bool, opt_ena: bool);
    pub fn chk_opt_weak(opt_name: *mut c_char, hw_exists: bool, opt_ena: bool);
}

/*
 * Check required config option:
 *  - panic in case of OPT enabled but corresponding HW absent.
 *  - warn in case of OPT disabled but corresponding HW exists.
 *
 * `IS_ENABLED` is supplied by the configuration environment.
 */
macro_rules! CHK_OPT_STRICT {
    ($opt_name:ident, $hw_exists:expr) => {{
        unsafe {
            chk_opt_strict(
                stringify!($opt_name).as_ptr() as *mut c_char,
                $hw_exists,
                IS_ENABLED!($opt_name),
            );
        }
    }};
}

/*
 * Check optional config option:
 *  - panic in case of OPT enabled but corresponding HW absent.
 *
 * `IS_ENABLED` is supplied by the configuration environment.
 */
macro_rules! CHK_OPT_WEAK {
    ($opt_name:ident, $hw_exists:expr) => {{
        unsafe {
            chk_opt_weak(
                stringify!($opt_name).as_ptr() as *mut c_char,
                $hw_exists,
                IS_ENABLED!($opt_name),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
