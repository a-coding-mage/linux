// SPDX-License-Identifier: GPL-2.0
/*
 * config.c
 *
 * Helper functions for parsing config items.
 * Originally copied from GIT source.
 *
 * Copyright (C) Linus Torvalds, 2005
 * Copyright (C) Johannes Schindelin, 2005
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, zeroed};
use core::ptr;

type size_t = usize;
type u8 = core::ffi::c_uchar;
type u64 = core::ffi::c_ulonglong;
type FILE = c_void;
type va_list = *mut c_void;
type config_fn_t =
    Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>;

const MAXNAME: usize = 256;
const MAXPATHLEN: usize = 4096;
const PATH_MAX: usize = 4096;
const BUFSIZ: usize = 8192;
const EOF: c_int = -1;
const R_OK: c_int = 4;
const DEBUG_CACHE_DIR: &[u8] = b".debug\0";
const METRIC_ONLY_LEN: c_int = 20;
const ETC_PERFCONFIG: *const c_char = b"perfconfig\0".as_ptr() as *const c_char;
const AGGR_GLOBAL: c_int = 0;
const MAX_CACHE_LVL: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_mode: c_int,
    pub aggr_level: c_int,
    pub scale: bool,
    pub unit_width: c_int,
    pub run_count: c_int,
    pub metric_only_len: c_int,
    pub walltime_nsecs_stats: *mut stats,
    pub big_num: bool,
    pub ctl_fd: c_int,
    pub ctl_fd_ack: c_int,
    pub iostat_run: bool,
    pub no_csv_summary: bool,
}

#[repr(C)]
pub struct perf_config_section {
    pub node: list_head,
    pub items: list_head,
    pub name: *mut c_char,
    pub from_system_config: bool,
}

#[repr(C)]
pub struct perf_config_item {
    pub node: list_head,
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub from_system_config: bool,
}

#[repr(C)]
pub struct perf_config_set {
    pub sections: list_head,
}

#[repr(C)]
pub struct stat {
    pub st_uid: c_ulong,
    pub st_size: c_long,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub addr2line_timeout_ms: c_ulong,
    pub addr2line_disable_warn: c_int,
    pub show_hist_headers: c_int,
}

static mut walltime_nsecs_stats: stats = stats { _private: [] };

#[no_mangle]
pub static mut stat_config: perf_stat_config = perf_stat_config {
    aggr_mode: AGGR_GLOBAL,
    aggr_level: MAX_CACHE_LVL + 1,
    scale: true,
    unit_width: 4, /* strlen("unit") */
    run_count: 1,
    metric_only_len: METRIC_ONLY_LEN,
    walltime_nsecs_stats: unsafe { &raw mut walltime_nsecs_stats },
    big_num: true,
    ctl_fd: -1,
    ctl_fd_ack: -1,
    iostat_run: false,
    no_csv_summary: false,
};

#[no_mangle]
pub static mut buildid_dir: [c_char; MAXPATHLEN] = [0; MAXPATHLEN]; /* root dir for buildid, binary cache */

static mut config_file: *mut FILE = ptr::null_mut();
static mut config_file_name: *const c_char = ptr::null();
static mut config_linenr: c_int = 0;
static mut config_file_eof: c_int = 0;
static mut config_set: *mut perf_config_set = ptr::null_mut();

#[no_mangle]
pub static mut config_exclusive_filename: *const c_char = ptr::null();

unsafe extern "C" {
    fn fgetc(stream: *mut FILE) -> c_int;
    fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn geteuid() -> c_ulong;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> core::ffi::c_longlong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn mkpath(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> *const c_char;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn vsscanf(str: *const c_char, format: *const c_char, ap: va_list) -> c_int;
    fn system_path(path: *const c_char) -> *const c_char;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn perf_hist_config(var: *const c_char, value: *const c_char) -> c_int;
    fn perf_callchain_config(var: *const c_char, value: *const c_char) -> c_int;
    fn addr2line_configure(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int;
    fn unwind__configure(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int;

    static mut proc_map_timeout: c_ulong;
    static mut symbol_conf: symbol_conf_t;
    static mut evsel__bpf_counter_events: *mut c_char;
}

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

#[inline]
unsafe fn list_del_init(entry: *mut list_head) {
    let next = (*entry).next;
    let prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
    INIT_LIST_HEAD(entry);
}

#[inline]
unsafe fn zfree(pptr: *mut *mut c_char) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = ptr::null_mut();
    }
}

#[inline]
unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

#[inline]
fn isspace(c: c_int) -> bool {
    matches!(c, 9 | 10 | 11 | 12 | 13 | 32)
}

#[inline]
fn isalnum(c: c_int) -> bool {
    (c >= b'0' as c_int && c <= b'9' as c_int)
        || (c >= b'a' as c_int && c <= b'z' as c_int)
        || (c >= b'A' as c_int && c <= b'Z' as c_int)
}

#[inline]
fn isalpha(c: c_int) -> bool {
    (c >= b'a' as c_int && c <= b'z' as c_int)
        || (c >= b'A' as c_int && c <= b'Z' as c_int)
}

#[inline]
fn tolower(c: c_int) -> c_int {
    if c >= b'A' as c_int && c <= b'Z' as c_int {
        c + (b'a' - b'A') as c_int
    } else {
        c
    }
}

unsafe fn get_next_char() -> c_int {
    let mut c: c_int;
    let f: *mut FILE;

    c = b'\n' as c_int;
    f = config_file;
    if !f.is_null() {
        c = fgetc(f);
        if c == b'\r' as c_int {
            /* DOS like systems */
            c = fgetc(f);
            if c != b'\n' as c_int {
                ungetc(c, f);
                c = b'\r' as c_int;
            }
        }
        if c == b'\n' as c_int {
            config_linenr += 1;
        }
        if c == EOF {
            config_file_eof = 1;
            c = b'\n' as c_int;
        }
    }
    c
}

unsafe fn parse_value() -> *mut c_char {
    static mut VALUE: [c_char; 1024] = [0; 1024];
    let mut quote = 0;
    let mut comment = 0;
    let mut space = 0;
    let mut len: size_t = 0;

    loop {
        let mut c = get_next_char();

        if len >= VALUE.len() - 1 {
            return ptr::null_mut();
        }
        if c == b'\n' as c_int {
            if quote != 0 {
                return ptr::null_mut();
            }
            VALUE[len] = 0;
            return VALUE.as_mut_ptr();
        }
        if comment != 0 {
            continue;
        }
        if isspace(c) && quote == 0 {
            space = 1;
            continue;
        }
        if quote == 0 && (c == b';' as c_int || c == b'#' as c_int) {
            comment = 1;
            continue;
        }
        if space != 0 {
            if len != 0 {
                VALUE[len] = b' ' as c_char;
                len += 1;
            }
            space = 0;
        }
        if c == b'\\' as c_int {
            c = get_next_char();
            match c {
                x if x == b'\n' as c_int => continue,
                x if x == b't' as c_int => c = b'\t' as c_int,
                x if x == b'b' as c_int => c = 8,
                x if x == b'n' as c_int => c = b'\n' as c_int,
                x if x == b'\\' as c_int || x == b'"' as c_int => {}
                _ => return ptr::null_mut(),
            }
            VALUE[len] = c as c_char;
            len += 1;
            continue;
        }
        if c == b'"' as c_int {
            quote = 1 - quote;
            continue;
        }
        VALUE[len] = c as c_char;
        len += 1;
    }
}

#[inline]
fn iskeychar(c: c_int) -> bool {
    isalnum(c) || c == b'-' as c_int || c == b'_' as c_int
}

unsafe fn get_value(fn_: config_fn_t, data: *mut c_void, name: *mut c_char, mut len: u32) -> c_int {
    let mut c: c_int;
    let mut value: *mut c_char;

    loop {
        c = get_next_char();
        if config_file_eof != 0 {
            break;
        }
        if !iskeychar(c) {
            break;
        }
        *name.add(len as usize) = c as c_char;
        len += 1;
        if len as usize >= MAXNAME {
            return -1;
        }
    }
    *name.add(len as usize) = 0;
    while c == b' ' as c_int || c == b'\t' as c_int {
        c = get_next_char();
    }

    value = ptr::null_mut();
    if c != b'\n' as c_int {
        if c != b'=' as c_int {
            return -1;
        }
        value = parse_value();
        if value.is_null() {
            return -1;
        }
    }
    fn_.unwrap()(name, value, data)
}

unsafe fn get_extended_base_var(name: *mut c_char, mut baselen: c_int, mut c: c_int) -> c_int {
    loop {
        if c == b'\n' as c_int {
            return -1;
        }
        c = get_next_char();
        if !isspace(c) {
            break;
        }
    }

    /* We require the format to be '[base "extension"]' */
    if c != b'"' as c_int {
        return -1;
    }
    *name.add(baselen as usize) = b'.' as c_char;
    baselen += 1;

    loop {
        let mut ch = get_next_char();

        if ch == b'\n' as c_int {
            return -1;
        }
        if ch == b'"' as c_int {
            break;
        }
        if ch == b'\\' as c_int {
            ch = get_next_char();
            if ch == b'\n' as c_int {
                return -1;
            }
        }
        *name.add(baselen as usize) = ch as c_char;
        baselen += 1;
        if baselen > (MAXNAME / 2) as c_int {
            return -1;
        }
    }

    /* Final ']' */
    if get_next_char() != b']' as c_int {
        return -1;
    }
    baselen
}

unsafe fn get_base_var(name: *mut c_char) -> c_int {
    let mut baselen = 0;

    loop {
        let c = get_next_char();
        if config_file_eof != 0 {
            return -1;
        }
        if c == b']' as c_int {
            return baselen;
        }
        if isspace(c) {
            return get_extended_base_var(name, baselen, c);
        }
        if !iskeychar(c) && c != b'.' as c_int {
            return -1;
        }
        if baselen > (MAXNAME / 2) as c_int {
            return -1;
        }
        *name.add(baselen as usize) = tolower(c) as c_char;
        baselen += 1;
    }
}

unsafe fn perf_parse_file(fn_: config_fn_t, data: *mut c_void) -> c_int {
    let mut comment = 0;
    let mut baselen = 0;
    static mut VAR: [c_char; MAXNAME] = [0; MAXNAME];

    /* U+FEFF Byte Order Mark in UTF8 */
    static UTF8_BOM: [u8; 4] = [0xef, 0xbb, 0xbf, 0];
    let utf8_bom = UTF8_BOM.as_ptr();
    let mut bomptr: *const u8 = utf8_bom;

    loop {
        let mut c = get_next_char();
        let line: c_int;

        if !bomptr.is_null() && *bomptr != 0 {
            /* We are at the file beginning; skip UTF8-encoded BOM
             * if present. Sane editors won't put this in on their
             * own, but e.g. Windows Notepad will do it happily. */
            if c as u8 == *bomptr {
                bomptr = bomptr.add(1);
                continue;
            } else {
                /* Do not tolerate partial BOM. */
                if bomptr != utf8_bom {
                    break;
                }
                /* No BOM at file beginning. Cool. */
                bomptr = ptr::null();
            }
        }
        if c == b'\n' as c_int {
            if config_file_eof != 0 {
                return 0;
            }
            comment = 0;
            continue;
        }
        if comment != 0 || isspace(c) {
            continue;
        }
        if c == b'#' as c_int || c == b';' as c_int {
            comment = 1;
            continue;
        }
        if c == b'[' as c_int {
            baselen = get_base_var(VAR.as_mut_ptr());
            if baselen <= 0 {
                break;
            }
            VAR[baselen as usize] = b'.' as c_char;
            baselen += 1;
            VAR[baselen as usize] = 0;
            continue;
        }
        if !isalpha(c) {
            break;
        }
        VAR[baselen as usize] = tolower(c) as c_char;

        /*
         * The get_value function might or might not reach the '\n',
         * so saving the current line number for error reporting.
         */
        line = config_linenr;
        if get_value(fn_, data, VAR.as_mut_ptr(), (baselen + 1) as u32) < 0 {
            config_linenr = line;
            break;
        }
    }
    pr_err(
        b"bad config file line %d in %s\n\0".as_ptr() as *const c_char,
        config_linenr,
        config_file_name,
    );
    -1
}

unsafe fn parse_unit_factor(end: *const c_char, val: *mut c_ulong) -> c_int {
    if *end == 0 {
        return 1;
    } else if strcasecmp(end, b"k\0".as_ptr() as *const c_char) == 0 {
        *val *= 1024;
        return 1;
    } else if strcasecmp(end, b"m\0".as_ptr() as *const c_char) == 0 {
        *val *= 1024 * 1024;
        return 1;
    } else if strcasecmp(end, b"g\0".as_ptr() as *const c_char) == 0 {
        *val *= 1024 * 1024 * 1024;
        return 1;
    }
    0
}

unsafe fn perf_parse_llong(value: *const c_char, ret: *mut core::ffi::c_longlong) -> c_int {
    if !value.is_null() && *value != 0 {
        let mut end: *mut c_char = ptr::null_mut();
        let val = strtoll(value, &mut end, 0);
        let mut factor: c_ulong = 1;

        if parse_unit_factor(end, &mut factor) == 0 {
            return 0;
        }
        *ret = val * factor as core::ffi::c_longlong;
        return 1;
    }
    0
}

unsafe fn perf_parse_long(value: *const c_char, ret: *mut c_long) -> c_int {
    if !value.is_null() && *value != 0 {
        let mut end: *mut c_char = ptr::null_mut();
        let val = strtol(value, &mut end, 0);
        let mut factor: c_ulong = 1;
        if parse_unit_factor(end, &mut factor) == 0 {
            return 0;
        }
        *ret = val * factor as c_long;
        return 1;
    }
    0
}

unsafe fn bad_config(name: *const c_char) {
    if !config_file_name.is_null() {
        pr_warning(
            b"bad config value for '%s' in %s, ignoring...\n\0".as_ptr() as *const c_char,
            name,
            config_file_name,
        );
    } else {
        pr_warning(
            b"bad config value for '%s', ignoring...\n\0".as_ptr() as *const c_char,
            name,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_u64(
    dest: *mut u64,
    name: *const c_char,
    value: *const c_char,
) -> c_int {
    let mut ret: core::ffi::c_longlong = 0;

    if perf_parse_llong(value, &mut ret) == 0 {
        bad_config(name);
        return -1;
    }

    *dest = ret as u64;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_int(
    dest: *mut c_int,
    name: *const c_char,
    value: *const c_char,
) -> c_int {
    let mut ret: c_long = 0;
    if perf_parse_long(value, &mut ret) == 0 {
        bad_config(name);
        return -1;
    }
    *dest = ret as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_u8(
    dest: *mut u8,
    name: *const c_char,
    value: *const c_char,
) -> c_int {
    let mut ret: c_long = 0;

    if perf_parse_long(value, &mut ret) == 0 {
        bad_config(name);
        return -1;
    }
    *dest = ret as u8;
    0
}

unsafe fn perf_config_bool_or_int(
    name: *const c_char,
    value: *const c_char,
    is_bool: *mut c_int,
) -> c_int {
    let mut ret: c_int = 0;

    *is_bool = 1;
    if value.is_null() {
        return 1;
    }
    if *value == 0 {
        return 0;
    }
    if strcasecmp(value, b"true\0".as_ptr() as *const c_char) == 0
        || strcasecmp(value, b"yes\0".as_ptr() as *const c_char) == 0
        || strcasecmp(value, b"on\0".as_ptr() as *const c_char) == 0
    {
        return 1;
    }
    if strcasecmp(value, b"false\0".as_ptr() as *const c_char) == 0
        || strcasecmp(value, b"no\0".as_ptr() as *const c_char) == 0
        || strcasecmp(value, b"off\0".as_ptr() as *const c_char) == 0
    {
        return 0;
    }
    *is_bool = 0;
    if perf_config_int(&mut ret, name, value) < 0 {
        -1
    } else {
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_bool(name: *const c_char, value: *const c_char) -> c_int {
    let mut discard: c_int = 0;
    (perf_config_bool_or_int(name, value, &mut discard) != 0) as c_int
}

unsafe fn perf_config_dirname(name: *const c_char, value: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }
    value
}

unsafe fn perf_buildid_config(var: *const c_char, value: *const c_char) -> c_int {
    /* same dir for all commands */
    if strcmp(var, b"buildid.dir\0".as_ptr() as *const c_char) == 0 {
        let dir = perf_config_dirname(var, value);

        if dir.is_null() {
            pr_err(b"Invalid buildid directory!\n\0".as_ptr() as *const c_char);
            return -1;
        }
        strncpy(buildid_dir.as_mut_ptr(), dir, MAXPATHLEN - 1);
        buildid_dir[MAXPATHLEN - 1] = 0;
    }

    0
}

unsafe fn perf_default_core_config(var: *const c_char, value: *const c_char) -> c_int {
    if strcmp(var, b"core.proc-map-timeout\0".as_ptr() as *const c_char) == 0 {
        proc_map_timeout = strtoul(value, ptr::null_mut(), 10);
    }

    if strcmp(var, b"core.addr2line-timeout\0".as_ptr() as *const c_char) == 0 {
        symbol_conf.addr2line_timeout_ms = strtoul(value, ptr::null_mut(), 10);
    }

    if strcmp(var, b"core.addr2line-disable-warn\0".as_ptr() as *const c_char) == 0 {
        symbol_conf.addr2line_disable_warn = perf_config_bool(var, value);
    }

    /* Add other config variables here. */
    0
}

unsafe fn perf_ui_config(var: *const c_char, value: *const c_char) -> c_int {
    /* Add other config variables here. */
    if strcmp(var, b"ui.show-headers\0".as_ptr() as *const c_char) == 0 {
        symbol_conf.show_hist_headers = perf_config_bool(var, value);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_stat__set_big_num(set: c_int) {
    stat_config.big_num = set != 0;
}

unsafe fn perf_stat__set_no_csv_summary(set: c_int) {
    stat_config.no_csv_summary = set != 0;
}

unsafe fn perf_stat_config(var: *const c_char, value: *const c_char) -> c_int {
    if strcmp(var, b"stat.big-num\0".as_ptr() as *const c_char) == 0 {
        perf_stat__set_big_num(perf_config_bool(var, value));
    }

    if strcmp(var, b"stat.no-csv-summary\0".as_ptr() as *const c_char) == 0 {
        perf_stat__set_no_csv_summary(perf_config_bool(var, value));
    }

    if strcmp(var, b"stat.bpf-counter-events\0".as_ptr() as *const c_char) == 0 {
        evsel__bpf_counter_events = strdup(value);
    }

    /* Add other config variables here. */
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_default_config(
    var: *const c_char,
    value: *const c_char,
    dummy: *mut c_void,
) -> c_int {
    if strstarts(var, b"core.\0".as_ptr() as *const c_char) {
        return perf_default_core_config(var, value);
    }

    if strstarts(var, b"hist.\0".as_ptr() as *const c_char) {
        return perf_hist_config(var, value);
    }

    if strstarts(var, b"ui.\0".as_ptr() as *const c_char) {
        return perf_ui_config(var, value);
    }

    if strstarts(var, b"call-graph.\0".as_ptr() as *const c_char) {
        return perf_callchain_config(var, value);
    }

    if strstarts(var, b"buildid.\0".as_ptr() as *const c_char) {
        return perf_buildid_config(var, value);
    }

    if strstarts(var, b"stat.\0".as_ptr() as *const c_char) {
        return perf_stat_config(var, value);
    }

    if strstarts(var, b"addr2line.\0".as_ptr() as *const c_char) {
        return addr2line_configure(var, value, dummy);
    }

    if strstarts(var, b"unwind.\0".as_ptr() as *const c_char) {
        return unwind__configure(var, value, dummy);
    }

    /* Add other config variables here. */
    0
}

unsafe fn perf_config_from_file(
    fn_: config_fn_t,
    filename: *const c_char,
    data: *mut c_void,
) -> c_int {
    let mut ret: c_int;
    let f = fopen(filename, b"r\0".as_ptr() as *const c_char);

    ret = -1;
    if !f.is_null() {
        config_file = f;
        config_file_name = filename;
        config_linenr = 1;
        config_file_eof = 0;
        ret = perf_parse_file(fn_, data);
        fclose(f);
        config_file_name = ptr::null();
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_etc_perfconfig() -> *const c_char {
    static mut SYSTEM_WIDE: *const c_char = ptr::null();
    if SYSTEM_WIDE.is_null() {
        SYSTEM_WIDE = system_path(ETC_PERFCONFIG);
    }
    SYSTEM_WIDE
}

unsafe fn perf_env_bool(k: *const c_char, def: c_int) -> c_int {
    let v = getenv(k);
    if !v.is_null() {
        perf_config_bool(k, v)
    } else {
        def
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_system() -> c_int {
    (perf_env_bool(b"PERF_CONFIG_NOSYSTEM\0".as_ptr() as *const c_char, 0) == 0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_global() -> c_int {
    (perf_env_bool(b"PERF_CONFIG_NOGLOBAL\0".as_ptr() as *const c_char, 0) == 0) as c_int
}

unsafe fn home_perfconfig() -> *mut c_char {
    let mut home: *const c_char = ptr::null();
    let mut config: *mut c_char;
    let mut st: stat = zeroed();
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    home = getenv(b"HOME\0".as_ptr() as *const c_char);

    /*
     * Skip reading user config if:
     *   - there is no place to read it from (HOME)
     *   - we are asked not to (PERF_CONFIG_NOGLOBAL=1)
     */
    if home.is_null() || *home == 0 || perf_config_global() == 0 {
        return ptr::null_mut();
    }

    config = strdup(mkpath(
        path.as_mut_ptr(),
        path.len(),
        b"%s/.perfconfig\0".as_ptr() as *const c_char,
        home,
    ));
    if config.is_null() {
        pr_warning(
            b"Not enough memory to process %s/.perfconfig, ignoring it.\n\0".as_ptr()
                as *const c_char,
            home,
        );
        return ptr::null_mut();
    }

    if stat(config, &mut st) < 0 {
        free(config as *mut c_void);
        return ptr::null_mut();
    }

    if st.st_uid != 0 && st.st_uid != geteuid() {
        pr_warning(
            b"File %s not owned by current user or root, ignoring it.\n\0".as_ptr()
                as *const c_char,
            config,
        );
        free(config as *mut c_void);
        return ptr::null_mut();
    }

    if st.st_size != 0 {
        return config;
    }

    free(config as *mut c_void);
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_home_perfconfig() -> *const c_char {
    static mut CONFIG: *const c_char = ptr::null();
    static mut FAILED: bool = false;

    if FAILED || !CONFIG.is_null() {
        return CONFIG;
    }

    CONFIG = home_perfconfig();
    if CONFIG.is_null() {
        FAILED = true;
    }

    CONFIG
}

unsafe fn section_from_node(node: *mut list_head) -> *mut perf_config_section {
    (node as *mut u8).sub(offset_of!(perf_config_section, node)) as *mut perf_config_section
}

unsafe fn item_from_node(node: *mut list_head) -> *mut perf_config_item {
    (node as *mut u8).sub(offset_of!(perf_config_item, node)) as *mut perf_config_item
}

unsafe fn find_section(
    sections: *mut list_head,
    section_name: *const c_char,
) -> *mut perf_config_section {
    let mut pos = (*sections).next;
    while pos != sections {
        let section = section_from_node(pos);
        if strcmp((*section).name, section_name) == 0 {
            return section;
        }
        pos = (*pos).next;
    }

    ptr::null_mut()
}

unsafe fn find_config_item(
    name: *const c_char,
    section: *mut perf_config_section,
) -> *mut perf_config_item {
    let head = &mut (*section).items as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let item = item_from_node(pos);
        if strcmp((*item).name, name) == 0 {
            return item;
        }
        pos = (*pos).next;
    }

    ptr::null_mut()
}

unsafe fn add_section(
    sections: *mut list_head,
    section_name: *const c_char,
) -> *mut perf_config_section {
    let section = zalloc(core::mem::size_of::<perf_config_section>()) as *mut perf_config_section;

    if section.is_null() {
        return ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*section).items);
    (*section).name = strdup(section_name);
    if (*section).name.is_null() {
        pr_debug(b"%s: strdup failed\n\0".as_ptr() as *const c_char, b"add_section\0".as_ptr());
        free(section as *mut c_void);
        return ptr::null_mut();
    }

    list_add_tail(&mut (*section).node, sections);
    section
}

unsafe fn add_config_item(
    section: *mut perf_config_section,
    name: *const c_char,
) -> *mut perf_config_item {
    let item = zalloc(core::mem::size_of::<perf_config_item>()) as *mut perf_config_item;

    if item.is_null() {
        return ptr::null_mut();
    }

    (*item).name = strdup(name);
    if (*item).name.is_null() {
        pr_debug(
            b"%s: strdup failed\n\0".as_ptr() as *const c_char,
            b"add_config_item\0".as_ptr(),
        );
        free(item as *mut c_void);
        return ptr::null_mut();
    }

    list_add_tail(&mut (*item).node, &mut (*section).items);
    item
}

unsafe fn set_value(item: *mut perf_config_item, value: *const c_char) -> c_int {
    let val = strdup(value);

    if val.is_null() {
        return -1;
    }

    zfree(&mut (*item).value);
    (*item).value = val;
    0
}

unsafe extern "C" fn collect_config(
    var: *const c_char,
    value: *const c_char,
    perf_config_set: *mut c_void,
) -> c_int {
    let mut ret = -1;
    let mut ptr_: *mut c_char;
    let key: *mut c_char;
    let section_name: *mut c_char;
    let name: *mut c_char;
    let mut section: *mut perf_config_section = ptr::null_mut();
    let mut item: *mut perf_config_item = ptr::null_mut();
    let set = perf_config_set as *mut perf_config_set;
    let sections: *mut list_head;

    if set.is_null() {
        return -1;
    }

    key = strdup(var);
    ptr_ = key;
    if key.is_null() {
        pr_debug(
            b"%s: strdup failed\n\0".as_ptr() as *const c_char,
            b"collect_config\0".as_ptr(),
        );
        return -1;
    }

    sections = &mut (*set).sections;
    section_name = strsep(&mut ptr_, b".\0".as_ptr() as *const c_char);
    name = ptr_;
    if name.is_null() || value.is_null() {
        free(key as *mut c_void);
        return ret;
    }

    section = find_section(sections, section_name);
    if section.is_null() {
        section = add_section(sections, section_name);
        if section.is_null() {
            free(key as *mut c_void);
            return ret;
        }
    }

    item = find_config_item(name, section);
    if item.is_null() {
        item = add_config_item(section, name);
        if item.is_null() {
            free(key as *mut c_void);
            return ret;
        }
    }

    /* perf_config_set can contain both user and system config items.
     * So we should know where each value is from.
     * The classification would be needed when a particular config file
     * is overwritten by setting feature i.e. set_config().
     */
    if strcmp(config_file_name, perf_etc_perfconfig()) == 0 {
        (*section).from_system_config = true;
        (*item).from_system_config = true;
    } else {
        (*section).from_system_config = false;
        (*item).from_system_config = false;
    }

    ret = set_value(item, value);

    free(key as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_set__collect(
    set: *mut perf_config_set,
    file_name: *const c_char,
    var: *const c_char,
    value: *const c_char,
) -> c_int {
    config_file_name = file_name;
    collect_config(var, value, set as *mut c_void)
}

unsafe fn perf_config_set__init(set: *mut perf_config_set) -> c_int {
    let ret = -1;

    /* Setting $PERF_CONFIG makes perf read _only_ the given config file. */
    if !config_exclusive_filename.is_null() {
        return perf_config_from_file(Some(collect_config), config_exclusive_filename, set as *mut c_void);
    }
    if perf_config_system() != 0 && access(perf_etc_perfconfig(), R_OK) == 0 {
        if perf_config_from_file(Some(collect_config), perf_etc_perfconfig(), set as *mut c_void) < 0 {
            return ret;
        }
    }
    if perf_config_global() != 0 && !perf_home_perfconfig().is_null() {
        if perf_config_from_file(Some(collect_config), perf_home_perfconfig(), set as *mut c_void) < 0 {
            return ret;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_set__new() -> *mut perf_config_set {
    let set = zalloc(core::mem::size_of::<perf_config_set>()) as *mut perf_config_set;

    if !set.is_null() {
        INIT_LIST_HEAD(&mut (*set).sections);
        perf_config_set__init(set);
    }

    set
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_set__load_file(file: *const c_char) -> *mut perf_config_set {
    let set = zalloc(core::mem::size_of::<perf_config_set>()) as *mut perf_config_set;

    if !set.is_null() {
        INIT_LIST_HEAD(&mut (*set).sections);
        perf_config_from_file(Some(collect_config), file, set as *mut c_void);
    }

    set
}

unsafe fn perf_config__init() -> c_int {
    if config_set.is_null() {
        config_set = perf_config_set__new();
    }

    config_set.is_null() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_set(
    set: *mut perf_config_set,
    fn_: config_fn_t,
    data: *mut c_void,
) -> c_int {
    let mut ret = 0;
    let mut key: [c_char; BUFSIZ] = [0; BUFSIZ];

    let sections = &mut (*set).sections as *mut list_head;
    let mut spos = (*sections).next;
    while spos != sections {
        let section = section_from_node(spos);
        let items = &mut (*section).items as *mut list_head;
        let mut ipos = (*items).next;
        while ipos != items {
            let item = item_from_node(ipos);
            let value = (*item).value;

            if !value.is_null() {
                scnprintf(
                    key.as_mut_ptr(),
                    key.len(),
                    b"%s.%s\0".as_ptr() as *const c_char,
                    (*section).name,
                    (*item).name,
                );
                ret = fn_.unwrap()(key.as_mut_ptr(), value, data);
                if ret < 0 {
                    pr_err(
                        b"Error in the given config file: wrong config key-value pair %s=%s\n\0"
                            .as_ptr() as *const c_char,
                        key.as_mut_ptr(),
                        value,
                    );
                    /*
                     * Can't be just a 'break', as perf_config_set__for_each_entry()
                     * expands to two nested for() loops.
                     */
                    return ret;
                }
            }
            ipos = (*ipos).next;
        }
        spos = (*spos).next;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_config(fn_: config_fn_t, data: *mut c_void) -> c_int {
    if config_set.is_null() && perf_config__init() != 0 {
        return -1;
    }

    perf_config_set(config_set, fn_, data)
}

#[no_mangle]
pub unsafe extern "C" fn perf_config__exit() {
    perf_config_set__delete(config_set);
    config_set = ptr::null_mut();
}

unsafe fn perf_config_item__delete(item: *mut perf_config_item) {
    zfree(&mut (*item).name);
    zfree(&mut (*item).value);
    free(item as *mut c_void);
}

unsafe fn perf_config_section__purge(section: *mut perf_config_section) {
    let head = &mut (*section).items as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let item = item_from_node(pos);
        let next = (*pos).next;
        list_del_init(&mut (*item).node);
        perf_config_item__delete(item);
        pos = next;
    }
}

unsafe fn perf_config_section__delete(section: *mut perf_config_section) {
    perf_config_section__purge(section);
    zfree(&mut (*section).name);
    free(section as *mut c_void);
}

unsafe fn perf_config_set__purge(set: *mut perf_config_set) {
    let head = &mut (*set).sections as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let section = section_from_node(pos);
        let next = (*pos).next;
        list_del_init(&mut (*section).node);
        perf_config_section__delete(section);
        pos = next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_set__delete(set: *mut perf_config_set) {
    if set.is_null() {
        return;
    }

    perf_config_set__purge(set);
    free(set as *mut c_void);
}

/*
 * Call this to report error for your variable that should not
 * get a boolean value (i.e. "[my] var" means "true").
 */
#[no_mangle]
pub unsafe extern "C" fn config_error_nonbool(var: *const c_char) -> c_int {
    pr_err(b"Missing value for '%s'\0".as_ptr() as *const c_char, var);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn set_buildid_dir(dir: *const c_char) {
    if !dir.is_null() {
        scnprintf(
            buildid_dir.as_mut_ptr(),
            MAXPATHLEN,
            b"%s\0".as_ptr() as *const c_char,
            dir,
        );
    }

    /* default to $HOME/.debug */
    if buildid_dir[0] == 0 {
        let home = getenv(b"HOME\0".as_ptr() as *const c_char);

        if !home.is_null() {
            snprintf(
                buildid_dir.as_mut_ptr(),
                MAXPATHLEN,
                b"%s/%s\0".as_ptr() as *const c_char,
                home,
                DEBUG_CACHE_DIR.as_ptr(),
            );
        } else {
            strncpy(
                buildid_dir.as_mut_ptr(),
                DEBUG_CACHE_DIR.as_ptr() as *const c_char,
                MAXPATHLEN - 1,
            );
        }
        buildid_dir[MAXPATHLEN - 1] = 0;
    }
    /* for communicating with external commands */
    setenv(
        b"PERF_BUILDID_DIR\0".as_ptr() as *const c_char,
        buildid_dir.as_ptr(),
        1,
    );
}

#[repr(C)]
struct perf_config_scan_data {
    name: *const c_char,
    fmt: *const c_char,
    value: *const c_char,
    args: va_list,
    ret: c_int,
}

unsafe extern "C" fn perf_config_scan_cb(
    var: *const c_char,
    value: *const c_char,
    data: *mut c_void,
) -> c_int {
    let d = data as *mut perf_config_scan_data;

    if strcmp(var, (*d).name) == 0 {
        (*d).ret = vsscanf(value, (*d).fmt, (*d).args);
    }

    0
}

/* Rust has no stable direct equivalent for defining this C variadic function.
 * The item below preserves the externally visible interface and source-level
 * control flow intent; a target integration can map `args` to the platform
 * `va_list` representation used by the surrounding C ABI.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_config_scan_with_va_list(
    name: *const c_char,
    fmt: *const c_char,
    args: va_list,
) -> c_int {
    let mut d = perf_config_scan_data {
        name,
        fmt,
        value: ptr::null(),
        args,
        ret: 0,
    };

    perf_config(Some(perf_config_scan_cb), &mut d as *mut _ as *mut c_void);

    d.ret
}

unsafe extern "C" fn perf_config_get_cb(
    var: *const c_char,
    value: *const c_char,
    data: *mut c_void,
) -> c_int {
    let d = data as *mut perf_config_scan_data;

    if strcmp(var, (*d).name) == 0 {
        (*d).value = value;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_config_get(name: *const c_char) -> *const c_char {
    let mut d = perf_config_scan_data {
        name,
        fmt: ptr::null(),
        value: ptr::null(),
        args: ptr::null_mut(),
        ret: 0,
    };

    perf_config(Some(perf_config_get_cb), &mut d as *mut _ as *mut c_void);
    d.value
}
