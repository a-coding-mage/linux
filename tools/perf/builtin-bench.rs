// SPDX-License-Identifier: GPL-2.0
/*
 * builtin-bench.c
 *
 * General benchmarking collections provided by perf
 *
 * Copyright (C) 2009, Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
 */

/*
 * Available benchmark collection list:
 *
 *  sched ... scheduler and IPC performance
 *  syscall ... System call performance
 *  mem   ... memory access performance
 *  numa  ... NUMA scheduling and MM performance
 *  futex ... Futex performance
 *  epoll ... Event poll performance
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type bench_fn_t = Option<unsafe extern "C" fn(argc: c_int, argv: *mut *const c_char) -> c_int>;

#[repr(C)]
struct bench {
    name: *const c_char,
    summary: *const c_char,
    fn_: bench_fn_t,
}

#[repr(C)]
struct collection {
    name: *const c_char,
    summary: *const c_char,
    benchmarks: *const bench,
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn setvbuf(stream: *mut c_void, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn prctl(option: c_int, ...) -> c_int;

    fn zalloc(size: usize) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn BUG_ON(condition: bool);

    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;

    static bench_options: [option; 3];

    fn bench_sched_messaging(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_sched_pipe(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_sched_seccomp_notify(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_syscall_basic(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_syscall_getpgid(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_syscall_fork(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_syscall_execve(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_mem_memcpy(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_mem_memset(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_mem_find_bit(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_mem_mmap(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_futex_hash(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_futex_wake(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_futex_wake_parallel(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_futex_requeue(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_futex_lock_pi(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_synthesize(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_kallsyms_parse(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_inject_build_id(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_evlist_open_close(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_pmu_scan(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_breakpoint_thread(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_breakpoint_enable(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_uprobe_baseline(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_uprobe_empty(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_uprobe_trace_printk(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_uprobe_empty_ret(argc: c_int, argv: *mut *const c_char) -> c_int;
    fn bench_uprobe_trace_printk_ret(argc: c_int, argv: *mut *const c_char) -> c_int;

    #[cfg(HAVE_LIBNUMA_SUPPORT)]
    fn bench_numa(argc: c_int, argv: *mut *const c_char) -> c_int;
    #[cfg(HAVE_EVENTFD_SUPPORT)]
    fn bench_epoll_wait(argc: c_int, argv: *mut *const c_char) -> c_int;
    #[cfg(HAVE_EVENTFD_SUPPORT)]
    fn bench_epoll_ctl(argc: c_int, argv: *mut *const c_char) -> c_int;
}

const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_DEFAULT_STR: *const c_char = b"default\0".as_ptr() as *const c_char;
const BENCH_FORMAT_SIMPLE: c_int = 1;
const BENCH_FORMAT_SIMPLE_STR: *const c_char = b"simple\0".as_ptr() as *const c_char;
const BENCH_FORMAT_UNKNOWN: c_int = -1;
const PARSE_OPT_STOP_AT_NON_OPTION: c_int = 1;
const PR_SET_NAME: c_int = 15;
const _IONBF: c_int = 2;
const LC_ALL: c_int = 6;

#[cfg(HAVE_LIBNUMA_SUPPORT)]
static numa_benchmarks: [bench; 3] = [
    bench { name: b"mem\0".as_ptr() as *const c_char, summary: b"Benchmark for NUMA workloads\0".as_ptr() as *const c_char, fn_: Some(bench_numa) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all NUMA benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static sched_benchmarks: [bench; 5] = [
    bench { name: b"messaging\0".as_ptr() as *const c_char, summary: b"Benchmark for scheduling and IPC\0".as_ptr() as *const c_char, fn_: Some(bench_sched_messaging) },
    bench { name: b"pipe\0".as_ptr() as *const c_char, summary: b"Benchmark for pipe() between two processes\0".as_ptr() as *const c_char, fn_: Some(bench_sched_pipe) },
    bench { name: b"seccomp-notify\0".as_ptr() as *const c_char, summary: b"Benchmark for seccomp user notify\0".as_ptr() as *const c_char, fn_: Some(bench_sched_seccomp_notify) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all scheduler benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static syscall_benchmarks: [bench; 6] = [
    bench { name: b"basic\0".as_ptr() as *const c_char, summary: b"Benchmark for basic getppid(2) calls\0".as_ptr() as *const c_char, fn_: Some(bench_syscall_basic) },
    bench { name: b"getpgid\0".as_ptr() as *const c_char, summary: b"Benchmark for getpgid(2) calls\0".as_ptr() as *const c_char, fn_: Some(bench_syscall_getpgid) },
    bench { name: b"fork\0".as_ptr() as *const c_char, summary: b"Benchmark for fork(2) calls\0".as_ptr() as *const c_char, fn_: Some(bench_syscall_fork) },
    bench { name: b"execve\0".as_ptr() as *const c_char, summary: b"Benchmark for execve(2) calls\0".as_ptr() as *const c_char, fn_: Some(bench_syscall_execve) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all syscall benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static mem_benchmarks: [bench; 6] = [
    bench { name: b"memcpy\0".as_ptr() as *const c_char, summary: b"Benchmark for memcpy() functions\0".as_ptr() as *const c_char, fn_: Some(bench_mem_memcpy) },
    bench { name: b"memset\0".as_ptr() as *const c_char, summary: b"Benchmark for memset() functions\0".as_ptr() as *const c_char, fn_: Some(bench_mem_memset) },
    bench { name: b"find_bit\0".as_ptr() as *const c_char, summary: b"Benchmark for find_bit() functions\0".as_ptr() as *const c_char, fn_: Some(bench_mem_find_bit) },
    bench { name: b"mmap\0".as_ptr() as *const c_char, summary: b"Benchmark for mmap() mappings\0".as_ptr() as *const c_char, fn_: Some(bench_mem_mmap) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all memory access benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static futex_benchmarks: [bench; 7] = [
    bench { name: b"hash\0".as_ptr() as *const c_char, summary: b"Benchmark for futex hash table\0".as_ptr() as *const c_char, fn_: Some(bench_futex_hash) },
    bench { name: b"wake\0".as_ptr() as *const c_char, summary: b"Benchmark for futex wake calls\0".as_ptr() as *const c_char, fn_: Some(bench_futex_wake) },
    bench { name: b"wake-parallel\0".as_ptr() as *const c_char, summary: b"Benchmark for parallel futex wake calls\0".as_ptr() as *const c_char, fn_: Some(bench_futex_wake_parallel) },
    bench { name: b"requeue\0".as_ptr() as *const c_char, summary: b"Benchmark for futex requeue calls\0".as_ptr() as *const c_char, fn_: Some(bench_futex_requeue) },
    /* pi-futexes */
    bench { name: b"lock-pi\0".as_ptr() as *const c_char, summary: b"Benchmark for futex lock_pi calls\0".as_ptr() as *const c_char, fn_: Some(bench_futex_lock_pi) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all futex benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

#[cfg(HAVE_EVENTFD_SUPPORT)]
static epoll_benchmarks: [bench; 4] = [
    bench { name: b"wait\0".as_ptr() as *const c_char, summary: b"Benchmark epoll concurrent epoll_waits\0".as_ptr() as *const c_char, fn_: Some(bench_epoll_wait) },
    bench { name: b"ctl\0".as_ptr() as *const c_char, summary: b"Benchmark epoll concurrent epoll_ctls\0".as_ptr() as *const c_char, fn_: Some(bench_epoll_ctl) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all futex benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static internals_benchmarks: [bench; 6] = [
    bench { name: b"synthesize\0".as_ptr() as *const c_char, summary: b"Benchmark perf event synthesis\0".as_ptr() as *const c_char, fn_: Some(bench_synthesize) },
    bench { name: b"kallsyms-parse\0".as_ptr() as *const c_char, summary: b"Benchmark kallsyms parsing\0".as_ptr() as *const c_char, fn_: Some(bench_kallsyms_parse) },
    bench { name: b"inject-build-id\0".as_ptr() as *const c_char, summary: b"Benchmark build-id injection\0".as_ptr() as *const c_char, fn_: Some(bench_inject_build_id) },
    bench { name: b"evlist-open-close\0".as_ptr() as *const c_char, summary: b"Benchmark evlist open and close\0".as_ptr() as *const c_char, fn_: Some(bench_evlist_open_close) },
    bench { name: b"pmu-scan\0".as_ptr() as *const c_char, summary: b"Benchmark sysfs PMU info scanning\0".as_ptr() as *const c_char, fn_: Some(bench_pmu_scan) },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static breakpoint_benchmarks: [bench; 4] = [
    bench { name: b"thread\0".as_ptr() as *const c_char, summary: b"Benchmark thread start/finish with breakpoints\0".as_ptr() as *const c_char, fn_: Some(bench_breakpoint_thread) },
    bench { name: b"enable\0".as_ptr() as *const c_char, summary: b"Benchmark breakpoint enable/disable\0".as_ptr() as *const c_char, fn_: Some(bench_breakpoint_enable) },
    bench { name: b"all\0".as_ptr() as *const c_char, summary: b"Run all breakpoint benchmarks\0".as_ptr() as *const c_char, fn_: None },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static uprobe_benchmarks: [bench; 6] = [
    bench { name: b"baseline\0".as_ptr() as *const c_char, summary: b"Baseline libc usleep(1000) call\0".as_ptr() as *const c_char, fn_: Some(bench_uprobe_baseline) },
    bench { name: b"empty\0".as_ptr() as *const c_char, summary: b"Attach empty BPF prog to uprobe on usleep, system wide\0".as_ptr() as *const c_char, fn_: Some(bench_uprobe_empty) },
    bench { name: b"trace_printk\0".as_ptr() as *const c_char, summary: b"Attach trace_printk BPF prog to uprobe on usleep syswide\0".as_ptr() as *const c_char, fn_: Some(bench_uprobe_trace_printk) },
    bench { name: b"empty_ret\0".as_ptr() as *const c_char, summary: b"Attach empty BPF prog to uretprobe on usleep, system wide\0".as_ptr() as *const c_char, fn_: Some(bench_uprobe_empty_ret) },
    bench { name: b"trace_printk_ret\0".as_ptr() as *const c_char, summary: b"Attach trace_printk BPF prog to uretprobe on usleep syswide\0".as_ptr() as *const c_char, fn_: Some(bench_uprobe_trace_printk_ret) },
    bench { name: core::ptr::null(), summary: core::ptr::null(), fn_: None },
];

static collections: [collection; 10] = [
    collection { name: b"sched\0".as_ptr() as *const c_char, summary: b"Scheduler and IPC benchmarks\0".as_ptr() as *const c_char, benchmarks: sched_benchmarks.as_ptr() },
    collection { name: b"syscall\0".as_ptr() as *const c_char, summary: b"System call benchmarks\0".as_ptr() as *const c_char, benchmarks: syscall_benchmarks.as_ptr() },
    collection { name: b"mem\0".as_ptr() as *const c_char, summary: b"Memory access benchmarks\0".as_ptr() as *const c_char, benchmarks: mem_benchmarks.as_ptr() },
    /* #ifdef HAVE_LIBNUMA_SUPPORT: { "numa", "NUMA scheduling and MM benchmarks", numa_benchmarks } */
    collection { name: b"futex\0".as_ptr() as *const c_char, summary: b"Futex stressing benchmarks\0".as_ptr() as *const c_char, benchmarks: futex_benchmarks.as_ptr() },
    /* #ifdef HAVE_EVENTFD_SUPPORT: { "epoll", "Epoll stressing benchmarks", epoll_benchmarks } */
    collection { name: b"internals\0".as_ptr() as *const c_char, summary: b"Perf-internals benchmarks\0".as_ptr() as *const c_char, benchmarks: internals_benchmarks.as_ptr() },
    collection { name: b"breakpoint\0".as_ptr() as *const c_char, summary: b"Breakpoint benchmarks\0".as_ptr() as *const c_char, benchmarks: breakpoint_benchmarks.as_ptr() },
    collection { name: b"uprobe\0".as_ptr() as *const c_char, summary: b"uprobe benchmarks\0".as_ptr() as *const c_char, benchmarks: uprobe_benchmarks.as_ptr() },
    collection { name: b"all\0".as_ptr() as *const c_char, summary: b"All benchmarks\0".as_ptr() as *const c_char, benchmarks: core::ptr::null() },
    collection { name: core::ptr::null(), summary: core::ptr::null(), benchmarks: core::ptr::null() },
];

static mut bench_format_str: *const c_char = core::ptr::null();

/* Output/formatting style, exported to benchmark modules: */
#[unsafe(no_mangle)]
pub static mut bench_format: c_int = BENCH_FORMAT_DEFAULT;
#[unsafe(no_mangle)]
pub static mut bench_repeat: c_uint = 10; /* default number of times to repeat the run */

static bench_usage: [*const c_char; 2] = [
    b"perf bench [<common options>] <collection> <benchmark> [<options>]\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe fn dump_benchmarks(coll: *const collection) {
    let mut benchp: *const bench;

    printf(
        b"\n        # List of available benchmarks for collection '%s':\n\n\0".as_ptr() as *const c_char,
        (*coll).name,
    );

    benchp = (*coll).benchmarks;
    while !benchp.is_null() && !(*benchp).name.is_null() {
        printf(
            b"%14s: %s\n\0".as_ptr() as *const c_char,
            (*benchp).name,
            (*benchp).summary,
        );
        benchp = benchp.add(1);
    }

    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn print_usage() {
    let mut coll: *const collection;
    let mut i: c_int;

    printf(b"Usage: \n\0".as_ptr() as *const c_char);
    i = 0;
    while !bench_usage[i as usize].is_null() {
        printf(b"\t%s\n\0".as_ptr() as *const c_char, bench_usage[i as usize]);
        i += 1;
    }
    printf(b"\n\0".as_ptr() as *const c_char);

    printf(b"        # List of all available benchmark collections:\n\n\0".as_ptr() as *const c_char);

    coll = collections.as_ptr();
    while !(*coll).name.is_null() {
        printf(
            b"%14s: %s\n\0".as_ptr() as *const c_char,
            (*coll).name,
            (*coll).summary,
        );
        coll = coll.add(1);
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn bench_str2int(str_: *const c_char) -> c_int {
    if str_.is_null() {
        return BENCH_FORMAT_DEFAULT;
    }

    if strcmp(str_, BENCH_FORMAT_DEFAULT_STR) == 0 {
        return BENCH_FORMAT_DEFAULT;
    } else if strcmp(str_, BENCH_FORMAT_SIMPLE_STR) == 0 {
        return BENCH_FORMAT_SIMPLE;
    }

    BENCH_FORMAT_UNKNOWN
}

/*
 * Run a specific benchmark but first rename the running task's ->comm[]
 * to something meaningful:
 */
unsafe fn run_bench(
    coll_name: *const c_char,
    bench_name: *const c_char,
    fn_: bench_fn_t,
    argc: c_int,
    argv: *mut *const c_char,
) -> c_int {
    let size: c_int;
    let name: *mut c_char;
    let ret: c_int;

    size = (strlen(coll_name) + 1 + strlen(bench_name) + 1) as c_int;

    name = zalloc(size as usize);
    BUG_ON(name.is_null());

    scnprintf(
        name,
        size as usize,
        b"%s-%s\0".as_ptr() as *const c_char,
        coll_name,
        bench_name,
    );

    prctl(PR_SET_NAME, name);
    *argv.add(0) = name as *const c_char;

    ret = fn_.expect("benchmark function pointer must not be NULL")(argc, argv);

    free(name as *mut c_void);

    ret
}

unsafe fn run_collection(coll: *const collection) {
    let mut benchp: *const bench;
    let mut argv: [*const c_char; 2] = [core::ptr::null(); 2];

    argv[1] = core::ptr::null();
    /*
     * TODO:
     *
     * Preparing preset parameters for
     * embedded, ordinary PC, HPC, etc...
     * would be helpful.
     */
    benchp = (*coll).benchmarks;
    while !benchp.is_null() && !(*benchp).name.is_null() {
        if (*benchp).fn_.is_none() {
            break;
        }
        printf(
            b"# Running %s/%s benchmark...\n\0".as_ptr() as *const c_char,
            (*coll).name,
            (*benchp).name,
        );

        argv[1] = (*benchp).name;
        run_bench((*coll).name, (*benchp).name, (*benchp).fn_, 1, argv.as_mut_ptr());
        printf(b"\n\0".as_ptr() as *const c_char);
        benchp = benchp.add(1);
    }
}

unsafe fn run_all_collections() {
    let mut coll: *const collection;

    coll = collections.as_ptr();
    while !(*coll).name.is_null() {
        run_collection(coll);
        coll = coll.add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_bench(mut argc: c_int, mut argv: *mut *const c_char) -> c_int {
    let mut coll: *const collection;
    let mut ret: c_int = 0;

    /* Unbuffered output */
    setvbuf(stdout, core::ptr::null_mut(), _IONBF, 0);
    setlocale(LC_ALL, b"\0".as_ptr() as *const c_char);

    if argc < 2 {
        /* No collection specified. */
        print_usage();
        return ret;
    }

    argc = parse_options(
        argc,
        argv,
        bench_options.as_ptr(),
        bench_usage.as_ptr(),
        PARSE_OPT_STOP_AT_NON_OPTION,
    );

    bench_format = bench_str2int(bench_format_str);
    if bench_format == BENCH_FORMAT_UNKNOWN {
        printf(
            b"Unknown format descriptor: '%s'\n\0".as_ptr() as *const c_char,
            bench_format_str,
        );
        return ret;
    }

    if bench_repeat == 0 {
        printf(b"Invalid repeat option: Must specify a positive value\n\0".as_ptr() as *const c_char);
        return ret;
    }

    if argc < 1 {
        print_usage();
        return ret;
    }

    if strcmp(*argv.add(0), b"all\0".as_ptr() as *const c_char) == 0 {
        run_all_collections();
        return ret;
    }

    coll = collections.as_ptr();
    while !(*coll).name.is_null() {
        let mut benchp: *const bench;

        if strcmp((*coll).name, *argv.add(0)) != 0 {
            coll = coll.add(1);
            continue;
        }

        if argc < 2 {
            /* No bench specified. */
            dump_benchmarks(coll);
            return ret;
        }

        if strcmp(*argv.add(1), b"all\0".as_ptr() as *const c_char) == 0 {
            run_collection(coll);
            return ret;
        }

        benchp = (*coll).benchmarks;
        while !benchp.is_null() && !(*benchp).name.is_null() {
            if strcmp((*benchp).name, *argv.add(1)) != 0 {
                benchp = benchp.add(1);
                continue;
            }

            if bench_format == BENCH_FORMAT_DEFAULT {
                printf(
                    b"# Running '%s/%s' benchmark:\n\0".as_ptr() as *const c_char,
                    (*coll).name,
                    (*benchp).name,
                );
            }
            ret = run_bench((*coll).name, (*benchp).name, (*benchp).fn_, argc - 1, argv.add(1));
            return ret;
        }

        if strcmp(*argv.add(1), b"-h\0".as_ptr() as *const c_char) == 0
            || strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0
        {
            dump_benchmarks(coll);
            return ret;
        }

        printf(
            b"Unknown benchmark: '%s' for collection '%s'\n\0".as_ptr() as *const c_char,
            *argv.add(1),
            *argv.add(0),
        );
        ret = 1;
        return ret;
    }

    printf(
        b"Unknown collection: '%s'\n\0".as_ptr() as *const c_char,
        *argv.add(0),
    );
    ret = 1;

    ret
}
