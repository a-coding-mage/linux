// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Translated from C. External symbols are supplied by the surrounding benchmark,
 * libbpf, libc, and generated BPF skeleton bindings.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type error_t = c_int;
type __u32 = u32;
type __u64 = u64;

const ARG_RB_BACK2BACK: c_int = 2000;
const ARG_RB_USE_OUTPUT: c_int = 2001;
const ARG_RB_BATCH_CNT: c_int = 2002;
const ARG_RB_SAMPLED: c_int = 2003;
const ARG_RB_SAMPLE_RATE: c_int = 2004;
const ARG_RB_OVERWRITE: c_int = 2005;
const ARG_RB_BENCH_PRODUCER: c_int = 2006;

const ARGP_ERR_UNKNOWN: error_t = 7;
const EPOLL_CLOEXEC: c_int = 0x80000;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLLIN: u32 = 0x001;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const PERF_COUNT_SW_BPF_OUTPUT: __u64 = 10;
const PERF_TYPE_SOFTWARE: __u32 = 1;
const PERF_SAMPLE_RAW: __u64 = 1 << 10;
const PERF_RECORD_SAMPLE: __u32 = 9;
const PERF_RECORD_LOST: __u32 = 2;
const LIBBPF_PERF_EVENT_ERROR: bpf_perf_event_ret = -1;
const LIBBPF_PERF_EVENT_CONT: bpf_perf_event_ret = 0;
const BPF_F_RB_OVERWRITE: __u32 = 1 << 0;
const __NR_getpgid: c_long = 121;

const RINGBUF_BUSY_BIT: c_int = 1 << 31;
const RINGBUF_DISCARD_BIT: c_int = 1 << 30;
const RINGBUF_META_LEN: __u32 = 8;

#[repr(C)]
struct args_t {
    back2back: bool,
    batch_cnt: c_int,
    sampled: bool,
    sample_rate: c_int,
    ringbuf_sz: c_int, /* per-ringbuf, in bytes */
    ringbuf_use_output: bool, /* use slower output API */
    perfbuf_sz: c_int, /* per-CPU size, in pages */
    overwrite: bool,
    bench_producer: bool,
}

static mut args: args_t = args_t {
    back2back: false,
    batch_cnt: 500,
    sampled: false,
    sample_rate: 500,
    ringbuf_sz: 512 * 1024,
    ringbuf_use_output: false,
    perfbuf_sz: 128,
    overwrite: false,
    bench_producer: false,
};

#[repr(C)]
struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
    group: c_int,
}

#[repr(C)]
struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
struct counter {
    value: c_long,
}

#[repr(C)]
struct bench_res {
    hits: c_long,
    drops: c_long,
}

#[repr(C)]
struct bench {
    name: *const c_char,
    argp: *const argp,
    validate: Option<unsafe extern "C" fn()>,
    setup: Option<unsafe extern "C" fn()>,
    producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    consumer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    report_progress: *const c_void,
    report_final: *const c_void,
}

#[repr(C)]
struct bench_env {
    bench_name: *const c_char,
    consumer_cnt: c_int,
    producer_cnt: c_int,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct ring_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct ringbuf_bench_maps {
    ringbuf: *mut bpf_map,
}

#[repr(C)]
struct ringbuf_bench_progs {
    bench_ringbuf: *mut bpf_program,
}

#[repr(C)]
struct ringbuf_bench_bss {
    hits: c_long,
    dropped: c_long,
}

#[repr(C)]
struct ringbuf_bench_rodata {
    batch_cnt: c_int,
    use_output: c_int,
    bench_producer: bool,
    wakeup_data_size: c_int,
}

#[repr(C)]
struct ringbuf_bench {
    maps: ringbuf_bench_maps,
    progs: ringbuf_bench_progs,
    bss: *mut ringbuf_bench_bss,
    rodata: *mut ringbuf_bench_rodata,
}

#[repr(C)]
struct perfbuf_bench_maps {
    perfbuf: *mut bpf_map,
}

#[repr(C)]
struct perfbuf_bench_progs {
    bench_perfbuf: *mut bpf_program,
}

#[repr(C)]
struct perfbuf_bench_bss {
    dropped: c_long,
}

#[repr(C)]
struct perfbuf_bench_rodata {
    batch_cnt: c_int,
}

#[repr(C)]
struct perfbuf_bench {
    maps: perfbuf_bench_maps,
    progs: perfbuf_bench_progs,
    bss: *mut perfbuf_bench_bss,
    rodata: *mut perfbuf_bench_rodata,
}

#[repr(C)]
struct epoll_data {
    ptr: *mut c_void,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct perf_event_attr {
    type_: __u32,
    size: __u32,
    config: __u64,
    sample_period: __u64,
    sample_type: __u64,
    wakeup_events: __u32,
}

#[repr(C)]
struct perf_event_header {
    type_: __u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
struct perf_event_mmap_page {
    data_head: __u64,
    data_tail: __u64,
}

type bpf_perf_event_ret = c_int;
type perf_buffer_event_fn = Option<unsafe extern "C" fn()>;
type perf_buffer_sample_fn = Option<unsafe extern "C" fn()>;
type perf_buffer_lost_fn = Option<unsafe extern "C" fn()>;

/* copies of internal libbpf definitions */
#[repr(C)]
struct perf_cpu_buf {
    pb: *mut perf_buffer,
    base: *mut c_void, /* mmap()'ed memory */
    buf: *mut c_void, /* for reconstructing segmented data */
    buf_size: size_t,
    fd: c_int,
    cpu: c_int,
    map_key: c_int,
}

#[repr(C)]
struct perf_buffer {
    event_cb: perf_buffer_event_fn,
    sample_cb: perf_buffer_sample_fn,
    lost_cb: perf_buffer_lost_fn,
    ctx: *mut c_void, /* passed into callbacks */

    page_size: size_t,
    mmap_size: size_t,
    cpu_bufs: *mut *mut perf_cpu_buf,
    events: *mut epoll_event,
    cpu_cnt: c_int, /* number of allocated CPU buffers */
    epoll_fd: c_int, /* perf event FD */
    map_fd: c_int, /* BPF_MAP_TYPE_PERF_EVENT_ARRAY BPF map FD */
}

unsafe extern "C" {
    static mut env: bench_env;
    static mut errno: c_int;
    static hits_drops_report_progress: c_void;
    static hits_drops_report_final: c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn getpagesize() -> c_int;
    fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    static MAP_FAILED: *mut c_void;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn atomic_swap(ptr: *mut c_long, val: c_long) -> c_long;
    fn atomic_inc(ptr: *mut c_long);
    fn smp_load_acquire_u64(ptr: *const __u64) -> __u64;
    fn smp_load_acquire_int(ptr: *const c_int) -> c_int;
    fn smp_store_release_u64(ptr: *mut __u64, val: __u64);
    fn ring_buffer_read_head(header: *mut perf_event_mmap_page) -> __u64;
    fn ring_buffer_write_tail(header: *mut perf_event_mmap_page, val: __u64);

    fn setup_libbpf();
    fn ringbuf_bench__open() -> *mut ringbuf_bench;
    fn ringbuf_bench__load(skel: *mut ringbuf_bench) -> c_int;
    fn perfbuf_bench__open() -> *mut perfbuf_bench;
    fn perfbuf_bench__load(skel: *mut perfbuf_bench) -> c_int;
    fn bpf_map__map_flags(map: *mut bpf_map) -> __u32;
    fn bpf_map__set_map_flags(map: *mut bpf_map, flags: __u32) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_uint) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn ring_buffer__new(
        map_fd: c_int,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, size_t) -> c_int>,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut ring_buffer;
    fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__new_raw(
        map_fd: c_int,
        page_cnt: size_t,
        attr: *mut perf_event_attr,
        event_cb: Option<
            unsafe extern "C" fn(*mut c_void, c_int, *mut perf_event_header) -> bpf_perf_event_ret,
        >,
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
}

static opts: [argp_option; 8] = [
    argp_option { name: b"rb-b2b\0".as_ptr() as *const c_char, key: ARG_RB_BACK2BACK, arg: ptr::null(), flags: 0, doc: b"Back-to-back mode\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-use-output\0".as_ptr() as *const c_char, key: ARG_RB_USE_OUTPUT, arg: ptr::null(), flags: 0, doc: b"Use bpf_ringbuf_output() instead of bpf_ringbuf_reserve()\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-batch-cnt\0".as_ptr() as *const c_char, key: ARG_RB_BATCH_CNT, arg: b"CNT\0".as_ptr() as *const c_char, flags: 0, doc: b"Set BPF-side record batch count\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-sampled\0".as_ptr() as *const c_char, key: ARG_RB_SAMPLED, arg: ptr::null(), flags: 0, doc: b"Notification sampling\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-sample-rate\0".as_ptr() as *const c_char, key: ARG_RB_SAMPLE_RATE, arg: b"RATE\0".as_ptr() as *const c_char, flags: 0, doc: b"Notification sample rate\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-overwrite\0".as_ptr() as *const c_char, key: ARG_RB_OVERWRITE, arg: ptr::null(), flags: 0, doc: b"Overwrite mode\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"rb-bench-producer\0".as_ptr() as *const c_char, key: ARG_RB_BENCH_PRODUCER, arg: ptr::null(), flags: 0, doc: b"Benchmark producer\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: ptr::null(), key: 0, arg: ptr::null(), flags: 0, doc: ptr::null(), group: 0 },
];

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    match key {
        ARG_RB_BACK2BACK => {
            args.back2back = true;
        }
        ARG_RB_USE_OUTPUT => {
            args.ringbuf_use_output = true;
        }
        ARG_RB_BATCH_CNT => {
            args.batch_cnt = strtol(arg, ptr::null_mut(), 10) as c_int;
            if args.batch_cnt < 0 {
                fprintf(stderr, b"Invalid batch count.\0".as_ptr() as *const c_char);
                argp_usage(state);
            }
        }
        ARG_RB_SAMPLED => {
            args.sampled = true;
        }
        ARG_RB_SAMPLE_RATE => {
            args.sample_rate = strtol(arg, ptr::null_mut(), 10) as c_int;
            if args.sample_rate < 0 {
                fprintf(stderr, b"Invalid perfbuf sample rate.\0".as_ptr() as *const c_char);
                argp_usage(state);
            }
        }
        ARG_RB_OVERWRITE => {
            args.overwrite = true;
        }
        ARG_RB_BENCH_PRODUCER => {
            args.bench_producer = true;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    0
}

/* exported into benchmark runner */
#[unsafe(no_mangle)]
pub static bench_ringbufs_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

/* RINGBUF-LIBBPF benchmark */

static mut buf_hits: counter = counter { value: 0 };

#[inline]
unsafe fn bufs_trigger_batch() {
    let _ = syscall(__NR_getpgid);
}

unsafe extern "C" fn bufs_validate() {
    if args.bench_producer && strcmp(env.bench_name, b"rb-libbpf\0".as_ptr() as *const c_char) != 0 {
        fprintf(stderr, b"--rb-bench-producer only works with rb-libbpf!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if args.overwrite && !args.bench_producer {
        fprintf(stderr, b"overwrite mode only works with --rb-bench-producer for now!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if args.bench_producer && env.consumer_cnt != 0 {
        fprintf(stderr, b"no consumer is needed for --rb-bench-producer!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if args.bench_producer && args.back2back {
        fprintf(stderr, b"back-to-back mode makes no sense for --rb-bench-producer!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if args.bench_producer && args.sampled {
        fprintf(stderr, b"sampling mode makes no sense for --rb-bench-producer!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if !args.bench_producer && env.consumer_cnt != 1 {
        fprintf(stderr, b"benchmarks without --rb-bench-producer require exactly one consumer!\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if args.back2back && env.producer_cnt > 1 {
        fprintf(stderr, b"back-to-back mode makes sense only for single-producer case!\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn bufs_sample_producer(_input: *mut c_void) -> *mut c_void {
    if args.back2back {
        /* initial batch to get everything started */
        bufs_trigger_batch();
        return ptr::null_mut();
    }

    loop {
        bufs_trigger_batch();
    }
}

#[repr(C)]
struct ringbuf_libbpf_ctx {
    skel: *mut ringbuf_bench,
    ringbuf: *mut ring_buffer,
}

static mut ringbuf_libbpf_ctx: ringbuf_libbpf_ctx = ringbuf_libbpf_ctx {
    skel: ptr::null_mut(),
    ringbuf: ptr::null_mut(),
};

unsafe extern "C" fn ringbuf_libbpf_measure(res: *mut bench_res) {
    let ctx = &raw mut ringbuf_libbpf_ctx;

    if args.bench_producer {
        (*res).hits = atomic_swap(&mut (*(*(*ctx).skel).bss).hits, 0);
    } else {
        (*res).hits = atomic_swap(&mut buf_hits.value, 0);
    }
    (*res).drops = atomic_swap(&mut (*(*(*ctx).skel).bss).dropped, 0);
}

unsafe fn ringbuf_setup_skeleton() -> *mut ringbuf_bench {
    let mut flags: __u32;
    let ringbuf: *mut bpf_map;
    let skel: *mut ringbuf_bench;

    setup_libbpf();

    skel = ringbuf_bench__open();
    if skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    (*(*skel).rodata).batch_cnt = args.batch_cnt;
    (*(*skel).rodata).use_output = if args.ringbuf_use_output { 1 } else { 0 };
    (*(*skel).rodata).bench_producer = args.bench_producer;

    if args.sampled {
        /* record data + header take 16 bytes */
        (*(*skel).rodata).wakeup_data_size = args.sample_rate * 16;
    }

    ringbuf = (*skel).maps.ringbuf;
    if args.overwrite {
        flags = bpf_map__map_flags(ringbuf) | BPF_F_RB_OVERWRITE;
        bpf_map__set_map_flags(ringbuf, flags);
    }

    bpf_map__set_max_entries(ringbuf, args.ringbuf_sz as c_uint);

    if ringbuf_bench__load(skel) != 0 {
        fprintf(stderr, b"failed to load skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    skel
}

unsafe extern "C" fn buf_process_sample(_ctx: *mut c_void, _data: *mut c_void, _len: size_t) -> c_int {
    atomic_inc(&mut buf_hits.value);
    0
}

unsafe extern "C" fn ringbuf_libbpf_setup() {
    let ctx = &raw mut ringbuf_libbpf_ctx;
    let link: *mut bpf_link;
    let map_fd: c_int;

    (*ctx).skel = ringbuf_setup_skeleton();

    map_fd = bpf_map__fd((*(*ctx).skel).maps.ringbuf);
    (*ctx).ringbuf = ring_buffer__new(map_fd, Some(buf_process_sample), ptr::null_mut(), ptr::null_mut());
    if (*ctx).ringbuf.is_null() {
        fprintf(stderr, b"failed to create ringbuf\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    link = bpf_program__attach((*(*ctx).skel).progs.bench_ringbuf);
    if link.is_null() {
        fprintf(stderr, b"failed to attach program!\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn ringbuf_libbpf_consumer(_input: *mut c_void) -> *mut c_void {
    let ctx = &raw mut ringbuf_libbpf_ctx;

    while ring_buffer__poll((*ctx).ringbuf, -1) >= 0 {
        if args.back2back {
            bufs_trigger_batch();
        }
    }
    fprintf(stderr, b"ringbuf polling failed!\n\0".as_ptr() as *const c_char);
    ptr::null_mut()
}

/* RINGBUF-CUSTOM benchmark */
#[repr(C)]
struct ringbuf_custom {
    consumer_pos: *mut __u64,
    producer_pos: *mut __u64,
    mask: __u64,
    data: *mut c_void,
    map_fd: c_int,
}

#[repr(C)]
struct ringbuf_custom_ctx {
    skel: *mut ringbuf_bench,
    ringbuf: ringbuf_custom,
    epoll_fd: c_int,
    event: epoll_event,
}

static mut ringbuf_custom_ctx: ringbuf_custom_ctx = ringbuf_custom_ctx {
    skel: ptr::null_mut(),
    ringbuf: ringbuf_custom {
        consumer_pos: ptr::null_mut(),
        producer_pos: ptr::null_mut(),
        mask: 0,
        data: ptr::null_mut(),
        map_fd: 0,
    },
    epoll_fd: 0,
    event: epoll_event {
        events: 0,
        data: epoll_data { ptr: ptr::null_mut() },
    },
};

unsafe extern "C" fn ringbuf_custom_measure(res: *mut bench_res) {
    let ctx = &raw mut ringbuf_custom_ctx;

    (*res).hits = atomic_swap(&mut buf_hits.value, 0);
    (*res).drops = atomic_swap(&mut (*(*(*ctx).skel).bss).dropped, 0);
}

unsafe extern "C" fn ringbuf_custom_setup() {
    let ctx = &raw mut ringbuf_custom_ctx;
    let page_size: size_t = getpagesize() as size_t;
    let link: *mut bpf_link;
    let r: *mut ringbuf_custom;
    let mut tmp: *mut c_void;
    let err: c_int;

    (*ctx).skel = ringbuf_setup_skeleton();

    (*ctx).epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if (*ctx).epoll_fd < 0 {
        fprintf(stderr, b"failed to create epoll fd: %d\n\0".as_ptr() as *const c_char, -errno);
        exit(1);
    }

    r = &raw mut (*ctx).ringbuf;
    (*r).map_fd = bpf_map__fd((*(*ctx).skel).maps.ringbuf);
    (*r).mask = (args.ringbuf_sz - 1) as __u64;

    /* Map writable consumer page */
    tmp = mmap(ptr::null_mut(), page_size, PROT_READ | PROT_WRITE, MAP_SHARED, (*r).map_fd, 0);
    if tmp == MAP_FAILED {
        fprintf(stderr, b"failed to mmap consumer page: %d\n\0".as_ptr() as *const c_char, -errno);
        exit(1);
    }
    (*r).consumer_pos = tmp as *mut __u64;

    /* Map read-only producer page and data pages. */
    tmp = mmap(ptr::null_mut(), page_size + 2 * args.ringbuf_sz as size_t, PROT_READ, MAP_SHARED, (*r).map_fd, page_size as isize);
    if tmp == MAP_FAILED {
        fprintf(stderr, b"failed to mmap data pages: %d\n\0".as_ptr() as *const c_char, -errno);
        exit(1);
    }
    (*r).producer_pos = tmp as *mut __u64;
    (*r).data = tmp.add(page_size);

    (*ctx).event.events = EPOLLIN;
    err = epoll_ctl((*ctx).epoll_fd, EPOLL_CTL_ADD, (*r).map_fd, &mut (*ctx).event);
    if err < 0 {
        fprintf(stderr, b"failed to epoll add ringbuf: %d\n\0".as_ptr() as *const c_char, -errno);
        exit(1);
    }

    link = bpf_program__attach((*(*ctx).skel).progs.bench_ringbuf);
    if link.is_null() {
        fprintf(stderr, b"failed to attach program\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

#[inline]
unsafe fn roundup_len(mut len: __u32) -> c_int {
    /* clear out top 2 bits */
    len <<= 2;
    len >>= 2;
    /* add length prefix */
    len = len.wrapping_add(RINGBUF_META_LEN);
    /* round up to 8 byte alignment */
    ((len + 7) / 8 * 8) as c_int
}

unsafe fn ringbuf_custom_process_ring(r: *mut ringbuf_custom) {
    let mut cons_pos: c_ulong;
    let mut prod_pos: c_ulong;
    let mut len_ptr: *mut c_int;
    let mut len: c_int;
    let mut got_new_data: bool;

    cons_pos = smp_load_acquire_u64((*r).consumer_pos) as c_ulong;
    loop {
        got_new_data = false;
        prod_pos = smp_load_acquire_u64((*r).producer_pos) as c_ulong;
        while cons_pos < prod_pos {
            len_ptr = ((*r).data as *mut u8).add((cons_pos as __u64 & (*r).mask) as usize) as *mut c_int;
            len = smp_load_acquire_int(len_ptr);

            /* sample not committed yet, bail out for now */
            if (len & RINGBUF_BUSY_BIT) != 0 {
                return;
            }

            got_new_data = true;
            cons_pos = cons_pos.wrapping_add(roundup_len(len as __u32) as c_ulong);

            atomic_inc(&mut buf_hits.value);
        }
        if got_new_data {
            smp_store_release_u64((*r).consumer_pos, cons_pos as __u64);
        } else {
            break;
        }
    }
}

unsafe extern "C" fn ringbuf_custom_consumer(_input: *mut c_void) -> *mut c_void {
    let ctx = &raw mut ringbuf_custom_ctx;
    let mut cnt: c_int;

    loop {
        if args.back2back {
            bufs_trigger_batch();
        }
        cnt = epoll_wait((*ctx).epoll_fd, &mut (*ctx).event, 1, -1);
        if cnt > 0 {
            ringbuf_custom_process_ring(&mut (*ctx).ringbuf);
        }
        if cnt < 0 {
            break;
        }
    }
    fprintf(stderr, b"ringbuf polling failed!\n\0".as_ptr() as *const c_char);
    ptr::null_mut()
}

/* PERFBUF-LIBBPF benchmark */
#[repr(C)]
struct perfbuf_libbpf_ctx {
    skel: *mut perfbuf_bench,
    perfbuf: *mut perf_buffer,
}

static mut perfbuf_libbpf_ctx: perfbuf_libbpf_ctx = perfbuf_libbpf_ctx {
    skel: ptr::null_mut(),
    perfbuf: ptr::null_mut(),
};

unsafe extern "C" fn perfbuf_measure(res: *mut bench_res) {
    let ctx = &raw mut perfbuf_libbpf_ctx;

    (*res).hits = atomic_swap(&mut buf_hits.value, 0);
    (*res).drops = atomic_swap(&mut (*(*(*ctx).skel).bss).dropped, 0);
}

unsafe fn perfbuf_setup_skeleton() -> *mut perfbuf_bench {
    let skel: *mut perfbuf_bench;

    setup_libbpf();

    skel = perfbuf_bench__open();
    if skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    (*(*skel).rodata).batch_cnt = args.batch_cnt;

    if perfbuf_bench__load(skel) != 0 {
        fprintf(stderr, b"failed to load skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    skel
}

unsafe extern "C" fn perfbuf_process_sample_raw(
    _input_ctx: *mut c_void,
    _cpu: c_int,
    e: *mut perf_event_header,
) -> bpf_perf_event_ret {
    match (*e).type_ {
        PERF_RECORD_SAMPLE => {
            atomic_inc(&mut buf_hits.value);
        }
        PERF_RECORD_LOST => {}
        _ => return LIBBPF_PERF_EVENT_ERROR,
    }
    LIBBPF_PERF_EVENT_CONT
}

unsafe extern "C" fn perfbuf_libbpf_setup() {
    let ctx = &raw mut perfbuf_libbpf_ctx;
    let mut attr: perf_event_attr;
    let link: *mut bpf_link;

    (*ctx).skel = perfbuf_setup_skeleton();

    attr = mem::zeroed();
    attr.config = PERF_COUNT_SW_BPF_OUTPUT;
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.sample_type = PERF_SAMPLE_RAW;
    /* notify only every Nth sample */
    if args.sampled {
        attr.sample_period = args.sample_rate as __u64;
        attr.wakeup_events = args.sample_rate as __u32;
    } else {
        attr.sample_period = 1;
        attr.wakeup_events = 1;
    }

    if args.sample_rate > args.batch_cnt {
        fprintf(
            stderr,
            b"sample rate %d is too high for given batch count %d\n\0".as_ptr() as *const c_char,
            args.sample_rate,
            args.batch_cnt,
        );
        exit(1);
    }

    (*ctx).perfbuf = perf_buffer__new_raw(
        bpf_map__fd((*(*ctx).skel).maps.perfbuf),
        args.perfbuf_sz as size_t,
        &mut attr,
        Some(perfbuf_process_sample_raw),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if (*ctx).perfbuf.is_null() {
        fprintf(stderr, b"failed to create perfbuf\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    link = bpf_program__attach((*(*ctx).skel).progs.bench_perfbuf);
    if link.is_null() {
        fprintf(stderr, b"failed to attach program\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn perfbuf_libbpf_consumer(_input: *mut c_void) -> *mut c_void {
    let ctx = &raw mut perfbuf_libbpf_ctx;

    while perf_buffer__poll((*ctx).perfbuf, -1) >= 0 {
        if args.back2back {
            bufs_trigger_batch();
        }
    }
    fprintf(stderr, b"perfbuf polling failed!\n\0".as_ptr() as *const c_char);
    ptr::null_mut()
}

/* PERFBUF-CUSTOM benchmark */

unsafe extern "C" fn perfbuf_custom_consumer(_input: *mut c_void) -> *mut c_void {
    let ctx = &raw mut perfbuf_libbpf_ctx;
    let pb: *mut perf_buffer = (*ctx).perfbuf;
    let mut cpu_buf: *mut perf_cpu_buf;
    let mut header: *mut perf_event_mmap_page;
    let mmap_mask: size_t = (*pb).mmap_size - 1;
    let mut ehdr: *mut perf_event_header;
    let mut data_head: __u64;
    let mut data_tail: __u64;
    let mut ehdr_size: size_t;
    let mut base: *mut c_void;
    let mut i: c_int;
    let mut cnt: c_int;

    loop {
        if args.back2back {
            bufs_trigger_batch();
        }
        cnt = epoll_wait((*pb).epoll_fd, (*pb).events, (*pb).cpu_cnt, -1);
        if cnt <= 0 {
            fprintf(stderr, b"perf epoll failed: %d\n\0".as_ptr() as *const c_char, -errno);
            exit(1);
        }

        i = 0;
        while i < cnt {
            cpu_buf = (*(*pb).events.add(i as usize)).data.ptr as *mut perf_cpu_buf;
            header = (*cpu_buf).base as *mut perf_event_mmap_page;
            base = (header as *mut u8).add((*pb).page_size) as *mut c_void;

            data_head = ring_buffer_read_head(header);
            data_tail = (*header).data_tail;
            while data_head != data_tail {
                ehdr = (base as *mut u8).add((data_tail as size_t) & mmap_mask) as *mut perf_event_header;
                ehdr_size = (*ehdr).size as size_t;

                if (*ehdr).type_ == PERF_RECORD_SAMPLE {
                    atomic_inc(&mut buf_hits.value);
                }

                data_tail = data_tail.wrapping_add(ehdr_size as __u64);
            }
            ring_buffer_write_tail(header, data_tail);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
pub static bench_rb_libbpf: bench = bench {
    name: b"rb-libbpf\0".as_ptr() as *const c_char,
    argp: &bench_ringbufs_argp,
    validate: Some(bufs_validate),
    setup: Some(ringbuf_libbpf_setup),
    producer_thread: Some(bufs_sample_producer),
    consumer_thread: Some(ringbuf_libbpf_consumer),
    measure: Some(ringbuf_libbpf_measure),
    report_progress: unsafe { &hits_drops_report_progress as *const c_void },
    report_final: unsafe { &hits_drops_report_final as *const c_void },
};

#[unsafe(no_mangle)]
pub static bench_rb_custom: bench = bench {
    name: b"rb-custom\0".as_ptr() as *const c_char,
    argp: &bench_ringbufs_argp,
    validate: Some(bufs_validate),
    setup: Some(ringbuf_custom_setup),
    producer_thread: Some(bufs_sample_producer),
    consumer_thread: Some(ringbuf_custom_consumer),
    measure: Some(ringbuf_custom_measure),
    report_progress: unsafe { &hits_drops_report_progress as *const c_void },
    report_final: unsafe { &hits_drops_report_final as *const c_void },
};

#[unsafe(no_mangle)]
pub static bench_pb_libbpf: bench = bench {
    name: b"pb-libbpf\0".as_ptr() as *const c_char,
    argp: &bench_ringbufs_argp,
    validate: Some(bufs_validate),
    setup: Some(perfbuf_libbpf_setup),
    producer_thread: Some(bufs_sample_producer),
    consumer_thread: Some(perfbuf_libbpf_consumer),
    measure: Some(perfbuf_measure),
    report_progress: unsafe { &hits_drops_report_progress as *const c_void },
    report_final: unsafe { &hits_drops_report_final as *const c_void },
};

#[unsafe(no_mangle)]
pub static bench_pb_custom: bench = bench {
    name: b"pb-custom\0".as_ptr() as *const c_char,
    argp: &bench_ringbufs_argp,
    validate: Some(bufs_validate),
    setup: Some(perfbuf_libbpf_setup),
    producer_thread: Some(bufs_sample_producer),
    consumer_thread: Some(perfbuf_custom_consumer),
    measure: Some(perfbuf_measure),
    report_progress: unsafe { &hits_drops_report_progress as *const c_void },
    report_final: unsafe { &hits_drops_report_final as *const c_void },
};
