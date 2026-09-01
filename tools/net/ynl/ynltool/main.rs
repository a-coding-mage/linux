// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */
/* Copyright Meta Platforms, Inc. and affiliates */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_int, c_uint, c_void, VaList, VaListImpl};

type size_t = usize;
type FILE = c_void;
type json_writer_t = c_void;

const no_argument: c_int = 0;
const SRC_VERSION: *const c_char = b"SRC_VERSION\0".as_ptr() as *const c_char;
const HELP_SPEC_OPTIONS: &str = "OPTIONS := { -j | --json } { -p | --pretty }";

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut opterr: c_int;
    static mut optind: c_int;

    fn exit(status: c_int) -> !;
    fn setlinebuf(stream: *mut FILE);
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaListImpl<'_>) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;

    fn jsonw_destroy(writer: *mut *mut json_writer_t);
    fn jsonw_null(writer: *mut json_writer_t);
    fn jsonw_start_object(writer: *mut json_writer_t);
    fn jsonw_name(writer: *mut json_writer_t, name: *const c_char);
    fn jsonw_printf(writer: *mut json_writer_t, fmt: *const c_char, ...) -> c_int;
    fn jsonw_end_object(writer: *mut json_writer_t);
    fn jsonw_vprintf_enquote(
        writer: *mut json_writer_t,
        fmt: *const c_char,
        ap: VaListImpl<'_>,
    ) -> c_int;
    fn jsonw_new(file: *mut FILE) -> *mut json_writer_t;
    fn jsonw_pretty(writer: *mut json_writer_t, on: bool);

    fn do_page_pool(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_qstats(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

pub static mut bin_name: *const c_char = core::ptr::null();
static mut last_argc: c_int = 0;
static mut last_argv: *mut *mut c_char = core::ptr::null_mut();
static mut last_do_help: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int> = None;
pub static mut json_wtr: *mut json_writer_t = core::ptr::null_mut();
pub static mut pretty_output: bool = false;
pub static mut json_output: bool = false;

unsafe extern "C" fn clean_and_exit(i: c_int) -> ! {
    if json_output {
        jsonw_destroy(core::ptr::addr_of_mut!(json_wtr));
    }

    exit(i);
}

#[no_mangle]
pub unsafe extern "C" fn usage() {
    if let Some(help) = last_do_help {
        help(last_argc - 1, last_argv.add(1));
    }

    clean_and_exit(-1);
}

unsafe extern "C" fn do_help(
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        concat!(
            "Usage: %s [OPTIONS] OBJECT { COMMAND | help }\n",
            "       %s version\n",
            "\n",
            "       OBJECT := { page-pool | qstats }\n",
            "       ",
            "OPTIONS := { -j | --json } { -p | --pretty }",
            "\n",
            "",
            "\0"
        )
        .as_ptr() as *const c_char,
        bin_name,
        bin_name,
    );

    0
}

unsafe extern "C" fn do_version(
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> c_int {
    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_name(json_wtr, b"version\0".as_ptr() as *const c_char);
        jsonw_printf(json_wtr, SRC_VERSION);
        jsonw_end_object(json_wtr);
    } else {
        printf(
            b"%s SRC_VERSION\n\0".as_ptr() as *const c_char,
            bin_name,
        );
    }
    0
}

static commands: [cmd; 5] = [
    cmd {
        cmd: b"help\0".as_ptr() as *const c_char,
        func: Some(do_help),
    },
    cmd {
        cmd: b"page-pool\0".as_ptr() as *const c_char,
        func: Some(do_page_pool),
    },
    cmd {
        cmd: b"qstats\0".as_ptr() as *const c_char,
        func: Some(do_qstats),
    },
    cmd {
        cmd: b"version\0".as_ptr() as *const c_char,
        func: Some(do_version),
    },
    cmd {
        cmd: core::ptr::null(),
        func: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn cmd_select(
    cmds: *const cmd,
    argc: c_int,
    argv: *mut *mut c_char,
    help: unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int,
) -> c_int {
    let mut i: c_uint;

    last_argc = argc;
    last_argv = argv;
    last_do_help = Some(help);

    if argc < 1 && (*cmds.add(0)).func.is_some() {
        return ((*cmds.add(0)).func.unwrap())(argc, argv);
    }

    i = 0;
    while !(*cmds.add(i as usize)).cmd.is_null() {
        if is_prefix(*argv, (*cmds.add(i as usize)).cmd) {
            if (*cmds.add(i as usize)).func.is_none() {
                p_err(
                    b"command '%s' is not available\0".as_ptr() as *const c_char,
                    (*cmds.add(i as usize)).cmd,
                );
                return -1;
            }
            return ((*cmds.add(i as usize)).func.unwrap())(argc - 1, argv.add(1));
        }
        i = i.wrapping_add(1);
    }

    help(argc - 1, argv.add(1));

    -1
}

#[no_mangle]
pub unsafe extern "C" fn is_prefix(pfx: *const c_char, str_: *const c_char) -> bool {
    if pfx.is_null() {
        return false;
    }
    if strlen(str_) < strlen(pfx) {
        return false;
    }

    memcmp(str_ as *const c_void, pfx as *const c_void, strlen(pfx)) == 0
}

/* Last argument MUST be NULL pointer */
#[no_mangle]
pub unsafe extern "C" fn detect_common_prefix(arg: *const c_char, mut ap: ...) -> c_int {
    let mut count: c_uint = 0;
    let mut ref_: *const c_char;
    let mut msg: [c_char; 256] = [0; 256];

    snprintf(
        msg.as_mut_ptr(),
        core::mem::size_of_val(&msg),
        b"ambiguous prefix: '%s' could be '\0".as_ptr() as *const c_char,
        arg,
    );
    loop {
        ref_ = ap.arg::<*const c_char>();
        if ref_.is_null() {
            break;
        }
        if !is_prefix(arg, ref_) {
            continue;
        }
        count = count.wrapping_add(1);
        if count > 1 {
            strncat(
                msg.as_mut_ptr(),
                b"' or '\0".as_ptr() as *const c_char,
                core::mem::size_of_val(&msg) - strlen(msg.as_ptr()) - 1,
            );
        }
        strncat(
            msg.as_mut_ptr(),
            ref_,
            core::mem::size_of_val(&msg) - strlen(msg.as_ptr()) - 1,
        );
    }
    strncat(
        msg.as_mut_ptr(),
        b"'\0".as_ptr() as *const c_char,
        core::mem::size_of_val(&msg) - strlen(msg.as_ptr()) - 1,
    );

    if count >= 2 {
        p_err(b"%s\0".as_ptr() as *const c_char, msg.as_ptr());
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn p_err(fmt: *const c_char, mut ap: ...) {
    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_name(json_wtr, b"error\0".as_ptr() as *const c_char);
        jsonw_vprintf_enquote(json_wtr, fmt, ap.as_va_list());
        jsonw_end_object(json_wtr);
    } else {
        fprintf(stderr, b"Error: \0".as_ptr() as *const c_char);
        vfprintf(stderr, fmt, ap.as_va_list());
        fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
    }
}

#[no_mangle]
pub unsafe extern "C" fn p_info(fmt: *const c_char, mut ap: ...) {
    if json_output {
        return;
    }

    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    static options: [option; 5] = [
        option {
            name: b"json\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: core::ptr::null_mut(),
            val: 'j' as c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: core::ptr::null_mut(),
            val: 'h' as c_int,
        },
        option {
            name: b"pretty\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: core::ptr::null_mut(),
            val: 'p' as c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: core::ptr::null_mut(),
            val: 'V' as c_int,
        },
        option {
            name: core::ptr::null(),
            has_arg: 0,
            flag: core::ptr::null_mut(),
            val: 0,
        },
    ];
    let mut version_requested: bool = false;
    let mut opt: c_int;
    let ret: c_int;

    setlinebuf(stdout);

    last_do_help = Some(do_help);
    pretty_output = false;
    json_output = false;
    bin_name = b"ynltool\0".as_ptr() as *const c_char;

    opterr = 0;
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"Vhjp\0".as_ptr() as *const c_char,
            options.as_ptr(),
            core::ptr::null_mut(),
        );
        if opt < 0 {
            break;
        }
        match opt {
            x if x == 'V' as c_int => {
                version_requested = true;
            }
            x if x == 'h' as c_int => {
                return do_help(argc, argv);
            }
            x if x == 'p' as c_int => {
                pretty_output = true;
                if !json_output {
                    json_wtr = jsonw_new(stdout);
                    if json_wtr.is_null() {
                        p_err(b"failed to create JSON writer\0".as_ptr() as *const c_char);
                        return -1;
                    }
                    json_output = true;
                }
                jsonw_pretty(json_wtr, pretty_output);
            }
            x if x == 'j' as c_int => {
                if !json_output {
                    json_wtr = jsonw_new(stdout);
                    if json_wtr.is_null() {
                        p_err(b"failed to create JSON writer\0".as_ptr() as *const c_char);
                        return -1;
                    }
                    json_output = true;
                }
                jsonw_pretty(json_wtr, pretty_output);
            }
            _ => {
                p_err(
                    b"unrecognized option '%s'\0".as_ptr() as *const c_char,
                    *argv.add((optind - 1) as usize),
                );
                if json_output {
                    clean_and_exit(-1);
                } else {
                    usage();
                }
            }
        }
    }

    argc -= optind;
    argv = argv.add(optind as usize);
    if argc < 0 {
        usage();
    }

    if version_requested {
        ret = do_version(argc, argv);
    } else {
        ret = cmd_select(commands.as_ptr(), argc, argv, do_help);
    }

    if json_output {
        jsonw_destroy(core::ptr::addr_of_mut!(json_wtr));
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
