// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Cloudflare */

/*
 * All of these benchmarks operate on tries with keys in the range
 * [0, args.nr_entries), i.e. there are no gaps or partially filled
 * branches of the trie for any key < args.nr_entries.
 *
 * This gives an idea of worst-case behaviour.
 */

/* Dependencies translated from:
 * <argp.h>, <linux/time64.h>, <linux/if_ether.h>,
 * "lpm_trie_bench.skel.h", "lpm_trie_map.skel.h", "bench.h",
 * "testing_helpers.h", and "progs/lpm_trie.h".
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_void};
use core::ptr;

type __u32 = u32;
type error_t = c_int;

const UINT_MAX: c_long = c_uint::MAX as c_long;
const ARGP_ERR_UNKNOWN: error_t = 7;
const ETH_HLEN: usize = 14;
const NSEC_PER_SEC: c_long = 1_000_000_000;

const ARG_NR_ENTRIES: c_int = 9000;
const ARG_PREFIX_LEN: c_int = ARG_NR_ENTRIES + 1;
const ARG_RANDOM: c_int = ARG_PREFIX_LEN + 1;

const LPM_OP_NOOP: c_int = 0;
const LPM_OP_BASELINE: c_int = 1;
const LPM_OP_LOOKUP: c_int = 2;
const LPM_OP_INSERT: c_int = 3;
const LPM_OP_UPDATE: c_int = 4;
const LPM_OP_DELETE: c_int = 5;
const LPM_OP_FREE: c_int = 6;

const LPM_BENCH_SUCCESS: c_uint = 0;
const LPM_BENCH_REINIT_MAP: c_uint = 1;

#[repr(C)]
struct ctx {
    bench: *mut lpm_trie_bench,
}

static mut CTX: ctx = ctx {
    bench: ptr::null_mut(),
};

#[repr(C)]
struct args {
    nr_entries: __u32,
    prefixlen: __u32,
    random: bool,
}

static mut ARGS: args = args {
    nr_entries: 0,
    prefixlen: 32,
    random: false,
};

#[repr(C)]
struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
}

#[repr(C)]
struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
struct trie_key {
    prefixlen: __u32,
    data: __u32,
}

#[repr(C)]
struct bpf_map_batch_opts {
    sz: usize,
    elem_flags: u64,
    flags: u64,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    data_in: *mut c_void,
    data_size_in: u32,
    repeat: u32,
    retval: u32,
}

#[repr(C)]
struct bench_res {
    hits: f64,
    duration_ns: f64,
}

#[repr(C)]
struct bench {
    name: *const c_char,
    argp: *const argp,
    validate: Option<unsafe extern "C" fn()>,
    setup: Option<unsafe extern "C" fn()>,
    producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    report_progress: Option<unsafe extern "C" fn(c_int, *mut bench_res, c_long)>,
    report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

#[repr(C)]
struct bench_env {
    consumer_cnt: c_int,
    producer_cnt: c_int,
}

#[repr(C)]
struct lpm_trie_bench_bss {
    nr_entries: __u32,
    prefixlen: __u32,
    random: bool,
    op: c_int,
    hits: f64,
    duration_ns: f64,
}

#[repr(C)]
struct lpm_trie_bench_maps {
    trie_map: *mut bpf_map,
}

#[repr(C)]
struct lpm_trie_bench_progs {
    run_bench: *mut bpf_program,
}

#[repr(C)]
struct lpm_trie_bench {
    bss: *mut lpm_trie_bench_bss,
    maps: lpm_trie_bench_maps,
    progs: lpm_trie_bench_progs,
}

#[repr(C)]
struct lpm_trie_map_maps {
    trie_free_map: *mut bpf_map,
}

#[repr(C)]
struct lpm_trie_map {
    maps: lpm_trie_map_maps,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut env: bench_env;
    static mut stderr: *mut c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn argp_usage(state: *mut argp_state);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn sqrt(x: c_double) -> c_double;
    fn atomic_swap(ptr: *mut f64, val: f64) -> f64;

    fn bpf_map_update_batch(
        fd: c_int,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map_delete_batch(
        fd: c_int,
        keys: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn lpm_trie_bench__open_and_load() -> *mut lpm_trie_bench;
    fn lpm_trie_bench__attach(skel: *mut lpm_trie_bench) -> c_int;
    fn lpm_trie_map__open_and_load() -> *mut lpm_trie_map;
    fn lpm_trie_map__destroy(skel: *mut lpm_trie_map);

    fn ops_report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long);
    fn ops_report_final(res: *mut bench_res, res_cnt: c_int);
}

static OPTS: [argp_option; 4] = [
    argp_option {
        name: c"nr_entries".as_ptr(),
        key: ARG_NR_ENTRIES,
        arg: c"NR_ENTRIES".as_ptr(),
        flags: 0,
        doc: c"Number of unique entries in the LPM trie".as_ptr(),
    },
    argp_option {
        name: c"prefix_len".as_ptr(),
        key: ARG_PREFIX_LEN,
        arg: c"PREFIX_LEN".as_ptr(),
        flags: 0,
        doc: c"Number of prefix bits to use in the LPM trie".as_ptr(),
    },
    argp_option {
        name: c"random".as_ptr(),
        key: ARG_RANDOM,
        arg: ptr::null(),
        flags: 0,
        doc: c"Access random keys during op".as_ptr(),
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
    },
];

unsafe extern "C" fn lpm_parse_arg(
    key: c_int,
    arg: *mut c_char,
    state: *mut argp_state,
) -> error_t {
    let ret: c_long;

    match key {
        ARG_NR_ENTRIES => {
            ret = unsafe { strtol(arg, ptr::null_mut(), 10) };
            if ret < 1 || ret > UINT_MAX {
                unsafe {
                    fprintf(stderr, c"Invalid nr_entries count.".as_ptr());
                    argp_usage(state);
                }
            }
            unsafe {
                ARGS.nr_entries = ret as __u32;
            }
        }
        ARG_PREFIX_LEN => {
            ret = unsafe { strtol(arg, ptr::null_mut(), 10) };
            if ret < 1 || ret > UINT_MAX {
                unsafe {
                    fprintf(stderr, c"Invalid prefix_len value.".as_ptr());
                    argp_usage(state);
                }
            }
            unsafe {
                ARGS.prefixlen = ret as __u32;
            }
        }
        ARG_RANDOM => unsafe {
            ARGS.random = true;
        },
        _ => return ARGP_ERR_UNKNOWN,
    }
    0
}

#[unsafe(no_mangle)]
pub static bench_lpm_trie_map_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(lpm_parse_arg),
};

unsafe extern "C" fn validate_common() {
    unsafe {
        if env.consumer_cnt != 0 {
            fprintf(stderr, c"benchmark doesn't support consumer\n".as_ptr());
            exit(1);
        }

        if ARGS.nr_entries == 0 {
            fprintf(stderr, c"Missing --nr_entries parameter\n".as_ptr());
            exit(1);
        }

        if (1usize << ARGS.prefixlen) < ARGS.nr_entries as usize {
            fprintf(stderr, c"prefix_len value too small for nr_entries\n".as_ptr());
            exit(1);
        }
    }
}

unsafe extern "C" fn lpm_insert_validate() {
    unsafe {
        validate_common();

        if env.producer_cnt != 1 {
            fprintf(stderr, c"lpm-trie-insert requires a single producer\n".as_ptr());
            exit(1);
        }

        if ARGS.random {
            fprintf(stderr, c"lpm-trie-insert does not support --random\n".as_ptr());
            exit(1);
        }
    }
}

unsafe extern "C" fn lpm_delete_validate() {
    unsafe {
        validate_common();

        if env.producer_cnt != 1 {
            fprintf(stderr, c"lpm-trie-delete requires a single producer\n".as_ptr());
            exit(1);
        }

        if ARGS.random {
            fprintf(stderr, c"lpm-trie-delete does not support --random\n".as_ptr());
            exit(1);
        }
    }
}

unsafe extern "C" fn lpm_free_validate() {
    unsafe {
        validate_common();

        if env.producer_cnt != 1 {
            fprintf(stderr, c"lpm-trie-free requires a single producer\n".as_ptr());
            exit(1);
        }

        if ARGS.random {
            fprintf(stderr, c"lpm-trie-free does not support --random\n".as_ptr());
            exit(1);
        }
    }
}

static mut keys: *mut trie_key = ptr::null_mut();
static mut vals: *mut __u32 = ptr::null_mut();

unsafe fn fill_map(map_fd: c_int) {
    let err: c_int;

    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    unsafe {
        err = bpf_map_update_batch(
            map_fd,
            keys as *mut c_void,
            vals as *mut c_void,
            &mut ARGS.nr_entries,
            &opts,
        );
        if err != 0 {
            fprintf(
                stderr,
                c"failed to batch update keys to map: %d\n".as_ptr(),
                -err,
            );
            exit(1);
        }
    }
}

unsafe fn empty_map(map_fd: c_int) {
    let err: c_int;

    let opts = bpf_map_batch_opts {
        sz: core::mem::size_of::<bpf_map_batch_opts>(),
        elem_flags: 0,
        flags: 0,
    };

    unsafe {
        err = bpf_map_delete_batch(map_fd, keys as *mut c_void, &mut ARGS.nr_entries, &opts);
        if err != 0 {
            fprintf(
                stderr,
                c"failed to batch delete keys for map: %d\n".as_ptr(),
                -err,
            );
            exit(1);
        }
    }
}

unsafe extern "C" fn attach_prog() {
    let mut i: c_int;

    unsafe {
        CTX.bench = lpm_trie_bench__open_and_load();
        if CTX.bench.is_null() {
            fprintf(stderr, c"failed to open skeleton\n".as_ptr());
            exit(1);
        }

        (*(*CTX.bench).bss).nr_entries = ARGS.nr_entries;
        (*(*CTX.bench).bss).prefixlen = ARGS.prefixlen;
        (*(*CTX.bench).bss).random = ARGS.random;

        if lpm_trie_bench__attach(CTX.bench) != 0 {
            fprintf(stderr, c"failed to attach skeleton\n".as_ptr());
            exit(1);
        }

        keys = calloc(ARGS.nr_entries as usize, core::mem::size_of::<trie_key>()) as *mut trie_key;
        vals = calloc(ARGS.nr_entries as usize, core::mem::size_of::<__u32>()) as *mut __u32;

        i = 0;
        while i < ARGS.nr_entries as c_int {
            let k: *mut trie_key = keys.add(i as usize);
            let v: *mut __u32 = vals.add(i as usize);

            (*k).prefixlen = ARGS.prefixlen;
            (*k).data = i as __u32;
            *v = 1;
            i += 1;
        }
    }
}

unsafe fn attach_prog_and_fill_map() {
    let fd: c_int;

    unsafe {
        attach_prog();

        fd = bpf_map__fd((*CTX.bench).maps.trie_map);
        fill_map(fd);
    }
}

unsafe extern "C" fn lpm_noop_setup() {
    unsafe {
        attach_prog();
        (*(*CTX.bench).bss).op = LPM_OP_NOOP;
    }
}

unsafe extern "C" fn lpm_baseline_setup() {
    unsafe {
        attach_prog();
        (*(*CTX.bench).bss).op = LPM_OP_BASELINE;
    }
}

unsafe extern "C" fn lpm_lookup_setup() {
    unsafe {
        attach_prog_and_fill_map();
        (*(*CTX.bench).bss).op = LPM_OP_LOOKUP;
    }
}

unsafe extern "C" fn lpm_insert_setup() {
    unsafe {
        attach_prog();
        (*(*CTX.bench).bss).op = LPM_OP_INSERT;
    }
}

unsafe extern "C" fn lpm_update_setup() {
    unsafe {
        attach_prog_and_fill_map();
        (*(*CTX.bench).bss).op = LPM_OP_UPDATE;
    }
}

unsafe extern "C" fn lpm_delete_setup() {
    unsafe {
        attach_prog_and_fill_map();
        (*(*CTX.bench).bss).op = LPM_OP_DELETE;
    }
}

unsafe extern "C" fn lpm_free_setup() {
    unsafe {
        attach_prog();
        (*(*CTX.bench).bss).op = LPM_OP_FREE;
    }
}

unsafe extern "C" fn lpm_measure(res: *mut bench_res) {
    unsafe {
        (*res).hits = atomic_swap(&mut (*(*CTX.bench).bss).hits, 0.0);
        (*res).duration_ns = atomic_swap(&mut (*(*CTX.bench).bss).duration_ns, 0.0);
    }
}

unsafe fn bench_reinit_map() {
    let fd: c_int;

    unsafe {
        fd = bpf_map__fd((*CTX.bench).maps.trie_map);

        match (*(*CTX.bench).bss).op {
            LPM_OP_INSERT => {
                /* trie_map needs to be emptied */
                empty_map(fd);
            }
            LPM_OP_DELETE => {
                /* trie_map needs to be refilled */
                fill_map(fd);
            }
            _ => {
                fprintf(
                    stderr,
                    c"Unexpected REINIT return code for op %d\n".as_ptr(),
                    (*(*CTX.bench).bss).op,
                );
                exit(1);
            }
        }
    }
}

/* For NOOP, BASELINE, LOOKUP, INSERT, UPDATE, and DELETE */
unsafe extern "C" fn lpm_producer(_unused: *mut c_void) -> *mut c_void {
    let mut err: c_int;
    let mut in_: [c_char; ETH_HLEN] = [0; ETH_HLEN]; /* unused */

    let mut opts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: in_.as_mut_ptr() as *mut c_void,
        data_size_in: core::mem::size_of_val(&in_) as u32,
        repeat: 1,
        retval: 0,
    };

    loop {
        unsafe {
            let fd: c_int = bpf_program__fd((*CTX.bench).progs.run_bench);
            err = bpf_prog_test_run_opts(fd, &mut opts);
            if err != 0 {
                fprintf(stderr, c"failed to run BPF prog: %d\n".as_ptr(), err);
                exit(1);
            }

            /* Check for kernel error code */
            if (opts.retval as c_int) < 0 {
                fprintf(
                    stderr,
                    c"BPF prog returned error: %d\n".as_ptr(),
                    opts.retval,
                );
                exit(1);
            }

            match opts.retval {
                LPM_BENCH_SUCCESS => {}
                LPM_BENCH_REINIT_MAP => {
                    bench_reinit_map();
                }
                _ => {
                    fprintf(
                        stderr,
                        c"Unexpected BPF prog return code %d for op %d\n".as_ptr(),
                        opts.retval,
                        (*(*CTX.bench).bss).op,
                    );
                    exit(1);
                }
            }
        }
    }
}

unsafe extern "C" fn lpm_free_producer(_unused: *mut c_void) -> *mut c_void {
    loop {
        let skel: *mut lpm_trie_map;

        unsafe {
            skel = lpm_trie_map__open_and_load();
            if skel.is_null() {
                fprintf(stderr, c"failed to open skeleton\n".as_ptr());
                exit(1);
            }

            fill_map(bpf_map__fd((*skel).maps.trie_free_map));
            lpm_trie_map__destroy(skel);
        }
    }
}

/*
 * The standard bench op_report_*() functions assume measurements are
 * taken over a 1-second interval but operations that modify the map
 * (INSERT, DELETE, and FREE) cannot run indefinitely without
 * "resetting" the map to the initial state. Depending on the size of
 * the map, this likely needs to happen before the 1-second timer fires.
 *
 * Calculate the fraction of a second over which the op measurement was
 * taken (to ignore any time spent doing the reset) and report the
 * throughput results per second.
 */
unsafe fn frac_second_report_progress(
    iter: c_int,
    res: *mut bench_res,
    delta_ns: c_long,
    rate_divisor: c_double,
    rate: c_char,
) {
    let hits_per_sec: c_double;
    let hits_per_prod: c_double;

    unsafe {
        hits_per_sec = (*res).hits / rate_divisor / ((*res).duration_ns / NSEC_PER_SEC as c_double);
        hits_per_prod = hits_per_sec / env.producer_cnt as c_double;

        printf(
            c"Iter %3d (%7.3lfus): ".as_ptr(),
            iter,
            (delta_ns - NSEC_PER_SEC) as c_double / 1000.0,
        );
        printf(
            c"hits %8.3lf%c/s (%7.3lf%c/prod)\n".as_ptr(),
            hits_per_sec,
            rate as c_int,
            hits_per_prod,
            rate as c_int,
        );
    }
}

unsafe fn frac_second_report_final(
    res: *mut bench_res,
    res_cnt: c_int,
    lat_divisor: c_double,
    rate_divisor: c_double,
    rate: c_char,
    unit: *const c_char,
) {
    let mut hits_mean: c_double = 0.0;
    let mut hits_stddev: c_double = 0.0;
    let mut latency: c_double = 0.0;
    let mut i: c_int;

    unsafe {
        i = 0;
        while i < res_cnt {
            let r = res.add(i as usize);
            let val: c_double =
                (*r).hits / rate_divisor / ((*r).duration_ns / NSEC_PER_SEC as c_double);
            hits_mean += val / (0.0 + res_cnt as c_double);
            latency += (*r).duration_ns / (*r).hits / (0.0 + res_cnt as c_double);
            i += 1;
        }

        if res_cnt > 1 {
            i = 0;
            while i < res_cnt {
                let r = res.add(i as usize);
                let val: c_double =
                    (*r).hits / rate_divisor / ((*r).duration_ns / NSEC_PER_SEC as c_double);
                hits_stddev += (hits_mean - val) * (hits_mean - val) / (res_cnt as c_double - 1.0);
                i += 1;
            }

            hits_stddev = sqrt(hits_stddev);
        }
        printf(
            c"Summary: throughput %8.3lf \u{00B1} %5.3lf %c ops/s (%7.3lf%c ops/prod), ".as_ptr(),
            hits_mean,
            hits_stddev,
            rate as c_int,
            hits_mean / env.producer_cnt as c_double,
            rate as c_int,
        );
        printf(
            c"latency %8.3lf %s/op\n".as_ptr(),
            latency / lat_divisor / env.producer_cnt as c_double,
            unit,
        );
    }
}

unsafe extern "C" fn insert_ops_report_progress(
    iter: c_int,
    res: *mut bench_res,
    delta_ns: c_long,
) {
    let rate_divisor: c_double = 1000000.0;
    let rate: c_char = b'M' as c_char;

    unsafe {
        frac_second_report_progress(iter, res, delta_ns, rate_divisor, rate);
    }
}

unsafe extern "C" fn delete_ops_report_progress(
    iter: c_int,
    res: *mut bench_res,
    delta_ns: c_long,
) {
    let rate_divisor: c_double = 1000000.0;
    let rate: c_char = b'M' as c_char;

    unsafe {
        frac_second_report_progress(iter, res, delta_ns, rate_divisor, rate);
    }
}

unsafe extern "C" fn free_ops_report_progress(
    iter: c_int,
    res: *mut bench_res,
    delta_ns: c_long,
) {
    let rate_divisor: c_double = 1000.0;
    let rate: c_char = b'K' as c_char;

    unsafe {
        frac_second_report_progress(iter, res, delta_ns, rate_divisor, rate);
    }
}

unsafe extern "C" fn insert_ops_report_final(res: *mut bench_res, res_cnt: c_int) {
    let lat_divisor: c_double = 1.0;
    let rate_divisor: c_double = 1000000.0;
    let unit: *const c_char = c"ns".as_ptr();
    let rate: c_char = b'M' as c_char;

    unsafe {
        frac_second_report_final(res, res_cnt, lat_divisor, rate_divisor, rate, unit);
    }
}

unsafe extern "C" fn delete_ops_report_final(res: *mut bench_res, res_cnt: c_int) {
    let lat_divisor: c_double = 1.0;
    let rate_divisor: c_double = 1000000.0;
    let unit: *const c_char = c"ns".as_ptr();
    let rate: c_char = b'M' as c_char;

    unsafe {
        frac_second_report_final(res, res_cnt, lat_divisor, rate_divisor, rate, unit);
    }
}

unsafe extern "C" fn free_ops_report_final(res: *mut bench_res, res_cnt: c_int) {
    let lat_divisor: c_double = 1000000.0;
    let rate_divisor: c_double = 1000.0;
    let unit: *const c_char = c"ms".as_ptr();
    let rate: c_char = b'K' as c_char;

    unsafe {
        frac_second_report_final(res, res_cnt, lat_divisor, rate_divisor, rate, unit);
    }
}

/* noop bench measures harness-overhead */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_noop: bench = bench {
    name: c"lpm-trie-noop".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(validate_common),
    setup: Some(lpm_noop_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(ops_report_progress),
    report_final: Some(ops_report_final),
};

/* baseline overhead for lookup and update */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_baseline: bench = bench {
    name: c"lpm-trie-baseline".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(validate_common),
    setup: Some(lpm_baseline_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(ops_report_progress),
    report_final: Some(ops_report_final),
};

/* measure cost of doing a lookup on existing entries in a full trie */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_lookup: bench = bench {
    name: c"lpm-trie-lookup".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(validate_common),
    setup: Some(lpm_lookup_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(ops_report_progress),
    report_final: Some(ops_report_final),
};

/* measure cost of inserting new entries into an empty trie */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_insert: bench = bench {
    name: c"lpm-trie-insert".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(lpm_insert_validate),
    setup: Some(lpm_insert_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(insert_ops_report_progress),
    report_final: Some(insert_ops_report_final),
};

/* measure cost of updating existing entries in a full trie */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_update: bench = bench {
    name: c"lpm-trie-update".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(validate_common),
    setup: Some(lpm_update_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(ops_report_progress),
    report_final: Some(ops_report_final),
};

/* measure cost of deleting existing entries from a full trie */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_delete: bench = bench {
    name: c"lpm-trie-delete".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(lpm_delete_validate),
    setup: Some(lpm_delete_setup),
    producer_thread: Some(lpm_producer),
    measure: Some(lpm_measure),
    report_progress: Some(delete_ops_report_progress),
    report_final: Some(delete_ops_report_final),
};

/* measure cost of freeing a full trie */
#[unsafe(no_mangle)]
pub static bench_lpm_trie_free: bench = bench {
    name: c"lpm-trie-free".as_ptr(),
    argp: &bench_lpm_trie_map_argp,
    validate: Some(lpm_free_validate),
    setup: Some(lpm_free_setup),
    producer_thread: Some(lpm_free_producer),
    measure: Some(lpm_measure),
    report_progress: Some(free_ops_report_progress),
    report_final: Some(free_ops_report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
