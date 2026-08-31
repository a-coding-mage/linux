// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long};

/* Dependencies from <fcntl.h>, "bench.h", and "test_overhead.skel.h". */
const O_WRONLY: c_int = 1;
const O_TRUNC: c_int = 0o1000;

#[repr(C)]
pub struct Counter {
    pub value: c_long,
}

#[repr(C)]
pub struct Env {
    pub producer_cnt: c_int,
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct BenchRes {
    pub hits: c_long,
}

#[repr(C)]
pub struct BpfProgram {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct BpfLink {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct TestOverheadProgs {
    pub prog1: *mut BpfProgram,
    pub prog2: *mut BpfProgram,
    pub prog3: *mut BpfProgram,
    pub prog4: *mut BpfProgram,
    pub prog5: *mut BpfProgram,
}

#[repr(C)]
pub struct TestOverhead {
    pub progs: TestOverheadProgs,
}

#[repr(C)]
pub struct Bench {
    pub name: *const c_char,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut BenchRes)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut BenchRes, *mut BenchRes, *mut c_void)>,
    pub report_final: Option<unsafe extern "C" fn(*mut BenchRes, *mut BenchRes, *mut c_void)>,
}

unsafe extern "C" {
    static mut env: Env;
    static mut stderr: *mut c_void;
    static mut errno: c_int;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;

    fn setup_libbpf();
    fn test_overhead__open_and_load() -> *mut TestOverhead;
    fn bpf_program__attach(prog: *mut BpfProgram) -> *mut BpfLink;

    fn atomic_inc(value: *mut c_long);
    fn atomic_swap(value: *mut c_long, new_value: c_long) -> c_long;

    fn hits_drops_report_progress(
        res: *mut BenchRes,
        prev: *mut BenchRes,
        ctx: *mut c_void,
    );
    fn hits_drops_report_final(
        res: *mut BenchRes,
        prev: *mut BenchRes,
        ctx: *mut c_void,
    );
}

/* BPF triggering benchmarks */
#[repr(C)]
struct Ctx {
    skel: *mut TestOverhead,
    hits: Counter,
    fd: c_int,
}

static mut ctx: Ctx = Ctx {
    skel: std::ptr::null_mut(),
    hits: Counter { value: 0 },
    fd: 0,
};

unsafe extern "C" fn validate() {
    unsafe {
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
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    let buf = *b"test_overhead\0";
    let mut err: isize;

    loop {
        unsafe {
            err = write(ctx.fd, buf.as_ptr() as *const c_void, buf.len());
            if err < 0 {
                fprintf(stderr, c"write failed\n".as_ptr());
                exit(1);
            }
            atomic_inc(&raw mut ctx.hits.value);
        }
    }
}

unsafe extern "C" fn measure(res: *mut BenchRes) {
    unsafe {
        (*res).hits = atomic_swap(&raw mut ctx.hits.value, 0);
    }
}

unsafe extern "C" fn setup_ctx() {
    unsafe {
        setup_libbpf();

        ctx.skel = test_overhead__open_and_load();
        if ctx.skel.is_null() {
            fprintf(stderr, c"failed to open skeleton\n".as_ptr());
            exit(1);
        }

        ctx.fd = open(c"/proc/self/comm".as_ptr(), O_WRONLY | O_TRUNC);
        if ctx.fd < 0 {
            fprintf(
                stderr,
                c"failed to open /proc/self/comm: %d\n".as_ptr(),
                -errno,
            );
            exit(1);
        }
    }
}

unsafe extern "C" fn attach_bpf(prog: *mut BpfProgram) {
    let link: *mut BpfLink;

    unsafe {
        link = bpf_program__attach(prog);
        if link.is_null() {
            fprintf(stderr, c"failed to attach program!\n".as_ptr());
            exit(1);
        }
    }
}

unsafe extern "C" fn setup_base() {
    unsafe {
        setup_ctx();
    }
}

unsafe extern "C" fn setup_kprobe() {
    unsafe {
        setup_ctx();
        attach_bpf((*ctx.skel).progs.prog1);
    }
}

unsafe extern "C" fn setup_kretprobe() {
    unsafe {
        setup_ctx();
        attach_bpf((*ctx.skel).progs.prog2);
    }
}

unsafe extern "C" fn setup_rawtp() {
    unsafe {
        setup_ctx();
        attach_bpf((*ctx.skel).progs.prog3);
    }
}

unsafe extern "C" fn setup_fentry() {
    unsafe {
        setup_ctx();
        attach_bpf((*ctx.skel).progs.prog4);
    }
}

unsafe extern "C" fn setup_fexit() {
    unsafe {
        setup_ctx();
        attach_bpf((*ctx.skel).progs.prog5);
    }
}

#[unsafe(no_mangle)]
pub static bench_rename_base: Bench = Bench {
    name: c"rename-base".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_base),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_rename_kprobe: Bench = Bench {
    name: c"rename-kprobe".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_kprobe),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_rename_kretprobe: Bench = Bench {
    name: c"rename-kretprobe".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_kretprobe),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_rename_rawtp: Bench = Bench {
    name: c"rename-rawtp".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_rawtp),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_rename_fentry: Bench = Bench {
    name: c"rename-fentry".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_fentry),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_rename_fexit: Bench = Bench {
    name: c"rename-fexit".as_ptr(),
    validate: Some(validate),
    setup: Some(setup_fexit),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};
