// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/builtin-data.c.
// C include dependencies:
// linux/compiler.h, stdio.h, string.h, builtin.h, debug.h,
// subcmd/parse-options.h, data-convert.h, util/util.h

use core::ffi::{c_char, c_int, c_void};

type data_cmd_fn_t = unsafe extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int;

#[repr(C)]
struct data_cmd {
    name: *const c_char,
    summary: *const c_char,
    fn_: Option<data_cmd_fn_t>,
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_data_convert_opts {
    force: bool,
    all: bool,
    time_str: *const c_char,
    tod: bool,
}

const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;

unsafe extern "C" {
    static mut verbose: c_int;
    static mut input_name: *const c_char;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn parse_options_subcommand(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        subcommands: *const *const c_char,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn bt_convert__perf2json(
        input_name: *const c_char,
        to_json: *const c_char,
        opts: *mut perf_data_convert_opts,
    ) -> c_int;

    // Available only when the C build has both
    // HAVE_BABELTRACE2_CTF_WRITER_SUPPORT and HAVE_LIBTRACEEVENT.
    fn bt_convert__perf2ctf(
        input_name: *const c_char,
        to_ctf: *const c_char,
        opts: *mut perf_data_convert_opts,
    ) -> c_int;
}

static DATA_SUBCOMMAND_CONVERT: &[u8] = b"convert\0";
static DATA_USAGE_CONVERT: &[u8] = b"perf data convert [<options>]\0";

static DATA_CMD_CONVERT_SUMMARY: &[u8] = b"converts data file between formats\0";

static ERR_BOTH_OUTPUTS: &[u8] = b"You cannot specify both --to-ctf and --to-json.\n\0";
static ERR_NO_OUTPUT: &[u8] = b"You must specify one of --to-ctf or --to-json.\n\0";
static ERR_NO_CTF_SUPPORT: &[u8] = b"The babeltrace2 ctf support is not compiled in. Ensure you have both\nlibbabeltrace2-dev[el] and libtraceevent-dev[el] installed or set\nPKG_CONFIG_PATH to find a local installation of those libraries.\n\0";
static ERR_UNKNOWN_COMMAND: &[u8] = b"Unknown command: %s\n\0";

static DATA_SUBCOMMANDS: [*const c_char; 2] = [
    DATA_SUBCOMMAND_CONVERT.as_ptr() as *const c_char,
    core::ptr::null(),
];

static DATA_USAGE: [*const c_char; 2] = [
    DATA_USAGE_CONVERT.as_ptr() as *const c_char,
    core::ptr::null(),
];

static mut TO_JSON: *const c_char = core::ptr::null();
static mut TO_CTF: *const c_char = core::ptr::null();
static mut OPTS: perf_data_convert_opts = perf_data_convert_opts {
    force: false,
    all: false,
    time_str: core::ptr::null(),
    tod: false,
};

// Original C option table:
// static const struct option data_options[] = {
//      OPT_INCR('v', "verbose", &verbose, "be more verbose"),
//      OPT_STRING('i', "input", &input_name, "file", "input file name"),
//      OPT_STRING(0, "to-json", &to_json, NULL, "Convert to JSON format"),
//      OPT_STRING(0, "to-ctf", &to_ctf, NULL, "Convert to CTF format"),
//      OPT_BOOLEAN(0, "tod", &opts.tod, "Convert time to wall clock time"),
//      OPT_BOOLEAN('f', "force", &opts.force, "don't complain, do it"),
//      OPT_BOOLEAN(0, "all", &opts.all, "Convert all events"),
//      OPT_STRING(0, "time", &opts.time_str, "str",
//                 "Time span of interest (start,stop)"),
//      OPT_END()
// };
// The OPT_* macro expansions and struct option layout are supplied by
// subcmd/parse-options.h and are not defined in this isolated source file.
unsafe extern "C" {
    static data_options: [option; 0];
}

unsafe extern "C" fn cmd_data_convert(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    argc = parse_options(
        argc,
        argv,
        data_options.as_ptr(),
        DATA_USAGE.as_ptr(),
        0,
    );
    if argc != 0 {
        usage_with_options(DATA_USAGE.as_ptr(), data_options.as_ptr());
        return -1;
    }

    if !TO_JSON.is_null() && !TO_CTF.is_null() {
        pr_err(ERR_BOTH_OUTPUTS.as_ptr() as *const c_char);
        return -1;
    }
    if TO_JSON.is_null() && TO_CTF.is_null() {
        pr_err(ERR_NO_OUTPUT.as_ptr() as *const c_char);
        return -1;
    }

    if !TO_JSON.is_null() {
        return bt_convert__perf2json(input_name, TO_JSON, &raw mut OPTS);
    }

    if !TO_CTF.is_null() {
        // C conditional:
        // #if defined(HAVE_BABELTRACE2_CTF_WRITER_SUPPORT) && defined(HAVE_LIBTRACEEVENT)
        #[cfg(all(
            HAVE_BABELTRACE2_CTF_WRITER_SUPPORT,
            HAVE_LIBTRACEEVENT
        ))]
        {
            return bt_convert__perf2ctf(input_name, TO_CTF, &raw mut OPTS);
        }

        // #else
        #[cfg(not(all(
            HAVE_BABELTRACE2_CTF_WRITER_SUPPORT,
            HAVE_LIBTRACEEVENT
        )))]
        {
            pr_err(ERR_NO_CTF_SUPPORT.as_ptr() as *const c_char);
            return -1;
        }
    }

    0
}

static mut DATA_CMDS: [data_cmd; 2] = [
    data_cmd {
        name: DATA_SUBCOMMAND_CONVERT.as_ptr() as *const c_char,
        summary: DATA_CMD_CONVERT_SUMMARY.as_ptr() as *const c_char,
        fn_: Some(cmd_data_convert),
    },
    data_cmd {
        name: core::ptr::null(),
        summary: core::ptr::null(),
        fn_: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn cmd_data(
    mut argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut cmd: *mut data_cmd;
    let cmdstr: *const c_char;

    argc = parse_options_subcommand(
        argc,
        argv,
        data_options.as_ptr(),
        DATA_SUBCOMMANDS.as_ptr(),
        DATA_USAGE.as_ptr(),
        PARSE_OPT_STOP_AT_NON_OPTION,
    );

    if argc == 0 {
        usage_with_options(DATA_USAGE.as_ptr(), data_options.as_ptr());
        return -1;
    }

    cmdstr = *argv.add(0);

    // for_each_cmd(cmd)
    cmd = DATA_CMDS.as_mut_ptr();
    while !cmd.is_null() && !(*cmd).name.is_null() {
        if strcmp((*cmd).name, cmdstr) != 0 {
            cmd = cmd.add(1);
            continue;
        }

        return ((*cmd).fn_.unwrap())(argc, argv);
    }

    pr_err(ERR_UNKNOWN_COMMAND.as_ptr() as *const c_char, cmdstr);
    usage_with_options(DATA_USAGE.as_ptr(), data_options.as_ptr());
    -1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
