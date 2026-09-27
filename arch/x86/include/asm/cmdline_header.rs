/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header guard and <asm/setup.h> dependency.

use kernel::ffi::c_char;

extern "C" {
    pub static mut builtin_cmdline: [c_char; COMMAND_LINE_SIZE];

    pub fn cmdline_find_option_bool(
        cmdline_ptr: *const c_char,
        option: *const c_char,
    ) -> kernel::ffi::c_int;

    pub fn cmdline_find_option(
        cmdline_ptr: *const c_char,
        option: *const c_char,
        buffer: *mut c_char,
        bufsize: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
