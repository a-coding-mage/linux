// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

/* Translated from:
 *   <sys/random.h>
 *   <argp.h>
 *   "bench.h"
 *   "bpf_hashmap_lookup.skel.h"
 *   "bpf_util.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type error_t = c_int;
type u32 = u32;
type u64 = u64;
type __u32 = u32;

const ARGP_ERR_UNKNOWN: error_t = 7;
const UINT_MAX: c_long = c_uint::MAX as c_long;
const NULL: *mut c_void = ptr::null_mut();
const BPF_ANY: u64 = 0;
const BPF_F_NO_PREALLOC: __u32 = 1;
const __NR_getpgid: c_long = 121;

/* only available to kernel, so define it here */
const BPF_MAX_LOOPS: __u32 = 1 << 23;

const MAX_KEY_SIZE: __u32 = 1024; /* the size of the key map */

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
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench_res {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
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
    pub quiet: bool,
    pub affinity: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_map_type {
    BPF_MAP_TYPE_HASH,
    BPF_MAP_TYPE_RHASH,
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
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_hashmap_lookup_maps {
    pub hash_map_bench: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_hashmap_lookup_progs {
    pub benchmark: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_hashmap_lookup_bss {
    pub nr_entries: __u32,
    pub nr_loops: __u32,
    pub key: [u32; (MAX_KEY_SIZE / 4) as usize],
    pub percpu_times: [[u64; 32]; 0],
}

#[repr(C)]
pub struct bpf_hashmap_lookup {
    pub maps: bpf_hashmap_lookup_maps,
    pub progs: bpf_hashmap_lookup_progs,
    pub bss: *mut bpf_hashmap_lookup_bss,
}

unsafe extern "C" {
    static mut env: env;
    static mut stderr: *mut c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn sqrt(x: f64) -> f64;

    fn setup_libbpf();
    fn bpf_num_possible_cpus() -> c_uint;
    fn bpf_hashmap_lookup__open() -> *mut bpf_hashmap_lookup;
    fn bpf_hashmap_lookup__load(skel: *mut bpf_hashmap_lookup) -> c_int;
    fn bpf_hashmap_lookup__destroy(skel: *mut bpf_hashmap_lookup);
    fn bpf_map__set_type(map: *mut bpf_map, map_type: bpf_map_type);
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32);
    fn bpf_map__set_key_size(map: *mut bpf_map, key_size: __u32);
    fn bpf_map__set_value_size(map: *mut bpf_map, value_size: __u32);
    fn bpf_map__set_map_flags(map: *mut bpf_map, map_flags: __u32);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
}

/* BPF triggering benchmarks */
#[repr(C)]
struct ctx {
    skel: *mut bpf_hashmap_lookup,
}

static mut ctx: ctx = ctx {
    skel: ptr::null_mut(),
};

#[repr(C)]
struct args {
    key_size: __u32,
    map_flags: __u32,
    max_entries: __u32,
    nr_entries: __u32,
    nr_loops: __u32,
}

static mut args: args = args {
    key_size: 4,
    map_flags: 0,
    max_entries: 1000,
    nr_entries: 500,
    nr_loops: 1000000,
};

const ARG_KEY_SIZE: c_int = 8001;
const ARG_MAP_FLAGS: c_int = ARG_KEY_SIZE + 1;
const ARG_MAX_ENTRIES: c_int = ARG_MAP_FLAGS + 1;
const ARG_NR_ENTRIES: c_int = ARG_MAX_ENTRIES + 1;
const ARG_NR_LOOPS: c_int = ARG_NR_ENTRIES + 1;

static opts: [argp_option; 6] = [
    argp_option {
        name: c"key_size".as_ptr(),
        key: ARG_KEY_SIZE,
        arg: c"KEY_SIZE".as_ptr(),
        flags: 0,
        doc: c"The hashmap key size (max 1024)".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"map_flags".as_ptr(),
        key: ARG_MAP_FLAGS,
        arg: c"MAP_FLAGS".as_ptr(),
        flags: 0,
        doc: c"The hashmap flags passed to BPF_MAP_CREATE".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"max_entries".as_ptr(),
        key: ARG_MAX_ENTRIES,
        arg: c"MAX_ENTRIES".as_ptr(),
        flags: 0,
        doc: c"The hashmap max entries".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"nr_entries".as_ptr(),
        key: ARG_NR_ENTRIES,
        arg: c"NR_ENTRIES".as_ptr(),
        flags: 0,
        doc: c"The number of entries to insert/lookup".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"nr_loops".as_ptr(),
        key: ARG_NR_LOOPS,
        arg: c"NR_LOOPS".as_ptr(),
        flags: 0,
        doc: c"The number of loops for the benchmark".as_ptr(),
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

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    let ret: c_long;

    match key {
        ARG_KEY_SIZE => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > MAX_KEY_SIZE as c_long {
                fprintf(stderr, c"invalid key_size".as_ptr());
                argp_usage(state);
            }
            args.key_size = ret as __u32;
        }
        ARG_MAP_FLAGS => {
            ret = strtol(arg, ptr::null_mut(), 0);
            if ret < 0 || ret > UINT_MAX {
                fprintf(stderr, c"invalid map_flags".as_ptr());
                argp_usage(state);
            }
            args.map_flags = ret as __u32;
        }
        ARG_MAX_ENTRIES => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > UINT_MAX {
                fprintf(stderr, c"invalid max_entries".as_ptr());
                argp_usage(state);
            }
            args.max_entries = ret as __u32;
        }
        ARG_NR_ENTRIES => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > UINT_MAX {
                fprintf(stderr, c"invalid nr_entries".as_ptr());
                argp_usage(state);
            }
            args.nr_entries = ret as __u32;
        }
        ARG_NR_LOOPS => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > BPF_MAX_LOOPS as c_long {
                fprintf(
                    stderr,
                    c"invalid nr_loops: %ld (min=1 max=%u)\n".as_ptr(),
                    ret,
                    BPF_MAX_LOOPS,
                );
                argp_usage(state);
            }
            args.nr_loops = ret as __u32;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }

    0
}

#[unsafe(no_mangle)]
pub static bench_hashmap_lookup_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if env.consumer_cnt != 0 {
        fprintf(stderr, c"benchmark doesn't support consumer!\n".as_ptr());
        exit(1);
    }

    if args.nr_entries > args.max_entries {
        fprintf(
            stderr,
            c"args.nr_entries is too big! (max %u, got %u)\n".as_ptr(),
            args.max_entries,
            args.nr_entries,
        );
        exit(1);
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    loop {
        /* trigger the bpf program */
        syscall(__NR_getpgid);
    }
}

unsafe extern "C" fn measure(_res: *mut bench_res) {}

#[inline]
unsafe fn patch_key(i: u32, key: *mut u32) {
    if cfg!(target_endian = "little") {
        *key = i.wrapping_add(1);
    } else {
        *key = i.wrapping_add(1).swap_bytes();
    }
    /* the rest of key is random */
}

unsafe fn hashmap_lookup_setup(map_type: bpf_map_type) {
    let link: *mut bpf_link;
    let mut map_flags: __u32;
    let map_fd: c_int;
    let ret: c_int;
    let mut i: c_int;

    setup_libbpf();

    ctx.skel = bpf_hashmap_lookup__open();
    if ctx.skel.is_null() {
        fprintf(stderr, c"failed to open skeleton\n".as_ptr());
        exit(1);
    }

    map_flags = args.map_flags;
    if map_type == bpf_map_type::BPF_MAP_TYPE_RHASH {
        map_flags |= BPF_F_NO_PREALLOC;
    }

    bpf_map__set_type((*ctx.skel).maps.hash_map_bench, map_type);
    bpf_map__set_max_entries((*ctx.skel).maps.hash_map_bench, args.max_entries);
    bpf_map__set_key_size((*ctx.skel).maps.hash_map_bench, args.key_size);
    bpf_map__set_value_size((*ctx.skel).maps.hash_map_bench, 8);
    bpf_map__set_map_flags((*ctx.skel).maps.hash_map_bench, map_flags);

    (*(*ctx.skel).bss).nr_entries = args.nr_entries;
    (*(*ctx.skel).bss).nr_loops = args.nr_loops / args.nr_entries;

    if args.key_size > 4 {
        i = 1;
        while i < (args.key_size / 4) as c_int {
            (*(*ctx.skel).bss).key[i as usize] = 2654435761u32.wrapping_mul(i as u32);
            i += 1;
        }
    }

    ret = bpf_hashmap_lookup__load(ctx.skel);
    if ret != 0 {
        bpf_hashmap_lookup__destroy(ctx.skel);
        fprintf(stderr, c"failed to load map: %s".as_ptr(), strerror(-ret));
        exit(1);
    }

    /* fill in the hash_map */
    map_fd = bpf_map__fd((*ctx.skel).maps.hash_map_bench);
    let mut i: u64 = 0;
    while i < args.nr_entries as u64 {
        patch_key(i as u32, (*(*ctx.skel).bss).key.as_mut_ptr());
        bpf_map_update_elem(
            map_fd,
            (*(*ctx.skel).bss).key.as_ptr() as *const c_void,
            &i as *const u64 as *const c_void,
            BPF_ANY,
        );
        i += 1;
    }

    link = bpf_program__attach((*ctx.skel).progs.benchmark);
    if link.is_null() {
        fprintf(stderr, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn setup() {
    hashmap_lookup_setup(bpf_map_type::BPF_MAP_TYPE_HASH);
}

unsafe extern "C" fn rhash_setup() {
    hashmap_lookup_setup(bpf_map_type::BPF_MAP_TYPE_RHASH);
}

#[inline]
unsafe fn events_from_time(time: u64) -> f64 {
    if time != 0 {
        return args.nr_loops as f64 * 1000000000u64 as f64 / time as f64 / 1000000.0f64;
    }

    0.0
}

unsafe fn compute_events(
    times: *mut u64,
    events_mean: *mut f64,
    events_stddev: *mut f64,
    mean_time: *mut u64,
) -> c_int {
    let mut i: c_int;
    let mut n: c_int = 0;

    *events_mean = 0.0;
    *events_stddev = 0.0;
    *mean_time = 0;

    i = 0;
    while i < 32 {
        if *times.add(i as usize) == 0 {
            break;
        }
        *mean_time += *times.add(i as usize);
        *events_mean += events_from_time(*times.add(i as usize));
        n += 1;
        i += 1;
    }
    if n == 0 {
        return 0;
    }

    *mean_time /= n as u64;
    *events_mean /= n as f64;

    if n > 1 {
        i = 0;
        while i < n {
            let events_i: f64 = *events_mean - events_from_time(*times.add(i as usize));
            *events_stddev += events_i * events_i / (n - 1) as f64;
            i += 1;
        }
        *events_stddev = sqrt(*events_stddev);
    }

    n
}

unsafe extern "C" fn hashmap_report_final(_res: *mut bench_res, _res_cnt: c_int) {
    let nr_cpus: c_uint = bpf_num_possible_cpus();
    let mut events_mean: f64 = 0.0;
    let mut events_stddev: f64 = 0.0;
    let mut mean_time: u64 = 0;
    let mut i: c_int;
    let mut n: c_int;

    i = 0;
    while i < nr_cpus as c_int {
        n = compute_events(
            (*(*ctx.skel).bss).percpu_times.as_mut_ptr().add(i as usize) as *mut u64,
            &mut events_mean,
            &mut events_stddev,
            &mut mean_time,
        );
        if n == 0 {
            i += 1;
            continue;
        }

        if env.quiet {
            /* we expect only one cpu to be present */
            if env.affinity {
                printf(c"%.3lf\n".as_ptr(), events_mean);
            } else {
                printf(c"cpu%02d %.3lf\n".as_ptr(), i, events_mean);
            }
        } else {
            printf(
                c"cpu%02d: lookup %.3lfM ± %.3lfM events/sec (approximated from %d samples of ~%lums)\n"
                    .as_ptr(),
                i,
                events_mean,
                2.0f64 * events_stddev,
                n,
                (mean_time / 1000000) as c_ulong,
            );
        }

        i += 1;
    }
}

#[unsafe(no_mangle)]
pub static bench_bpf_hashmap_lookup: bench = bench {
    name: c"bpf-hashmap-lookup".as_ptr(),
    argp: &bench_hashmap_lookup_argp,
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: None,
    report_final: Some(hashmap_report_final),
};

#[unsafe(no_mangle)]
pub static bench_bpf_rhashmap_lookup: bench = bench {
    name: c"bpf-rhashmap-lookup".as_ptr(),
    argp: &bench_hashmap_lookup_argp,
    validate: Some(validate),
    setup: Some(rhash_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: None,
    report_final: Some(hashmap_report_final),
};
