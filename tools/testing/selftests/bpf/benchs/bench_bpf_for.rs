// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies: <argp.h>, "bench.h", "bpf_for_bench.skel.h"

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type error_t = c_int;
type __u32 = u32;

const ARG_NR_LOOPS: c_int = 4000;
const ARGP_ERR_UNKNOWN: error_t = -7;
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
pub struct bench_res {
    pub hits: u64,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn()>,
    pub report_final: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct env {
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct bpf_for_bench_bss {
    pub hits: u64,
    pub nr_loops: __u32,
}

#[repr(C)]
pub struct bpf_for_bench_progs {
    pub benchmark: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_for_bench {
    pub bss: *mut bpf_for_bench_bss,
    pub progs: bpf_for_bench_progs,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct ctx {
    skel: *mut bpf_for_bench,
}

#[repr(C)]
struct args {
    nr_loops: __u32,
}

unsafe extern "C" {
    static mut env: env;
    static mut stderr: *mut c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;
    fn atomic_swap(ptr: *mut u64, val: u64) -> u64;

    fn setup_libbpf();
    fn bpf_for_bench__open_and_load() -> *mut bpf_for_bench;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;

    fn ops_report_progress();
    fn ops_report_final();
}

/* BPF triggering benchmarks */
static mut ctx: ctx = ctx {
    skel: ptr::null_mut(),
};

static mut args: args = args {
    /*
     * Default to a large loop count so the per-iteration bpf_iter_num_next() cost dominates
     * the one-time bpf_iter_num_new()/destroy() setup and teardown.
     */
    nr_loops: 1000,
};

static OPTS: [argp_option; 2] = [
    argp_option {
        name: c"nr_loops".as_ptr(),
        key: ARG_NR_LOOPS,
        arg: c"nr_loops".as_ptr(),
        flags: 0,
        doc: c"Set number of iterations for the bpf_for() loop".as_ptr(),
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

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, _state: *mut argp_state) -> error_t {
    match key {
        ARG_NR_LOOPS => {
            args.nr_loops = strtol(arg, ptr::null_mut(), 10) as __u32;
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

/* exported into benchmark runner */
#[unsafe(no_mangle)]
pub static bench_bpf_for_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            c"benchmark doesn't support consumer!\n".as_ptr(),
        );
        exit(1);
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    while true {
        /* trigger the bpf program */
        syscall(__NR_getpgid);
    }

    ptr::null_mut()
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*ctx.skel).bss).hits, 0);
}

unsafe extern "C" fn setup() {
    let link: *mut bpf_link;

    setup_libbpf();

    ctx.skel = bpf_for_bench__open_and_load();
    if ctx.skel.is_null() {
        fprintf(stderr, c"failed to open skeleton\n".as_ptr());
        exit(1);
    }

    link = bpf_program__attach((*ctx.skel).progs.benchmark);
    if link.is_null() {
        fprintf(stderr, c"failed to attach program!\n".as_ptr());
        exit(1);
    }

    (*(*ctx.skel).bss).nr_loops = args.nr_loops;
}

#[unsafe(no_mangle)]
pub static bench_bpf_for: bench = bench {
    name: c"bpf-for".as_ptr(),
    argp: &bench_bpf_for_argp,
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(ops_report_progress),
    report_final: Some(ops_report_final),
};
