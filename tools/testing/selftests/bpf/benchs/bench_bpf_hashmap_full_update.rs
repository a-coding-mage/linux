// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Bytedance */

/*
 * C dependencies:
 *   #include "bench.h"
 *   #include "bpf_hashmap_full_update_bench.skel.h"
 *   #include "bpf_util.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type U64 = u64;

const MAX_LOOP_NUM: c_int = 10000;
const BPF_ANY: u64 = 0;
const BPF_F_NO_PREALLOC: u32 = 1;
const __NR_GETPGID: c_long = 121;

#[repr(C)]
pub struct bench_res {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

#[repr(C)]
pub struct env {
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct bpf_link {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_map_type {
    BPF_MAP_TYPE_HASH,
    BPF_MAP_TYPE_RHASH,
}

#[repr(C)]
pub struct bpf_hashmap_full_update_bench_maps {
    pub hash_map_bench: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_hashmap_full_update_bench_progs {
    pub benchmark: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_hashmap_full_update_bench_bss {
    pub nr_loops: U64,
    pub percpu_time: *mut U64,
}

#[repr(C)]
pub struct bpf_hashmap_full_update_bench {
    pub maps: bpf_hashmap_full_update_bench_maps,
    pub progs: bpf_hashmap_full_update_bench_progs,
    pub bss: *mut bpf_hashmap_full_update_bench_bss,
}

#[repr(C)]
struct ctx {
    skel: *mut bpf_hashmap_full_update_bench,
}

static mut CTX: ctx = ctx {
    skel: ptr::null_mut(),
};

unsafe extern "C" {
    static env: env;
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;

    fn setup_libbpf();
    fn bpf_num_possible_cpus() -> c_uint;

    fn bpf_hashmap_full_update_bench__open() -> *mut bpf_hashmap_full_update_bench;
    fn bpf_hashmap_full_update_bench__load(skel: *mut bpf_hashmap_full_update_bench) -> c_int;

    fn bpf_map__set_type(map: *mut bpf_map, map_type: bpf_map_type);
    fn bpf_map__set_map_flags(map: *mut bpf_map, flags: u32);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
}

unsafe extern "C" fn validate() {
    unsafe {
        if env.consumer_cnt != 0 {
            fprintf(
                stderr,
                c"benchmark doesn't support consumer!\n".as_ptr(),
            );
            exit(1);
        }
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    loop {
        /* trigger the bpf program */
        unsafe {
            syscall(__NR_GETPGID);
        }
    }

    #[allow(unreachable_code)]
    ptr::null_mut()
}

unsafe extern "C" fn measure(_res: *mut bench_res) {}

unsafe extern "C" fn hashmap_full_update_setup(map_type: bpf_map_type) {
    unsafe {
        let link: *mut bpf_link;
        let map_fd: c_int;
        let mut i: c_int;
        let max_entries: c_int;

        setup_libbpf();

        CTX.skel = bpf_hashmap_full_update_bench__open();
        if CTX.skel.is_null() {
            fprintf(stderr, c"failed to open skeleton\n".as_ptr());
            exit(1);
        }

        bpf_map__set_type((*CTX.skel).maps.hash_map_bench, map_type);
        if map_type == bpf_map_type::BPF_MAP_TYPE_RHASH {
            bpf_map__set_map_flags(
                (*CTX.skel).maps.hash_map_bench,
                BPF_F_NO_PREALLOC,
            );
        }

        if bpf_hashmap_full_update_bench__load(CTX.skel) != 0 {
            fprintf(stderr, c"failed to load skeleton\n".as_ptr());
            exit(1);
        }

        (*(*CTX.skel).bss).nr_loops = MAX_LOOP_NUM as U64;

        link = bpf_program__attach((*CTX.skel).progs.benchmark);
        if link.is_null() {
            fprintf(stderr, c"failed to attach program!\n".as_ptr());
            exit(1);
        }

        /* fill hash_map */
        map_fd = bpf_map__fd((*CTX.skel).maps.hash_map_bench);
        max_entries = bpf_map__max_entries((*CTX.skel).maps.hash_map_bench);
        i = 0;
        while i < max_entries {
            bpf_map_update_elem(
                map_fd,
                &i as *const c_int as *const c_void,
                &i as *const c_int as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }
}

unsafe extern "C" fn setup() {
    unsafe {
        hashmap_full_update_setup(bpf_map_type::BPF_MAP_TYPE_HASH);
    }
}

unsafe extern "C" fn rhash_setup() {
    unsafe {
        hashmap_full_update_setup(bpf_map_type::BPF_MAP_TYPE_RHASH);
    }
}

unsafe extern "C" fn hashmap_report_final(_res: *mut bench_res, _res_cnt: c_int) {
    unsafe {
        let nr_cpus: c_uint = bpf_num_possible_cpus();
        let mut i: c_int;

        i = 0;
        while i < nr_cpus as c_int {
            let time: U64 = *(*(*CTX.skel).bss).percpu_time.add(i as usize);

            if time == 0 {
                i += 1;
                continue;
            }

            printf(
                c"%d:hash_map_full_perf %lld events per sec\n".as_ptr(),
                i,
                (*(*CTX.skel).bss).nr_loops.wrapping_mul(1000000000_i64 as U64) / time,
            );
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub static bench_bpf_hashmap_full_update: bench = bench {
    name: c"bpf-hashmap-full-update".as_ptr(),
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: None,
    report_final: Some(hashmap_report_final),
};

#[unsafe(no_mangle)]
pub static bench_bpf_rhashmap_full_update: bench = bench {
    name: c"bpf-rhashmap-full-update".as_ptr(),
    validate: Some(validate),
    setup: Some(rhash_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: None,
    report_final: Some(hashmap_report_final),
};
