/* SPDX-License-Identifier: GPL-2.0-only */

use std::os::raw::{c_char, c_ulong};

unsafe extern "C" {
    pub fn control_init(
        control_host: *const c_char,
        control_port: *const c_char,
        server: bool,
    );
    pub fn control_cleanup();
    pub fn control_writeln(str: *const c_char);
    pub fn control_readln() -> *mut c_char;
    pub fn control_readulong() -> c_ulong;
    pub fn control_expectln(str: *const c_char);
    pub fn control_cmpln(line: *mut c_char, str: *const c_char, fail: bool) -> bool;
    pub fn control_writeulong(value: c_ulong);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
