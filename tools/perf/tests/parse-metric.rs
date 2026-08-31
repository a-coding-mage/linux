// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/parse-metric.c.
// Original C dependencies included:
// <linux/compiler.h>, <errno.h>, <string.h>, <perf/cpumap.h>,
// <perf/evlist.h>, "metricgroup.h", "tests.h",
// "pmu-events/pmu-events.h", "evlist.h", "rblist.h", "debug.h",
// "expr.h", "stat.h", "pmus.h"

use core::ffi::{c_char, c_double, c_int, c_void};

type u64 = u64;

const ENOMEM: c_int = 12;

#[repr(C)]
struct value {
    event: *const c_char,
    val: u64,
}

#[repr(C)]
struct counts {
    val: u64,
    ena: u64,
    run: u64,
}

#[repr(C)]
struct aggr_stat {
    counts: counts,
}

#[repr(C)]
struct evsel_stats {
    aggr: *mut aggr_stat,
}

#[repr(C)]
struct evsel {
    name: *const c_char,
    supported: bool,
    stats: *mut evsel_stats,
}

#[repr(C)]
struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
struct pmu_metrics_table {
    _private: [u8; 0],
}

#[repr(C)]
struct metric_expr {
    metric_name: *const c_char,
    nd: list_head,
}

#[repr(C)]
struct metric_event {
    head: list_head,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct test_suite {
    _private: [u8; 0],
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr: c_int);
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut c_void;
    fn evlist__metric_events(evlist: *mut evlist) -> *mut c_void;
    fn evlist__alloc_stats(config: *mut c_void, evlist: *mut evlist, alloc_raw: bool) -> c_int;
    fn evlist__free_stats(evlist: *mut evlist);
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_evlist__set_maps(core: *mut c_void, cpus: *mut perf_cpu_map, threads: *mut c_void);
    fn find_core_metrics_table(arch: *const c_char, cpu: *const c_char) -> *const pmu_metrics_table;
    fn metricgroup__parse_groups_test(
        evlist: *mut evlist,
        table: *const pmu_metrics_table,
        name: *const c_char,
        cputype_filter: bool,
    ) -> c_int;
    fn metricgroup__lookup(events: *mut c_void, evsel: *mut evsel, create: bool) -> *mut metric_event;
    fn test_generic_metric(mexp: *mut metric_expr, cpu: c_int) -> c_double;
}

extern "C" {
    // Rust declaration for the external iteration supplied by evlist/list helpers.
    fn evlist__first_entry(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next_entry(evsel: *mut evsel) -> *mut evsel;
    fn evlist__entry_is_end(evlist: *mut evlist, evsel: *mut evsel) -> bool;
    fn list_first_metric_expr(head: *mut list_head) -> *mut metric_expr;
    fn list_next_metric_expr(mexp: *mut metric_expr) -> *mut metric_expr;
    fn list_metric_expr_is_end(head: *mut list_head, mexp: *mut metric_expr) -> bool;
    fn test_assert_val(msg: *const c_char, cond: bool) -> c_int;
    fn define_suite(name: *const c_char, test: unsafe extern "C" fn(*mut test_suite, c_int) -> c_int);
}

unsafe fn find_value(name: *const c_char, values: *mut value) -> u64 {
    let mut v: *mut value = values;

    while !(*v).event.is_null() {
        if strcmp(name, (*v).event) == 0 {
            return (*v).val;
        }
        v = v.add(1);
    }
    0
}

unsafe fn load_runtime_stat(evlist: *mut evlist, vals: *mut value) {
    let mut evsel: *mut evsel;
    let mut count: u64;

    evlist__alloc_aggr_stats(evlist, 1);
    evsel = evlist__first_entry(evlist);
    while !evlist__entry_is_end(evlist, evsel) {
        count = find_value((*evsel).name, vals);
        (*evsel).supported = true;
        (*(*(*evsel).stats).aggr).counts.val = count;
        (*(*(*evsel).stats).aggr).counts.ena = 1;
        (*(*(*evsel).stats).aggr).counts.run = 1;
        evsel = evlist__next_entry(evsel);
    }
}

unsafe fn compute_single(evlist: *mut evlist, name: *const c_char) -> c_double {
    let mut mexp: *mut metric_expr;
    let mut me: *mut metric_event;
    let mut evsel: *mut evsel;

    evsel = evlist__first_entry(evlist);
    while !evlist__entry_is_end(evlist, evsel) {
        me = metricgroup__lookup(evlist__metric_events(evlist), evsel, false);
        if !me.is_null() {
            mexp = list_first_metric_expr(&mut (*me).head);
            while !list_metric_expr_is_end(&mut (*me).head, mexp) {
                if strcmp((*mexp).metric_name, name) != 0 {
                    mexp = list_next_metric_expr(mexp);
                    continue;
                }
                return test_generic_metric(mexp, 0);
            }
        }
        evsel = evlist__next_entry(evsel);
    }
    0.0
}

unsafe fn __compute_metric(
    name: *const c_char,
    vals: *mut value,
    name1: *const c_char,
    ratio1: *mut c_double,
    name2: *const c_char,
    ratio2: *mut c_double,
) -> c_int {
    let pme_test: *const pmu_metrics_table;
    let cpus: *mut perf_cpu_map;
    let evlist: *mut evlist;
    let mut err: c_int;

    /*
     * We need to prepare evlist for stat mode running on CPU 0
     * because that's where all the stats are going to be created.
     */
    evlist = evlist__new();
    if evlist.is_null() {
        return -ENOMEM;
    }

    cpus = perf_cpu_map__new(b"0\0".as_ptr() as *const c_char);
    if cpus.is_null() {
        evlist__put(evlist);
        return -ENOMEM;
    }

    perf_evlist__set_maps(evlist__core(evlist), cpus, core::ptr::null_mut());

    /* Parse the metric into metric_events list. */
    pme_test = find_core_metrics_table(
        b"testarch\0".as_ptr() as *const c_char,
        b"testcpu\0".as_ptr() as *const c_char,
    );
    err = metricgroup__parse_groups_test(evlist, pme_test, name, false);
    if err != 0 {
        goto_out(evlist, cpus, err)
    } else {
        err = evlist__alloc_stats(core::ptr::null_mut(), evlist, false);
        if err != 0 {
            goto_out(evlist, cpus, err)
        } else {
            /* Load the runtime stats with given numbers for events. */
            load_runtime_stat(evlist, vals);

            /* And execute the metric */
            if !name1.is_null() && !ratio1.is_null() {
                *ratio1 = compute_single(evlist, name1);
            }
            if !name2.is_null() && !ratio2.is_null() {
                *ratio2 = compute_single(evlist, name2);
            }

            goto_out(evlist, cpus, err)
        }
    }
}

unsafe fn goto_out(evlist: *mut evlist, cpus: *mut perf_cpu_map, err: c_int) -> c_int {
    /* ... cleanup. */
    evlist__free_stats(evlist);
    perf_cpu_map__put(cpus);
    evlist__put(evlist);
    err
}

unsafe fn compute_metric(name: *const c_char, vals: *mut value, ratio: *mut c_double) -> c_int {
    __compute_metric(
        name,
        vals,
        name,
        ratio,
        core::ptr::null(),
        core::ptr::null_mut(),
    )
}

unsafe fn compute_metric_group(
    name: *const c_char,
    vals: *mut value,
    name1: *const c_char,
    ratio1: *mut c_double,
    name2: *const c_char,
    ratio2: *mut c_double,
) -> c_int {
    __compute_metric(name, vals, name1, ratio1, name2, ratio2)
}

macro_rules! test_assert_val {
    ($msg:expr, $cond:expr) => {{
        let __ret = test_assert_val($msg.as_ptr() as *const c_char, $cond);
        if __ret != 0 {
            return __ret;
        }
    }};
}

unsafe fn test_ipc() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"inst_retired.any\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"cpu_clk_unhalted.thread\0".as_ptr() as *const c_char, val: 200 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(b"IPC\0".as_ptr() as *const c_char, vals.as_mut_ptr(), &mut ratio) == 0
    );

    test_assert_val!(b"IPC failed, wrong ratio\0", ratio == 1.5);
    0
}

unsafe fn test_frontend() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"idq_uops_not_delivered.core\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"cpu_clk_unhalted.thread\0".as_ptr() as *const c_char, val: 200 },
        value { event: b"cpu_clk_unhalted.one_thread_active\0".as_ptr() as *const c_char, val: 400 },
        value { event: b"cpu_clk_unhalted.ref_xclk\0".as_ptr() as *const c_char, val: 600 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(
            b"Frontend_Bound_SMT\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            &mut ratio,
        ) == 0
    );

    test_assert_val!(b"Frontend_Bound_SMT failed, wrong ratio\0", ratio == 0.45);
    0
}

unsafe fn test_cache_miss_cycles() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"l1d-loads-misses\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"l1i-loads-misses\0".as_ptr() as *const c_char, val: 200 },
        value { event: b"inst_retired.any\0".as_ptr() as *const c_char, val: 400 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(
            b"cache_miss_cycles\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            &mut ratio,
        ) == 0
    );

    test_assert_val!(b"cache_miss_cycles failed, wrong ratio\0", ratio == 1.25);
    0
}

/*
 * DCache_L2_All_Hits = l2_rqsts.demand_data_rd_hit + l2_rqsts.pf_hit + l2_rqsts.rfo_hi
 * DCache_L2_All_Miss = max(l2_rqsts.all_demand_data_rd - l2_rqsts.demand_data_rd_hit, 0) +
 *                      l2_rqsts.pf_miss + l2_rqsts.rfo_miss
 * DCache_L2_All      = dcache_l2_all_hits + dcache_l2_all_miss
 * DCache_L2_Hits     = d_ratio(dcache_l2_all_hits, dcache_l2_all)
 * DCache_L2_Misses   = d_ratio(dcache_l2_all_miss, dcache_l2_all)
 *
 * l2_rqsts.demand_data_rd_hit = 100
 * l2_rqsts.pf_hit             = 200
 * l2_rqsts.rfo_hi             = 300
 * l2_rqsts.all_demand_data_rd = 400
 * l2_rqsts.pf_miss            = 500
 * l2_rqsts.rfo_miss           = 600
 *
 * DCache_L2_All_Hits = 600
 * DCache_L2_All_Miss = MAX(400 - 100, 0) + 500 + 600 = 1400
 * DCache_L2_All      = 600 + 1400  = 2000
 * DCache_L2_Hits     = 600 / 2000  = 0.3
 * DCache_L2_Misses   = 1400 / 2000 = 0.7
 */
unsafe fn test_dcache_l2() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"l2_rqsts.demand_data_rd_hit\0".as_ptr() as *const c_char, val: 100 },
        value { event: b"l2_rqsts.pf_hit\0".as_ptr() as *const c_char, val: 200 },
        value { event: b"l2_rqsts.rfo_hit\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"l2_rqsts.all_demand_data_rd\0".as_ptr() as *const c_char, val: 400 },
        value { event: b"l2_rqsts.pf_miss\0".as_ptr() as *const c_char, val: 500 },
        value { event: b"l2_rqsts.rfo_miss\0".as_ptr() as *const c_char, val: 600 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(
            b"DCache_L2_Hits\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            &mut ratio,
        ) == 0
    );

    test_assert_val!(b"DCache_L2_Hits failed, wrong ratio\0", ratio == 0.3);

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(
            b"DCache_L2_Misses\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            &mut ratio,
        ) == 0
    );

    test_assert_val!(b"DCache_L2_Misses failed, wrong ratio\0", ratio == 0.7);
    0
}

unsafe fn test_recursion_fail() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"inst_retired.any\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"cpu_clk_unhalted.thread\0".as_ptr() as *const c_char, val: 200 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to find recursion\0",
        compute_metric(b"M1\0".as_ptr() as *const c_char, vals.as_mut_ptr(), &mut ratio) == -1
    );

    test_assert_val!(
        b"failed to find recursion\0",
        compute_metric(b"M3\0".as_ptr() as *const c_char, vals.as_mut_ptr(), &mut ratio) == -1
    );
    0
}

unsafe fn test_memory_bandwidth() -> c_int {
    let mut ratio: c_double = 0.0;
    let mut vals = [
        value { event: b"l1d.replacement\0".as_ptr() as *const c_char, val: 4000000 },
        value { event: b"duration_time\0".as_ptr() as *const c_char, val: 200000000 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to compute metric\0",
        compute_metric(
            b"L1D_Cache_Fill_BW\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            &mut ratio,
        ) == 0
    );
    test_assert_val!(b"L1D_Cache_Fill_BW, wrong ratio\0", 1.28 == ratio);

    0
}

unsafe fn test_metric_group() -> c_int {
    let mut ratio1: c_double = 0.0;
    let mut ratio2: c_double = 0.0;
    let mut vals = [
        value { event: b"cpu_clk_unhalted.thread\0".as_ptr() as *const c_char, val: 200 },
        value { event: b"l1d-loads-misses\0".as_ptr() as *const c_char, val: 300 },
        value { event: b"l1i-loads-misses\0".as_ptr() as *const c_char, val: 200 },
        value { event: b"inst_retired.any\0".as_ptr() as *const c_char, val: 400 },
        value { event: core::ptr::null(), val: 0 },
    ];

    test_assert_val!(
        b"failed to find recursion\0",
        compute_metric_group(
            b"group1\0".as_ptr() as *const c_char,
            vals.as_mut_ptr(),
            b"IPC\0".as_ptr() as *const c_char,
            &mut ratio1,
            b"cache_miss_cycles\0".as_ptr() as *const c_char,
            &mut ratio2,
        ) == 0
    );

    test_assert_val!(b"group IPC failed, wrong ratio\0", ratio1 == 2.0);

    test_assert_val!(b"group cache_miss_cycles failed, wrong ratio\0", ratio2 == 1.25);
    0
}

unsafe extern "C" fn test__parse_metric(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test_assert_val!(b"IPC failed\0", test_ipc() == 0);
    test_assert_val!(b"frontend failed\0", test_frontend() == 0);
    test_assert_val!(b"DCache_L2 failed\0", test_dcache_l2() == 0);
    test_assert_val!(b"recursion fail failed\0", test_recursion_fail() == 0);
    test_assert_val!(b"Memory bandwidth\0", test_memory_bandwidth() == 0);
    test_assert_val!(b"cache_miss_cycles failed\0", test_cache_miss_cycles() == 0);
    test_assert_val!(b"test metric group\0", test_metric_group() == 0);
    0
}

#[used]
static DEFINE_SUITE_PARSE_METRIC: unsafe extern "C" fn() = {
    unsafe extern "C" fn register() {
        define_suite(
            b"Parse and process metrics\0".as_ptr() as *const c_char,
            test__parse_metric,
        );
    }
    register
};
