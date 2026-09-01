// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use core::mem;
use core::ptr;

const BATCH_LINE_LEN_MAX: usize = 65536;
const BATCH_ARG_NB_MAX: usize = 4096;

const no_argument: c_int = 0;
const required_argument: c_int = 1;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

/* bpftool's major and minor version numbers are aligned on libbpf's. There is
 * an offset of 6 for the version number, because bpftool's version was higher
 * than libbpf's when we adopted this scheme. The patch number remains at 0
 * for now. Set BPFTOOL_VERSION to override.
 */
const BPFTOOL_PATCH_VERSION: c_uint = 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
pub struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut stdin: *mut FILE;
    static mut errno: c_int;
    static mut optind: c_int;
    static mut opterr: c_int;
    static mut optarg: *mut c_char;

    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strspn(s: *const c_char, accept: *const c_char) -> usize;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncat(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn setlinebuf(stream: *mut FILE);
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn jsonw_new(f: *mut FILE) -> *mut json_writer_t;
    fn jsonw_destroy(writer: *mut *mut json_writer_t);
    fn jsonw_null(writer: *mut json_writer_t);
    fn jsonw_pretty(writer: *mut json_writer_t, pretty: bool);
    fn jsonw_start_object(writer: *mut json_writer_t);
    fn jsonw_end_object(writer: *mut json_writer_t);
    fn jsonw_start_array(writer: *mut json_writer_t);
    fn jsonw_end_array(writer: *mut json_writer_t);
    fn jsonw_name(writer: *mut json_writer_t, name: *const c_char);
    fn jsonw_printf(writer: *mut json_writer_t, format: *const c_char, ...);
    fn jsonw_bool_field(writer: *mut json_writer_t, name: *const c_char, value: bool);
    fn jsonw_string(writer: *mut json_writer_t, value: *const c_char);

    fn libbpf_major_version() -> c_uint;
    fn libbpf_minor_version() -> c_uint;
    fn libbpf_version_string() -> *const c_char;
    fn libbpf_set_print(
        print: Option<unsafe extern "C" fn(c_int, *const c_char, *mut c_void) -> c_int>,
    );
    fn print_all_levels(level: c_int, format: *const c_char, args: *mut c_void) -> c_int;
    fn btf__parse(path: *const c_char, opts: *mut c_void) -> *mut btf;
    fn btf__free(btf: *mut btf);

    fn p_err(format: *const c_char, ...);

    fn do_prog(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_map(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_link(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_cgroup(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_perf(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_net(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_feature(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_btf(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_gen(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_struct_ops(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_iter(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn do_token(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

#[unsafe(no_mangle)]
pub static mut bin_name: *const c_char = ptr::null();
static mut last_argc: c_int = 0;
static mut last_argv: *mut *mut c_char = ptr::null_mut();
static mut last_do_help: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int> = None;
#[unsafe(no_mangle)]
pub static mut json_wtr: *mut json_writer_t = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut pretty_output: bool = false;
#[unsafe(no_mangle)]
pub static mut json_output: bool = false;
#[unsafe(no_mangle)]
pub static mut show_pinned: bool = false;
#[unsafe(no_mangle)]
pub static mut block_mount: bool = false;
#[unsafe(no_mangle)]
pub static mut verifier_logs: bool = false;
#[unsafe(no_mangle)]
pub static mut relaxed_maps: bool = false;
#[unsafe(no_mangle)]
pub static mut use_loader: bool = false;
#[unsafe(no_mangle)]
pub static mut base_btf: *mut btf = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut refs_table: *mut hashmap = ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut sign_progs: bool = false;
#[unsafe(no_mangle)]
pub static mut private_key_path: *const c_char = ptr::null();
#[unsafe(no_mangle)]
pub static mut cert_path: *const c_char = ptr::null();

unsafe extern "C" fn clean_and_exit(i: c_int) -> ! {
    if json_output {
        jsonw_destroy(&raw mut json_wtr);
    }

    exit(i);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage() {
    last_do_help.unwrap()(last_argc - 1, last_argv.add(1));

    clean_and_exit(-1);
}

unsafe extern "C" fn do_help(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    /* HELP_SPEC_OPTIONS is supplied by main.h in C. */
    fprintf(
        stderr,
        b"Usage: %s [OPTIONS] OBJECT { COMMAND | help }\n       %s batch file FILE\n       %s version\n\n       OBJECT := { prog | map | link | cgroup | perf | net | feature | btf | gen | struct_ops | iter | token }\n       HELP_SPEC_OPTIONS |\n                    {-V|--version} }\n\0"
            .as_ptr() as *const c_char,
        bin_name,
        bin_name,
        bin_name,
    );

    0
}

unsafe extern "C" fn do_batch(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut buf = [0 as c_char; BATCH_LINE_LEN_MAX];
    let mut contline = [0 as c_char; BATCH_LINE_LEN_MAX];
    let mut n_argv = [ptr::null_mut::<c_char>(); BATCH_ARG_NB_MAX];
    let mut lines: c_uint = 0;
    let mut n_argc: c_int;
    let mut fp: *mut FILE;
    let mut cp: *mut c_char;
    let mut err: c_int = 0;
    let mut i: c_int;
    let mut argv = argv;

    if argc < 2 {
        p_err(b"too few parameters for batch\0".as_ptr() as *const c_char);
        return -1;
    } else if argc > 2 {
        p_err(b"too many parameters for batch\0".as_ptr() as *const c_char);
        return -1;
    } else if !is_prefix(*argv, b"file\0".as_ptr() as *const c_char) {
        p_err(
            b"expected 'file', got: %s\0".as_ptr() as *const c_char,
            *argv,
        );
        return -1;
    }
    argv = argv.add(1);

    if strcmp(*argv, b"-\0".as_ptr() as *const c_char) == 0 {
        fp = stdin;
    } else {
        fp = fopen(*argv, b"r\0".as_ptr() as *const c_char);
    }
    if fp.is_null() {
        p_err(
            b"Can't open file (%s): %s\0".as_ptr() as *const c_char,
            *argv,
            strerror(errno),
        );
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    }
    while !fgets(buf.as_mut_ptr(), BATCH_LINE_LEN_MAX as c_int, fp).is_null() {
        cp = strchr(buf.as_mut_ptr(), b'#' as c_int);
        if !cp.is_null() {
            *cp = 0;
        }

        if strlen(buf.as_ptr()) == BATCH_LINE_LEN_MAX - 1 {
            errno = E2BIG;
            break;
        }

        /* Append continuation lines if any (coming after a line ending
         * with '\' in the batch file).
         */
        loop {
            cp = strstr(buf.as_ptr(), b"\\\n\0".as_ptr() as *const c_char);
            if cp.is_null() {
                break;
            }
            if fgets(contline.as_mut_ptr(), BATCH_LINE_LEN_MAX as c_int, fp).is_null()
                || strlen(contline.as_ptr()) == 0
            {
                p_err(
                    b"missing continuation line on command %u\0".as_ptr() as *const c_char,
                    lines,
                );
                err = -1;
                goto_err_close(fp, err);
                return err;
            }

            cp = strchr(contline.as_mut_ptr(), b'#' as c_int);
            if !cp.is_null() {
                *cp = 0;
            }

            if strlen(buf.as_ptr()) + strlen(contline.as_ptr()) + 1 > BATCH_LINE_LEN_MAX {
                p_err(
                    b"command %u is too long\0".as_ptr() as *const c_char,
                    lines,
                );
                err = -1;
                goto_err_close(fp, err);
                return err;
            }
            *buf.as_mut_ptr().add(strlen(buf.as_ptr()) - 2) = 0;
            strcat(buf.as_mut_ptr(), contline.as_ptr());
        }

        n_argc = make_args(
            buf.as_mut_ptr(),
            n_argv.as_mut_ptr(),
            BATCH_ARG_NB_MAX as c_int,
            lines as c_int,
        );
        if n_argc == 0 {
            continue;
        }
        if n_argc < 0 {
            err = n_argc;
            goto_err_close(fp, err);
            return err;
        }

        if json_output {
            jsonw_start_object(json_wtr);
            jsonw_name(json_wtr, b"command\0".as_ptr() as *const c_char);
            jsonw_start_array(json_wtr);
            i = 0;
            while i < n_argc {
                jsonw_string(json_wtr, n_argv[i as usize]);
                i += 1;
            }
            jsonw_end_array(json_wtr);
            jsonw_name(json_wtr, b"output\0".as_ptr() as *const c_char);
        }

        err = cmd_select(commands.as_ptr(), n_argc, n_argv.as_mut_ptr(), Some(do_help));

        if json_output {
            jsonw_end_object(json_wtr);
        }

        if err != 0 {
            goto_err_close(fp, err);
            return err;
        }

        lines += 1;
    }

    if errno != 0 && errno != ENOENT {
        p_err(
            b"reading batch file failed: %s\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        err = -1;
    } else if !json_output {
        printf(
            b"processed %u commands\n\0".as_ptr() as *const c_char,
            lines,
        );
    }

    goto_err_close(fp, err);
    err
}

unsafe fn goto_err_close(fp: *mut FILE, err: c_int) {
    if fp != stdin {
        fclose(fp);
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    let _ = err;
}

static mut commands: [cmd; 17] = [
    cmd {
        cmd: b"help\0".as_ptr() as *const c_char,
        func: Some(do_help),
    },
    cmd {
        cmd: b"batch\0".as_ptr() as *const c_char,
        func: Some(do_batch),
    },
    cmd {
        cmd: b"prog\0".as_ptr() as *const c_char,
        func: Some(do_prog),
    },
    cmd {
        cmd: b"map\0".as_ptr() as *const c_char,
        func: Some(do_map),
    },
    cmd {
        cmd: b"link\0".as_ptr() as *const c_char,
        func: Some(do_link),
    },
    cmd {
        cmd: b"cgroup\0".as_ptr() as *const c_char,
        func: Some(do_cgroup),
    },
    cmd {
        cmd: b"perf\0".as_ptr() as *const c_char,
        func: Some(do_perf),
    },
    cmd {
        cmd: b"net\0".as_ptr() as *const c_char,
        func: Some(do_net),
    },
    cmd {
        cmd: b"feature\0".as_ptr() as *const c_char,
        func: Some(do_feature),
    },
    cmd {
        cmd: b"btf\0".as_ptr() as *const c_char,
        func: Some(do_btf),
    },
    cmd {
        cmd: b"gen\0".as_ptr() as *const c_char,
        func: Some(do_gen),
    },
    cmd {
        cmd: b"struct_ops\0".as_ptr() as *const c_char,
        func: Some(do_struct_ops),
    },
    cmd {
        cmd: b"iter\0".as_ptr() as *const c_char,
        func: Some(do_iter),
    },
    cmd {
        cmd: b"token\0".as_ptr() as *const c_char,
        func: Some(do_token),
    },
    cmd {
        cmd: b"version\0".as_ptr() as *const c_char,
        func: Some(do_version),
    },
    cmd {
        cmd: ptr::null(),
        func: None,
    },
    cmd {
        cmd: ptr::null(),
        func: None,
    },
];

unsafe fn print_feature(feature: *const c_char, state: bool, nb_features: *mut c_uint) {
    if state {
        printf(
            b"%s %s\0".as_ptr() as *const c_char,
            if *nb_features != 0 {
                b",\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            feature,
        );
        *nb_features = (*nb_features).wrapping_add(1);
    }
}

unsafe extern "C" fn do_version(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    /* HAVE_LIBBFD_SUPPORT, HAVE_LLVM_SUPPORT, BPFTOOL_WITHOUT_SKELETONS,
     * and BPFTOOL_WITHOUT_CRYPTO are build-time C conditions.
     */
    let has_libbfd: bool = false;
    let has_llvm: bool = false;
    let has_skeletons: bool = true;
    let has_crypto: bool = true;
    let mut bootstrap = false;
    let mut i: c_int;

    i = 0;
    while !commands[i as usize].cmd.is_null() {
        if strcmp(commands[i as usize].cmd, b"prog\0".as_ptr() as *const c_char) == 0 {
            /* Assume we run a bootstrap version if "bpftool prog"
             * is not available.
             */
            bootstrap = commands[i as usize].func.is_none();
            break;
        }
        i += 1;
    }

    if json_output {
        jsonw_start_object(json_wtr); /* root object */

        jsonw_name(json_wtr, b"version\0".as_ptr() as *const c_char);
        jsonw_printf(
            json_wtr,
            b"\"%d.%d.%d\"\0".as_ptr() as *const c_char,
            libbpf_major_version().wrapping_add(6),
            libbpf_minor_version(),
            BPFTOOL_PATCH_VERSION,
        );
        jsonw_name(json_wtr, b"libbpf_version\0".as_ptr() as *const c_char);
        jsonw_printf(
            json_wtr,
            b"\"%u.%u\"\0".as_ptr() as *const c_char,
            libbpf_major_version(),
            libbpf_minor_version(),
        );

        jsonw_name(json_wtr, b"features\0".as_ptr() as *const c_char);
        jsonw_start_object(json_wtr); /* features */
        jsonw_bool_field(json_wtr, b"libbfd\0".as_ptr() as *const c_char, has_libbfd);
        jsonw_bool_field(json_wtr, b"llvm\0".as_ptr() as *const c_char, has_llvm);
        jsonw_bool_field(json_wtr, b"crypto\0".as_ptr() as *const c_char, has_crypto);
        jsonw_bool_field(json_wtr, b"skeletons\0".as_ptr() as *const c_char, has_skeletons);
        jsonw_bool_field(json_wtr, b"bootstrap\0".as_ptr() as *const c_char, bootstrap);
        jsonw_end_object(json_wtr); /* features */

        jsonw_end_object(json_wtr); /* root object */
    } else {
        let mut nb_features: c_uint = 0;

        printf(
            b"%s v%d.%d.%d\n\0".as_ptr() as *const c_char,
            bin_name,
            libbpf_major_version().wrapping_add(6),
            libbpf_minor_version(),
            BPFTOOL_PATCH_VERSION,
        );
        printf(
            b"using libbpf %s\n\0".as_ptr() as *const c_char,
            libbpf_version_string(),
        );
        printf(b"features:\0".as_ptr() as *const c_char);
        print_feature(
            b"libbfd\0".as_ptr() as *const c_char,
            has_libbfd,
            &mut nb_features,
        );
        print_feature(
            b"llvm\0".as_ptr() as *const c_char,
            has_llvm,
            &mut nb_features,
        );
        print_feature(
            b"crypto\0".as_ptr() as *const c_char,
            has_crypto,
            &mut nb_features,
        );
        print_feature(
            b"skeletons\0".as_ptr() as *const c_char,
            has_skeletons,
            &mut nb_features,
        );
        print_feature(
            b"bootstrap\0".as_ptr() as *const c_char,
            bootstrap,
            &mut nb_features,
        );
        printf(b"\n\0".as_ptr() as *const c_char);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_select(
    cmds: *const cmd,
    argc: c_int,
    argv: *mut *mut c_char,
    help: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
) -> c_int {
    let mut i: c_uint;

    last_argc = argc;
    last_argv = argv;
    last_do_help = help;

    if argc < 1 && (*cmds).func.is_some() {
        return (*cmds).func.unwrap()(argc, argv);
    }

    i = 0;
    while !(*cmds.add(i as usize)).cmd.is_null() {
        if is_prefix(*argv, (*cmds.add(i as usize)).cmd) {
            if (*cmds.add(i as usize)).func.is_none() {
                p_err(
                    b"command '%s' is not supported in bootstrap mode\0".as_ptr() as *const c_char,
                    (*cmds.add(i as usize)).cmd,
                );
                return -1;
            }
            return (*cmds.add(i as usize)).func.unwrap()(argc - 1, argv.add(1));
        }
        i += 1;
    }

    help.unwrap()(argc - 1, argv.add(1));

    -1
}

#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_common_prefix(arg: *const c_char, mut refs: ...) -> c_int {
    let mut count: c_uint = 0;
    let mut ref_: *const c_char;
    let mut msg = [0 as c_char; 256];

    snprintf(
        msg.as_mut_ptr(),
        mem::size_of_val(&msg),
        b"ambiguous prefix: '%s' could be '\0".as_ptr() as *const c_char,
        arg,
    );
    loop {
        ref_ = refs.arg::<*const c_char>();
        if ref_.is_null() {
            break;
        }
        if !is_prefix(arg, ref_) {
            continue;
        }
        count += 1;
        if count > 1 {
            let len = strlen(msg.as_ptr());
            strncat(
                msg.as_mut_ptr(),
                b"' or '\0".as_ptr() as *const c_char,
                mem::size_of_val(&msg) - len - 1,
            );
        }
        let len = strlen(msg.as_ptr());
        strncat(
            msg.as_mut_ptr(),
            ref_,
            mem::size_of_val(&msg) - len - 1,
        );
    }
    let len = strlen(msg.as_ptr());
    strncat(
        msg.as_mut_ptr(),
        b"'\0".as_ptr() as *const c_char,
        mem::size_of_val(&msg) - len - 1,
    );

    if count >= 2 {
        p_err(b"%s\0".as_ptr() as *const c_char, msg.as_ptr());
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprint_hex(
    f: *mut FILE,
    arg: *mut c_void,
    n: c_uint,
    sep: *const c_char,
) {
    let data = arg as *mut c_uchar;
    let mut i: c_uint;

    i = 0;
    while i < n {
        let mut pfx = b"\0".as_ptr() as *const c_char;

        if i == 0 {
            /* nothing */
        } else if i % 16 == 0 {
            fprintf(f, b"\n\0".as_ptr() as *const c_char);
        } else if i % 8 == 0 {
            fprintf(f, b"  \0".as_ptr() as *const c_char);
        } else {
            pfx = sep;
        }

        fprintf(
            f,
            b"%s%02hhx\0".as_ptr() as *const c_char,
            if i != 0 {
                pfx
            } else {
                b"\0".as_ptr() as *const c_char
            },
            *data.add(i as usize) as c_int,
        );
        i += 1;
    }
}

/* Split command line into argument vector. */
unsafe fn make_args(
    line: *mut c_char,
    n_argv: *mut *mut c_char,
    maxargs: c_int,
    cmd_nb: c_int,
) -> c_int {
    static ws: &[u8; 5] = b" \t\r\n\0";
    let mut cp = line;
    let mut n_argc: c_int = 0;

    while *cp != 0 {
        /* Skip leading whitespace. */
        cp = cp.add(strspn(cp, ws.as_ptr() as *const c_char));

        if *cp == 0 {
            break;
        }

        if n_argc >= maxargs - 1 {
            p_err(
                b"too many arguments to command %d\0".as_ptr() as *const c_char,
                cmd_nb,
            );
            return -1;
        }

        /* Word begins with quote. */
        if *cp == b'\'' as c_char || *cp == b'"' as c_char {
            let quote = *cp;
            cp = cp.add(1);

            *n_argv.add(n_argc as usize) = cp;
            n_argc += 1;
            /* Find ending quote. */
            cp = strchr(cp, quote as c_int);
            if cp.is_null() {
                p_err(
                    b"unterminated quoted string in command %d\0".as_ptr() as *const c_char,
                    cmd_nb,
                );
                return -1;
            }
        } else {
            *n_argv.add(n_argc as usize) = cp;
            n_argc += 1;

            /* Find end of word. */
            cp = cp.add(strcspn(cp, ws.as_ptr() as *const c_char));
            if *cp == 0 {
                break;
            }
        }

        /* Separate words. */
        *cp = 0;
        cp = cp.add(1);
    }
    *n_argv.add(n_argc as usize) = ptr::null_mut();

    n_argc
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    static options: [option; 12] = [
        option {
            name: b"json\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'j' as c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'h' as c_int,
        },
        option {
            name: b"pretty\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'p' as c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'V' as c_int,
        },
        option {
            name: b"bpffs\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'f' as c_int,
        },
        option {
            name: b"mapcompat\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'm' as c_int,
        },
        option {
            name: b"nomount\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'n' as c_int,
        },
        option {
            name: b"debug\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'd' as c_int,
        },
        option {
            name: b"use-loader\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'L' as c_int,
        },
        option {
            name: b"sign\0".as_ptr() as *const c_char,
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: b'S' as c_int,
        },
        option {
            name: b"base-btf\0".as_ptr() as *const c_char,
            has_arg: required_argument,
            flag: ptr::null_mut(),
            val: b'B' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];
    let mut version_requested = false;
    let mut opt: c_int;
    let ret: c_int;

    setlinebuf(stdout);

    /* USE_LIBCAP is a build-time C condition.
     * Libcap < 2.63 hooks before main() to compute the number of
     * capabilities of the running kernel, and doing so it calls prctl()
     * which may fail and set errno to non-zero.
     * Let's reset errno to make sure this does not interfere with the
     * batch mode.
     */

    last_do_help = Some(do_help);
    pretty_output = false;
    json_output = false;
    show_pinned = false;
    block_mount = false;
    bin_name = b"bpftool\0".as_ptr() as *const c_char;

    opterr = 0;
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"VhpjfLmndSi:k:B:l\0".as_ptr() as *const c_char,
            options.as_ptr(),
            ptr::null_mut(),
        );
        if opt < 0 {
            break;
        }
        match opt {
            x if x == b'V' as c_int => {
                version_requested = true;
            }
            x if x == b'h' as c_int => {
                return do_help(argc, argv);
            }
            x if x == b'p' as c_int => {
                pretty_output = true;
                /* fall through */
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
            x if x == b'j' as c_int => {
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
            x if x == b'f' as c_int => {
                show_pinned = true;
            }
            x if x == b'm' as c_int => {
                relaxed_maps = true;
            }
            x if x == b'n' as c_int => {
                block_mount = true;
            }
            x if x == b'd' as c_int => {
                libbpf_set_print(Some(print_all_levels));
                verifier_logs = true;
            }
            x if x == b'B' as c_int => {
                base_btf = btf__parse(optarg, ptr::null_mut());
                if base_btf.is_null() {
                    p_err(
                        b"failed to parse base BTF at '%s': %d\n\0".as_ptr() as *const c_char,
                        optarg,
                        -errno,
                    );
                    return -1;
                }
            }
            x if x == b'L' as c_int => {
                use_loader = true;
            }
            x if x == b'S' as c_int => {
                sign_progs = true;
                use_loader = true;
            }
            x if x == b'k' as c_int => {
                private_key_path = optarg;
            }
            x if x == b'i' as c_int => {
                cert_path = optarg;
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

    if sign_progs && (private_key_path.is_null() || cert_path.is_null()) {
        p_err(b"-i <identity_x509_cert> and -k <private_key> must be supplied with -S for signing\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if !sign_progs && (!private_key_path.is_null() || !cert_path.is_null()) {
        p_err(b"--sign (or -S) must be explicitly passed with -i <identity_x509_cert> and -k <private_key> to sign the programs\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    if version_requested {
        ret = do_version(argc, argv);
    } else {
        ret = cmd_select(commands.as_ptr(), argc, argv, Some(do_help));
    }

    if json_output {
        jsonw_destroy(&raw mut json_wtr);
    }

    btf__free(base_btf);

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
