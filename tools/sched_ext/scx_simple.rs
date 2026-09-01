/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// C dependencies translated as external declarations:
// <stdio.h>, <unistd.h>, <signal.h>, <assert.h>, <libgen.h>,
// <bpf/bpf.h>, <scx/common.h>, "scx_simple.bpf.skel.h"

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type VaList = *mut c_void;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

#[allow(non_camel_case_types)]
type libbpf_print_level = c_int;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_simple_rodata {
    pub fifo_sched: bool,
}

#[repr(C)]
pub struct scx_simple_maps {
    pub stats: *mut c_void,
}

#[repr(C)]
pub struct scx_simple {
    pub rodata: *mut scx_simple_rodata,
    pub maps: scx_simple_maps,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
    static mut optind: c_int;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: VaList) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn sleep(seconds: u32) -> u32;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;

    fn libbpf_set_print(
        print_fn: Option<
            unsafe extern "C" fn(libbpf_print_level, *const c_char, VaList) -> c_int,
        >,
    );
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn scx_simple__destroy(skel: *mut scx_simple);
}

pub const HELP_FMT: &[u8] = b"A simple sched_ext scheduler.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Usage: %s [-f] [-v]\n\
\n\
  -f            Use FIFO scheduling instead of weighted vtime scheduling\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut verbose: bool = false;
static mut exit_req: c_int = 0;

unsafe extern "C" fn libbpf_print_fn(
    level: libbpf_print_level,
    format: *const c_char,
    args: VaList,
) -> c_int {
    unsafe {
        if level == LIBBPF_DEBUG && !verbose {
            return 0;
        }
        vfprintf(stderr, format, args)
    }
}

extern "C" fn sigint_handler(_simple: c_int) {
    unsafe {
        exit_req = 1;
    }
}

unsafe fn read_stats(skel: *mut scx_simple, stats: *mut __u64) {
    unsafe {
        let nr_cpus = libbpf_num_possible_cpus();
        assert!(nr_cpus > 0);
        let mut cnts = vec![[0u64; 2]; nr_cpus as usize];
        let mut idx: __u32;

        *stats.add(0) = 0;
        *stats.add(1) = 0;

        idx = 0;
        while idx < 2 {
            let mut ret: c_int;
            let mut cpu: c_int;

            ret = bpf_map_lookup_elem(
                bpf_map__fd((*skel).maps.stats),
                &idx as *const __u32 as *const c_void,
                cnts.as_mut_ptr().add(idx as usize) as *mut c_void,
            );
            if ret < 0 {
                idx += 1;
                continue;
            }
            cpu = 0;
            while cpu < nr_cpus {
                *stats.add(idx as usize) =
                    (*stats.add(idx as usize)).wrapping_add(cnts[cpu as usize][idx as usize]);
                cpu += 1;
            }
            idx += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut skel: *mut scx_simple;
        let mut link: *mut bpf_link;
        let mut opt: __s32;
        let mut ecode: __u64;

        libbpf_set_print(Some(libbpf_print_fn));
        signal(SIGINT, sigint_handler);
        signal(SIGTERM, sigint_handler);

        'restart: loop {
            optind = 1;

            // SCX_OPS_OPEN(simple_ops, scx_simple) from scx/common.h.
            skel = SCX_OPS_OPEN!(simple_ops, scx_simple);

            loop {
                opt = getopt(argc, argv, c"fvh".as_ptr()) as __s32;
                if opt == -1 {
                    break;
                }
                match opt {
                    x if x == b'f' as __s32 => {
                        (*(*skel).rodata).fifo_sched = true;
                    }
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

            // SCX_OPS_LOAD(skel, simple_ops, scx_simple, uei) from scx/common.h.
            SCX_OPS_LOAD!(skel, simple_ops, scx_simple, uei);
            // SCX_OPS_ATTACH(skel, simple_ops, scx_simple) from scx/common.h.
            link = SCX_OPS_ATTACH!(skel, simple_ops, scx_simple);

            while exit_req == 0 && !UEI_EXITED!(skel, uei) {
                let mut stats: [__u64; 2] = [0; 2];

                read_stats(skel, stats.as_mut_ptr());
                printf(
                    c"local=%llu global=%llu\n".as_ptr(),
                    stats[0],
                    stats[1],
                );
                fflush(stdout);
                sleep(1);
            }

            bpf_link__destroy(link);
            ecode = UEI_REPORT!(skel, uei);
            scx_simple__destroy(skel);

            if exit_req == 0 && UEI_ECODE_RESTART!(ecode) {
                continue 'restart;
            }
            break;
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
