// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */
// C dependencies: argp.h, pthread.h, sys/types.h, sys/stat.h, sys/param.h,
// fcntl.h, bench.h, bpf_util.h, cgroup_helpers.h, htab_mem_bench.skel.h

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

type ErrorT = c_int;
type SsizeT = isize;
type U32 = u32;

const ARG_VALUE_SIZE: c_int = 10000;
const ARG_USE_CASE: c_int = 10001;
const ARG_PREALLOCATED: c_int = 10002;

const ARGP_ERR_UNKNOWN: ErrorT = -7;
const O_RDONLY: c_int = 0;
const BPF_F_NO_PREALLOC: c_uint = 1;
const BPF_MAP_TYPE_HASH: BpfMapType = 1;
const BPF_MAP_TYPE_RHASH: BpfMapType = 30;
const __NR_GETPGID: c_long = 121;
const __NR_GETPPID: c_long = 110;

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
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> ErrorT>,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_barrier_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

type BpfMapType = c_uint;

#[repr(C)]
pub struct htab_mem_bench_maps {
    pub htab: *mut bpf_map,
}

#[repr(C)]
pub struct htab_mem_bench_bss {
    pub nr_thread: c_int,
    pub op_cnt: c_ulong,
}

#[repr(C)]
pub struct htab_mem_bench {
    pub obj: *mut bpf_object,
    pub maps: htab_mem_bench_maps,
    pub bss: *mut htab_mem_bench_bss,
}

#[repr(C)]
pub struct bench_res {
    pub hits: c_double,
    pub drops: c_double,
    pub false_hits: c_double,
    pub important_hits: c_double,
    pub gp_ct: c_ulong,
}

#[repr(C)]
pub struct bench_env {
    pub producer_cnt: c_int,
    pub nr_cpus: c_int,
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
struct htab_mem_use_case {
    name: *const c_char,
    progs: *const *const c_char,
    /* Do synchronization between addition thread and deletion thread */
    need_sync: bool,
}

#[repr(C)]
struct htab_mem_ctx {
    uc: *const htab_mem_use_case,
    skel: *mut htab_mem_bench,
    notify: *mut pthread_barrier_t,
    fd: c_int,
}

#[repr(C)]
struct htab_mem_args {
    value_size: U32,
    use_case: *const c_char,
    preallocated: bool,
}

unsafe extern "C" {
    static mut env: bench_env;
    static mut stderr: *mut c_void;

    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn argp_usage(state: *mut argp_state);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn setup_libbpf();
    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
    fn htab_mem_bench__open() -> *mut htab_mem_bench;
    fn htab_mem_bench__load(skel: *mut htab_mem_bench) -> c_int;
    fn htab_mem_bench__attach(skel: *mut htab_mem_bench) -> c_int;
    fn htab_mem_bench__destroy(skel: *mut htab_mem_bench);
    fn bpf_map__set_type(map: *mut bpf_map, type_: BpfMapType) -> c_int;
    fn bpf_map__set_value_size(map: *mut bpf_map, size: U32) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: U32) -> c_int;
    fn bpf_map__set_map_flags(map: *mut bpf_map, flags: c_uint) -> c_int;
    fn bpf_map__map_flags(map: *const bpf_map) -> c_uint;
    fn bpf_object__find_program_by_name(
        obj: *const bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
    fn atomic_swap(ptr: *mut c_ulong, val: c_ulong) -> c_ulong;
    fn sqrt(x: c_double) -> c_double;
}

static mut CTX: htab_mem_ctx = htab_mem_ctx {
    uc: core::ptr::null(),
    skel: core::ptr::null_mut(),
    notify: core::ptr::null_mut(),
    fd: 0,
};

static OW_PROGS: [*const c_char; 2] = [c"overwrite".as_ptr(), core::ptr::null()];
static BATCH_PROGS: [*const c_char; 2] = [c"batch_add_batch_del".as_ptr(), core::ptr::null()];
static ADD_DEL_PROGS: [*const c_char; 3] = [
    c"add_only".as_ptr(),
    c"del_only".as_ptr(),
    core::ptr::null(),
];
static USE_CASES: [htab_mem_use_case; 3] = [
    htab_mem_use_case {
        name: c"overwrite".as_ptr(),
        progs: OW_PROGS.as_ptr(),
        need_sync: false,
    },
    htab_mem_use_case {
        name: c"batch_add_batch_del".as_ptr(),
        progs: BATCH_PROGS.as_ptr(),
        need_sync: false,
    },
    htab_mem_use_case {
        name: c"add_del_on_diff_cpu".as_ptr(),
        progs: ADD_DEL_PROGS.as_ptr(),
        need_sync: true,
    },
];

static mut ARGS: htab_mem_args = htab_mem_args {
    value_size: 8,
    use_case: c"overwrite".as_ptr(),
    preallocated: false,
};

static OPTS: [argp_option; 4] = [
    argp_option {
        name: c"value-size".as_ptr(),
        key: ARG_VALUE_SIZE,
        arg: c"VALUE_SIZE".as_ptr(),
        flags: 0,
        doc: c"Set the value size of hash map (default 8)".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"use-case".as_ptr(),
        key: ARG_USE_CASE,
        arg: c"USE_CASE".as_ptr(),
        flags: 0,
        doc: c"Set the use case of hash map: overwrite|batch_add_batch_del|add_del_on_diff_cpu".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"preallocated".as_ptr(),
        key: ARG_PREALLOCATED,
        arg: core::ptr::null(),
        flags: 0,
        doc: c"use preallocated hash map".as_ptr(),
        group: 0,
    },
    argp_option {
        name: core::ptr::null(),
        key: 0,
        arg: core::ptr::null(),
        flags: 0,
        doc: core::ptr::null(),
        group: 0,
    },
];

unsafe extern "C" fn htab_mem_parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> ErrorT {
    match key {
        ARG_VALUE_SIZE => {
            ARGS.value_size = strtoul(arg, core::ptr::null_mut(), 10) as U32;
            if ARGS.value_size > 4096 {
                fprintf(stderr, c"too big value size %u\n".as_ptr(), ARGS.value_size);
                argp_usage(state);
            }
        }
        ARG_USE_CASE => {
            ARGS.use_case = strdup(arg);
            if ARGS.use_case.is_null() {
                fprintf(stderr, c"no mem for use-case\n".as_ptr());
                argp_usage(state);
            }
        }
        ARG_PREALLOCATED => {
            ARGS.preallocated = true;
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

#[no_mangle]
pub static bench_htab_mem_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(htab_mem_parse_arg),
};

unsafe extern "C" fn htab_mem_validate() {
    if strcmp(USE_CASES[2].name, ARGS.use_case) == 0 && env.producer_cnt % 2 != 0 {
        fprintf(
            stderr,
            c"%s needs an even number of producers\n".as_ptr(),
            ARGS.use_case,
        );
        exit(1);
    }
}

unsafe fn htab_mem_bench_init_barriers() -> c_int {
    let barriers: *mut pthread_barrier_t;
    let mut i: c_uint;
    let nr: c_uint;

    if !(*CTX.uc).need_sync {
        return 0;
    }

    nr = ((env.producer_cnt + 1) / 2) as c_uint;
    barriers = calloc(nr as usize, core::mem::size_of::<pthread_barrier_t>())
        as *mut pthread_barrier_t;
    if barriers.is_null() {
        return -1;
    }

    /* Used for synchronization between two threads */
    i = 0;
    while i < nr {
        pthread_barrier_init(barriers.add(i as usize), core::ptr::null(), 2);
        i += 1;
    }

    CTX.notify = barriers;
    0
}

unsafe fn htab_mem_bench_exit_barriers() {
    let mut i: c_uint;
    let nr: c_uint;

    if CTX.notify.is_null() {
        return;
    }

    nr = ((env.producer_cnt + 1) / 2) as c_uint;
    i = 0;
    while i < nr {
        pthread_barrier_destroy(CTX.notify.add(i as usize));
        i += 1;
    }
    free(CTX.notify as *mut c_void);
}

unsafe fn htab_mem_find_use_case_or_exit(name: *const c_char) -> *const htab_mem_use_case {
    let mut i: c_uint;

    i = 0;
    while (i as usize) < USE_CASES.len() {
        if strcmp(name, USE_CASES[i as usize].name) == 0 {
            return &USE_CASES[i as usize];
        }
        i += 1;
    }

    fprintf(stderr, c"no such use-case: %s\n".as_ptr(), name);
    fprintf(stderr, c"available use case:".as_ptr());
    i = 0;
    while (i as usize) < USE_CASES.len() {
        fprintf(stderr, c" %s".as_ptr(), USE_CASES[i as usize].name);
        i += 1;
    }
    fprintf(stderr, c"\n".as_ptr());
    exit(1);
}

unsafe fn htab_mem_setup_impl(map_type: BpfMapType) {
    let map: *mut bpf_map;
    let mut names: *const *const c_char;
    let mut err: c_int;

    setup_libbpf();

    CTX.uc = htab_mem_find_use_case_or_exit(ARGS.use_case);
    err = htab_mem_bench_init_barriers();
    if err != 0 {
        fprintf(stderr, c"failed to init barrier\n".as_ptr());
        exit(1);
    }

    CTX.fd = cgroup_setup_and_join(c"/htab_mem".as_ptr());
    if CTX.fd < 0 {
        htab_mem_setup_impl_cleanup();
    }

    CTX.skel = htab_mem_bench__open();
    if CTX.skel.is_null() {
        fprintf(stderr, c"failed to open skeleton\n".as_ptr());
        htab_mem_setup_impl_cleanup();
    }

    map = (*CTX.skel).maps.htab;
    bpf_map__set_type(map, map_type);
    bpf_map__set_value_size(map, ARGS.value_size);
    /* Ensure that different CPUs can operate on different subset */
    bpf_map__set_max_entries(map, core::cmp::max(8192, 64 * env.nr_cpus) as U32);
    if map_type != BPF_MAP_TYPE_RHASH && ARGS.preallocated {
        bpf_map__set_map_flags(map, bpf_map__map_flags(map) & !BPF_F_NO_PREALLOC);
    }

    names = (*CTX.uc).progs;
    while !(*names).is_null() {
        let prog: *mut bpf_program;

        prog = bpf_object__find_program_by_name((*CTX.skel).obj, *names);
        if prog.is_null() {
            fprintf(stderr, c"no such program %s\n".as_ptr(), *names);
            htab_mem_setup_impl_cleanup();
        }
        bpf_program__set_autoload(prog, true);
        names = names.add(1);
    }
    (*(*CTX.skel).bss).nr_thread = env.producer_cnt;

    err = htab_mem_bench__load(CTX.skel);
    if err != 0 {
        fprintf(stderr, c"failed to load skeleton\n".as_ptr());
        htab_mem_setup_impl_cleanup();
    }
    err = htab_mem_bench__attach(CTX.skel);
    if err != 0 {
        fprintf(stderr, c"failed to attach skeleton\n".as_ptr());
        htab_mem_setup_impl_cleanup();
    }
}

unsafe fn htab_mem_setup_impl_cleanup() -> ! {
    htab_mem_bench__destroy(CTX.skel);
    htab_mem_bench_exit_barriers();
    if CTX.fd >= 0 {
        close(CTX.fd);
        cleanup_cgroup_environment();
    }
    exit(1);
}

unsafe extern "C" fn htab_mem_setup() {
    htab_mem_setup_impl(BPF_MAP_TYPE_HASH);
}

unsafe extern "C" fn rhtab_mem_setup() {
    htab_mem_setup_impl(BPF_MAP_TYPE_RHASH);
}

unsafe fn htab_mem_add_fn(notify: *mut pthread_barrier_t) {
    loop {
        /* Do addition */
        syscall(__NR_GETPGID, 0);
        /* Notify deletion thread to do deletion */
        pthread_barrier_wait(notify);
        /* Wait for deletion to complete */
        pthread_barrier_wait(notify);
    }
}

unsafe fn htab_mem_delete_fn(notify: *mut pthread_barrier_t) {
    loop {
        /* Wait for addition to complete */
        pthread_barrier_wait(notify);
        /* Do deletion */
        syscall(__NR_GETPPID);
        /* Notify addition thread to do addition */
        pthread_barrier_wait(notify);
    }
}

unsafe extern "C" fn htab_mem_producer(arg: *mut c_void) -> *mut c_void {
    let notify: *mut pthread_barrier_t;
    let seq: c_int;

    if !(*CTX.uc).need_sync {
        loop {
            syscall(__NR_GETPGID, 0);
        }
    }

    seq = arg as c_long as c_int;
    notify = CTX.notify.add((seq / 2) as usize);
    if seq & 1 != 0 {
        htab_mem_delete_fn(notify);
    } else {
        htab_mem_add_fn(notify);
    }
    core::ptr::null_mut()
}

unsafe fn htab_mem_read_mem_cgrp_file(name: *const c_char, value: *mut c_ulong) {
    let mut buf = [0 as c_char; 32];
    let got: SsizeT;
    let fd: c_int;

    fd = openat(CTX.fd, name, O_RDONLY);
    if fd < 0 {
        /* cgroup v1 ? */
        fprintf(stderr, c"no %s\n".as_ptr(), name);
        *value = 0;
        return;
    }

    got = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
    close(fd);
    if got <= 0 {
        *value = 0;
        return;
    }
    buf[got as usize] = 0;

    *value = strtoull(buf.as_ptr(), core::ptr::null_mut(), 0);
}

unsafe extern "C" fn htab_mem_measure(res: *mut bench_res) {
    (*res).hits = (atomic_swap(&mut (*(*CTX.skel).bss).op_cnt, 0) / env.producer_cnt as c_ulong)
        as c_double;
    htab_mem_read_mem_cgrp_file(c"memory.current".as_ptr(), &mut (*res).gp_ct);
}

unsafe extern "C" fn htab_mem_report_progress(
    iter: c_int,
    res: *mut bench_res,
    delta_ns: c_long,
) {
    let loop_: c_double;
    let mem: c_double;

    loop_ = (*res).hits / 1000.0 / (delta_ns as c_double / 1000000000.0);
    mem = (*res).gp_ct as c_double / 1048576.0;
    printf(
        c"Iter %3d (%7.3lfus): ".as_ptr(),
        iter,
        (delta_ns - 1000000000) as c_double / 1000.0,
    );
    printf(
        c"per-prod-op %7.2lfk/s, memory usage %7.2lfMiB\n".as_ptr(),
        loop_,
        mem,
    );
}

unsafe extern "C" fn htab_mem_report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut mem_mean: c_double = 0.0;
    let mut mem_stddev: c_double = 0.0;
    let mut loop_mean: c_double = 0.0;
    let mut loop_stddev: c_double = 0.0;
    let mut peak_mem: c_ulong = 0;
    let mut i: c_int;

    i = 0;
    while i < res_cnt {
        loop_mean += (*res.add(i as usize)).hits / 1000.0 / (0.0 + res_cnt as c_double);
        mem_mean +=
            (*res.add(i as usize)).gp_ct as c_double / 1048576.0 / (0.0 + res_cnt as c_double);
        i += 1;
    }
    if res_cnt > 1 {
        i = 0;
        while i < res_cnt {
            loop_stddev += (loop_mean - (*res.add(i as usize)).hits / 1000.0)
                * (loop_mean - (*res.add(i as usize)).hits / 1000.0)
                / (res_cnt as c_double - 1.0);
            mem_stddev += (mem_mean - (*res.add(i as usize)).gp_ct as c_double / 1048576.0)
                * (mem_mean - (*res.add(i as usize)).gp_ct as c_double / 1048576.0)
                / (res_cnt as c_double - 1.0);
            i += 1;
        }
        loop_stddev = sqrt(loop_stddev);
        mem_stddev = sqrt(mem_stddev);
    }

    htab_mem_read_mem_cgrp_file(c"memory.peak".as_ptr(), &mut peak_mem);
    printf(
        c"Summary: per-prod-op %7.2lf \u{00B1} %7.2lfk/s, memory usage %7.2lf \u{00B1} %7.2lfMiB, peak memory usage %7.2lfMiB\n".as_ptr(),
        loop_mean,
        loop_stddev,
        mem_mean,
        mem_stddev,
        peak_mem as c_double / 1048576.0,
    );

    close(CTX.fd);
    cleanup_cgroup_environment();
}

unsafe extern "C" fn rhtab_mem_validate() {
    if ARGS.preallocated {
        fprintf(
            stderr,
            c"rhash map does not support preallocation\n".as_ptr(),
        );
        exit(1);
    }
    htab_mem_validate();
}

#[no_mangle]
pub static bench_htab_mem: bench = bench {
    name: c"htab-mem".as_ptr(),
    argp: &bench_htab_mem_argp,
    validate: Some(htab_mem_validate),
    setup: Some(htab_mem_setup),
    producer_thread: Some(htab_mem_producer),
    measure: Some(htab_mem_measure),
    report_progress: Some(htab_mem_report_progress),
    report_final: Some(htab_mem_report_final),
};

#[no_mangle]
pub static bench_rhtab_mem: bench = bench {
    name: c"rhtab-mem".as_ptr(),
    argp: &bench_htab_mem_argp,
    validate: Some(rhtab_mem_validate),
    setup: Some(rhtab_mem_setup),
    producer_thread: Some(htab_mem_producer),
    measure: Some(htab_mem_measure),
    report_progress: Some(htab_mem_report_progress),
    report_final: Some(htab_mem_report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
