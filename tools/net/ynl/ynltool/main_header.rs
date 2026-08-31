/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */
/* Copyright Meta Platforms, Inc. and affiliates */

/*
 * C header guard and C-only includes omitted.
 * The original header included <stdbool.h>, <stdio.h>, <stdlib.h>, <errno.h>,
 * <string.h>, and "json_writer.h".
 */

use std::ffi::{c_char, c_int, c_void};

/* Opaque type supplied by json_writer.h in the original C source. */
pub type json_writer_t = c_void;

macro_rules! NEXT_ARG {
    ($argc:ident, $argv:ident) => {{
        $argc -= 1;
        $argv = unsafe { $argv.add(1) };
        if $argc < 0 {
            unsafe { usage() };
        }
    }};
}

macro_rules! NEXT_ARGP {
    ($argc:expr, $argv:expr) => {{
        unsafe {
            *$argc -= 1;
            *$argv = (*$argv).add(1);
            if *$argc < 0 {
                usage();
            }
        }
    }};
}

macro_rules! BAD_ARG {
    ($argv:ident) => {{
        unsafe {
            p_err(c"what is '%s'?".as_ptr(), *$argv);
        }
        -1
    }};
}

macro_rules! GET_ARG {
    ($argc:ident, $argv:ident) => {{
        $argc -= 1;
        let arg = unsafe { *$argv };
        $argv = unsafe { $argv.add(1) };
        arg
    }};
}

macro_rules! REQ_ARGS {
    ($argc:ident, $argv:ident, $cnt:expr) => {{
        let _cnt: c_int = $cnt;
        let _res: bool;

        if $argc < _cnt {
            unsafe {
                p_err(
                    c"'%s' needs at least %d arguments, %d found".as_ptr(),
                    *$argv.offset(-1),
                    _cnt,
                    $argc,
                );
            }
            _res = false;
        } else {
            _res = true;
        }
        _res
    }};
}

pub(crate) use BAD_ARG;
pub(crate) use GET_ARG;
pub(crate) use NEXT_ARG;
pub(crate) use NEXT_ARGP;
pub(crate) use REQ_ARGS;

pub const HELP_SPEC_OPTIONS: &str = "OPTIONS := { {-j|--json} [{-p|--pretty}] }";

unsafe extern "C" {
    pub static bin_name: *const c_char;

    pub static mut json_wtr: *mut json_writer_t;
    pub static mut json_output: bool;
    pub static mut pretty_output: bool;

    pub fn p_err(fmt: *const c_char, ...);
    pub fn p_info(fmt: *const c_char, ...);

    pub fn is_prefix(pfx: *const c_char, str: *const c_char) -> bool;
    pub fn detect_common_prefix(arg: *const c_char, ...) -> c_int;
    pub fn usage() -> !;
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    pub fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int>,
    ) -> c_int;

    /* subcommands */
    pub fn do_page_pool(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn do_qstats(argc: c_int, argv: *mut *mut c_char) -> c_int;
}
