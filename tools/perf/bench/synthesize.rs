// SPDX-License-Identifier: GPL-2.0
/*
 * Benchmark synthesis of perf events such as at the start of a 'perf
 * record'. Synthesis is done on the current process and the 'dummy' event
 * handlers are invoked that support dump_trace but otherwise do nothing.
 *
 * Copyright 2019 Google LLC.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const UINT_MAX: c_uint = c_uint::MAX;
const USEC_PER_SEC: u64 = 1_000_000;
const ENOMEM: c_int = 12;
const EXIT_FAILURE: c_int = 1;
const _SC_NPROCESSORS_ONLN: c_int = 84;

#[repr(C)]
pub struct r#option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _bindgen_union_align: u64,
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machines {
    host: machine,
}

#[repr(C)]
pub struct perf_session {
    machines: machines,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    pid: *const c_char,
    tid: *const c_char,
    cpu_list: *const c_char,
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_read(v: *const atomic_t) -> c_int;

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;

    fn __machine__synthesize_threads(
        machine: *mut machine,
        tool: *mut perf_tool,
        target: *mut target,
        threads: *mut perf_thread_map,
        process: Option<
            unsafe extern "C" fn(
                *const perf_tool,
                *mut perf_event,
                *mut perf_sample,
                *mut machine,
            ) -> c_int,
        >,
        needs_mmap: bool,
        data_mmap: bool,
        nr_threads_synthesize: c_uint,
    ) -> c_int;

    fn perf_set_singlethreaded();
    fn perf_set_multithreaded();
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn __perf_session__new(
        data: *mut c_void,
        tool: *mut perf_tool,
        trace_event_repipe: bool,
        env: *mut perf_env,
    ) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn thread_map__new_by_pid(pid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn getpid() -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn sysconf(name: c_int) -> c_ulong;
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const r#option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const r#option);
    fn exit(status: c_int) -> !;
}

static mut MIN_THREADS: c_uint = 1;
static mut MAX_THREADS: c_uint = UINT_MAX;
static mut SINGLE_ITERATIONS: c_uint = 10000;
static mut MULTI_ITERATIONS: c_uint = 10;
static mut RUN_ST: bool = false;
static mut RUN_MT: bool = false;

/*
 * Original C initializer, dependent on parse-options.h macros:
 *
 * static const struct option options[] = {
 *      OPT_BOOLEAN('s', "st", &run_st, "Run single threaded benchmark"),
 *      OPT_BOOLEAN('t', "mt", &run_mt, "Run multi-threaded benchmark"),
 *      OPT_UINTEGER('m', "min-threads", &min_threads,
 *              "Minimum number of threads in multithreaded bench"),
 *      OPT_UINTEGER('M', "max-threads", &max_threads,
 *              "Maximum number of threads in multithreaded bench"),
 *      OPT_UINTEGER('i', "single-iterations", &single_iterations,
 *              "Number of iterations used to compute single-threaded average"),
 *      OPT_UINTEGER('I', "multi-iterations", &multi_iterations,
 *              "Number of iterations used to compute multi-threaded average"),
 *      OPT_END()
 * };
 */
static OPTIONS: [r#option; 7] = [
    r#option { _private: [] },
    r#option { _private: [] },
    r#option { _private: [] },
    r#option { _private: [] },
    r#option { _private: [] },
    r#option { _private: [] },
    r#option { _private: [] },
];

static BENCH_USAGE_0: &[u8] = b"perf bench internals synthesize <options>\0";
static BENCH_USAGE: [*const c_char; 2] = [BENCH_USAGE_0.as_ptr() as *const c_char, core::ptr::null()];

static mut EVENT_COUNT: atomic_t = atomic_t { counter: 0 };

unsafe extern "C" fn process_synthesized_event(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    unsafe {
        atomic_inc(core::ptr::addr_of_mut!(EVENT_COUNT));
    }
    0
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    unsafe {
        (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
        (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
        if (*result).tv_usec < 0 {
            (*result).tv_sec -= 1;
            (*result).tv_usec += 1000000;
        }
    }
}

unsafe fn do_run_single_threaded(
    session: *mut perf_session,
    threads: *mut perf_thread_map,
    target: *mut target,
    data_mmap: bool,
) -> c_int {
    let nr_threads_synthesize: c_uint = 1;
    let mut start = timeval { tv_sec: 0, tv_usec: 0 };
    let mut end = timeval { tv_sec: 0, tv_usec: 0 };
    let mut diff = timeval { tv_sec: 0, tv_usec: 0 };
    let mut runtime_us: u64;
    let mut i: c_uint;
    let mut time_average: f64;
    let mut time_stddev: f64;
    let mut event_average: f64;
    let mut event_stddev: f64;
    let mut err: c_int;
    let mut time_stats: stats = stats { _private: [] };
    let mut event_stats: stats = stats { _private: [] };

    unsafe {
        init_stats(&mut time_stats);
        init_stats(&mut event_stats);

        i = 0;
        while i < SINGLE_ITERATIONS {
            atomic_set(core::ptr::addr_of_mut!(EVENT_COUNT), 0);
            gettimeofday(&mut start, core::ptr::null_mut());
            err = __machine__synthesize_threads(
                &mut (*session).machines.host,
                core::ptr::null_mut(),
                target,
                threads,
                Some(process_synthesized_event),
                true,
                data_mmap,
                nr_threads_synthesize,
            );
            if err != 0 {
                return err;
            }

            gettimeofday(&mut end, core::ptr::null_mut());
            timersub(&end, &start, &mut diff);
            runtime_us = (diff.tv_sec as u64)
                .wrapping_mul(USEC_PER_SEC)
                .wrapping_add(diff.tv_usec as u64);
            update_stats(&mut time_stats, runtime_us);
            update_stats(&mut event_stats, atomic_read(core::ptr::addr_of!(EVENT_COUNT)) as u64);

            i = i.wrapping_add(1);
        }

        time_average = avg_stats(&mut time_stats);
        time_stddev = stddev_stats(&mut time_stats);
        printf(
            b"  Average %ssynthesis took: %.3f usec (+- %.3f usec)\n\0".as_ptr() as *const c_char,
            if data_mmap {
                b"data \0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            time_average,
            time_stddev,
        );

        event_average = avg_stats(&mut event_stats);
        event_stddev = stddev_stats(&mut event_stats);
        printf(
            b"  Average num. events: %.3f (+- %.3f)\n\0".as_ptr() as *const c_char,
            event_average,
            event_stddev,
        );

        printf(
            b"  Average time per event %.3f usec\n\0".as_ptr() as *const c_char,
            time_average / event_average,
        );
    }
    0
}

unsafe fn run_single_threaded() -> c_int {
    let mut session: *mut perf_session;
    let mut target = target {
        pid: b"self\0".as_ptr() as *const c_char,
        tid: core::ptr::null(),
        cpu_list: core::ptr::null(),
    };
    let mut threads: *mut perf_thread_map;
    let mut host_env: perf_env = perf_env { _private: [] };
    let mut err: c_int;

    unsafe {
        perf_set_singlethreaded();
        perf_env__init(&mut host_env);
        session = __perf_session__new(
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            false,
            &mut host_env,
        );
        if IS_ERR(session as *const c_void) {
            pr_err(b"Session creation failed.\n\0".as_ptr() as *const c_char);
            perf_env__exit(&mut host_env);
            return PTR_ERR(session as *const c_void);
        }
        threads = thread_map__new_by_pid(getpid());
        if threads.is_null() {
            pr_err(b"Thread map creation failed.\n\0".as_ptr() as *const c_char);
            err = -ENOMEM;
            if !threads.is_null() {
                perf_thread_map__put(threads);
            }
            perf_session__delete(session);
            perf_env__exit(&mut host_env);
            return err;
        }

        puts(
            b"Computing performance of single threaded perf event synthesis by\nsynthesizing events on the perf process itself:\0"
                .as_ptr() as *const c_char,
        );

        err = do_run_single_threaded(session, threads, &mut target, false);
        if err == 0 {
            err = do_run_single_threaded(session, threads, &mut target, true);
        }

        if !threads.is_null() {
            perf_thread_map__put(threads);
        }

        perf_session__delete(session);
        perf_env__exit(&mut host_env);
    }
    err
}

unsafe fn do_run_multi_threaded(
    target: *mut target,
    nr_threads_synthesize: c_uint,
) -> c_int {
    let mut start = timeval { tv_sec: 0, tv_usec: 0 };
    let mut end = timeval { tv_sec: 0, tv_usec: 0 };
    let mut diff = timeval { tv_sec: 0, tv_usec: 0 };
    let mut runtime_us: u64;
    let mut i: c_uint;
    let mut time_average: f64;
    let mut time_stddev: f64;
    let mut event_average: f64;
    let mut event_stddev: f64;
    let mut err: c_int = 0;
    let mut time_stats: stats = stats { _private: [] };
    let mut event_stats: stats = stats { _private: [] };
    let mut session: *mut perf_session;
    let mut host_env: perf_env = perf_env { _private: [] };

    unsafe {
        perf_env__init(&mut host_env);
        init_stats(&mut time_stats);
        init_stats(&mut event_stats);
        i = 0;
        while i < MULTI_ITERATIONS {
            session = __perf_session__new(
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                false,
                &mut host_env,
            );
            if IS_ERR(session as *const c_void) {
                err = PTR_ERR(session as *const c_void);
                perf_env__exit(&mut host_env);
                return err;
            }
            atomic_set(core::ptr::addr_of_mut!(EVENT_COUNT), 0);
            gettimeofday(&mut start, core::ptr::null_mut());
            err = __machine__synthesize_threads(
                &mut (*session).machines.host,
                core::ptr::null_mut(),
                target,
                core::ptr::null_mut(),
                Some(process_synthesized_event),
                true,
                false,
                nr_threads_synthesize,
            );
            if err != 0 {
                perf_session__delete(session);
                perf_env__exit(&mut host_env);
                return err;
            }

            gettimeofday(&mut end, core::ptr::null_mut());
            timersub(&end, &start, &mut diff);
            runtime_us = (diff.tv_sec as u64)
                .wrapping_mul(USEC_PER_SEC)
                .wrapping_add(diff.tv_usec as u64);
            update_stats(&mut time_stats, runtime_us);
            update_stats(&mut event_stats, atomic_read(core::ptr::addr_of!(EVENT_COUNT)) as u64);
            perf_session__delete(session);

            i = i.wrapping_add(1);
        }

        time_average = avg_stats(&mut time_stats);
        time_stddev = stddev_stats(&mut time_stats);
        printf(
            b"    Average synthesis took: %.3f usec (+- %.3f usec)\n\0".as_ptr() as *const c_char,
            time_average,
            time_stddev,
        );

        event_average = avg_stats(&mut event_stats);
        event_stddev = stddev_stats(&mut event_stats);
        printf(
            b"    Average num. events: %.3f (+- %.3f)\n\0".as_ptr() as *const c_char,
            event_average,
            event_stddev,
        );

        printf(
            b"    Average time per event %.3f usec\n\0".as_ptr() as *const c_char,
            time_average / event_average,
        );

        perf_env__exit(&mut host_env);
    }
    err
}

unsafe fn run_multi_threaded() -> c_int {
    let mut target = target {
        pid: core::ptr::null(),
        tid: core::ptr::null(),
        cpu_list: b"0\0".as_ptr() as *const c_char,
    };
    let mut nr_threads_synthesize: c_uint;
    let mut err: c_int;

    unsafe {
        if MAX_THREADS == UINT_MAX {
            MAX_THREADS = sysconf(_SC_NPROCESSORS_ONLN) as c_uint;
        }

        puts(
            b"Computing performance of multi threaded perf event synthesis by\nsynthesizing events on CPU 0:\0"
                .as_ptr() as *const c_char,
        );

        nr_threads_synthesize = MIN_THREADS;
        while nr_threads_synthesize <= MAX_THREADS {
            if nr_threads_synthesize == 1 {
                perf_set_singlethreaded();
            } else {
                perf_set_multithreaded();
            }

            printf(
                b"  Number of synthesis threads: %u\n\0".as_ptr() as *const c_char,
                nr_threads_synthesize,
            );

            err = do_run_multi_threaded(&mut target, nr_threads_synthesize);
            if err != 0 {
                return err;
            }

            nr_threads_synthesize = nr_threads_synthesize.wrapping_add(1);
        }
        perf_set_singlethreaded();
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_synthesize(argc: c_int, argv: *mut *const c_char) -> c_int {
    let mut argc = argc;
    let mut err: c_int = 0;

    unsafe {
        argc = parse_options(argc, argv, OPTIONS.as_ptr(), BENCH_USAGE.as_ptr(), 0);
        if argc != 0 {
            usage_with_options(BENCH_USAGE.as_ptr(), OPTIONS.as_ptr());
            exit(EXIT_FAILURE);
        }

        /*
         * If neither single threaded or multi-threaded are specified, default
         * to running just single threaded.
         */
        if !RUN_ST && !RUN_MT {
            RUN_ST = true;
        }

        if RUN_ST {
            err = run_single_threaded();
        }

        if err == 0 && RUN_MT {
            err = run_multi_threaded();
        }
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
