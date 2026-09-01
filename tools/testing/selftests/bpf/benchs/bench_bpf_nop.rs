// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "bench.h"
// #include "bench_bpf_timing.h"
// #include "bpf_nop_bench.skel.h"
// #include "bpf_util.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

#[repr(C)]
pub struct ctx {
    skel: *mut bpf_nop_bench,
    timing: bpf_bench_timing,
    prog_fd: c_int,
}

static mut CTX: ctx = ctx {
    skel: ptr::null_mut(),
    timing: unsafe { MaybeUninit::<bpf_bench_timing>::zeroed().assume_init() },
    prog_fd: 0,
};

extern "C" {
    static mut env: env;
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn setup_libbpf();
    fn bpf_nop_bench__open() -> *mut bpf_nop_bench;
    fn bpf_nop_bench__load(skel: *mut bpf_nop_bench) -> c_int;
    fn bpf_nop_bench__destroy(skel: *mut bpf_nop_bench);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_bench_calibrate(
        timing: *mut bpf_bench_timing,
        run_once: Option<unsafe extern "C" fn(*mut c_void)>,
        ctx: *mut c_void,
    );
    fn bpf_bench_timing_measure(timing: *mut bpf_bench_timing, res: *mut bench_res);
    fn bpf_bench_timing_report(
        timing: *mut bpf_bench_timing,
        name: *const c_char,
        ctx: *mut c_void,
    );
}

unsafe extern "C" {
    // External declarations supplied by the translated headers/skeleton.
    pub type FILE;
    pub type bpf_bench_timing;
    pub type bpf_program;
    pub type bench_res;
}

#[repr(C)]
pub struct env {
    pub consumer_cnt: c_int,
    pub duration_sec: c_int,
}

#[repr(C)]
pub struct bpf_nop_bench {
    pub progs: bpf_nop_bench__progs,
}

#[repr(C)]
pub struct bpf_nop_bench__progs {
    pub bench_nop: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    sz: usize,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

unsafe fn bench_timing_init(timing: *mut bpf_bench_timing, skel: *mut bpf_nop_bench, idx: c_int) {
    // Translation of BENCH_TIMING_INIT(&ctx.timing, skel, 0); supplied by bench_bpf_timing.h.
    BENCH_TIMING_INIT(timing, skel, idx);
}

extern "C" {
    fn BENCH_TIMING_INIT(timing: *mut bpf_bench_timing, skel: *mut bpf_nop_bench, idx: c_int);
}

unsafe extern "C" fn nop_validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            b"benchmark doesn't support consumers\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn nop_run_once(_unused: *mut c_void) {
    // Translation of LIBBPF_OPTS(bpf_test_run_opts, topts);
    let mut topts: bpf_test_run_opts = MaybeUninit::<bpf_test_run_opts>::zeroed().assume_init();

    bpf_prog_test_run_opts(CTX.prog_fd, &mut topts);
}

unsafe extern "C" fn nop_setup() {
    let mut skel: *mut bpf_nop_bench;
    let err: c_int;

    setup_libbpf();

    skel = bpf_nop_bench__open();
    if skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    err = bpf_nop_bench__load(skel);
    if err != 0 {
        fprintf(
            stderr,
            b"failed to load skeleton: %s\n\0".as_ptr() as *const c_char,
            strerror(-err),
        );
        bpf_nop_bench__destroy(skel);
        exit(1);
    }

    CTX.skel = skel;
    CTX.prog_fd = bpf_program__fd((*skel).progs.bench_nop);

    bench_timing_init(&mut CTX.timing, skel, 0);
    bpf_bench_calibrate(&mut CTX.timing, Some(nop_run_once), ptr::null_mut());

    env.duration_sec = 600;
}

unsafe extern "C" fn nop_producer(_input: *mut c_void) -> *mut c_void {
    while true {
        nop_run_once(ptr::null_mut());
    }

    ptr::null_mut()
}

unsafe extern "C" fn nop_measure(res: *mut bench_res) {
    bpf_bench_timing_measure(&mut CTX.timing, res);
}

unsafe extern "C" fn nop_report_final(_res: *mut bench_res, _res_cnt: c_int) {
    bpf_bench_timing_report(
        &mut CTX.timing,
        b"bpf-nop\0".as_ptr() as *const c_char,
        ptr::null_mut(),
    );
}

#[no_mangle]
pub static bench_bpf_nop: bench = bench {
    name: b"bpf-nop\0".as_ptr() as *const c_char,
    validate: Some(nop_validate),
    setup: Some(nop_setup),
    producer_thread: Some(nop_producer),
    measure: Some(nop_measure),
    report_final: Some(nop_report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
