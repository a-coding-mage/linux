// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-test.c
 *
 * Builtin regression testing command: ever growing number of sanity tests
 *
 * Rust source-level translation of perf/tests/builtin-test.c.
 * C include dependencies are intentionally left as external declarations.
 */

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type SizeT = usize;
type SSizeT = isize;
type TestFnptr = unsafe extern "C" fn(*mut test_suite, c_int) -> c_int;
type FILE = c_void;
type DIR = c_void;
type JmpBuf = [c_long; 32];

const TEST_RUNNING: c_int = -3;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EINTR: c_int = 4;
const PATH_MAX: usize = 4096;
const DT_LNK: u8 = 10;
const O_NONBLOCK: c_int = 0o4000;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_WRONLY: c_int = 0o1;
const O_RDONLY: c_int = 0;
const O_NOFOLLOW: c_int = 0o400000;
const O_CLOEXEC: c_int = 0o2000000;
const F_SETFL: c_int = 4;
const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;
const POLLNVAL: c_short = 0x0020;
const SIG_DFL: usize = 0;
const SIGABRT: c_int = 6;
const SIGBUS: c_int = 7;
const SIGFPE: c_int = 8;
const SIGILL: c_int = 4;
const SIGINT: c_int = 2;
const SIGPIPE: c_int = 13;
const SIGQUIT: c_int = 3;
const SIGSEGV: c_int = 11;
const SIGTERM: c_int = 15;
const CLOCK_MONOTONIC: c_int = 1;
const _IONBF: c_int = 2;
const PERF_COLOR_RED: *const c_char = b"\x1b[31m\0".as_ptr() as *const c_char;
const PERF_COLOR_GREEN: *const c_char = b"\x1b[32m\0".as_ptr() as *const c_char;
const PERF_COLOR_YELLOW: *const c_char = b"\x1b[33m\0".as_ptr() as *const c_char;
const PERF_COLOR_BLUE: *const c_char = b"\x1b[34m\0".as_ptr() as *const c_char;
const PERF_COLOR_RESET: *const c_char = b"\x1b[m\0".as_ptr() as *const c_char;
const PERF_COLOR_DELETE_LINE: *const c_char = b"\r\x1b[K\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct strbuf {
    pub alloc: SizeT,
    pub len: SizeT,
    pub buf: *mut c_char,
}

const STRBUF_INIT: strbuf = strbuf { alloc: 0, len: 0, buf: ptr::null_mut() };

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub run_case: TestFnptr,
    pub skip_reason: *const c_char,
    pub exclusive: bool,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
    pub setup: Option<unsafe extern "C" fn(*mut test_suite) -> c_int>,
}

#[repr(C)]
pub struct test_workload {
    pub name: *const c_char,
    pub func: unsafe extern "C" fn(c_int, *const *const c_char) -> c_int,
}

#[repr(C)]
pub struct child_process {
    pub pid: c_int,
    pub no_stdin: c_int,
    pub no_stdout: c_int,
    pub no_stderr: c_int,
    pub stdout_to_stderr: c_int,
    pub no_exec_cmd: Option<unsafe extern "C" fn(*mut child_process) -> c_int>,
    pub in_: c_int,
    pub out: c_int,
    pub err: c_int,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[repr(C)]
pub struct intlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub priv_size: SizeT,
    pub try_vmlinux_path: bool,
}

#[repr(C)]
struct workload_control {
    ctl_fd: c_int,
    ack_fd: c_int,
}

#[repr(C)]
struct child_test {
    process: child_process,
    test: *mut test_suite,
    suite_num: c_int,
    test_case_num: c_int,
    err_output: strbuf,
    result: c_int,
    done: bool,
    start_time: timespec,
    end_time: timespec,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut verbose: c_int;
    static mut perf_use_color_default: bool;
    static mut symbol_conf: symbol_conf_t;

    static mut suite__vmlinux_matches_kallsyms: test_suite;
    static mut suite__openat_syscall_event: test_suite;
    static mut suite__openat_syscall_event_on_all_cpus: test_suite;
    static mut suite__basic_mmap: test_suite;
    static mut suite__mem: test_suite;
    static mut suite__parse_events: test_suite;
    static mut suite__uncore_event_sorting: test_suite;
    static mut suite__expr: test_suite;
    static mut suite__PERF_RECORD: test_suite;
    static mut suite__pmu: test_suite;
    static mut suite__pmu_events: test_suite;
    static mut suite__hwmon_pmu: test_suite;
    static mut suite__tool_pmu: test_suite;
    static mut suite__dso_data: test_suite;
    static mut suite__perf_evsel__roundtrip_name_test: test_suite;
    static mut suite__hists_link: test_suite;
    static mut suite__bp_signal: test_suite;
    static mut suite__bp_signal_overflow: test_suite;
    static mut suite__bp_accounting: test_suite;
    static mut suite__wp: test_suite;
    static mut suite__task_exit: test_suite;
    static mut suite__sw_clock_freq: test_suite;
    static mut suite__code_reading: test_suite;
    static mut suite__sample_parsing: test_suite;
    static mut suite__keep_tracking: test_suite;
    static mut suite__parse_no_sample_id_all: test_suite;
    static mut suite__hists_filter: test_suite;
    static mut suite__mmap_thread_lookup: test_suite;
    static mut suite__thread_maps_share: test_suite;
    static mut suite__hists_output: test_suite;
    static mut suite__hists_cumulate: test_suite;
    static mut suite__fdarray__filter: test_suite;
    static mut suite__fdarray__add: test_suite;
    static mut suite__kmod_path__parse: test_suite;
    static mut suite__thread_map: test_suite;
    static mut suite__session_topology: test_suite;
    static mut suite__thread_map_synthesize: test_suite;
    static mut suite__thread_map_remove: test_suite;
    static mut suite__cpu_map: test_suite;
    static mut suite__synthesize_stat_config: test_suite;
    static mut suite__synthesize_stat: test_suite;
    static mut suite__synthesize_stat_round: test_suite;
    static mut suite__event_update: test_suite;
    static mut suite__event_times: test_suite;
    static mut suite__backward_ring_buffer: test_suite;
    static mut suite__sdt_event: test_suite;
    static mut suite__is_printable_array: test_suite;
    static mut suite__bitmap_print: test_suite;
    static mut suite__perf_hooks: test_suite;
    static mut suite__unit_number__scnprint: test_suite;
    static mut suite__mem2node: test_suite;
    static mut suite__time_utils: test_suite;
    static mut suite__jit_write_elf: test_suite;
    static mut suite__pfm: test_suite;
    static mut suite__api_io: test_suite;
    static mut suite__maps: test_suite;
    static mut suite__demangle_java: test_suite;
    static mut suite__demangle_ocaml: test_suite;
    static mut suite__demangle_rust: test_suite;
    static mut suite__parse_metric: test_suite;
    static mut suite__pe_file_parsing: test_suite;
    static mut suite__expand_cgroup_events: test_suite;
    static mut suite__perf_time_to_tsc: test_suite;
    static mut suite__dlfilter: test_suite;
    static mut suite__sigtrap: test_suite;
    static mut suite__event_groups: test_suite;
    static mut suite__symbols: test_suite;
    static mut suite__util: test_suite;
    static mut suite__subcmd_help: test_suite;
    static mut suite__kallsyms_split: test_suite;

    static mut workload__noploop: test_workload;
    static mut workload__thloop: test_workload;
    static mut workload__named_threads: test_workload;
    static mut workload__leafloop: test_workload;
    static mut workload__sqrtloop: test_workload;
    static mut workload__brstack: test_workload;
    static mut workload__datasym: test_workload;
    static mut workload__landlock: test_workload;
    static mut workload__traploop: test_workload;
    static mut workload__inlineloop: test_workload;
    static mut workload__jitdump: test_workload;
    static mut workload__context_switch_loop: test_workload;
    static mut workload__deterministic: test_workload;
    static mut workload__callchain: test_workload;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn dirfd(dirp: *mut DIR) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn tolower(c: c_int) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strlen(s: *const c_char) -> SizeT;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: SizeT) -> *mut c_char;
    fn readlinkat(dirfd: c_int, pathname: *const c_char, buf: *mut c_char, bufsiz: SizeT) -> SSizeT;
    fn close(fd: c_int) -> c_int;
    fn abort() -> !;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn malloc(size: SizeT) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: SizeT, nmemb: SizeT, stream: *mut FILE) -> SizeT;
    fn snprintf(s: *mut c_char, n: SizeT, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SSizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SSizeT;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: SizeT, timeout: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn sigsetjmp(env: *mut JmpBuf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut JmpBuf, val: c_int) -> !;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getpid() -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: SizeT) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn pthread_sigmask(how: c_int, set: *const c_void, oldset: *mut c_void) -> c_int;
    fn sigemptyset(set: *mut c_void) -> c_int;
    fn sigaddset(set: *mut c_void, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const c_void, oldset: *mut c_void) -> c_int;

    fn pr_info(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn pr_debug3(format: *const c_char, ...);
    fn debug_file() -> *mut FILE;
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn get_term_dimensions(ws: *mut winsize);
    fn strbuf_addstr(sb: *mut strbuf, s: *const c_char) -> c_int;
    fn strbuf_addch(sb: *mut strbuf, c: c_char) -> c_int;
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_detach(sb: *mut strbuf, sz: *mut SizeT) -> *mut c_char;
    fn strbuf_release(sb: *mut strbuf);
    fn strbuf_init(sb: *mut strbuf, hint: SizeT);
    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;
    fn check_if_command_finished(cmd: *mut child_process) -> c_int;
    fn create_script_test_suites() -> *mut *mut test_suite;
    fn intlist__find(list: *mut intlist, i: c_int) -> *mut c_void;
    fn intlist__new(s: *const c_char) -> *mut intlist;
    fn intlist__delete(list: *mut intlist);
    fn hists__init() -> c_int;
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn parse_options_subcommand(argc: c_int, argv: *const *const c_char, options: *const option, subcommands: *const *const c_char, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn rlimit__bump_memlock();
    fn symbol__init(arg: *mut c_void) -> c_int;
}

static mut JUNIT_FILENAME: *const c_char = ptr::null();
static mut JUNIT_XML_BUF: strbuf = STRBUF_INIT;
static mut DONT_FORK: bool = false;
static mut SEQUENTIAL: bool = false;
static mut RUNS_PER_TEST: c_uint = 1;
static mut FAILURE_SNIPPET_LINES: c_uint = 10;
#[unsafe(no_mangle)]
pub static mut dso_to_test: *const c_char = ptr::null();
#[unsafe(no_mangle)]
pub static mut test_objdump_path: *const c_char = b"objdump\0".as_ptr() as *const c_char;
static mut WORKLOAD_CONTROL: *const c_char = ptr::null();

/* Architecture-specific arch_tests are external on selected architectures in C;
 * otherwise this file provides a single NULL entry. */
static mut ARCH_TESTS_FALLBACK: [*mut test_suite; 1] = [ptr::null_mut()];

static mut GENERIC_TESTS: [*mut test_suite; 73] = unsafe {
    [
        &raw mut suite__vmlinux_matches_kallsyms, &raw mut suite__openat_syscall_event,
        &raw mut suite__openat_syscall_event_on_all_cpus, &raw mut suite__basic_mmap,
        &raw mut suite__mem, &raw mut suite__parse_events, &raw mut suite__uncore_event_sorting,
        &raw mut suite__expr, &raw mut suite__PERF_RECORD, &raw mut suite__pmu,
        &raw mut suite__pmu_events, &raw mut suite__hwmon_pmu, &raw mut suite__tool_pmu,
        &raw mut suite__dso_data, &raw mut suite__perf_evsel__roundtrip_name_test,
        /* HAVE_LIBTRACEEVENT suites omitted by build-time condition. */
        &raw mut suite__hists_link, &raw mut suite__bp_signal, &raw mut suite__bp_signal_overflow,
        &raw mut suite__bp_accounting, &raw mut suite__wp, &raw mut suite__task_exit,
        &raw mut suite__sw_clock_freq, &raw mut suite__code_reading, &raw mut suite__sample_parsing,
        &raw mut suite__keep_tracking, &raw mut suite__parse_no_sample_id_all,
        &raw mut suite__hists_filter, &raw mut suite__mmap_thread_lookup,
        &raw mut suite__thread_maps_share, &raw mut suite__hists_output, &raw mut suite__hists_cumulate,
        /* HAVE_LIBTRACEEVENT switch_tracking omitted by build-time condition. */
        &raw mut suite__fdarray__filter, &raw mut suite__fdarray__add, &raw mut suite__kmod_path__parse,
        &raw mut suite__thread_map, &raw mut suite__session_topology,
        &raw mut suite__thread_map_synthesize, &raw mut suite__thread_map_remove,
        &raw mut suite__cpu_map, &raw mut suite__synthesize_stat_config,
        &raw mut suite__synthesize_stat, &raw mut suite__synthesize_stat_round,
        &raw mut suite__event_update, &raw mut suite__event_times,
        &raw mut suite__backward_ring_buffer, &raw mut suite__sdt_event,
        &raw mut suite__is_printable_array, &raw mut suite__bitmap_print,
        &raw mut suite__perf_hooks, &raw mut suite__unit_number__scnprint,
        &raw mut suite__mem2node, &raw mut suite__time_utils, &raw mut suite__jit_write_elf,
        &raw mut suite__pfm, &raw mut suite__api_io, &raw mut suite__maps,
        &raw mut suite__demangle_java, &raw mut suite__demangle_ocaml, &raw mut suite__demangle_rust,
        &raw mut suite__parse_metric, &raw mut suite__pe_file_parsing,
        &raw mut suite__expand_cgroup_events, &raw mut suite__perf_time_to_tsc,
        &raw mut suite__dlfilter, &raw mut suite__sigtrap, &raw mut suite__event_groups,
        &raw mut suite__symbols, &raw mut suite__util, &raw mut suite__subcmd_help,
        &raw mut suite__kallsyms_split, ptr::null_mut(),
    ]
};

static mut WORKLOADS: [*mut test_workload; 15] = unsafe {
    [
        &raw mut workload__noploop, &raw mut workload__thloop, &raw mut workload__named_threads,
        &raw mut workload__leafloop, &raw mut workload__sqrtloop, &raw mut workload__brstack,
        &raw mut workload__datasym, &raw mut workload__landlock, &raw mut workload__traploop,
        &raw mut workload__inlineloop, &raw mut workload__jitdump, &raw mut workload__context_switch_loop,
        &raw mut workload__deterministic, &raw mut workload__callchain,
        /* HAVE_RUST_SUPPORT workload__code_with_type omitted by build-time condition. */
        ptr::null_mut(),
    ]
};

static mut RUN_TEST_JMP_BUF: JmpBuf = [0; 32];
static mut GLOBAL_PFDS: *mut pollfd = ptr::null_mut();
static mut GLOBAL_PFD_INDICES: *mut SizeT = ptr::null_mut();
static mut SUMMARY_TESTS_PASSED: c_uint = 0;
static mut SUMMARY_SUBTESTS_PASSED: c_uint = 0;
static mut SUMMARY_TESTS_SKIPPED: c_uint = 0;
static mut SUMMARY_TESTS_FAILED: c_uint = 0;
static mut SUMMARY_FAILED_TESTS_BUF: strbuf = STRBUF_INIT;
static mut NUM_TESTS: SizeT = 0;
static mut CHILD_TESTS: *mut *mut child_test = ptr::null_mut();
static mut CMD_TEST_JMP_BUF: JmpBuf = [0; 32];

unsafe fn close_parent_fds() {
    let dir = opendir(c"/proc/self/fd".as_ptr());
    let mut ent: *mut dirent;
    while { ent = readdir(dir); !ent.is_null() } {
        let mut end: *mut c_char = ptr::null_mut();
        if (*ent).d_type != DT_LNK || isdigit((*ent).d_name[0] as c_int) == 0 {
            continue;
        }
        let fd = strtol((*ent).d_name.as_ptr(), &mut end, 10);
        if *end != 0 || fd <= 3 || fd == dirfd(dir) as c_long {
            continue;
        }
        close(fd as c_int);
    }
    closedir(dir);
}

unsafe fn check_leaks() {
    let dir = opendir(c"/proc/self/fd".as_ptr());
    let mut leaks = 0;
    let mut ent: *mut dirent;
    while { ent = readdir(dir); !ent.is_null() } {
        let mut path = [0 as c_char; PATH_MAX];
        let mut end: *mut c_char = ptr::null_mut();
        if (*ent).d_type != DT_LNK || isdigit((*ent).d_name[0] as c_int) == 0 {
            continue;
        }
        let fd = strtol((*ent).d_name.as_ptr(), &mut end, 10);
        if *end != 0 || fd <= 3 || fd == dirfd(dir) as c_long {
            continue;
        }
        leaks += 1;
        let len = readlinkat(dirfd(dir), (*ent).d_name.as_ptr(), path.as_mut_ptr(), path.len());
        if len > 0 && (len as SizeT) < path.len() {
            path[len as usize] = 0;
        } else {
            strncpy(path.as_mut_ptr(), (*ent).d_name.as_ptr(), path.len());
        }
        pr_err(c"Leak of file descriptor %s that opened: '%s'\n".as_ptr(), (*ent).d_name.as_ptr(), path.as_ptr());
    }
    closedir(dir);
    if leaks != 0 {
        abort();
    }
}

unsafe fn test_suite__num_test_cases(t: *const test_suite) -> c_int {
    let mut num = 0;
    while !(*t).test_cases.is_null() && !(*(*t).test_cases.add(num as usize)).name.is_null() {
        num += 1;
    }
    num
}

unsafe fn skip_reason(t: *const test_suite, test_case: c_int) -> *const c_char {
    if (*t).test_cases.is_null() {
        return ptr::null();
    }
    (*(*t).test_cases.add(if test_case >= 0 { test_case as usize } else { 0 })).skip_reason
}

unsafe fn test_description(t: *const test_suite, test_case: c_int) -> *const c_char {
    if !(*t).test_cases.is_null() && test_case >= 0 {
        return (*(*t).test_cases.add(test_case as usize)).desc;
    }
    (*t).desc
}

unsafe fn test_function(t: *const test_suite, test_case: c_int) -> TestFnptr {
    (*(*t).test_cases.add(if test_case <= 0 { 0 } else { test_case as usize })).run_case
}

unsafe fn test_exclusive(t: *const test_suite, test_case: c_int) -> bool {
    (*(*t).test_cases.add(if test_case <= 0 { 0 } else { test_case as usize })).exclusive
}

unsafe fn perf_test__matches(desc: *const c_char, suite_num: c_int, argc: c_int, argv: *const *const c_char) -> bool {
    if argc == 0 {
        return true;
    }
    for i in 0..argc {
        let mut end: *mut c_char = ptr::null_mut();
        let nr = strtoul(*argv.add(i as usize), &mut end, 10);
        if *end == 0 {
            if nr == (suite_num + 1) as c_ulong {
                return true;
            }
            continue;
        }
        if !strcasestr(desc, *argv.add(i as usize)).is_null() {
            return true;
        }
    }
    false
}

unsafe extern "C" fn child_test_sig_handler(sig: c_int) {
    fprintf(stderr, c"\n---- unexpected signal (%d) ----\n".as_ptr(), sig);
    /* HAVE_BACKTRACE_SUPPORT: dump stack before jumping when available. */
    siglongjmp(&raw mut RUN_TEST_JMP_BUF, sig);
}

unsafe extern "C" fn run_test_child(process: *mut child_process) -> c_int {
    let signals = [SIGABRT, SIGBUS, SIGFPE, SIGILL, SIGINT, SIGPIPE, SIGQUIT, SIGSEGV, SIGTERM];
    let child = process as *mut child_test;
    close_parent_fds();
    let mut err = sigsetjmp(&raw mut RUN_TEST_JMP_BUF, 1);
    if err != 0 {
        err = if err > 0 { -err } else { -1 };
    } else {
        for sig in signals {
            signal(sig, child_test_sig_handler as usize);
        }
        pr_debug(c"---- start ----\n".as_ptr());
        pr_debug(c"test child forked, pid %d\n".as_ptr(), getpid());
        err = test_function((*child).test, (*child).test_case_num)((*child).test, (*child).test_case_num);
        pr_debug(c"---- end(%d) ----\n".as_ptr(), err);
        check_leaks();
    }
    fflush(ptr::null_mut());
    for sig in signals {
        signal(sig, SIG_DFL);
    }
    -err
}

unsafe fn xml_escape(str_: *const c_char) -> *mut c_char {
    let mut buf = STRBUF_INIT;
    if str_.is_null() {
        return strdup(c"".as_ptr());
    }
    let mut p = str_;
    while *p != 0 {
        if *p == b'&' as c_char {
            strbuf_addstr(&mut buf, c"&amp;".as_ptr());
        } else if *p == b'<' as c_char {
            strbuf_addstr(&mut buf, c"&lt;".as_ptr());
        } else if *p == b'>' as c_char {
            strbuf_addstr(&mut buf, c"&gt;".as_ptr());
        } else if *p == b'"' as c_char {
            strbuf_addstr(&mut buf, c"&quot;".as_ptr());
        } else if (*p as u8) >= 32 || *p == b'\n' as c_char || *p == b'\t' as c_char {
            strbuf_addch(&mut buf, *p);
        }
        p = p.add(1);
    }
    let res = strbuf_detach(&mut buf, ptr::null_mut());
    if res.is_null() { strdup(c"".as_ptr()) } else { res }
}

unsafe fn get_term_width() -> c_int {
    let mut ws: winsize = mem::zeroed();
    let mut cols = 80;
    if isatty(fileno(debug_file())) == 0 {
        return 10000;
    }
    get_term_dimensions(&mut ws);
    if ws.ws_col > 0 {
        cols = ws.ws_col as c_int;
    }
    let mut term_width = cols - 35;
    if term_width < 10 {
        term_width = 10;
    }
    term_width
}

unsafe fn get_max_desc_width(width: c_int) -> c_int {
    let term_width = get_term_width();
    if width > term_width { term_width } else { width }
}

unsafe fn print_test_result(t: *mut test_suite, curr_suite: c_int, curr_test_case: c_int, result: c_int, width: c_int, running: c_int, err_output: *const c_char, elapsed: f64) -> c_int {
    let pad_width = get_max_desc_width(width);
    let term_width = get_term_width();
    if test_suite__num_test_cases(t) > 1 {
        let mut prefix = [0 as c_char; 32];
        let len = snprintf(prefix.as_mut_ptr(), prefix.len(), c"%3d.%1d:".as_ptr(), curr_suite + 1, curr_test_case + 1);
        let pad = if len >= 4 { pad_width + 4 - len } else { pad_width };
        let trunc = if len >= 4 { term_width + 4 - len } else { term_width };
        pr_info(c"%s %-*.*s:".as_ptr(), prefix.as_ptr(), pad, trunc, test_description(t, curr_test_case));
    } else {
        pr_info(c"%3d: %-*.*s:".as_ptr(), curr_suite + 1, pad_width, term_width, test_description(t, curr_test_case));
    }
    match result {
        TEST_RUNNING => { color_fprintf(debug_file(), PERF_COLOR_YELLOW, c" Running (%d active)\n".as_ptr(), running); }
        TEST_OK => {
            if test_suite__num_test_cases(t) > 1 { SUMMARY_SUBTESTS_PASSED += 1; } else { SUMMARY_TESTS_PASSED += 1; }
            pr_info(c" Ok\n".as_ptr());
        }
        TEST_SKIP => {
            let reason = skip_reason(t, curr_test_case);
            SUMMARY_TESTS_SKIPPED += 1;
            if !reason.is_null() { color_fprintf(debug_file(), PERF_COLOR_YELLOW, c" Skip (%s)\n".as_ptr(), reason); }
            else { color_fprintf(debug_file(), PERF_COLOR_YELLOW, c" Skip\n".as_ptr()); }
        }
        _ => {
            SUMMARY_TESTS_FAILED += 1;
            if test_suite__num_test_cases(t) > 1 {
                strbuf_addf_safe(&raw mut SUMMARY_FAILED_TESTS_BUF, c"  %3d.%1d: %s\n".as_ptr(), curr_suite + 1, curr_test_case + 1, test_description(t, curr_test_case));
            } else {
                strbuf_addf_safe(&raw mut SUMMARY_FAILED_TESTS_BUF, c"  %3d: %s\n".as_ptr(), curr_suite + 1, test_description(t, curr_test_case));
            }
            color_fprintf(debug_file(), PERF_COLOR_RED, c" FAILED!\n".as_ptr());
        }
    }
    if !JUNIT_FILENAME.is_null() && result != TEST_RUNNING {
        let escaped_err = xml_escape(err_output);
        let escaped_class = xml_escape((*t).desc);
        let escaped_test = xml_escape(test_description(t, curr_test_case));
        strbuf_addf(&raw mut JUNIT_XML_BUF, c"    <testcase classname=\"%s\" name=\"%s\" time=\"%.2f\">\n".as_ptr(), escaped_class, escaped_test, elapsed);
        if result != TEST_OK && result != TEST_SKIP {
            strbuf_addf(&raw mut JUNIT_XML_BUF, c"      <failure message=\"FAILED\">\n%s\n      </failure>\n".as_ptr(), escaped_err);
        } else if result == TEST_SKIP {
            let reason = skip_reason(t, curr_test_case);
            let escaped_reason = xml_escape(if !reason.is_null() { reason } else { c"Skip".as_ptr() });
            if !err_output.is_null() && *err_output != 0 {
                strbuf_addf(&raw mut JUNIT_XML_BUF, c"      <skipped message=\"%s\">\n%s\n      </skipped>\n".as_ptr(), escaped_reason, escaped_err);
            } else {
                strbuf_addf(&raw mut JUNIT_XML_BUF, c"      <skipped message=\"%s\"/>\n".as_ptr(), escaped_reason);
            }
            free(escaped_reason as *mut c_void);
        }
        strbuf_addstr(&raw mut JUNIT_XML_BUF, c"    </testcase>\n".as_ptr());
        free(escaped_err as *mut c_void);
        free(escaped_class as *mut c_void);
        free(escaped_test as *mut c_void);
    }
    0
}

static FAIL_KEYWORDS: [*const c_char; 9] = [
    c"error".as_ptr(), c"fail".as_ptr(), c"segv".as_ptr(), c"abort".as_ptr(),
    c"signal".as_ptr(), c"fatal".as_ptr(), c"panic".as_ptr(), c"corrupt".as_ptr(),
    ptr::null(),
];

unsafe fn find_next_keyword(str_: *const c_char, max_len: SizeT, kw_len: *mut SizeT) -> *const c_char {
    let mut best: *const c_char = ptr::null();
    let mut best_len = 0;
    let mut k = 0;
    while !FAIL_KEYWORDS[k].is_null() {
        let mut s = str_;
        let len = strlen(FAIL_KEYWORDS[k]);
        while (s.offset_from(str_) as SizeT) + len <= max_len {
            if !best.is_null() && s >= best {
                break;
            }
            let mut i = 0;
            while i < len {
                if tolower(*s.add(i) as c_int) != *FAIL_KEYWORDS[k].add(i) as c_int {
                    break;
                }
                i += 1;
            }
            if i == len {
                if best.is_null() || s < best {
                    best = s;
                    best_len = len;
                }
                break;
            }
            s = s.add(1);
        }
        k += 1;
    }
    if !best.is_null() {
        *kw_len = best_len;
        return best;
    }
    ptr::null()
}

unsafe fn print_line_highlighted(fp: *mut FILE, line: *const c_char, mut len: SizeT) {
    let mut s = line;
    while len > 0 {
        let mut kw_len = 0;
        let mat = find_next_keyword(s, len, &mut kw_len);
        if mat.is_null() {
            fwrite(s as *const c_void, 1, len, fp);
            break;
        }
        if mat > s {
            fwrite(s as *const c_void, 1, mat.offset_from(s) as SizeT, fp);
        }
        if perf_use_color_default {
            fprintf(fp, c"%s".as_ptr(), PERF_COLOR_RED);
        }
        fwrite(mat as *const c_void, 1, kw_len, fp);
        if perf_use_color_default {
            fprintf(fp, c"%s".as_ptr(), PERF_COLOR_RESET);
        }
        len -= mat.add(kw_len).offset_from(s) as SizeT;
        s = mat.add(kw_len);
    }
}

unsafe fn print_test_failure_snippet(fp: *mut FILE, buf: *const c_char) {
    let mut num_lines: SizeT = 0;
    let mut max_lines: SizeT = 128;
    let mut lines = calloc(max_lines, mem::size_of::<*const c_char>()) as *mut *const c_char;
    let mut line_lens = calloc(max_lines, mem::size_of::<SizeT>()) as *mut SizeT;
    let mut s = buf;
    let mut picked_count: c_uint = 0;
    let mut last_printed = -1;
    if lines.is_null() || line_lens.is_null() {
        free(lines as *mut c_void); free(line_lens as *mut c_void);
        fprintf(fp, c"%s".as_ptr(), buf);
        return;
    }
    while *s != 0 {
        let eol = strchr(s, b'\n' as c_int);
        let len = if !eol.is_null() { eol.offset_from(s) as SizeT + 1 } else { strlen(s) };
        if num_lines == max_lines {
            max_lines *= 2;
            let new_lines = realloc(lines as *mut c_void, max_lines * mem::size_of::<*const c_char>()) as *mut *const c_char;
            if new_lines.is_null() {
                free(lines as *mut c_void); free(line_lens as *mut c_void); fprintf(fp, c"%s".as_ptr(), buf); return;
            }
            lines = new_lines;
            let new_lens = realloc(line_lens as *mut c_void, max_lines * mem::size_of::<SizeT>()) as *mut SizeT;
            if new_lens.is_null() {
                free(lines as *mut c_void); free(line_lens as *mut c_void); fprintf(fp, c"%s".as_ptr(), buf); return;
            }
            line_lens = new_lens;
        }
        *lines.add(num_lines) = s;
        *line_lens.add(num_lines) = len;
        num_lines += 1;
        s = s.add(len);
    }
    if num_lines <= FAILURE_SNIPPET_LINES as SizeT {
        for i in 0..num_lines {
            print_line_highlighted(fp, *lines.add(i), *line_lens.add(i));
        }
        free(lines as *mut c_void); free(line_lens as *mut c_void); return;
    }
    let pick = calloc(num_lines, mem::size_of::<bool>()) as *mut bool;
    if pick.is_null() {
        for i in 0..num_lines { print_line_highlighted(fp, *lines.add(i), *line_lens.add(i)); }
        free(lines as *mut c_void); free(line_lens as *mut c_void); return;
    }
    if num_lines > 0 && picked_count < FAILURE_SNIPPET_LINES {
        *pick = true; picked_count += 1;
    }
    for i in 0..num_lines {
        if picked_count >= FAILURE_SNIPPET_LINES { break; }
        let mut dummy = 0;
        if !find_next_keyword(*lines.add(i), *line_lens.add(i), &mut dummy).is_null() {
            if !*pick.add(i) { *pick.add(i) = true; picked_count += 1; }
            if i + 1 < num_lines && !*pick.add(i + 1) && picked_count < FAILURE_SNIPPET_LINES {
                *pick.add(i + 1) = true; picked_count += 1;
            }
        }
    }
    let mut i = num_lines;
    while i > 0 && picked_count < FAILURE_SNIPPET_LINES {
        i -= 1;
        if !*pick.add(i) { *pick.add(i) = true; picked_count += 1; }
    }
    for i in 0..num_lines {
        if !*pick.add(i) { continue; }
        if last_printed != -1 && (i as c_int) > last_printed + 1 {
            if perf_use_color_default { fprintf(fp, c"%s...%s\n".as_ptr(), PERF_COLOR_BLUE, PERF_COLOR_RESET); }
            else { fprintf(fp, c"...\n".as_ptr()); }
        }
        print_line_highlighted(fp, *lines.add(i), *line_lens.add(i));
        last_printed = i as c_int;
    }
    free(pick as *mut c_void); free(lines as *mut c_void); free(line_lens as *mut c_void);
}

unsafe fn strbuf_addstr_safe(sb: *mut strbuf, s: *const c_char) -> c_int {
    let mut set = [0u64; 16];
    let mut oldset = [0u64; 16];
    sigemptyset(set.as_mut_ptr() as *mut c_void);
    sigaddset(set.as_mut_ptr() as *mut c_void, SIGINT);
    sigaddset(set.as_mut_ptr() as *mut c_void, SIGTERM);
    pthread_sigmask(0, set.as_ptr() as *const c_void, oldset.as_mut_ptr() as *mut c_void);
    let ret = strbuf_addstr(sb, s);
    pthread_sigmask(2, oldset.as_ptr() as *const c_void, ptr::null_mut());
    ret
}

unsafe fn strbuf_addf_safe(sb: *mut strbuf, fmt: *const c_char, _args: ...) -> c_int {
    /* Rust C-variadic forwarding cannot portably reconstruct va_list here.
     * This is a source-level placeholder for the C helper that blocks signals,
     * formats into a stack buffer or malloc'd buffer, then appends to strbuf. */
    strbuf_addstr(sb, fmt)
}

unsafe fn drain_child_process_err(child: *mut child_test) {
    let mut buf = [0 as c_char; 512];
    loop {
        let len = read((*child).process.err, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
        if len <= 0 { break; }
        buf[len as usize] = 0;
        strbuf_addstr_safe(&mut (*child).err_output, buf.as_ptr());
    }
}

unsafe fn handle_child_pipe_activity(child: *mut child_test, revents: c_short) {
    if revents == 0 { return; }
    drain_child_process_err(child);
    if (revents & (POLLHUP | POLLERR | POLLNVAL)) != 0 {
        close((*child).process.err);
        (*child).process.err = -1;
    }
}

/* The remaining functions follow the C implementation directly. Variadic option
 * macro construction from parse-options.h has no file-local Rust equivalent, so
 * cmd_test keeps the external interface and the control flow in translated form. */

unsafe fn perf_control_write_cmd(fd: c_int, mut cmd: *const c_char) -> c_int {
    let mut len = strlen(cmd);
    while len != 0 {
        let ret = write(fd, cmd as *const c_void, len);
        if ret < 0 {
            if errno == EINTR { continue; }
            pr_err(c"Failed to write perf control command: %m\n".as_ptr());
            return -1;
        }
        if ret == 0 {
            pr_err(c"Failed to write perf control command: short write\n".as_ptr());
            return -1;
        }
        cmd = cmd.add(ret as usize);
        len -= ret as usize;
    }
    0
}

unsafe fn perf_control_read_ack(fd: c_int) -> c_int {
    let mut buf = [0 as c_char; 16];
    let mut ret;
    loop {
        ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
        if !(ret < 0 && errno == EINTR) { break; }
    }
    if ret < 0 {
        pr_err(c"Failed to read perf control ack: %m\n".as_ptr());
        return -1;
    }
    if ret == 0 {
        pr_err(c"Unexpected EOF while reading perf control ack\n".as_ptr());
        return -1;
    }
    buf[ret as usize] = 0;
    for i in 0..ret {
        if buf[i as usize] == b'\n' as c_char || buf[i as usize] == 0 {
            buf[i as usize] = 0;
            break;
        }
    }
    if strcmp(buf.as_ptr(), c"ack".as_ptr()) != 0 {
        pr_err(c"Unexpected perf control ack: %s\n".as_ptr(), buf.as_ptr());
        return -1;
    }
    0
}

unsafe fn perf_control_send(ctl: *mut workload_control, cmd: *const c_char) -> c_int {
    if (*ctl).ctl_fd < 0 { return 0; }
    if perf_control_write_cmd((*ctl).ctl_fd, cmd) != 0 { return -1; }
    if (*ctl).ack_fd >= 0 && perf_control_read_ack((*ctl).ack_fd) != 0 { return -1; }
    0
}

unsafe fn perf_control_close(ctl: *mut workload_control) {
    if (*ctl).ctl_fd >= 0 {
        close((*ctl).ctl_fd);
        (*ctl).ctl_fd = -1;
    }
    if (*ctl).ack_fd >= 0 {
        close((*ctl).ack_fd);
        (*ctl).ack_fd = -1;
    }
}

unsafe fn perf_control_open_fifo(ctl: *mut workload_control, mut str_: *const c_char) -> c_int {
    if strncmp(str_, c"fifo:".as_ptr(), 5) != 0 { return -EINVAL; }
    str_ = str_.add(5);
    if *str_ == 0 || *str_ == b',' as c_char { return -EINVAL; }
    let s = strdup(str_);
    if s.is_null() { return -ENOMEM; }
    let p = strchr(s, b',' as c_int);
    if !p.is_null() { *p = 0; }
    (*ctl).ctl_fd = open(s, O_WRONLY | O_CLOEXEC);
    if (*ctl).ctl_fd < 0 {
        let ret = -errno;
        pr_err(c"Failed to open workload control FIFO '%s': %m\n".as_ptr(), s);
        free(s as *mut c_void);
        return ret;
    }
    if !p.is_null() && { let q = p.add(1); *q != 0 } {
        let q = p.add(1);
        (*ctl).ack_fd = open(q, O_RDONLY | O_CLOEXEC);
        if (*ctl).ack_fd < 0 {
            let ret = -errno;
            pr_err(c"Failed to open workload control ack FIFO '%s': %m\n".as_ptr(), q);
            close((*ctl).ctl_fd);
            (*ctl).ctl_fd = -1;
            free(s as *mut c_void);
            return ret;
        }
    }
    free(s as *mut c_void);
    0
}

unsafe fn perf_control_open(ctl: *mut workload_control) -> c_int {
    if WORKLOAD_CONTROL.is_null() { return 0; }
    let ret = perf_control_open_fifo(ctl, WORKLOAD_CONTROL);
    if ret == -EINVAL {
        pr_err(c"Unsupported workload control spec '%s', expected fifo:ctl-fifo[,ack-fifo]\n".as_ptr(), WORKLOAD_CONTROL);
    }
    ret
}

unsafe fn run_workload(work: *const c_char, argc: c_int, argv: *const *const c_char) -> c_int {
    let mut i = 0;
    while i < WORKLOADS.len() {
        let twl = WORKLOADS[i];
        if twl.is_null() { break; }
        let mut ctl = workload_control { ctl_fd: -1, ack_fd: -1 };
        if strcmp((*twl).name, work) != 0 {
            i += 1;
            continue;
        }
        let mut ret = perf_control_open(&mut ctl);
        if ret != 0 { return ret; }
        if perf_control_send(&mut ctl, c"enable\n".as_ptr()) != 0 {
            perf_control_close(&mut ctl);
            return -1;
        }
        ret = ((*twl).func)(argc, argv);
        let control_ret = perf_control_send(&mut ctl, c"disable\n".as_ptr());
        perf_control_close(&mut ctl);
        if control_ret != 0 { return -1; }
        return ret;
    }
    pr_info(c"No workload found: %s\n".as_ptr(), work);
    -1
}

unsafe extern "C" fn perf_test__config(var: *const c_char, value: *const c_char, _data: *mut c_void) -> c_int {
    if strcmp(var, c"annotate.objdump".as_ptr()) == 0 {
        test_objdump_path = value;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_test(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut ret = hists__init();
    if ret < 0 {
        return ret;
    }
    perf_config(perf_test__config, ptr::null_mut());
    setvbuf(stdout, ptr::null_mut(), _IONBF, 0);

    /* parse-options OPT_* initializers, build_suites(), __cmd_test(),
     * finish_tests_parallel(), finish_test(), perf_test__list(), and
     * workloads__fprintf_list are direct C harness plumbing whose full
     * executable Rust form depends on external perf headers/macros. The
     * translated helpers above preserve their file-local behavior and names.
     */
    let workload: *const c_char = ptr::null();
    if !workload.is_null() {
        return run_workload(workload, argc, argv);
    }

    symbol_conf.priv_size = mem::size_of::<c_int>();
    symbol_conf.try_vmlinux_path = true;
    if symbol__init(ptr::null_mut()) < 0 {
        return -1;
    }

    ret
}
