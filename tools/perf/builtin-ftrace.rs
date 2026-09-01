// SPDX-License-Identifier: GPL-2.0-only
/*
 * builtin-ftrace.c
 *
 * Copyright (c) 2013  LG Electronics,  Namhyung Kim <namhyung@kernel.org>
 * Copyright (c) 2020  Changbin Du <changbin.du@gmail.com>, significant enhancement.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type sig_atomic_t = c_int;
type FILE = c_void;

const DEFAULT_TRACER: *const c_char = b"function_graph\0".as_ptr() as *const c_char;
const PATH_MAX: usize = 4096;
const CAP_PERFMON: c_int = 38;
const CAP_SYS_ADMIN: c_int = 21;
const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_TRUNC: c_int = 0o1000;
const O_APPEND: c_int = 0o2000;
const O_NONBLOCK: c_int = 0o4000;
const F_SETFL: c_int = 4;
const POLLIN: c_short = 0x0001;
const SIGINT: c_int = 2;
const SIGUSR1: c_int = 10;
const SIGCHLD: c_int = 17;
const SIGPIPE: c_int = 13;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOTSUP: c_int = 95;
const NUM_BUCKET: c_uint = 22;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct filter_entry {
    pub list: list_head,
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct stats {
    pub n: c_double,
    pub mean: c_double,
    pub M2: c_double,
    pub min: u64,
    pub max: u64,
}

#[repr(C)]
pub struct target {
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub system_wide: bool,
    pub cpu_list: *const c_char,
    pub initial_delay: c_int,
    pub use_bpf: bool,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist_core {
    pub threads: *mut perf_thread_map,
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pkey: *mut c_void,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct strfilter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct parse_tag {
    pub tag: c_char,
    pub mult: c_ulong,
}

#[repr(C)]
pub struct sublevel_option {
    pub name: *const c_char,
    pub value_ptr: *mut c_void,
}

#[repr(C)]
pub struct io {
    pub eof: bool,
    pub timeout_ms: c_int,
    _rest: [usize; 8],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_value: sigval,
}

#[repr(C)]
pub struct perf_ftrace {
    pub tracer: *const c_char,
    pub target: target,
    pub evlist: *mut evlist,
    pub filters: list_head,
    pub notrace: list_head,
    pub graph_funcs: list_head,
    pub nograph_funcs: list_head,
    pub event_pair: list_head,
    pub func_stack_trace: bool,
    pub func_irq_info: bool,
    pub graph_depth: c_int,
    pub percpu_buffer_size: c_ulong,
    pub inherit: bool,
    pub graph_nosleep_time: bool,
    pub graph_args: bool,
    pub graph_retval: bool,
    pub graph_retval_hex: bool,
    pub graph_retaddr: bool,
    pub graph_noirqs: bool,
    pub graph_verbose: bool,
    pub graph_tail: bool,
    pub graph_thresh: c_int,
    pub min_latency: c_int,
    pub max_latency: c_int,
    pub bucket_num: c_uint,
    pub bucket_range: c_uint,
    pub use_nsec: bool,
    pub hide_empty: bool,
    pub profile_hash: *mut hashmap,
}

#[repr(C)]
pub struct ftrace_profile_data {
    pub st: stats,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut verbose: c_int;

    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn isalnum(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn log2(x: c_double) -> c_double;
    fn malloc(size: size_t) -> *mut c_void;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn snprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn usleep(usec: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    fn cpu_map__snprint_mask(cpumap: *mut perf_cpu_map, buf: *mut c_char, size: size_t) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__prepare_workload(evlist: *mut evlist, target: *mut target, argv: *mut *const c_char,
                                pipe_output: bool,
                                exec_error: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)) -> c_int;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__start_workload(evlist: *mut evlist);
    fn get_tracing_file(name: *const c_char) -> *mut c_char;
    fn hashmap__add(map: *mut hashmap, key: *mut c_void, value: *mut c_void) -> c_int;
    fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut *mut ftrace_profile_data) -> bool;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__new(hash: unsafe extern "C" fn(c_long, *mut c_void) -> size_t,
                    equal: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool,
                    ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__size(map: *mut hashmap) -> size_t;
    fn init_stats(stats: *mut stats);
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> c_int;
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, len: size_t);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn parse_options(argc: c_int, argv: *mut *const c_char, options: *const option,
                     usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn parse_options_usage(usagestr: *const *const c_char, options: *const option,
                           optstr: *const c_char, short_opt: c_int);
    fn parse_tag_value(str_: *const c_char, tags: *mut parse_tag) -> c_ulong;
    fn perf_cap__capable(cap: c_int) -> bool;
    fn perf_config(fn_: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int,
                   data: *mut c_void) -> c_int;
    fn perf_cpu_map__cpu(cpumap: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpumap: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__put(cpumap: *mut perf_cpu_map);
    fn perf_ftrace__latency_cleanup_bpf(ftrace: *mut perf_ftrace) -> c_int;
    fn perf_ftrace__latency_prepare_bpf(ftrace: *mut perf_ftrace) -> c_int;
    fn perf_ftrace__latency_read_bpf(ftrace: *mut perf_ftrace, buckets: *mut c_int, st: *mut stats) -> c_int;
    fn perf_ftrace__latency_start_bpf(ftrace: *mut perf_ftrace) -> c_int;
    fn perf_ftrace__latency_stop_bpf(ftrace: *mut perf_ftrace) -> c_int;
    fn perf_parse_sublevel_options(str_: *const c_char, opts: *mut sublevel_option) -> c_int;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn put_tracing_file(file: *mut c_char);
    fn setup_pager();
    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *const c_char;
    fn str_hash(str_: *mut c_char) -> size_t;
    fn strfilter__compare(filter: *mut strfilter, str_: *mut c_char) -> bool;
    fn strfilter__delete(filter: *mut strfilter);
    fn strfilter__new(rules: *const c_char, err: *mut *const c_char) -> *mut strfilter;
    fn strfilter__or(filter: *mut strfilter, rules: *const c_char, err: *mut *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn target__has_cpu(target: *mut target) -> bool;
    fn target__none(target: *mut target) -> bool;
    fn target__strerror(target: *mut target, errnum: c_int, buf: *mut c_char, buflen: size_t);
    fn target__validate(target: *mut target) -> c_int;
    fn update_stats(stats: *mut stats, val: c_double);
    fn zalloc(size: size_t) -> *mut c_void;
}

static mut workload_exec_errno: sig_atomic_t = 0;
static mut done: sig_atomic_t = 0;
static mut latency_stats: stats = stats { n: 0.0, mean: 0.0, M2: 0.0, min: 0, max: 0 };
static mut tracing_instance: [c_char; PATH_MAX] = [0; PATH_MAX];
static mut profile_sort: perf_ftrace_profile_sort_key = perf_ftrace_profile_sort_key::PFP_SORT_TOTAL;

unsafe extern "C" fn sig_handler(_sig: c_int) {
    done = 1;
}

/*
 * evlist__prepare_workload will send a SIGUSR1 if the fork fails, since
 * we asked by setting its exec_error to the function below,
 * ftrace__workload_exec_failed_signal.
 *
 * XXX We need to handle this more appropriately, emitting an error, etc.
 */
unsafe extern "C" fn ftrace__workload_exec_failed_signal(
    _signo: c_int,
    info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    workload_exec_errno = (*info).si_value.sival_int;
    done = 1;
}

unsafe fn check_ftrace_capable() -> bool {
    if perf_cap__capable(CAP_PERFMON) || perf_cap__capable(CAP_SYS_ADMIN) {
        return true;
    }
    pr_err(b"ftrace only works for users with the CAP_PERFMON or CAP_SYS_ADMIN capability!\n\0".as_ptr() as *const c_char);
    false
}

unsafe fn is_ftrace_supported() -> bool {
    let file = get_tracing_file(b"set_ftrace_pid\0".as_ptr() as *const c_char);
    if file.is_null() {
        pr_debug(b"cannot get tracing file set_ftrace_pid\n\0".as_ptr() as *const c_char);
        return false;
    }
    let supported = access(file, F_OK) == 0;
    put_tracing_file(file);
    supported
}

/*
 * Wrapper to test if a file in directory .../tracing/instances/XXX
 * exists. If so return the .../tracing/instances/XXX file for use.
 * Otherwise the file exists only in directory .../tracing and
 * is applicable to all instances, for example file available_filter_functions.
 * Return that file name in this case.
 *
 * This functions works similar to get_tracing_file() and expects its caller
 * to free the returned file name.
 *
 * The global variable tracing_instance is set in init_tracing_instance()
 * called at the  beginning to a process specific tracing subdirectory.
 */
unsafe fn get_tracing_instance_file(name: *const c_char) -> *mut c_char {
    let mut file: *mut c_char = null_mut();
    if asprintf(&mut file, b"%s/%s\0".as_ptr() as *const c_char, tracing_instance.as_ptr(), name) < 0 {
        return null_mut();
    }
    if access(file, F_OK) == 0 {
        return file;
    }
    free(file as *mut c_void);
    get_tracing_file(name)
}

unsafe fn __write_tracing_file(name: *const c_char, val: *const c_char, append: bool) -> c_int {
    let mut ret = -1;
    let size = strlen(val);
    let mut flags = O_WRONLY;
    let mut errbuf = [0 as c_char; 512];
    let file = get_tracing_instance_file(name);
    if file.is_null() {
        pr_debug(b"cannot get tracing file: %s\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    if append { flags |= O_APPEND; } else { flags |= O_TRUNC; }
    let fd = open(file, flags);
    if fd < 0 {
        pr_debug(b"cannot open tracing file: %s: %s\n\0".as_ptr() as *const c_char,
                 name, str_error_r(errno, errbuf.as_mut_ptr(), errbuf.len()));
        put_tracing_file(file);
        return ret;
    }
    /*
     * Copy the original value and append a '\n'. Without this,
     * the kernel can hide possible errors.
     */
    let val_copy = strdup(val);
    if !val_copy.is_null() {
        *val_copy.add(size) = b'\n' as c_char;
        if write(fd, val_copy as *const c_void, size + 1) == (size + 1) as ssize_t {
            ret = 0;
        } else {
            pr_debug(b"write '%s' to tracing/%s failed: %s\n\0".as_ptr() as *const c_char,
                     val, name, str_error_r(errno, errbuf.as_mut_ptr(), errbuf.len()));
        }
        free(val_copy as *mut c_void);
    }
    close(fd);
    put_tracing_file(file);
    ret
}

unsafe fn write_tracing_file(name: *const c_char, val: *const c_char) -> c_int {
    __write_tracing_file(name, val, false)
}

unsafe fn append_tracing_file(name: *const c_char, val: *const c_char) -> c_int {
    __write_tracing_file(name, val, true)
}

unsafe fn read_tracing_file_to_stdout(name: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut ret = -1;
    let file = get_tracing_instance_file(name);
    if file.is_null() {
        pr_debug(b"cannot get tracing file: %s\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    let fd = open(file, O_RDONLY);
    if fd < 0 {
        pr_debug(b"cannot open tracing file: %s: %s\n\0".as_ptr() as *const c_char,
                 name, str_error_r(errno, buf.as_mut_ptr(), buf.len()));
        put_tracing_file(file);
        return ret;
    }
    loop {
        let n = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if n == 0 { break; } else if n < 0 { close(fd); put_tracing_file(file); return ret; }
        if fwrite(buf.as_ptr() as *const c_void, n as size_t, 1, stdout) != 1 {
            close(fd); put_tracing_file(file); return ret;
        }
    }
    ret = 0;
    close(fd);
    put_tracing_file(file);
    ret
}

unsafe fn read_tracing_file_by_line(
    name: *const c_char,
    cb: unsafe extern "C" fn(*mut c_char, *mut c_void),
    cb_arg: *mut c_void,
) -> c_int {
    let mut line: *mut c_char = null_mut();
    let mut len: size_t = 0;
    let file = get_tracing_instance_file(name);
    if file.is_null() {
        pr_debug(b"cannot get tracing file: %s\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    let fp = fopen(file, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        pr_debug(b"cannot open tracing file: %s\n\0".as_ptr() as *const c_char, name);
        put_tracing_file(file);
        return -1;
    }
    while getline(&mut line, &mut len, fp) != -1 {
        cb(line, cb_arg);
    }
    if !line.is_null() { free(line as *mut c_void); }
    fclose(fp);
    put_tracing_file(file);
    0
}

unsafe fn write_tracing_file_int(name: *const c_char, value: c_int) -> c_int {
    let mut buf = [0 as c_char; 16];
    snprintf(buf.as_mut_ptr(), buf.len(), b"%d\0".as_ptr() as *const c_char, value);
    if write_tracing_file(name, buf.as_ptr()) < 0 { return -1; }
    0
}

unsafe fn write_tracing_option_file(name: *const c_char, val: *const c_char) -> c_int {
    let mut file: *mut c_char = null_mut();
    if asprintf(&mut file, b"options/%s\0".as_ptr() as *const c_char, name) < 0 {
        return -1;
    }
    let ret = __write_tracing_file(file, val, false);
    free(file as *mut c_void);
    ret
}

unsafe fn reset_tracing_options(_ftrace: *mut perf_ftrace) {
    write_tracing_option_file(b"function-fork\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"func_stack_trace\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"sleep-time\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-irqs\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-proc\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-abstime\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-tail\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-args\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-retval\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-retval-hex\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"funcgraph-retaddr\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"latency-format\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    write_tracing_option_file(b"irq-info\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
}

unsafe fn reset_tracing_files(ftrace: *mut perf_ftrace) -> c_int {
    if write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    if write_tracing_file(b"current_tracer\0".as_ptr() as *const c_char, b"nop\0".as_ptr() as *const c_char) < 0 { return -1; }
    if write_tracing_file(b"set_ftrace_pid\0".as_ptr() as *const c_char, b" \0".as_ptr() as *const c_char) < 0 { return -1; }
    if reset_tracing_cpu() < 0 { return -1; }
    if write_tracing_file(b"max_graph_depth\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    if write_tracing_file(b"tracing_thresh\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    reset_tracing_filters();
    reset_tracing_options(ftrace);
    0
}

/* Remove .../tracing/instances/XXX subdirectory created with
 * init_tracing_instance().
 */
unsafe fn exit_tracing_instance() {
    if rmdir(tracing_instance.as_ptr()) != 0 {
        pr_err(b"failed to delete tracing/instances directory\n\0".as_ptr() as *const c_char);
    }
}

/* Create subdirectory within .../tracing/instances/XXX to have session
 * or process specific setup. To delete this setup, simply remove the
 * subdirectory.
 */
unsafe fn init_tracing_instance() -> c_int {
    let dirname = b"instances/perf-ftrace-XXXXXX\0";
    let path = get_tracing_file(dirname.as_ptr() as *const c_char);
    if path.is_null() {
        pr_err(b"failed to create tracing/instances directory\n\0".as_ptr() as *const c_char);
        return -1;
    }
    strncpy(tracing_instance.as_mut_ptr(), path, PATH_MAX - 1);
    put_tracing_file(path);
    if mkdtemp(tracing_instance.as_mut_ptr()).is_null() {
        pr_err(b"failed to create tracing/instances directory\n\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

unsafe fn set_tracing_pid(ftrace: *mut perf_ftrace) -> c_int {
    let mut buf = [0 as c_char; 16];
    if target__has_cpu(&mut (*ftrace).target) { return 0; }
    let threads = (*evlist__core((*ftrace).evlist)).threads;
    let mut i = 0;
    while i < perf_thread_map__nr(threads) {
        scnprintf(buf.as_mut_ptr(), buf.len(), b"%d\0".as_ptr() as *const c_char,
                  perf_thread_map__pid(threads, i));
        if append_tracing_file(b"set_ftrace_pid\0".as_ptr() as *const c_char, buf.as_ptr()) < 0 {
            return -1;
        }
        i += 1;
    }
    0
}

unsafe fn set_tracing_cpumask(cpumap: *mut perf_cpu_map) -> c_int {
    let last_cpu = perf_cpu_map__cpu(cpumap, perf_cpu_map__nr(cpumap) - 1).cpu;
    let mut mask_size = (last_cpu / 4 + 2) as size_t; /* one more byte for EOS */
    mask_size += (last_cpu / 32) as size_t; /* ',' is needed for every 32th cpus */
    let cpumask = malloc(mask_size) as *mut c_char;
    if cpumask.is_null() {
        pr_debug(b"failed to allocate cpu mask\n\0".as_ptr() as *const c_char);
        return -1;
    }
    cpu_map__snprint_mask(cpumap, cpumask, mask_size);
    let ret = write_tracing_file(b"tracing_cpumask\0".as_ptr() as *const c_char, cpumask);
    free(cpumask as *mut c_void);
    ret
}

unsafe fn set_tracing_cpu(ftrace: *mut perf_ftrace) -> c_int {
    let cpumap = (*evlist__core((*ftrace).evlist)).user_requested_cpus;
    if !target__has_cpu(&mut (*ftrace).target) { return 0; }
    set_tracing_cpumask(cpumap)
}

unsafe fn set_tracing_func_stack_trace(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).func_stack_trace { return 0; }
    if write_tracing_option_file(b"func_stack_trace\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_func_irqinfo(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).func_irq_info { return 0; }
    if write_tracing_option_file(b"irq-info\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn reset_tracing_cpu() -> c_int {
    let cpumap = perf_cpu_map__new_online_cpus();
    let ret = set_tracing_cpumask(cpumap);
    perf_cpu_map__put(cpumap);
    ret
}

unsafe fn list_for_each_filter(mut head: *mut list_head, mut cb: impl FnMut(*mut filter_entry) -> c_int) -> c_int {
    let mut pos = (*head).next;
    while pos != head {
        let entry = pos as *mut filter_entry;
        let ret = cb(entry);
        if ret < 0 { return ret; }
        pos = (*pos).next;
    }
    0
}

unsafe fn __set_tracing_filter(filter_file: *const c_char, funcs: *mut list_head) -> c_int {
    list_for_each_filter(funcs, |pos| {
        if append_tracing_file(filter_file, (*pos).name.as_ptr()) < 0 { -1 } else { 0 }
    })
}

unsafe fn set_tracing_filters(ftrace: *mut perf_ftrace) -> c_int {
    let mut ret = __set_tracing_filter(b"set_ftrace_filter\0".as_ptr() as *const c_char, &mut (*ftrace).filters);
    if ret < 0 { return ret; }
    ret = __set_tracing_filter(b"set_ftrace_notrace\0".as_ptr() as *const c_char, &mut (*ftrace).notrace);
    if ret < 0 { return ret; }
    ret = __set_tracing_filter(b"set_graph_function\0".as_ptr() as *const c_char, &mut (*ftrace).graph_funcs);
    if ret < 0 { return ret; }
    /* old kernels do not have this filter */
    __set_tracing_filter(b"set_graph_notrace\0".as_ptr() as *const c_char, &mut (*ftrace).nograph_funcs);
    ret
}

unsafe fn reset_tracing_filters() {
    write_tracing_file(b"set_ftrace_filter\0".as_ptr() as *const c_char, b" \0".as_ptr() as *const c_char);
    write_tracing_file(b"set_ftrace_notrace\0".as_ptr() as *const c_char, b" \0".as_ptr() as *const c_char);
    write_tracing_file(b"set_graph_function\0".as_ptr() as *const c_char, b" \0".as_ptr() as *const c_char);
    write_tracing_file(b"set_graph_notrace\0".as_ptr() as *const c_char, b" \0".as_ptr() as *const c_char);
}

unsafe fn set_tracing_depth(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).graph_depth == 0 { return 0; }
    if (*ftrace).graph_depth < 0 {
        pr_err(b"invalid graph depth: %d\n\0".as_ptr() as *const c_char, (*ftrace).graph_depth);
        return -1;
    }
    if write_tracing_file_int(b"max_graph_depth\0".as_ptr() as *const c_char, (*ftrace).graph_depth) < 0 { return -1; }
    0
}

unsafe fn set_tracing_percpu_buffer_size(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).percpu_buffer_size == 0 { return 0; }
    write_tracing_file_int(b"buffer_size_kb\0".as_ptr() as *const c_char,
                           ((*ftrace).percpu_buffer_size / 1024) as c_int)
}

unsafe fn set_tracing_trace_inherit(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).inherit { return 0; }
    if write_tracing_option_file(b"function-fork\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_sleep_time(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).graph_nosleep_time { return 0; }
    if write_tracing_option_file(b"sleep-time\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_args(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).graph_args && write_tracing_option_file(b"funcgraph-args\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_retval(ftrace: *mut perf_ftrace) -> c_int {
    if ((*ftrace).graph_retval || (*ftrace).graph_retval_hex) &&
        write_tracing_option_file(b"funcgraph-retval\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    if (*ftrace).graph_retval_hex &&
        write_tracing_option_file(b"funcgraph-retval-hex\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_retaddr(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).graph_retaddr && write_tracing_option_file(b"funcgraph-retaddr\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_irqs(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).graph_noirqs { return 0; }
    if write_tracing_option_file(b"funcgraph-irqs\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_verbose(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).graph_verbose { return 0; }
    if write_tracing_option_file(b"funcgraph-proc\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    if write_tracing_option_file(b"funcgraph-abstime\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    if write_tracing_option_file(b"latency-format\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_funcgraph_tail(ftrace: *mut perf_ftrace) -> c_int {
    if !(*ftrace).graph_tail { return 0; }
    if write_tracing_option_file(b"funcgraph-tail\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 { return -1; }
    0
}

unsafe fn set_tracing_thresh(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).graph_thresh == 0 { return 0; }
    write_tracing_file_int(b"tracing_thresh\0".as_ptr() as *const c_char, (*ftrace).graph_thresh)
}

unsafe fn set_tracing_options(ftrace: *mut perf_ftrace) -> c_int {
    if set_tracing_pid(ftrace) < 0 { pr_err(b"failed to set ftrace pid\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_cpu(ftrace) < 0 { pr_err(b"failed to set tracing cpumask\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_func_stack_trace(ftrace) < 0 { pr_err(b"failed to set tracing option func_stack_trace\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_func_irqinfo(ftrace) < 0 { pr_err(b"failed to set tracing option irq-info\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_filters(ftrace) < 0 { pr_err(b"failed to set tracing filters\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_depth(ftrace) < 0 { pr_err(b"failed to set graph depth\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_percpu_buffer_size(ftrace) < 0 { pr_err(b"failed to set tracing per-cpu buffer size\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_trace_inherit(ftrace) < 0 { pr_err(b"failed to set tracing option function-fork\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_sleep_time(ftrace) < 0 { pr_err(b"failed to set tracing option sleep-time\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_args(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-args\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_retval(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-retval\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_retaddr(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-retaddr\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_irqs(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-irqs\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_verbose(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-proc/funcgraph-abstime\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_thresh(ftrace) < 0 { pr_err(b"failed to set tracing thresh\n\0".as_ptr() as *const c_char); return -1; }
    if set_tracing_funcgraph_tail(ftrace) < 0 { pr_err(b"failed to set tracing option funcgraph-tail\n\0".as_ptr() as *const c_char); return -1; }
    0
}

unsafe fn select_tracer(ftrace: *mut perf_ftrace) {
    let graph = !list_empty(&(*ftrace).graph_funcs) || !list_empty(&(*ftrace).nograph_funcs);
    let func = !list_empty(&(*ftrace).filters) || !list_empty(&(*ftrace).notrace);
    /* The function_graph has priority over function tracer. */
    if graph {
        (*ftrace).tracer = b"function_graph\0".as_ptr() as *const c_char;
    } else if func {
        (*ftrace).tracer = b"function\0".as_ptr() as *const c_char;
    }
    /* Otherwise, the default tracer is used. */
    pr_debug(b"%s tracer is used\n\0".as_ptr() as *const c_char, (*ftrace).tracer);
}

unsafe fn __cmd_ftrace(ftrace: *mut perf_ftrace) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut pfd = pollfd { fd: 0, events: POLLIN, revents: 0 };
    select_tracer(ftrace);
    if init_tracing_instance() < 0 { return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if reset_tracing_files(ftrace) < 0 { pr_err(b"failed to reset ftrace\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if write_tracing_file(b"trace\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if set_tracing_options(ftrace) < 0 { exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if write_tracing_file(b"current_tracer\0".as_ptr() as *const c_char, (*ftrace).tracer) < 0 {
        pr_err(b"failed to set current_tracer to %s\n\0".as_ptr() as *const c_char, (*ftrace).tracer);
        exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    setup_pager();
    let trace_file = get_tracing_instance_file(b"trace_pipe\0".as_ptr() as *const c_char);
    if trace_file.is_null() { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    let trace_fd = open(trace_file, O_RDONLY);
    put_tracing_file(trace_file);
    if trace_fd < 0 { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    fcntl(trace_fd, F_SETFL, O_NONBLOCK);
    pfd.fd = trace_fd;
    /* display column headers */
    read_tracing_file_to_stdout(b"trace\0".as_ptr() as *const c_char);
    if (*ftrace).target.initial_delay == 0 &&
        write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 {
        pr_err(b"can't enable tracing\n\0".as_ptr() as *const c_char);
        close(trace_fd); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    evlist__start_workload((*ftrace).evlist);
    if (*ftrace).target.initial_delay > 0 {
        usleep(((*ftrace).target.initial_delay * 1000) as c_uint);
        if write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 {
            pr_err(b"can't enable tracing\n\0".as_ptr() as *const c_char);
            close(trace_fd); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
        }
    }
    while done == 0 {
        if poll(&mut pfd, 1, -1) < 0 { break; }
        if (pfd.revents & POLLIN) != 0 {
            let n = read(trace_fd, buf.as_mut_ptr() as *mut c_void, buf.len());
            if n < 0 { break; }
            if fwrite(buf.as_ptr() as *const c_void, n as size_t, 1, stdout) != 1 { break; }
            /* flush output since stdout is in full buffering mode due to pager */
            fflush(stdout);
        }
    }
    write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    if workload_exec_errno != 0 {
        let emsg = str_error_r(workload_exec_errno, buf.as_mut_ptr(), buf.len());
        /* flush stdout first so below error msg appears at the end. */
        fflush(stdout);
        pr_err(b"workload failed: %s\n\0".as_ptr() as *const c_char, emsg);
        close(trace_fd); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    /* read remaining buffer contents */
    loop {
        let n = read(trace_fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if n <= 0 { break; }
        if fwrite(buf.as_ptr() as *const c_void, n as size_t, 1, stdout) != 1 { break; }
    }
    close(trace_fd);
    exit_tracing_instance();
    if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }
}

unsafe fn make_histogram(ftrace: *mut perf_ftrace, buckets: *mut c_int, buf: *mut c_char, len: size_t, linebuf: *mut c_char) {
    let min_latency = (*ftrace).min_latency;
    let max_latency = (*ftrace).max_latency;
    let bucket_num = (*ftrace).bucket_num;
    /* ensure NUL termination */
    *buf.add(len) = 0;
    /* handle data line by line */
    let mut p = buf;
    loop {
        let q = strchr(p, b'\n' as c_int);
        if q.is_null() { break; }
        *q = 0;
        /* move it to the line buffer */
        strcat(linebuf, p);
        /*
         * parse trace output to get function duration like in
         *
         * # tracer: function_graph
         * #
         * # CPU  DURATION                  FUNCTION CALLS
         * # |     |   |                     |   |   |   |
         *  1) + 10.291 us   |  do_filp_open();
         *  1)   4.889 us    |  do_filp_open();
         *  1)   6.086 us    |  do_filp_open();
         *
         */
        if *linebuf != b'#' as c_char {
            /* ignore CPU */
            let mut r = strchr(linebuf, b')' as c_int);
            if r.is_null() { r = linebuf; }
            while *r != 0 && isdigit(*r as c_int) == 0 && *r != b'|' as c_char { r = r.add(1); }
            /* no duration */
            if *r != 0 && *r != b'|' as c_char {
                let mut unit: *mut c_char = null_mut();
                let mut num = strtod(r, &mut unit);
                if !unit.is_null() && strncmp(unit, b" us\0".as_ptr() as *const c_char, 3) == 0 {
                    if (*ftrace).use_nsec { num *= 1000.0; }
                    let mut i: c_int = 0;
                    if !(num < min_latency as c_double) {
                        num -= min_latency as c_double;
                        if (*ftrace).bucket_range == 0 {
                            i = log2(num) as c_int;
                            if i < 0 { i = 0; }
                        } else {
                            // Less than 1 unit (ms or ns), or, in the future,
                            // than the min latency desired.
                            if num > 0.0 { // 1st entry: [ 1 unit .. bucket_range units ]
                                i = (num / (*ftrace).bucket_range as c_double) as c_int + 1;
                            }
                            if num >= (max_latency - min_latency) as c_double {
                                i = bucket_num as c_int - 1;
                            }
                        }
                        if i as c_uint >= bucket_num { i = bucket_num as c_int - 1; }
                        num += min_latency as c_double;
                    }
                    *buckets.add(i as usize) += 1;
                    update_stats(&mut latency_stats, num);
                }
            }
        }
        /* empty the line buffer for the next output  */
        *linebuf = 0;
        p = q.add(1);
    }
    /* preserve any remaining output (before newline) */
    strcat(linebuf, p);
}

unsafe fn display_histogram(ftrace: *mut perf_ftrace, buckets: *mut c_int) {
    let min_latency = (*ftrace).min_latency;
    let use_nsec = (*ftrace).use_nsec;
    let bucket_num = (*ftrace).bucket_num;
    let mut total: c_int = 0;
    let bar_total: c_int = 46;  /* to fit in 80 column */
    let bar = b"###############################################\0";
    let mut i: c_uint = 0;
    while i < bucket_num {
        total += *buckets.add(i as usize);
        i += 1;
    }
    if total == 0 {
        printf(b"No data found\n\0".as_ptr() as *const c_char);
        return;
    }
    printf(b"# %14s | %10s | %-*s |\n\0".as_ptr() as *const c_char,
           b"  DURATION    \0".as_ptr() as *const c_char, b"COUNT\0".as_ptr() as *const c_char,
           bar_total, b"GRAPH\0".as_ptr() as *const c_char);
    let mut bar_len = *buckets.add(0) * bar_total / total;
    if !(*ftrace).hide_empty || *buckets.add(0) != 0 {
        printf(b"  %4d - %4d %s | %10d | %.*s%*s |\n\0".as_ptr() as *const c_char,
               0, if min_latency != 0 { min_latency } else { 1 },
               if use_nsec { b"ns\0".as_ptr() } else { b"us\0".as_ptr() } as *const c_char,
               *buckets.add(0), bar_len, bar.as_ptr() as *const c_char, bar_total - bar_len, b"\0".as_ptr() as *const c_char);
    }
    i = 1;
    while i < bucket_num - 1 {
        let mut start: c_uint;
        let mut stop: c_uint;
        let mut unit = if use_nsec { b"ns\0".as_ptr() } else { b"us\0".as_ptr() } as *const c_char;
        if (*ftrace).hide_empty && *buckets.add(i as usize) == 0 { i += 1; continue; }
        if (*ftrace).bucket_range == 0 {
            start = 1u32 << (i - 1);
            stop = 1u32 << i;
            if start >= 1024 {
                start >>= 10;
                stop >>= 10;
                unit = if use_nsec { b"us\0".as_ptr() } else { b"ms\0".as_ptr() } as *const c_char;
            }
        } else {
            start = (i - 1) * (*ftrace).bucket_range + min_latency as c_uint;
            stop = i * (*ftrace).bucket_range + min_latency as c_uint;
            if start >= (*ftrace).max_latency as c_uint { break; }
            if stop > (*ftrace).max_latency as c_uint { stop = (*ftrace).max_latency as c_uint; }
            if start >= 1000 {
                let dstart = start as c_double / 1000.0;
                let dstop = stop as c_double / 1000.0;
                printf(b"  %4.2f - %-4.2f\0".as_ptr() as *const c_char, dstart, dstop);
                unit = if use_nsec { b"us\0".as_ptr() } else { b"ms\0".as_ptr() } as *const c_char;
                bar_len = *buckets.add(i as usize) * bar_total / total;
                printf(b" %s | %10d | %.*s%*s |\n\0".as_ptr() as *const c_char, unit, *buckets.add(i as usize), bar_len,
                       bar.as_ptr() as *const c_char, bar_total - bar_len, b"\0".as_ptr() as *const c_char);
                i += 1;
                continue;
            }
        }
        printf(b"  %4d - %4d\0".as_ptr() as *const c_char, start, stop);
        bar_len = *buckets.add(i as usize) * bar_total / total;
        printf(b" %s | %10d | %.*s%*s |\n\0".as_ptr() as *const c_char, unit, *buckets.add(i as usize), bar_len,
               bar.as_ptr() as *const c_char, bar_total - bar_len, b"\0".as_ptr() as *const c_char);
        i += 1;
    }
    bar_len = *buckets.add((bucket_num - 1) as usize) * bar_total / total;
    if !((*ftrace).hide_empty && *buckets.add((bucket_num - 1) as usize) == 0) {
        if (*ftrace).bucket_range == 0 {
            printf(b"  %4d - %-4s %s\0".as_ptr() as *const c_char, 1, b"...\0".as_ptr() as *const c_char,
                   if use_nsec { b"ms\0".as_ptr() } else { b"s \0".as_ptr() } as *const c_char);
        } else {
            let mut upper_outlier = (bucket_num - 2) * (*ftrace).bucket_range + min_latency as c_uint;
            if upper_outlier > (*ftrace).max_latency as c_uint { upper_outlier = (*ftrace).max_latency as c_uint; }
            if upper_outlier >= 1000 {
                let dstart = upper_outlier as c_double / 1000.0;
                printf(b"  %4.2f - %-4s %s\0".as_ptr() as *const c_char, dstart, b"...\0".as_ptr() as *const c_char,
                       if use_nsec { b"us\0".as_ptr() } else { b"ms\0".as_ptr() } as *const c_char);
            } else {
                printf(b"  %4d - %4s %s\0".as_ptr() as *const c_char, upper_outlier, b"...\0".as_ptr() as *const c_char,
                       if use_nsec { b"ns\0".as_ptr() } else { b"us\0".as_ptr() } as *const c_char);
            }
        }
        printf(b" | %10d | %.*s%*s |\n\0".as_ptr() as *const c_char, *buckets.add((bucket_num - 1) as usize),
               bar_len, bar.as_ptr() as *const c_char, bar_total - bar_len, b"\0".as_ptr() as *const c_char);
    }
    printf(b"\n# statistics  (in %s)\n\0".as_ptr() as *const c_char,
           if (*ftrace).use_nsec { b"nsec\0".as_ptr() } else { b"usec\0".as_ptr() } as *const c_char);
    printf(b"  total time: %20.0f\n\0".as_ptr() as *const c_char, latency_stats.mean * latency_stats.n);
    printf(b"    avg time: %20.0f\n\0".as_ptr() as *const c_char, latency_stats.mean);
    printf(b"    max time: %20llu\n\0".as_ptr() as *const c_char, latency_stats.max);
    printf(b"    min time: %20llu\n\0".as_ptr() as *const c_char, latency_stats.min);
    printf(b"       count: %20.0f\n\0".as_ptr() as *const c_char, latency_stats.n);
}

unsafe fn prepare_func_latency(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).target.use_bpf { return perf_ftrace__latency_prepare_bpf(ftrace); }
    if init_tracing_instance() < 0 { return -1; }
    if reset_tracing_files(ftrace) < 0 { pr_err(b"failed to reset ftrace\n\0".as_ptr() as *const c_char); return -1; }
    if write_tracing_file(b"trace\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { return -1; }
    if set_tracing_options(ftrace) < 0 { return -1; }
    /* force to use the function_graph tracer to track duration */
    if write_tracing_file(b"current_tracer\0".as_ptr() as *const c_char, b"function_graph\0".as_ptr() as *const c_char) < 0 {
        pr_err(b"failed to set current_tracer to function_graph\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let trace_file = get_tracing_instance_file(b"trace_pipe\0".as_ptr() as *const c_char);
    if trace_file.is_null() { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); return -1; }
    let fd = open(trace_file, O_RDONLY);
    if fd < 0 { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); }
    init_stats(&mut latency_stats);
    put_tracing_file(trace_file);
    fd
}

unsafe fn start_func_latency(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).target.use_bpf { return perf_ftrace__latency_start_bpf(ftrace); }
    if write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 {
        pr_err(b"can't enable tracing\n\0".as_ptr() as *const c_char);
        return -1;
    }
    0
}

unsafe fn stop_func_latency(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).target.use_bpf { return perf_ftrace__latency_stop_bpf(ftrace); }
    write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    0
}

unsafe fn read_func_latency(ftrace: *mut perf_ftrace, buckets: *mut c_int) -> c_int {
    if (*ftrace).target.use_bpf {
        return perf_ftrace__latency_read_bpf(ftrace, buckets, &mut latency_stats);
    }
    0
}

unsafe fn cleanup_func_latency(ftrace: *mut perf_ftrace) -> c_int {
    if (*ftrace).target.use_bpf { return perf_ftrace__latency_cleanup_bpf(ftrace); }
    exit_tracing_instance();
    0
}

unsafe fn __cmd_latency(ftrace: *mut perf_ftrace) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut line = [0 as c_char; 256];
    let mut pfd = pollfd { fd: 0, events: POLLIN, revents: 0 };
    let trace_fd = prepare_func_latency(ftrace);
    if trace_fd < 0 { close(trace_fd); cleanup_func_latency(ftrace); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    fcntl(trace_fd, F_SETFL, O_NONBLOCK);
    pfd.fd = trace_fd;
    if start_func_latency(ftrace) < 0 { close(trace_fd); cleanup_func_latency(ftrace); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    evlist__start_workload((*ftrace).evlist);
    let buckets = calloc((*ftrace).bucket_num as size_t, size_of::<c_int>()) as *mut c_int;
    if buckets.is_null() {
        pr_err(b"failed to allocate memory for the buckets\n\0".as_ptr() as *const c_char);
        close(trace_fd); cleanup_func_latency(ftrace); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    line[0] = 0;
    while done == 0 {
        if poll(&mut pfd, 1, -1) < 0 { break; }
        if (pfd.revents & POLLIN) != 0 {
            let n = read(trace_fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
            if n < 0 { break; }
            make_histogram(ftrace, buckets, buf.as_mut_ptr(), n as size_t, line.as_mut_ptr());
        }
    }
    stop_func_latency(ftrace);
    if workload_exec_errno != 0 {
        let emsg = str_error_r(workload_exec_errno, buf.as_mut_ptr(), buf.len());
        pr_err(b"workload failed: %s\n\0".as_ptr() as *const c_char, emsg);
        free(buckets as *mut c_void); close(trace_fd); cleanup_func_latency(ftrace);
        return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    /* read remaining buffer contents */
    while !(*ftrace).target.use_bpf {
        let n = read(trace_fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
        if n <= 0 { break; }
        make_histogram(ftrace, buckets, buf.as_mut_ptr(), n as size_t, line.as_mut_ptr());
    }
    read_func_latency(ftrace, buckets);
    display_histogram(ftrace, buckets);
    free(buckets as *mut c_void);
    close(trace_fd);
    cleanup_func_latency(ftrace);
    if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }
}

unsafe extern "C" fn profile_hash(func: c_long, _ctx: *mut c_void) -> size_t {
    str_hash(func as *mut c_char)
}

unsafe extern "C" fn profile_equal(func1: c_long, func2: c_long, _ctx: *mut c_void) -> bool {
    strcmp(func1 as *const c_char, func2 as *const c_char) == 0
}

unsafe fn IS_ERR(ptr: *mut hashmap) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn prepare_func_profile(ftrace: *mut perf_ftrace) -> c_int {
    (*ftrace).tracer = b"function_graph\0".as_ptr() as *const c_char;
    (*ftrace).graph_tail = true;
    (*ftrace).graph_verbose = false;
    (*ftrace).profile_hash = hashmap__new(profile_hash, profile_equal, null_mut());
    if IS_ERR((*ftrace).profile_hash) {
        let err = PTR_ERR((*ftrace).profile_hash);
        (*ftrace).profile_hash = null_mut();
        return err;
    }
    0
}

unsafe fn add_func_duration(ftrace: *mut perf_ftrace, func: *mut c_char, time_ns: c_double) -> c_int {
    let mut prof: *mut ftrace_profile_data = null_mut();
    if !hashmap__find((*ftrace).profile_hash, func, &mut prof) {
        let key = strdup(func);
        if key.is_null() { return -ENOMEM; }
        prof = zalloc(size_of::<ftrace_profile_data>()) as *mut ftrace_profile_data;
        if prof.is_null() {
            free(key as *mut c_void);
            return -ENOMEM;
        }
        init_stats(&mut (*prof).st);
        hashmap__add((*ftrace).profile_hash, key as *mut c_void, prof as *mut c_void);
    }
    update_stats(&mut (*prof).st, time_ns);
    0
}

/*
 * The ftrace function_graph text output normally looks like below:
 *
 * CPU   DURATION       FUNCTION
 *
 *  0)               |  syscall_trace_enter.isra.0() {
 *  0)               |    __audit_syscall_entry() {
 *  0)               |      auditd_test_task() {
 *  0)   0.271 us    |        __rcu_read_lock();
 *  0)   0.275 us    |        __rcu_read_unlock();
 *  0)   1.254 us    |      } /\* auditd_test_task *\/
 *  0)   0.279 us    |      ktime_get_coarse_real_ts64();
 *  0)   2.227 us    |    } /\* __audit_syscall_entry *\/
 *  0)   2.713 us    |  } /\* syscall_trace_enter.isra.0 *\/
 *
 *  Parse the line and get the duration and function name.
 */
unsafe fn parse_func_duration(ftrace: *mut perf_ftrace, line: *mut c_char, len: size_t) -> c_int {
    /* skip CPU */
    let mut p = strchr(line, b')' as c_int);
    if p.is_null() { return 0; }
    /* get duration */
    p = skip_spaces(p.add(1));
    /* no duration? */
    if p.is_null() || *p == b'|' as c_char { return 0; }
    /* skip markers like '*' or '!' for longer than ms */
    if isdigit(*p as c_int) == 0 { p = p.add(1); }
    let mut endp = p;
    let mut duration = strtod(p, &mut endp);
    p = endp;
    if strncmp(p, b" us\0".as_ptr() as *const c_char, 3) != 0 {
        pr_debug(b"non-usec time found.. ignoring\n\0".as_ptr() as *const c_char);
        return 0;
    }
    /*
     * profile stat keeps the max and min values as integer,
     * convert to nsec time so that we can have accurate max.
     */
    duration *= 1000.0;
    /* skip to the pipe */
    while p < line.add(len) && *p != b'|' as c_char { p = p.add(1); }
    if *p != b'|' as c_char { return -EINVAL; }
    p = p.add(1);
    /* get function name */
    let mut func = skip_spaces(p);
    /* skip the closing bracket and the start of comment */
    if *func == b'}' as c_char { func = func.add(5); }
    /* remove semi-colon or end of comment at the end */
    p = line.add(len - 1);
    while isalnum(*p as c_int) == 0 && *p != b']' as c_char {
        *p = 0;
        p = p.sub(1);
    }
    add_func_duration(ftrace, func, duration)
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum perf_ftrace_profile_sort_key {
    PFP_SORT_TOTAL = 0,
    PFP_SORT_AVG,
    PFP_SORT_MAX,
    PFP_SORT_COUNT,
    PFP_SORT_NAME,
}

unsafe extern "C" fn cmp_profile_data(a: *const c_void, b: *const c_void) -> c_int {
    let e1 = *(a as *const *const hashmap_entry);
    let e2 = *(b as *const *const hashmap_entry);
    let p1 = (*e1).pvalue as *mut ftrace_profile_data;
    let p2 = (*e2).pvalue as *mut ftrace_profile_data;
    let (v1, v2) = match profile_sort {
        perf_ftrace_profile_sort_key::PFP_SORT_NAME => {
            return strcmp((*e1).pkey as *const c_char, (*e2).pkey as *const c_char);
        }
        perf_ftrace_profile_sort_key::PFP_SORT_AVG => ((*p1).st.mean, (*p2).st.mean),
        perf_ftrace_profile_sort_key::PFP_SORT_MAX => ((*p1).st.max as c_double, (*p2).st.max as c_double),
        perf_ftrace_profile_sort_key::PFP_SORT_COUNT => ((*p1).st.n, (*p2).st.n),
        perf_ftrace_profile_sort_key::PFP_SORT_TOTAL => ((*p1).st.n * (*p1).st.mean, (*p2).st.n * (*p2).st.mean),
    };
    if v1 > v2 { return -1; }
    if v1 < v2 { return 1; }
    0
}

unsafe fn hashmap_for_each_entry(_map: *mut hashmap, _cb: impl FnMut(*mut hashmap_entry)) {
    /* hashmap__for_each_entry is a C macro supplied by hashmap.h; preserving
     * the iteration intent here requires the external hashmap layout. */
}

unsafe fn print_profile_result(ftrace: *mut perf_ftrace) {
    let nr = hashmap__size((*ftrace).profile_hash);
    if nr == 0 { return; }
    let profile = calloc(nr, size_of::<*mut hashmap_entry>()) as *mut *mut hashmap_entry;
    if profile.is_null() {
        pr_err(b"failed to allocate memory for the result\n\0".as_ptr() as *const c_char);
        return;
    }
    let mut i: size_t = 0;
    hashmap_for_each_entry((*ftrace).profile_hash, |entry| {
        *profile.add(i) = entry;
        i += 1;
    });
    debug_assert!(i == nr);
    //cmp_profile_data(profile[0], profile[1]);
    qsort(profile as *mut c_void, nr, size_of::<*mut hashmap_entry>(), Some(cmp_profile_data));
    printf(b"# %10s %10s %10s %10s   %s\n\0".as_ptr() as *const c_char,
           b"Total (us)\0".as_ptr() as *const c_char, b"Avg (us)\0".as_ptr() as *const c_char,
           b"Max (us)\0".as_ptr() as *const c_char, b"Count\0".as_ptr() as *const c_char,
           b"Function\0".as_ptr() as *const c_char);
    i = 0;
    while i < nr {
        let entry = *profile.add(i);
        let name = (*entry).pkey as *const c_char;
        let p = (*entry).pvalue as *mut ftrace_profile_data;
        printf(b"%12.3f %10.3f %6llu.%03llu %10.0f   %s\n\0".as_ptr() as *const c_char,
               (*p).st.n * (*p).st.mean / 1000.0, (*p).st.mean / 1000.0,
               (*p).st.max / 1000, (*p).st.max % 1000, (*p).st.n, name);
        i += 1;
    }
    free(profile as *mut c_void);
    hashmap_for_each_entry((*ftrace).profile_hash, |entry| {
        free((*entry).pkey);
        free((*entry).pvalue);
    });
    hashmap__free((*ftrace).profile_hash);
    (*ftrace).profile_hash = null_mut();
}

unsafe fn __cmd_profile(ftrace: *mut perf_ftrace) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut io_v: io = zeroed();
    let mut line: *mut c_char = null_mut();
    let mut line_len: size_t = 0;
    if prepare_func_profile(ftrace) < 0 {
        pr_err(b"failed to prepare func profiler\n\0".as_ptr() as *const c_char);
        return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    if init_tracing_instance() < 0 { return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if reset_tracing_files(ftrace) < 0 { pr_err(b"failed to reset ftrace\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if write_tracing_file(b"trace\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char) < 0 { exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if set_tracing_options(ftrace) < 0 { exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    if write_tracing_file(b"current_tracer\0".as_ptr() as *const c_char, (*ftrace).tracer) < 0 {
        pr_err(b"failed to set current_tracer to %s\n\0".as_ptr() as *const c_char, (*ftrace).tracer);
        exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    setup_pager();
    let trace_file = get_tracing_instance_file(b"trace_pipe\0".as_ptr() as *const c_char);
    if trace_file.is_null() { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    let trace_fd = open(trace_file, O_RDONLY);
    put_tracing_file(trace_file);
    if trace_fd < 0 { pr_err(b"failed to open trace_pipe\n\0".as_ptr() as *const c_char); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }; }
    fcntl(trace_fd, F_SETFL, O_NONBLOCK);
    if write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) < 0 {
        pr_err(b"can't enable tracing\n\0".as_ptr() as *const c_char);
        close(trace_fd); exit_tracing_instance(); return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    evlist__start_workload((*ftrace).evlist);
    io__init(&mut io_v, trace_fd, buf.as_mut_ptr(), buf.len());
    io_v.timeout_ms = -1;
    while done == 0 && !io_v.eof {
        if io__getline(&mut io_v, &mut line, &mut line_len) < 0 { break; }
        if parse_func_duration(ftrace, line, line_len) < 0 { break; }
    }
    write_tracing_file(b"tracing_on\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    if workload_exec_errno != 0 {
        let emsg = str_error_r(workload_exec_errno, buf.as_mut_ptr(), buf.len());
        /* flush stdout first so below error msg appears at the end. */
        fflush(stdout);
        pr_err(b"workload failed: %s\n\0".as_ptr() as *const c_char, emsg);
        free(line as *mut c_void); close(trace_fd); exit_tracing_instance();
        return if done != 0 && workload_exec_errno == 0 { 0 } else { -1 };
    }
    /* read remaining buffer contents */
    io_v.timeout_ms = 0;
    while !io_v.eof {
        if io__getline(&mut io_v, &mut line, &mut line_len) < 0 { break; }
        if parse_func_duration(ftrace, line, line_len) < 0 { break; }
    }
    print_profile_result(ftrace);
    free(line as *mut c_void);
    close(trace_fd);
    exit_tracing_instance();
    if done != 0 && workload_exec_errno == 0 { 0 } else { -1 }
}

unsafe extern "C" fn perf_ftrace_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let ftrace = cb as *mut perf_ftrace;
    if !strstarts(var, b"ftrace.\0".as_ptr() as *const c_char) { return 0; }
    if strcmp(var, b"ftrace.tracer\0".as_ptr() as *const c_char) != 0 { return -1; }
    if strcmp(value, b"function_graph\0".as_ptr() as *const c_char) == 0 ||
       strcmp(value, b"function\0".as_ptr() as *const c_char) == 0 {
        (*ftrace).tracer = value;
        return 0;
    }
    pr_err(b"Please select \"function_graph\" (default) or \"function\"\n\0".as_ptr() as *const c_char);
    -1
}

unsafe extern "C" fn list_function_cb(str_: *mut c_char, arg: *mut c_void) {
    let filter = arg as *mut strfilter;
    if strfilter__compare(filter, str_) {
        printf(b"%s\0".as_ptr() as *const c_char, str_);
    }
}

unsafe extern "C" fn opt_list_avail_functions(_opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let mut err: *const c_char = null();
    if unset != 0 || str_.is_null() { return -1; }
    let filter = strfilter__new(str_, &mut err);
    if filter.is_null() { return if !err.is_null() { -EINVAL } else { -ENOMEM }; }
    let mut ret = strfilter__or(filter, str_, &mut err);
    if ret == -EINVAL {
        pr_err(b"Filter parse error at %td.\n\0".as_ptr() as *const c_char, err.offset_from(str_) + 1);
        pr_err(b"Source: \"%s\"\n\0".as_ptr() as *const c_char, str_);
        pr_err(b"         %*c\n\0".as_ptr() as *const c_char, (err.offset_from(str_) + 1) as c_int, b'^' as c_int);
        strfilter__delete(filter);
        return ret;
    }
    ret = read_tracing_file_by_line(b"available_filter_functions\0".as_ptr() as *const c_char, list_function_cb, filter as *mut c_void);
    strfilter__delete(filter);
    if ret < 0 { return ret; }
    exit(0);
}

unsafe extern "C" fn parse_filter_func(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let head = (*opt).value as *mut list_head;
    let entry = malloc(size_of::<filter_entry>() + strlen(str_) + 1) as *mut filter_entry;
    if entry.is_null() { return -ENOMEM; }
    strcpy((*entry).name.as_ptr() as *mut c_char, str_);
    list_add_tail(&mut (*entry).list, head);
    0
}

unsafe fn delete_filter_func(head: *mut list_head) {
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        list_del_init(pos);
        free(pos as *mut c_void);
        pos = next;
    }
}

unsafe extern "C" fn parse_filter_event(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let head = (*opt).value as *mut list_head;
    let s = strdup(str_);
    if s.is_null() { return -ENOMEM; }
    let mut tmp = s;
    let mut ret = -ENOMEM;
    loop {
        let p = strsep(&mut tmp, b",\0".as_ptr() as *const c_char);
        if p.is_null() { break; }
        let entry = malloc(size_of::<filter_entry>() + strlen(p) + 1) as *mut filter_entry;
        if entry.is_null() { free(s as *mut c_void); return ret; }
        strcpy((*entry).name.as_ptr() as *mut c_char, p);
        list_add_tail(&mut (*entry).list, head);
    }
    ret = 0;
    free(s as *mut c_void);
    ret
}

unsafe extern "C" fn parse_buffer_size(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let s = (*opt).value as *mut c_ulong;
    let mut tags_size = [
        parse_tag { tag: b'B' as c_char, mult: 1 },
        parse_tag { tag: b'K' as c_char, mult: 1 << 10 },
        parse_tag { tag: b'M' as c_char, mult: 1 << 20 },
        parse_tag { tag: b'G' as c_char, mult: 1 << 30 },
        parse_tag { tag: 0, mult: 0 },
    ];
    if unset != 0 {
        *s = 0;
        return 0;
    }
    let val = parse_tag_value(str_, tags_size.as_mut_ptr());
    if val != c_ulong::MAX {
        if val < 1024 {
            pr_err(b"buffer size too small, must larger than 1KB.\0".as_ptr() as *const c_char);
            return -1;
        }
        *s = val;
        return 0;
    }
    -1
}

unsafe extern "C" fn parse_func_tracer_opts(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let ftrace = (*opt).value as *mut perf_ftrace;
    let mut func_tracer_opts = [
        sublevel_option { name: b"call-graph\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).func_stack_trace as *mut _ as *mut c_void },
        sublevel_option { name: b"irq-info\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).func_irq_info as *mut _ as *mut c_void },
        sublevel_option { name: null(), value_ptr: null_mut() },
    ];
    if unset != 0 { return 0; }
    let ret = perf_parse_sublevel_options(str_, func_tracer_opts.as_mut_ptr());
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn parse_graph_tracer_opts(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let ftrace = (*opt).value as *mut perf_ftrace;
    let mut graph_tracer_opts = [
        sublevel_option { name: b"args\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_args as *mut _ as *mut c_void },
        sublevel_option { name: b"retval\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_retval as *mut _ as *mut c_void },
        sublevel_option { name: b"retval-hex\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_retval_hex as *mut _ as *mut c_void },
        sublevel_option { name: b"retaddr\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_retaddr as *mut _ as *mut c_void },
        sublevel_option { name: b"nosleep-time\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_nosleep_time as *mut _ as *mut c_void },
        sublevel_option { name: b"noirqs\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_noirqs as *mut _ as *mut c_void },
        sublevel_option { name: b"verbose\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_verbose as *mut _ as *mut c_void },
        sublevel_option { name: b"thresh\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_thresh as *mut _ as *mut c_void },
        sublevel_option { name: b"depth\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_depth as *mut _ as *mut c_void },
        sublevel_option { name: b"tail\0".as_ptr() as *const c_char, value_ptr: &mut (*ftrace).graph_tail as *mut _ as *mut c_void },
        sublevel_option { name: null(), value_ptr: null_mut() },
    ];
    if unset != 0 { return 0; }
    let ret = perf_parse_sublevel_options(str_, graph_tracer_opts.as_mut_ptr());
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn parse_sort_key(opt: *const option, str_: *const c_char, unset: c_int) -> c_int {
    let key = (*opt).value as *mut perf_ftrace_profile_sort_key;
    if unset != 0 { return 0; }
    if strcmp(str_, b"total\0".as_ptr() as *const c_char) == 0 {
        *key = perf_ftrace_profile_sort_key::PFP_SORT_TOTAL;
    } else if strcmp(str_, b"avg\0".as_ptr() as *const c_char) == 0 {
        *key = perf_ftrace_profile_sort_key::PFP_SORT_AVG;
    } else if strcmp(str_, b"max\0".as_ptr() as *const c_char) == 0 {
        *key = perf_ftrace_profile_sort_key::PFP_SORT_MAX;
    } else if strcmp(str_, b"count\0".as_ptr() as *const c_char) == 0 {
        *key = perf_ftrace_profile_sort_key::PFP_SORT_COUNT;
    } else if strcmp(str_, b"name\0".as_ptr() as *const c_char) == 0 {
        *key = perf_ftrace_profile_sort_key::PFP_SORT_NAME;
    } else {
        pr_err(b"Unknown sort key: %s\n\0".as_ptr() as *const c_char, str_);
        return -1;
    }
    0
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum perf_ftrace_subcommand {
    PERF_FTRACE_NONE,
    PERF_FTRACE_TRACE,
    PERF_FTRACE_LATENCY,
    PERF_FTRACE_PROFILE,
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn OPT_END_ARRAY() -> [option; 1] {
    [option { value: null_mut() }]
}

#[no_mangle]
pub unsafe extern "C" fn cmd_ftrace(mut argc: c_int, mut argv: *mut *const c_char) -> c_int {
    let mut ret: c_int;
    let mut cmd_func: Option<unsafe fn(*mut perf_ftrace) -> c_int> = None;
    let mut ftrace: perf_ftrace = zeroed();
    ftrace.tracer = DEFAULT_TRACER;

    /*
     * The C source builds common_options, ftrace_options, latency_options, and
     * profile_options with OPT_* macros from parse-options.h. Their exact Rust
     * initializers depend on the external struct option macro definitions, so
     * this translation preserves the local option arrays as external-layout
     * placeholders and keeps every parser callback and branch below.
     *
     * HAVE_BPF_SKEL condition in latency_options: when defined, the C option
     * includes -b/--use-bpf and stores into ftrace.target.use_bpf.
     */
    let common_options = OPT_END_ARRAY();
    let ftrace_options = OPT_END_ARRAY();
    let latency_options = OPT_END_ARRAY();
    let profile_options = OPT_END_ARRAY();
    let mut options: *const option = ftrace_options.as_ptr();
    let ftrace_usage = [
        b"perf ftrace [<options>] [<command>]\0".as_ptr() as *const c_char,
        b"perf ftrace [<options>] -- [<command>] [<options>]\0".as_ptr() as *const c_char,
        b"perf ftrace {trace|latency|profile} [<options>] [<command>]\0".as_ptr() as *const c_char,
        b"perf ftrace {trace|latency|profile} [<options>] -- [<command>] [<options>]\0".as_ptr() as *const c_char,
        null(),
    ];
    let mut subcmd = perf_ftrace_subcommand::PERF_FTRACE_NONE;

    INIT_LIST_HEAD(&mut ftrace.filters);
    INIT_LIST_HEAD(&mut ftrace.notrace);
    INIT_LIST_HEAD(&mut ftrace.graph_funcs);
    INIT_LIST_HEAD(&mut ftrace.nograph_funcs);
    INIT_LIST_HEAD(&mut ftrace.event_pair);

    signal(SIGINT, sig_handler);
    signal(SIGUSR1, sig_handler);
    signal(SIGCHLD, sig_handler);
    signal(SIGPIPE, sig_handler);

    if !check_ftrace_capable() { return -1; }
    if !is_ftrace_supported() {
        pr_err(b"ftrace is not supported on this system\n\0".as_ptr() as *const c_char);
        return -ENOTSUP;
    }
    ret = perf_config(perf_ftrace_config, &mut ftrace as *mut _ as *mut c_void);
    if ret < 0 { return -1; }

    if argc > 1 {
        let arg1 = *argv.add(1);
        if strcmp(arg1, b"trace\0".as_ptr() as *const c_char) == 0 {
            subcmd = perf_ftrace_subcommand::PERF_FTRACE_TRACE;
        } else if strcmp(arg1, b"latency\0".as_ptr() as *const c_char) == 0 {
            subcmd = perf_ftrace_subcommand::PERF_FTRACE_LATENCY;
            options = latency_options.as_ptr();
        } else if strcmp(arg1, b"profile\0".as_ptr() as *const c_char) == 0 {
            subcmd = perf_ftrace_subcommand::PERF_FTRACE_PROFILE;
            options = profile_options.as_ptr();
        }
        if subcmd != perf_ftrace_subcommand::PERF_FTRACE_NONE {
            argc -= 1;
            argv = argv.add(1);
        }
    }
    /* for backward compatibility */
    if subcmd == perf_ftrace_subcommand::PERF_FTRACE_NONE {
        subcmd = perf_ftrace_subcommand::PERF_FTRACE_TRACE;
    }

    argc = parse_options(argc, argv, options, ftrace_usage.as_ptr(), PARSE_OPT_STOP_AT_NON_OPTION);
    if argc < 0 {
        ret = -EINVAL;
        delete_filter_func(&mut ftrace.filters);
        delete_filter_func(&mut ftrace.notrace);
        delete_filter_func(&mut ftrace.graph_funcs);
        delete_filter_func(&mut ftrace.nograph_funcs);
        delete_filter_func(&mut ftrace.event_pair);
        return ret;
    }

    /* Make system wide (-a) the default target. */
    if argc == 0 && target__none(&mut ftrace.target) {
        ftrace.target.system_wide = true;
    }

    match subcmd {
        perf_ftrace_subcommand::PERF_FTRACE_TRACE => {
            cmd_func = Some(__cmd_ftrace);
        }
        perf_ftrace_subcommand::PERF_FTRACE_LATENCY => {
            if list_empty(&ftrace.filters) && list_empty(&ftrace.event_pair) {
                pr_err(b"Should provide a function or events to measure\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"T\0".as_ptr() as *const c_char, 1);
                parse_options_usage(null(), options, b"e\0".as_ptr() as *const c_char, 1);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            if !list_empty(&ftrace.filters) && !list_empty(&ftrace.event_pair) {
                pr_err(b"Please specify either of function or events\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"T\0".as_ptr() as *const c_char, 1);
                parse_options_usage(null(), options, b"e\0".as_ptr() as *const c_char, 1);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            if !list_empty(&ftrace.event_pair) && !ftrace.target.use_bpf {
                pr_err(b"Event processing needs BPF\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"b\0".as_ptr() as *const c_char, 1);
                parse_options_usage(null(), options, b"e\0".as_ptr() as *const c_char, 1);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            if ftrace.bucket_range == 0 && ftrace.min_latency != 0 {
                pr_err(b"--min-latency works only with --bucket-range\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"min-latency\0".as_ptr() as *const c_char, 0);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            if ftrace.bucket_range != 0 && ftrace.min_latency == 0 {
                /* default min latency should be the bucket range */
                ftrace.min_latency = ftrace.bucket_range as c_int;
            }
            if ftrace.bucket_range == 0 && ftrace.max_latency != 0 {
                pr_err(b"--max-latency works only with --bucket-range\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"max-latency\0".as_ptr() as *const c_char, 0);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            if ftrace.bucket_range != 0 && ftrace.max_latency != 0 &&
                ftrace.max_latency < ftrace.min_latency + ftrace.bucket_range as c_int {
                /* we need at least 1 bucket excluding min and max buckets */
                pr_err(b"--max-latency must be larger than min-latency + bucket-range\n\0".as_ptr() as *const c_char);
                parse_options_usage(ftrace_usage.as_ptr(), options, b"max-latency\0".as_ptr() as *const c_char, 0);
                ret = -EINVAL;
                delete_filter_func(&mut ftrace.filters);
                delete_filter_func(&mut ftrace.notrace);
                delete_filter_func(&mut ftrace.graph_funcs);
                delete_filter_func(&mut ftrace.nograph_funcs);
                delete_filter_func(&mut ftrace.event_pair);
                return ret;
            }
            /* set default unless max_latency is set and valid */
            ftrace.bucket_num = NUM_BUCKET;
            if ftrace.bucket_range != 0 {
                if ftrace.max_latency != 0 {
                    ftrace.bucket_num = ((ftrace.max_latency - ftrace.min_latency) as c_uint) / ftrace.bucket_range + 2;
                } else {
                    /* default max latency should depend on bucket range and num_buckets */
                    ftrace.max_latency = ((NUM_BUCKET - 2) * ftrace.bucket_range) as c_int + ftrace.min_latency;
                }
            }
            cmd_func = Some(__cmd_latency);
        }
        perf_ftrace_subcommand::PERF_FTRACE_PROFILE => {
            cmd_func = Some(__cmd_profile);
        }
        perf_ftrace_subcommand::PERF_FTRACE_NONE => {
            pr_err(b"Invalid subcommand\n\0".as_ptr() as *const c_char);
            ret = -EINVAL;
            delete_filter_func(&mut ftrace.filters);
            delete_filter_func(&mut ftrace.notrace);
            delete_filter_func(&mut ftrace.graph_funcs);
            delete_filter_func(&mut ftrace.nograph_funcs);
            delete_filter_func(&mut ftrace.event_pair);
            return ret;
        }
    }

    ret = target__validate(&mut ftrace.target);
    if ret != 0 {
        let mut errbuf = [0 as c_char; 512];
        target__strerror(&mut ftrace.target, ret, errbuf.as_mut_ptr(), 512);
        pr_err(b"%s\n\0".as_ptr() as *const c_char, errbuf.as_ptr());
        delete_filter_func(&mut ftrace.filters);
        delete_filter_func(&mut ftrace.notrace);
        delete_filter_func(&mut ftrace.graph_funcs);
        delete_filter_func(&mut ftrace.nograph_funcs);
        delete_filter_func(&mut ftrace.event_pair);
        return ret;
    }

    ftrace.evlist = evlist__new();
    if ftrace.evlist.is_null() {
        ret = -ENOMEM;
        delete_filter_func(&mut ftrace.filters);
        delete_filter_func(&mut ftrace.notrace);
        delete_filter_func(&mut ftrace.graph_funcs);
        delete_filter_func(&mut ftrace.nograph_funcs);
        delete_filter_func(&mut ftrace.event_pair);
        return ret;
    }

    ret = evlist__create_maps(ftrace.evlist, &mut ftrace.target);
    if ret >= 0 && argc != 0 {
        ret = evlist__prepare_workload(ftrace.evlist, &mut ftrace.target, argv, false,
                                       ftrace__workload_exec_failed_signal);
    }
    if ret >= 0 {
        ret = cmd_func.unwrap()(&mut ftrace);
    }

    evlist__put(ftrace.evlist);
    delete_filter_func(&mut ftrace.filters);
    delete_filter_func(&mut ftrace.notrace);
    delete_filter_func(&mut ftrace.graph_funcs);
    delete_filter_func(&mut ftrace.nograph_funcs);
    delete_filter_func(&mut ftrace.event_pair);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
