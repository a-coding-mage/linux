/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2024 Emil Tsalapatis <etsal@meta.com>
 * Copyright (c) 2024 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

type __s32 = i32;
type __u64 = u64;
type bool_ = bool;
type va_list = *mut c_void;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

pub static help_fmt: &[u8] = b"\
A simple arena-based sched_ext scheduler.\n\
\n\
Modified version of scx_simple that demonstrates arena-based data structures.\n\
\n\
Usage: %s [-v]\n\
\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\
\0";

static mut verbose: bool_ = false;
static mut exit_req: c_int = 0;

type libbpf_print_level = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct alloc_stats {
    pub chunk_allocs: __u64,
    pub data_allocs: __u64,
    pub alloc_ops: __u64,
    pub free_ops: __u64,
    pub active_allocs: __u64,
    pub arena_pages_used: __u64,
}

#[repr(C)]
pub struct scx_sdt_bss {
    pub stat_enqueue: __u64,
    pub stat_init: __u64,
    pub stat_exit: __u64,
    pub stat_select_idle_cpu: __u64,
    pub stat_select_busy_cpu: __u64,
    pub alloc_stats: alloc_stats,
}

#[repr(C)]
pub struct scx_sdt {
    pub bss: *mut scx_sdt_bss,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut optind: c_int;

    fn vfprintf(stream: *mut FILE, format: *const c_char, args: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn sleep(seconds: c_int) -> c_int;
    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>)
        -> Option<unsafe extern "C" fn(c_int)>;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;

    fn libbpf_set_print(
        fn_: Option<
            unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int,
        >,
    );
    fn bpf_link__destroy(link: *mut bpf_link);

    /*
     * Generated skeleton and sched_ext helper interfaces supplied by
     * scx_sdt.h, scx_sdt.bpf.skel.h, and scx/common.h in the original source.
     * These correspond to the C macros:
     *   SCX_OPS_OPEN(sdt_ops, scx_sdt)
     *   SCX_OPS_LOAD(skel, sdt_ops, scx_sdt, uei)
     *   SCX_OPS_ATTACH(skel, sdt_ops, scx_sdt)
     *   UEI_EXITED(skel, uei)
     *   UEI_REPORT(skel, uei)
     *   UEI_ECODE_RESTART(ecode)
     */
    fn SCX_OPS_OPEN_sdt_ops_scx_sdt() -> *mut scx_sdt;
    fn SCX_OPS_LOAD_skel_sdt_ops_scx_sdt_uei(skel: *mut scx_sdt);
    fn SCX_OPS_ATTACH_skel_sdt_ops_scx_sdt(skel: *mut scx_sdt) -> *mut bpf_link;
    fn UEI_EXITED_skel_uei(skel: *mut scx_sdt) -> bool_;
    fn UEI_REPORT_skel_uei(skel: *mut scx_sdt) -> __u64;
    fn UEI_ECODE_RESTART(ecode: __u64) -> bool_;
    fn scx_sdt__destroy(obj: *mut scx_sdt);
}

unsafe extern "C" fn libbpf_print_fn(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    unsafe {
        if level == LIBBPF_DEBUG && !verbose {
            return 0;
        }
        vfprintf(stderr, format, args)
    }
}

unsafe extern "C" fn sigint_handler(_sig: c_int) {
    unsafe {
        exit_req = 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut skel: *mut scx_sdt;
        let mut link: *mut bpf_link;
        let mut opt: __s32;
        let mut ecode: __u64;

        libbpf_set_print(Some(libbpf_print_fn));
        signal(SIGINT, Some(sigint_handler));
        signal(SIGTERM, Some(sigint_handler));

        'restart: loop {
            optind = 1;
            skel = SCX_OPS_OPEN_sdt_ops_scx_sdt();

            loop {
                opt = getopt(argc, argv, b"vh\0".as_ptr() as *const c_char);
                if opt == -1 {
                    break;
                }

                match opt {
                    v if v == b'v' as __s32 => {
                        verbose = true;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            help_fmt.as_ptr() as *const c_char,
                            basename(*argv.add(0)),
                        );
                        return (opt != b'h' as __s32) as c_int;
                    }
                }
            }

            SCX_OPS_LOAD_skel_sdt_ops_scx_sdt_uei(skel);
            link = SCX_OPS_ATTACH_skel_sdt_ops_scx_sdt(skel);

            while exit_req == 0 && !UEI_EXITED_skel_uei(skel) {
                printf(b"====SCHEDULING STATS====\n\0".as_ptr() as *const c_char);
                printf(
                    b"enqueues=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).stat_enqueue,
                );
                printf(
                    b"inits=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).stat_init,
                );
                printf(
                    b"exits=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).stat_exit,
                );
                printf(b"\n\0".as_ptr() as *const c_char);

                printf(
                    b"select_idle_cpu=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).stat_select_idle_cpu,
                );
                printf(
                    b"select_busy_cpu=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).stat_select_busy_cpu,
                );
                printf(b"\n\0".as_ptr() as *const c_char);

                printf(b"====ALLOCATION STATS====\n\0".as_ptr() as *const c_char);
                printf(
                    b"chunk allocs=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.chunk_allocs,
                );
                printf(
                    b"data_allocs=%llu\n\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.data_allocs,
                );
                printf(
                    b"alloc_ops=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.alloc_ops,
                );
                printf(
                    b"free_ops=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.free_ops,
                );
                printf(
                    b"active_allocs=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.active_allocs,
                );
                printf(
                    b"arena_pages_used=%llu\t\0".as_ptr() as *const c_char,
                    (*(*skel).bss).alloc_stats.arena_pages_used,
                );
                printf(b"\n\n\0".as_ptr() as *const c_char);

                fflush(stdout);
                sleep(1);
            }

            bpf_link__destroy(link);
            ecode = UEI_REPORT_skel_uei(skel);
            scx_sdt__destroy(skel);

            if exit_req == 0 && UEI_ECODE_RESTART(ecode) {
                continue 'restart;
            }
            return 0;
        }
    }
}
