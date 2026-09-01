// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020 Matt Helsley <mhelsley@vmware.com>
 * Weak definitions necessary to compile objtool without
 * some subcommands (e.g. check, orc).
 */

use std::os::raw::{c_char, c_int, c_void};

// Dependencies originally provided by:
// <stdbool.h>, <errno.h>, <objtool/objtool.h>, <objtool/arch.h>,
// and <objtool/builtin.h>.

pub const ENOSYS: c_int = 38;

#[repr(C)]
pub struct objtool_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

macro_rules! UNSUPPORTED {
    ($name:literal) => {{
        unsafe {
            fprintf(
                stderr,
                concat!("error: objtool: ", $name, " not implemented\n\0").as_ptr()
                    as *const c_char,
            );
        }
        return ENOSYS;
    }};
}

// C source marks this definition __weak.
#[no_mangle]
pub unsafe extern "C" fn orc_dump(_objname: *const c_char) -> c_int {
    UNSUPPORTED!("ORC");
}

// C source marks this definition __weak.
#[no_mangle]
pub unsafe extern "C" fn orc_create(file: *mut objtool_file) -> c_int {
    let _ = file;
    UNSUPPORTED!("ORC");
}

// C source marks this definition __weak.
#[no_mangle]
pub unsafe extern "C" fn cmd_klp(argc: c_int, argv: *const *const c_char) -> c_int {
    let _ = argc;
    let _ = argv;
    UNSUPPORTED!("klp");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
