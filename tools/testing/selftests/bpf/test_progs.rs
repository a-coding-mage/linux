// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

/* Translated from test_progs.c. C includes are external dependencies:
 * test_progs.h, testing_helpers.h, cgroup_helpers.h, json_writer.h,
 * network_helpers.h, verification_cert.h, prog_tests/tests.h, libc/libbpf
 * and Linux/POSIX headers.
 */

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type useconds_t = c_uint;
type pthread_t = usize;
type timer_t = *mut c_void;
type FILE = c_void;
type va_list = *mut c_void;
type error_t = c_int;
type __u32 = u32;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const EXIT_NO_TEST: c_int = 2;
const EXIT_ERR_SETUP_INFRA: c_int = 3;
const TEST_NUM_WIDTH: c_int = 7;
const MAX_BACKTRACE_SZ: usize = 128;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const CLONE_NEWNET: c_int = 0x40000000;
const SIGEV_THREAD: c_int = 2;
const SIGSEGV: c_int = 11;
const SIGINT: c_int = 2;
const SA_RESETHAND: c_int = 0x80000000u32 as c_int;
const CLOCK_MONOTONIC: c_int = 1;
const STDERR_FILENO: c_int = 2;
const AF_UNIX: c_int = 1;
const SOCK_SEQPACKET: c_int = 5;
const SOCK_CLOEXEC: c_int = 0o2000000;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const KEY_SPEC_SESSION_KEYRING: c_int = -3;
const __NR_nanosleep: c_long = 35;
const __NR_add_key: c_long = 248;

const VERBOSE_NONE: c_int = 0;
const VERBOSE_NORMAL: c_int = 1;
const VERBOSE_VERY: c_int = 2;
const VERBOSE_SUPER: c_int = 3;
const WD_NOTIFY: c_int = 0;
const WD_KILL: c_int = 1;
const LIBBPF_DEBUG: c_int = 0;
const LIBBPF_STRICT_ALL: c_int = 0xffffffffu32 as c_int;
const ARGP_KEY_ARG: c_int = 0x1000002;
const ARGP_KEY_END: c_int = 0x1000001;
const ARGP_ERR_UNKNOWN: c_int = 7;
const OPTION_ARG_OPTIONAL: c_int = 0x1;
const PERF_MAX_STACK_DEPTH: usize = 127;
const MAX_LOG_TRUNK_SIZE: usize = 8192;
const MAX_SUBTEST_NAME: usize = 1024;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct itimerspec {
    it_interval: timespec,
    it_value: timespec,
}

#[repr(C)]
struct sigval {
    sival_ptr: *mut c_void,
}

#[repr(C)]
struct sigevent {
    sigev_notify: c_int,
    sigev_signo: c_int,
    sigev_value: sigval,
    sigev_notify_function: Option<unsafe extern "C" fn(sigval)>,
}

#[repr(C)]
struct sigaction {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_flags: c_int,
}

#[repr(C)]
struct pthread_mutex_t {
    __opaque: [u8; 40],
}

#[repr(C)]
struct cpu_set_t {
    __bits: [usize; 16],
}

#[repr(C)]
struct bpf_object { _unused: [u8; 0] }
#[repr(C)]
struct bpf_map { _unused: [u8; 0] }
#[repr(C)]
struct btf { _unused: [u8; 0] }
#[repr(C)]
struct btf_type { name_off: __u32 }
#[repr(C)]
struct btf_enum { name_off: __u32, val: c_int }
#[repr(C)]
struct bpf_stack_build_id { _unused: [u8; 0] }
#[repr(C)]
struct tmonitor_ctx { _unused: [u8; 0] }
#[repr(C)]
struct nstoken { _unused: [u8; 0] }
#[repr(C)]
struct json_writer_t { _unused: [u8; 0] }

#[repr(C)]
struct test_filter {
    name: *const c_char,
    subtest_cnt: c_int,
    subtests: *mut *const c_char,
}

#[repr(C)]
struct test_filter_set {
    cnt: c_int,
    tests: *mut test_filter,
}

#[repr(C)]
struct test_selector {
    blacklist: test_filter_set,
    whitelist: test_filter_set,
    num_set: *mut bool,
    num_set_len: c_int,
}

#[repr(C)]
struct subtest_state {
    name: *mut c_char,
    log_buf: *mut c_char,
    log_cnt: size_t,
    error_cnt: c_int,
    skipped: bool,
    filtered: bool,
    should_tmon: bool,
}

#[repr(C)]
struct test_state {
    log_buf: *mut c_char,
    log_cnt: size_t,
    stdout_saved: *mut FILE,
    subtest_states: *mut subtest_state,
    subtest_num: c_int,
    error_cnt: c_int,
    skip_cnt: c_int,
    sub_succ_cnt: c_int,
    force_log: bool,
    tested: bool,
}

#[repr(C)]
struct prog_test_def {
    test_name: *const c_char,
    test_num: c_int,
    run_test: Option<unsafe extern "C" fn()>,
    run_serial_test: Option<unsafe extern "C" fn()>,
    should_run: bool,
    not_built: bool,
    selected: bool,
    need_cgroup_cleanup: bool,
    should_tmon: bool,
}

#[repr(C)]
struct test_env {
    verbosity: c_int,
    worker_id: c_int,
    stdout_saved: *mut FILE,
    stderr_saved: *mut FILE,
    subtest_state: *mut subtest_state,
    test_state: *mut test_state,
    test: *mut prog_test_def,
    secs_till_notify: c_int,
    secs_till_kill: c_int,
    watchdog_state: c_int,
    watchdog: timer_t,
    main_thread: pthread_t,
    nr_cpus: c_int,
    saved_netns_fd: c_int,
    test_selector: test_selector,
    subtest_selector: test_selector,
    tmon_selector: test_selector,
    verifier_stats: bool,
    get_test_cnt: bool,
    list_test_names: bool,
    workers: c_int,
    debug: bool,
    error_summary: bool,
    json: *mut FILE,
    not_built_cnt: c_int,
    succ_cnt: c_int,
    sub_succ_cnt: c_int,
    fail_cnt: c_int,
    skip_cnt: c_int,
    worker_socks: *mut c_int,
    worker_pids: *mut pid_t,
    worker_current_test: *mut c_int,
    jit_enabled: c_int,
    has_testmod: bool,
}

#[repr(C)]
struct msg_do_test { num: c_int }
#[repr(C)]
struct msg_test_done {
    num: c_int,
    have_log: bool,
    error_cnt: c_int,
    skip_cnt: c_int,
    sub_succ_cnt: c_int,
    subtest_num: c_int,
}
#[repr(C)]
struct msg_subtest_done {
    num: c_int,
    name: [c_char; MAX_SUBTEST_NAME],
    error_cnt: c_int,
    skipped: bool,
    filtered: bool,
    have_log: bool,
}
#[repr(C)]
struct msg_test_log {
    log_buf: [c_char; MAX_LOG_TRUNK_SIZE],
    is_last: bool,
}
#[repr(C)]
union msg_payload {
    do_test: core::mem::ManuallyDrop<msg_do_test>,
    test_done: core::mem::ManuallyDrop<msg_test_done>,
    subtest_done: core::mem::ManuallyDrop<msg_subtest_done>,
    test_log: core::mem::ManuallyDrop<msg_test_log>,
}
#[repr(C)]
struct msg {
    type_: msg_type,
    payload: msg_payload,
}
type msg_type = c_int;
const MSG_DO_TEST: msg_type = 0;
const MSG_TEST_DONE: msg_type = 1;
const MSG_SUBTEST_DONE: msg_type = 2;
const MSG_TEST_LOG: msg_type = 3;
const MSG_EXIT: msg_type = 4;

#[repr(C)]
struct dispatch_data {
    worker_id: c_int,
    sock_fd: c_int,
}

#[repr(C)]
struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
}

#[repr(C)]
struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
    doc: *const c_char,
}

#[repr(C)]
struct argp_state {
    input: *mut c_void,
}

#[repr(C)]
struct libbpf_output_capture_t {
    buf: *mut c_char,
    buf_sz: size_t,
}

#[repr(C)]
struct netns_obj {
    nsname: *mut c_char,
    tmon: *mut tmonitor_ctx,
    nstoken: *mut nstoken,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    static mut extra_prog_load_log_flags: c_int;
    static test_progs_verification_cert: [u8; 0];
    static test_progs_verification_cert_len: size_t;
    static BPF_TESTMOD_TEST_FILE: *const c_char;

    fn fflush(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: va_list) -> c_int;
    fn dprintf(fd: c_int, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn open_memstream(buf: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn exit(status: c_int) -> !;
    fn pthread_self() -> pthread_t;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_tryjoin_np(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn timer_create(clockid: c_int, sevp: *mut sigevent, timerid: *mut timer_t) -> c_int;
    fn timer_settime(timerid: timer_t, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn srand(seed: c_uint);
    fn time(tloc: *mut c_long) -> c_long;
    fn get_nprocs() -> c_int;
    fn backtrace(buffer: *mut *mut c_void, size: c_int) -> c_int;
    fn backtrace_symbols_fd(buffer: *const *mut c_void, size: c_int, fd: c_int);
    fn assert(expr: bool);

    fn setup_cgroup_environment() -> c_int;
    fn cleanup_cgroup_environment();
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn make_netns(name: *const c_char) -> c_int;
    fn remove_netns(name: *const c_char);
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn traffic_monitor_start(nsname: *const c_char, test: *const c_char, subtest: *const c_char) -> *mut tmonitor_ctx;
    fn traffic_monitor_stop(ctx: *mut tmonitor_ctx);
    fn traffic_monitor_set_print(cb: unsafe extern "C" fn(*const c_char, va_list) -> c_int);
    fn parse_num_list(arg: *mut c_char, set: *mut *mut bool, len: *mut c_int) -> c_int;
    fn parse_test_list_file(path: *mut c_char, set: *mut test_filter_set, glob: bool) -> c_int;
    fn parse_test_list(arg: *mut c_char, set: *mut test_filter_set, glob: bool) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn argp_parse(argp: *const argp, argc: c_int, argv: *mut *mut c_char, flags: c_uint, arg_index: *mut c_int, input: *mut c_void) -> c_int;
    fn btf__type_cnt(btf: *mut btf) -> __u32;
    fn btf__type_by_id(btf: *mut btf, id: __u32) -> *const btf_type;
    fn btf_is_enum(t: *const btf_type) -> bool;
    fn btf_enum(t: *const btf_type) -> *const btf_enum;
    fn btf_vlen(t: *const btf_type) -> __u32;
    fn btf__str_by_offset(btf: *mut btf, off: __u32) -> *const c_char;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn libbpf_set_strict_mode(mode: c_int);
    fn libbpf_set_print(cb: unsafe extern "C" fn(c_int, *const c_char, va_list) -> c_int);
    fn libbpf_num_possible_cpus() -> c_int;
    fn is_jit_enabled() -> c_int;
    fn unload_bpf_testmod(verbose: bool);
    fn load_bpf_testmod(verbose: bool) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> ssize_t;
    fn jsonw_new(file: *mut FILE) -> *mut json_writer_t;
    fn jsonw_destroy(w: *mut *mut json_writer_t);
    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_string_field(w: *mut json_writer_t, name: *const c_char, value: *const c_char);
    fn jsonw_uint_field(w: *mut json_writer_t, name: *const c_char, value: c_int);
    fn jsonw_bool_field(w: *mut json_writer_t, name: *const c_char, value: bool);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut btf, name: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...);
}

static mut env_verbosity: c_int = 0;
static mut env: test_env = unsafe { zeroed() };
static mut stdout_lock: pthread_mutex_t = pthread_mutex_t { __opaque: [0; 40] };
static mut current_test_idx: c_int = 0;
static mut current_test_lock: pthread_mutex_t = pthread_mutex_t { __opaque: [0; 40] };
static mut stdout_output_lock: pthread_mutex_t = pthread_mutex_t { __opaque: [0; 40] };

/* prog_tests/tests.h expands weak extern test declarations and the initializer
 * for prog_test_defs in C. The generated Rust binding is expected to provide
 * equivalent storage for this translation unit.
 */
static mut prog_test_defs: *mut prog_test_def = ptr::null_mut();
static mut prog_test_cnt: c_int = 0;
static mut test_states: *mut test_state = ptr::null_mut();

static mut argp_program_version: *const c_char = b"test_progs 0.1\0".as_ptr() as *const c_char;
static mut argp_program_bug_address: *const c_char = b"<bpf@vger.kernel.org>\0".as_ptr() as *const c_char;
static argp_program_doc: &[u8] = b"BPF selftests test runner\x0bOptions accepting the NAMES parameter take either a comma-separated list\nof test names, or a filename prefixed with @. The file contains one name\n(or wildcard pattern) per line, and comments beginning with # are ignored.\n\nThese options can be passed repeatedly to read multiple files.\n\0";

const ARG_TEST_NUM: c_int = b'n' as c_int;
const ARG_TEST_NAME: c_int = b't' as c_int;
const ARG_TEST_NAME_BLACKLIST: c_int = b'b' as c_int;
const ARG_VERIFIER_STATS: c_int = b's' as c_int;
const ARG_VERBOSE: c_int = b'v' as c_int;
const ARG_GET_TEST_CNT: c_int = b'c' as c_int;
const ARG_LIST_TEST_NAMES: c_int = b'l' as c_int;
const ARG_TEST_NAME_GLOB_ALLOWLIST: c_int = b'a' as c_int;
const ARG_TEST_NAME_GLOB_DENYLIST: c_int = b'd' as c_int;
const ARG_NUM_WORKERS: c_int = b'j' as c_int;
const ARG_DEBUG: c_int = -1;
const ARG_JSON_SUMMARY: c_int = b'J' as c_int;
const ARG_TRAFFIC_MONITOR: c_int = b'm' as c_int;
const ARG_WATCHDOG_TIMEOUT: c_int = b'w' as c_int;
const ARG_NO_ERROR_SUMMARY: c_int = -2;

static opts: &[argp_option] = &[
    argp_option { name: b"num\0".as_ptr() as *const c_char, key: ARG_TEST_NUM, arg: b"NUM\0".as_ptr() as *const c_char, flags: 0, doc: b"Run test number NUM only \0".as_ptr() as *const c_char },
    argp_option { name: b"name\0".as_ptr() as *const c_char, key: ARG_TEST_NAME, arg: b"NAMES\0".as_ptr() as *const c_char, flags: 0, doc: b"Run tests with names containing any string from NAMES list\0".as_ptr() as *const c_char },
    argp_option { name: b"name-blacklist\0".as_ptr() as *const c_char, key: ARG_TEST_NAME_BLACKLIST, arg: b"NAMES\0".as_ptr() as *const c_char, flags: 0, doc: b"Don't run tests with names containing any string from NAMES list\0".as_ptr() as *const c_char },
    argp_option { name: b"verifier-stats\0".as_ptr() as *const c_char, key: ARG_VERIFIER_STATS, arg: ptr::null(), flags: 0, doc: b"Output verifier statistics\0".as_ptr() as *const c_char },
    argp_option { name: b"verbose\0".as_ptr() as *const c_char, key: ARG_VERBOSE, arg: b"LEVEL\0".as_ptr() as *const c_char, flags: OPTION_ARG_OPTIONAL, doc: b"Verbose output (use -vv or -vvv for progressively verbose output)\0".as_ptr() as *const c_char },
    argp_option { name: b"count\0".as_ptr() as *const c_char, key: ARG_GET_TEST_CNT, arg: ptr::null(), flags: 0, doc: b"Get number of selected top-level tests \0".as_ptr() as *const c_char },
    argp_option { name: b"list\0".as_ptr() as *const c_char, key: ARG_LIST_TEST_NAMES, arg: ptr::null(), flags: 0, doc: b"List test names that would run (without running them) \0".as_ptr() as *const c_char },
    argp_option { name: b"allow\0".as_ptr() as *const c_char, key: ARG_TEST_NAME_GLOB_ALLOWLIST, arg: b"NAMES\0".as_ptr() as *const c_char, flags: 0, doc: b"Run tests with name matching the pattern (supports '*' wildcard).\0".as_ptr() as *const c_char },
    argp_option { name: b"deny\0".as_ptr() as *const c_char, key: ARG_TEST_NAME_GLOB_DENYLIST, arg: b"NAMES\0".as_ptr() as *const c_char, flags: 0, doc: b"Don't run tests with name matching the pattern (supports '*' wildcard).\0".as_ptr() as *const c_char },
    argp_option { name: b"workers\0".as_ptr() as *const c_char, key: ARG_NUM_WORKERS, arg: b"WORKERS\0".as_ptr() as *const c_char, flags: OPTION_ARG_OPTIONAL, doc: b"Number of workers to run in parallel, default to number of cpus.\0".as_ptr() as *const c_char },
    argp_option { name: b"debug\0".as_ptr() as *const c_char, key: ARG_DEBUG, arg: ptr::null(), flags: 0, doc: b"print extra debug information for test_progs.\0".as_ptr() as *const c_char },
    argp_option { name: b"json-summary\0".as_ptr() as *const c_char, key: ARG_JSON_SUMMARY, arg: b"FILE\0".as_ptr() as *const c_char, flags: 0, doc: b"Write report in json format to this file.\0".as_ptr() as *const c_char },
    /* #ifdef TRAFFIC_MONITOR */
    argp_option { name: b"traffic-monitor\0".as_ptr() as *const c_char, key: ARG_TRAFFIC_MONITOR, arg: b"NAMES\0".as_ptr() as *const c_char, flags: 0, doc: b"Monitor network traffic of tests with name matching the pattern (supports '*' wildcard).\0".as_ptr() as *const c_char },
    argp_option { name: b"watchdog-timeout\0".as_ptr() as *const c_char, key: ARG_WATCHDOG_TIMEOUT, arg: b"SECONDS\0".as_ptr() as *const c_char, flags: 0, doc: b"Kill the process if tests are not making progress for specified number of seconds.\0".as_ptr() as *const c_char },
    argp_option { name: b"no-error-summary\0".as_ptr() as *const c_char, key: ARG_NO_ERROR_SUMMARY, arg: ptr::null(), flags: 0, doc: b"Do not re-print the aggregated error logs of failed tests at the end of the run.\0".as_ptr() as *const c_char },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null() },
];

static mut libbpf_capture_stream: *mut FILE = ptr::null_mut();
static mut libbpf_output_capture: libbpf_output_capture_t = libbpf_output_capture_t { buf: ptr::null_mut(), buf_sz: 0 };

unsafe fn cpu_zero(set: *mut cpu_set_t) { memset(set as *mut c_void, 0, size_of::<cpu_set_t>()); }
unsafe fn cpu_set(cpu: c_int, set: *mut cpu_set_t) {
    let idx = cpu as usize / (8 * size_of::<usize>());
    let bit = cpu as usize % (8 * size_of::<usize>());
    (*set).__bits[idx] |= 1usize << bit;
}

unsafe fn verbose() -> bool {
    env.verbosity > VERBOSE_NONE
}

unsafe fn stdio_hijack_init(log_buf: *mut *mut c_char, log_cnt: *mut size_t) {
    /* #ifdef __GLIBC__ */
    if verbose() && env.worker_id == -1 { return; }
    fflush(stdout);
    fflush(stderr);
    stdout = open_memstream(log_buf, log_cnt);
    if stdout.is_null() {
        stdout = env.stdout_saved;
        perror(b"open_memstream\0".as_ptr() as *const c_char);
        return;
    }
    if !env.subtest_state.is_null() {
        (*env.subtest_state).stdout_saved = stdout;
    } else {
        (*env.test_state).stdout_saved = stdout;
    }
    stderr = stdout;
}

unsafe fn stdio_hijack(log_buf: *mut *mut c_char, log_cnt: *mut size_t) {
    if verbose() && env.worker_id == -1 { return; }
    env.stdout_saved = stdout;
    env.stderr_saved = stderr;
    stdio_hijack_init(log_buf, log_cnt);
}

unsafe fn stdio_restore() {
    if verbose() && env.worker_id == -1 { return; }
    fflush(stdout);
    pthread_mutex_lock(&raw mut stdout_lock);
    if !env.subtest_state.is_null() {
        if !(*env.subtest_state).stdout_saved.is_null() { fclose((*env.subtest_state).stdout_saved); }
        (*env.subtest_state).stdout_saved = ptr::null_mut();
        stdout = (*env.test_state).stdout_saved;
        stderr = (*env.test_state).stdout_saved;
    } else {
        if !(*env.test_state).stdout_saved.is_null() { fclose((*env.test_state).stdout_saved); }
        (*env.test_state).stdout_saved = ptr::null_mut();
        stdout = env.stdout_saved;
        stderr = env.stderr_saved;
    }
    pthread_mutex_unlock(&raw mut stdout_lock);
}

unsafe extern "C" fn traffic_monitor_print_fn(format: *const c_char, args: va_list) -> c_int {
    pthread_mutex_lock(&raw mut stdout_lock);
    vfprintf(stdout, format, args);
    pthread_mutex_unlock(&raw mut stdout_lock);
    0
}

/* Adapted from perf/util/string.c */
unsafe fn glob_match(mut str_: *const c_char, mut pat: *const c_char) -> bool {
    while *str_ != 0 && *pat != 0 && *pat != b'*' as c_char {
        if *str_ != *pat { return false; }
        str_ = str_.add(1);
        pat = pat.add(1);
    }
    /* Check wild card */
    if *pat == b'*' as c_char {
        while *pat == b'*' as c_char { pat = pat.add(1); }
        if *pat == 0 { return true; } /* Tail wild card matches all */
        while *str_ != 0 {
            if glob_match(str_, pat) { return true; }
            str_ = str_.add(1);
        }
    }
    *str_ == 0 && *pat == 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usleep(usec: useconds_t) -> c_int {
    let ts = timespec {
        tv_sec: (usec / 1000000) as c_long,
        tv_nsec: ((usec % 1000000) * 1000) as c_long,
    };
    syscall(__NR_nanosleep, &ts as *const timespec, ptr::null::<c_void>()) as c_int
}

unsafe extern "C" fn watchdog_timer_func(_sigval: sigval) {
    let mut timeout: itimerspec = zeroed();
    let mut test_name = [0 as c_char; 256];
    let err: c_int;
    if !env.subtest_state.is_null() {
        snprintf(test_name.as_mut_ptr(), test_name.len(), b"%s/%s\0".as_ptr() as *const c_char, (*env.test).test_name, (*env.subtest_state).name);
    } else {
        snprintf(test_name.as_mut_ptr(), test_name.len(), b"%s\0".as_ptr() as *const c_char, (*env.test).test_name);
    }
    match env.watchdog_state {
        WD_NOTIFY => {
            fprintf(env.stderr_saved, b"WATCHDOG: test case %s executes for %d seconds...\n\0".as_ptr() as *const c_char, test_name.as_ptr(), env.secs_till_notify);
            timeout.it_value.tv_sec = (env.secs_till_kill - env.secs_till_notify) as c_long;
            env.watchdog_state = WD_KILL;
            err = timer_settime(env.watchdog, 0, &timeout, ptr::null_mut());
            if err != 0 { fprintf(env.stderr_saved, b"Failed to arm watchdog timer\n\0".as_ptr() as *const c_char); }
        }
        WD_KILL => {
            fprintf(env.stderr_saved, b"WATCHDOG: test case %s executes for %d seconds, terminating with SIGSEGV\n\0".as_ptr() as *const c_char, test_name.as_ptr(), env.secs_till_kill);
            pthread_kill(env.main_thread, SIGSEGV);
        }
        _ => {}
    }
}

unsafe fn watchdog_start() {
    let mut timeout: itimerspec = zeroed();
    if env.secs_till_kill == 0 { return; }
    if env.secs_till_notify > 0 {
        env.watchdog_state = WD_NOTIFY;
        timeout.it_value.tv_sec = env.secs_till_notify as c_long;
    } else {
        env.watchdog_state = WD_KILL;
        timeout.it_value.tv_sec = env.secs_till_kill as c_long;
    }
    let err = timer_settime(env.watchdog, 0, &timeout, ptr::null_mut());
    if err != 0 { fprintf(env.stderr_saved, b"Failed to start watchdog timer\n\0".as_ptr() as *const c_char); }
}

unsafe fn watchdog_stop() {
    let timeout: itimerspec = zeroed();
    env.watchdog_state = WD_NOTIFY;
    let err = timer_settime(env.watchdog, 0, &timeout, ptr::null_mut());
    if err != 0 { fprintf(env.stderr_saved, b"Failed to stop watchdog timer\n\0".as_ptr() as *const c_char); }
}

unsafe fn watchdog_init() {
    let mut watchdog_sev: sigevent = zeroed();
    watchdog_sev.sigev_notify = SIGEV_THREAD;
    watchdog_sev.sigev_notify_function = Some(watchdog_timer_func);
    env.main_thread = pthread_self();
    let err = timer_create(CLOCK_MONOTONIC, &mut watchdog_sev, &raw mut env.watchdog);
    if err != 0 { fprintf(stderr, b"Failed to initialize watchdog timer\n\0".as_ptr() as *const c_char); }
}

unsafe fn should_run(sel: *mut test_selector, num: c_int, name: *const c_char) -> bool {
    for i in 0..(*sel).blacklist.cnt {
        let t = (*sel).blacklist.tests.add(i as usize);
        if glob_match(name, (*t).name) && (*t).subtest_cnt == 0 { return false; }
    }
    for i in 0..(*sel).whitelist.cnt {
        let t = (*sel).whitelist.tests.add(i as usize);
        if glob_match(name, (*t).name) { return true; }
    }
    if (*sel).whitelist.cnt == 0 && (*sel).num_set.is_null() { return true; }
    num < (*sel).num_set_len && *(*sel).num_set.add(num as usize)
}

unsafe fn match_subtest(filter: *mut test_filter_set, test_name: *const c_char, subtest_name: *const c_char) -> bool {
    for i in 0..(*filter).cnt {
        let test = (*filter).tests.add(i as usize);
        if glob_match(test_name, (*test).name) {
            if (*test).subtest_cnt == 0 { return true; }
            for j in 0..(*test).subtest_cnt {
                if glob_match(subtest_name, *(*test).subtests.add(j as usize)) { return true; }
            }
        }
    }
    false
}

unsafe fn match_subtest_desc(filter: *mut test_filter_set, test_name: *const c_char, subtest_name: *const c_char, subtest_desc: *const c_char) -> bool {
    if match_subtest(filter, test_name, subtest_name) { return true; }
    if subtest_desc.is_null() || *subtest_desc == 0 || strcmp(subtest_name, subtest_desc) == 0 { return false; }
    match_subtest(filter, test_name, subtest_desc)
}

unsafe fn should_run_subtest(sel: *mut test_selector, subtest_sel: *mut test_selector, subtest_num: c_int, test_name: *const c_char, subtest_name: *const c_char, subtest_desc: *const c_char) -> bool {
    if match_subtest_desc(&raw mut (*sel).blacklist, test_name, subtest_name, subtest_desc) { return false; }
    if match_subtest_desc(&raw mut (*sel).whitelist, test_name, subtest_name, subtest_desc) { return true; }
    if (*sel).whitelist.cnt == 0 && (*subtest_sel).num_set.is_null() { return true; }
    subtest_num < (*subtest_sel).num_set_len && *(*subtest_sel).num_set.add(subtest_num as usize)
}

unsafe fn should_tmon(sel: *mut test_selector, name: *const c_char) -> bool {
    for i in 0..(*sel).whitelist.cnt {
        let t = (*sel).whitelist.tests.add(i as usize);
        if glob_match(name, (*t).name) && (*t).subtest_cnt == 0 { return true; }
    }
    false
}

unsafe fn test_result(failed: bool, skipped: bool) -> *const c_char {
    if failed { b"FAIL\0".as_ptr() as *const c_char } else if skipped { b"SKIP\0".as_ptr() as *const c_char } else { b"OK\0".as_ptr() as *const c_char }
}

unsafe fn print_test_result(test: *const prog_test_def, test_state: *const test_state) {
    let skipped_cnt = (*test_state).skip_cnt;
    let subtests_cnt = (*test_state).subtest_num;
    fprintf(env.stdout_saved, b"#%-*d %s:\0".as_ptr() as *const c_char, TEST_NUM_WIDTH, (*test).test_num, (*test).test_name);
    if (*test_state).error_cnt != 0 {
        fprintf(env.stdout_saved, b"FAIL\0".as_ptr() as *const c_char);
    } else if (*test).not_built {
        fprintf(env.stdout_saved, b"SKIP (not built)\0".as_ptr() as *const c_char);
    } else if skipped_cnt == 0 {
        fprintf(env.stdout_saved, b"OK\0".as_ptr() as *const c_char);
    } else if skipped_cnt == subtests_cnt || subtests_cnt == 0 {
        fprintf(env.stdout_saved, b"SKIP\0".as_ptr() as *const c_char);
    } else {
        fprintf(env.stdout_saved, b"OK (SKIP: %d/%d)\0".as_ptr() as *const c_char, skipped_cnt, subtests_cnt);
    }
    fprintf(env.stdout_saved, b"\n\0".as_ptr() as *const c_char);
}

unsafe fn print_test_log(log_buf: *mut c_char, log_cnt: size_t) {
    *log_buf.add(log_cnt) = 0;
    fprintf(env.stdout_saved, b"%s\0".as_ptr() as *const c_char, log_buf);
    if *log_buf.add(log_cnt - 1) != b'\n' as c_char {
        fprintf(env.stdout_saved, b"\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn print_subtest_name(test_num: c_int, subtest_num: c_int, test_name: *const c_char, subtest_name: *mut c_char, result: *mut c_char) {
    let mut test_num_str = [0 as c_char; 32];
    snprintf(test_num_str.as_mut_ptr(), test_num_str.len(), b"%d/%d\0".as_ptr() as *const c_char, test_num, subtest_num);
    fprintf(env.stdout_saved, b"#%-*s %s/%s\0".as_ptr() as *const c_char, TEST_NUM_WIDTH, test_num_str.as_ptr(), test_name, subtest_name);
    if !result.is_null() { fprintf(env.stdout_saved, b":%s\0".as_ptr() as *const c_char, result); }
    fprintf(env.stdout_saved, b"\n\0".as_ptr() as *const c_char);
}

unsafe fn jsonw_write_log_message(w: *mut json_writer_t, log_buf: *mut c_char, log_cnt: size_t) {
    /* open_memstream ensures that log_buf is null terminated; in parallel mode
     * log_buf can be NULL if there is no message.
     */
    if log_cnt != 0 {
        jsonw_string_field(w, b"message\0".as_ptr() as *const c_char, log_buf);
    } else {
        jsonw_string_field(w, b"message\0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char);
    }
}

unsafe fn dump_test_log(test: *const prog_test_def, test_state: *const test_state, skip_ok_subtests: bool, par_exec_result: bool, quiet: bool, w: *mut json_writer_t) {
    let test_failed = (*test_state).error_cnt > 0;
    let force_log = (*test_state).force_log;
    let print_test = verbose() || force_log || test_failed;
    if env.worker_id != -1 { return; }
    if verbose() && !par_exec_result { return; }
    if (*test_state).log_cnt != 0 && print_test && !quiet { print_test_log((*test_state).log_buf, (*test_state).log_cnt); }
    if !w.is_null() && print_test {
        jsonw_start_object(w);
        jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, (*test).test_name);
        jsonw_uint_field(w, b"number\0".as_ptr() as *const c_char, (*test).test_num);
        jsonw_write_log_message(w, (*test_state).log_buf, (*test_state).log_cnt);
        jsonw_bool_field(w, b"failed\0".as_ptr() as *const c_char, test_failed);
        jsonw_name(w, b"subtests\0".as_ptr() as *const c_char);
        jsonw_start_array(w);
    }
    for i in 0..(*test_state).subtest_num {
        let subtest_state = (*test_state).subtest_states.add(i as usize);
        let subtest_failed = (*subtest_state).error_cnt != 0;
        let subtest_filtered = (*subtest_state).filtered;
        let print_subtest = verbose() || force_log || subtest_failed;
        if (skip_ok_subtests && !subtest_failed) || subtest_filtered { continue; }
        if (*subtest_state).log_cnt != 0 && print_subtest && !quiet {
            print_test_log((*subtest_state).log_buf, (*subtest_state).log_cnt);
        }
        if !quiet {
            print_subtest_name((*test).test_num, i + 1, (*test).test_name, (*subtest_state).name, test_result((*subtest_state).error_cnt != 0, (*subtest_state).skipped) as *mut c_char);
        }
        if !w.is_null() && print_subtest {
            jsonw_start_object(w);
            jsonw_string_field(w, b"name\0".as_ptr() as *const c_char, (*subtest_state).name);
            jsonw_uint_field(w, b"number\0".as_ptr() as *const c_char, i + 1);
            jsonw_write_log_message(w, (*subtest_state).log_buf, (*subtest_state).log_cnt);
            jsonw_bool_field(w, b"failed\0".as_ptr() as *const c_char, subtest_failed);
            jsonw_end_object(w);
        }
    }
    if !w.is_null() && print_test {
        jsonw_end_array(w);
        jsonw_end_object(w);
    }
    if !quiet { print_test_result(test, test_state); }
}

unsafe fn reset_affinity() {
    let mut cpuset: cpu_set_t = zeroed();
    cpu_zero(&mut cpuset);
    for i in 0..env.nr_cpus { cpu_set(i, &mut cpuset); }
    let mut err = sched_setaffinity(0, size_of::<cpu_set_t>(), &cpuset);
    if err < 0 {
        fprintf(stderr, b"Failed to reset process affinity: %d!\n\0".as_ptr() as *const c_char, err);
        exit(EXIT_ERR_SETUP_INFRA);
    }
    err = pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpuset);
    if err < 0 {
        fprintf(stderr, b"Failed to reset thread affinity: %d!\n\0".as_ptr() as *const c_char, err);
        exit(EXIT_ERR_SETUP_INFRA);
    }
}

unsafe fn save_netns() {
    env.saved_netns_fd = open(b"/proc/self/ns/net\0".as_ptr() as *const c_char, O_RDONLY);
    if env.saved_netns_fd == -1 {
        perror(b"open(/proc/self/ns/net)\0".as_ptr() as *const c_char);
        exit(EXIT_ERR_SETUP_INFRA);
    }
}

unsafe fn restore_netns() {
    if setns(env.saved_netns_fd, CLONE_NEWNET) == -1 {
        perror(b"setns(CLONE_NEWNS)\0".as_ptr() as *const c_char);
        exit(EXIT_ERR_SETUP_INFRA);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__end_subtest() {
    let test = env.test;
    let test_state = env.test_state;
    let subtest_state = env.subtest_state;
    if (*subtest_state).error_cnt != 0 {
        (*test_state).error_cnt += 1;
    } else if !(*subtest_state).skipped {
        (*test_state).sub_succ_cnt += 1;
    } else {
        (*test_state).skip_cnt += 1;
    }
    if verbose() && env.workers == 0 {
        print_subtest_name((*test).test_num, (*test_state).subtest_num, (*test).test_name, (*subtest_state).name, test_result((*subtest_state).error_cnt != 0, (*subtest_state).skipped) as *mut c_char);
    }
    stdio_restore();
    env.subtest_state = ptr::null_mut();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__start_subtest_with_desc(subtest_name: *const c_char, subtest_desc: *const c_char) -> bool {
    let test = env.test;
    let state = env.test_state;
    let sub_state_size = size_of::<subtest_state>();
    if !env.subtest_state.is_null() { test__end_subtest(); }
    (*state).subtest_num += 1;
    let tmp = realloc((*state).subtest_states as *mut c_void, (*state).subtest_num as usize * sub_state_size);
    if tmp.is_null() {
        (*state).subtest_num -= 1;
        fprintf(stderr, b"Not enough memory to allocate subtest result\n\0".as_ptr() as *const c_char);
        return false;
    }
    (*state).subtest_states = tmp as *mut subtest_state;
    let subtest_state = (*state).subtest_states.add((*state).subtest_num as usize - 1);
    memset(subtest_state as *mut c_void, 0, sub_state_size);
    if subtest_name.is_null() || *subtest_name == 0 {
        fprintf(env.stderr_saved, b"Subtest #%d didn't provide sub-test name!\n\0".as_ptr() as *const c_char, (*state).subtest_num);
        return false;
    }
    let subtest_display_name = if !subtest_desc.is_null() { subtest_desc } else { subtest_name };
    (*subtest_state).name = strdup(subtest_display_name);
    if (*subtest_state).name.is_null() {
        fprintf(env.stderr_saved, b"Subtest #%d: failed to copy subtest name!\n\0".as_ptr() as *const c_char, (*state).subtest_num);
        return false;
    }
    if !should_run_subtest(&raw mut env.test_selector, &raw mut env.subtest_selector, (*state).subtest_num, (*test).test_name, subtest_name, subtest_desc) {
        (*subtest_state).filtered = true;
        return false;
    }
    (*subtest_state).should_tmon = match_subtest_desc(&raw mut env.tmon_selector.whitelist, (*test).test_name, subtest_name, subtest_desc);
    env.subtest_state = subtest_state;
    stdio_hijack_init(&raw mut (*subtest_state).log_buf, &raw mut (*subtest_state).log_cnt);
    watchdog_start();
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__start_subtest(subtest_name: *const c_char) -> bool {
    test__start_subtest_with_desc(subtest_name, ptr::null())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__force_log() { (*env.test_state).force_log = true; }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__skip() {
    if !env.subtest_state.is_null() { (*env.subtest_state).skipped = true; } else { (*env.test_state).skip_cnt += 1; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__fail() {
    if !env.subtest_state.is_null() { (*env.subtest_state).error_cnt += 1; } else { (*env.test_state).error_cnt += 1; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__join_cgroup(path: *const c_char) -> c_int {
    if !(*env.test).need_cgroup_cleanup {
        if setup_cgroup_environment() != 0 {
            fprintf(stderr, b"#%d %s: Failed to setup cgroup environment\n\0".as_ptr() as *const c_char, (*env.test).test_num, (*env.test).test_name);
            return -1;
        }
        (*env.test).need_cgroup_cleanup = true;
    }
    let fd = create_and_get_cgroup(path);
    if fd < 0 {
        fprintf(stderr, b"#%d %s: Failed to create cgroup '%s' (errno=%d)\n\0".as_ptr() as *const c_char, (*env.test).test_num, (*env.test).test_name, path, errno);
        return fd;
    }
    if join_cgroup(path) != 0 {
        fprintf(stderr, b"#%d %s: Failed to join cgroup '%s' (errno=%d)\n\0".as_ptr() as *const c_char, (*env.test).test_num, (*env.test).test_name, path, errno);
        return -1;
    }
    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int {
    let map = bpf_object__find_map_by_name(obj, name);
    if map.is_null() {
        fprintf(stdout, b"%s:FAIL:map '%s' not found\n\0".as_ptr() as *const c_char, test, name);
        test__fail();
        return -1;
    }
    bpf_map__fd(map)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int {
    let mut key: __u32 = 0;
    let mut next_key: __u32 = 0;
    let mut val_buf = [0 as c_char; PERF_MAX_STACK_DEPTH * size_of::<bpf_stack_build_id>()];
    let mut err = bpf_map_get_next_key(map1_fd, ptr::null(), &mut key as *mut _ as *mut c_void);
    if err != 0 { return err; }
    err = bpf_map_lookup_elem(map2_fd, &key as *const _ as *const c_void, val_buf.as_mut_ptr() as *mut c_void);
    if err != 0 { return err; }
    while bpf_map_get_next_key(map1_fd, &key as *const _ as *const c_void, &mut next_key as *mut _ as *mut c_void) == 0 {
        err = bpf_map_lookup_elem(map2_fd, &next_key as *const _ as *const c_void, val_buf.as_mut_ptr() as *mut c_void);
        if err != 0 { return err; }
        key = next_key;
    }
    if errno != ENOENT { return -1; }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compare_stack_ips(smap_fd: c_int, amap_fd: c_int, stack_trace_len: c_int) -> c_int {
    let mut key: __u32 = 0;
    let mut next_key: __u32 = 0;
    let mut err = -(ENOMEM as c_int);
    let val_buf1 = malloc(stack_trace_len as usize) as *mut c_char;
    let val_buf2 = malloc(stack_trace_len as usize) as *mut c_char;
    if val_buf1.is_null() || val_buf2.is_null() { goto_out_compare_stack_ips!(val_buf1, val_buf2, err); }
    err = 0;
    let mut cur_key_p: *mut __u32 = ptr::null_mut();
    let mut next_key_p: *mut __u32 = &mut key;
    while bpf_map_get_next_key(smap_fd, cur_key_p as *const c_void, next_key_p as *mut c_void) == 0 {
        err = bpf_map_lookup_elem(smap_fd, next_key_p as *const c_void, val_buf1 as *mut c_void);
        if err != 0 { break; }
        err = bpf_map_lookup_elem(amap_fd, next_key_p as *const c_void, val_buf2 as *mut c_void);
        if err != 0 { break; }
        for i in 0..stack_trace_len {
            if *val_buf1.add(i as usize) != *val_buf2.add(i as usize) {
                err = -1;
                break;
            }
        }
        if err != 0 { break; }
        key = *next_key_p;
        cur_key_p = &mut key;
        next_key_p = &mut next_key;
    }
    if err == 0 && errno != ENOENT { err = -1; }
    free(val_buf1 as *mut c_void);
    free(val_buf2 as *mut c_void);
    err
}

macro_rules! goto_out_compare_stack_ips {
    ($v1:expr, $v2:expr, $err:expr) => {{
        free($v1 as *mut c_void);
        free($v2 as *mut c_void);
        return $err;
    }};
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netns_new(nsname: *const c_char, open_: bool) -> *mut netns_obj {
    let netns_obj = malloc(size_of::<netns_obj>()) as *mut netns_obj;
    if netns_obj.is_null() { return ptr::null_mut(); }
    memset(netns_obj as *mut c_void, 0, size_of::<netns_obj>());
    (*netns_obj).nsname = strdup(nsname);
    if (*netns_obj).nsname.is_null() { return netns_fail(netns_obj, nsname); }
    if make_netns(nsname) != 0 { return netns_fail(netns_obj, nsname); }
    if (*env.test).should_tmon || (!env.subtest_state.is_null() && (*env.subtest_state).should_tmon) {
        let test_name = (*env.test).test_name;
        let subtest_name = if !env.subtest_state.is_null() { (*env.subtest_state).name } else { ptr::null_mut() };
        (*netns_obj).tmon = traffic_monitor_start(nsname, test_name, subtest_name);
        if (*netns_obj).tmon.is_null() {
            fprintf(stderr, b"Failed to start traffic monitor for %s\n\0".as_ptr() as *const c_char, nsname);
            return netns_fail(netns_obj, nsname);
        }
    } else {
        (*netns_obj).tmon = ptr::null_mut();
    }
    if open_ {
        (*netns_obj).nstoken = open_netns(nsname);
        if (*netns_obj).nstoken.is_null() { return netns_fail(netns_obj, nsname); }
    }
    netns_obj
}

unsafe fn netns_fail(netns_obj: *mut netns_obj, nsname: *const c_char) -> *mut netns_obj {
    traffic_monitor_stop((*netns_obj).tmon);
    remove_netns(nsname);
    free((*netns_obj).nsname as *mut c_void);
    free(netns_obj as *mut c_void);
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn netns_free(netns_obj: *mut netns_obj) {
    if netns_obj.is_null() { return; }
    traffic_monitor_stop((*netns_obj).tmon);
    close_netns((*netns_obj).nstoken);
    remove_netns((*netns_obj).nsname);
    free((*netns_obj).nsname as *mut c_void);
    free(netns_obj as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_libbpf_log_capture() -> c_int {
    if !libbpf_capture_stream.is_null() {
        PRINT_FAIL(b"%s: libbpf_capture_stream != NULL\n\0".as_ptr() as *const c_char, b"start_libbpf_log_capture\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    libbpf_capture_stream = open_memstream(&raw mut libbpf_output_capture.buf, &raw mut libbpf_output_capture.buf_sz);
    if libbpf_capture_stream.is_null() {
        PRINT_FAIL(b"%s: open_memstream failed errno=%d\n\0".as_ptr() as *const c_char, b"start_libbpf_log_capture\0".as_ptr() as *const c_char, errno);
        return -EINVAL;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_libbpf_log_capture() -> *mut c_char {
    if libbpf_capture_stream.is_null() { return ptr::null_mut(); }
    fputc(0, libbpf_capture_stream);
    fclose(libbpf_capture_stream);
    libbpf_capture_stream = ptr::null_mut();
    let buf = libbpf_output_capture.buf;
    memset(&raw mut libbpf_output_capture as *mut c_void, 0, size_of::<libbpf_output_capture_t>());
    buf
}

unsafe extern "C" fn libbpf_print_fn(level: c_int, format: *const c_char, args: va_list) -> c_int {
    if !libbpf_capture_stream.is_null() && level != LIBBPF_DEBUG {
        let args2 = args;
        vfprintf(libbpf_capture_stream, format, args2);
    }
    if env.verbosity < VERBOSE_VERY && level == LIBBPF_DEBUG { return 0; }
    vfprintf(stdout, format, args);
    0
}

unsafe fn free_test_filter_set(set: *const test_filter_set) {
    if set.is_null() { return; }
    for i in 0..(*set).cnt {
        let test = (*set).tests.add(i as usize);
        free((*test).name as *mut c_void);
        for j in 0..(*test).subtest_cnt { free(*(*test).subtests.add(j as usize) as *mut c_void); }
        free((*test).subtests as *mut c_void);
    }
    free((*set).tests as *mut c_void);
}

unsafe fn free_test_selector(test_selector: *mut test_selector) {
    free_test_filter_set(&raw const (*test_selector).blacklist);
    free_test_filter_set(&raw const (*test_selector).whitelist);
    free((*test_selector).num_set as *mut c_void);
}

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    let envp = (*state).input as *mut test_env;
    let mut err = 0;
    match key {
        ARG_TEST_NUM => {
            let subtest_str = strchr(arg, b'/' as c_int);
            if !subtest_str.is_null() {
                *subtest_str = 0;
                if parse_num_list(subtest_str.add(1), &raw mut (*envp).subtest_selector.num_set, &raw mut (*envp).subtest_selector.num_set_len) != 0 {
                    fprintf(stderr, b"Failed to parse subtest numbers.\n\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
            }
            if parse_num_list(arg, &raw mut (*envp).test_selector.num_set, &raw mut (*envp).test_selector.num_set_len) != 0 {
                fprintf(stderr, b"Failed to parse test numbers.\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }
        ARG_TEST_NAME_GLOB_ALLOWLIST | ARG_TEST_NAME => {
            err = if *arg == b'@' as c_char {
                parse_test_list_file(arg.add(1), &raw mut (*envp).test_selector.whitelist, key == ARG_TEST_NAME_GLOB_ALLOWLIST)
            } else {
                parse_test_list(arg, &raw mut (*envp).test_selector.whitelist, key == ARG_TEST_NAME_GLOB_ALLOWLIST)
            };
        }
        ARG_TEST_NAME_GLOB_DENYLIST | ARG_TEST_NAME_BLACKLIST => {
            err = if *arg == b'@' as c_char {
                parse_test_list_file(arg.add(1), &raw mut (*envp).test_selector.blacklist, key == ARG_TEST_NAME_GLOB_DENYLIST)
            } else {
                parse_test_list(arg, &raw mut (*envp).test_selector.blacklist, key == ARG_TEST_NAME_GLOB_DENYLIST)
            };
        }
        ARG_VERIFIER_STATS => (*envp).verifier_stats = true,
        ARG_VERBOSE => {
            (*envp).verbosity = VERBOSE_NORMAL;
            if !arg.is_null() {
                if strcmp(arg, b"v\0".as_ptr() as *const c_char) == 0 {
                    (*envp).verbosity = VERBOSE_VERY;
                    extra_prog_load_log_flags = 1;
                } else if strcmp(arg, b"vv\0".as_ptr() as *const c_char) == 0 {
                    (*envp).verbosity = VERBOSE_SUPER;
                    extra_prog_load_log_flags = 2;
                } else {
                    fprintf(stderr, b"Unrecognized verbosity setting ('%s'), only -v and -vv are supported\n\0".as_ptr() as *const c_char, arg);
                    return -EINVAL;
                }
            }
            env_verbosity = (*envp).verbosity;
            if verbose() && setenv(b"SELFTESTS_VERBOSE\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char, 1) == -1 {
                fprintf(stderr, b"Unable to setenv SELFTESTS_VERBOSE=1 (errno=%d)\0".as_ptr() as *const c_char, errno);
                return -EINVAL;
            }
        }
        ARG_GET_TEST_CNT => (*envp).get_test_cnt = true,
        ARG_LIST_TEST_NAMES => (*envp).list_test_names = true,
        ARG_NUM_WORKERS => {
            if !arg.is_null() {
                (*envp).workers = atoi(arg);
                if (*envp).workers == 0 {
                    fprintf(stderr, b"Invalid number of worker: %s.\0".as_ptr() as *const c_char, arg);
                    return -EINVAL;
                }
            } else {
                (*envp).workers = get_nprocs();
            }
        }
        ARG_DEBUG => (*envp).debug = true,
        ARG_NO_ERROR_SUMMARY => (*envp).error_summary = false,
        ARG_JSON_SUMMARY => {
            (*envp).json = fopen(arg, b"w\0".as_ptr() as *const c_char);
            if (*envp).json.is_null() {
                perror(b"Failed to open json summary file\0".as_ptr() as *const c_char);
                return -errno;
            }
        }
        ARGP_KEY_ARG => argp_usage(state),
        ARGP_KEY_END => {}
        ARG_TRAFFIC_MONITOR => {
            err = if *arg == b'@' as c_char {
                parse_test_list_file(arg.add(1), &raw mut (*envp).tmon_selector.whitelist, true)
            } else {
                parse_test_list(arg, &raw mut (*envp).tmon_selector.whitelist, true)
            };
        }
        ARG_WATCHDOG_TIMEOUT => {
            (*envp).secs_till_kill = atoi(arg);
            if (*envp).secs_till_kill < 0 {
                fprintf(stderr, b"Invalid watchdog timeout: %s.\n\0".as_ptr() as *const c_char, arg);
                return -EINVAL;
            }
            if (*envp).secs_till_kill < (*envp).secs_till_notify { (*envp).secs_till_notify = 0; }
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cd_flavor_subdir(exec_name: *const c_char) -> c_int {
    let mut flavor = strrchr(exec_name, b'/' as c_int);
    if flavor.is_null() { flavor = exec_name as *mut c_char; } else { flavor = flavor.add(1); }
    flavor = strrchr(flavor, b'-' as c_int);
    if flavor.is_null() { return 0; }
    flavor = flavor.add(1);
    if verbose() { fprintf(stdout, b"Switching to flavor '%s' subdirectory...\n\0".as_ptr() as *const c_char, flavor); }
    chdir(flavor)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_module_test_read(read_sz: c_int) -> c_int {
    let fd = open(BPF_TESTMOD_TEST_FILE, O_RDONLY);
    let err = -errno;
    if !ASSERT_GE(fd, 0, b"testmod_file_open\0".as_ptr() as *const c_char) { return err; }
    read(fd, ptr::null_mut(), read_sz as usize);
    close(fd);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_module_test_write(write_sz: c_int) -> c_int {
    let buf = malloc(write_sz as usize) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    memset(buf as *mut c_void, b'a' as c_int, write_sz as usize);
    *buf.add(write_sz as usize - 1) = 0;
    let fd = open(BPF_TESTMOD_TEST_FILE, O_WRONLY);
    let err = -errno;
    if !ASSERT_GE(fd, 0, b"testmod_file_open\0".as_ptr() as *const c_char) {
        free(buf as *mut c_void);
        return err;
    }
    write(fd, buf as *const c_void, write_sz as usize);
    close(fd);
    free(buf as *mut c_void);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_sysctl(sysctl: *const c_char, value: *const c_char) -> c_int {
    let fd = open(sysctl, O_WRONLY);
    if !ASSERT_NEQ(fd, -1, b"open sysctl\0".as_ptr() as *const c_char) { return -1; }
    let len = strlen(value) as c_int;
    let err = write(fd, value as *const c_void, len as usize) as c_int;
    close(fd);
    if !ASSERT_EQ(err, len, b"write sysctl\0".as_ptr() as *const c_char) { return -1; }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_bpf_max_tramp_links_from(btfp: *mut btf) -> c_int {
    let type_cnt = btf__type_cnt(btfp);
    let mut i: __u32 = 1;
    while i < type_cnt {
        let t = btf__type_by_id(btfp, i);
        if t.is_null() || !btf_is_enum(t) || (*t).name_off != 0 {
            i += 1;
            continue;
        }
        let mut e = btf_enum(t);
        let vlen = btf_vlen(t);
        let mut j: __u32 = 0;
        while j < vlen {
            let name = btf__str_by_offset(btfp, (*e).name_off);
            if !name.is_null() && strcmp(name, b"BPF_MAX_TRAMP_LINKS\0".as_ptr() as *const c_char) == 0 { return (*e).val; }
            j += 1;
            e = e.add(1);
        }
        i += 1;
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_bpf_max_tramp_links() -> c_int {
    let vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(vmlinux_btf, b"vmlinux btf\0".as_ptr() as *const c_char) { return -1; }
    let ret = get_bpf_max_tramp_links_from(vmlinux_btf);
    btf__free(vmlinux_btf);
    ret
}

unsafe fn dump_crash_log() {
    fflush(stdout);
    stdout = env.stdout_saved;
    stderr = env.stderr_saved;
    if !env.test.is_null() {
        (*env.test_state).error_cnt += 1;
        dump_test_log(env.test, env.test_state, true, false, false, ptr::null_mut());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crash_handler(signum: c_int) {
    let mut bt = [ptr::null_mut(); MAX_BACKTRACE_SZ];
    let sz = backtrace(bt.as_mut_ptr(), bt.len() as c_int);
    dump_crash_log();
    if env.worker_id != -1 { fprintf(stderr, b"[%d]: \0".as_ptr() as *const c_char, env.worker_id); }
    fprintf(stderr, b"Caught signal #%d!\nStack trace:\n\0".as_ptr() as *const c_char, signum);
    backtrace_symbols_fd(bt.as_ptr(), sz, STDERR_FILENO);
}

/* #ifdef __SANITIZE_ADDRESS__ */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __asan_on_error() { dump_crash_log(); }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hexdump(prefix: *const c_char, buf: *const c_void, len: size_t) {
    for i in 0..len {
        if i % 16 == 0 {
            if i != 0 { fprintf(stdout, b"\n\0".as_ptr() as *const c_char); }
            fprintf(stdout, b"%s\0".as_ptr() as *const c_char, prefix);
        }
        if i != 0 && i % 8 == 0 && i % 16 != 0 { fprintf(stdout, b"\t\0".as_ptr() as *const c_char); }
        fprintf(stdout, b"%02X \0".as_ptr() as *const c_char, *(buf as *const u8).add(i) as c_int);
    }
    fprintf(stdout, b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn sigint_handler(_signum: c_int) {
    for i in 0..env.workers {
        if *env.worker_socks.add(i as usize) > 0 { close(*env.worker_socks.add(i as usize)); }
    }
}

unsafe fn str_msg(msgp: *const msg, buf: *mut c_char) -> *const c_char {
    match (*msgp).type_ {
        MSG_DO_TEST => sprintf(buf, b"MSG_DO_TEST %d\0".as_ptr() as *const c_char, (*msgp).payload.do_test.num),
        MSG_TEST_DONE => sprintf(buf, b"MSG_TEST_DONE %d (log: %d)\0".as_ptr() as *const c_char, (*msgp).payload.test_done.num, (*msgp).payload.test_done.have_log as c_int),
        MSG_SUBTEST_DONE => sprintf(buf, b"MSG_SUBTEST_DONE %d (log: %d)\0".as_ptr() as *const c_char, (*msgp).payload.subtest_done.num, (*msgp).payload.subtest_done.have_log as c_int),
        MSG_TEST_LOG => sprintf(buf, b"MSG_TEST_LOG (cnt: %zu, last: %d)\0".as_ptr() as *const c_char, strlen((*msgp).payload.test_log.log_buf.as_ptr()), (*msgp).payload.test_log.is_last as c_int),
        MSG_EXIT => sprintf(buf, b"MSG_EXIT\0".as_ptr() as *const c_char),
        _ => sprintf(buf, b"UNKNOWN\0".as_ptr() as *const c_char),
    };
    buf
}

unsafe fn send_message(sock: c_int, msgp: *const msg) -> c_int {
    let mut buf = [0 as c_char; 256];
    if env.debug { fprintf(stderr, b"Sending msg: %s\n\0".as_ptr() as *const c_char, str_msg(msgp, buf.as_mut_ptr())); }
    send(sock, msgp as *const c_void, size_of::<msg>(), 0) as c_int
}

unsafe fn recv_message(sock: c_int, msgp: *mut msg) -> c_int {
    memset(msgp as *mut c_void, 0, size_of::<msg>());
    let ret = recv(sock, msgp as *mut c_void, size_of::<msg>(), 0) as c_int;
    if ret >= 0 && env.debug {
        let mut buf = [0 as c_char; 256];
        fprintf(stderr, b"Received msg: %s\n\0".as_ptr() as *const c_char, str_msg(msgp, buf.as_mut_ptr()));
    }
    ret
}

unsafe fn ns_is_needed(test_name: *const c_char) -> bool {
    if strlen(test_name) < 3 { return false; }
    strncmp(test_name, b"ns_\0".as_ptr() as *const c_char, 3) == 0
}

unsafe fn run_one_test(test_num: c_int) {
    let test = prog_test_defs.add(test_num as usize);
    let state = test_states.add(test_num as usize);
    let mut ns: *mut netns_obj = ptr::null_mut();
    env.test = test;
    env.test_state = state;
    stdio_hijack(&raw mut (*state).log_buf, &raw mut (*state).log_cnt);
    watchdog_start();
    if ns_is_needed((*test).test_name) { ns = netns_new((*test).test_name, true); }
    if let Some(run) = (*test).run_test { run(); } else if let Some(run) = (*test).run_serial_test { run(); }
    netns_free(ns);
    watchdog_stop();
    if !env.subtest_state.is_null() { test__end_subtest(); }
    (*state).tested = true;
    stdio_restore();
    if verbose() && env.worker_id == -1 { print_test_result(test, state); }
    reset_affinity();
    restore_netns();
    if (*test).need_cgroup_cleanup { cleanup_cgroup_environment(); }
    free(stop_libbpf_log_capture() as *mut c_void);
    dump_test_log(test, state, false, false, false, ptr::null_mut());
}

unsafe fn read_prog_test_msg(sock_fd: c_int, msgp: *mut msg, type_: msg_type) -> c_int {
    if recv_message(sock_fd, msgp) < 0 { return 1; }
    if (*msgp).type_ != type_ {
        printf(b"%s: unexpected message type %d. expected %d\n\0".as_ptr() as *const c_char, b"read_prog_test_msg\0".as_ptr() as *const c_char, (*msgp).type_, type_);
        return 1;
    }
    0
}

unsafe fn dispatch_thread_read_log(sock_fd: c_int, log_buf: *mut *mut c_char, log_cnt: *mut size_t) -> c_int {
    let log_fp = open_memstream(log_buf, log_cnt);
    if log_fp.is_null() { return 1; }
    let mut result = 0;
    loop {
        let mut msgv: msg = zeroed();
        if read_prog_test_msg(sock_fd, &mut msgv, MSG_TEST_LOG) != 0 {
            result = 1;
            break;
        }
        fprintf(log_fp, b"%s\0".as_ptr() as *const c_char, msgv.payload.test_log.log_buf.as_ptr());
        if msgv.payload.test_log.is_last { break; }
    }
    fclose(log_fp);
    result
}

unsafe fn dispatch_thread_send_subtests(sock_fd: c_int, state: *mut test_state) -> c_int {
    let subtest_num = (*state).subtest_num;
    (*state).subtest_states = calloc(subtest_num as usize, size_of::<subtest_state>()) as *mut subtest_state;
    if (*state).subtest_states.is_null() {
        (*state).subtest_num = 0;
        return -ENOMEM;
    }
    for i in 0..subtest_num {
        let subtest_state = (*state).subtest_states.add(i as usize);
        let mut msgv: msg = zeroed();
        if read_prog_test_msg(sock_fd, &mut msgv, MSG_SUBTEST_DONE) != 0 { return 1; }
        (*subtest_state).name = strdup(msgv.payload.subtest_done.name.as_ptr());
        (*subtest_state).error_cnt = msgv.payload.subtest_done.error_cnt;
        (*subtest_state).skipped = msgv.payload.subtest_done.skipped;
        (*subtest_state).filtered = msgv.payload.subtest_done.filtered;
        if msgv.payload.subtest_done.have_log {
            if dispatch_thread_read_log(sock_fd, &raw mut (*subtest_state).log_buf, &raw mut (*subtest_state).log_cnt) != 0 { return 1; }
        }
    }
    0
}

unsafe extern "C" fn dispatch_thread(ctx: *mut c_void) -> *mut c_void {
    let data = ctx as *mut dispatch_data;
    let sock_fd = (*data).sock_fd;
    loop {
        let test_to_run: c_int;
        let test: *mut prog_test_def;
        pthread_mutex_lock(&raw mut current_test_lock);
        if current_test_idx >= prog_test_cnt {
            pthread_mutex_unlock(&raw mut current_test_lock);
            break;
        }
        test = prog_test_defs.add(current_test_idx as usize);
        test_to_run = current_test_idx;
        current_test_idx += 1;
        pthread_mutex_unlock(&raw mut current_test_lock);
        if !(*test).should_run || (*test).run_serial_test.is_some() { continue; }
        let mut msg_do: msg = zeroed();
        msg_do.type_ = MSG_DO_TEST;
        msg_do.payload.do_test.num = test_to_run;
        if send_message(sock_fd, &msg_do) < 0 {
            perror(b"Fail to send command\0".as_ptr() as *const c_char);
            break;
        }
        *env.worker_current_test.add((*data).worker_id as usize) = test_to_run;
        let mut protocol_error = false;
        let mut msgv: msg = zeroed();
        if read_prog_test_msg(sock_fd, &mut msgv, MSG_TEST_DONE) != 0 { protocol_error = true; }
        if !protocol_error && test_to_run != msgv.payload.test_done.num { protocol_error = true; }
        if protocol_error {
            if env.debug { fprintf(stderr, b"[%d]: Protocol/IO error: %s.\n\0".as_ptr() as *const c_char, (*data).worker_id, ptr::null::<c_char>()); }
            break;
        }
        let state = test_states.add(test_to_run as usize);
        (*state).tested = true;
        (*state).error_cnt = msgv.payload.test_done.error_cnt;
        (*state).skip_cnt = msgv.payload.test_done.skip_cnt;
        (*state).sub_succ_cnt = msgv.payload.test_done.sub_succ_cnt;
        (*state).subtest_num = msgv.payload.test_done.subtest_num;
        if msgv.payload.test_done.have_log {
            if dispatch_thread_read_log(sock_fd, &raw mut (*state).log_buf, &raw mut (*state).log_cnt) != 0 { break; }
        }
        if (*state).subtest_num != 0 && dispatch_thread_send_subtests(sock_fd, state) != 0 { break; }
        pthread_mutex_lock(&raw mut stdout_output_lock);
        dump_test_log(test, state, false, true, false, ptr::null_mut());
        pthread_mutex_unlock(&raw mut stdout_output_lock);
    }
    let mut msg_exit: msg = zeroed();
    msg_exit.type_ = MSG_EXIT;
    if send_message(sock_fd, &msg_exit) < 0 && env.debug {
        fprintf(stderr, b"[%d]: send_message msg_exit.\n\0".as_ptr() as *const c_char, (*data).worker_id);
    }
    ptr::null_mut()
}

unsafe fn calculate_summary_and_print_errors(envp: *mut test_env) {
    let mut succ_cnt = 0;
    let mut fail_cnt = 0;
    let mut sub_succ_cnt = 0;
    let mut sub_fail_cnt = 0;
    let mut skip_cnt = 0;
    let mut w: *mut json_writer_t = ptr::null_mut();
    for i in 0..prog_test_cnt {
        let test = prog_test_defs.add(i as usize);
        let state = test_states.add(i as usize);
        if !(*state).tested { continue; }
        sub_succ_cnt += (*state).sub_succ_cnt;
        skip_cnt += (*state).skip_cnt;
        if (*state).error_cnt != 0 {
            fail_cnt += 1;
            for j in 0..(*state).subtest_num {
                if (*(*state).subtest_states.add(j as usize)).error_cnt != 0 { sub_fail_cnt += 1; }
            }
        } else if !(*test).not_built {
            succ_cnt += 1;
        }
    }
    if !(*envp).json.is_null() {
        w = jsonw_new((*envp).json);
        if w.is_null() { fprintf((*envp).stderr_saved, b"Failed to create new JSON stream.\0".as_ptr() as *const c_char); }
    }
    if !w.is_null() {
        jsonw_start_object(w);
        jsonw_uint_field(w, b"success\0".as_ptr() as *const c_char, succ_cnt);
        jsonw_uint_field(w, b"success_subtest\0".as_ptr() as *const c_char, sub_succ_cnt);
        jsonw_uint_field(w, b"skipped\0".as_ptr() as *const c_char, skip_cnt);
        jsonw_uint_field(w, b"failed\0".as_ptr() as *const c_char, fail_cnt);
        jsonw_uint_field(w, b"failed_subtest\0".as_ptr() as *const c_char, sub_fail_cnt);
        jsonw_name(w, b"results\0".as_ptr() as *const c_char);
        jsonw_start_array(w);
    }
    if !verbose() && fail_cnt != 0 && ((*envp).error_summary || !w.is_null()) {
        if (*envp).error_summary { printf(b"\nAll error logs:\n\0".as_ptr() as *const c_char); }
        for i in 0..prog_test_cnt {
            let test = prog_test_defs.add(i as usize);
            let state = test_states.add(i as usize);
            if !(*state).tested || (*state).error_cnt == 0 { continue; }
            dump_test_log(test, state, true, true, !(*envp).error_summary, w);
        }
    }
    if !w.is_null() {
        jsonw_end_array(w);
        jsonw_end_object(w);
        jsonw_destroy(&mut w);
    }
    if !(*envp).json.is_null() { fclose((*envp).json); }
    if (*envp).not_built_cnt != 0 {
        printf(b"Summary: %d/%d PASSED, %d SKIPPED (%d not built), %d/%d FAILED\n\0".as_ptr() as *const c_char, succ_cnt, sub_succ_cnt, skip_cnt, (*envp).not_built_cnt, fail_cnt, sub_fail_cnt);
    } else {
        printf(b"Summary: %d/%d PASSED, %d SKIPPED, %d/%d FAILED\n\0".as_ptr() as *const c_char, succ_cnt, sub_succ_cnt, skip_cnt, fail_cnt, sub_fail_cnt);
    }
    (*envp).succ_cnt = succ_cnt;
    (*envp).sub_succ_cnt = sub_succ_cnt;
    (*envp).fail_cnt = fail_cnt;
    (*envp).skip_cnt = skip_cnt;
}

unsafe fn server_main() {
    let sigact_int = sigaction { sa_handler: Some(sigint_handler), sa_flags: SA_RESETHAND };
    sigaction(SIGINT, &sigact_int, ptr::null_mut());
    let dispatcher_threads = calloc(size_of::<pthread_t>(), env.workers as usize) as *mut pthread_t;
    let data = calloc(size_of::<dispatch_data>(), env.workers as usize) as *mut dispatch_data;
    env.worker_current_test = calloc(size_of::<c_int>(), env.workers as usize) as *mut c_int;
    for i in 0..env.workers {
        (*data.add(i as usize)).worker_id = i;
        (*data.add(i as usize)).sock_fd = *env.worker_socks.add(i as usize);
        let rc = pthread_create(dispatcher_threads.add(i as usize), ptr::null(), dispatch_thread, data.add(i as usize) as *mut c_void);
        if rc != 0 {
            perror(b"Failed to launch dispatcher thread\0".as_ptr() as *const c_char);
            exit(EXIT_ERR_SETUP_INFRA);
        }
    }
    for i in 0..env.workers {
        loop {
            let ret = pthread_tryjoin_np(*dispatcher_threads.add(i as usize), ptr::null_mut());
            if ret == 0 { break; }
            if ret == EBUSY {
                if env.debug { fprintf(stderr, b"Still waiting for thread %d (test %d).\n\0".as_ptr() as *const c_char, i, *env.worker_current_test.add(i as usize) + 1); }
                usleep(1000 * 1000);
                continue;
            }
            fprintf(stderr, b"Unexpected error joining dispatcher thread: %d\0".as_ptr() as *const c_char, ret);
            break;
        }
    }
    free(dispatcher_threads as *mut c_void);
    free(env.worker_current_test as *mut c_void);
    free(data as *mut c_void);
    save_netns();
    for i in 0..prog_test_cnt {
        let test = prog_test_defs.add(i as usize);
        if !(*test).should_run || (*test).run_serial_test.is_none() { continue; }
        run_one_test(i);
    }
    for i in 0..prog_test_cnt {
        let test = prog_test_defs.add(i as usize);
        let state = test_states.add(i as usize);
        if (*test).not_built && (*test).selected {
            (*state).tested = true;
            (*state).skip_cnt = 1;
            env.not_built_cnt += 1;
            print_test_result(test, state);
        }
    }
    fflush(stderr);
    fflush(stdout);
    calculate_summary_and_print_errors(&raw mut env);
    for i in 0..env.workers {
        let mut wstatus = 0;
        let pid = waitpid(*env.worker_pids.add(i as usize), &mut wstatus, 0);
        if pid != *env.worker_pids.add(i as usize) { perror(b"Unable to reap worker\0".as_ptr() as *const c_char); }
    }
}

unsafe fn worker_main_send_log(sock: c_int, log_buf: *mut c_char, log_cnt: size_t) {
    let mut src = log_buf;
    let mut slen = log_cnt;
    while slen != 0 {
        let mut msg_log: msg = zeroed();
        msg_log.type_ = MSG_TEST_LOG;
        let dest = msg_log.payload.test_log.log_buf.as_mut_ptr();
        let len = if slen >= MAX_LOG_TRUNK_SIZE { MAX_LOG_TRUNK_SIZE } else { slen };
        memcpy(dest as *mut c_void, src as *const c_void, len);
        src = src.add(len);
        slen -= len;
        if slen == 0 { msg_log.payload.test_log.is_last = true; }
        assert(send_message(sock, &msg_log) >= 0);
    }
}

unsafe fn free_subtest_state(state: *mut subtest_state) {
    if !(*state).log_buf.is_null() {
        free((*state).log_buf as *mut c_void);
        (*state).log_buf = ptr::null_mut();
        (*state).log_cnt = 0;
    }
    free((*state).name as *mut c_void);
    (*state).name = ptr::null_mut();
}

unsafe fn worker_main_send_subtests(sock: c_int, state: *mut test_state) -> c_int {
    let mut result = 0;
    let mut msgv: msg = zeroed();
    msgv.type_ = MSG_SUBTEST_DONE;
    let mut i = 0;
    while i < (*state).subtest_num {
        let subtest_state = (*state).subtest_states.add(i as usize);
        msgv.payload.subtest_done.num = i;
        strscpy(msgv.payload.subtest_done.name.as_mut_ptr(), (*subtest_state).name, MAX_SUBTEST_NAME);
        msgv.payload.subtest_done.error_cnt = (*subtest_state).error_cnt;
        msgv.payload.subtest_done.skipped = (*subtest_state).skipped;
        msgv.payload.subtest_done.filtered = (*subtest_state).filtered;
        msgv.payload.subtest_done.have_log = false;
        if verbose() || (*state).force_log || (*subtest_state).error_cnt != 0 {
            if (*subtest_state).log_cnt != 0 { msgv.payload.subtest_done.have_log = true; }
        }
        if send_message(sock, &msgv) < 0 {
            perror(b"Fail to send message done\0".as_ptr() as *const c_char);
            result = 1;
            break;
        }
        if msgv.payload.subtest_done.have_log { worker_main_send_log(sock, (*subtest_state).log_buf, (*subtest_state).log_cnt); }
        free_subtest_state(subtest_state);
        i += 1;
    }
    while i < (*state).subtest_num {
        free_subtest_state((*state).subtest_states.add(i as usize));
        i += 1;
    }
    free((*state).subtest_states as *mut c_void);
    result
}

unsafe fn worker_main(sock: c_int) -> c_int {
    save_netns();
    watchdog_init();
    loop {
        let mut msgv: msg = zeroed();
        if recv_message(sock, &mut msgv) < 0 { break; }
        match msgv.type_ {
            MSG_EXIT => {
                if env.debug { fprintf(stderr, b"[%d]: worker exit.\n\0".as_ptr() as *const c_char, env.worker_id); }
                break;
            }
            MSG_DO_TEST => {
                let test_to_run = msgv.payload.do_test.num;
                let test = prog_test_defs.add(test_to_run as usize);
                let state = test_states.add(test_to_run as usize);
                if env.debug { fprintf(stderr, b"[%d]: #%d:%s running.\n\0".as_ptr() as *const c_char, env.worker_id, test_to_run + 1, (*test).test_name); }
                run_one_test(test_to_run);
                let mut done: msg = zeroed();
                done.type_ = MSG_TEST_DONE;
                done.payload.test_done.num = test_to_run;
                done.payload.test_done.error_cnt = (*state).error_cnt;
                done.payload.test_done.skip_cnt = (*state).skip_cnt;
                done.payload.test_done.sub_succ_cnt = (*state).sub_succ_cnt;
                done.payload.test_done.subtest_num = (*state).subtest_num;
                done.payload.test_done.have_log = false;
                if verbose() || (*state).force_log || (*state).error_cnt != 0 {
                    if (*state).log_cnt != 0 { done.payload.test_done.have_log = true; }
                }
                if send_message(sock, &done) < 0 {
                    perror(b"Fail to send message done\0".as_ptr() as *const c_char);
                    break;
                }
                if done.payload.test_done.have_log { worker_main_send_log(sock, (*state).log_buf, (*state).log_cnt); }
                if !(*state).log_buf.is_null() {
                    free((*state).log_buf as *mut c_void);
                    (*state).log_buf = ptr::null_mut();
                    (*state).log_cnt = 0;
                }
                if (*state).subtest_num != 0 && worker_main_send_subtests(sock, state) != 0 { break; }
                if env.debug { fprintf(stderr, b"[%d]: #%d:%s done.\n\0".as_ptr() as *const c_char, env.worker_id, test_to_run + 1, (*test).test_name); }
            }
            _ => {
                if env.debug { fprintf(stderr, b"[%d]: unknown message.\n\0".as_ptr() as *const c_char, env.worker_id); }
                return -1;
            }
        }
    }
    0
}

unsafe fn free_test_states() {
    for i in 0..prog_test_cnt {
        let test_state = test_states.add(i as usize);
        for j in 0..(*test_state).subtest_num {
            free_subtest_state((*test_state).subtest_states.add(j as usize));
        }
        free((*test_state).subtest_states as *mut c_void);
        free((*test_state).log_buf as *mut c_void);
        (*test_state).subtest_states = ptr::null_mut();
        (*test_state).log_buf = ptr::null_mut();
    }
}

unsafe fn register_session_key(key_data: *const c_char, key_data_size: size_t) -> __u32 {
    syscall(__NR_add_key, b"asymmetric\0".as_ptr() as *const c_char, b"libbpf_session_key\0".as_ptr() as *const c_char, key_data as *const c_void, key_data_size, KEY_SPEC_SESSION_KEYRING) as __u32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static ARGP: argp = argp { options: opts.as_ptr(), parser: Some(parse_arg), doc: argp_program_doc.as_ptr() as *const c_char };
    let sigact = sigaction { sa_handler: Some(crash_handler), sa_flags: SA_RESETHAND };
    sigaction(SIGSEGV, &sigact, ptr::null_mut());
    env.stdout_saved = stdout;
    env.stderr_saved = stderr;
    env.secs_till_notify = 10;
    env.secs_till_kill = 120;
    env.error_summary = true;
    let mut err = argp_parse(&ARGP, argc, argv, 0, ptr::null_mut(), &raw mut env as *mut c_void);
    if err != 0 { return err; }
    err = cd_flavor_subdir(*argv.add(0));
    if err != 0 { return err; }
    watchdog_init();
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    libbpf_set_print(libbpf_print_fn);
    err = register_session_key(test_progs_verification_cert.as_ptr() as *const c_char, test_progs_verification_cert_len) as c_int;
    if err < 0 { return err; }
    traffic_monitor_set_print(traffic_monitor_print_fn);
    srand(time(ptr::null_mut()) as c_uint);
    env.jit_enabled = is_jit_enabled();
    env.nr_cpus = libbpf_num_possible_cpus();
    if env.nr_cpus < 0 {
        fprintf(stderr, b"Failed to get number of CPUs: %d!\n\0".as_ptr() as *const c_char, env.nr_cpus);
        return -1;
    }
    env.has_testmod = true;
    if !env.list_test_names {
        unload_bpf_testmod(verbose());
        if load_bpf_testmod(verbose()) != 0 {
            fprintf(env.stderr_saved, b"WARNING! Selftests relying on bpf_testmod.ko will be skipped.\n\0".as_ptr() as *const c_char);
            env.has_testmod = false;
        }
    }
    for i in 0..prog_test_cnt {
        let test = prog_test_defs.add(i as usize);
        (*test).test_num = i + 1;
        (*test).selected = should_run(&raw mut env.test_selector, (*test).test_num, (*test).test_name);
        (*test).should_run = (*test).selected;
        if (*test).run_test.is_some() && (*test).run_serial_test.is_some() {
            fprintf(stderr, b"Test %d:%s must have either test_%s() or serial_test_%sl() defined.\n\0".as_ptr() as *const c_char, (*test).test_num, (*test).test_name, (*test).test_name, (*test).test_name);
            exit(EXIT_ERR_SETUP_INFRA);
        }
        if (*test).run_test.is_none() && (*test).run_serial_test.is_none() {
            (*test).not_built = true;
            (*test).should_run = false;
            continue;
        }
        if (*test).should_run { (*test).should_tmon = should_tmon(&raw mut env.tmon_selector, (*test).test_name); }
    }
    if env.get_test_cnt || env.list_test_names { env.workers = 0; }
    env.worker_id = -1;
    if env.workers != 0 {
        env.worker_pids = calloc(size_of::<pid_t>(), env.workers as usize) as *mut pid_t;
        env.worker_socks = calloc(size_of::<c_int>(), env.workers as usize) as *mut c_int;
        if env.debug { fprintf(stdout, b"Launching %d workers.\n\0".as_ptr() as *const c_char, env.workers); }
        for i in 0..env.workers {
            let mut sv = [0 as c_int; 2];
            if socketpair(AF_UNIX, SOCK_SEQPACKET | SOCK_CLOEXEC, 0, sv.as_mut_ptr()) < 0 {
                perror(b"Fail to create worker socket\0".as_ptr() as *const c_char);
                return -1;
            }
            let pid = fork();
            if pid < 0 {
                perror(b"Failed to fork worker\0".as_ptr() as *const c_char);
                return -1;
            } else if pid != 0 {
                close(sv[1]);
                *env.worker_pids.add(i as usize) = pid;
                *env.worker_socks.add(i as usize) = sv[0];
            } else {
                close(sv[0]);
                env.worker_id = i;
                return worker_main(sv[1]);
            }
        }
        if env.worker_id == -1 {
            server_main();
            return main_out();
        }
    }
    save_netns();
    for i in 0..prog_test_cnt {
        let test = prog_test_defs.add(i as usize);
        let state = test_states.add(i as usize);
        if !(*test).should_run {
            if (*test).not_built && (*test).selected && !env.get_test_cnt && !env.list_test_names {
                (*state).tested = true;
                (*state).skip_cnt = 1;
                env.not_built_cnt += 1;
                print_test_result(test, state);
            }
            continue;
        }
        if env.get_test_cnt {
            env.succ_cnt += 1;
            continue;
        }
        if env.list_test_names {
            fprintf(env.stdout_saved, b"%s\n\0".as_ptr() as *const c_char, (*test).test_name);
            env.succ_cnt += 1;
            continue;
        }
        run_one_test(i);
    }
    if env.get_test_cnt {
        printf(b"%d\n\0".as_ptr() as *const c_char, env.succ_cnt);
        return main_out();
    }
    if env.list_test_names { return main_out(); }
    calculate_summary_and_print_errors(&raw mut env);
    close(env.saved_netns_fd);
    main_out()
}

unsafe fn main_out() -> c_int {
    if !env.list_test_names && env.has_testmod { unload_bpf_testmod(verbose()); }
    free_test_selector(&raw mut env.test_selector);
    free_test_selector(&raw mut env.subtest_selector);
    free_test_selector(&raw mut env.tmon_selector);
    free_test_states();
    if env.succ_cnt + env.fail_cnt + env.skip_cnt == 0 { return EXIT_NO_TEST; }
    if env.fail_cnt != 0 { EXIT_FAILURE } else { EXIT_SUCCESS }
}
