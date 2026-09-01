// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Translated from testing/selftests/bpf/bench.c.  C includes:
 * <argp.h>, <linux/compiler.h>, <sys/time.h>, <sched.h>, <fcntl.h>,
 * <pthread.h>, <sys/sysinfo.h>, <signal.h>, "bench.h", "bpf_util.h",
 * and "testing_helpers.h".
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

type ErrorT = c_int;
type SizeT = usize;
type SighandlerT = Option<unsafe extern "C" fn(c_int)>;
type PthreadT = usize;
type VaList = *mut c_void;
type __u64 = u64;

const LIBBPF_DEBUG: c_int = 0;
const LIBBPF_STRICT_ALL: c_int = 0;
const ARGP_KEY_ARG: c_int = 0;
const ARGP_ERR_UNKNOWN: ErrorT = -1;
const SIGALRM: c_int = 14;
const ITIMER_REAL: c_int = 0;

const ARG_PROD_AFFINITY_SET: c_int = 1000;
const ARG_CONS_AFFINITY_SET: c_int = 1001;

#[repr(C)]
pub struct cpu_set {
    pub cpus: *mut c_int,
    pub cpus_len: c_int,
    pub next_cpu: c_int,
}

#[repr(C)]
pub struct env {
    pub warmup_sec: c_int,
    pub duration_sec: c_int,
    pub affinity: bool,
    pub quiet: bool,
    pub consumer_cnt: c_int,
    pub producer_cnt: c_int,
    pub verbose: bool,
    pub list: bool,
    pub stacktrace: bool,
    pub bench_name: *mut c_char,
    pub prod_cpus: cpu_set,
    pub cons_cpus: cpu_set,
    pub nr_cpus: c_int,
}

#[repr(C)]
pub struct bench_res {
    pub hits: c_long,
    pub drops: c_long,
    pub false_hits: c_long,
    pub important_hits: c_long,
    pub gp_ns: f64,
    pub gp_ct: c_long,
    pub stime: f64,
}

#[repr(C)]
pub struct basic_stats {
    pub mean: f64,
    pub stddev: f64,
}

#[repr(C)]
pub struct argp_option {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: c_int,
    pub header: *const c_char,
    pub group: c_int,
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> ErrorT>,
    pub args_doc: *const c_char,
    pub doc: *const c_char,
    pub children: *const argp_child,
    pub help_filter: *const c_void,
    pub argp_domain: *const c_char,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: SighandlerT,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u64; 16],
}

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [u8; 40],
}

#[repr(C)]
pub struct pthread_cond_t {
    _private: [u8; 48],
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub consumer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(c_int, *mut bench_res, c_long)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

#[repr(C)]
struct bench_state {
    res_cnt: c_int,
    results: *mut bench_res,
    consumers: *mut PthreadT,
    producers: *mut PthreadT,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: VaList) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn sqrt(x: f64) -> f64;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn get_nprocs() -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn setitimer(which: c_int, new_value: *const itimerval, old_value: *mut itimerval) -> c_int;
    fn pthread_create(thread: *mut PthreadT, attr: *const c_void,
                      start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
                      arg: *mut c_void) -> c_int;
    fn pthread_setaffinity_np(thread: PthreadT, cpusetsize: SizeT,
                              cpuset: *const cpu_set_t) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn argp_parse(argp: *const argp, argc: c_int, argv: *mut *mut c_char,
                  flags: c_uint, arg_index: *mut c_int, input: *mut c_void) -> ErrorT;
    fn argp_usage(state: *mut argp_state);
    fn libbpf_set_strict_mode(mode: c_int);
    fn libbpf_set_print(print_fn: Option<unsafe extern "C" fn(c_int, *const c_char, VaList) -> c_int>);
    fn parse_num_list(arg: *mut c_char, nums: *mut *mut c_int, nums_len: *mut c_int) -> c_int;
    fn get_time_ns() -> c_long;
}

#[no_mangle]
pub static mut env: env = env {
    warmup_sec: 1,
    duration_sec: 5,
    affinity: false,
    quiet: false,
    consumer_cnt: 0,
    producer_cnt: 1,
    verbose: false,
    list: false,
    stacktrace: false,
    bench_name: ptr::null_mut(),
    prod_cpus: cpu_set { cpus: ptr::null_mut(), cpus_len: 0, next_cpu: 0 },
    cons_cpus: cpu_set { cpus: ptr::null_mut(), cpus_len: 0, next_cpu: 0 },
    nr_cpus: 0,
};

unsafe extern "C" fn libbpf_print_fn(level: c_int, format: *const c_char, args: VaList) -> c_int {
    if level == LIBBPF_DEBUG && !env.verbose {
        return 0;
    }
    vfprintf(stderr, format, args)
}

#[no_mangle]
pub unsafe extern "C" fn setup_libbpf() {
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    libbpf_set_print(Some(libbpf_print_fn));
}

#[no_mangle]
pub unsafe extern "C" fn false_hits_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    let total = (*res).false_hits + (*res).hits + (*res).drops;

    printf(c"Iter %3d (%7.3lfus): ".as_ptr(), iter, (delta_ns - 1000000000) as f64 / 1000.0);
    printf(c"%ld false hits of %ld total operations. Percentage = %2.2f %%\n".as_ptr(),
           (*res).false_hits, total, ((*res).false_hits as f32 / total as f32) as f64 * 100.0);
}

#[no_mangle]
pub unsafe extern "C" fn false_hits_report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut total_hits: c_long = 0;
    let mut total_drops: c_long = 0;
    let mut total_false_hits: c_long = 0;
    let total_ops: c_long;

    for i in 0..res_cnt {
        total_hits += (*res.offset(i as isize)).hits;
        total_false_hits += (*res.offset(i as isize)).false_hits;
        total_drops += (*res.offset(i as isize)).drops;
    }
    total_ops = total_hits + total_false_hits + total_drops;

    printf(c"Summary: %ld false hits of %ld total operations. ".as_ptr(), total_false_hits, total_ops);
    printf(c"Percentage =  %2.2f %%\n".as_ptr(),
           (total_false_hits as f32 / total_ops as f32) as f64 * 100.0);
}

#[no_mangle]
pub unsafe extern "C" fn hits_drops_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    let hits_per_sec = (*res).hits as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);
    let hits_per_prod = hits_per_sec / env.producer_cnt as f64;
    let drops_per_sec = (*res).drops as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);

    printf(c"Iter %3d (%7.3lfus): ".as_ptr(), iter, (delta_ns - 1000000000) as f64 / 1000.0);
    printf(c"hits %8.3lfM/s (%7.3lfM/prod), drops %8.3lfM/s, total operations %8.3lfM/s\n".as_ptr(),
           hits_per_sec, hits_per_prod, drops_per_sec, hits_per_sec + drops_per_sec);
}

#[no_mangle]
pub unsafe extern "C" fn grace_period_latency_basic_stats(res: *mut bench_res, res_cnt: c_int,
                                                          gp_stat: *mut basic_stats) {
    memset(gp_stat as *mut c_void, 0, mem::size_of::<basic_stats>());

    for i in 0..res_cnt {
        (*gp_stat).mean += (*res.offset(i as isize)).gp_ns / 1000.0
            / (*res.offset(i as isize)).gp_ct as f64 / (0.0 + res_cnt as f64);
    }

    if res_cnt > 1 {
        for i in 0..res_cnt {
            let it_mean_diff = (*res.offset(i as isize)).gp_ns / 1000.0
                / (*res.offset(i as isize)).gp_ct as f64 - (*gp_stat).mean;
            (*gp_stat).stddev += (it_mean_diff * it_mean_diff) / (res_cnt as f64 - 1.0);
        }
    }
    (*gp_stat).stddev = sqrt((*gp_stat).stddev);
}

#[no_mangle]
pub unsafe extern "C" fn grace_period_ticks_basic_stats(res: *mut bench_res, res_cnt: c_int,
                                                        gp_stat: *mut basic_stats) {
    memset(gp_stat as *mut c_void, 0, mem::size_of::<basic_stats>());
    for i in 0..res_cnt {
        (*gp_stat).mean += (*res.offset(i as isize)).stime
            / (*res.offset(i as isize)).gp_ct as f64 / (0.0 + res_cnt as f64);
    }

    if res_cnt > 1 {
        for i in 0..res_cnt {
            let it_mean_diff = (*res.offset(i as isize)).stime
                / (*res.offset(i as isize)).gp_ct as f64 - (*gp_stat).mean;
            (*gp_stat).stddev += (it_mean_diff * it_mean_diff) / (res_cnt as f64 - 1.0);
        }
    }
    (*gp_stat).stddev = sqrt((*gp_stat).stddev);
}

#[no_mangle]
pub unsafe extern "C" fn hits_drops_report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut hits_mean = 0.0;
    let mut drops_mean = 0.0;
    let mut hits_stddev = 0.0;
    let mut drops_stddev = 0.0;
    let mut total_ops_stddev = 0.0;

    for i in 0..res_cnt {
        hits_mean += (*res.offset(i as isize)).hits as f64 / 1000000.0 / (0.0 + res_cnt as f64);
        drops_mean += (*res.offset(i as isize)).drops as f64 / 1000000.0 / (0.0 + res_cnt as f64);
    }
    let total_ops_mean = hits_mean + drops_mean;

    if res_cnt > 1 {
        for i in 0..res_cnt {
            hits_stddev += (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                * (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
            drops_stddev += (drops_mean - (*res.offset(i as isize)).drops as f64 / 1000000.0)
                * (drops_mean - (*res.offset(i as isize)).drops as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
            let total_ops = (*res.offset(i as isize)).hits + (*res.offset(i as isize)).drops;
            total_ops_stddev += (total_ops_mean - total_ops as f64 / 1000000.0)
                * (total_ops_mean - total_ops as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
        }
        hits_stddev = sqrt(hits_stddev);
        drops_stddev = sqrt(drops_stddev);
        total_ops_stddev = sqrt(total_ops_stddev);
    }
    printf(c"Summary: hits %8.3lf \u{00B1} %5.3lfM/s (%7.3lfM/prod), ".as_ptr(),
           hits_mean, hits_stddev, hits_mean / env.producer_cnt as f64);
    printf(c"drops %8.3lf \u{00B1} %5.3lfM/s, ".as_ptr(), drops_mean, drops_stddev);
    printf(c"total operations %8.3lf \u{00B1} %5.3lfM/s\n".as_ptr(), total_ops_mean, total_ops_stddev);
}

#[no_mangle]
pub unsafe extern "C" fn ops_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    let hits_per_sec = (*res).hits as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);
    let hits_per_prod = hits_per_sec / env.producer_cnt as f64;

    printf(c"Iter %3d (%7.3lfus): ".as_ptr(), iter, (delta_ns - 1000000000) as f64 / 1000.0);
    printf(c"hits %8.3lfM/s (%7.3lfM/prod)\n".as_ptr(), hits_per_sec, hits_per_prod);
}

#[no_mangle]
pub unsafe extern "C" fn ops_report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut hits_mean = 0.0;
    let mut hits_stddev = 0.0;

    for i in 0..res_cnt {
        hits_mean += (*res.offset(i as isize)).hits as f64 / 1000000.0 / (0.0 + res_cnt as f64);
    }

    if res_cnt > 1 {
        for i in 0..res_cnt {
            hits_stddev += (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                * (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
        }
        hits_stddev = sqrt(hits_stddev);
    }
    printf(c"Summary: throughput %8.3lf \u{00B1} %5.3lf M ops/s (%7.3lfM ops/prod), ".as_ptr(),
           hits_mean, hits_stddev, hits_mean / env.producer_cnt as f64);
    printf(c"latency %8.3lf ns/op\n".as_ptr(), 1000.0 / hits_mean * env.producer_cnt as f64);
}

#[no_mangle]
pub unsafe extern "C" fn local_storage_report_progress(iter: c_int, res: *mut bench_res,
                                                       delta_ns: c_long) {
    let delta_sec = delta_ns as f64 / 1000000000.0;
    let hits_per_sec = (*res).hits as f64 / 1000000.0 / delta_sec;
    let important_hits_per_sec = (*res).important_hits as f64 / 1000000.0 / delta_sec;

    printf(c"Iter %3d (%7.3lfus): ".as_ptr(), iter, (delta_ns - 1000000000) as f64 / 1000.0);
    printf(c"hits %8.3lfM/s ".as_ptr(), hits_per_sec);
    printf(c"important_hits %8.3lfM/s\n".as_ptr(), important_hits_per_sec);
}

#[no_mangle]
pub unsafe extern "C" fn local_storage_report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut important_hits_mean = 0.0;
    let mut important_hits_stddev = 0.0;
    let mut hits_mean = 0.0;
    let mut hits_stddev = 0.0;

    for i in 0..res_cnt {
        hits_mean += (*res.offset(i as isize)).hits as f64 / 1000000.0 / (0.0 + res_cnt as f64);
        important_hits_mean += (*res.offset(i as isize)).important_hits as f64 / 1000000.0 / (0.0 + res_cnt as f64);
    }

    if res_cnt > 1 {
        for i in 0..res_cnt {
            hits_stddev += (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                * (hits_mean - (*res.offset(i as isize)).hits as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
            important_hits_stddev += (important_hits_mean - (*res.offset(i as isize)).important_hits as f64 / 1000000.0)
                * (important_hits_mean - (*res.offset(i as isize)).important_hits as f64 / 1000000.0)
                / (res_cnt as f64 - 1.0);
        }

        hits_stddev = sqrt(hits_stddev);
        important_hits_stddev = sqrt(important_hits_stddev);
    }
    printf(c"Summary: hits throughput %8.3lf \u{00B1} %5.3lf M ops/s, ".as_ptr(), hits_mean, hits_stddev);
    printf(c"hits latency %8.3lf ns/op, ".as_ptr(), 1000.0 / hits_mean);
    printf(c"important_hits throughput %8.3lf \u{00B1} %5.3lf M ops/s\n".as_ptr(),
           important_hits_mean, important_hits_stddev);
}

#[no_mangle]
pub static argp_program_version: *const c_char = c"benchmark".as_ptr();
#[no_mangle]
pub static argp_program_bug_address: *const c_char = c"<bpf@vger.kernel.org>".as_ptr();
#[no_mangle]
pub static argp_program_doc: &[u8; 340] =
    b"benchmark    Generic benchmarking framework.\n\
\n\
This tool runs benchmarks.\n\
\n\
USAGE: benchmark <bench-name>\n\
\n\
EXAMPLES:\n\
    # run 'count-local' benchmark with 1 producer and 1 consumer\n\
    benchmark count-local\n\
    # run 'count-local' with 16 producer and 8 consumer thread, pinned to CPUs\n\
    benchmark -p16 -c8 -a count-local\n\0";

static OPTS: [argp_option; 13] = [
    argp_option { name: c"list".as_ptr(), key: 'l' as c_int, arg: ptr::null(), flags: 0, doc: c"List available benchmarks".as_ptr(), group: 0 },
    argp_option { name: c"duration".as_ptr(), key: 'd' as c_int, arg: c"SEC".as_ptr(), flags: 0, doc: c"Duration of benchmark, seconds".as_ptr(), group: 0 },
    argp_option { name: c"warmup".as_ptr(), key: 'w' as c_int, arg: c"SEC".as_ptr(), flags: 0, doc: c"Warm-up period, seconds".as_ptr(), group: 0 },
    argp_option { name: c"producers".as_ptr(), key: 'p' as c_int, arg: c"NUM".as_ptr(), flags: 0, doc: c"Number of producer threads".as_ptr(), group: 0 },
    argp_option { name: c"consumers".as_ptr(), key: 'c' as c_int, arg: c"NUM".as_ptr(), flags: 0, doc: c"Number of consumer threads".as_ptr(), group: 0 },
    argp_option { name: c"verbose".as_ptr(), key: 'v' as c_int, arg: ptr::null(), flags: 0, doc: c"Verbose debug output".as_ptr(), group: 0 },
    argp_option { name: c"affinity".as_ptr(), key: 'a' as c_int, arg: ptr::null(), flags: 0, doc: c"Set consumer/producer thread affinity".as_ptr(), group: 0 },
    argp_option { name: c"quiet".as_ptr(), key: 'q' as c_int, arg: ptr::null(), flags: 0, doc: c"Be more quiet".as_ptr(), group: 0 },
    argp_option { name: c"stacktrace".as_ptr(), key: 's' as c_int, arg: ptr::null(), flags: 0, doc: c"Get stack trace".as_ptr(), group: 0 },
    argp_option { name: c"prod-affinity".as_ptr(), key: ARG_PROD_AFFINITY_SET, arg: c"CPUSET".as_ptr(), flags: 0, doc: c"Set of CPUs for producer threads; implies --affinity".as_ptr(), group: 0 },
    argp_option { name: c"cons-affinity".as_ptr(), key: ARG_CONS_AFFINITY_SET, arg: c"CPUSET".as_ptr(), flags: 0, doc: c"Set of CPUs for consumer threads; implies --affinity".as_ptr(), group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
];

unsafe extern "C" {
    static bench_ringbufs_argp: argp;
    static bench_bloom_map_argp: argp;
    static bench_bpf_loop_argp: argp;
    static bench_bpf_for_argp: argp;
    static bench_local_storage_argp: argp;
    static bench_local_storage_rcu_tasks_trace_argp: argp;
    static bench_strncmp_argp: argp;
    static bench_hashmap_lookup_argp: argp;
    static bench_local_storage_create_argp: argp;
    static bench_htab_mem_argp: argp;
    static bench_trigger_batch_argp: argp;
    static bench_crypto_argp: argp;
    static bench_sockmap_argp: argp;
    static bench_lpm_trie_map_argp: argp;
    static bench_xdp_lb_argp: argp;
}

static BENCH_PARSERS: [argp_child; 16] = unsafe {
    [
        argp_child { argp: &bench_ringbufs_argp, flags: 0, header: c"Ring buffers benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_bloom_map_argp, flags: 0, header: c"Bloom filter map benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_bpf_loop_argp, flags: 0, header: c"bpf_loop helper benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_bpf_for_argp, flags: 0, header: c"bpf_for loop benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_local_storage_argp, flags: 0, header: c"local_storage benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_strncmp_argp, flags: 0, header: c"bpf_strncmp helper benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_local_storage_rcu_tasks_trace_argp, flags: 0, header: c"local_storage RCU Tasks Trace slowdown benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_hashmap_lookup_argp, flags: 0, header: c"Hashmap lookup benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_local_storage_create_argp, flags: 0, header: c"local-storage-create benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_htab_mem_argp, flags: 0, header: c"hash map memory benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_trigger_batch_argp, flags: 0, header: c"BPF triggering benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_crypto_argp, flags: 0, header: c"bpf crypto benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_sockmap_argp, flags: 0, header: c"bpf sockmap benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_lpm_trie_map_argp, flags: 0, header: c"LPM trie map benchmark".as_ptr(), group: 0 },
        argp_child { argp: &bench_xdp_lb_argp, flags: 0, header: c"XDP load-balancer benchmark".as_ptr(), group: 0 },
        argp_child { argp: ptr::null(), flags: 0, header: ptr::null(), group: 0 },
    ]
};

/* Make pos_args global, so that we can run argp_parse twice, if necessary */
static mut POS_ARGS: c_int = 0;

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> ErrorT {
    match key {
        x if x == 'v' as c_int => env.verbose = true,
        x if x == 'l' as c_int => env.list = true,
        x if x == 'd' as c_int => {
            env.duration_sec = strtol(arg, ptr::null_mut(), 10) as c_int;
            if env.duration_sec <= 0 {
                fprintf(stderr, c"Invalid duration: %s\n".as_ptr(), arg);
                argp_usage(state);
            }
        }
        x if x == 'w' as c_int => {
            env.warmup_sec = strtol(arg, ptr::null_mut(), 10) as c_int;
            if env.warmup_sec <= 0 {
                fprintf(stderr, c"Invalid warm-up duration: %s\n".as_ptr(), arg);
                argp_usage(state);
            }
        }
        x if x == 'p' as c_int => {
            env.producer_cnt = strtol(arg, ptr::null_mut(), 10) as c_int;
            if env.producer_cnt < 0 {
                fprintf(stderr, c"Invalid producer count: %s\n".as_ptr(), arg);
                argp_usage(state);
            }
        }
        x if x == 'c' as c_int => {
            env.consumer_cnt = strtol(arg, ptr::null_mut(), 10) as c_int;
            if env.consumer_cnt < 0 {
                fprintf(stderr, c"Invalid consumer count: %s\n".as_ptr(), arg);
                argp_usage(state);
            }
        }
        x if x == 'a' as c_int => env.affinity = true,
        x if x == 'q' as c_int => env.quiet = true,
        x if x == 's' as c_int => env.stacktrace = true,
        ARG_PROD_AFFINITY_SET => {
            env.affinity = true;
            if parse_num_list(arg, &mut env.prod_cpus.cpus, &mut env.prod_cpus.cpus_len) != 0 {
                fprintf(stderr, c"Invalid format of CPU set for producers.".as_ptr());
                argp_usage(state);
            }
        }
        ARG_CONS_AFFINITY_SET => {
            env.affinity = true;
            if parse_num_list(arg, &mut env.cons_cpus.cpus, &mut env.cons_cpus.cpus_len) != 0 {
                fprintf(stderr, c"Invalid format of CPU set for consumers.".as_ptr());
                argp_usage(state);
            }
        }
        ARGP_KEY_ARG => {
            if {
                let old = POS_ARGS;
                POS_ARGS += 1;
                old
            } != 0 {
                fprintf(stderr, c"Unrecognized positional argument: %s\n".as_ptr(), arg);
                argp_usage(state);
            }
            env.bench_name = strdup(arg);
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    0
}

unsafe fn parse_cmdline_args_init(argc: c_int, argv: *mut *mut c_char) {
    static ARGP_INIT: argp = argp {
        options: OPTS.as_ptr(),
        parser: Some(parse_arg),
        args_doc: ptr::null(),
        doc: argp_program_doc.as_ptr() as *const c_char,
        children: BENCH_PARSERS.as_ptr(),
        help_filter: ptr::null(),
        argp_domain: ptr::null(),
    };
    if argp_parse(&ARGP_INIT, argc, argv, 0, ptr::null_mut(), ptr::null_mut()) != 0 {
        exit(1);
    }
}

unsafe fn parse_cmdline_args_final(argc: c_int, argv: *mut *mut c_char) {
    let mut bench_parsers: [argp_child; 2] = [
        argp_child { argp: ptr::null(), flags: 0, header: ptr::null(), group: 0 },
        argp_child { argp: ptr::null(), flags: 0, header: ptr::null(), group: 0 },
    ];
    let argp = argp {
        options: OPTS.as_ptr(),
        parser: Some(parse_arg),
        args_doc: ptr::null(),
        doc: argp_program_doc.as_ptr() as *const c_char,
        children: bench_parsers.as_ptr(),
        help_filter: ptr::null(),
        argp_domain: ptr::null(),
    };

    /* Parse arguments the second time with the correct set of parsers */
    if !(*bench).argp.is_null() {
        bench_parsers[0].argp = (*bench).argp;
        bench_parsers[0].header = (*bench).name;
        POS_ARGS = 0;
        if argp_parse(&argp, argc, argv, 0, ptr::null_mut(), ptr::null_mut()) != 0 {
            exit(1);
        }
    }
}

static mut LAST_TIME_NS: __u64 = 0;

unsafe extern "C" fn sigalarm_handler(_signo: c_int) {
    let new_time_ns = get_time_ns();
    let delta_ns = new_time_ns - LAST_TIME_NS as c_long;

    collect_measurements(delta_ns);

    LAST_TIME_NS = new_time_ns as __u64;
}

/* set up periodic 1-second timer */
unsafe fn setup_timer() {
    static SIGALARM_ACTION: sigaction = sigaction {
        sa_handler: Some(sigalarm_handler),
    };
    let mut timer_settings: itimerval = mem::zeroed();
    let mut err: c_int;

    LAST_TIME_NS = get_time_ns() as __u64;
    err = sigaction(SIGALRM, &SIGALARM_ACTION, ptr::null_mut());
    if err < 0 {
        fprintf(stderr, c"failed to install SIGALRM handler: %d\n".as_ptr(), -errno);
        exit(1);
    }
    timer_settings.it_interval.tv_sec = 1;
    timer_settings.it_value.tv_sec = 1;
    err = setitimer(ITIMER_REAL, &timer_settings, ptr::null_mut());
    if err < 0 {
        fprintf(stderr, c"failed to arm interval timer: %d\n".as_ptr(), -errno);
        exit(1);
    }
}

unsafe fn cpu_zero(cpuset: *mut cpu_set_t) {
    memset(cpuset as *mut c_void, 0, mem::size_of::<cpu_set_t>());
}

unsafe fn cpu_set(cpu: c_int, cpuset: *mut cpu_set_t) {
    let bits = cpuset as *mut u64;
    *bits.add((cpu as usize) / 64) |= 1u64 << ((cpu as usize) % 64);
}

unsafe fn set_thread_affinity(thread: PthreadT, cpu: c_int) {
    let mut cpuset: cpu_set_t = mem::zeroed();

    cpu_zero(&mut cpuset);
    cpu_set(cpu, &mut cpuset);
    let err = pthread_setaffinity_np(thread, mem::size_of::<cpu_set_t>(), &cpuset);
    if err != 0 {
        fprintf(stderr, c"setting affinity to CPU #%d failed: %d\n".as_ptr(), cpu, -err);
        exit(1);
    }
}

unsafe fn next_cpu(cpu_set: *mut cpu_set) -> c_int {
    if !(*cpu_set).cpus.is_null() {
        let mut i: c_int;

        /* find next available CPU */
        i = (*cpu_set).next_cpu;
        while i < (*cpu_set).cpus_len {
            if *(*cpu_set).cpus.offset(i as isize) != 0 {
                (*cpu_set).next_cpu = i + 1;
                return i;
            }
            i += 1;
        }
        fprintf(stderr, c"Not enough CPUs specified, need CPU #%d or higher.\n".as_ptr(), i);
        exit(1);
    }

    let cpu = (*cpu_set).next_cpu % env.nr_cpus;
    (*cpu_set).next_cpu += 1;
    cpu
}

static mut STATE: bench_state = bench_state {
    res_cnt: 0,
    results: ptr::null_mut(),
    consumers: ptr::null_mut(),
    producers: ptr::null_mut(),
};

#[no_mangle]
pub static mut bench: *const bench = ptr::null();

unsafe extern "C" {
    static bench_count_global: bench;
    static bench_count_local: bench;
    static bench_rename_base: bench;
    static bench_rename_kprobe: bench;
    static bench_rename_kretprobe: bench;
    static bench_rename_rawtp: bench;
    static bench_rename_fentry: bench;
    static bench_rename_fexit: bench;

    /* pure counting benchmarks to establish theoretical limits */
    static bench_trig_usermode_count: bench;
    static bench_trig_syscall_count: bench;
    static bench_trig_kernel_count: bench;

    /* batched, staying mostly in-kernel benchmarks */
    static bench_trig_kprobe: bench;
    static bench_trig_kretprobe: bench;
    static bench_trig_kprobe_multi: bench;
    static bench_trig_kretprobe_multi: bench;
    static bench_trig_fentry: bench;
    static bench_trig_kprobe_multi_all: bench;
    static bench_trig_kretprobe_multi_all: bench;
    static bench_trig_fexit: bench;
    static bench_trig_fmodret: bench;
    static bench_trig_tp: bench;
    static bench_trig_rawtp: bench;

    /* uprobe/uretprobe benchmarks */
    static bench_trig_uprobe_nop: bench;
    static bench_trig_uretprobe_nop: bench;
    static bench_trig_uprobe_push: bench;
    static bench_trig_uretprobe_push: bench;
    static bench_trig_uprobe_ret: bench;
    static bench_trig_uretprobe_ret: bench;
    static bench_trig_uprobe_multi_nop: bench;
    static bench_trig_uretprobe_multi_nop: bench;
    static bench_trig_uprobe_multi_push: bench;
    static bench_trig_uretprobe_multi_push: bench;
    static bench_trig_uprobe_multi_ret: bench;
    static bench_trig_uretprobe_multi_ret: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_uprobe_nop10: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_uretprobe_nop10: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_uprobe_multi_nop10: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_uretprobe_multi_nop10: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_usdt_nop: bench;
    #[cfg(target_arch = "x86_64")]
    static bench_trig_usdt_nop10: bench;

    static bench_rb_libbpf: bench;
    static bench_rb_custom: bench;
    static bench_pb_libbpf: bench;
    static bench_pb_custom: bench;
    static bench_bloom_lookup: bench;
    static bench_bloom_update: bench;
    static bench_bloom_false_positive: bench;
    static bench_hashmap_without_bloom: bench;
    static bench_hashmap_with_bloom: bench;
    static bench_bpf_loop: bench;
    static bench_bpf_for: bench;
    static bench_strncmp_no_helper: bench;
    static bench_strncmp_helper: bench;
    static bench_bpf_hashmap_full_update: bench;
    static bench_bpf_rhashmap_full_update: bench;
    static bench_local_storage_cache_seq_get: bench;
    static bench_local_storage_cache_interleaved_get: bench;
    static bench_local_storage_cache_hashmap_control: bench;
    static bench_local_storage_tasks_trace: bench;
    static bench_bpf_hashmap_lookup: bench;
    static bench_bpf_rhashmap_lookup: bench;
    static bench_local_storage_create: bench;
    static bench_htab_mem: bench;
    static bench_rhtab_mem: bench;
    static bench_crypto_encrypt: bench;
    static bench_crypto_decrypt: bench;
    static bench_sockmap: bench;
    static bench_lpm_trie_noop: bench;
    static bench_lpm_trie_baseline: bench;
    static bench_lpm_trie_lookup: bench;
    static bench_lpm_trie_insert: bench;
    static bench_lpm_trie_update: bench;
    static bench_lpm_trie_delete: bench;
    static bench_lpm_trie_free: bench;
    static bench_bpf_nop: bench;
    static bench_xdp_lb: bench;
}

#[cfg(target_arch = "x86_64")]
static BENCHS: [*const bench; 73] = unsafe {
    [
        &bench_count_global, &bench_count_local, &bench_rename_base, &bench_rename_kprobe,
        &bench_rename_kretprobe, &bench_rename_rawtp, &bench_rename_fentry, &bench_rename_fexit,
        /* pure counting benchmarks for establishing theoretical limits */
        &bench_trig_usermode_count, &bench_trig_kernel_count, &bench_trig_syscall_count,
        /* batched, staying mostly in-kernel triggers */
        &bench_trig_kprobe, &bench_trig_kretprobe, &bench_trig_kprobe_multi,
        &bench_trig_kretprobe_multi, &bench_trig_fentry, &bench_trig_kprobe_multi_all,
        &bench_trig_kretprobe_multi_all, &bench_trig_fexit, &bench_trig_fmodret,
        &bench_trig_tp, &bench_trig_rawtp,
        /* uprobes */
        &bench_trig_uprobe_nop, &bench_trig_uretprobe_nop, &bench_trig_uprobe_push,
        &bench_trig_uretprobe_push, &bench_trig_uprobe_ret, &bench_trig_uretprobe_ret,
        &bench_trig_uprobe_multi_nop, &bench_trig_uretprobe_multi_nop,
        &bench_trig_uprobe_multi_push, &bench_trig_uretprobe_multi_push,
        &bench_trig_uprobe_multi_ret, &bench_trig_uretprobe_multi_ret,
        &bench_trig_uprobe_nop10, &bench_trig_uretprobe_nop10,
        &bench_trig_uprobe_multi_nop10, &bench_trig_uretprobe_multi_nop10,
        &bench_trig_usdt_nop, &bench_trig_usdt_nop10,
        /* ringbuf/perfbuf benchmarks */
        &bench_rb_libbpf, &bench_rb_custom, &bench_pb_libbpf, &bench_pb_custom,
        &bench_bloom_lookup, &bench_bloom_update, &bench_bloom_false_positive,
        &bench_hashmap_without_bloom, &bench_hashmap_with_bloom, &bench_bpf_loop,
        &bench_bpf_for, &bench_strncmp_no_helper, &bench_strncmp_helper,
        &bench_bpf_hashmap_full_update, &bench_bpf_rhashmap_full_update,
        &bench_local_storage_cache_seq_get, &bench_local_storage_cache_interleaved_get,
        &bench_local_storage_cache_hashmap_control, &bench_local_storage_tasks_trace,
        &bench_bpf_hashmap_lookup, &bench_bpf_rhashmap_lookup, &bench_local_storage_create,
        &bench_htab_mem, &bench_rhtab_mem, &bench_crypto_encrypt, &bench_crypto_decrypt,
        &bench_sockmap, &bench_lpm_trie_noop, &bench_lpm_trie_baseline,
        &bench_lpm_trie_lookup, &bench_lpm_trie_insert, &bench_lpm_trie_update,
        &bench_lpm_trie_delete, &bench_lpm_trie_free, &bench_bpf_nop, &bench_xdp_lb,
    ]
};

#[cfg(not(target_arch = "x86_64"))]
static BENCHS: [*const bench; 67] = unsafe {
    [
        &bench_count_global, &bench_count_local, &bench_rename_base, &bench_rename_kprobe,
        &bench_rename_kretprobe, &bench_rename_rawtp, &bench_rename_fentry, &bench_rename_fexit,
        &bench_trig_usermode_count, &bench_trig_kernel_count, &bench_trig_syscall_count,
        &bench_trig_kprobe, &bench_trig_kretprobe, &bench_trig_kprobe_multi,
        &bench_trig_kretprobe_multi, &bench_trig_fentry, &bench_trig_kprobe_multi_all,
        &bench_trig_kretprobe_multi_all, &bench_trig_fexit, &bench_trig_fmodret,
        &bench_trig_tp, &bench_trig_rawtp, &bench_trig_uprobe_nop, &bench_trig_uretprobe_nop,
        &bench_trig_uprobe_push, &bench_trig_uretprobe_push, &bench_trig_uprobe_ret,
        &bench_trig_uretprobe_ret, &bench_trig_uprobe_multi_nop, &bench_trig_uretprobe_multi_nop,
        &bench_trig_uprobe_multi_push, &bench_trig_uretprobe_multi_push,
        &bench_trig_uprobe_multi_ret, &bench_trig_uretprobe_multi_ret,
        &bench_rb_libbpf, &bench_rb_custom, &bench_pb_libbpf, &bench_pb_custom,
        &bench_bloom_lookup, &bench_bloom_update, &bench_bloom_false_positive,
        &bench_hashmap_without_bloom, &bench_hashmap_with_bloom, &bench_bpf_loop,
        &bench_bpf_for, &bench_strncmp_no_helper, &bench_strncmp_helper,
        &bench_bpf_hashmap_full_update, &bench_bpf_rhashmap_full_update,
        &bench_local_storage_cache_seq_get, &bench_local_storage_cache_interleaved_get,
        &bench_local_storage_cache_hashmap_control, &bench_local_storage_tasks_trace,
        &bench_bpf_hashmap_lookup, &bench_bpf_rhashmap_lookup, &bench_local_storage_create,
        &bench_htab_mem, &bench_rhtab_mem, &bench_crypto_encrypt, &bench_crypto_decrypt,
        &bench_sockmap, &bench_lpm_trie_noop, &bench_lpm_trie_baseline,
        &bench_lpm_trie_lookup, &bench_lpm_trie_insert, &bench_lpm_trie_update,
        &bench_lpm_trie_delete, &bench_lpm_trie_free, &bench_bpf_nop, &bench_xdp_lb,
    ]
};

unsafe fn find_benchmark() {
    if env.bench_name.is_null() {
        fprintf(stderr, c"benchmark name is not specified\n".as_ptr());
        exit(1);
    }
    for i in 0..BENCHS.len() {
        if strcmp((*BENCHS[i]).name, env.bench_name) == 0 {
            bench = BENCHS[i];
            break;
        }
    }
    if bench.is_null() {
        fprintf(stderr, c"benchmark '%s' not found\n".as_ptr(), env.bench_name);
        exit(1);
    }
}

unsafe fn setup_benchmark() {
    let mut err: c_int;

    if !env.quiet {
        printf(c"Setting up benchmark '%s'...\n".as_ptr(), (*bench).name);
    }

    STATE.producers = calloc(env.producer_cnt as SizeT, mem::size_of::<PthreadT>()) as *mut PthreadT;
    STATE.consumers = calloc(env.consumer_cnt as SizeT, mem::size_of::<PthreadT>()) as *mut PthreadT;
    STATE.results = calloc((env.duration_sec + env.warmup_sec + 2) as SizeT,
                           mem::size_of::<bench_res>()) as *mut bench_res;
    if STATE.producers.is_null() || STATE.consumers.is_null() || STATE.results.is_null() {
        exit(1);
    }

    if let Some(validate) = (*bench).validate {
        validate();
    }
    if let Some(setup) = (*bench).setup {
        setup();
    }

    for i in 0..env.consumer_cnt {
        if (*bench).consumer_thread.is_none() {
            fprintf(stderr, c"benchmark doesn't support consumers!\n".as_ptr());
            exit(1);
        }
        err = pthread_create(STATE.consumers.offset(i as isize), ptr::null(),
                             (*bench).consumer_thread, i as c_long as *mut c_void);
        if err != 0 {
            fprintf(stderr, c"failed to create consumer thread #%d: %d\n".as_ptr(), i, -err);
            exit(1);
        }
        if env.affinity {
            set_thread_affinity(*STATE.consumers.offset(i as isize), next_cpu(&mut env.cons_cpus));
        }
    }

    /* unless explicit producer CPU list is specified, continue after
     * last consumer CPU
     */
    if env.prod_cpus.cpus.is_null() {
        env.prod_cpus.next_cpu = env.cons_cpus.next_cpu;
    }

    for i in 0..env.producer_cnt {
        if (*bench).producer_thread.is_none() {
            fprintf(stderr, c"benchmark doesn't support producers!\n".as_ptr());
            exit(1);
        }
        err = pthread_create(STATE.producers.offset(i as isize), ptr::null(),
                             (*bench).producer_thread, i as c_long as *mut c_void);
        if err != 0 {
            fprintf(stderr, c"failed to create producer thread #%d: %d\n".as_ptr(), i, -err);
            exit(1);
        }
        if env.affinity {
            set_thread_affinity(*STATE.producers.offset(i as isize), next_cpu(&mut env.prod_cpus));
        }
    }

    if !env.quiet {
        printf(c"Benchmark '%s' started.\n".as_ptr(), (*bench).name);
    }
}

static mut BENCH_DONE_MTX: pthread_mutex_t = pthread_mutex_t { _private: [0; 40] };
static mut BENCH_DONE: pthread_cond_t = pthread_cond_t { _private: [0; 48] };

#[no_mangle]
pub unsafe extern "C" fn bench_force_done() {
    pthread_mutex_lock(&mut BENCH_DONE_MTX);
    pthread_cond_signal(&mut BENCH_DONE);
    pthread_mutex_unlock(&mut BENCH_DONE_MTX);
}

unsafe fn collect_measurements(delta_ns: c_long) {
    let iter = STATE.res_cnt;
    STATE.res_cnt += 1;
    let res = STATE.results.offset(iter as isize);

    if let Some(measure) = (*bench).measure {
        measure(res);
    }

    if let Some(report_progress) = (*bench).report_progress {
        report_progress(iter, res, delta_ns);
    }

    if iter == env.duration_sec + env.warmup_sec {
        bench_force_done();
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    env.nr_cpus = get_nprocs();
    parse_cmdline_args_init(argc, argv);

    if env.list {
        printf(c"Available benchmarks:\n".as_ptr());
        for i in 0..BENCHS.len() {
            printf(c"- %s\n".as_ptr(), (*BENCHS[i]).name);
        }
        return 0;
    }

    find_benchmark();
    parse_cmdline_args_final(argc, argv);

    setup_benchmark();

    setup_timer();

    pthread_mutex_lock(&mut BENCH_DONE_MTX);
    pthread_cond_wait(&mut BENCH_DONE, &mut BENCH_DONE_MTX);
    pthread_mutex_unlock(&mut BENCH_DONE_MTX);

    if let Some(report_final) = (*bench).report_final {
        /* skip first sample */
        report_final(STATE.results.offset(env.warmup_sec as isize),
                     STATE.res_cnt - env.warmup_sec);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
