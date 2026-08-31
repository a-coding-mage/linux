// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015-2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/*
 * Translated from objtool/builtin-check.c.
 *
 * C include dependencies:
 * <subcmd/parse-options.h>, <string.h>, <stdlib.h>, <fcntl.h>, <unistd.h>,
 * <errno.h>, <sys/stat.h>, <sys/sendfile.h>, <objtool/builtin.h>,
 * <objtool/objtool.h>, <objtool/warn.h>
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

const ORIG_SUFFIX: &[u8] = b".orig\0";
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;

type size_t = usize;
type ssize_t = isize;
type off_t = c_long;
type mode_t = c_uint;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf {
    pub changed: bool,
}

#[repr(C)]
pub struct objtool_file {
    pub elf: *mut elf,
}

#[repr(C)]
pub struct stat {
    pub st_dev: c_long,
    pub st_ino: c_long,
    pub st_nlink: c_long,
    pub st_mode: mode_t,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_long,
    pub st_size: off_t,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub __glibc_reserved: [c_long; 3],
}

#[repr(C)]
pub struct opts {
    pub disas: *const c_char,
    pub hack_jump_label: bool,
    pub hack_noinstr: bool,
    pub hack_skylake: bool,
    pub ibt: bool,
    pub klp_symids: bool,
    pub mcount: bool,
    pub noabs: bool,
    pub noinstr: bool,
    pub orc: bool,
    pub retpoline: bool,
    pub rethunk: bool,
    pub unret: bool,
    pub prefix: c_int,
    pub sls: bool,
    pub stackval: bool,
    pub static_call: bool,
    pub uaccess: bool,
    pub dump_orc: bool,
    pub cfi: bool,
    pub fineibt: bool,
    pub backtrace: bool,
    pub backup: bool,
    pub dryrun: bool,
    pub link: bool,
    pub module: bool,
    pub mnop: bool,
    pub no_unreachable: bool,
    pub output: *const c_char,
    pub sec_address: bool,
    pub stats: bool,
    pub trace: *const c_char,
    pub verbose: bool,
    pub werror: bool,
    pub wide: bool,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn getenv(name: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option) -> !;

    fn orc_dump(objname: *const c_char) -> c_int;
    fn objtool_open_read(objname: *const c_char) -> *mut objtool_file;
    fn has_multiple_files(elf: *mut elf) -> bool;
    fn check(file: *mut objtool_file) -> c_int;
    fn elf_write(elf: *mut elf) -> c_int;
    fn elf_close(elf: *mut elf) -> c_int;

    fn ERROR(format: *const c_char, ...);
    fn ERROR_GLIBC(format: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub static mut orig_argc: c_int = 0;
static mut orig_argv: *mut *mut c_char = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut objname: *const c_char = ptr::null();
#[unsafe(no_mangle)]
pub static mut opts: opts = opts {
    disas: ptr::null(),
    hack_jump_label: false,
    hack_noinstr: false,
    hack_skylake: false,
    ibt: false,
    klp_symids: false,
    mcount: false,
    noabs: false,
    noinstr: false,
    orc: false,
    retpoline: false,
    rethunk: false,
    unret: false,
    prefix: 0,
    sls: false,
    stackval: false,
    static_call: false,
    uaccess: false,
    dump_orc: false,
    cfi: false,
    fineibt: false,
    backtrace: false,
    backup: false,
    dryrun: false,
    link: false,
    module: false,
    mnop: false,
    no_unreachable: false,
    output: ptr::null(),
    sec_address: false,
    stats: false,
    trace: ptr::null(),
    verbose: false,
    werror: false,
    wide: false,
};

static CHECK_USAGE_0: &[u8] = b"objtool <actions> [<options>] file.o\0";
static mut check_usage: [*const c_char; 2] = [CHECK_USAGE_0.as_ptr() as *const c_char, ptr::null()];

static ENV_USAGE_0: &[u8] = b"OBJTOOL_ARGS=\"<options>\"\0";
static mut env_usage: [*const c_char; 2] = [ENV_USAGE_0.as_ptr() as *const c_char, ptr::null()];

unsafe fn c_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe extern "C" fn parse_dump(
    _opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    if str_.is_null() || strcmp(str_, c_lit(b"orc\0")) == 0 {
        opts.dump_orc = true;
        return 0;
    }

    -1
}

unsafe extern "C" fn parse_hacks(
    _opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let mut found = false;

    /*
     * Use strstr() as a lazy method of checking for comma-separated
     * options.
     *
     * No string provided == enable all options.
     */

    if str_.is_null() || !strstr(str_, c_lit(b"jump_label\0")).is_null() {
        opts.hack_jump_label = true;
        found = true;
    }

    if str_.is_null() || !strstr(str_, c_lit(b"noinstr\0")).is_null() {
        opts.hack_noinstr = true;
        found = true;
    }

    if str_.is_null() || !strstr(str_, c_lit(b"skylake\0")).is_null() {
        opts.hack_skylake = true;
        found = true;
    }

    if found { 0 } else { -1 }
}

/*
 * static const struct option check_options[] = {
 *     OPT_GROUP("Actions:"),
 *     OPT_STRING_OPTARG('d', "disas", &opts.disas, "function-pattern", "disassemble functions", "*"),
 *     OPT_CALLBACK_OPTARG('h', "hacks", NULL, NULL, "jump_label,noinstr,skylake", "patch toolchain bugs/limitations", parse_hacks),
 *     OPT_BOOLEAN('i', "ibt", &opts.ibt, "validate and annotate IBT"),
 *     OPT_BOOLEAN(0, "klp-symids", &opts.klp_symids, "generate .klp.symids for duplicate symbol disambiguation"),
 *     OPT_BOOLEAN('m', "mcount", &opts.mcount, "annotate mcount/fentry calls for ftrace"),
 *     OPT_BOOLEAN(0, "noabs", &opts.noabs, "reject absolute references in allocatable sections"),
 *     OPT_BOOLEAN('n', "noinstr", &opts.noinstr, "validate noinstr rules"),
 *     OPT_BOOLEAN(0, "orc", &opts.orc, "generate ORC metadata"),
 *     OPT_BOOLEAN('r', "retpoline", &opts.retpoline, "validate and annotate retpoline usage"),
 *     OPT_BOOLEAN(0, "rethunk", &opts.rethunk, "validate and annotate rethunk usage"),
 *     OPT_BOOLEAN(0, "unret", &opts.unret, "validate entry unret placement"),
 *     OPT_INTEGER(0, "prefix", &opts.prefix, "generate or grow prefix symbols for N-byte function padding"),
 *     OPT_BOOLEAN('l', "sls", &opts.sls, "validate straight-line-speculation mitigations"),
 *     OPT_BOOLEAN('s', "stackval", &opts.stackval, "validate frame pointer rules"),
 *     OPT_BOOLEAN('t', "static-call", &opts.static_call, "annotate static calls"),
 *     OPT_BOOLEAN('u', "uaccess", &opts.uaccess, "validate uaccess rules for SMAP"),
 *     OPT_CALLBACK_OPTARG(0, "dump", NULL, NULL, "orc", "dump metadata", parse_dump),
 *
 *     OPT_GROUP("Options:"),
 *     OPT_BOOLEAN(0, "cfi", &opts.cfi, "grow kCFI preamble symbols (use with --prefix)"),
 *     OPT_BOOLEAN(0, "fineibt", &opts.fineibt, "create .cfi_sites section for FineIBT"),
 *     OPT_BOOLEAN(0, "backtrace", &opts.backtrace, "unwind on error"),
 *     OPT_BOOLEAN(0, "backup", &opts.backup, "create backup (.orig) file on warning/error"),
 *     OPT_BOOLEAN(0, "dry-run", &opts.dryrun, "don't write modifications"),
 *     OPT_BOOLEAN(0, "link", &opts.link, "object is a linked object"),
 *     OPT_BOOLEAN(0, "module", &opts.module, "object is part of a kernel module"),
 *     OPT_BOOLEAN(0, "mnop", &opts.mnop, "nop out mcount call sites"),
 *     OPT_BOOLEAN(0, "no-unreachable", &opts.no_unreachable, "skip 'unreachable instruction' warnings"),
 *     OPT_STRING('o', "output", &opts.output, "file", "output file name"),
 *     OPT_BOOLEAN(0, "sec-address", &opts.sec_address, "print section addresses in warnings"),
 *     OPT_BOOLEAN(0, "stats", &opts.stats, "print statistics"),
 *     OPT_STRING(0, "trace", &opts.trace, "func", "trace function validation"),
 *     OPT_BOOLEAN('v', "verbose", &opts.verbose, "verbose warnings"),
 *     OPT_BOOLEAN(0, "werror", &opts.werror, "return error on warnings"),
 *     OPT_BOOLEAN(0, "wide", &opts.wide, "wide output"),
 *
 *     OPT_END(),
 * };
 *
 * The actual `struct option` layout and OPT_* initializers are supplied by
 * <subcmd/parse-options.h>; keep this symbol as the Rust reference point for
 * calls into parse_options().
 */
unsafe extern "C" {
    static check_options: [option; 0];
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_parse_options(
    mut argc: c_int,
    argv: *const *const c_char,
    usage: *const *const c_char,
) -> c_int {
    let mut envv: [*const c_char; 16] = [ptr::null(); 16];
    let mut env: *mut c_char;
    let mut envc: c_int;

    env = getenv(c_lit(b"OBJTOOL_ARGS\0"));
    if !env.is_null() {
        envv[0] = c_lit(b"OBJTOOL_ARGS\0");
        envc = 1;
        while (envc as usize) < envv.len() {
            envv[envc as usize] = env;
            envc += 1;
            env = strchr(env, b' ' as c_int);
            if env.is_null() {
                break;
            }
            *env = b'\0' as c_char;
            env = env.add(1);
        }

        parse_options(envc, envv.as_ptr(), check_options.as_ptr(), env_usage.as_ptr(), 0);
    }

    env = getenv(c_lit(b"OBJTOOL_VERBOSE\0"));
    if !env.is_null() && strcmp(env, c_lit(b"1\0")) == 0 {
        opts.verbose = true;
    }

    argc = parse_options(argc, argv, check_options.as_ptr(), usage, 0);
    if argc != 1 {
        usage_with_options(usage, check_options.as_ptr());
    }
    argc
}

unsafe fn opts_valid() -> bool {
    if opts.mnop && !opts.mcount {
        ERROR(c_lit(b"--mnop requires --mcount\0"));
        return false;
    }

    if opts.noinstr && !opts.link {
        ERROR(c_lit(b"--noinstr requires --link\0"));
        return false;
    }

    if opts.ibt && !opts.link {
        ERROR(c_lit(b"--ibt requires --link\0"));
        return false;
    }

    if opts.unret && !opts.link {
        ERROR(c_lit(b"--unret requires --link\0"));
        return false;
    }

    if opts.cfi && opts.prefix == 0 {
        ERROR(c_lit(b"--cfi requires --prefix\0"));
        return false;
    }

    if opts.fineibt && !opts.cfi {
        ERROR(c_lit(b"--fineibt requires --cfi\0"));
        return false;
    }

    if opts.klp_symids && !opts.link {
        ERROR(c_lit(b"--klp-symids requires --link\0"));
        return false;
    }

    if !opts.disas.is_null()
        || opts.hack_jump_label
        || opts.hack_noinstr
        || opts.ibt
        || opts.klp_symids
        || opts.mcount
        || opts.noabs
        || opts.noinstr
        || opts.orc
        || opts.retpoline
        || opts.rethunk
        || opts.sls
        || opts.stackval
        || opts.static_call
        || opts.uaccess
    {
        if opts.dump_orc {
            ERROR(c_lit(b"--dump can't be combined with other actions\0"));
            return false;
        }

        return true;
    }

    if opts.dump_orc {
        return true;
    }

    ERROR(c_lit(b"At least one action required\0"));
    false
}

unsafe fn copy_file(src: *const c_char, dst: *const c_char) -> c_int {
    let mut to_copy: size_t;
    let mut copied: ssize_t;
    let dst_fd: c_int;
    let src_fd: c_int;
    let mut statbuf: stat = std::mem::zeroed();
    let mut offset: off_t = 0;

    src_fd = open(src, O_RDONLY);
    if src_fd == -1 {
        ERROR(
            c_lit(b"can't open %s for reading: %s\0"),
            src,
            strerror(errno),
        );
        return 1;
    }

    dst_fd = open(dst, O_WRONLY | O_CREAT | O_TRUNC, 0o400 as c_int);
    if dst_fd == -1 {
        ERROR(
            c_lit(b"can't open %s for writing: %s\0"),
            dst,
            strerror(errno),
        );
        return 1;
    }

    if fstat(src_fd, &mut statbuf) == -1 {
        ERROR_GLIBC(c_lit(b"fstat\0"));
        return 1;
    }

    if fchmod(dst_fd, statbuf.st_mode) == -1 {
        ERROR_GLIBC(c_lit(b"fchmod\0"));
        return 1;
    }

    to_copy = statbuf.st_size as size_t;
    while to_copy > 0 {
        copied = sendfile(dst_fd, src_fd, &mut offset, to_copy);
        if copied == -1 {
            ERROR_GLIBC(c_lit(b"sendfile\0"));
            return 1;
        }
        to_copy = to_copy.wrapping_sub(copied as size_t);
    }

    close(dst_fd);
    close(src_fd);
    0
}

unsafe fn save_argv(argc: c_int, argv: *const *const c_char) {
    orig_argv = calloc(argc as size_t, std::mem::size_of::<*mut c_char>() as size_t) as *mut *mut c_char;
    if orig_argv.is_null() {
        ERROR_GLIBC(c_lit(b"calloc\0"));
        exit(1);
    }

    for i in 0..argc {
        *orig_argv.add(i as usize) = strdup(*argv.add(i as usize));
        if (*orig_argv.add(i as usize)).is_null() {
            ERROR_GLIBC(c_lit(b"strdup(%s)\0"), *argv.add(i as usize));
            exit(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_backup() -> c_int {
    let backup: *mut c_char;

    /*
     * Make a backup before kbuild deletes the file so the error
     * can be recreated without recompiling or relinking.
     */
    backup = malloc(strlen(objname) + strlen(c_lit(ORIG_SUFFIX)) + 1) as *mut c_char;
    if backup.is_null() {
        ERROR_GLIBC(c_lit(b"malloc\0"));
        return 1;
    }

    strcpy(backup, objname);
    strcat(backup, c_lit(ORIG_SUFFIX));
    if copy_file(objname, backup) != 0 {
        return 1;
    }

    /*
     * Print the cmdline args to make it easier to recreate.
     */

    fprintf(stderr, c_lit(b"%s\0"), *orig_argv.add(0));

    for i in 1..orig_argc {
        let arg = *orig_argv.add(i as usize);

        /* Modify the printed args to use the backup */
        if opts.output.is_null() && strcmp(arg, objname) == 0 {
            fprintf(stderr, c_lit(b" %s -o %s\0"), backup, objname);
        } else {
            fprintf(stderr, c_lit(b" %s\0"), arg);
        }
    }

    fprintf(stderr, c_lit(b"\n\0"));
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn objtool_run(argc: c_int, argv: *const *const c_char) -> c_int {
    let file: *mut objtool_file;
    let mut ret: c_int = 0;

    orig_argc = argc;
    save_argv(argc, argv);

    cmd_parse_options(argc, argv, check_usage.as_ptr());

    if !opts_valid() {
        return 1;
    }

    objname = *argv.add(0);

    if opts.dump_orc {
        return orc_dump(objname);
    }

    if !opts.dryrun && !opts.output.is_null() {
        /* copy original .o file to output file */
        if copy_file(objname, opts.output) != 0 {
            return 1;
        }

        /* from here on, work directly on the output file */
        objname = opts.output;
    }

    file = objtool_open_read(objname);
    if file.is_null() {
        return 1;
    }

    if !opts.link && has_multiple_files((*file).elf) {
        ERROR(c_lit(b"Linked object requires --link\0"));
        return 1;
    }

    ret = check(file);
    if ret != 0 {
        return ret;
    }

    if !opts.dryrun && (*(*file).elf).changed && elf_write((*file).elf) != 0 {
        return 1;
    }

    elf_close((*file).elf)
}
