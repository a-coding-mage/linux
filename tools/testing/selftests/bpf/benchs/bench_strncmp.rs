// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */
// C dependencies: <argp.h>, "bench.h", "strncmp_bench.skel.h"

use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use std::mem;
use std::ptr;

type error_t = c_int;
type u32 = u32;
type size_t = usize;

const ARG_CMP_STR_LEN: c_int = 5000;
const ARGP_ERR_UNKNOWN: error_t = 7;
const STDERR_FILENO: c_int = 2;
const __NR_getpgid: c_long = 121;

#[repr(C)]
pub struct strncmp_ctx {
    pub skel: *mut strncmp_bench,
}

static mut ctx: strncmp_ctx = strncmp_ctx {
    skel: ptr::null_mut(),
};

#[repr(C)]
pub struct strncmp_args {
    pub cmp_str_len: u32,
}

static mut args: strncmp_args = strncmp_args { cmp_str_len: 32 };

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
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strncmp_bench {
    pub bss: *mut strncmp_bench_bss,
    pub rodata: *mut strncmp_bench_rodata,
    pub progs: strncmp_bench_progs,
}

#[repr(C)]
pub struct strncmp_bench_bss {
    pub hits: i64,
    // Supplied by strncmp_bench.skel.h; exact bound is external to this file.
    pub str_: [c_char; 0],
}

#[repr(C)]
pub struct strncmp_bench_rodata {
    // Supplied by strncmp_bench.skel.h; exact bound is external to this file.
    pub target: [c_char; 0],
    pub cmp_str_len: u32,
}

#[repr(C)]
pub struct strncmp_bench_progs {
    pub strncmp_no_helper: *mut bpf_program,
    pub strncmp_helper: *mut bpf_program,
}

#[repr(C)]
pub struct bench_env {
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct bench_res {
    pub hits: i64,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut bench_res, *mut bench_res)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res)>,
}

unsafe extern "C" {
    static env: bench_env;
    static mut stderr: *mut c_void;

    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn setup_libbpf();
    fn strncmp_bench__open() -> *mut strncmp_bench;
    fn strncmp_bench__load(obj: *mut strncmp_bench) -> c_int;
    fn strncmp_bench__destroy(obj: *mut strncmp_bench);
    fn srandom(seed: u32);
    fn random() -> c_long;
    fn time(tloc: *mut c_long) -> c_long;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn syscall(num: c_long, ...) -> c_long;
    fn atomic_swap(ptr: *mut i64, val: i64) -> i64;

    fn hits_drops_report_progress(res: *mut bench_res, prev: *mut bench_res);
    fn hits_drops_report_final(res: *mut bench_res);
}

static opts: [argp_option; 2] = [
    argp_option {
        name: b"cmp-str-len\0".as_ptr() as *const c_char,
        key: ARG_CMP_STR_LEN,
        arg: b"CMP_STR_LEN\0".as_ptr() as *const c_char,
        flags: 0,
        doc: b"Set the length of compared string\0".as_ptr() as *const c_char,
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

unsafe extern "C" fn strncmp_parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> error_t {
    match key {
        ARG_CMP_STR_LEN => {
            args.cmp_str_len = strtoul(arg, ptr::null_mut(), 10) as u32;
            if args.cmp_str_len == 0
                || (args.cmp_str_len as usize)
                    >= mem::size_of_val(&(*(*ctx.skel).bss).str_)
            {
                fprintf(
                    stderr,
                    b"Invalid cmp str len (limit %zu)\n\0".as_ptr() as *const c_char,
                    mem::size_of_val(&(*(*ctx.skel).bss).str_),
                );
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
pub static bench_strncmp_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(strncmp_parse_arg),
};

unsafe extern "C" fn strncmp_validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            stderr,
            b"strncmp benchmark doesn't support consumer!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn strncmp_setup() {
    let mut err: c_int;
    let target: *mut c_char;
    let mut i: size_t;
    let sz: size_t;

    sz = mem::size_of_val(&(*(*ctx.skel).rodata).target);
    if sz == 0 || sz < mem::size_of_val(&(*(*ctx.skel).bss).str_) {
        fprintf(
            stderr,
            b"invalid string size (target %zu, src %zu)\n\0".as_ptr() as *const c_char,
            sz,
            mem::size_of_val(&(*(*ctx.skel).bss).str_),
        );
        exit(1);
    }

    setup_libbpf();

    ctx.skel = strncmp_bench__open();
    if ctx.skel.is_null() {
        fprintf(
            stderr,
            b"failed to open skeleton\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }

    srandom(time(ptr::null_mut()) as u32);
    target = (*(*ctx.skel).rodata).target.as_mut_ptr();
    i = 0;
    while i < sz - 1 {
        *target.add(i) = (b'1' as c_long + random() % 9) as c_char;
        i += 1;
    }
    *target.add(sz - 1) = b'\0' as c_char;

    (*(*ctx.skel).rodata).cmp_str_len = args.cmp_str_len;

    memcpy(
        (*(*ctx.skel).bss).str_.as_mut_ptr() as *mut c_void,
        target as *const c_void,
        args.cmp_str_len as size_t,
    );
    *(*(*ctx.skel).bss).str_.as_mut_ptr().add(args.cmp_str_len as usize) = b'\0' as c_char;
    /* Make bss->str < rodata->target */
    *(*(*ctx.skel).bss)
        .str_
        .as_mut_ptr()
        .add(args.cmp_str_len as usize - 1) -= 1;

    err = strncmp_bench__load(ctx.skel);
    if err != 0 {
        fprintf(
            stderr,
            b"failed to load skeleton\n\0".as_ptr() as *const c_char,
        );
        strncmp_bench__destroy(ctx.skel);
        exit(1);
    }
}

unsafe extern "C" fn strncmp_attach_prog(prog: *mut bpf_program) {
    let link: *mut bpf_link;

    link = bpf_program__attach(prog);
    if link.is_null() {
        fprintf(
            stderr,
            b"failed to attach program!\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn strncmp_no_helper_setup() {
    strncmp_setup();
    strncmp_attach_prog((*ctx.skel).progs.strncmp_no_helper);
}

unsafe extern "C" fn strncmp_helper_setup() {
    strncmp_setup();
    strncmp_attach_prog((*ctx.skel).progs.strncmp_helper);
}

unsafe extern "C" fn strncmp_producer(_ctx: *mut c_void) -> *mut c_void {
    loop {
        syscall(__NR_getpgid);
    }
}

unsafe extern "C" fn strncmp_measure(res: *mut bench_res) {
    (*res).hits = atomic_swap(&mut (*(*ctx.skel).bss).hits, 0);
}

#[unsafe(no_mangle)]
pub static bench_strncmp_no_helper: bench = bench {
    name: b"strncmp-no-helper\0".as_ptr() as *const c_char,
    argp: &bench_strncmp_argp,
    validate: Some(strncmp_validate),
    setup: Some(strncmp_no_helper_setup),
    producer_thread: Some(strncmp_producer),
    measure: Some(strncmp_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_strncmp_helper: bench = bench {
    name: b"strncmp-helper\0".as_ptr() as *const c_char,
    argp: &bench_strncmp_argp,
    validate: Some(strncmp_validate),
    setup: Some(strncmp_helper_setup),
    producer_thread: Some(strncmp_producer),
    measure: Some(strncmp_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};
