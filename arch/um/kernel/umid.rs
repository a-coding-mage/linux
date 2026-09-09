// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int};

// Supplied by the UML OS and kernel initialization code.
unsafe extern "C" {
    fn os_warn(format: *const c_char, ...);
    fn set_umid(name: *mut c_char) -> c_int;
}

// Supplied by <asm/errno.h>.
const EEXIST: c_int = 17;

/* Changed by set_umid_arg */
static mut umid_inited: c_int = 0;

// C: static int __init set_umid_arg(char *name, int *add)
unsafe fn set_umid_arg(name: *mut c_char, add: *mut c_int) -> c_int {
    let err: c_int;

    if umid_inited != 0 {
        os_warn(b"umid already set\n\0".as_ptr() as *const c_char);
        return 0;
    }

    *add = 0;
    err = set_umid(name);
    if err == -EEXIST {
        os_warn(
            b"umid '%s' already in use\n\0".as_ptr() as *const c_char,
            name,
        );
    } else if err == 0 {
        umid_inited = 1;
    }

    0
}

// __uml_setup("umid=", set_umid_arg, ...)
// The registration is performed by the UML initialization framework.
#[allow(dead_code)]
static UMID_SETUP_NAME: &[u8] = b"umid=\0";
#[allow(dead_code)]
static UMID_SETUP_HELP: &[u8] = b"umid=<name>\n    This is used to assign a unique identity to this UML machine and\n    is used for naming the pid file and management console socket.\n\n\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
