/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void, VaList};
use core::ptr;

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scx_pair_rodata {
    pub nr_cpu_ids: __u32,
    pub pair_batch_dur_ns: __u64,
}

#[repr(C)]
pub struct scx_pair_rodata_pair_cpu {
    pub pair_cpu: *mut __s32,
}

#[repr(C)]
pub struct scx_pair_rodata_pair_id {
    pub pair_id: *mut __u32,
}

#[repr(C)]
pub struct scx_pair_rodata_in_pair_idx {
    pub in_pair_idx: *mut __u32,
}

#[repr(C)]
pub struct scx_pair_bss {
    pub nr_total: __u64,
    pub nr_dispatched: __u64,
    pub nr_missing: __u64,
    pub nr_kicks: __u64,
    pub nr_preemptions: __u64,
    pub nr_exps: __u64,
    pub nr_exp_waits: __u64,
    pub nr_exp_empty: __u64,
    pub nr_cgrp_next: __u64,
    pub nr_cgrp_coll: __u64,
    pub nr_cgrp_empty: __u64,
}

#[repr(C)]
pub struct scx_pair_maps {
    pub pair_ctx: *mut bpf_map,
    pub cgrp_q_arr: *mut bpf_map,
}

#[repr(C)]
pub struct scx_pair {
    pub rodata: *mut scx_pair_rodata,
    pub rodata_pair_cpu: *mut scx_pair_rodata_pair_cpu,
    pub rodata_pair_id: *mut scx_pair_rodata_pair_id,
    pub rodata_in_pair_idx: *mut scx_pair_rodata_in_pair_idx,
    pub bss: *mut scx_pair_bss,
    pub maps: scx_pair_maps,
}

#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 0,
    LIBBPF_INFO = 1,
    LIBBPF_DEBUG = 2,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut optind: c_int;
    static mut optarg: *mut c_char;

    static BPF_MAP_TYPE_QUEUE: c_int;
    static BPF_ANY: c_uint;
    static MAX_CGRPS: __s32;
    static MAX_QUEUED: __u32;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: VaList<'_, '_>) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn close(fd: c_int) -> c_int;

    fn libbpf_set_print(
        fn_: Option<
            unsafe extern "C" fn(libbpf_print_level, *const c_char, VaList<'_, '_>) -> c_int,
        >,
    );
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn scx_pair__destroy(obj: *mut scx_pair);
}

unsafe extern "C" {
    static pair_ops: c_void;
    static uei: c_void;
}

macro_rules! SCX_OPS_OPEN {
    ($ops:ident, $skel:ident) => {
        compile_error!("external scx/common.h macro SCX_OPS_OPEN is required")
    };
}

macro_rules! SCX_OPS_LOAD {
    ($skel:expr, $ops:ident, $name:ident, $uei:ident) => {
        compile_error!("external scx/common.h macro SCX_OPS_LOAD is required")
    };
}

macro_rules! SCX_OPS_ATTACH {
    ($skel:expr, $ops:ident, $name:ident) => {
        compile_error!("external scx/common.h macro SCX_OPS_ATTACH is required")
    };
}

macro_rules! RESIZE_ARRAY {
    ($skel:expr, $section:ident, $name:ident, $cnt:expr) => {
        compile_error!("external scx/common.h macro RESIZE_ARRAY is required")
    };
}

macro_rules! SCX_BUG_ON {
    ($cond:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        compile_error!("external scx/common.h macro SCX_BUG_ON is required")
    };
}

macro_rules! UEI_EXITED {
    ($skel:expr, $uei:ident) => {
        compile_error!("external scx/common.h macro UEI_EXITED is required")
    };
}

macro_rules! UEI_REPORT {
    ($skel:expr, $uei:ident) => {
        compile_error!("external scx/common.h macro UEI_REPORT is required")
    };
}

macro_rules! UEI_ECODE_RESTART {
    ($ecode:expr) => {
        compile_error!("external scx/common.h macro UEI_ECODE_RESTART is required")
    };
}

macro_rules! __COMPAT_ENUM_OR_ZERO {
    ($enum_name:expr, $value_name:expr) => {
        compile_error!("external scx/common.h macro __COMPAT_ENUM_OR_ZERO is required")
    };
}

pub static help_fmt: &[u8] =
    b"A demo sched_ext core-scheduler which always makes every sibling CPU pair\n\
execute from the same CPU cgroup.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Usage: %s [-S STRIDE] [-v]\n\
\n\
  -S STRIDE     Override CPU pair stride (default: nr_cpus_ids / 2)\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut verbose: bool = false;
static mut exit_req: c_int = 0;

unsafe extern "C" fn libbpf_print_fn(
    level: libbpf_print_level,
    format: *const c_char,
    args: VaList<'_, '_>,
) -> c_int {
    if level == libbpf_print_level::LIBBPF_DEBUG && !verbose {
        return 0;
    }
    vfprintf(stderr, format, args)
}

extern "C" fn sigint_handler(_dummy: c_int) {
    unsafe {
        ptr::write_volatile(&raw mut exit_req, 1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut skel: *mut scx_pair;
    let mut link: *mut bpf_link;
    let mut seq: __u64 = 0;
    let mut ecode: __u64;
    let mut stride: __s32;
    let mut i: __s32;
    let mut opt: __s32;
    let mut outer_fd: __s32;
    let mut pair_id: __u32 = 0;

    libbpf_set_print(Some(libbpf_print_fn));
    signal(2, sigint_handler);
    signal(15, sigint_handler);

    loop {
        optind = 1;
        skel = SCX_OPS_OPEN!(pair_ops, scx_pair);

        (*(*skel).rodata).nr_cpu_ids = libbpf_num_possible_cpus() as __u32;
        (*(*skel).rodata).pair_batch_dur_ns =
            __COMPAT_ENUM_OR_ZERO!("scx_public_consts\0".as_ptr() as *const c_char,
                                   "SCX_SLICE_DFL\0".as_ptr() as *const c_char);

        /* pair up the earlier half to the latter by default, override with -s */
        stride = ((*(*skel).rodata).nr_cpu_ids / 2) as __s32;

        loop {
            opt = getopt(argc, argv, b"S:vh\0".as_ptr() as *const c_char);
            if opt == -1 {
                break;
            }

            match opt {
                83 => {
                    stride = strtoul(optarg, ptr::null_mut(), 0) as __s32;
                }
                118 => {
                    verbose = true;
                }
                _ => {
                    fprintf(
                        stderr,
                        help_fmt.as_ptr() as *const c_char,
                        basename(*argv.add(0)),
                    );
                    return if opt != 104 { 1 } else { 0 };
                }
            }
        }

        /* Stride must be positive to pair distinct CPUs. */
        if stride <= 0 {
            fprintf(
                stderr,
                b"Invalid stride %d, must be positive\n\0".as_ptr() as *const c_char,
                stride,
            );
            scx_pair__destroy(skel);
            return -1;
        }

        if ((*(*skel).rodata).nr_cpu_ids & 1) != 0 {
            fprintf(
                stderr,
                b"scx_pair requires an even CPU count, got %u\n\0".as_ptr() as *const c_char,
                (*(*skel).rodata).nr_cpu_ids,
            );
            scx_pair__destroy(skel);
            return -1;
        }

        bpf_map__set_max_entries((*skel).maps.pair_ctx, (*(*skel).rodata).nr_cpu_ids / 2);

        /* Resize arrays so their element count is equal to cpu count. */
        RESIZE_ARRAY!(skel, rodata, pair_cpu, (*(*skel).rodata).nr_cpu_ids);
        RESIZE_ARRAY!(skel, rodata, pair_id, (*(*skel).rodata).nr_cpu_ids);
        RESIZE_ARRAY!(skel, rodata, in_pair_idx, (*(*skel).rodata).nr_cpu_ids);

        i = 0;
        while i < (*(*skel).rodata).nr_cpu_ids as __s32 {
            *(*(*skel).rodata_pair_cpu).pair_cpu.add(i as usize) = -1;
            i += 1;
        }

        printf(b"Pairs: \0".as_ptr() as *const c_char);
        i = 0;
        while i < (*(*skel).rodata).nr_cpu_ids as __s32 {
            let j: c_int = (i + stride) % (*(*skel).rodata).nr_cpu_ids as __s32;

            if *(*(*skel).rodata_pair_cpu).pair_cpu.add(i as usize) >= 0 {
                i += 1;
                continue;
            }

            SCX_BUG_ON!(
                i == j,
                b"Invalid stride %d - CPU%d wants to be its own pair\0".as_ptr() as *const c_char,
                stride,
                i
            );

            SCX_BUG_ON!(
                *(*(*skel).rodata_pair_cpu).pair_cpu.add(j as usize) >= 0,
                b"Invalid stride %d - three CPUs (%d, %d, %d) want to be a pair\0".as_ptr()
                    as *const c_char,
                stride,
                i,
                j,
                *(*(*skel).rodata_pair_cpu).pair_cpu.add(j as usize)
            );

            *(*(*skel).rodata_pair_cpu).pair_cpu.add(i as usize) = j;
            *(*(*skel).rodata_pair_cpu).pair_cpu.add(j as usize) = i;
            *(*(*skel).rodata_pair_id).pair_id.add(i as usize) = pair_id;
            *(*(*skel).rodata_pair_id).pair_id.add(j as usize) = pair_id;
            *(*(*skel).rodata_in_pair_idx).in_pair_idx.add(i as usize) = 0;
            *(*(*skel).rodata_in_pair_idx).in_pair_idx.add(j as usize) = 1;
            pair_id = pair_id.wrapping_add(1);

            printf(b"[%d, %d] \0".as_ptr() as *const c_char, i, j);
            i += 1;
        }
        printf(b"\n\0".as_ptr() as *const c_char);

        SCX_OPS_LOAD!(skel, pair_ops, scx_pair, uei);

        /*
         * Populate the cgrp_q_arr map which is an array containing per-cgroup
         * queues. It'd probably be better to do this from BPF but there are too
         * many to initialize statically and there's no way to dynamically
         * populate from BPF.
         */
        outer_fd = bpf_map__fd((*skel).maps.cgrp_q_arr);
        SCX_BUG_ON!(
            outer_fd < 0,
            b"Failed to get outer_fd: %d\0".as_ptr() as *const c_char,
            outer_fd
        );

        printf(b"Initializing\0".as_ptr() as *const c_char);
        i = 0;
        while i < MAX_CGRPS {
            let inner_fd: __s32;

            if ptr::read_volatile(&raw const exit_req) != 0 {
                break;
            }

            inner_fd = bpf_map_create(
                BPF_MAP_TYPE_QUEUE,
                ptr::null(),
                0,
                core::mem::size_of::<__u32>() as __u32,
                MAX_QUEUED,
                ptr::null(),
            );
            SCX_BUG_ON!(
                inner_fd < 0,
                b"Failed to get inner_fd: %d\0".as_ptr() as *const c_char,
                inner_fd
            );
            SCX_BUG_ON!(
                bpf_map_update_elem(
                    outer_fd,
                    (&i as *const __s32).cast::<c_void>(),
                    (&inner_fd as *const __s32).cast::<c_void>(),
                    BPF_ANY,
                ) != 0,
                b"Failed to set inner map\0".as_ptr() as *const c_char
            );
            close(inner_fd);

            if (i % 10) == 0 {
                printf(b".\0".as_ptr() as *const c_char);
            }
            fflush(stdout);
            i += 1;
        }
        printf(b"\n\0".as_ptr() as *const c_char);

        /*
         * Fully initialized, attach and run.
         */
        link = SCX_OPS_ATTACH!(skel, pair_ops, scx_pair);

        while ptr::read_volatile(&raw const exit_req) == 0 && !UEI_EXITED!(skel, uei) {
            printf(b"[SEQ %llu]\n\0".as_ptr() as *const c_char, seq);
            seq = seq.wrapping_add(1);
            printf(
                b" total:%10llu dispatch:%10llu   missing:%10llu\n\0".as_ptr() as *const c_char,
                (*(*skel).bss).nr_total,
                (*(*skel).bss).nr_dispatched,
                (*(*skel).bss).nr_missing,
            );
            printf(
                b" kicks:%10llu preemptions:%7llu\n\0".as_ptr() as *const c_char,
                (*(*skel).bss).nr_kicks,
                (*(*skel).bss).nr_preemptions,
            );
            printf(
                b"   exp:%10llu exp_wait:%10llu exp_empty:%10llu\n\0".as_ptr() as *const c_char,
                (*(*skel).bss).nr_exps,
                (*(*skel).bss).nr_exp_waits,
                (*(*skel).bss).nr_exp_empty,
            );
            printf(
                b"cgnext:%10llu   cgcoll:%10llu   cgempty:%10llu\n\0".as_ptr() as *const c_char,
                (*(*skel).bss).nr_cgrp_next,
                (*(*skel).bss).nr_cgrp_coll,
                (*(*skel).bss).nr_cgrp_empty,
            );
            fflush(stdout);
            sleep(1);
        }

        bpf_link__destroy(link);
        ecode = UEI_REPORT!(skel, uei);
        scx_pair__destroy(skel);

        if ptr::read_volatile(&raw const exit_req) == 0 && UEI_ECODE_RESTART!(ecode) {
            continue;
        }
        break;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
