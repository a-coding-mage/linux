// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/util/stat-shadow.c.
// C include dependencies are represented as external declarations and opaque
// C-compatible types below.

use core::ffi::{c_char, c_double, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const ENOMEM: c_int = 12;
const FP_ZERO: c_int = 2;

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_map: *mut cpu_aggr_map,
    pub user_requested_cpu_list: *const c_char,
    pub system_wide: bool,
    pub metric_only: bool,
    pub iostat_run: bool,
}

#[repr(C)]
pub struct cpu_aggr_map {
    pub nr: c_int,
    pub map: *mut cpu_aggr_map_entry,
}

#[repr(C)]
pub struct cpu_aggr_map_entry {
    pub cpu: perf_cpu,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub stats: *mut perf_stat_evsel,
    pub metric_leader: *mut evsel,
    pub pmu: *const perf_pmu,
    pub evlist: *mut evlist,
    pub supported: bool,
    pub name: *const c_char,
    pub default_show_events: bool,
    pub default_metricgroup: bool,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_stat_evsel {
    pub aggr: *mut perf_stat_aggr,
}

#[repr(C)]
pub struct perf_stat_aggr {
    pub counts: perf_counts_values,
    pub nr: c_int,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: c_double,
    pub ena: c_double,
    pub run: c_double,
}

#[repr(C)]
pub struct metric_expr {
    pub metric_events: *mut *mut evsel,
    pub metric_refs: *mut metric_ref,
    pub metric_name: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub metric_unit: *const c_char,
    pub runtime: c_int,
    pub default_metricgroup_name: *const c_char,
    pub nd: list_head,
}

#[repr(C)]
pub struct metric_ref {
    pub metric_name: *const c_char,
}

#[repr(C)]
pub struct expr_parse_ctx {
    pub sctx: expr_scanner_ctx,
}

#[repr(C)]
pub struct expr_scanner_ctx {
    pub user_requested_cpu_list: *mut c_char,
    pub runtime: c_int,
    pub system_wide: bool,
}

#[repr(C)]
pub struct perf_stat_output_ctx {
    pub print_metric: print_metric_t,
    pub print_metricgroup_header: print_metricgroup_header_t,
    pub new_line: Option<unsafe extern "C" fn(*mut perf_stat_config, *mut c_void)>,
    pub ctx: *mut c_void,
    pub force_header: bool,
}

#[repr(C)]
pub struct metric_event {
    pub head: list_head,
    pub is_default: bool,
}

#[repr(C)]
pub struct rblist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tool_pmu_event {
    TOOL_PMU__EVENT_DURATION_TIME,
    TOOL_PMU__EVENT_USER_TIME,
    TOOL_PMU__EVENT_SYSTEM_TIME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metric_threshold_classify {
    METRIC_THRESHOLD_UNKNOWN,
    METRIC_THRESHOLD_GOOD,
    METRIC_THRESHOLD_BAD,
}

pub type print_metric_t = Option<
    unsafe extern "C" fn(
        *mut perf_stat_config,
        *mut c_void,
        metric_threshold_classify,
        *const c_char,
        *const c_char,
        c_double,
    ),
>;

pub type print_metricgroup_header_t =
    Option<unsafe extern "C" fn(*mut perf_stat_config, *mut c_void, *const c_char)>;

unsafe extern "C" {
    fn evsel__tool_event(evsel: *const evsel) -> tool_pmu_event;
    fn pr_debug(fmt: *const c_char, ...);
    fn evsel__source_count(evsel: *mut evsel) -> c_int;
    fn evsel__metric_id(evsel: *mut evsel) -> *const c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn expr__add_id_val_source_count_aggr_nr(
        ctx: *mut expr_parse_ctx,
        id: *mut c_char,
        val: c_double,
        source_count: c_int,
        aggr_nr: c_int,
    );
    fn expr__add_ref(ctx: *mut expr_parse_ctx, mref: *mut metric_ref) -> c_int;
    fn expr__ctx_new() -> *mut expr_parse_ctx;
    fn expr__ctx_free(ctx: *mut expr_parse_ctx);
    fn expr__parse(val: *mut c_double, ctx: *mut expr_parse_ctx, expr: *const c_char) -> c_int;
    fn isnan(x: c_double) -> c_int;
    fn fpclassify(x: c_double) -> c_int;
    fn perf_pmu__convert_scale(
        unit: *const c_char,
        new_unit: *mut *mut c_char,
        scale: *mut c_double,
    ) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn evlist__metric_events(evlist: *mut evlist) -> *mut rblist;
    fn metricgroup__lookup(
        metric_events: *mut rblist,
        evsel: *mut evsel,
        create: bool,
    ) -> *mut metric_event;
    fn iostat_print_metric(
        config: *mut perf_stat_config,
        evsel: *mut evsel,
        out: *mut perf_stat_output_ctx,
    );
}

#[inline]
unsafe fn c_str_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

#[inline]
unsafe fn container_of_metric_expr(ptr: *mut list_head) -> *mut metric_expr {
    (ptr as *mut u8).sub(core::mem::offset_of!(metric_expr, nd)) as *mut metric_expr
}

unsafe fn tool_pmu__is_time_event(
    config: *const perf_stat_config,
    evsel: *const evsel,
    tool_aggr_idx: *mut c_int,
) -> bool {
    let event = evsel__tool_event(evsel);
    let mut aggr_idx: c_int;

    if event != tool_pmu_event::TOOL_PMU__EVENT_DURATION_TIME
        && event != tool_pmu_event::TOOL_PMU__EVENT_USER_TIME
        && event != tool_pmu_event::TOOL_PMU__EVENT_SYSTEM_TIME
    {
        return false;
    }

    if !config.is_null() {
        aggr_idx = 0;
        while aggr_idx < (*(*config).aggr_map).nr {
            if (*(*(*config).aggr_map).map.add(aggr_idx as usize)).cpu.cpu == 0 {
                *tool_aggr_idx = aggr_idx;
                return true;
            }
            aggr_idx += 1;
        }
        pr_debug(c_str_lit(b"Unexpected CPU0 missing in aggregation for tool event.\n\0"));
    }
    *tool_aggr_idx = 0; /* Assume the first aggregation index works. */
    true
}

unsafe fn prepare_metric(
    config: *mut perf_stat_config,
    mexp: *const metric_expr,
    evsel: *const evsel,
    pctx: *mut expr_parse_ctx,
    aggr_idx: c_int,
) -> c_int {
    let metric_events = (*mexp).metric_events;
    let metric_refs = (*mexp).metric_refs;
    let mut i: c_int = 0;

    while !(*metric_events.add(i as usize)).is_null() {
        let mut source_count: c_int = 0;
        let mut tool_aggr_idx: c_int = MaybeUninit::<c_int>::uninit().assume_init();
        let mut aggr_nr: c_int = 1;
        let is_tool_time =
            tool_pmu__is_time_event(config, *metric_events.add(i as usize), &mut tool_aggr_idx);
        let mut ps = (**metric_events.add(i as usize)).stats;
        let n: *mut c_char;
        let mut val: c_double;

        /*
         * If there are multiple uncore PMUs and we're not reading the
         * leader's stats, determine the stats for the appropriate
         * uncore PMU.
         */
        if !evsel.is_null()
            && !(*evsel).metric_leader.is_null()
            && (*evsel).pmu != (*(*evsel).metric_leader).pmu
            && (**metric_events.add(i as usize)).pmu == (*(*evsel).metric_leader).pmu
        {
            /* evlist__for_each_entry(evsel->evlist, pos) */
            let mut pos: *mut evsel = ptr::null_mut();
            /*
             * TODO: external intrusive evlist iteration macro dependency.
             * The loop body is preserved below as the source-level operation
             * applied to each `pos` yielded by evlist__for_each_entry.
             */
            while !pos.is_null() {
                if (*pos).pmu != (*evsel).pmu {
                    continue;
                }
                if (*pos).metric_leader != *metric_events.add(i as usize) {
                    continue;
                }
                ps = (*pos).stats;
                source_count = 1;
                break;
            }
        }
        /* Time events are always on CPU0, the first aggregation index. */
        if ps.is_null() || !(**metric_events.add(i as usize)).supported {
            /*
             * Not supported events will have a count of 0, which
             * can be confusing in a metric. Explicitly set the
             * value to NAN. Not counted events (enable time of 0)
             * are read as 0.
             */
            val = c_double::NAN;
            source_count = 0;
            aggr_nr = 0;
        } else {
            let aggr = (*ps)
                .aggr
                .add(if is_tool_time { tool_aggr_idx } else { aggr_idx } as usize);

            if (*aggr).counts.run == 0.0 {
                val = c_double::NAN;
                source_count = 0;
                aggr_nr = 0;
            } else {
                val = (*aggr).counts.val;
                if is_tool_time {
                    /* Convert time event nanoseconds to seconds. */
                    val *= 1e-9;
                }
                if source_count == 0 {
                    source_count = evsel__source_count(*metric_events.add(i as usize));
                }
                aggr_nr = if (*aggr).nr != 0 { (*aggr).nr } else { 1 };
            }
        }
        n = strdup(evsel__metric_id(*metric_events.add(i as usize)));
        if n.is_null() {
            return -ENOMEM;
        }

        expr__add_id_val_source_count_aggr_nr(pctx, n, val, source_count, aggr_nr);
        i += 1;
    }

    let mut j: c_int = 0;
    while !metric_refs.is_null() && !(*metric_refs.add(j as usize)).metric_name.is_null() {
        let ret = expr__add_ref(pctx, metric_refs.add(j as usize));

        if ret != 0 {
            return ret;
        }
        j += 1;
    }

    i
}

unsafe fn generic_metric(
    config: *mut perf_stat_config,
    mexp: *mut metric_expr,
    evsel: *mut evsel,
    aggr_idx: c_int,
    out: *mut perf_stat_output_ctx,
) {
    let print_metric = (*out).print_metric;
    let metric_name = (*mexp).metric_name;
    let metric_expr = (*mexp).metric_expr;
    let metric_threshold = (*mexp).metric_threshold;
    let metric_unit = (*mexp).metric_unit;
    let metric_events = (*mexp).metric_events;
    let runtime = (*mexp).runtime;
    let pctx: *mut expr_parse_ctx;
    let mut ratio: c_double = 0.0;
    let mut scale: c_double = 0.0;
    let mut threshold: c_double = 0.0;
    let i: c_int;
    let ctxp = (*out).ctx;
    let mut thresh = metric_threshold_classify::METRIC_THRESHOLD_UNKNOWN;

    pctx = expr__ctx_new();
    if pctx.is_null() {
        return;
    }

    if !(*config).user_requested_cpu_list.is_null() {
        (*pctx).sctx.user_requested_cpu_list = strdup((*config).user_requested_cpu_list);
    }
    (*pctx).sctx.runtime = runtime;
    (*pctx).sctx.system_wide = (*config).system_wide;
    i = prepare_metric(config, mexp, evsel, pctx, aggr_idx);
    if i < 0 {
        expr__ctx_free(pctx);
        return;
    }
    if (*metric_events.add(i as usize)).is_null() {
        if expr__parse(&mut ratio, pctx, metric_expr) == 0 {
            let mut unit: *mut c_char = ptr::null_mut();
            let mut metric_bf = [0 as c_char; 128];

            if !metric_threshold.is_null()
                && expr__parse(&mut threshold, pctx, metric_threshold) == 0
                && isnan(threshold) == 0
            {
                thresh = if fpclassify(threshold) == FP_ZERO {
                    metric_threshold_classify::METRIC_THRESHOLD_GOOD
                } else {
                    metric_threshold_classify::METRIC_THRESHOLD_BAD
                };
            }

            if !metric_unit.is_null() && !metric_name.is_null() {
                if perf_pmu__convert_scale(metric_unit, &mut unit, &mut scale) >= 0 {
                    ratio *= scale;
                }
                if !strstr(metric_expr, c_str_lit(b"?\0")).is_null() {
                    scnprintf(
                        metric_bf.as_mut_ptr(),
                        metric_bf.len(),
                        c_str_lit(b"%s  %s_%d\0"),
                        unit,
                        metric_name,
                        runtime,
                    );
                } else {
                    scnprintf(
                        metric_bf.as_mut_ptr(),
                        metric_bf.len(),
                        c_str_lit(b"%s  %s\0"),
                        unit,
                        metric_name,
                    );
                }

                if let Some(print_metric) = print_metric {
                    print_metric(
                        config,
                        ctxp,
                        thresh,
                        c_str_lit(b"%8.1f\0"),
                        metric_bf.as_ptr(),
                        ratio,
                    );
                }
            } else if let Some(print_metric) = print_metric {
                print_metric(
                    config,
                    ctxp,
                    thresh,
                    c_str_lit(b"%8.2f\0"),
                    if !metric_name.is_null() {
                        metric_name
                    } else if (*out).force_header {
                        (*evsel).name
                    } else {
                        c_str_lit(b"\0")
                    },
                    ratio,
                );
            }
        } else if let Some(print_metric) = print_metric {
            print_metric(
                config,
                ctxp,
                thresh,
                ptr::null(),
                if (*out).force_header {
                    if !metric_name.is_null() {
                        metric_name
                    } else {
                        (*evsel).name
                    }
                } else {
                    c_str_lit(b"\0")
                },
                0.0,
            );
        }
    } else if let Some(print_metric) = print_metric {
        print_metric(
            config,
            ctxp,
            thresh,
            ptr::null(),
            if (*out).force_header {
                if !metric_name.is_null() {
                    metric_name
                } else {
                    (*evsel).name
                }
            } else {
                c_str_lit(b"\0")
            },
            0.0,
        );
    }

    expr__ctx_free(pctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_generic_metric(mexp: *mut metric_expr, aggr_idx: c_int) -> c_double {
    let pctx: *mut expr_parse_ctx;
    let mut ratio: c_double = 0.0;

    pctx = expr__ctx_new();
    if pctx.is_null() {
        return c_double::NAN;
    }

    if prepare_metric(ptr::null_mut(), mexp, ptr::null(), pctx, aggr_idx) < 0 {
        goto_out(pctx);
        return ratio;
    }

    if expr__parse(&mut ratio, pctx, (*mexp).metric_expr) != 0 {
        ratio = 0.0;
    }

    expr__ctx_free(pctx);
    ratio
}

#[inline]
unsafe fn goto_out(pctx: *mut expr_parse_ctx) {
    expr__ctx_free(pctx);
}

unsafe fn perf_stat__print_metricgroup_header(
    config: *mut perf_stat_config,
    evsel: *mut evsel,
    ctxp: *mut c_void,
    name: *const c_char,
    out: *mut perf_stat_output_ctx,
) {
    let need_full_name = perf_pmus__num_core_pmus() > 1;
    static mut LAST_NAME: *const c_char = ptr::null();
    static mut LAST_PMU: *const perf_pmu = ptr::null();
    let mut full_name = [0 as c_char; 64];

    /*
     * A metricgroup may have several metric events,
     * e.g.,TopdownL1 on e-core of ADL.
     * The name has been output by the first metric
     * event. Only align with other metics from
     * different metric events.
     */
    if !LAST_NAME.is_null() && strcmp(LAST_NAME, name) == 0 && LAST_PMU == (*evsel).pmu {
        if let Some(print_metricgroup_header) = (*out).print_metricgroup_header {
            print_metricgroup_header(config, ctxp, ptr::null());
        }
        return;
    }

    if need_full_name && !(*evsel).pmu.is_null() {
        scnprintf(
            full_name.as_mut_ptr(),
            full_name.len(),
            c_str_lit(b"%s (%s)\0"),
            name,
            (*(*evsel).pmu).name,
        );
    } else {
        scnprintf(
            full_name.as_mut_ptr(),
            full_name.len(),
            c_str_lit(b"%s\0"),
            name,
        );
    }

    if let Some(print_metricgroup_header) = (*out).print_metricgroup_header {
        print_metricgroup_header(config, ctxp, full_name.as_ptr());
    }

    LAST_NAME = name;
    LAST_PMU = (*evsel).pmu;
}

/**
 * perf_stat__print_shadow_stats_metricgroup - Print out metrics associated with the evsel
 *					       For the non-default, all metrics associated
 *					       with the evsel are printed.
 *					       For the default mode, only the metrics from
 *					       the same metricgroup and the name of the
 *					       metricgroup are printed. To print the metrics
 *					       from the next metricgroup (if available),
 *					       invoke the function with correspoinding
 *					       metric_expr.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_stat__print_shadow_stats_metricgroup(
    config: *mut perf_stat_config,
    evsel: *mut evsel,
    aggr_idx: c_int,
    num: *mut c_int,
    from: *mut c_void,
    out: *mut perf_stat_output_ctx,
) -> *mut c_void {
    let me: *mut metric_event;
    let mut mexp = from as *mut metric_expr;
    let ctxp = (*out).ctx;
    let mut header_printed = false;
    let mut name: *const c_char = ptr::null();
    let metric_events = evlist__metric_events((*evsel).evlist);

    me = metricgroup__lookup(metric_events, evsel, false);
    if me.is_null() {
        return ptr::null_mut();
    }

    if mexp.is_null() {
        mexp = container_of_metric_expr((*me).head.next);
    }

    /* list_for_each_entry_from(mexp, &me->head, nd) */
    while !mexp.is_null() && &mut (*mexp).nd as *mut list_head != &mut (*me).head as *mut list_head
    {
        /* Print the display name of the Default metricgroup */
        if !(*config).metric_only && (*me).is_default {
            if name.is_null() {
                name = (*mexp).default_metricgroup_name;
            }
            /*
             * Two or more metricgroup may share the same metric
             * event, e.g., TopdownL1 and TopdownL2 on SPR.
             * Return and print the prefix, e.g., noise, running
             * for the next metricgroup.
             */
            if strcmp(name, (*mexp).default_metricgroup_name) != 0 {
                return mexp as *mut c_void;
            }
            /* Only print the name of the metricgroup once */
            if !header_printed && !(*evsel).default_show_events {
                header_printed = true;
                perf_stat__print_metricgroup_header(config, evsel, ctxp, name, out);
            }
        }

        let old_num = *num;
        *num += 1;
        if old_num > 0 {
            if let Some(new_line) = (*out).new_line {
                new_line(config, ctxp);
            }
        }
        generic_metric(config, mexp, evsel, aggr_idx, out);
        mexp = container_of_metric_expr((*mexp).nd.next);
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_stat__print_shadow_stats(
    config: *mut perf_stat_config,
    evsel: *mut evsel,
    aggr_idx: c_int,
    out: *mut perf_stat_output_ctx,
) {
    let print_metric = (*out).print_metric;
    let ctxp = (*out).ctx;
    let mut num: c_int = 0;

    if (*config).iostat_run {
        iostat_print_metric(config, evsel, out);
    }

    perf_stat__print_shadow_stats_metricgroup(config, evsel, aggr_idx, &mut num, ptr::null_mut(), out);

    if num == 0 {
        if let Some(print_metric) = print_metric {
            print_metric(
                config,
                ctxp,
                metric_threshold_classify::METRIC_THRESHOLD_UNKNOWN,
                ptr::null(),
                ptr::null(),
                0.0,
            );
        }
    }
}

/**
 * perf_stat__skip_metric_event - Skip the evsel in the Default metricgroup,
 *				  if it's not running or not the metric event.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_stat__skip_metric_event(evsel: *mut evsel) -> bool {
    if !(*evsel).default_metricgroup {
        return false;
    }

    metricgroup__lookup(evlist__metric_events((*evsel).evlist), evsel, false).is_null()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
