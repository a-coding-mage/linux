// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/stat.c.  Header-provided types, constants, list
// iteration helpers, and functions are expected to be supplied by surrounding
// Rust bindings for the original perf sources.

use core::ffi::{c_char, c_int, c_long, c_void};

type u64 = u64;
type size_t = usize;
type bool_ = bool;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static mut verbose: c_int;

    fn sqrt(x: f64) -> f64;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut c_void);

    fn init_stats(stats: *mut stats);
    fn evsel__nr_cpus(evsel: *mut evsel) -> c_int;
    fn evsel__cpus(evsel: *mut evsel) -> *mut perf_cpu_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_counts__new(ncpus: c_int, nthreads: c_int) -> *mut perf_counts;
    fn perf_counts__delete(counts: *mut perf_counts);
    fn perf_counts__reset(counts: *mut perf_counts);
    fn perf_counts(counts: *mut perf_counts, cpu: c_int, thread: c_int) -> *mut perf_counts_values;
    fn evsel__alloc_counts(evsel: *mut evsel) -> c_int;
    fn evsel__free_counts(evsel: *mut evsel);
    fn evsel__reset_counts(evsel: *mut evsel);
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__idx(cpus: *mut perf_cpu_map, cpu: perf_cpu) -> c_int;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *mut perf_cpu_map) -> bool_;
    fn cpu__get_socket_id(cpu: perf_cpu) -> c_int;
    fn cpu__get_die_id(cpu: perf_cpu) -> c_int;
    fn hashmap__new(
        hash: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool_>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__find(map: *mut hashmap, key: *const c_void, value: *mut c_void) -> bool_;
    fn hashmap__add(map: *mut hashmap, key: *mut c_void, value: c_long) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn evsel__compute_deltas(
        evsel: *mut evsel,
        cpu_map_idx: c_int,
        thread: c_int,
        count: *mut perf_counts_values,
    );
    fn perf_counts_values__scale(
        count: *mut perf_counts_values,
        scale: bool_,
        other: *mut c_void,
    );
    fn aggr_cpu_id__equal(a: *const aggr_cpu_id, b: *const aggr_cpu_id) -> bool_;
    fn evsel__zero_per_pkg(evsel: *mut evsel);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_hybrid(evsel: *const evsel) -> bool_;
    fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn perf_event__read_stat_config(sc: *mut perf_stat_config, config: *const perf_record_stat_config);
    fn pr_err(format: *const c_char, ...);
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stats {
    pub n: f64,
    pub mean: f64,
    pub M2: f64,
    pub max: u64,
    pub min: u64,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_stat_aggr {
    pub counts: perf_counts_values,
    pub nr: c_int,
    pub failed: bool_,
    pub used: bool_,
}

#[repr(C)]
pub struct perf_stat_evsel {
    pub res_stats: stats,
    pub aggr: *mut perf_stat_aggr,
    pub nr_aggr: c_int,
    pub group_data: *mut c_void,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct aggr_cpu_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aggr_cpu_map {
    pub nr: c_int,
    pub map: *mut aggr_cpu_id,
}

#[repr(C)]
pub struct evsel_core {
    pub threads: *mut perf_thread_map,
    pub cpus: *mut perf_cpu_map,
    pub node: list_head,
}

#[repr(C)]
pub struct evsel {
    pub stats: *mut perf_stat_evsel,
    pub prev_raw_counts: *mut perf_counts,
    pub counts: *mut perf_counts,
    pub core: evsel_core,
    pub per_pkg_mask: *mut hashmap,
    pub per_pkg: bool_,
    pub snapshot: bool_,
    pub err: c_int,
    pub evlist: *mut evlist,
    pub first_wildcard_match: *mut evsel,
    pub pmu: *mut perf_pmu,
    pub percore: bool_,
    pub supported: bool_,
}

#[repr(C)]
pub struct perf_counts {
    pub scaled: c_int,
}

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_map: *mut aggr_cpu_map,
    pub aggr_mode: c_int,
    pub scale: bool_,
    pub system_wide: bool_,
    pub hybrid_merge: bool_,
    pub output: *mut FILE,
    pub interval: u32,
    pub aggr_get_id: unsafe extern "C" fn(*mut perf_stat_config, perf_cpu) -> aggr_cpu_id,
}

#[repr(C)]
pub struct perf_pmu {
    pub is_core: bool_,
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_record_stat {
    pub id: u64,
    pub cpu: c_int,
    pub thread: c_int,
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_record_stat_round {
    pub time: u64,
    pub type_: c_int,
}

#[repr(C)]
pub struct perf_record_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    pub stat: core::mem::ManuallyDrop<perf_record_stat>,
    pub stat_round: core::mem::ManuallyDrop<perf_record_stat_round>,
    pub stat_config: core::mem::ManuallyDrop<perf_record_stat_config>,
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

const AGGR_GLOBAL: c_int = 0;
const AGGR_THREAD: c_int = 1;
const AGGR_NONE: c_int = 2;
const PERF_STAT_ROUND_TYPE__FINAL: c_int = 0;

unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(_evlist: *mut evlist, _f: F) {
    // C list macro evlist__for_each_entry(evlist, evsel): provided by perf headers.
    todo!("external list iteration macro")
}

unsafe fn perf_cpu_map_for_each_idx<F: FnMut(c_int)>(_cpus: *mut perf_cpu_map, _f: F) {
    // C macro perf_cpu_map__for_each_idx(idx, cpus): provided by perf headers.
    todo!("external CPU map index iteration macro")
}

unsafe fn perf_cpu_map_for_each_cpu<F: FnMut(perf_cpu, c_int)>(_cpus: *mut perf_cpu_map, _f: F) {
    // C macro perf_cpu_map__for_each_cpu(cpu, idx, cpus): provided by perf headers.
    todo!("external CPU map iteration macro")
}

#[no_mangle]
pub unsafe extern "C" fn update_stats(stats: *mut stats, val: u64) {
    let delta: f64;

    (*stats).n += 1.0;
    delta = val as f64 - (*stats).mean;
    (*stats).mean += delta / (*stats).n;
    (*stats).M2 += delta * (val as f64 - (*stats).mean);

    if val > (*stats).max {
        (*stats).max = val;
    }

    if val < (*stats).min {
        (*stats).min = val;
    }
}

#[no_mangle]
pub unsafe extern "C" fn avg_stats(stats: *mut stats) -> f64 {
    (*stats).mean
}

/*
 * http://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
 *
 *       (\Sum n_i^2) - ((\Sum n_i)^2)/n
 * s^2 = -------------------------------
 *                  n - 1
 *
 * http://en.wikipedia.org/wiki/Stddev
 *
 * The std dev of the mean is related to the std dev by:
 *
 *             s
 * s_mean = -------
 *          sqrt(n)
 *
 */
#[no_mangle]
pub unsafe extern "C" fn stddev_stats(stats: *mut stats) -> f64 {
    let variance: f64;
    let variance_mean: f64;

    if (*stats).n < 2.0 {
        return 0.0;
    }

    variance = (*stats).M2 / ((*stats).n - 1.0);
    variance_mean = variance / (*stats).n;

    sqrt(variance_mean)
}

#[no_mangle]
pub unsafe extern "C" fn rel_stddev_stats(stddev: f64, avg: f64) -> f64 {
    let mut pct = 0.0;

    if avg != 0.0 {
        pct = 100.0 * stddev / avg;
    }

    pct
}

unsafe fn evsel__reset_aggr_stats(evsel: *mut evsel) {
    let ps = (*evsel).stats;
    let aggr = (*ps).aggr;

    if !aggr.is_null() {
        memset(
            aggr as *mut c_void,
            0,
            core::mem::size_of::<perf_stat_aggr>() * (*ps).nr_aggr as usize,
        );
    }
}

unsafe fn evsel__reset_stat_priv(evsel: *mut evsel) {
    let ps = (*evsel).stats;

    init_stats(&mut (*ps).res_stats);
    evsel__reset_aggr_stats(evsel);
}

unsafe fn evsel__alloc_aggr_stats(evsel: *mut evsel, nr_aggr: c_int) -> c_int {
    let ps = (*evsel).stats;

    if ps.is_null() {
        return 0;
    }

    (*ps).nr_aggr = nr_aggr;
    (*ps).aggr = calloc(nr_aggr as usize, core::mem::size_of::<perf_stat_aggr>()) as *mut perf_stat_aggr;
    if (*ps).aggr.is_null() {
        return -ENOMEM;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr_aggr: c_int) -> c_int {
    let mut ret = 0;

    evlist_for_each_entry(evlist, |evsel| {
        if ret == 0 && evsel__alloc_aggr_stats(evsel, nr_aggr) < 0 {
            ret = -1;
        }
    });
    ret
}

unsafe fn evsel__alloc_stat_priv(evsel: *mut evsel, nr_aggr: c_int) -> c_int {
    let ps: *mut perf_stat_evsel;

    ps = zalloc(core::mem::size_of::<perf_stat_evsel>()) as *mut perf_stat_evsel;
    if ps.is_null() {
        return -ENOMEM;
    }

    (*evsel).stats = ps;

    if nr_aggr != 0 && evsel__alloc_aggr_stats(evsel, nr_aggr) < 0 {
        (*evsel).stats = core::ptr::null_mut();
        free(ps as *mut c_void);
        return -ENOMEM;
    }

    evsel__reset_stat_priv(evsel);
    0
}

unsafe fn evsel__free_stat_priv(evsel: *mut evsel) {
    let ps = (*evsel).stats;

    if !ps.is_null() {
        zfree(&mut (*ps).aggr as *mut *mut perf_stat_aggr as *mut c_void);
        zfree(&mut (*ps).group_data as *mut *mut c_void as *mut c_void);
    }
    zfree(&mut (*evsel).stats as *mut *mut perf_stat_evsel as *mut c_void);
}

unsafe fn evsel__alloc_prev_raw_counts(evsel: *mut evsel) -> c_int {
    let cpu_map_nr = evsel__nr_cpus(evsel);
    let nthreads = perf_thread_map__nr((*evsel).core.threads);
    let counts: *mut perf_counts;

    counts = perf_counts__new(cpu_map_nr, nthreads);
    if !counts.is_null() {
        (*evsel).prev_raw_counts = counts;
    }

    if !counts.is_null() { 0 } else { -ENOMEM }
}

unsafe fn evsel__free_prev_raw_counts(evsel: *mut evsel) {
    perf_counts__delete((*evsel).prev_raw_counts);
    (*evsel).prev_raw_counts = core::ptr::null_mut();
}

unsafe fn evsel__reset_prev_raw_counts(evsel: *mut evsel) {
    if !(*evsel).prev_raw_counts.is_null() {
        perf_counts__reset((*evsel).prev_raw_counts);
    }
}

unsafe fn evsel__alloc_stats(evsel: *mut evsel, nr_aggr: c_int, alloc_raw: bool_) -> c_int {
    if evsel__alloc_stat_priv(evsel, nr_aggr) < 0
        || evsel__alloc_counts(evsel) < 0
        || (alloc_raw && evsel__alloc_prev_raw_counts(evsel) < 0)
    {
        return -ENOMEM;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn evlist__alloc_stats(
    config: *mut perf_stat_config,
    evlist: *mut evlist,
    alloc_raw: bool_,
) -> c_int {
    let mut nr_aggr = 0;
    let mut failed = false;

    if !config.is_null() && !(*config).aggr_map.is_null() {
        nr_aggr = (*(*config).aggr_map).nr;
    }

    evlist_for_each_entry(evlist, |evsel| {
        if !failed && evsel__alloc_stats(evsel, nr_aggr, alloc_raw) != 0 {
            failed = true;
        }
    });

    if !failed {
        return 0;
    }

    evlist__free_stats(evlist);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn evlist__free_stats(evlist: *mut evlist) {
    evlist_for_each_entry(evlist, |evsel| {
        evsel__free_stat_priv(evsel);
        evsel__free_counts(evsel);
        evsel__free_prev_raw_counts(evsel);
    });
}

#[no_mangle]
pub unsafe extern "C" fn evlist__reset_stats(evlist: *mut evlist) {
    evlist_for_each_entry(evlist, |evsel| {
        evsel__reset_stat_priv(evsel);
        evsel__reset_counts(evsel);
    });
}

#[no_mangle]
pub unsafe extern "C" fn evlist__reset_aggr_stats(evlist: *mut evlist) {
    evlist_for_each_entry(evlist, |evsel| evsel__reset_aggr_stats(evsel));
}

#[no_mangle]
pub unsafe extern "C" fn evlist__reset_prev_raw_counts(evlist: *mut evlist) {
    evlist_for_each_entry(evlist, |evsel| evsel__reset_prev_raw_counts(evsel));
}

unsafe fn evsel__copy_prev_raw_counts(evsel: *mut evsel) {
    let nthreads = perf_thread_map__nr((*evsel).core.threads);

    for thread in 0..nthreads {
        perf_cpu_map_for_each_idx(evsel__cpus(evsel), |idx| {
            *perf_counts((*evsel).counts, idx, thread) =
                *perf_counts((*evsel).prev_raw_counts, idx, thread);
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn evlist__copy_prev_raw_counts(evlist: *mut evlist) {
    evlist_for_each_entry(evlist, |evsel| evsel__copy_prev_raw_counts(evsel));
}

unsafe fn evsel__copy_res_stats(evsel: *mut evsel) {
    let ps = (*evsel).stats;

    /*
     * For GLOBAL aggregation mode, it updates the counts for each run
     * in the evsel->stats.res_stats.  See perf_stat_process_counter().
     */
    (*(*ps).aggr.add(0)).counts.val = avg_stats(&mut (*ps).res_stats);
}

#[no_mangle]
pub unsafe extern "C" fn evlist__copy_res_stats(config: *mut perf_stat_config, evlist: *mut evlist) {
    if (*config).aggr_mode != AGGR_GLOBAL {
        return;
    }

    evlist_for_each_entry(evlist, |evsel| evsel__copy_res_stats(evsel));
}

unsafe extern "C" fn pkg_id_hash(__key: c_long, _ctx: *mut c_void) -> size_t {
    let key = __key as *mut u64;

    (*key & 0xffffffff) as size_t
}

unsafe extern "C" fn pkg_id_equal(__key1: c_long, __key2: c_long, _ctx: *mut c_void) -> bool_ {
    let key1 = __key1 as *mut u64;
    let key2 = __key2 as *mut u64;

    *key1 == *key2
}

unsafe fn check_per_pkg(
    counter: *mut evsel,
    vals: *mut perf_counts_values,
    cpu_map_idx: c_int,
    skip: *mut bool_,
) -> c_int {
    let mut mask = (*counter).per_pkg_mask;
    let cpus = evsel__cpus(counter);
    let cpu = perf_cpu_map__cpu(cpus, cpu_map_idx);
    let s: c_int;
    let d: c_int;
    let mut ret = 0;
    let key: *mut u64;

    *skip = false;

    if !(*counter).per_pkg {
        return 0;
    }

    if perf_cpu_map__is_any_cpu_or_is_empty(cpus) {
        return 0;
    }

    if mask.is_null() {
        mask = hashmap__new(Some(pkg_id_hash), Some(pkg_id_equal), core::ptr::null_mut());
        if IS_ERR(mask as *const c_void) {
            return -ENOMEM;
        }

        (*counter).per_pkg_mask = mask;
    }

    /*
     * we do not consider an event that has not run as a good
     * instance to mark a package as used (skip=1). Otherwise
     * we may run into a situation where the first CPU in a package
     * is not running anything, yet the second is, and this function
     * would mark the package as used after the first CPU and would
     * not read the values from the second CPU.
     */
    if !((*vals).run != 0 && (*vals).ena != 0) {
        return 0;
    }

    s = cpu__get_socket_id(cpu);
    if s < 0 {
        return -1;
    }

    /*
     * On multi-die system, die_id > 0. On no-die system, die_id = 0.
     * We use hashmap(socket, die) to check the used socket+die pair.
     */
    d = cpu__get_die_id(cpu);
    if d < 0 {
        return -1;
    }

    key = malloc(core::mem::size_of::<u64>()) as *mut u64;
    if key.is_null() {
        return -ENOMEM;
    }

    *key = ((d as u64) << 32) | s as u64;
    if hashmap__find(mask, key as *const c_void, core::ptr::null_mut()) {
        *skip = true;
        free(key as *mut c_void);
    } else {
        ret = hashmap__add(mask, key as *mut c_void, 1);
    }

    ret
}

unsafe fn evsel__count_has_error(
    evsel: *mut evsel,
    count: *mut perf_counts_values,
    config: *mut perf_stat_config,
) -> bool_ {
    /* the evsel was failed already */
    if (*evsel).err != 0 || (*(*evsel).counts).scaled == -1 {
        return true;
    }

    /* this is meaningful for CPU aggregation modes only */
    if (*config).aggr_mode == AGGR_GLOBAL {
        return false;
    }

    /* it's considered ok when it actually ran */
    if (*count).ena != 0 && (*count).run != 0 {
        return false;
    }

    true
}

unsafe fn process_counter_values(
    config: *mut perf_stat_config,
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
    mut count: *mut perf_counts_values,
) -> c_int {
    let ps = (*evsel).stats;
    static mut ZERO: perf_counts_values = perf_counts_values { val: 0, ena: 0, run: 0 };
    let mut skip = false;

    if check_per_pkg(evsel, count, cpu_map_idx, &mut skip) != 0 {
        pr_err(c"failed to read per-pkg counter\n".as_ptr());
        return -1;
    }

    if skip {
        count = &mut ZERO;
    }

    if !(*evsel).snapshot {
        evsel__compute_deltas(evsel, cpu_map_idx, thread, count);
    }
    perf_counts_values__scale(count, (*config).scale, core::ptr::null_mut());

    if (*config).aggr_mode == AGGR_THREAD {
        let aggr_counts = &mut (*(*ps).aggr.add(thread as usize)).counts as *mut perf_counts_values;

        /*
         * Skip value 0 when enabling --per-thread globally,
         * otherwise too many 0 output.
         */
        if (*count).val == 0 && (*config).system_wide {
            return 0;
        }

        (*(*ps).aggr.add(thread as usize)).nr += 1;

        (*aggr_counts).val += (*count).val;
        (*aggr_counts).ena += (*count).ena;
        (*aggr_counts).run += (*count).run;
        return 0;
    }

    if !(*ps).aggr.is_null() {
        let cpu = perf_cpu_map__cpu((*evsel).core.cpus, cpu_map_idx);
        let aggr_id = ((*config).aggr_get_id)(config, cpu);
        let mut i = 0;

        while i < (*ps).nr_aggr {
            if !aggr_cpu_id__equal(&aggr_id, (*(*config).aggr_map).map.add(i as usize)) {
                i += 1;
                continue;
            }

            let ps_aggr = (*ps).aggr.add(i as usize);
            (*ps_aggr).nr += 1;

            /*
             * When any result is bad, make them all to give consistent output
             * in interval mode.  But per-task counters can have 0 enabled time
             * when some tasks are idle.
             */
            if evsel__count_has_error(evsel, count, config) && !(*ps_aggr).failed {
                (*ps_aggr).counts.val = 0;
                (*ps_aggr).counts.ena = 0;
                (*ps_aggr).counts.run = 0;
                (*ps_aggr).failed = true;
            }

            if !(*ps_aggr).failed {
                (*ps_aggr).counts.val += (*count).val;
                (*ps_aggr).counts.ena += (*count).ena;
                (*ps_aggr).counts.run += (*count).run;
            }
            break;
        }
    }

    0
}

unsafe fn process_counter_maps(config: *mut perf_stat_config, counter: *mut evsel) -> c_int {
    let nthreads = perf_thread_map__nr((*counter).core.threads);
    let ncpus = evsel__nr_cpus(counter);

    for thread in 0..nthreads {
        for idx in 0..ncpus {
            if process_counter_values(
                config,
                counter,
                idx,
                thread,
                perf_counts((*counter).counts, idx, thread),
            ) != 0
            {
                return -1;
            }
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_stat_process_counter(
    config: *mut perf_stat_config,
    counter: *mut evsel,
) -> c_int {
    let ps = (*counter).stats;
    let count: *mut u64;
    let ret: c_int;

    if (*counter).per_pkg {
        evsel__zero_per_pkg(counter);
    }

    ret = process_counter_maps(config, counter);
    if ret != 0 {
        return ret;
    }

    if (*config).aggr_mode != AGGR_GLOBAL {
        return 0;
    }

    /*
     * GLOBAL aggregation mode only has a single aggr counts,
     * so we can use ps->aggr[0] as the actual output.
     */
    count = &mut (*(*ps).aggr.add(0)).counts.val as *mut u64;
    update_stats(&mut (*ps).res_stats, *count);

    if verbose > 0 {
        fprintf(
            (*config).output,
            c"%s: %llu %llu %llu\n".as_ptr(),
            evsel__name(counter),
            *count.add(0),
            *count.add(1),
            *count.add(2),
        );
    }

    0
}

unsafe fn evsel__merge_aggr_counters(evsel: *mut evsel, alias: *mut evsel) -> c_int {
    let ps_a = (*evsel).stats;
    let ps_b = (*alias).stats;
    let mut i: c_int;

    if (*ps_a).aggr.is_null() && (*ps_b).aggr.is_null() {
        return 0;
    }

    if (*ps_a).nr_aggr != (*ps_b).nr_aggr {
        pr_err(c"Unmatched aggregation mode between aliases\n".as_ptr());
        return -1;
    }

    i = 0;
    while i < (*ps_a).nr_aggr {
        let aggr_counts_a = &mut (*(*ps_a).aggr.add(i as usize)).counts as *mut perf_counts_values;
        let aggr_counts_b = &mut (*(*ps_b).aggr.add(i as usize)).counts as *mut perf_counts_values;

        (*(*ps_a).aggr.add(i as usize)).nr += (*(*ps_b).aggr.add(i as usize)).nr;

        (*aggr_counts_a).val += (*aggr_counts_b).val;
        (*aggr_counts_a).ena += (*aggr_counts_b).ena;
        (*aggr_counts_a).run += (*aggr_counts_b).run;
        i += 1;
    }

    0
}

unsafe fn evsel__merge_aliases(_evsel: *mut evsel) {
    /*
     * struct evlist *evlist = evsel->evlist;
     * struct evsel *alias;
     *
     * alias = list_prepare_entry(evsel, &(evlist__core(evlist)->entries), core.node);
     * list_for_each_entry_continue(alias, &evlist__core(evlist)->entries, core.node) {
     *     if (alias->first_wildcard_match == evsel) {
     *         Merge the same events on different PMUs.
     *         evsel__merge_aggr_counters(evsel, alias);
     *     }
     * }
     */
    todo!("external list_prepare_entry/list_for_each_entry_continue macros")
}

unsafe fn evsel__should_merge_hybrid(evsel: *const evsel, config: *const perf_stat_config) -> bool_ {
    (*config).hybrid_merge && evsel__is_hybrid(evsel)
}

unsafe fn evsel__merge_stats(evsel: *mut evsel, config: *mut perf_stat_config) {
    if (*evsel).pmu.is_null()
        || !(*(*evsel).pmu).is_core
        || evsel__should_merge_hybrid(evsel, config)
    {
        evsel__merge_aliases(evsel);
    }
}

/* merge the same uncore and hybrid events if requested */
#[no_mangle]
pub unsafe extern "C" fn perf_stat_merge_counters(config: *mut perf_stat_config, evlist: *mut evlist) {
    if (*config).aggr_mode == AGGR_NONE {
        return;
    }

    evlist_for_each_entry(evlist, |evsel| evsel__merge_stats(evsel, config));
}

unsafe fn evsel__update_percore_stats(evsel: *mut evsel, core_id: *mut aggr_cpu_id) {
    let ps = (*evsel).stats;
    let mut counts = perf_counts_values { val: 0, ena: 0, run: 0 };

    /* collect per-core counts */
    perf_cpu_map_for_each_cpu((*evsel).core.cpus, |cpu, idx| {
        let aggr = (*ps).aggr.add(idx as usize);

        let id = aggr_cpu_id__core(cpu, core::ptr::null_mut());
        if !aggr_cpu_id__equal(core_id, &id) {
            return;
        }

        counts.val += (*aggr).counts.val;
        counts.ena += (*aggr).counts.ena;
        counts.run += (*aggr).counts.run;
    });

    /* update aggregated per-core counts for each CPU */
    perf_cpu_map_for_each_cpu((*evsel).core.cpus, |cpu, idx| {
        let aggr = (*ps).aggr.add(idx as usize);

        let id = aggr_cpu_id__core(cpu, core::ptr::null_mut());
        if !aggr_cpu_id__equal(core_id, &id) {
            return;
        }

        (*aggr).counts.val = counts.val;
        (*aggr).counts.ena = counts.ena;
        (*aggr).counts.run = counts.run;

        (*aggr).used = true;
    });
}

/* we have an aggr_map for cpu, but want to aggregate the counters per-core */
unsafe fn evsel__process_percore(evsel: *mut evsel) {
    let ps = (*evsel).stats;

    if !(*evsel).percore {
        return;
    }

    perf_cpu_map_for_each_cpu((*evsel).core.cpus, |cpu, idx| {
        let aggr = (*ps).aggr.add(idx as usize);

        if (*aggr).used {
            return;
        }

        let mut core_id = aggr_cpu_id__core(cpu, core::ptr::null_mut());
        evsel__update_percore_stats(evsel, &mut core_id);
    });
}

/* process cpu stats on per-core events */
#[no_mangle]
pub unsafe extern "C" fn perf_stat_process_percore(
    config: *mut perf_stat_config,
    evlist: *mut evlist,
) {
    if (*config).aggr_mode != AGGR_NONE {
        return;
    }

    evlist_for_each_entry(evlist, |evsel| evsel__process_percore(evsel));
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_stat_event(
    _tool: *const perf_tool,
    session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    let mut count: perf_counts_values;
    let ptr: *mut perf_counts_values;
    let st = &(*event).stat as *const _ as *mut perf_record_stat;
    let counter: *mut evsel;
    let cpu_map_idx: c_int;

    count.val = (*st).val;
    count.ena = (*st).ena;
    count.run = (*st).run;

    counter = evlist__id2evsel((*session).evlist, (*st).id);
    if counter.is_null() {
        pr_err(c"Failed to resolve counter for stat event.\n".as_ptr());
        return -EINVAL;
    }
    cpu_map_idx = perf_cpu_map__idx(evsel__cpus(counter), perf_cpu { cpu: (*st).cpu });
    if cpu_map_idx == -1 {
        pr_err(
            c"Invalid CPU %d for event %s.\n".as_ptr(),
            (*st).cpu,
            evsel__name(counter),
        );
        return -EINVAL;
    }
    ptr = perf_counts((*counter).counts, cpu_map_idx, (*st).thread);
    if ptr.is_null() {
        pr_err(
            c"Failed to find perf count for CPU %d thread %d on event %s.\n".as_ptr(),
            (*st).cpu,
            (*st).thread,
            evsel__name(counter),
        );
        return -EINVAL;
    }
    *ptr = count;
    (*counter).supported = true;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_stat(event: *mut perf_event, fp: *mut FILE) -> size_t {
    let st = event as *mut perf_record_stat;
    let mut ret: size_t;

    ret = fprintf(
        fp,
        c"\n... id %llu, cpu %d, thread %d\n".as_ptr(),
        (*st).id,
        (*st).cpu,
        (*st).thread,
    ) as size_t;
    ret += fprintf(
        fp,
        c"... value %llu, enabled %llu, running %llu\n".as_ptr(),
        (*st).val,
        (*st).ena,
        (*st).run,
    ) as size_t;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_stat_round(
    event: *mut perf_event,
    fp: *mut FILE,
) -> size_t {
    let rd = event as *mut perf_record_stat_round;
    let ret: size_t;

    ret = fprintf(
        fp,
        c"\n... time %llu, type %s\n".as_ptr(),
        (*rd).time,
        if (*rd).type_ == PERF_STAT_ROUND_TYPE__FINAL {
            c"FINAL".as_ptr()
        } else {
            c"INTERVAL".as_ptr()
        },
    ) as size_t;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_stat_config(
    event: *mut perf_event,
    fp: *mut FILE,
) -> size_t {
    let mut sc: perf_stat_config = core::mem::zeroed();
    let mut ret: size_t;

    perf_event__read_stat_config(&mut sc, &(*event).stat_config as *const _ as *const perf_record_stat_config);

    ret = fprintf(fp, c"\n".as_ptr()) as size_t;
    ret += fprintf(fp, c"... aggr_mode %d\n".as_ptr(), sc.aggr_mode) as size_t;
    ret += fprintf(fp, c"... scale     %d\n".as_ptr(), sc.scale as c_int) as size_t;
    ret += fprintf(fp, c"... interval  %u\n".as_ptr(), sc.interval) as size_t;

    ret
}
