// SPDX-License-Identifier: GPL-2.0
/*
 * cmdline.c: read the command line passed to us by the PROM.
 *
 * Copyright (C) 1998 Harald Koerfgen
 * Copyright (C) 2002, 2004  Maciej W. Rozycki
 */

use core::ffi::c_char;

// Supplied by the corresponding PROM and kernel support modules.
extern "C" {
    fn prom_is_rex(magic: u32) -> i32;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    static mut arcs_cmdline: *mut c_char;
}

// PROM_DEBUG is undefined in the source translation unit.

pub unsafe fn prom_init_cmdline(argc: i32, argv: *mut i32, magic: u32) {
    let mut arg: *mut c_char;
    let start_arg: i32;
    let mut i: i32;

    /*
     * collect args and prepare cmd_line
     */
    if prom_is_rex(magic) == 0 {
        start_arg = 1;
    } else {
        start_arg = 2;
    }
    i = start_arg;
    while i < argc {
        arg = (argv.add(i as usize).read() as isize) as *mut c_char;
        strcat(arcs_cmdline, arg);
        if i < argc - 1 {
            strcat(arcs_cmdline, b" \0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    // PROM_DEBUG logging is disabled, matching the source file's #undef.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
