/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2023 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2023 David Vernet <dvernet@meta.com>
 */
/* Translated from C. Original includes:
 * <stdio.h>, <signal.h>, <assert.h>, <unistd.h>, <libgen.h>, <limits.h>,
 * <inttypes.h>, <fcntl.h>, <time.h>, <bpf/bpf.h>, <scx/common.h>,
 * "scx_flatcg.h", "scx_flatcg.bpf.skel.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type size_t = usize;
type va_list = *mut c_void;

const FILEID_KERNFS: c_int = 0xfe;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const EOF: c_int = -1;

/* Constants from scx_flatcg.h. Kept as external dependencies. */
extern "C" {
    static FCG_NR_STATS: __u32;
    static FCG_STAT_ACT: usize;
    static FCG_STAT_DEACT: usize;
    static FCG_STAT_GLOBAL: usize;
    static FCG_STAT_LOCAL: usize;
    static FCG_STAT_HWT_CACHE: usize;
    static FCG_STAT_HWT_UPDATES: usize;
    static FCG_STAT_HWT_SKIP: usize;
    static FCG_STAT_HWT_RACE: usize;
    static FCG_STAT_ENQ_SKIP: usize;
    static FCG_STAT_ENQ_RACE: usize;
    static FCG_STAT_CNS_KEEP: usize;
    static FCG_STAT_CNS_EXPIRE: usize;
    static FCG_STAT_CNS_EMPTY: usize;
    static FCG_STAT_CNS_GONE: usize;
    static FCG_STAT_PNC_NEXT: usize;
    static FCG_STAT_PNC_EMPTY: usize;
    static FCG_STAT_PNC_NO_CGRP: usize;
    static FCG_STAT_PNC_GONE: usize;
    static FCG_STAT_PNC_RACE: usize;
    static FCG_STAT_PNC_FAIL: usize;
    static FCG_STAT_BAD_REMOVAL: usize;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

type libbpf_print_level = c_int;

#[repr(C)]
pub struct scx_flatcg_rodata {
    pub nr_cpus: c_int,
    pub cgrp_slice_ns: __u64,
    pub fifo_sched: bool,
}

#[repr(C)]
pub struct scx_flatcg_data {
    pub hweight_gen: __u64,
}

#[repr(C)]
pub struct scx_flatcg_maps {
    pub stats: *mut bpf_map,
}

#[repr(C)]
pub struct scx_flatcg {
    pub rodata: *mut scx_flatcg_rodata,
    pub data: *mut scx_flatcg_data,
    pub maps: scx_flatcg_maps,
}

extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn vfprintf(stream: *mut FILE, format: *const c_char, args: va_list) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn perror(s: *const c_char);
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtok_r(str: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;

    fn libbpf_set_print(
        fn_: Option<unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int>,
    );
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn SCX_OPS_OPEN_flatcg_ops_scx_flatcg() -> *mut scx_flatcg;
    fn SCX_OPS_LOAD_flatcg_ops_scx_flatcg_uei(skel: *mut scx_flatcg);
    fn SCX_OPS_ATTACH_flatcg_ops_scx_flatcg(skel: *mut scx_flatcg) -> *mut bpf_link;
    fn UEI_EXITED_scx_flatcg_uei(skel: *mut scx_flatcg) -> bool;
    fn UEI_REPORT_scx_flatcg_uei(skel: *mut scx_flatcg) -> __u64;
    fn UEI_ECODE_RESTART(ecode: __u64) -> bool;
    fn scx_flatcg__destroy(skel: *mut scx_flatcg);
    fn __COMPAT_ENUM_OR_ZERO(enum_name: *const c_char, value_name: *const c_char) -> __u64;
}

const help_fmt: &[u8] = b"A flattened cgroup hierarchy sched_ext scheduler.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Usage: %s [-s SLICE_US] [-i INTERVAL] [-f] [-v]\n\
\n\
  -s SLICE_US   Override slice duration\n\
  -i INTERVAL   Report interval\n\
  -f            Use FIFO scheduling instead of weighted vtime scheduling\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut verbose: bool = false;
static mut exit_req: c_int = 0;

unsafe extern "C" fn libbpf_print_fn(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    if level == LIBBPF_DEBUG && !verbose {
        return 0;
    }
    vfprintf(stderr, format, args)
}

extern "C" fn sigint_handler(_dummy: c_int) {
    unsafe {
        exit_req = 1;
    }
}

unsafe fn read_cpu_util(last_sum: *mut __u64, last_idle: *mut __u64) -> c_float {
    let mut fp: *mut FILE;
    let mut buf = [0 as c_char; 4096];
    let mut line: *mut c_char;
    let mut cur: *mut c_char = core::ptr::null_mut();
    let mut tok: *mut c_char;
    let mut sum: __u64 = 0;
    let mut idle: __u64 = 0;
    let mut delta_sum: __u64;
    let mut delta_idle: __u64;
    let mut idx: c_int;

    fp = fopen(b"/proc/stat\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        perror(b"fopen(\"/proc/stat\")\0".as_ptr() as *const c_char);
        return 0.0;
    }

    if fgets(buf.as_mut_ptr(), buf.len() as c_int, fp).is_null() {
        perror(b"fgets(\"/proc/stat\")\0".as_ptr() as *const c_char);
        fclose(fp);
        return 0.0;
    }
    fclose(fp);

    line = buf.as_mut_ptr();
    idx = 0;
    loop {
        tok = strtok_r(line, b" \n\0".as_ptr() as *const c_char, &mut cur);
        if tok.is_null() {
            break;
        }
        let mut endp: *mut c_char = core::ptr::null_mut();
        let v: __u64;

        if idx == 0 {
            line = core::ptr::null_mut();
            idx += 1;
            continue;
        }
        v = strtoull(tok, &mut endp, 0);
        if endp.is_null() || *endp != 0 {
            fprintf(
                stderr,
                b"failed to parse %dth field of /proc/stat (\"%s\")\n\0".as_ptr()
                    as *const c_char,
                idx,
                tok,
            );
            idx += 1;
            continue;
        }
        sum = sum.wrapping_add(v);
        if idx == 4 {
            idle = v;
        }
        idx += 1;
    }

    delta_sum = sum.wrapping_sub(*last_sum);
    delta_idle = idle.wrapping_sub(*last_idle);
    *last_sum = sum;
    *last_idle = idle;

    if delta_sum != 0 {
        (delta_sum.wrapping_sub(delta_idle) as c_float) / delta_sum as c_float
    } else {
        0.0
    }
}

unsafe fn fcg_read_stats(skel: *mut scx_flatcg, stats: *mut __u64) {
    let mut cnts: *mut __u64;
    let mut idx: __u32;
    let nr_stats = FCG_NR_STATS as usize;

    memset(
        stats as *mut c_void,
        0,
        core::mem::size_of::<__u64>() * nr_stats,
    );

    cnts = calloc((*(*skel).rodata).nr_cpus as size_t, core::mem::size_of::<__u64>()) as *mut __u64;
    if cnts.is_null() {
        return;
    }

    idx = 0;
    while idx < FCG_NR_STATS {
        let mut ret: c_int;
        let mut cpu: c_int;

        ret = bpf_map_lookup_elem(
            bpf_map__fd((*skel).maps.stats),
            &idx as *const __u32 as *const c_void,
            cnts as *mut c_void,
        );
        if ret < 0 {
            idx += 1;
            continue;
        }
        cpu = 0;
        while cpu < (*(*skel).rodata).nr_cpus {
            *stats.add(idx as usize) = (*stats.add(idx as usize)).wrapping_add(*cnts.add(cpu as usize));
            cpu += 1;
        }
        idx += 1;
    }

    free(cnts as *mut c_void);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut skel: *mut scx_flatcg;
    let mut link: *mut bpf_link;
    let mut intv_ts = timespec {
        tv_sec: 2,
        tv_nsec: 0,
    };
    let mut last_cpu_sum: __u64 = 0;
    let mut last_cpu_idle: __u64 = 0;
    let mut last_stats = vec![0 as __u64; FCG_NR_STATS as usize];
    let mut seq: c_ulong = 0;
    let mut opt: __s32;
    let mut ecode: __u64;

    libbpf_set_print(Some(libbpf_print_fn));
    signal(SIGINT, sigint_handler);
    signal(SIGTERM, sigint_handler);

    'restart: loop {
        optind = 1;
        skel = SCX_OPS_OPEN_flatcg_ops_scx_flatcg();

        (*(*skel).rodata).nr_cpus = libbpf_num_possible_cpus();
        assert!((*(*skel).rodata).nr_cpus > 0);
        (*(*skel).rodata).cgrp_slice_ns = __COMPAT_ENUM_OR_ZERO(
            b"scx_public_consts\0".as_ptr() as *const c_char,
            b"SCX_SLICE_DFL\0".as_ptr() as *const c_char,
        );

        loop {
            let mut v: c_double;

            opt = getopt(argc, argv, b"s:i:fvh\0".as_ptr() as *const c_char);
            if opt == EOF {
                break;
            }

            match opt as u8 as char {
                's' => {
                    v = strtod(optarg, core::ptr::null_mut());
                    (*(*skel).rodata).cgrp_slice_ns = (v * 1000.0) as __u64;
                }
                'i' => {
                    v = strtod(optarg, core::ptr::null_mut());
                    intv_ts.tv_sec = v as c_long;
                    intv_ts.tv_nsec = ((v - intv_ts.tv_sec as c_float as c_double) * 1000000000.0)
                        as c_long;
                }
                'f' => {
                    (*(*skel).rodata).fifo_sched = true;
                }
                'v' => {
                    verbose = true;
                }
                'h' => {
                    fprintf(stderr, help_fmt.as_ptr() as *const c_char, basename(*argv));
                    return 0;
                }
                _ => {
                    fprintf(stderr, help_fmt.as_ptr() as *const c_char, basename(*argv));
                    return (opt != 'h' as __s32) as c_int;
                }
            }
        }

        printf(
            b"slice=%.1lfms intv=%.1lfs\0".as_ptr() as *const c_char,
            (*(*skel).rodata).cgrp_slice_ns as c_double / 1000000.0,
            intv_ts.tv_sec as c_double + intv_ts.tv_nsec as c_double / 1000000000.0,
        );

        SCX_OPS_LOAD_flatcg_ops_scx_flatcg_uei(skel);
        link = SCX_OPS_ATTACH_flatcg_ops_scx_flatcg(skel);

        while exit_req == 0 && !UEI_EXITED_scx_flatcg_uei(skel) {
            let mut acc_stats = vec![0 as __u64; FCG_NR_STATS as usize];
            let mut stats = vec![0 as __u64; FCG_NR_STATS as usize];
            let mut cpu_util: c_float;
            let mut i: c_int;

            cpu_util = read_cpu_util(&mut last_cpu_sum, &mut last_cpu_idle);

            fcg_read_stats(skel, acc_stats.as_mut_ptr());
            i = 0;
            while i < FCG_NR_STATS as c_int {
                stats[i as usize] =
                    acc_stats[i as usize].wrapping_sub(last_stats[i as usize]);
                i += 1;
            }

            memcpy(
                last_stats.as_mut_ptr() as *mut c_void,
                acc_stats.as_ptr() as *const c_void,
                core::mem::size_of::<__u64>() * acc_stats.len(),
            );

            printf(
                b"\n[SEQ %6lu cpu=%5.1lf hweight_gen=%llu]\n\0".as_ptr() as *const c_char,
                seq,
                cpu_util as c_double * 100.0,
                (*(*skel).data).hweight_gen,
            );
            seq = seq.wrapping_add(1);
            printf(
                b"       act:%6llu  deact:%6llu global:%6llu local:%6llu\n\0".as_ptr()
                    as *const c_char,
                stats[FCG_STAT_ACT],
                stats[FCG_STAT_DEACT],
                stats[FCG_STAT_GLOBAL],
                stats[FCG_STAT_LOCAL],
            );
            printf(
                b"HWT  cache:%6llu update:%6llu   skip:%6llu  race:%6llu\n\0".as_ptr()
                    as *const c_char,
                stats[FCG_STAT_HWT_CACHE],
                stats[FCG_STAT_HWT_UPDATES],
                stats[FCG_STAT_HWT_SKIP],
                stats[FCG_STAT_HWT_RACE],
            );
            printf(
                b"ENQ   skip:%6llu   race:%6llu\n\0".as_ptr() as *const c_char,
                stats[FCG_STAT_ENQ_SKIP],
                stats[FCG_STAT_ENQ_RACE],
            );
            printf(
                b"CNS   keep:%6llu expire:%6llu  empty:%6llu  gone:%6llu\n\0".as_ptr()
                    as *const c_char,
                stats[FCG_STAT_CNS_KEEP],
                stats[FCG_STAT_CNS_EXPIRE],
                stats[FCG_STAT_CNS_EMPTY],
                stats[FCG_STAT_CNS_GONE],
            );
            printf(
                b"PNC   next:%6llu  empty:%6llu nocgrp:%6llu  gone:%6llu race:%6llu fail:%6llu\n\0"
                    .as_ptr() as *const c_char,
                stats[FCG_STAT_PNC_NEXT],
                stats[FCG_STAT_PNC_EMPTY],
                stats[FCG_STAT_PNC_NO_CGRP],
                stats[FCG_STAT_PNC_GONE],
                stats[FCG_STAT_PNC_RACE],
                stats[FCG_STAT_PNC_FAIL],
            );
            printf(
                b"BAD remove:%6llu\n\0".as_ptr() as *const c_char,
                acc_stats[FCG_STAT_BAD_REMOVAL],
            );
            fflush(stdout);

            nanosleep(&intv_ts, core::ptr::null_mut());
        }

        bpf_link__destroy(link);
        ecode = UEI_REPORT_scx_flatcg_uei(skel);
        scx_flatcg__destroy(skel);

        if exit_req == 0 && UEI_ECODE_RESTART(ecode) {
            continue 'restart;
        }
        break;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
