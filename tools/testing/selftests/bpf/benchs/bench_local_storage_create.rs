// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Translated from C. Dependencies from <sys/types.h>, <sys/socket.h>,
 * <pthread.h>, <argp.h>, "bench.h", and
 * "bench_local_storage_create.skel.h" are declared as external items.
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

type pthread_t = usize;
type error_t = c_int;

const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const BPF_MAP_TYPE_SK_STORAGE: c_int = 24;
const BPF_MAP_TYPE_TASK_STORAGE: c_int = 29;
const ARGP_ERR_UNKNOWN: error_t = 7;

#[repr(C)]
pub struct thread {
    fds: *mut c_int,
    pthds: *mut pthread_t,
    pthd_results: *mut c_int,
}

#[repr(C)]
pub struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
    group: c_int,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
pub struct bench_res {
    hits: c_long,
}

#[repr(C)]
pub struct bench {
    name: *const c_char,
    argp: *const argp,
    validate: Option<unsafe extern "C" fn()>,
    setup: Option<unsafe extern "C" fn()>,
    producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    report_progress: Option<unsafe extern "C" fn(c_int, *mut bench_res, c_long)>,
    report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

#[repr(C)]
pub struct bench_env {
    consumer_cnt: c_int,
    producer_cnt: c_int,
}

#[repr(C)]
pub struct bench_local_storage_create_bss {
    bench_pid: c_int,
    create_cnts: c_long,
    create_errs: c_long,
}

#[repr(C)]
pub struct bench_local_storage_create_progs {
    socket_post_create: *mut c_void,
    sched_process_fork: *mut c_void,
}

#[repr(C)]
pub struct bench_local_storage_create {
    bss: *mut bench_local_storage_create_bss,
    progs: bench_local_storage_create_progs,
}

unsafe extern "C" {
    static mut env: bench_env;
    static mut stderr: *mut c_void;

    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn getpid() -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sqrt(x: c_double) -> c_double;

    fn bench_local_storage_create__open_and_load() -> *mut bench_local_storage_create;
    fn bpf_program__attach(prog: *mut c_void) -> *mut c_void;
    fn atomic_swap(ptr: *mut c_long, val: c_long) -> c_long;
    fn atomic_inc(ptr: *mut c_long);
}

static mut skel: *mut bench_local_storage_create = ptr::null_mut();
static mut threads: *mut thread = ptr::null_mut();
static mut create_owner_errs: c_long = 0;
static mut storage_type: c_int = BPF_MAP_TYPE_SK_STORAGE;
static mut batch_sz: c_int = 32;

const ARG_BATCH_SZ: c_int = 9000;
const ARG_STORAGE_TYPE: c_int = 9001;

static opts: [argp_option; 3] = [
    argp_option {
        name: c"batch-size".as_ptr(),
        key: ARG_BATCH_SZ,
        arg: c"BATCH_SIZE".as_ptr(),
        flags: 0,
        doc: c"The number of storage creations in each batch".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"storage-type".as_ptr(),
        key: ARG_STORAGE_TYPE,
        arg: c"STORAGE_TYPE".as_ptr(),
        flags: 0,
        doc: c"The type of local storage to test (socket or task)".as_ptr(),
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

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    let ret: c_int;

    match key {
        ARG_BATCH_SZ => {
            ret = atoi(arg);
            if ret < 1 {
                fprintf(stderr, c"invalid batch-size\n".as_ptr());
                argp_usage(state);
            }
            batch_sz = ret;
        }
        ARG_STORAGE_TYPE => {
            if strcmp(arg, c"task".as_ptr()) == 0 {
                storage_type = BPF_MAP_TYPE_TASK_STORAGE;
            } else if strcmp(arg, c"socket".as_ptr()) == 0 {
                storage_type = BPF_MAP_TYPE_SK_STORAGE;
            } else {
                fprintf(stderr, c"invalid storage-type (socket or task)\n".as_ptr());
                argp_usage(state);
            }
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static bench_local_storage_create_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            c"local-storage-create benchmark does not need consumer\n".as_ptr(),
        );
        exit(1);
    }
}

unsafe extern "C" fn setup() {
    let mut i: c_int;

    skel = bench_local_storage_create__open_and_load();
    if skel.is_null() {
        fprintf(stderr, c"error loading skel\n".as_ptr());
        exit(1);
    }

    (*(*skel).bss).bench_pid = getpid();
    if storage_type == BPF_MAP_TYPE_SK_STORAGE {
        if bpf_program__attach((*skel).progs.socket_post_create).is_null() {
            fprintf(stderr, c"Error attaching bpf program\n".as_ptr());
            exit(1);
        }
    } else if bpf_program__attach((*skel).progs.sched_process_fork).is_null() {
        fprintf(stderr, c"Error attaching bpf program\n".as_ptr());
        exit(1);
    }

    threads = calloc(env.producer_cnt as usize, size_of::<thread>()) as *mut thread;

    if threads.is_null() {
        fprintf(stderr, c"cannot alloc thread_res\n".as_ptr());
        exit(1);
    }

    i = 0;
    while i < env.producer_cnt {
        let t: *mut thread = threads.add(i as usize);

        if storage_type == BPF_MAP_TYPE_SK_STORAGE {
            (*t).fds = malloc((batch_sz as usize) * size_of::<c_int>()) as *mut c_int;
            if (*t).fds.is_null() {
                fprintf(stderr, c"cannot alloc t->fds\n".as_ptr());
                exit(1);
            }
        } else {
            (*t).pthds = malloc((batch_sz as usize) * size_of::<pthread_t>()) as *mut pthread_t;
            if (*t).pthds.is_null() {
                fprintf(stderr, c"cannot alloc t->pthds\n".as_ptr());
                exit(1);
            }
            (*t).pthd_results =
                malloc((batch_sz as usize) * size_of::<c_int>()) as *mut c_int;
            if (*t).pthd_results.is_null() {
                fprintf(stderr, c"cannot alloc t->pthd_results\n".as_ptr());
                exit(1);
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*skel).bss).create_cnts, 0);
}

unsafe extern "C" fn sk_producer(input: *mut c_void) -> *mut c_void {
    let t: *mut thread = threads.add(input as c_long as usize);
    let fds: *mut c_int = (*t).fds;
    let mut i: c_int;

    loop {
        i = 0;
        while i < batch_sz {
            *fds.add(i as usize) = socket(AF_INET6, SOCK_DGRAM, 0);
            if *fds.add(i as usize) == -1 {
                atomic_inc(&mut create_owner_errs);
            }
            i += 1;
        }

        i = 0;
        while i < batch_sz {
            if *fds.add(i as usize) != -1 {
                close(*fds.add(i as usize));
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

unsafe extern "C" fn task_producer(input: *mut c_void) -> *mut c_void {
    let t: *mut thread = threads.add(input as c_long as usize);
    let pthds: *mut pthread_t = (*t).pthds;
    let pthd_results: *mut c_int = (*t).pthd_results;
    let mut i: c_int;

    loop {
        i = 0;
        while i < batch_sz {
            *pthd_results.add(i as usize) = pthread_create(
                pthds.add(i as usize),
                ptr::null(),
                thread_func,
                ptr::null_mut(),
            );
            if *pthd_results.add(i as usize) != 0 {
                atomic_inc(&mut create_owner_errs);
            }
            i += 1;
        }

        i = 0;
        while i < batch_sz {
            if *pthd_results.add(i as usize) == 0 {
                pthread_join(*pthds.add(i as usize), ptr::null_mut());
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn producer(input: *mut c_void) -> *mut c_void {
    if storage_type == BPF_MAP_TYPE_SK_STORAGE {
        sk_producer(input)
    } else {
        task_producer(input)
    }
}

unsafe extern "C" fn report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    let creates_per_sec: c_double;

    creates_per_sec = (*res).hits as c_double / 1000.0 / (delta_ns as c_double / 1000000000.0);

    printf(
        c"Iter %3d (%7.3lfus): ".as_ptr(),
        iter,
        (delta_ns - 1000000000) as c_double / 1000.0,
    );
    printf(
        c"creates %8.3lfk/s (%7.3lfk/prod)\n".as_ptr(),
        creates_per_sec,
        creates_per_sec / env.producer_cnt as c_double,
    );
}

unsafe extern "C" fn report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut creates_mean: c_double = 0.0;
    let mut creates_stddev: c_double = 0.0;
    let mut total_creates: c_long = 0;
    let mut i: c_int;

    i = 0;
    while i < res_cnt {
        creates_mean += (*res.add(i as usize)).hits as c_double / 1000.0 / (0.0 + res_cnt as c_double);
        total_creates += (*res.add(i as usize)).hits;
        i += 1;
    }

    if res_cnt > 1 {
        i = 0;
        while i < res_cnt {
            creates_stddev += (creates_mean - (*res.add(i as usize)).hits as c_double / 1000.0)
                * (creates_mean - (*res.add(i as usize)).hits as c_double / 1000.0)
                / (res_cnt as c_double - 1.0);
            i += 1;
        }
        creates_stddev = sqrt(creates_stddev);
    }
    printf(
        "Summary: creates %8.3lf ± %5.3lfk/s (%7.3lfk/prod), %ld total\n\0".as_ptr()
            as *const c_char,
        creates_mean,
        creates_stddev,
        creates_mean / env.producer_cnt as c_double,
        total_creates,
    );
    if create_owner_errs != 0 || (*(*skel).bss).create_errs != 0 {
        printf(
            c"%s() errors %ld create_errs %ld\n".as_ptr(),
            if storage_type == BPF_MAP_TYPE_SK_STORAGE {
                c"socket".as_ptr()
            } else {
                c"pthread_create".as_ptr()
            },
            create_owner_errs,
            (*(*skel).bss).create_errs,
        );
    }
}

/* Benchmark performance of creating bpf local storage  */
#[unsafe(no_mangle)]
pub static bench_local_storage_create: bench = bench {
    name: c"local-storage-create".as_ptr(),
    argp: &bench_local_storage_create_argp,
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(report_progress),
    report_final: Some(report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
