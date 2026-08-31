// SPDX-License-Identifier: GPL-2.0
/*
 * mem-memcpy.c
 *
 * Simple memcpy() and memset() benchmarks
 *
 * Written by Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type SizeT = usize;
type U64 = u64;
type S64 = i64;
type PthreadT = c_ulong;

const K: f64 = 1024.0;

const PAGE_SHIFT_4KB: c_uint = 12;
const PAGE_SHIFT_2MB: c_uint = 21;
const PAGE_SHIFT_1GB: c_uint = 30;

const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const MAP_POPULATE: c_int = 0x8000;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_HUGE_SHIFT: c_int = 26;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const USEC_PER_SEC: c_long = 1_000_000;
const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;

static mut SIZE_STR: *const c_char = b"1MB\0".as_ptr() as *const c_char;
static mut FUNCTION_STR: *const c_char = b"all\0".as_ptr() as *const c_char;
static mut PAGE_SIZE_STR: *const c_char = b"4KB\0".as_ptr() as *const c_char;
static mut CHUNK_SIZE_STR: *const c_char = b"0\0".as_ptr() as *const c_char;
static mut NR_LOOPS: c_uint = 1;
static mut USE_CYCLES: bool = false;
static mut CYCLES_FD: c_int = 0;
static mut SEED: c_uint = 0;
static mut NR_THREADS: c_uint = 1;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

// Original C initialized these arrays with OPT_* parse-options macros from included headers.
// The macro expansions are external to this isolated file.
static BENCH_COMMON_OPTIONS: [option; 1] = [option { _private: [] }];
static BENCH_MEM_OPTIONS: [option; 1] = [option { _private: [] }];

#[repr(C)]
#[derive(Copy, Clone)]
pub union bench_clock {
    pub cycles: U64,
    pub tv: timeval,
}

#[repr(C)]
pub struct bench_params {
    pub size: SizeT,
    pub size_total: SizeT,
    pub chunk_size: SizeT,
    pub nr_loops: c_uint,
    pub page_shift: c_uint,
    pub seed: c_uint,
}

#[repr(C)]
pub struct bench_mem_info {
    pub functions: *const function,
    pub do_op: Option<
        unsafe extern "C" fn(
            r: *const function,
            p: *mut bench_params,
            src: *mut c_void,
            dst: *mut c_void,
            rt: *mut bench_clock,
        ) -> c_int,
    >,
    pub usage: *const *const c_char,
    pub options: *const option,
    pub alloc_src: bool,
}

type MemInitT = unsafe extern "C" fn(
    *mut bench_mem_info,
    *mut bench_params,
    *mut *mut c_void,
    *mut *mut c_void,
) -> bool;
type MemFiniT = unsafe extern "C" fn(
    *mut bench_mem_info,
    *mut bench_params,
    *mut *mut c_void,
    *mut *mut c_void,
);
type MemcpyT = unsafe extern "C" fn(*mut c_void, *const c_void, SizeT) -> *mut c_void;
type MemsetT = unsafe extern "C" fn(*mut c_void, c_int, SizeT) -> *mut c_void;
type MmapOpT = unsafe extern "C" fn(*mut c_void, SizeT, c_uint, bool);

#[repr(C)]
pub union function_op {
    pub memcpy: Option<MemcpyT>,
    pub memset: Option<MemsetT>,
    pub mmap_op: Option<MmapOpT>,
}

#[repr(C)]
pub struct function_fn {
    pub init: Option<MemInitT>,
    pub fini: Option<MemFiniT>,
    pub op: function_op,
}

#[repr(C)]
pub struct function {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub fn_: function_fn,
}

static mut CYCLE_ATTR: perf_event_attr = perf_event_attr {
    type_: PERF_TYPE_HARDWARE,
    config: PERF_COUNT_HW_CPU_CYCLES,
};

static mut STATS: stats = stats { _private: [] };

unsafe extern "C" {
    static mut bench_format: c_int;
    static mut errno: c_int;

    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn getpid() -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> isize;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn mmap(
        addr: *mut c_void,
        length: SizeT,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: SizeT) -> c_int;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pthread_create(
        thread: *mut PthreadT,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: PthreadT, retval: *mut *mut c_void) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn perf_atoll(str_: *mut c_char) -> i64;
    fn ilog2(n: c_uint) -> c_uint;
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: f64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;
    fn rel_stddev_stats(stddev: f64, avg: f64) -> f64;
    fn pr_debug(fmt: *const c_char, ...) -> c_int;
}

unsafe fn bug_on(cond: bool) {
    if cond {
        panic!("BUG_ON");
    }
}

unsafe fn timer_sub(e: *const timeval, s: *const timeval, t: *mut timeval) {
    (*t).tv_sec = (*e).tv_sec - (*s).tv_sec;
    (*t).tv_usec = (*e).tv_usec - (*s).tv_usec;
    if (*t).tv_usec < 0 {
        (*t).tv_sec -= 1;
        (*t).tv_usec += USEC_PER_SEC;
    }
}

unsafe fn timer_add(a: *const timeval, b: *const timeval, t: *mut timeval) {
    (*t).tv_sec = (*a).tv_sec + (*b).tv_sec;
    (*t).tv_usec = (*a).tv_usec + (*b).tv_usec;
    if (*t).tv_usec >= USEC_PER_SEC {
        (*t).tv_sec += 1;
        (*t).tv_usec -= USEC_PER_SEC;
    }
}

unsafe extern "C" fn init_cycles() -> c_int {
    CYCLES_FD = sys_perf_event_open(
        &mut CYCLE_ATTR,
        getpid(),
        -1,
        -1,
        perf_event_open_cloexec_flag(),
    );

    if CYCLES_FD < 0 && errno == ENOSYS {
        pr_debug(b"No CONFIG_PERF_EVENTS=y kernel support configured?\n\0".as_ptr() as *const c_char);
        return -1;
    }

    CYCLES_FD
}

unsafe extern "C" fn get_cycles() -> U64 {
    let ret: c_int;
    let mut clk: U64 = 0;

    ret = read(
        CYCLES_FD,
        &mut clk as *mut U64 as *mut c_void,
        size_of::<U64>(),
    ) as c_int;
    bug_on(ret != size_of::<U64>() as c_int);

    clk
}

unsafe extern "C" fn clock_get(t: *mut bench_clock) {
    if USE_CYCLES {
        (*t).cycles = get_cycles();
    } else {
        bug_on(gettimeofday(&mut (*t).tv, null_mut()) != 0);
    }
}

unsafe extern "C" fn clock_diff(s: *mut bench_clock, e: *mut bench_clock) -> bench_clock {
    let mut t = bench_clock { cycles: 0 };

    if USE_CYCLES {
        t.cycles = (*e).cycles.wrapping_sub((*s).cycles);
    } else {
        timer_sub(&(*e).tv, &(*s).tv, &mut t.tv);
    }

    t
}

unsafe extern "C" fn clock_accum(a: *mut bench_clock, b: *mut bench_clock) {
    if USE_CYCLES {
        (*a).cycles = (*a).cycles.wrapping_add((*b).cycles);
    } else {
        let av = (*a).tv;
        let bv = (*b).tv;
        timer_add(&av, &bv, &mut (*a).tv);
    }
}

unsafe extern "C" fn timeval2double(ts: *mut timeval) -> f64 {
    ((*ts).tv_sec as f64 + (*ts).tv_usec as f64 / USEC_PER_SEC as f64) / NR_THREADS as f64
}

unsafe fn print_bps(x: f64) {
    if x < K {
        printf(b" %14lf bytes/sec\0".as_ptr() as *const c_char, x);
    } else if x < K * K {
        printf(b" %14lfd KB/sec\0".as_ptr() as *const c_char, x / K);
    } else if x < K * K * K {
        printf(b" %14lf MB/sec\0".as_ptr() as *const c_char, x / K / K);
    } else {
        printf(b" %14lf GB/sec\0".as_ptr() as *const c_char, x / K / K / K);
    }
}

unsafe extern "C" fn __bench_mem_function(info: *mut bench_mem_info, p: *mut bench_params, r_idx: c_int) {
    let r = (*info).functions.add(r_idx as usize);
    let mut result_bps: f64;
    let mut rt = bench_clock { cycles: 0 };
    let mut src: *mut c_void = null_mut();
    let mut dst: *mut c_void = null_mut();

    init_stats(&mut STATS);
    printf(
        b"# function '%s' (%s)\n\0".as_ptr() as *const c_char,
        (*r).name,
        (*r).desc,
    );

    if let Some(init) = (*r).fn_.init {
        if init(info, p, &mut src, &mut dst) {
            printf(
                b"# Memory allocation failed - maybe size (%s) %s?\n\0".as_ptr() as *const c_char,
                SIZE_STR,
                if (*p).page_shift != PAGE_SHIFT_4KB {
                    b"has insufficient hugepages\0".as_ptr() as *const c_char
                } else {
                    b"is too large\0".as_ptr() as *const c_char
                },
            );
            if let Some(fini) = (*r).fn_.fini {
                fini(info, p, &mut src, &mut dst);
            }
            return;
        }
    }

    if bench_format == BENCH_FORMAT_DEFAULT {
        printf(b"# Copying %s bytes ...\n\n\0".as_ptr() as *const c_char, SIZE_STR);
    }

    if let Some(do_op) = (*info).do_op {
        if do_op(r, p, src, dst, &mut rt) != 0 {
            if let Some(fini) = (*r).fn_.fini {
                fini(info, p, &mut src, &mut dst);
            }
            return;
        }
    }

    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            if USE_CYCLES {
                printf(
                    b" %14lf cycles/byte\0".as_ptr() as *const c_char,
                    rt.cycles as f64 / (*p).size_total as f64,
                );
            } else {
                result_bps = (*p).size_total as f64 / timeval2double(&mut rt.tv);
                print_bps(result_bps);
            }
            if NR_THREADS > 1 {
                printf(
                    b"/thread\t( +- %6.2f%% )\0".as_ptr() as *const c_char,
                    rel_stddev_stats(stddev_stats(&mut STATS), avg_stats(&mut STATS)),
                );
            }
            printf(b"\n\0".as_ptr() as *const c_char);
        }
        BENCH_FORMAT_SIMPLE => {
            if USE_CYCLES {
                printf(
                    b"%lf\n\0".as_ptr() as *const c_char,
                    rt.cycles as f64 / (*p).size_total as f64,
                );
            } else {
                result_bps = (*p).size_total as f64 / timeval2double(&mut rt.tv);
                printf(b"%lf\n\0".as_ptr() as *const c_char, result_bps);
            }
        }
        _ => {
            bug_on(true);
        }
    }

    if let Some(fini) = (*r).fn_.fini {
        fini(info, p, &mut src, &mut dst);
    }
}

unsafe extern "C" fn bench_mem_common(
    mut argc: c_int,
    argv: *const *const c_char,
    info: *mut bench_mem_info,
) -> c_int {
    let mut i: c_int;
    let mut p = bench_params {
        size: 0,
        size_total: 0,
        chunk_size: 0,
        nr_loops: 0,
        page_shift: 0,
        seed: 0,
    };
    let page_size: c_uint;

    argc = parse_options(argc, argv, (*info).options, (*info).usage, 0);
    let _ = argc;

    if USE_CYCLES {
        i = init_cycles();
        if i < 0 {
            fprintf(stderr, b"Failed to open cycles counter\n\0".as_ptr() as *const c_char);
            return i;
        }
    }

    p.nr_loops = NR_LOOPS;
    p.size = perf_atoll(SIZE_STR as *mut c_char) as SizeT;

    if (p.size as S64) <= 0 {
        fprintf(stderr, b"Invalid size:%s\n\0".as_ptr() as *const c_char, SIZE_STR);
        return 1;
    }
    p.size_total = p.size.wrapping_mul(p.nr_loops as SizeT);

    p.chunk_size = perf_atoll(CHUNK_SIZE_STR as *mut c_char) as SizeT;
    if (p.chunk_size as S64) < 0 || (p.chunk_size as S64) > p.size as S64 {
        fprintf(stderr, b"Invalid chunk_size:%s\n\0".as_ptr() as *const c_char, CHUNK_SIZE_STR);
        return 1;
    }
    if p.chunk_size == 0 {
        p.chunk_size = p.size;
    }

    page_size = perf_atoll(PAGE_SIZE_STR as *mut c_char) as c_uint;
    if page_size != (1u32 << PAGE_SHIFT_4KB)
        && page_size != (1u32 << PAGE_SHIFT_2MB)
        && page_size != (1u32 << PAGE_SHIFT_1GB)
    {
        fprintf(stderr, b"Invalid page-size:%s\n\0".as_ptr() as *const c_char, PAGE_SIZE_STR);
        return 1;
    }
    p.page_shift = ilog2(page_size);

    p.seed = SEED;

    if strncmp(FUNCTION_STR, b"all\0".as_ptr() as *const c_char, 3) == 0 {
        i = 0;
        while !(*(*info).functions.add(i as usize)).name.is_null() {
            __bench_mem_function(info, &mut p, i);
            i += 1;
        }
        return 0;
    }

    i = 0;
    while !(*(*info).functions.add(i as usize)).name.is_null() {
        if strcmp((*(*info).functions.add(i as usize)).name, FUNCTION_STR) == 0 {
            break;
        }
        i += 1;
    }
    if (*(*info).functions.add(i as usize)).name.is_null() {
        if strcmp(FUNCTION_STR, b"help\0".as_ptr() as *const c_char) != 0
            && strcmp(FUNCTION_STR, b"h\0".as_ptr() as *const c_char) != 0
        {
            printf(b"Unknown function: %s\n\0".as_ptr() as *const c_char, FUNCTION_STR);
        }
        printf(b"Available functions:\n\0".as_ptr() as *const c_char);
        i = 0;
        while !(*(*info).functions.add(i as usize)).name.is_null() {
            printf(
                b"\t%s ... %s\n\0".as_ptr() as *const c_char,
                (*(*info).functions.add(i as usize)).name,
                (*(*info).functions.add(i as usize)).desc,
            );
            i += 1;
        }
        return 1;
    }

    __bench_mem_function(info, &mut p, i);

    0
}

unsafe extern "C" fn memcpy_prefault(fn_: MemcpyT, size: SizeT, src: *mut c_void, dst: *mut c_void) {
    /* Make sure to always prefault zero pages even if MMAP_THRESH is crossed: */
    memset(src, 0, size);

    /*
     * We prefault the freshly allocated memory range here,
     * to not measure page fault overhead:
     */
    fn_(dst, src, size);
}

unsafe extern "C" fn do_memcpy(
    r: *const function,
    p: *mut bench_params,
    src: *mut c_void,
    dst: *mut c_void,
    rt: *mut bench_clock,
) -> c_int {
    let mut start = bench_clock { cycles: 0 };
    let mut end = bench_clock { cycles: 0 };
    let fn_ = (*r).fn_.op.memcpy.unwrap();

    memcpy_prefault(fn_, (*p).size, src, dst);

    clock_get(&mut start);
    let mut i: c_uint = 0;
    while i < (*p).nr_loops {
        let mut off: SizeT = 0;
        while off < (*p).size {
            fn_(
                (dst as *mut u8).add(off) as *mut c_void,
                (src as *const u8).add(off) as *const c_void,
                core::cmp::min((*p).chunk_size, (*p).size - off),
            );
            off = off.wrapping_add((*p).chunk_size);
        }
        i += 1;
    }
    clock_get(&mut end);

    *rt = clock_diff(&mut start, &mut end);

    0
}

unsafe extern "C" fn bench_mmap(size: SizeT, populate: bool, page_shift: c_uint) -> *mut c_void {
    let mut extra: c_int = if populate { MAP_POPULATE } else { 0 };

    if page_shift != PAGE_SHIFT_4KB {
        extra |= MAP_HUGETLB | ((page_shift as c_int) << MAP_HUGE_SHIFT);
    }

    let p = mmap(
        null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        extra | MAP_PRIVATE | MAP_ANONYMOUS,
        0,
        0,
    );

    if p == MAP_FAILED { null_mut() } else { p }
}

unsafe extern "C" fn bench_munmap(p: *mut c_void, size: SizeT) {
    if !p.is_null() {
        munmap(p, size);
    }
}

unsafe extern "C" fn mem_alloc(
    info: *mut bench_mem_info,
    p: *mut bench_params,
    src: *mut *mut c_void,
    dst: *mut *mut c_void,
) -> bool {
    let mut failed: bool;

    *dst = bench_mmap((*p).size, true, (*p).page_shift);
    failed = (*dst).is_null();

    if (*info).alloc_src {
        *src = bench_mmap((*p).size, true, (*p).page_shift);
        failed = failed || (*src).is_null();
    }

    failed
}

unsafe extern "C" fn mem_free(
    _info: *mut bench_mem_info,
    p: *mut bench_params,
    src: *mut *mut c_void,
    dst: *mut *mut c_void,
) {
    bench_munmap(*dst, (*p).size);
    bench_munmap(*src, (*p).size);

    *dst = null_mut();
    *src = null_mut();
}

static mut MEMCPY_FUNCTIONS: [function; 2] = [
    function {
        name: b"default\0".as_ptr() as *const c_char,
        desc: b"Default memcpy() provided by glibc\0".as_ptr() as *const c_char,
        fn_: function_fn {
            init: Some(mem_alloc),
            fini: Some(mem_free),
            op: function_op { memcpy: Some(memcpy) },
        },
    },
    // If HAVE_ARCH_X86_64_SUPPORT is enabled, C includes mem-memcpy-x86-64-asm-def.h
    // with MEMCPY_FN entries here.
    function {
        name: null(),
        desc: null(),
        fn_: function_fn {
            init: None,
            fini: None,
            op: function_op { memcpy: None },
        },
    },
];

static BENCH_MEM_MEMCPY_USAGE_0: &[u8] = b"perf bench mem memcpy <options>\0";
static BENCH_MEM_MEMCPY_USAGE: [*const c_char; 2] = [
    BENCH_MEM_MEMCPY_USAGE_0.as_ptr() as *const c_char,
    null(),
];

#[no_mangle]
pub unsafe extern "C" fn bench_mem_memcpy(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut info = bench_mem_info {
        functions: MEMCPY_FUNCTIONS.as_ptr(),
        do_op: Some(do_memcpy),
        usage: BENCH_MEM_MEMCPY_USAGE.as_ptr(),
        options: BENCH_MEM_OPTIONS.as_ptr(),
        alloc_src: true,
    };

    bench_mem_common(argc, argv, &mut info)
}

unsafe extern "C" fn do_memset(
    r: *const function,
    p: *mut bench_params,
    _src: *mut c_void,
    dst: *mut c_void,
    rt: *mut bench_clock,
) -> c_int {
    let mut start = bench_clock { cycles: 0 };
    let mut end = bench_clock { cycles: 0 };
    let fn_ = (*r).fn_.op.memset.unwrap();

    /*
     * We prefault the freshly allocated memory range here,
     * to not measure page fault overhead:
     */
    fn_(dst, -1, (*p).size);

    clock_get(&mut start);
    let mut i: c_uint = 0;
    while i < (*p).nr_loops {
        let mut off: SizeT = 0;
        while off < (*p).size {
            fn_(
                (dst as *mut u8).add(off) as *mut c_void,
                i as c_int,
                core::cmp::min((*p).chunk_size, (*p).size - off),
            );
            off = off.wrapping_add((*p).chunk_size);
        }
        i += 1;
    }
    clock_get(&mut end);

    *rt = clock_diff(&mut start, &mut end);

    0
}

static BENCH_MEM_MEMSET_USAGE_0: &[u8] = b"perf bench mem memset <options>\0";
static BENCH_MEM_MEMSET_USAGE: [*const c_char; 2] = [
    BENCH_MEM_MEMSET_USAGE_0.as_ptr() as *const c_char,
    null(),
];

static MEMSET_FUNCTIONS: [function; 2] = [
    function {
        name: b"default\0".as_ptr() as *const c_char,
        desc: b"Default memset() provided by glibc\0".as_ptr() as *const c_char,
        fn_: function_fn {
            init: Some(mem_alloc),
            fini: Some(mem_free),
            op: function_op { memset: Some(memset) },
        },
    },
    // If HAVE_ARCH_X86_64_SUPPORT is enabled, C includes mem-memset-x86-64-asm-def.h
    // with MEMSET_FN entries here.
    function {
        name: null(),
        desc: null(),
        fn_: function_fn {
            init: None,
            fini: None,
            op: function_op { memset: None },
        },
    },
];

#[no_mangle]
pub unsafe extern "C" fn bench_mem_memset(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut info = bench_mem_info {
        functions: MEMSET_FUNCTIONS.as_ptr(),
        do_op: Some(do_memset),
        usage: BENCH_MEM_MEMSET_USAGE.as_ptr(),
        options: BENCH_MEM_OPTIONS.as_ptr(),
        alloc_src: false,
    };

    bench_mem_common(argc, argv, &mut info)
}

unsafe extern "C" fn mmap_page_touch(dst: *mut c_void, size: SizeT, page_shift: c_uint, random: bool) {
    let npages: c_ulong = size as c_ulong / (1u64 << page_shift) as c_ulong;
    let mut offset: c_ulong = 0;
    let mut r: c_ulong = 0;

    let mut i: c_ulong = 0;
    while i < npages {
        if random {
            r = (rand() as c_ulong) % ((1u64 << page_shift) as c_ulong);
        }

        let ptr = (dst as *mut u8).add(offset as usize + r as usize);
        *ptr = (*ptr).wrapping_add(i as u8);
        offset = offset.wrapping_add((1u64 << page_shift) as c_ulong);
        i += 1;
    }
}

#[repr(C)]
pub struct mmap_data {
    pub id: PthreadT,
    pub func: *const function,
    pub params: *mut bench_params,
    pub result: bench_clock,
    pub seed: c_uint,
    pub error: c_int,
}

unsafe extern "C" fn do_mmap_thread(arg: *mut c_void) -> *mut c_void {
    let data = arg as *mut mmap_data;
    let r = (*data).func;
    let p = (*data).params;
    let mut start = bench_clock { cycles: 0 };
    let mut end = bench_clock { cycles: 0 };
    let mut diff: bench_clock;
    let fn_ = (*r).fn_.op.mmap_op.unwrap();
    let populate = strcmp((*r).name, b"populate\0".as_ptr() as *const c_char) == 0;
    let mut dst: *mut c_void;

    if (*data).seed != 0 {
        srand((*data).seed);
    }

    let mut i: c_uint = 0;
    while i < (*p).nr_loops {
        clock_get(&mut start);
        dst = bench_mmap((*p).size, populate, (*p).page_shift);
        if dst.is_null() {
            (*data).error = -ENOMEM;
            return null_mut();
        }

        fn_(dst, (*p).size, (*p).page_shift, (*p).seed != 0);
        clock_get(&mut end);
        diff = clock_diff(&mut start, &mut end);
        clock_accum(&mut (*data).result, &mut diff);

        bench_munmap(dst, (*p).size);
        i += 1;
    }

    data as *mut c_void
}

unsafe extern "C" fn do_mmap(
    r: *const function,
    p: *mut bench_params,
    _src: *mut c_void,
    _dst: *mut c_void,
    accum: *mut bench_clock,
) -> c_int {
    let data: *mut mmap_data;
    let mut error: c_int = 0;

    data = calloc(NR_THREADS as SizeT, size_of::<mmap_data>()) as *mut mmap_data;
    if data.is_null() {
        printf(b"# Failed to allocate thread resources\n\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut i: c_uint = 0;
    while i < NR_THREADS {
        (*data.add(i as usize)).func = r;
        (*data.add(i as usize)).params = p;
        if (*p).seed != 0 {
            (*data.add(i as usize)).seed = (*p).seed.wrapping_add(i);
        }

        if pthread_create(
            &mut (*data.add(i as usize)).id,
            null(),
            do_mmap_thread,
            data.add(i as usize) as *mut c_void,
        ) < 0
        {
            (*data.add(i as usize)).error = -errno;
        }
        i += 1;
    }

    i = 0;
    while i < NR_THREADS {
        let t = &mut (*data.add(i as usize)).result as *mut bench_clock;

        pthread_join((*data.add(i as usize)).id, null_mut());

        clock_accum(accum, t);
        if USE_CYCLES {
            update_stats(&mut STATS, (*t).cycles as f64);
        } else {
            update_stats(
                &mut STATS,
                ((*t).tv.tv_sec as f64) * 1e6f64 + (*t).tv.tv_usec as f64,
            );
        }
        error |= (*data.add(i as usize)).error;
        i += 1;
    }
    free(data as *mut c_void);

    if error != 0 {
        printf(
            b"# Memory allocation failed - maybe size (%s) %s?\n\0".as_ptr() as *const c_char,
            SIZE_STR,
            if (*p).page_shift != PAGE_SHIFT_4KB {
                b"has insufficient hugepages\0".as_ptr() as *const c_char
            } else {
                b"is too large\0".as_ptr() as *const c_char
            },
        );
    }
    if error != 0 { -1 } else { 0 }
}

static BENCH_MEM_MMAP_USAGE_0: &[u8] = b"perf bench mem mmap <options>\0";
static BENCH_MEM_MMAP_USAGE: [*const c_char; 2] = [
    BENCH_MEM_MMAP_USAGE_0.as_ptr() as *const c_char,
    null(),
];

static MMAP_FUNCTIONS: [function; 3] = [
    function {
        name: b"demand\0".as_ptr() as *const c_char,
        desc: b"Demand loaded mmap()\0".as_ptr() as *const c_char,
        fn_: function_fn {
            init: None,
            fini: None,
            op: function_op {
                mmap_op: Some(mmap_page_touch),
            },
        },
    },
    function {
        name: b"populate\0".as_ptr() as *const c_char,
        desc: b"Eagerly populated mmap()\0".as_ptr() as *const c_char,
        fn_: function_fn {
            init: None,
            fini: None,
            op: function_op {
                mmap_op: Some(mmap_page_touch),
            },
        },
    },
    function {
        name: null(),
        desc: null(),
        fn_: function_fn {
            init: None,
            fini: None,
            op: function_op { mmap_op: None },
        },
    },
];

#[no_mangle]
pub unsafe extern "C" fn bench_mem_mmap(argc: c_int, argv: *const *const c_char) -> c_int {
    // Original C declares bench_mmap_options with OPT_UINTEGER/OPT_PARENT/OPT_END macros here.
    // The macro expansions are supplied by external parse-options headers.
    static BENCH_MMAP_OPTIONS: [option; 1] = [option { _private: [] }];

    let mut info = bench_mem_info {
        functions: MMAP_FUNCTIONS.as_ptr(),
        do_op: Some(do_mmap),
        usage: BENCH_MEM_MMAP_USAGE.as_ptr(),
        options: BENCH_MMAP_OPTIONS.as_ptr(),
        alloc_src: false,
    };

    bench_mem_common(argc, argv, &mut info)
}
