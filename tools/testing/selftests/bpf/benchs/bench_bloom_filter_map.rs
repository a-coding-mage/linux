// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from C. External symbols and C library/kernel constants come from
 * the same dependencies as the original includes:
 * <argp.h>, <linux/log2.h>, <pthread.h>, "bench.h",
 * "bloom_filter_bench.skel.h", and "bpf_util.h".
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

type error_t = c_int;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const ARG_NR_ENTRIES: c_int = 3000;
const ARG_NR_HASH_FUNCS: c_int = 3001;
const ARG_VALUE_SIZE: c_int = 3002;

const UINT_MAX: c_long = c_uint::MAX as c_long;
const ARGP_ERR_UNKNOWN: error_t = -7;
const BPF_NOEXIST: __u64 = 1;
const EEXIST: c_int = 17;
const STDERR: *mut FILE = 2 as *mut FILE;

/* Values are provided by libc/kernel headers in the original C build. */
const __NR_GETPGID: c_long = 121;
const __NR_GETRANDOM: c_long = 318;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

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
pub struct pthread_mutex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_cond_t {
    _private: [u8; 0],
}

type pthread_t = c_ulong;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
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
pub struct bloom_filter_bench_maps {
    pub bloom_map: *mut bpf_map,
    pub hashmap: *mut bpf_map,
    pub array_map: *mut bpf_map,
}

#[repr(C)]
pub struct bloom_filter_bench_progs {
    pub bloom_lookup: *mut bpf_program,
    pub bloom_update: *mut bpf_program,
    pub bloom_hashmap_lookup: *mut bpf_program,
}

#[repr(C)]
pub struct bloom_filter_bench_rodata {
    pub hashmap_use_bloom: bool,
    pub count_false_hits: bool,
    pub nr_rand_bytes: c_int,
    pub hit_key: c_int,
    pub drop_key: c_int,
    pub false_hit_key: c_int,
}

#[repr(C)]
pub struct bloom_filter_bench_bss {
    pub rand_vals: *mut c_void,
    pub value_size: __u8,
    pub error: c_int,
    pub percpu_stats: *mut stat,
}

#[repr(C)]
pub struct bloom_filter_bench {
    pub maps: bloom_filter_bench_maps,
    pub progs: bloom_filter_bench_progs,
    pub rodata: *mut bloom_filter_bench_rodata,
    pub bss: *mut bloom_filter_bench_bss,
}

#[repr(C)]
pub struct env_t {
    pub consumer_cnt: c_int,
}

#[repr(C)]
pub struct bench_res {
    pub hits: c_ulong,
    pub drops: c_ulong,
    pub false_hits: c_ulong,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub argp: *const argp,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res)>,
}

unsafe extern "C" {
    static mut env: env_t;
    static mut errno: c_int;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;

    fn setup_libbpf();
    fn bpf_num_possible_cpus() -> c_uint;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    fn bpf_map__set_value_size(map: *mut bpf_map, value_size: __u32) -> c_int;
    fn bpf_map__set_key_size(map: *mut bpf_map, key_size: __u32) -> c_int;
    fn bpf_map__set_map_extra(map: *mut bpf_map, map_extra: __u64) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;

    fn bloom_filter_bench__open() -> *mut bloom_filter_bench;
    fn bloom_filter_bench__load(skel: *mut bloom_filter_bench) -> c_int;

    fn hits_drops_report_progress(res: *mut bench_res);
    fn hits_drops_report_final(res: *mut bench_res);
    fn false_hits_report_progress(res: *mut bench_res);
    fn false_hits_report_final(res: *mut bench_res);
}

#[repr(C)]
struct ctx {
    use_array_map: bool,
    use_hashmap: bool,
    hashmap_use_bloom: bool,
    count_false_hits: bool,

    skel: *mut bloom_filter_bench,

    bloom_fd: c_int,
    hashmap_fd: c_int,
    array_map_fd: c_int,

    map_done_mtx: pthread_mutex_t,
    map_done_cv: pthread_cond_t,
    map_done: bool,
    map_prepare_err: bool,

    next_map_idx: AtomicU32,
}

/* Static pthread initializers are C macros. Keep zeroed storage here to preserve
 * the file-local initializer shape; the actual Rust build is expected to map
 * these to the platform pthread initializer values.
 */
static mut ctx: ctx = ctx {
    use_array_map: false,
    use_hashmap: false,
    hashmap_use_bloom: false,
    count_false_hits: false,
    skel: ptr::null_mut(),
    bloom_fd: 0,
    hashmap_fd: 0,
    array_map_fd: 0,
    map_done_mtx: pthread_mutex_t { _private: [] },
    map_done_cv: pthread_cond_t { _private: [] },
    map_done: false,
    map_prepare_err: false,
    next_map_idx: AtomicU32::new(0),
};

#[repr(C)]
struct stat {
    stats: [__u32; 3],
}

#[repr(C)]
struct args_t {
    nr_entries: __u32,
    nr_hash_funcs: __u8,
    value_size: __u8,
}

static mut args: args_t = args_t {
    nr_entries: 1000,
    nr_hash_funcs: 3,
    value_size: 8,
};

static opts: [argp_option; 4] = [
    argp_option {
        name: c"nr_entries".as_ptr(),
        key: ARG_NR_ENTRIES,
        arg: c"NR_ENTRIES".as_ptr(),
        flags: 0,
        doc: c"Set number of expected unique entries in the bloom filter".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"nr_hash_funcs".as_ptr(),
        key: ARG_NR_HASH_FUNCS,
        arg: c"NR_HASH_FUNCS".as_ptr(),
        flags: 0,
        doc: c"Set number of hash functions in the bloom filter".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"value_size".as_ptr(),
        key: ARG_VALUE_SIZE,
        arg: c"VALUE_SIZE".as_ptr(),
        flags: 0,
        doc: c"Set value size (in bytes) of bloom filter entries".as_ptr(),
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
        ARG_NR_ENTRIES => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > UINT_MAX {
                fprintf(STDERR, c"Invalid nr_entries count.".as_ptr());
                argp_usage(state);
            }
            args.nr_entries = ret as __u32;
        }
        ARG_NR_HASH_FUNCS => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > 15 {
                fprintf(
                    STDERR,
                    c"The bloom filter must use 1 to 15 hash functions.".as_ptr(),
                );
                argp_usage(state);
            }
            args.nr_hash_funcs = ret as __u8;
        }
        ARG_VALUE_SIZE => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 2 || ret > 256 {
                fprintf(
                    STDERR,
                    c"Invalid value size. Must be between 2 and 256 bytes".as_ptr(),
                );
                argp_usage(state);
            }
            args.value_size = ret as __u8;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }

    0
}

/* exported into benchmark runner */
#[unsafe(no_mangle)]
pub static bench_bloom_map_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

unsafe extern "C" fn validate() {
    if env.consumer_cnt != 0 {
        fprintf(
            STDERR,
            c"The bloom filter benchmarks do not support consumer\n".as_ptr(),
        );
        exit(1);
    }
}

#[inline]
unsafe fn trigger_bpf_program() {
    syscall(__NR_GETPGID);
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    loop {
        trigger_bpf_program();
    }
}

unsafe extern "C" fn map_prepare_thread(_arg: *mut c_void) -> *mut c_void {
    let val_size: __u32;
    let mut i: __u32;
    let mut val: *mut c_void = ptr::null_mut();
    let mut err: c_int;

    val_size = args.value_size as __u32;
    val = malloc(val_size as usize);
    if val.is_null() {
        ctx.map_prepare_err = true;
        goto_done(val);
        return ptr::null_mut();
    }

    loop {
        i = ctx.next_map_idx.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
        if i > args.nr_entries {
            break;
        }

        loop {
            /* Populate hashmap, bloom filter map, and array map with the same
             * random values
             */
            err = syscall(__NR_GETRANDOM, val, val_size, 0) as c_int;
            if err != val_size as c_int {
                ctx.map_prepare_err = true;
                fprintf(STDERR, c"failed to get random value: %d\n".as_ptr(), -errno);
                break;
            }

            if ctx.use_hashmap {
                err = bpf_map_update_elem(ctx.hashmap_fd, val, val, BPF_NOEXIST);
                if err != 0 {
                    if err != -EEXIST {
                        ctx.map_prepare_err = true;
                        fprintf(
                            STDERR,
                            c"failed to add elem to hashmap: %d\n".as_ptr(),
                            -errno,
                        );
                        break;
                    }
                    continue;
                }
            }

            i = i.wrapping_sub(1);

            if ctx.use_array_map {
                err = bpf_map_update_elem(
                    ctx.array_map_fd,
                    (&i as *const __u32).cast::<c_void>(),
                    val,
                    0,
                );
                if err != 0 {
                    ctx.map_prepare_err = true;
                    fprintf(
                        STDERR,
                        c"failed to add elem to array map: %d\n".as_ptr(),
                        -errno,
                    );
                    break;
                }
            }

            if ctx.use_hashmap && !ctx.hashmap_use_bloom {
                break;
            }

            err = bpf_map_update_elem(ctx.bloom_fd, ptr::null(), val, 0);
            if err != 0 {
                ctx.map_prepare_err = true;
                fprintf(
                    STDERR,
                    c"failed to add elem to bloom filter map: %d\n".as_ptr(),
                    -errno,
                );
                break;
            }

            break;
        }

        if ctx.map_prepare_err {
            break;
        }
    }

    goto_done(val);
    ptr::null_mut()
}

unsafe fn goto_done(val: *mut c_void) {
    pthread_mutex_lock(&raw mut ctx.map_done_mtx);
    ctx.map_done = true;
    pthread_cond_signal(&raw mut ctx.map_done_cv);
    pthread_mutex_unlock(&raw mut ctx.map_done_mtx);

    if !val.is_null() {
        free(val);
    }
}

unsafe fn populate_maps() {
    let nr_cpus: c_uint = bpf_num_possible_cpus();
    let mut map_thread: pthread_t = 0;
    let mut i: c_int;
    let mut err: c_int;
    let nr_rand_bytes: c_int;

    ctx.bloom_fd = bpf_map__fd((*ctx.skel).maps.bloom_map);
    ctx.hashmap_fd = bpf_map__fd((*ctx.skel).maps.hashmap);
    ctx.array_map_fd = bpf_map__fd((*ctx.skel).maps.array_map);

    i = 0;
    while i < nr_cpus as c_int {
        err = pthread_create(
            &mut map_thread,
            ptr::null(),
            Some(map_prepare_thread),
            ptr::null_mut(),
        );
        if err != 0 {
            fprintf(STDERR, c"failed to create pthread: %d\n".as_ptr(), -errno);
            exit(1);
        }
        i += 1;
    }

    pthread_mutex_lock(&raw mut ctx.map_done_mtx);
    while !ctx.map_done {
        pthread_cond_wait(&raw mut ctx.map_done_cv, &raw mut ctx.map_done_mtx);
    }
    pthread_mutex_unlock(&raw mut ctx.map_done_mtx);

    if ctx.map_prepare_err {
        exit(1);
    }

    nr_rand_bytes = syscall(
        __NR_GETRANDOM,
        (*(*ctx.skel).bss).rand_vals,
        (*(*ctx.skel).rodata).nr_rand_bytes,
        0,
    ) as c_int;
    if nr_rand_bytes != (*(*ctx.skel).rodata).nr_rand_bytes {
        fprintf(STDERR, c"failed to get random bytes\n".as_ptr());
        exit(1);
    }
}

unsafe fn check_args() {
    if args.value_size < 8 {
        let nr_unique_entries: __u64 = 1_u64 << (args.value_size * 8);

        if (args.nr_entries as __u64) > nr_unique_entries {
            fprintf(
                STDERR,
                c"Not enough unique values for the nr_entries requested\n".as_ptr(),
            );
            exit(1);
        }
    }
}

unsafe fn setup_skeleton() -> *mut bloom_filter_bench {
    let skel: *mut bloom_filter_bench;

    check_args();

    setup_libbpf();

    skel = bloom_filter_bench__open();
    if skel.is_null() {
        fprintf(STDERR, c"failed to open skeleton\n".as_ptr());
        exit(1);
    }

    (*(*skel).rodata).hashmap_use_bloom = ctx.hashmap_use_bloom;
    (*(*skel).rodata).count_false_hits = ctx.count_false_hits;

    /* Resize number of entries */
    bpf_map__set_max_entries((*skel).maps.hashmap, args.nr_entries);

    bpf_map__set_max_entries((*skel).maps.array_map, args.nr_entries);

    bpf_map__set_max_entries((*skel).maps.bloom_map, args.nr_entries);

    /* Set value size */
    bpf_map__set_value_size((*skel).maps.array_map, args.value_size as __u32);

    bpf_map__set_value_size((*skel).maps.bloom_map, args.value_size as __u32);

    bpf_map__set_value_size((*skel).maps.hashmap, args.value_size as __u32);

    /* For the hashmap, we use the value as the key as well */
    bpf_map__set_key_size((*skel).maps.hashmap, args.value_size as __u32);

    (*(*skel).bss).value_size = args.value_size;

    /* Set number of hash functions */
    bpf_map__set_map_extra((*skel).maps.bloom_map, args.nr_hash_funcs as __u64);

    if bloom_filter_bench__load(skel) != 0 {
        fprintf(STDERR, c"failed to load skeleton\n".as_ptr());
        exit(1);
    }

    skel
}

unsafe extern "C" fn bloom_lookup_setup() {
    let link: *mut bpf_link;

    ctx.use_array_map = true;

    ctx.skel = setup_skeleton();

    populate_maps();

    link = bpf_program__attach((*ctx.skel).progs.bloom_lookup);
    if link.is_null() {
        fprintf(STDERR, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn bloom_update_setup() {
    let link: *mut bpf_link;

    ctx.use_array_map = true;

    ctx.skel = setup_skeleton();

    populate_maps();

    link = bpf_program__attach((*ctx.skel).progs.bloom_update);
    if link.is_null() {
        fprintf(STDERR, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn false_positive_setup() {
    let link: *mut bpf_link;

    ctx.use_hashmap = true;
    ctx.hashmap_use_bloom = true;
    ctx.count_false_hits = true;

    ctx.skel = setup_skeleton();

    populate_maps();

    link = bpf_program__attach((*ctx.skel).progs.bloom_hashmap_lookup);
    if link.is_null() {
        fprintf(STDERR, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn hashmap_with_bloom_setup() {
    let link: *mut bpf_link;

    ctx.use_hashmap = true;
    ctx.hashmap_use_bloom = true;

    ctx.skel = setup_skeleton();

    populate_maps();

    link = bpf_program__attach((*ctx.skel).progs.bloom_hashmap_lookup);
    if link.is_null() {
        fprintf(STDERR, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn hashmap_no_bloom_setup() {
    let link: *mut bpf_link;

    ctx.use_hashmap = true;

    ctx.skel = setup_skeleton();

    populate_maps();

    link = bpf_program__attach((*ctx.skel).progs.bloom_hashmap_lookup);
    if link.is_null() {
        fprintf(STDERR, c"failed to attach program!\n".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    let mut total_hits: c_ulong = 0;
    let mut total_drops: c_ulong = 0;
    let mut total_false_hits: c_ulong = 0;
    static mut LAST_HITS: c_ulong = 0;
    static mut LAST_DROPS: c_ulong = 0;
    static mut LAST_FALSE_HITS: c_ulong = 0;
    let nr_cpus: c_uint = bpf_num_possible_cpus();
    let hit_key: c_int;
    let drop_key: c_int;
    let false_hit_key: c_int;
    let mut i: c_int;

    hit_key = (*(*ctx.skel).rodata).hit_key;
    drop_key = (*(*ctx.skel).rodata).drop_key;
    false_hit_key = (*(*ctx.skel).rodata).false_hit_key;

    if (*(*ctx.skel).bss).error != 0 {
        fprintf(
            STDERR,
            c"error (%d) when searching the bloom filter\n".as_ptr(),
            (*(*ctx.skel).bss).error,
        );
        exit(1);
    }

    i = 0;
    while i < nr_cpus as c_int {
        let s: *mut stat = (*(*ctx.skel).bss).percpu_stats.add(i as usize);

        total_hits = total_hits.wrapping_add((*s).stats[hit_key as usize] as c_ulong);
        total_drops = total_drops.wrapping_add((*s).stats[drop_key as usize] as c_ulong);
        total_false_hits =
            total_false_hits.wrapping_add((*s).stats[false_hit_key as usize] as c_ulong);
        i += 1;
    }

    (*res).hits = total_hits.wrapping_sub(LAST_HITS);
    (*res).drops = total_drops.wrapping_sub(LAST_DROPS);
    (*res).false_hits = total_false_hits.wrapping_sub(LAST_FALSE_HITS);

    LAST_HITS = total_hits;
    LAST_DROPS = total_drops;
    LAST_FALSE_HITS = total_false_hits;
}

#[unsafe(no_mangle)]
pub static bench_bloom_lookup: bench = bench {
    name: c"bloom-lookup".as_ptr(),
    argp: &bench_bloom_map_argp,
    validate: Some(validate),
    setup: Some(bloom_lookup_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_bloom_update: bench = bench {
    name: c"bloom-update".as_ptr(),
    argp: &bench_bloom_map_argp,
    validate: Some(validate),
    setup: Some(bloom_update_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_bloom_false_positive: bench = bench {
    name: c"bloom-false-positive".as_ptr(),
    argp: &bench_bloom_map_argp,
    validate: Some(validate),
    setup: Some(false_positive_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(false_hits_report_progress),
    report_final: Some(false_hits_report_final),
};

#[unsafe(no_mangle)]
pub static bench_hashmap_without_bloom: bench = bench {
    name: c"hashmap-without-bloom".as_ptr(),
    argp: &bench_bloom_map_argp,
    validate: Some(validate),
    setup: Some(hashmap_no_bloom_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};

#[unsafe(no_mangle)]
pub static bench_hashmap_with_bloom: bench = bench {
    name: c"hashmap-with-bloom".as_ptr(),
    argp: &bench_bloom_map_argp,
    validate: Some(validate),
    setup: Some(hashmap_with_bloom_setup),
    producer_thread: Some(producer),
    measure: Some(measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
};
