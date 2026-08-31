// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from C. Original includes:
 * <argp.h>
 * <sys/prctl.h>
 * "local_storage_rcu_tasks_trace_bench.skel.h"
 * "bench.h"
 * <signal.h>
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_void};
use core::ptr;

type __u32 = u32;
type error_t = c_int;

const ARG_NR_PROCS: c_int = 7000;
const ARG_KTHREAD_PID: c_int = 7001;
const MAX_SLEEP_PROCS: __u32 = 150000;

const ARGP_ERR_UNKNOWN: error_t = 7;
const UINT_MAX: c_long = c_uint::MAX as c_long;
const PR_SET_PDEATHSIG: c_int = 1;
const SIGKILL: c_int = 9;
const __NR_getpgid: c_long = 121;

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
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct local_storage_rcu_tasks_trace_bench {
    pub progs: local_storage_rcu_tasks_trace_bench_progs,
    pub bss: *mut local_storage_rcu_tasks_trace_bench_bss,
}

#[repr(C)]
pub struct local_storage_rcu_tasks_trace_bench_progs {
    pub get_local: *mut bpf_program,
    pub pregp_step: *mut bpf_program,
    pub postgp: *mut bpf_program,
}

#[repr(C)]
pub struct local_storage_rcu_tasks_trace_bench_bss {
    pub gp_hits: c_long,
    pub gp_times: c_long,
    pub unexpected: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench_env {
    pub producer_cnt: c_int,
    pub consumer_cnt: c_int,
    pub quiet: bool,
}

#[repr(C)]
pub struct bench_res {
    pub gp_ct: c_long,
    pub gp_ns: c_long,
    pub stime: c_long,
}

#[repr(C)]
pub struct basic_stats {
    pub mean: c_double,
    pub stddev: c_double,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(c_int, *mut bench_res, c_long)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

#[repr(C)]
struct Args {
    nr_procs: __u32,
    kthread_pid: __u32,
}

static mut args: Args = Args {
    nr_procs: 1000,
    kthread_pid: 0,
};

static OPT_NAME_NR_PROCS: &[u8] = b"nr_procs\0";
static OPT_ARG_NR_PROCS: &[u8] = b"NR_PROCS\0";
static OPT_DOC_NR_PROCS: &[u8] = b"Set number of user processes to spin up\0";
static OPT_NAME_KTHREAD_PID: &[u8] = b"kthread_pid\0";
static OPT_ARG_KTHREAD_PID: &[u8] = b"PID\0";
static OPT_DOC_KTHREAD_PID: &[u8] = b"Pid of rcu_tasks_trace kthread for ticks tracking\0";

static opts: [argp_option; 3] = [
    argp_option {
        name: OPT_NAME_NR_PROCS.as_ptr() as *const c_char,
        key: ARG_NR_PROCS,
        arg: OPT_ARG_NR_PROCS.as_ptr() as *const c_char,
        flags: 0,
        doc: OPT_DOC_NR_PROCS.as_ptr() as *const c_char,
        group: 0,
    },
    argp_option {
        name: OPT_NAME_KTHREAD_PID.as_ptr() as *const c_char,
        key: ARG_KTHREAD_PID,
        arg: OPT_ARG_KTHREAD_PID.as_ptr() as *const c_char,
        flags: 0,
        doc: OPT_DOC_KTHREAD_PID.as_ptr() as *const c_char,
        group: 0,
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
        group: 0,
    },
];

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut env: bench_env;
    static mut errno: c_int;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *const argp_state);
    fn exit(status: c_int) -> !;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn rand() -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn getpid() -> c_int;
    fn fork() -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn getppid() -> c_int;

    fn setup_libbpf();
    fn local_storage_rcu_tasks_trace_bench__open_and_load() -> *mut local_storage_rcu_tasks_trace_bench;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut c_void;
    fn grace_period_latency_basic_stats(res: *mut bench_res, res_cnt: c_int, stat: *mut basic_stats);
    fn grace_period_ticks_basic_stats(res: *mut bench_res, res_cnt: c_int, stat: *mut basic_stats);
    fn atomic_swap(ptr: *mut c_long, val: c_long) -> c_long;
}

unsafe extern "C" fn parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> error_t {
    let ret: c_long;

    match key {
        ARG_NR_PROCS => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > UINT_MAX {
                fprintf(stderr, c"invalid nr_procs\n".as_ptr());
                argp_usage(state);
            }
            args.nr_procs = ret as __u32;
        }
        ARG_KTHREAD_PID => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 {
                fprintf(stderr, c"invalid kthread_pid\n".as_ptr());
                argp_usage(state);
            }
            args.kthread_pid = ret as __u32;
        }
        /* The C source contains an extra unreachable "break;" before default. */
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static bench_local_storage_rcu_tasks_trace_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if env.producer_cnt != 1 {
        fprintf(
            stderr,
            c"benchmark doesn't support multi-producer!\n".as_ptr(),
        );
        exit(1);
    }
    if env.consumer_cnt != 0 {
        fprintf(stderr, c"benchmark doesn't support consumer!\n".as_ptr());
        exit(1);
    }

    if args.nr_procs > MAX_SLEEP_PROCS {
        fprintf(
            stderr,
            c"benchmark supports up to %u sleeper procs!\n".as_ptr(),
            MAX_SLEEP_PROCS,
        );
        exit(1);
    }
}

unsafe extern "C" fn kthread_pid_ticks() -> c_long {
    let mut procfs_path: [c_char; 100] = [0; 100];
    let mut stime: c_long = 0;
    let mut f: *mut FILE;

    if args.kthread_pid == 0 {
        return -1;
    }

    sprintf(
        procfs_path.as_mut_ptr(),
        c"/proc/%u/stat".as_ptr(),
        args.kthread_pid,
    );
    f = fopen(procfs_path.as_ptr(), c"r".as_ptr());
    if f.is_null() {
        fprintf(
            stderr,
            c"couldn't open %s, exiting\n".as_ptr(),
            procfs_path.as_ptr(),
        );
        goto_err_out(f);
    }
    if fscanf(
        f,
        c"%*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %*s %ld".as_ptr(),
        &mut stime as *mut c_long,
    ) != 1
    {
        fprintf(
            stderr,
            c"fscanf of %s failed, exiting\n".as_ptr(),
            procfs_path.as_ptr(),
        );
        goto_err_out(f);
    }
    fclose(f);
    stime
}

unsafe fn goto_err_out(f: *mut FILE) -> ! {
    if !f.is_null() {
        fclose(f);
    }
    exit(1);
}

#[repr(C)]
struct Ctx {
    skel: *mut local_storage_rcu_tasks_trace_bench,
    prev_kthread_stime: c_long,
}

static mut ctx: Ctx = Ctx {
    skel: ptr::null_mut(),
    prev_kthread_stime: 0,
};

unsafe extern "C" fn sleep_and_loop() {
    loop {
        sleep((rand() % 4) as c_uint);
        syscall(__NR_getpgid);
    }
}

unsafe extern "C" fn local_storage_tasks_trace_setup() {
    let mut i: c_int;
    let mut err: c_int;
    let mut forkret: c_int;
    let runner_pid: c_int;

    runner_pid = getpid();

    i = 0;
    while i < args.nr_procs as c_int {
        forkret = fork();
        if forkret < 0 {
            fprintf(
                stderr,
                c"Error forking sleeper proc %u of %u, exiting\n".as_ptr(),
                i,
                args.nr_procs,
            );
            exit(1);
        }

        if forkret == 0 {
            err = prctl(PR_SET_PDEATHSIG, SIGKILL);
            if err < 0 {
                fprintf(
                    stderr,
                    c"prctl failed with err %d, exiting\n".as_ptr(),
                    errno,
                );
                exit(1);
            }

            if getppid() != runner_pid {
                fprintf(
                    stderr,
                    c"Runner died while spinning up procs, exiting\n".as_ptr(),
                );
                exit(1);
            }
            sleep_and_loop();
        }

        i += 1;
    }
    printf(
        c"Spun up %u procs (our pid %d)\n".as_ptr(),
        args.nr_procs,
        runner_pid,
    );

    setup_libbpf();

    ctx.skel = local_storage_rcu_tasks_trace_bench__open_and_load();
    if ctx.skel.is_null() {
        fprintf(stderr, c"Error doing open_and_load, exiting\n".as_ptr());
        exit(1);
    }

    ctx.prev_kthread_stime = kthread_pid_ticks();

    if bpf_program__attach((*ctx.skel).progs.get_local).is_null() {
        fprintf(stderr, c"Error attaching bpf program\n".as_ptr());
        exit(1);
    }

    if bpf_program__attach((*ctx.skel).progs.pregp_step).is_null() {
        fprintf(stderr, c"Error attaching bpf program\n".as_ptr());
        exit(1);
    }

    if bpf_program__attach((*ctx.skel).progs.postgp).is_null() {
        fprintf(stderr, c"Error attaching bpf program\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    let ticks: c_long;

    (*res).gp_ct = atomic_swap(&mut (*(*ctx.skel).bss).gp_hits as *mut c_long, 0);
    (*res).gp_ns = atomic_swap(&mut (*(*ctx.skel).bss).gp_times as *mut c_long, 0);
    ticks = kthread_pid_ticks();
    (*res).stime = ticks - ctx.prev_kthread_stime;
    ctx.prev_kthread_stime = ticks;
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    loop {
        syscall(__NR_getpgid);
    }
}

unsafe extern "C" fn report_progress(iter: c_int, res: *mut bench_res, _delta_ns: c_long) {
    if (*(*ctx.skel).bss).unexpected != 0 {
        fprintf(
            stderr,
            c"Error: Unexpected order of bpf prog calls (postgp after pregp).".as_ptr(),
        );
        fprintf(stderr, c"Data can't be trusted, exiting\n".as_ptr());
        exit(1);
    }

    if env.quiet {
        return;
    }

    printf(
        c"Iter %d\t avg tasks_trace grace period latency\t%lf ns\n".as_ptr(),
        iter,
        (*res).gp_ns as c_double / (*res).gp_ct as c_double,
    );
    printf(
        c"Iter %d\t avg ticks per tasks_trace grace period\t%lf\n".as_ptr(),
        iter,
        (*res).stime as c_double / (*res).gp_ct as c_double,
    );
}

unsafe extern "C" fn report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut gp_stat: basic_stats = basic_stats {
        mean: 0.0,
        stddev: 0.0,
    };

    grace_period_latency_basic_stats(res, res_cnt, &mut gp_stat as *mut basic_stats);
    printf(c"SUMMARY tasks_trace grace period latency".as_ptr());
    printf(
        c"\tavg %.3lf us\tstddev %.3lf us\n".as_ptr(),
        gp_stat.mean,
        gp_stat.stddev,
    );
    grace_period_ticks_basic_stats(res, res_cnt, &mut gp_stat as *mut basic_stats);
    printf(c"SUMMARY ticks per tasks_trace grace period".as_ptr());
    printf(
        c"\tavg %.3lf\tstddev %.3lf\n".as_ptr(),
        gp_stat.mean,
        gp_stat.stddev,
    );
}

/* local-storage-tasks-trace: Benchmark performance of BPF local_storage's use
 * of RCU Tasks-Trace.
 *
 * Stress RCU Tasks Trace by forking many tasks, all of which do no work aside
 * from sleep() loop, and creating/destroying BPF task-local storage on wakeup.
 * The number of forked tasks is configurable.
 *
 * exercising code paths which call call_rcu_tasks_trace while there are many
 * thousands of tasks on the system should result in RCU Tasks-Trace having to
 * do a noticeable amount of work.
 *
 * This should be observable by measuring rcu_tasks_trace_kthread CPU usage
 * after the grace period has ended, or by measuring grace period latency.
 *
 * This benchmark uses both approaches, attaching to rcu_tasks_trace_pregp_step
 * and rcu_tasks_trace_postgp functions to measure grace period latency and
 * using /proc/PID/stat to measure rcu_tasks_trace_kthread kernel ticks
 */
static BENCH_NAME_LOCAL_STORAGE_TASKS_TRACE: &[u8] = b"local-storage-tasks-trace\0";

#[unsafe(no_mangle)]
pub static bench_local_storage_tasks_trace: bench = bench {
    name: BENCH_NAME_LOCAL_STORAGE_TASKS_TRACE.as_ptr() as *const c_char,
    argp: &bench_local_storage_rcu_tasks_trace_argp as *const argp,
    validate: Some(validate),
    setup: Some(local_storage_tasks_trace_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(report_progress),
    report_final: Some(report_final),
};
