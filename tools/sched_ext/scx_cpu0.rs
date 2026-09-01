// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025 Tejun Heo <tj@kernel.org>
 */

use core::ffi::{c_char, c_int, c_void};

type bool_ = bool;
type __s32 = i32;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type va_list = *mut c_void;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

const HELP_FMT: &[u8] = b"A cpu0 sched_ext scheduler.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Usage: %s [-v]\n\
\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut verbose: bool_ = false;
static mut exit_req: c_int = 0;

#[repr(C)]
pub struct scx_cpu0 {
    maps: scx_cpu0_maps,
}

#[repr(C)]
pub struct scx_cpu0_maps {
    stats: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

type libbpf_print_level = c_int;

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
    static mut optind: c_int;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: va_list) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn sleep(seconds: u32) -> u32;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn libbpf_set_print(print_fn: unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int);
    fn bpf_link__destroy(link: *mut bpf_link);
    fn scx_cpu0__destroy(skel: *mut scx_cpu0);

    /*
     * C macros from scx/common.h and scx_cpu0.bpf.skel.h:
     * SCX_OPS_OPEN(cpu0_ops, scx_cpu0)
     * SCX_OPS_LOAD(skel, cpu0_ops, scx_cpu0, uei)
     * SCX_OPS_ATTACH(skel, cpu0_ops, scx_cpu0)
     * UEI_EXITED(skel, uei)
     * UEI_REPORT(skel, uei)
     * UEI_ECODE_RESTART(ecode)
     */
    fn SCX_OPS_OPEN_cpu0_ops_scx_cpu0() -> *mut scx_cpu0;
    fn SCX_OPS_LOAD_cpu0_ops_scx_cpu0_uei(skel: *mut scx_cpu0);
    fn SCX_OPS_ATTACH_cpu0_ops_scx_cpu0(skel: *mut scx_cpu0) -> *mut bpf_link;
    fn UEI_EXITED_uei(skel: *mut scx_cpu0) -> bool_;
    fn UEI_REPORT_uei(skel: *mut scx_cpu0) -> __u64;
    fn UEI_ECODE_RESTART(ecode: __u64) -> bool_;
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

unsafe fn read_stats(skel: *mut scx_cpu0, stats: *mut __u64) {
    unsafe {
        let nr_cpus = libbpf_num_possible_cpus();
        assert!(nr_cpus > 0);

        let mut cnts: Vec<Vec<__u64>> = vec![vec![0; nr_cpus as usize]; 2];
        let mut idx: __u32;

        memset(
            stats as *mut c_void,
            0,
            core::mem::size_of::<__u64>() * 2,
        );

        idx = 0;
        while idx < 2 {
            let ret: c_int;
            let mut cpu: c_int;

            ret = bpf_map_lookup_elem(
                bpf_map__fd((*skel).maps.stats),
                &idx as *const __u32 as *const c_void,
                cnts[idx as usize].as_mut_ptr() as *mut c_void,
            );
            if ret < 0 {
                idx += 1;
                continue;
            }

            cpu = 0;
            while cpu < nr_cpus {
                *stats.add(idx as usize) =
                    (*stats.add(idx as usize)).wrapping_add(cnts[idx as usize][cpu as usize]);
                cpu += 1;
            }

            idx += 1;
        }
    }
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut skel: *mut scx_cpu0;
        let mut link: *mut bpf_link;
        let mut opt: __s32;
        let mut ecode: __u64;

        libbpf_set_print(libbpf_print_fn);
        signal(SIGINT, sigint_handler);
        signal(SIGTERM, sigint_handler);

        loop {
            optind = 1;
            skel = SCX_OPS_OPEN_cpu0_ops_scx_cpu0();

            loop {
                opt = getopt(argc, argv, b"vh\0".as_ptr() as *const c_char);
                if opt == -1 {
                    break;
                }

                match opt {
                    x if x == b'v' as __s32 => {
                        verbose = true;
                    }
                    _ => {
                        fprintf(
                            stderr,
                            HELP_FMT.as_ptr() as *const c_char,
                            basename(*argv.add(0)),
                        );
                        return (opt != b'h' as __s32) as c_int;
                    }
                }
            }

            SCX_OPS_LOAD_cpu0_ops_scx_cpu0_uei(skel);
            link = SCX_OPS_ATTACH_cpu0_ops_scx_cpu0(skel);

            while exit_req == 0 && !UEI_EXITED_uei(skel) {
                let mut stats: [__u64; 2] = [0; 2];

                read_stats(skel, stats.as_mut_ptr());
                printf(
                    b"local=%llu cpu0=%llu\n\0".as_ptr() as *const c_char,
                    stats[0],
                    stats[1],
                );
                fflush(stdout);
                sleep(1);
            }

            bpf_link__destroy(link);
            ecode = UEI_REPORT_uei(skel);
            scx_cpu0__destroy(skel);

            if !(exit_req == 0 && UEI_ECODE_RESTART(ecode)) {
                break;
            }
        }

        0
    }
}

fn main() {
    let args: Vec<std::ffi::CString> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .chain(core::iter::once(core::ptr::null_mut()))
        .collect();

    unsafe {
        std::process::exit(main_impl((argv.len() - 1) as c_int, argv.as_mut_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
