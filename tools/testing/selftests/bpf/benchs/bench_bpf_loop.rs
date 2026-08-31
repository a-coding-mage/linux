// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies in the original C source:
 * #include <argp.h>
 * #include "bench.h"
 * #include "bpf_loop_bench.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type ErrorT = c_int;

const ARG_NR_LOOPS: c_int = 4000;
const ARGP_ERR_UNKNOWN: ErrorT = 7;
const __NR_GETPGID: c_long = 121;

#[repr(C)]
pub struct argp_option {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

unsafe impl Sync for argp_option {}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> ErrorT>,
}

unsafe impl Sync for argp {}

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
    pub report_progress: *const c_void,
    pub report_final: *const c_void,
}

unsafe impl Sync for bench {}

#[repr(C)]
pub struct env {
    pub consumer_cnt: c_int,
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
pub struct bpf_loop_bench_bss {
    pub hits: u64,
    pub nr_loops: u32,
}

#[repr(C)]
pub struct bpf_loop_bench_progs {
    pub benchmark: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_loop_bench {
    pub bss: *mut bpf_loop_bench_bss,
    pub progs: bpf_loop_bench_progs,
}

/* BPF triggering benchmarks */
#[repr(C)]
struct ctx {
    skel: *mut bpf_loop_bench,
}

static mut CTX: ctx = ctx {
    skel: ptr::null_mut(),
};

#[repr(C)]
struct args {
    nr_loops: u32,
}

static mut ARGS: args = args { nr_loops: 10 };

static OPTS: [argp_option; 2] = [
    argp_option {
        name: b"nr_loops\0".as_ptr() as *const c_char,
        key: ARG_NR_LOOPS,
        arg: b"nr_loops\0".as_ptr() as *const c_char,
        flags: 0,
        doc: b"Set number of loops for the bpf_loop helper\0".as_ptr() as *const c_char,
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

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, _state: *mut argp_state) -> ErrorT {
    match key {
        ARG_NR_LOOPS => {
            ARGS.nr_loops = strtol(arg, ptr::null_mut(), 10) as u32;
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

/* exported into benchmark runner */
#[no_mangle]
pub static bench_bpf_loop_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if r#env.consumer_cnt != 0 {
        fprintf(
            stderr,
            b"benchmark doesn't support consumer!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    while true {
        /* trigger the bpf program */
        syscall(__NR_GETPGID);
    }

    ptr::null_mut()
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*CTX.skel).bss).hits, 0);
}

unsafe extern "C" fn setup() {
    let link: *mut bpf_link;

    setup_libbpf();

    CTX.skel = bpf_loop_bench__open_and_load();
    if CTX.skel.is_null() {
        fprintf(
            stderr,
            b"failed to open skeleton\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    link = bpf_program__attach((*CTX.skel).progs.benchmark);
    if link.is_null() {
        fprintf(
            stderr,
            b"failed to attach program!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    (*(*CTX.skel).bss).nr_loops = ARGS.nr_loops;
}

#[no_mangle]
pub static bench_bpf_loop: bench = bench {
    name: b"bpf-loop\0".as_ptr() as *const c_char,
    argp: &bench_bpf_loop_argp,
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: unsafe { ops_report_progress },
    report_final: unsafe { ops_report_final },
};

unsafe extern "C" {
    static mut r#env: env;
    static stderr: *mut c_void;
    static ops_report_progress: *const c_void;
    static ops_report_final: *const c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;

    fn setup_libbpf();
    fn bpf_loop_bench__open_and_load() -> *mut bpf_loop_bench;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn atomic_swap(ptr: *mut u64, val: u64) -> u64;
}
