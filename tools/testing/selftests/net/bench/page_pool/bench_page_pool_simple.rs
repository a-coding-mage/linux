// SPDX-License-Identifier: GPL-2.0-only
/*
 * Benchmark module for page_pool.
 *
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// C dependencies:
// #include <linux/interrupt.h>
// #include <linux/limits.h>
// #include <linux/module.h>
// #include <linux/mutex.h>
// #include <net/page_pool/helpers.h>
// #include "time_bench.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

static mut verbose: c_int = 1;
const MY_POOL_SIZE: c_int = 1024;

/* Makes tests selectable. Useful for perf-record to analyze a single test.
 * Hint: Bash shells support writing binary number like: $((2#101010)
 *
 * # modprobe bench_page_pool_simple run_flags=$((2#100))
 */
static mut run_flags: c_ulong = 0xFFFFFFFF;
// module_param(run_flags, ulong, 0);
// MODULE_PARM_DESC(run_flags, "Limit which bench test that runs");

/* Count the bit number from the enum */
#[repr(C)]
enum benchmark_bit {
    bit_run_bench_baseline,
    bit_run_bench_no_softirq01,
    bit_run_bench_no_softirq02,
    bit_run_bench_no_softirq03,
}

#[inline]
const fn bit(b: benchmark_bit) -> c_ulong {
    1u64.wrapping_shl(b as u32) as c_ulong
}

#[inline]
unsafe fn enabled(b: benchmark_bit) -> c_ulong {
    run_flags & bit(b)
}

/* notice time_bench is limited to U32_MAX nr loops */
static mut loops: c_ulong = 10000000;
// module_param(loops, ulong, 0);
// MODULE_PARM_DESC(loops, "Specify loops bench will run");

#[repr(C)]
pub struct time_bench_record {
    pub loops: c_int,
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

type gfp_t = c_ulong;

#[repr(C)]
pub struct page_pool_params {
    pub order: c_int,
    pub flags: c_int,
    pub pool_size: c_int,
    pub nid: c_int,
    pub dev: *mut c_void,
    pub dma_dir: c_int,
}

const GFP_ATOMIC: gfp_t = 0;
const NUMA_NO_NODE: c_int = -1;
const DMA_BIDIRECTIONAL: c_int = 0;
const U32_MAX: c_ulong = u32::MAX as c_ulong;
const ECHRNG: c_int = 44;

unsafe extern "C" {
    fn time_bench_start(rec: *mut time_bench_record);
    fn time_bench_stop(rec: *mut time_bench_record, loops_cnt: u64);
    fn time_bench_loop(
        loops: u32,
        samples: c_int,
        name: *const c_char,
        data: *mut c_void,
        func: unsafe extern "C" fn(*mut time_bench_record, *mut c_void) -> c_int,
    );

    fn barrier();
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn kcalloc(n: usize, size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn page_pool_alloc_pages(pp: *mut page_pool, gfp_mask: gfp_t) -> *mut page;
    fn page_pool_put_page(pp: *mut page_pool, page: *mut page, dma_sync_size: c_int, allow_direct: bool);
    fn page_pool_recycle_direct(pp: *mut page_pool, page: *mut page);
    fn get_page(page: *mut page);
    fn put_page(page: *mut page);
    fn page_pool_create(params: *mut page_pool_params) -> *mut page_pool;
    fn page_pool_destroy(pp: *mut page_pool);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn in_serving_softirq() -> bool;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

/* Timing at the nanosec level, we need to know the overhead
 * introduced by the for loop itself
 */
unsafe extern "C" fn time_bench_for_loop(rec: *mut time_bench_record, _data: *mut c_void) -> c_int {
    let mut loops_cnt: u64 = 0;
    let mut i: c_int;

    time_bench_start(rec);
    /** Loop to measure **/
    i = 0;
    while i < (*rec).loops {
        loops_cnt = loops_cnt.wrapping_add(1);
        barrier(); /* avoid compiler to optimize this loop */
        i += 1;
    }
    time_bench_stop(rec, loops_cnt);
    loops_cnt as c_int
}

unsafe extern "C" fn time_bench_atomic_inc(rec: *mut time_bench_record, _data: *mut c_void) -> c_int {
    let mut loops_cnt: u64 = 0;
    let mut cnt: atomic_t = core::mem::zeroed();
    let mut i: c_int;

    atomic_set(&mut cnt, 0);

    time_bench_start(rec);
    /** Loop to measure **/
    i = 0;
    while i < (*rec).loops {
        atomic_inc(&mut cnt);
        barrier(); /* avoid compiler to optimize this loop */
        i += 1;
    }
    loops_cnt = atomic_read(&cnt) as u64;
    time_bench_stop(rec, loops_cnt);
    loops_cnt as c_int
}

/* The ptr_ping in page_pool uses a spinlock. We need to know the minimum
 * overhead of taking+releasing a spinlock, to know the cycles that can be saved
 * by e.g. amortizing this via bulking.
 */
unsafe extern "C" fn time_bench_lock(rec: *mut time_bench_record, _data: *mut c_void) -> c_int {
    let mut loops_cnt: u64 = 0;
    let mut lock: spinlock_t = core::mem::zeroed();
    let mut i: c_int;

    spin_lock_init(&mut lock);

    time_bench_start(rec);
    /** Loop to measure **/
    i = 0;
    while i < (*rec).loops {
        spin_lock(&mut lock);
        loops_cnt = loops_cnt.wrapping_add(1);
        barrier(); /* avoid compiler to optimize this loop */
        spin_unlock(&mut lock);
        i += 1;
    }
    time_bench_stop(rec, loops_cnt);
    loops_cnt as c_int
}

/* Helper for filling some page's into ptr_ring */
unsafe fn pp_fill_ptr_ring(pp: *mut page_pool, elems: c_int) {
    /* GFP_ATOMIC needed when under run softirq */
    let gfp_mask: gfp_t = GFP_ATOMIC;
    let array: *mut *mut page;
    let mut i: c_int;

    array = kcalloc(elems as usize, size_of::<*mut page>(), gfp_mask) as *mut *mut page;

    i = 0;
    while i < elems {
        *array.offset(i as isize) = page_pool_alloc_pages(pp, gfp_mask);
        i += 1;
    }
    i = 0;
    while i < elems {
        page_pool_put_page(pp, *array.offset(i as isize), -1, false);
        i += 1;
    }

    kfree(array as *const c_void);
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum test_type {
    type_fast_path,
    type_ptr_ring,
    type_page_allocator,
}

/* Depends on compile optimizing this function */
unsafe fn time_bench_page_pool(
    rec: *mut time_bench_record,
    _data: *mut c_void,
    type_: test_type,
    func: *const c_char,
) -> c_int {
    let mut loops_cnt: u64 = 0;
    let gfp_mask: gfp_t = GFP_ATOMIC; /* GFP_ATOMIC is not really needed */
    let mut i: c_int;
    let err: c_int;

    let pp: *mut page_pool;
    let mut page: *mut page;

    let mut pp_params = page_pool_params {
        order: 0,
        flags: 0,
        pool_size: MY_POOL_SIZE,
        nid: NUMA_NO_NODE,
        dev: ptr::null_mut(), /* Only use for DMA mapping */
        dma_dir: DMA_BIDIRECTIONAL,
    };

    pp = page_pool_create(&mut pp_params);
    if IS_ERR(pp as *const c_void) {
        err = PTR_ERR(pp as *const c_void);
        pr_warn(c"%s: Error(%d) creating page_pool\n".as_ptr(), func, err);
        goto_out(pp, loops_cnt);
        return loops_cnt as c_int;
    }
    pp_fill_ptr_ring(pp, 64);

    if in_serving_softirq() {
        pr_warn(c"%s(): in_serving_softirq fast-path\n".as_ptr(), func);
    } else {
        pr_warn(c"%s(): Cannot use page_pool fast-path\n".as_ptr(), func);
    }

    time_bench_start(rec);
    /** Loop to measure **/
    i = 0;
    while i < (*rec).loops {
        /* Common fast-path alloc that depend on in_serving_softirq() */
        page = page_pool_alloc_pages(pp, gfp_mask);
        if page.is_null() {
            break;
        }
        loops_cnt = loops_cnt.wrapping_add(1);
        barrier(); /* avoid compiler to optimize this loop */

        /* The benchmarks purpose it to test different return paths.
         * Compiler should inline optimize other function calls out
         */
        if type_ == test_type::type_fast_path {
            /* Fast-path recycling e.g. XDP_DROP use-case */
            page_pool_recycle_direct(pp, page);
        } else if type_ == test_type::type_ptr_ring {
            /* Normal return path */
            page_pool_put_page(pp, page, -1, false);
        } else if type_ == test_type::type_page_allocator {
            /* Test if not pages are recycled, but instead
             * returned back into systems page allocator
             */
            get_page(page); /* cause no-recycling */
            page_pool_put_page(pp, page, -1, false);
            put_page(page);
        } else {
            panic!("BUILD_BUG()");
        }
        i += 1;
    }
    time_bench_stop(rec, loops_cnt);
    page_pool_destroy(pp);
    loops_cnt as c_int
}

unsafe fn goto_out(pp: *mut page_pool, loops_cnt: u64) {
    page_pool_destroy(pp);
    let _ = loops_cnt;
}

unsafe extern "C" fn time_bench_page_pool01_fast_path(
    rec: *mut time_bench_record,
    data: *mut c_void,
) -> c_int {
    time_bench_page_pool(rec, data, test_type::type_fast_path, c"time_bench_page_pool01_fast_path".as_ptr())
}

unsafe extern "C" fn time_bench_page_pool02_ptr_ring(
    rec: *mut time_bench_record,
    data: *mut c_void,
) -> c_int {
    time_bench_page_pool(rec, data, test_type::type_ptr_ring, c"time_bench_page_pool02_ptr_ring".as_ptr())
}

unsafe extern "C" fn time_bench_page_pool03_slow(
    rec: *mut time_bench_record,
    data: *mut c_void,
) -> c_int {
    time_bench_page_pool(rec, data, test_type::type_page_allocator, c"time_bench_page_pool03_slow".as_ptr())
}

unsafe fn run_benchmark_tests() -> c_int {
    let nr_loops: u32 = loops as u32;

    /* Baseline tests */
    if enabled(benchmark_bit::bit_run_bench_baseline) != 0 {
        time_bench_loop(
            nr_loops.wrapping_mul(10),
            0,
            c"for_loop".as_ptr(),
            ptr::null_mut(),
            time_bench_for_loop,
        );
        time_bench_loop(
            nr_loops.wrapping_mul(10),
            0,
            c"atomic_inc".as_ptr(),
            ptr::null_mut(),
            time_bench_atomic_inc,
        );
        time_bench_loop(nr_loops, 0, c"lock".as_ptr(), ptr::null_mut(), time_bench_lock);
    }

    /* This test cannot activate correct code path, due to no-softirq ctx */
    if enabled(benchmark_bit::bit_run_bench_no_softirq01) != 0 {
        time_bench_loop(
            nr_loops,
            0,
            c"no-softirq-page_pool01".as_ptr(),
            ptr::null_mut(),
            time_bench_page_pool01_fast_path,
        );
    }
    if enabled(benchmark_bit::bit_run_bench_no_softirq02) != 0 {
        time_bench_loop(
            nr_loops,
            0,
            c"no-softirq-page_pool02".as_ptr(),
            ptr::null_mut(),
            time_bench_page_pool02_ptr_ring,
        );
    }
    if enabled(benchmark_bit::bit_run_bench_no_softirq03) != 0 {
        time_bench_loop(
            nr_loops,
            0,
            c"no-softirq-page_pool03".as_ptr(),
            ptr::null_mut(),
            time_bench_page_pool03_slow,
        );
    }

    0
}

unsafe extern "C" fn bench_page_pool_simple_module_init() -> c_int {
    if verbose != 0 {
        pr_info(c"Loaded\n".as_ptr());
    }

    if loops > U32_MAX {
        pr_err(c"Module param loops(%lu) exceeded U32_MAX(%u)\n".as_ptr(), loops, U32_MAX as u32);
        return -ECHRNG;
    }

    run_benchmark_tests();

    0
}
// module_init(bench_page_pool_simple_module_init);

unsafe extern "C" fn bench_page_pool_simple_module_exit() {
    if verbose != 0 {
        pr_info(c"Unloaded\n".as_ptr());
    }
}
// module_exit(bench_page_pool_simple_module_exit);

// MODULE_DESCRIPTION("Benchmark of page_pool simple cases");
// MODULE_AUTHOR("Jesper Dangaard Brouer <netoptimizer@brouer.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
