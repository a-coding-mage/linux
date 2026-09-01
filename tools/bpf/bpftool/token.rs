// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2025 Didi Technology Co., Tao Chen */

/* Translated from bpf/bpftool/token.c. C includes are represented by the
 * external declarations below; definitions are supplied by the surrounding
 * bpftool build.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const MOUNTS_FILE: &[u8] = b"/proc/mounts\0";
const ITEMS_PER_LINE: c_int = 4;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut c_char,
    pub mnt_dir: *mut c_char,
    pub mnt_type: *mut c_char,
    pub mnt_opts: *mut c_char,
    pub mnt_freq: c_int,
    pub mnt_passno: c_int,
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cmd {
    pub cmd: *const c_char,
    pub func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

#[repr(C)]
struct token_set {
    header: *const c_char,
    key: *const c_char,
}

unsafe extern "C" {
    static mut json_output: bool;
    static mut json_wtr: *mut json_writer_t;
    static mut bin_name: *const c_char;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtok_r(
        str: *mut c_char,
        delim: *const c_char,
        saveptr: *mut *mut c_char,
    ) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;

    fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut FILE;
    fn getmntent(stream: *mut FILE) -> *mut mntent;
    fn endmntent(stream: *mut FILE) -> c_int;

    fn jsonw_start_array(wr: *mut json_writer_t);
    fn jsonw_end_array(wr: *mut json_writer_t);
    fn jsonw_start_object(wr: *mut json_writer_t);
    fn jsonw_end_object(wr: *mut json_writer_t);
    fn jsonw_string(wr: *mut json_writer_t, value: *const c_char);
    fn jsonw_string_field(wr: *mut json_writer_t, prop: *const c_char, value: *const c_char);
    fn jsonw_name(wr: *mut json_writer_t, name: *const c_char);
    fn jsonw_null(wr: *mut json_writer_t);

    fn p_err(format: *const c_char, ...);
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    ) -> c_int;

    fn BAD_ARG() -> c_int;
}

static SETS: [token_set; 4] = [
    token_set {
        header: b"allowed_cmds\0".as_ptr() as *const c_char,
        key: b"delegate_cmds\0".as_ptr() as *const c_char,
    },
    token_set {
        header: b"allowed_maps\0".as_ptr() as *const c_char,
        key: b"delegate_maps\0".as_ptr() as *const c_char,
    },
    token_set {
        header: b"allowed_progs\0".as_ptr() as *const c_char,
        key: b"delegate_progs\0".as_ptr() as *const c_char,
    },
    token_set {
        header: b"allowed_attachs\0".as_ptr() as *const c_char,
        key: b"delegate_attachs\0".as_ptr() as *const c_char,
    },
];

unsafe fn has_delegate_options(mnt_ops: *const c_char) -> bool {
    !strstr(mnt_ops, b"delegate_cmds\0".as_ptr() as *const c_char).is_null()
        || !strstr(mnt_ops, b"delegate_maps\0".as_ptr() as *const c_char).is_null()
        || !strstr(mnt_ops, b"delegate_progs\0".as_ptr() as *const c_char).is_null()
        || !strstr(mnt_ops, b"delegate_attachs\0".as_ptr() as *const c_char).is_null()
}

unsafe fn get_delegate_value(mut opts: *mut c_char, key: *const c_char) -> *mut c_char {
    let mut token: *mut c_char;
    let mut rest: *mut c_char = ptr::null_mut();
    let mut ret: *mut c_char = ptr::null_mut();

    if opts.is_null() {
        return ptr::null_mut();
    }

    token = strtok_r(opts, b",\0".as_ptr() as *const c_char, &mut rest);
    while !token.is_null() {
        if strncmp(token, key, strlen(key)) == 0
            && *token.add(strlen(key)) == b'=' as c_char
        {
            ret = token.add(strlen(key) + 1);
            break;
        }

        opts = ptr::null_mut();
        token = strtok_r(opts, b",\0".as_ptr() as *const c_char, &mut rest);
    }

    ret
}

unsafe fn print_items_per_line(input: *mut c_char, items_per_line: c_int) {
    let mut str_: *mut c_char;
    let mut rest: *mut c_char = ptr::null_mut();
    let mut cnt: c_int = 0;

    if input.is_null() {
        return;
    }

    str_ = strtok_r(input, b":\0".as_ptr() as *const c_char, &mut rest);
    while !str_.is_null() {
        if cnt % items_per_line == 0 {
            printf(b"\n\t  \0".as_ptr() as *const c_char);
        }

        printf(b"%-20s\0".as_ptr() as *const c_char, str_);
        cnt += 1;
        str_ = strtok_r(ptr::null_mut(), b":\0".as_ptr() as *const c_char, &mut rest);
    }
}

unsafe fn show_token_info_plain(mntent: *mut mntent) {
    let mut i: usize;

    printf(
        b"token_info  %s\0".as_ptr() as *const c_char,
        (*mntent).mnt_dir,
    );

    i = 0;
    while i < SETS.len() {
        let opts: *mut c_char;
        let value: *mut c_char;

        printf(b"\n\t%s:\0".as_ptr() as *const c_char, SETS[i].header);
        opts = strdup((*mntent).mnt_opts);
        value = get_delegate_value(opts, SETS[i].key);
        print_items_per_line(value, ITEMS_PER_LINE);
        free(opts as *mut c_void);

        i += 1;
    }

    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn split_json_array_str(input: *mut c_char) {
    let mut str_: *mut c_char;
    let mut rest: *mut c_char = ptr::null_mut();

    if input.is_null() {
        jsonw_start_array(json_wtr);
        jsonw_end_array(json_wtr);
        return;
    }

    jsonw_start_array(json_wtr);
    str_ = strtok_r(input, b":\0".as_ptr() as *const c_char, &mut rest);
    while !str_.is_null() {
        jsonw_string(json_wtr, str_);
        str_ = strtok_r(ptr::null_mut(), b":\0".as_ptr() as *const c_char, &mut rest);
    }
    jsonw_end_array(json_wtr);
}

unsafe fn show_token_info_json(mntent: *mut mntent) {
    let mut i: usize;

    jsonw_start_object(json_wtr);
    jsonw_string_field(
        json_wtr,
        b"token_info\0".as_ptr() as *const c_char,
        (*mntent).mnt_dir,
    );

    i = 0;
    while i < SETS.len() {
        let opts: *mut c_char;
        let value: *mut c_char;

        jsonw_name(json_wtr, SETS[i].header);
        opts = strdup((*mntent).mnt_opts);
        value = get_delegate_value(opts, SETS[i].key);
        split_json_array_str(value);
        free(opts as *mut c_void);

        i += 1;
    }

    jsonw_end_object(json_wtr);
}

unsafe fn __show_token_info(mntent: *mut mntent) -> c_int {
    if json_output {
        show_token_info_json(mntent);
    } else {
        show_token_info_plain(mntent);
    }

    0
}

unsafe fn show_token_info() -> c_int {
    let fp: *mut FILE;
    let mut ent: *mut mntent;

    fp = setmntent(
        MOUNTS_FILE.as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if fp.is_null() {
        p_err(
            b"Failed to open: %s\0".as_ptr() as *const c_char,
            MOUNTS_FILE.as_ptr() as *const c_char,
        );
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    }

    loop {
        ent = getmntent(fp);
        if ent.is_null() {
            break;
        }
        if strncmp(
            (*ent).mnt_type,
            b"bpf\0".as_ptr() as *const c_char,
            3,
        ) == 0
        {
            if has_delegate_options((*ent).mnt_opts) {
                __show_token_info(ent);
            }
        }
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    endmntent(fp);

    0
}

unsafe extern "C" fn do_show(argc: c_int, _argv: *mut *mut c_char) -> c_int {
    if argc != 0 {
        return BAD_ARG();
    }

    show_token_info()
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        b"Usage: %1$s %2$s { show | list }\n       %1$s %2$s help\n       %3$s }\n\n\0"
            .as_ptr() as *const c_char,
        bin_name,
        *argv.offset(-2),
        b"HELP_SPEC_OPTIONS\0".as_ptr() as *const c_char,
    );
    0
}

static CMDS: [cmd; 4] = [
    cmd {
        cmd: b"show\0".as_ptr() as *const c_char,
        func: Some(do_show),
    },
    cmd {
        cmd: b"list\0".as_ptr() as *const c_char,
        func: Some(do_show),
    },
    cmd {
        cmd: b"help\0".as_ptr() as *const c_char,
        func: Some(do_help),
    },
    cmd {
        cmd: ptr::null(),
        func: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn do_token(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(CMDS.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
