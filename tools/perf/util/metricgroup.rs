// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017, Intel Corporation.
 */

/* Manage metrics and groups of metrics from JSON files */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{self, size_of};
use core::ptr;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const TOOL_PMU__EVENT_MAX: usize = 16;
const TOOL_PMU__EVENT_NONE: c_int = 0;
const PMU_METRICS__NOT_FOUND: c_int = -1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rblist {
    pub node_cmp: Option<unsafe extern "C" fn(*mut rb_node, *const c_void) -> c_int>,
    pub node_new: Option<unsafe extern "C" fn(*mut rblist, *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(*mut rblist, *mut rb_node)>,
}

#[repr(C)]
pub struct evsel_core {
    pub idx: c_int,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

#[repr(C)]
pub struct evsel {
    pub metric_leader: *mut evsel,
    pub pmu: *mut perf_pmu,
    pub collect_stat: bool,
    pub metric_id: *const c_char,
    pub name: *mut c_char,
    pub default_show_events: bool,
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist_core {
    pub entries: list_head,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pkey: *const c_char,
}

#[repr(C)]
pub struct expr_id_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct expr_scanner_ctx {
    pub user_requested_cpu_list: *mut c_char,
    pub runtime: c_int,
    pub system_wide: bool,
}

#[repr(C)]
pub struct expr_parse_ctx {
    pub ids: *mut hashmap,
    pub sctx: expr_scanner_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct metric_ref {
    pub metric_name: *const c_char,
    pub metric_expr: *const c_char,
}

#[repr(C)]
pub struct metric_expr {
    pub nd: list_head,
    pub metric_refs: *mut metric_ref,
    pub metric_expr: *const c_char,
    pub metric_name: *mut c_char,
    pub metric_threshold: *const c_char,
    pub metric_unit: *const c_char,
    pub metric_events: *mut *mut evsel,
    pub runtime: c_int,
    pub default_metricgroup_name: *const c_char,
}

#[repr(C)]
pub struct metric_event {
    pub nd: rb_node,
    pub evsel: *mut evsel,
    pub is_default: bool,
    pub head: list_head,
}

#[repr(C)]
pub struct pmu_metrics_table {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_metric {
    pub pmu: *const c_char,
    pub metric_name: *const c_char,
    pub metric_group: *const c_char,
    pub metricgroup_no_group: *const c_char,
    pub default_metricgroup_name: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub unit: *const c_char,
    pub compat: *const c_char,
    pub event_grouping: c_int,
    pub default_show_events: bool,
}

#[repr(C)]
pub struct strbuf {
    pub buf: *mut c_char,
    pub len: usize,
    pub alloc: usize,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub name: *const c_char,
}

type pmu_metric_iter_fn =
    Option<unsafe extern "C" fn(*const pmu_metric, *const pmu_metrics_table, *mut c_void) -> c_int>;

const MetricNoGroupEvents: c_int = 0;
const MetricNoGroupEventsNmi: c_int = 1;
const MetricNoGroupEventsSmt: c_int = 2;
const MetricNoGroupEventsThresholdAndNmi: c_int = 3;
const MetricGroupEvents: c_int = 4;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn rblist__init(rblist: *mut rblist);
    fn rblist__exit(rblist: *mut rblist);
    fn rblist__find(rblist: *mut rblist, entry: *const c_void) -> *mut rb_node;
    fn rblist__add_node(rblist: *mut rblist, entry: *const c_void);
    fn rblist__nr_entries(rblist: *mut rblist) -> c_uint;
    fn rblist__entry(rblist: *mut rblist, idx: c_uint) -> *mut rb_node;

    fn sysctl__nmi_watchdog_enabled() -> bool;
    fn smt_on() -> bool;
    fn expr__ctx_new() -> *mut expr_parse_ctx;
    fn expr__ctx_free(ctx: *mut expr_parse_ctx);
    fn expr__find_ids(expr: *const c_char, one: *mut c_void, ctx: *mut expr_parse_ctx) -> c_int;
    fn expr__del_id(ctx: *mut expr_parse_ctx, id: *const c_char);
    fn expr__add_id(ctx: *mut expr_parse_ctx, id: *mut c_char) -> c_int;
    fn expr__get_id(ctx: *mut expr_parse_ctx, id: *const c_char, data: *mut *mut expr_id_data) -> c_int;
    fn expr__subset_of_ids(a: *mut expr_parse_ctx, b: *mut expr_parse_ctx) -> bool;
    fn ids__insert(ids: *mut hashmap, id: *mut c_char);

    fn hashmap__size(map: *mut hashmap) -> usize;
    fn hashmap__find(map: *mut hashmap, key: *const c_char, val: *mut *mut expr_id_data) -> bool;

    fn perf_pmus__num_core_pmus() -> c_int;
    fn is_pmu_core(pmu: *const c_char) -> bool;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__scan_for_uncore_id(pmu: *mut perf_pmu, compat: *const c_char) -> *mut perf_pmu;
    fn perf_pmu__name_wildcard_match(pmu: *mut perf_pmu, name: *const c_char) -> bool;

    fn pmu_metrics_table__default() -> *const pmu_metrics_table;
    fn pmu_metrics_table__find() -> *const pmu_metrics_table;
    fn pmu_metrics_table__for_each_metric(
        table: *const pmu_metrics_table,
        fn_: pmu_metric_iter_fn,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_metrics_table__find_metric(
        table: *const pmu_metrics_table,
        pmu: *mut perf_pmu,
        name: *const c_char,
        fn_: pmu_metric_iter_fn,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_for_each_sys_metric(fn_: pmu_metric_iter_fn, data: *mut c_void) -> c_int;

    fn tool_pmu__str_to_event(id: *const c_char) -> c_int;
    fn tool_pmu__event_to_str(ev: c_int) -> *const c_char;
    fn evsel__metric_id(evsel: *mut evsel) -> *const c_char;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_hybrid(evsel: *mut evsel) -> bool;
    fn evsel__is_tool(evsel: *mut evsel) -> bool;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__metric_events(evlist: *mut evlist) -> *mut rblist;
    fn evlist__find_evsel(evlist: *mut evlist, idx: c_int) -> *mut evsel;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__splice_list_tail(evlist: *mut evlist, entries: *mut list_head);

    fn strbuf_addch(sb: *mut strbuf, c: c_int) -> c_int;
    fn strbuf_add(sb: *mut strbuf, data: *const c_char, len: usize) -> c_int;
    fn strbuf_addstr(sb: *mut strbuf, s: *const c_char) -> c_int;
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_setlen(sb: *mut strbuf, len: usize) -> c_int;
    fn strbuf_release(sb: *mut strbuf);

    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, s: *const c_char);
    fn __parse_events(
        evlist: *mut evlist,
        str_: *const c_char,
        pmu_filter: *const c_char,
        cputype_filter: bool,
        err: *mut parse_events_error,
        fake_pmu: bool,
        warn_if_reordered: bool,
        fake_tp: bool,
    ) -> c_int;

    fn list_sort(
        priv_: *mut c_void,
        head: *mut list_head,
        cmp: Option<unsafe extern "C" fn(*mut c_void, *const list_head, *const list_head) -> c_int>,
    );
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! offset_of {
    ($ty:ty, $field:tt) => {{
        let uninit = core::mem::MaybeUninit::<$ty>::uninit();
        let base = uninit.as_ptr();
        unsafe { (&(*base).$field as *const _ as usize) - (base as usize) }
    }};
}

unsafe fn container_of<T>(ptr: *mut c_void, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}

unsafe fn zalloc(size: usize) -> *mut c_void {
    calloc(1, size)
}

unsafe fn zfree<T>(p: *mut *mut T) {
    if !(*p).is_null() {
        free(*p as *mut c_void);
        *p = ptr::null_mut();
    }
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add(new_: *mut list_head, head: *mut list_head) {
    (*new_).next = (*head).next;
    (*new_).prev = head;
    (*(*head).next).prev = new_;
    (*head).next = new_;
}

unsafe fn list_splice(list: *mut list_head, head: *mut list_head) {
    if (*list).next == list {
        return;
    }
    let first = (*list).next;
    let last = (*list).prev;
    let at = (*head).next;
    (*first).prev = head;
    (*head).next = first;
    (*last).next = at;
    (*at).prev = last;
}

unsafe fn list_del_init(entry: *mut list_head) {
    let next = (*entry).next;
    let prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
    INIT_LIST_HEAD(entry);
}

unsafe fn metric_list_first(head: *const list_head) -> *mut metric {
    if (*head).next == head as *mut list_head {
        ptr::null_mut()
    } else {
        container_of::<metric>((*head).next as *mut c_void, offset_of!(metric, nd))
    }
}

unsafe fn metric_next(pos: *mut metric, head: *const list_head) -> *mut metric {
    let n = (*pos).nd.next;
    if n == head as *mut list_head {
        ptr::null_mut()
    } else {
        container_of::<metric>(n as *mut c_void, offset_of!(metric, nd))
    }
}

unsafe fn metric_expr_first(head: *const list_head) -> *mut metric_expr {
    if (*head).next == head as *mut list_head {
        ptr::null_mut()
    } else {
        container_of::<metric_expr>((*head).next as *mut c_void, offset_of!(metric_expr, nd))
    }
}

unsafe fn metric_expr_next(pos: *mut metric_expr, head: *const list_head) -> *mut metric_expr {
    let n = (*pos).nd.next;
    if n == head as *mut list_head {
        ptr::null_mut()
    } else {
        container_of::<metric_expr>(n as *mut c_void, offset_of!(metric_expr, nd))
    }
}

/* External macro iteration from headers, preserved as dependency hooks. */
unsafe fn hashmap_for_each_entry(
    _map: *mut hashmap,
    _cb: unsafe fn(*mut hashmap_entry, *mut c_void) -> c_int,
    _data: *mut c_void,
) -> c_int {
    /* TODO: provided by hashmap__for_each_entry macro in translated dependencies. */
    0
}

unsafe fn evlist_for_each_entry(
    _evlist: *mut evlist,
    _cb: unsafe fn(*mut evsel, *mut c_void) -> c_int,
    _data: *mut c_void,
) -> c_int {
    /* TODO: provided by evlist__for_each_entry macro in translated dependencies. */
    0
}

unsafe fn evlist_for_each_entry_continue(
    _evlist: *mut evlist,
    _ev: *mut evsel,
    _cb: unsafe fn(*mut evsel, *mut c_void) -> c_int,
    _data: *mut c_void,
) -> c_int {
    /* TODO: provided by evlist__for_each_entry_continue macro in translated dependencies. */
    0
}

#[repr(C)]
pub struct metric {
    pub nd: list_head,
    /**
     * The expression parse context importantly holding the IDs contained
     * within the expression.
     */
    pub pctx: *mut expr_parse_ctx,
    pub pmu: *const c_char,
    /** The name of the metric such as "IPC". */
    pub metric_name: *const c_char,
    /** Modifier on the metric such as "u" or NULL for none. */
    pub modifier: *mut c_char,
    /** The expression to parse, for example, "instructions/cycles". */
    pub metric_expr: *const c_char,
    /** Optional threshold expression where zero value is green, otherwise red. */
    pub metric_threshold: *const c_char,
    /**
     * The "ScaleUnit" that scales and adds a unit to the metric during
     * output.
     */
    pub metric_unit: *const c_char,
    /**
     * Optional name of the metric group reported
     * if the Default metric group is being processed.
     */
    pub default_metricgroup_name: *const c_char,
    /** Optional null terminated array of referenced metrics. */
    pub metric_refs: *mut metric_ref,
    /**
     * Should events of the metric be grouped?
     */
    pub group_events: bool,
    /** Show events even if in the Default metric group. */
    pub default_show_events: bool,
    /**
     * Parsed events for the metric. Optional as events may be taken from a
     * different metric whose group contains all the IDs necessary for this
     * one.
     */
    pub evlist: *mut evlist,
}

static mut violate_nmi_constraint: bool = false;
static code_characters: &[u8] = b",-=@\0";

#[no_mangle]
pub unsafe extern "C" fn metricgroup__lookup(
    metric_events: *mut rblist,
    evsel: *mut evsel,
    create: bool,
) -> *mut metric_event {
    let mut nd: *mut rb_node;
    let mut me: metric_event = mem::zeroed();
    me.evsel = evsel;

    if metric_events.is_null() {
        return ptr::null_mut();
    }
    if !evsel.is_null() && !(*evsel).metric_leader.is_null() {
        me.evsel = (*evsel).metric_leader;
    }
    nd = rblist__find(metric_events, &me as *const _ as *const c_void);
    if !nd.is_null() {
        return container_of::<metric_event>(nd as *mut c_void, offset_of!(metric_event, nd));
    }
    if create {
        rblist__add_node(metric_events, &me as *const _ as *const c_void);
        nd = rblist__find(metric_events, &me as *const _ as *const c_void);
        if !nd.is_null() {
            return container_of::<metric_event>(nd as *mut c_void, offset_of!(metric_event, nd));
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn metric_event_cmp(rb_node: *mut rb_node, entry: *const c_void) -> c_int {
    let a = container_of::<metric_event>(rb_node as *mut c_void, offset_of!(metric_event, nd));
    let b = entry as *const metric_event;
    if (*a).evsel == (*b).evsel {
        return 0;
    }
    if ((*a).evsel as *mut c_char) < ((*b).evsel as *mut c_char) {
        return -1;
    }
    1
}

unsafe extern "C" fn metric_event_new(_rblist: *mut rblist, entry: *const c_void) -> *mut rb_node {
    let me = malloc(size_of::<metric_event>()) as *mut metric_event;
    if me.is_null() {
        return ptr::null_mut();
    }
    memcpy(me as *mut c_void, entry, size_of::<metric_event>());
    (*me).evsel = (*(entry as *mut metric_event)).evsel;
    (*me).is_default = false;
    INIT_LIST_HEAD(&mut (*me).head);
    &mut (*me).nd
}

unsafe extern "C" fn metric_event_delete(_rblist: *mut rblist, rb_node: *mut rb_node) {
    let me = container_of::<metric_event>(rb_node as *mut c_void, offset_of!(metric_event, nd));
    let mut expr = metric_expr_first(&(*me).head);
    while !expr.is_null() {
        let tmp = metric_expr_next(expr, &(*me).head);
        zfree(&mut (*expr).metric_name);
        zfree(&mut (*expr).metric_refs);
        zfree(&mut (*expr).metric_events);
        free(expr as *mut c_void);
        expr = tmp;
    }
    free(me as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__rblist_init(metric_events: *mut rblist) {
    rblist__init(metric_events);
    (*metric_events).node_cmp = Some(metric_event_cmp);
    (*metric_events).node_new = Some(metric_event_new);
    (*metric_events).node_delete = Some(metric_event_delete);
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__rblist_exit(metric_events: *mut rblist) {
    rblist__exit(metric_events);
}

unsafe fn metric__watchdog_constraint_hint(name: *const c_char, foot: bool) {
    if !foot {
        pr_warning(cstr!("Not grouping metric %s's events.\n"), name);
        violate_nmi_constraint = true;
        return;
    }
    if !violate_nmi_constraint {
        return;
    }
    pr_warning(cstr!("Try disabling the NMI watchdog to comply NO_NMI_WATCHDOG metric constraint:\n    echo 0 > /proc/sys/kernel/nmi_watchdog\n    perf stat ...\n    echo 1 > /proc/sys/kernel/nmi_watchdog\n"));
}

unsafe fn metric__group_events(pm: *const pmu_metric, metric_no_threshold: bool) -> bool {
    match (*pm).event_grouping {
        MetricNoGroupEvents => false,
        MetricNoGroupEventsNmi => {
            if !sysctl__nmi_watchdog_enabled() {
                true
            } else {
                metric__watchdog_constraint_hint((*pm).metric_name, false);
                false
            }
        }
        MetricNoGroupEventsSmt => !smt_on(),
        MetricNoGroupEventsThresholdAndNmi => {
            if metric_no_threshold || !sysctl__nmi_watchdog_enabled() {
                true
            } else {
                metric__watchdog_constraint_hint((*pm).metric_name, false);
                false
            }
        }
        MetricGroupEvents | _ => true,
    }
}

unsafe fn metric__free(m: *mut metric) {
    if m.is_null() {
        return;
    }
    zfree(&mut (*m).metric_refs);
    expr__ctx_free((*m).pctx);
    zfree(&mut (*m).modifier);
    evlist__put((*m).evlist);
    free(m as *mut c_void);
}

unsafe fn metric__new(
    pm: *const pmu_metric,
    modifier: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    runtime: c_int,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
) -> *mut metric {
    let m = zalloc(size_of::<metric>()) as *mut metric;
    if m.is_null() {
        return ptr::null_mut();
    }
    (*m).pctx = expr__ctx_new();
    if (*m).pctx.is_null() {
        metric__free(m);
        return ptr::null_mut();
    }
    (*m).pmu = if !(*pm).pmu.is_null() { (*pm).pmu } else { cstr!("cpu") };
    (*m).metric_name = (*pm).metric_name;
    (*m).default_metricgroup_name = if !(*pm).default_metricgroup_name.is_null() {
        (*pm).default_metricgroup_name
    } else {
        cstr!("")
    };
    (*m).modifier = ptr::null_mut();
    if !modifier.is_null() {
        (*m).modifier = strdup(modifier);
        if (*m).modifier.is_null() {
            metric__free(m);
            return ptr::null_mut();
        }
    }
    (*m).metric_expr = (*pm).metric_expr;
    (*m).metric_threshold = (*pm).metric_threshold;
    (*m).metric_unit = (*pm).unit;
    (*(*m).pctx).sctx.user_requested_cpu_list = ptr::null_mut();
    if !user_requested_cpu_list.is_null() {
        (*(*m).pctx).sctx.user_requested_cpu_list = strdup(user_requested_cpu_list);
        if (*(*m).pctx).sctx.user_requested_cpu_list.is_null() {
            metric__free(m);
            return ptr::null_mut();
        }
    }
    (*(*m).pctx).sctx.runtime = runtime;
    (*(*m).pctx).sctx.system_wide = system_wide;
    (*m).group_events = !metric_no_group && metric__group_events(pm, metric_no_threshold);
    (*m).default_show_events = (*pm).default_show_events;
    (*m).metric_refs = ptr::null_mut();
    (*m).evlist = ptr::null_mut();
    m
}

unsafe fn contains_metric_id(metric_events: *mut *mut evsel, num_events: c_int, metric_id: *const c_char) -> bool {
    let mut i = 0;
    while i < num_events {
        if strcmp(evsel__metric_id(*metric_events.add(i as usize)), metric_id) == 0 {
            return true;
        }
        i += 1;
    }
    false
}

unsafe fn setup_metric_events(
    pmu: *const c_char,
    ids: *mut hashmap,
    metric_evlist: *mut evlist,
    out_metric_events: *mut *mut *mut evsel,
) -> c_int {
    *out_metric_events = ptr::null_mut();
    let ids_size = hashmap__size(ids);
    let metric_events = calloc(ids_size + 1, size_of::<*mut c_void>()) as *mut *mut evsel;
    if metric_events.is_null() {
        return -ENOMEM;
    }
    #[repr(C)]
    struct Data {
        pmu: *const c_char,
        ids: *mut hashmap,
        metric_events: *mut *mut evsel,
        matched_events: usize,
        ids_size: usize,
        all_pmus: bool,
    }
    unsafe fn cb(ev: *mut evsel, data: *mut c_void) -> c_int {
        let d = data as *mut Data;
        if !(*d).all_pmus
            && !(*ev).pmu.is_null()
            && evsel__is_hybrid(ev)
            && strcmp((*(*ev).pmu).name, (*d).pmu) != 0
        {
            return 0;
        }
        let metric_id = evsel__metric_id(ev);
        if contains_metric_id((*d).metric_events, (*d).matched_events as c_int, metric_id) {
            return 0;
        }
        let mut val_ptr: *mut expr_id_data = ptr::null_mut();
        if hashmap__find((*d).ids, metric_id, &mut val_ptr) {
            pr_debug(cstr!("Matched metric-id %s to %s\n"), metric_id, evsel__name(ev));
            *(*d).metric_events.add((*d).matched_events) = ev;
            (*d).matched_events += 1;
            if (*d).matched_events >= (*d).ids_size {
                return 1;
            }
        }
        0
    }
    let all_pmus = strcmp(pmu, cstr!("all")) == 0
        || strcmp(pmu, cstr!("default_core")) == 0
        || perf_pmus__num_core_pmus() == 1
        || !is_pmu_core(pmu);
    let mut data = Data { pmu, ids, metric_events, matched_events: 0, ids_size, all_pmus };
    let _ = evlist_for_each_entry(metric_evlist, cb, &mut data as *mut _ as *mut c_void);
    if data.matched_events < ids_size {
        free(metric_events as *mut c_void);
        return -EINVAL;
    }
    let mut i = 0usize;
    while i < ids_size {
        let ev = *metric_events.add(i);
        (*ev).collect_stat = true;
        (*ev).metric_leader = ev;
        let metric_id = evsel__metric_id(ev);
        #[repr(C)]
        struct ContData {
            metric_id: *const c_char,
            leader: *mut evsel,
        }
        unsafe fn cont_cb(ev: *mut evsel, data: *mut c_void) -> c_int {
            let d = data as *mut ContData;
            if strcmp(evsel__metric_id(ev), (*d).metric_id) == 0 {
                (*ev).metric_leader = (*d).leader;
            }
            0
        }
        let mut cdata = ContData { metric_id, leader: *metric_events.add(i) };
        let _ = evlist_for_each_entry_continue(metric_evlist, ev, cont_cb, &mut cdata as *mut _ as *mut c_void);
        i += 1;
    }
    *out_metric_events = metric_events;
    0
}

unsafe fn match_metric_or_groups(metric_or_groups: *const c_char, sought: *const c_char) -> bool {
    if sought.is_null() {
        return false;
    }
    if strcmp(sought, cstr!("all")) == 0 {
        return true;
    }
    if metric_or_groups.is_null() {
        return strcasecmp(sought, cstr!("No_group")) == 0;
    }
    let len = strlen(sought);
    if strncasecmp(metric_or_groups, sought, len) == 0
        && (*metric_or_groups.add(len) == 0 || *metric_or_groups.add(len) == b';' as c_char)
    {
        return true;
    }
    let m = strchr(metric_or_groups, b';' as c_int);
    !m.is_null() && match_metric_or_groups(m.add(1), sought)
}

unsafe fn match_pm_metric_or_groups(
    pm: *const pmu_metric,
    pmu: *const c_char,
    metric_or_groups: *const c_char,
) -> bool {
    let pm_pmu = if !(*pm).pmu.is_null() { (*pm).pmu } else { cstr!("cpu") };
    let mut perf_pmu: *mut perf_pmu = ptr::null_mut();
    if !(*pm).pmu.is_null() {
        perf_pmu = perf_pmus__find((*pm).pmu);
    }
    if strcmp(pmu, cstr!("all")) != 0
        && strcmp(pm_pmu, pmu) != 0
        && (!perf_pmu.is_null() && !perf_pmu__name_wildcard_match(perf_pmu, pmu))
    {
        return false;
    }
    match_metric_or_groups((*pm).metric_group, metric_or_groups)
        || match_metric_or_groups((*pm).metric_name, metric_or_groups)
}

#[repr(C)]
struct metricgroup_iter_data {
    fn_: pmu_metric_iter_fn,
    data: *mut c_void,
}

unsafe extern "C" fn metricgroup__sys_event_iter(
    pm: *const pmu_metric,
    table: *const pmu_metrics_table,
    data: *mut c_void,
) -> c_int {
    let d = data as *mut metricgroup_iter_data;
    if (*pm).metric_expr.is_null() || (*pm).compat.is_null() {
        return 0;
    }
    let pmu = perf_pmus__scan_for_uncore_id(ptr::null_mut(), (*pm).compat);
    if !pmu.is_null() {
        ((*d).fn_).unwrap()(pm, table, (*d).data)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__for_each_metric(
    table: *const pmu_metrics_table,
    fn_: pmu_metric_iter_fn,
    data: *mut c_void,
) -> c_int {
    let mut sys_data = metricgroup_iter_data { fn_, data };
    let tables = [table, pmu_metrics_table__default()];
    let mut i = 0usize;
    while i < tables.len() {
        if !tables[i].is_null() {
            let ret = pmu_metrics_table__for_each_metric(tables[i], fn_, data);
            if ret != 0 {
                return ret;
            }
        }
        i += 1;
    }
    pmu_for_each_sys_metric(Some(metricgroup__sys_event_iter), &mut sys_data as *mut _ as *mut c_void)
}

unsafe fn encode_metric_id(sb: *mut strbuf, mut x: *const c_char) -> c_int {
    let mut ret = 0;
    while *x != 0 {
        let c = strchr(code_characters.as_ptr() as *const c_char, *x as c_int);
        if !c.is_null() {
            ret = strbuf_addch(sb, b'!' as c_int);
            if ret != 0 {
                break;
            }
            ret = strbuf_addch(sb, b'0' as c_int + c.offset_from(code_characters.as_ptr() as *const c_char) as c_int);
            if ret != 0 {
                break;
            }
        } else {
            ret = strbuf_addch(sb, *x as c_int);
            if ret != 0 {
                break;
            }
        }
        x = x.add(1);
    }
    ret
}

unsafe fn decode_metric_id(sb: *mut strbuf, mut x: *const c_char) -> c_int {
    let orig = x;
    while *x != 0 {
        let mut c = *x;
        if *x == b'!' as c_char {
            x = x.add(1);
            let i = (*x - b'0' as c_char) as usize;
            if i > strlen(code_characters.as_ptr() as *const c_char) {
                pr_err(cstr!("Bad metric-id encoding in: '%s'"), orig);
                return -1;
            }
            c = *(code_characters.as_ptr() as *const c_char).add(i);
        }
        let ret = strbuf_addch(sb, c as c_int);
        if ret != 0 {
            return ret;
        }
        x = x.add(1);
    }
    0
}

unsafe fn decode_all_metric_ids(perf_evlist: *mut evlist, modifier: *const c_char) -> c_int {
    let mut sb = strbuf { buf: ptr::null_mut(), len: 0, alloc: 0 };
    let mut ret = 0;
    #[repr(C)]
    struct Data {
        sb: *mut strbuf,
        modifier: *const c_char,
        ret: c_int,
    }
    unsafe fn cb(ev: *mut evsel, data: *mut c_void) -> c_int {
        let d = data as *mut Data;
        if (*ev).metric_id.is_null() {
            return 0;
        }
        (*d).ret = strbuf_setlen((*d).sb, 0);
        if (*d).ret != 0 {
            return 1;
        }
        (*d).ret = decode_metric_id((*d).sb, (*ev).metric_id);
        if (*d).ret != 0 {
            return 1;
        }
        free((*ev).metric_id as *mut c_void);
        (*ev).metric_id = strdup((*(*d).sb).buf);
        if (*ev).metric_id.is_null() {
            (*d).ret = -ENOMEM;
            return 1;
        }
        if !strstr((*ev).name, cstr!("metric-id=")).is_null() {
            let mut has_slash = false;
            zfree(&mut (*ev).name);
            let mut cur = strchr((*(*d).sb).buf, b'@' as c_int);
            while !cur.is_null() {
                *cur = b'/' as c_char;
                has_slash = true;
                cur = strchr(cur.add(1), b'@' as c_int);
            }
            if !(*d).modifier.is_null() {
                if !has_slash && strchr((*(*d).sb).buf, b':' as c_int).is_null() {
                    (*d).ret = strbuf_addch((*d).sb, b':' as c_int);
                    if (*d).ret != 0 {
                        return 1;
                    }
                }
                (*d).ret = strbuf_addstr((*d).sb, (*d).modifier);
                if (*d).ret != 0 {
                    return 1;
                }
            }
            (*ev).name = strdup((*(*d).sb).buf);
            if (*ev).name.is_null() {
                (*d).ret = -ENOMEM;
                return 1;
            }
        }
        0
    }
    let mut data = Data { sb: &mut sb, modifier, ret };
    let _ = evlist_for_each_entry(perf_evlist, cb, &mut data as *mut _ as *mut c_void);
    ret = data.ret;
    strbuf_release(&mut sb);
    ret
}

unsafe fn metricgroup__build_event_string(
    events: *mut strbuf,
    ctx: *const expr_parse_ctx,
    modifier: *const c_char,
    group_events: bool,
) -> c_int {
    #[repr(C)]
    struct Data {
        events: *mut strbuf,
        modifier: *const c_char,
        group_events: bool,
        no_group: bool,
        has_tool_events: bool,
        tool_events: [bool; TOOL_PMU__EVENT_MAX],
        ret: c_int,
    }
    unsafe fn cb(cur: *mut hashmap_entry, data: *mut c_void) -> c_int {
        let d = data as *mut Data;
        let mut sep: *mut c_char;
        let rsep: *mut c_char;
        let id = (*cur).pkey;
        let ev = tool_pmu__str_to_event(id);
        pr_debug(cstr!("found event %s\n"), id);
        if ev != TOOL_PMU__EVENT_NONE {
            (*d).has_tool_events = true;
            (*d).tool_events[ev as usize] = true;
            return 0;
        }
        if (*d).no_group {
            if (*d).group_events {
                (*d).ret = strbuf_addch((*d).events, b'{' as c_int);
                if (*d).ret != 0 { return 1; }
            }
            (*d).no_group = false;
        } else {
            (*d).ret = strbuf_addch((*d).events, b',' as c_int);
            if (*d).ret != 0 { return 1; }
        }
        sep = strchr(id, b'@' as c_int);
        if !sep.is_null() {
            (*d).ret = strbuf_add((*d).events, id, sep.offset_from(id) as usize);
            if (*d).ret != 0 { return 1; }
            (*d).ret = strbuf_addch((*d).events, b'/' as c_int);
            if (*d).ret != 0 { return 1; }
            rsep = strrchr(sep, b'@' as c_int);
            (*d).ret = strbuf_add((*d).events, sep.add(1), rsep.offset_from(sep) as usize - 1);
            if (*d).ret != 0 { return 1; }
            (*d).ret = strbuf_addstr((*d).events, cstr!(",metric-id="));
            if (*d).ret != 0 { return 1; }
            sep = rsep;
        } else {
            sep = strchr(id, b':' as c_int);
            if !sep.is_null() {
                (*d).ret = strbuf_add((*d).events, id, sep.offset_from(id) as usize);
            } else {
                (*d).ret = strbuf_addstr((*d).events, id);
            }
            if (*d).ret != 0 { return 1; }
            (*d).ret = strbuf_addstr((*d).events, cstr!("/metric-id="));
            if (*d).ret != 0 { return 1; }
        }
        (*d).ret = encode_metric_id((*d).events, id);
        if (*d).ret != 0 { return 1; }
        (*d).ret = strbuf_addstr((*d).events, cstr!("/"));
        if (*d).ret != 0 { return 1; }
        if !sep.is_null() {
            (*d).ret = strbuf_addstr((*d).events, sep.add(1));
            if (*d).ret != 0 { return 1; }
        }
        if !(*d).modifier.is_null() {
            (*d).ret = strbuf_addstr((*d).events, (*d).modifier);
            if (*d).ret != 0 { return 1; }
        }
        0
    }
    let mut data = Data {
        events,
        modifier,
        group_events,
        no_group: true,
        has_tool_events: false,
        tool_events: [false; TOOL_PMU__EVENT_MAX],
        ret: 0,
    };
    let _ = hashmap_for_each_entry((*ctx).ids, cb, &mut data as *mut _ as *mut c_void);
    if data.ret != 0 {
        return data.ret;
    }
    if !data.no_group && group_events {
        data.ret = strbuf_addf(events, cstr!("}:W"));
        if data.ret != 0 { return data.ret; }
    }
    if data.has_tool_events {
        let mut i = 0;
        while i < TOOL_PMU__EVENT_MAX as c_int {
            if data.tool_events[i as usize] {
                if !data.no_group {
                    data.ret = strbuf_addch(events, b',' as c_int);
                    if data.ret != 0 { return data.ret; }
                }
                data.no_group = false;
                data.ret = strbuf_addstr(events, tool_pmu__event_to_str(i));
                if data.ret != 0 { return data.ret; }
            }
            i += 1;
        }
    }
    data.ret
}

#[no_mangle]
pub unsafe extern "C" fn arch_get_runtimeparam(_pm: *const pmu_metric) -> c_int {
    1
}

#[repr(C)]
struct visited_metric {
    name: *const c_char,
    parent: *const visited_metric,
}

#[repr(C)]
struct metricgroup_add_iter_data {
    metric_list: *mut list_head,
    pmu: *const c_char,
    metric_name: *const c_char,
    modifier: *const c_char,
    ret: *mut c_int,
    has_match: *mut bool,
    metric_no_group: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    root_metric: *mut metric,
    visited: *const visited_metric,
    table: *const pmu_metrics_table,
}

unsafe extern "C" fn metricgroup__find_metric_callback(
    pm: *const pmu_metric,
    _table: *const pmu_metrics_table,
    vdata: *mut c_void,
) -> c_int {
    memcpy(vdata, pm as *const c_void, size_of::<pmu_metric>());
    0
}

#[repr(C)]
struct to_resolve {
    /* The metric to resolve. */
    pm: pmu_metric,
    /*
     * The key in the IDs map, this may differ from in case,
     * etc. from pm->metric_name.
     */
    key: *const c_char,
}

unsafe fn resolve_metric(
    metric_list: *mut list_head,
    pmu: *mut perf_pmu,
    modifier: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    root_metric: *mut metric,
    visited: *const visited_metric,
    table: *const pmu_metrics_table,
) -> c_int {
    let mut pending: *mut to_resolve = ptr::null_mut();
    let mut pending_cnt: c_int = 0;
    #[repr(C)]
    struct Data {
        pending: *mut *mut to_resolve,
        pending_cnt: *mut c_int,
        pmu: *mut perf_pmu,
        table: *const pmu_metrics_table,
        ret: c_int,
    }
    unsafe fn cb(cur: *mut hashmap_entry, data: *mut c_void) -> c_int {
        let d = data as *mut Data;
        let mut pm: pmu_metric = mem::zeroed();
        if pmu_metrics_table__find_metric(
            (*d).table,
            (*d).pmu,
            (*cur).pkey,
            Some(metricgroup__find_metric_callback),
            &mut pm as *mut _ as *mut c_void,
        ) != PMU_METRICS__NOT_FOUND
        {
            let new_pending = realloc(
                *(*d).pending as *mut c_void,
                ((*(*d).pending_cnt + 1) as usize) * size_of::<to_resolve>(),
            ) as *mut to_resolve;
            if new_pending.is_null() {
                (*d).ret = -ENOMEM;
                return 1;
            }
            *(*d).pending = new_pending;
            memcpy(
                &mut (*new_pending.add(*(*d).pending_cnt as usize)).pm as *mut _ as *mut c_void,
                &pm as *const _ as *const c_void,
                size_of::<pmu_metric>(),
            );
            (*new_pending.add(*(*d).pending_cnt as usize)).key = (*cur).pkey;
            *(*d).pending_cnt += 1;
        }
        0
    }
    let mut data = Data {
        pending: &mut pending,
        pending_cnt: &mut pending_cnt,
        pmu,
        table,
        ret: 0,
    };
    let _ = hashmap_for_each_entry((*(*root_metric).pctx).ids, cb, &mut data as *mut _ as *mut c_void);
    if data.ret != 0 {
        return data.ret;
    }
    let mut i = 0;
    while i < pending_cnt {
        expr__del_id((*root_metric).pctx, (*pending.add(i as usize)).key);
        i += 1;
    }
    let mut ret = 0;
    i = 0;
    while i < pending_cnt {
        ret = add_metric(
            metric_list,
            &(*pending.add(i as usize)).pm,
            modifier,
            metric_no_group,
            metric_no_threshold,
            user_requested_cpu_list,
            system_wide,
            root_metric,
            visited,
            table,
        );
        if ret != 0 {
            break;
        }
        i += 1;
    }
    free(pending as *mut c_void);
    ret
}

unsafe fn __add_metric(
    metric_list: *mut list_head,
    pm: *const pmu_metric,
    modifier: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    runtime: c_int,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    mut root_metric: *mut metric,
    visited: *const visited_metric,
    table: *const pmu_metrics_table,
) -> c_int {
    let is_root = root_metric.is_null();
    let mut visited_node = visited_metric { name: (*pm).metric_name, parent: visited };
    let mut vm = visited;
    while !vm.is_null() {
        if strcmp((*pm).metric_name, (*vm).name) == 0 {
            pr_err(cstr!("failed: recursion detected for %s\n"), (*pm).metric_name);
            return -1;
        }
        vm = (*vm).parent;
    }
    if is_root {
        root_metric = metric__new(pm, modifier, metric_no_group, metric_no_threshold, runtime, user_requested_cpu_list, system_wide);
        if root_metric.is_null() {
            return -ENOMEM;
        }
    } else {
        let mut cnt: c_int = 0;
        if !(*root_metric).metric_refs.is_null() {
            while !(*(*root_metric).metric_refs.add(cnt as usize)).metric_name.is_null() {
                if strcmp((*pm).metric_name, (*(*root_metric).metric_refs.add(cnt as usize)).metric_name) == 0 {
                    return 0;
                }
                cnt += 1;
            }
        }
        (*root_metric).metric_refs = realloc((*root_metric).metric_refs as *mut c_void, ((cnt + 2) as usize) * size_of::<metric_ref>()) as *mut metric_ref;
        if (*root_metric).metric_refs.is_null() {
            return -ENOMEM;
        }
        (*(*root_metric).metric_refs.add(cnt as usize)).metric_name = (*pm).metric_name;
        (*(*root_metric).metric_refs.add(cnt as usize)).metric_expr = (*pm).metric_expr;
        (*(*root_metric).metric_refs.add((cnt + 1) as usize)).metric_name = ptr::null();
        (*(*root_metric).metric_refs.add((cnt + 1) as usize)).metric_expr = ptr::null();
    }
    let mut expr = (*pm).metric_expr;
    if is_root && !(*pm).metric_threshold.is_null() {
        debug_assert!(!strstr((*pm).metric_threshold, (*pm).metric_name).is_null());
        expr = if metric_no_threshold { (*pm).metric_name } else { (*pm).metric_threshold };
        visited_node.name = cstr!("__threshold__");
    }
    let mut ret = expr__find_ids(expr, ptr::null_mut(), (*root_metric).pctx);
    if ret == 0 {
        let resolved_pmu = if !(*pm).pmu.is_null() && *(*pm).pmu != 0 {
            perf_pmus__find((*pm).pmu)
        } else {
            perf_pmus__scan_core(ptr::null_mut())
        };
        ret = resolve_metric(metric_list, resolved_pmu, modifier, metric_no_group, metric_no_threshold, user_requested_cpu_list, system_wide, root_metric, &visited_node, table);
    }
    if ret != 0 {
        if is_root {
            metric__free(root_metric);
        }
    } else if is_root {
        list_add(&mut (*root_metric).nd, metric_list);
    }
    ret
}

unsafe fn add_metric(
    metric_list: *mut list_head,
    pm: *const pmu_metric,
    modifier: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    root_metric: *mut metric,
    visited: *const visited_metric,
    table: *const pmu_metrics_table,
) -> c_int {
    pr_debug(cstr!("metric expr %s for %s\n"), (*pm).metric_expr, (*pm).metric_name);
    if strstr((*pm).metric_expr, cstr!("?")).is_null() {
        __add_metric(metric_list, pm, modifier, metric_no_group, metric_no_threshold, 0, user_requested_cpu_list, system_wide, root_metric, visited, table)
    } else {
        let count = arch_get_runtimeparam(pm);
        let mut ret = 0;
        let mut j = 0;
        while j < count && ret == 0 {
            ret = __add_metric(metric_list, pm, modifier, metric_no_group, metric_no_threshold, j, user_requested_cpu_list, system_wide, root_metric, visited, table);
            j += 1;
        }
        ret
    }
}

unsafe extern "C" fn metric_list_cmp(_priv: *mut c_void, l: *const list_head, r: *const list_head) -> c_int {
    let left = container_of::<metric>(l as *mut c_void, offset_of!(metric, nd));
    let right = container_of::<metric>(r as *mut c_void, offset_of!(metric, nd));
    let mut left_count = hashmap__size((*(*left).pctx).ids) as c_int;
    let mut right_count = hashmap__size((*(*right).pctx).ids) as c_int;
    let mut i = 0;
    while i < TOOL_PMU__EVENT_MAX as c_int {
        let mut data: *mut expr_id_data = ptr::null_mut();
        if expr__get_id((*left).pctx, tool_pmu__event_to_str(i), &mut data) == 0 {
            left_count -= 1;
        }
        if expr__get_id((*right).pctx, tool_pmu__event_to_str(i), &mut data) == 0 {
            right_count -= 1;
        }
        i += 1;
    }
    right_count - left_count
}

unsafe extern "C" fn default_metricgroup_cmp(_priv: *mut c_void, l: *const list_head, r: *const list_head) -> c_int {
    let left = container_of::<metric>(l as *mut c_void, offset_of!(metric, nd));
    let right = container_of::<metric>(r as *mut c_void, offset_of!(metric, nd));
    let diff = strcmp((*right).default_metricgroup_name, (*left).default_metricgroup_name);
    if diff != 0 {
        return diff;
    }
    strcmp((*right).metric_name, (*left).metric_name)
}

#[repr(C)]
struct metricgroup__add_metric_data {
    list: *mut list_head,
    pmu: *const c_char,
    metric_name: *const c_char,
    modifier: *const c_char,
    user_requested_cpu_list: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    system_wide: bool,
    has_match: bool,
}

unsafe extern "C" fn metricgroup__add_metric_callback(
    pm: *const pmu_metric,
    table: *const pmu_metrics_table,
    vdata: *mut c_void,
) -> c_int {
    let data = vdata as *mut metricgroup__add_metric_data;
    let mut ret = 0;
    if !(*pm).metric_expr.is_null() && match_pm_metric_or_groups(pm, (*data).pmu, (*data).metric_name) {
        let metric_no_group = (*data).metric_no_group || match_metric_or_groups((*pm).metricgroup_no_group, (*data).metric_name);
        (*data).has_match = true;
        ret = add_metric((*data).list, pm, (*data).modifier, metric_no_group, (*data).metric_no_threshold, (*data).user_requested_cpu_list, (*data).system_wide, ptr::null_mut(), ptr::null(), table);
    }
    ret
}

unsafe fn metricgroup__add_metric(
    pmu: *const c_char,
    metric_name: *const c_char,
    modifier: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    metric_list: *mut list_head,
    table: *const pmu_metrics_table,
) -> c_int {
    let mut list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    INIT_LIST_HEAD(&mut list);
    let mut data = metricgroup__add_metric_data {
        list: &mut list,
        pmu,
        metric_name,
        modifier,
        metric_no_group,
        metric_no_threshold,
        user_requested_cpu_list,
        system_wide,
        has_match: false,
    };
    let mut ret = metricgroup__for_each_metric(table, Some(metricgroup__add_metric_callback), &mut data as *mut _ as *mut c_void);
    if ret == 0 && !data.has_match {
        ret = -ENOENT;
    }
    list_splice(&mut list, metric_list);
    ret
}

unsafe fn metricgroup__add_metric_list(
    pmu: *const c_char,
    list: *const c_char,
    metric_no_group: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    metric_list: *mut list_head,
    table: *const pmu_metrics_table,
) -> c_int {
    let list_copy = strdup(list);
    if list_copy.is_null() {
        return -ENOMEM;
    }
    let mut list_itr = list_copy;
    let mut ret = 0;
    let mut count = 0;
    loop {
        let metric_name = strsep(&mut list_itr, cstr!(","));
        if metric_name.is_null() {
            break;
        }
        let mut modifier = strchr(metric_name, b':' as c_int);
        if !modifier.is_null() {
            *modifier = 0;
            modifier = modifier.add(1);
        }
        ret = metricgroup__add_metric(pmu, metric_name, modifier, metric_no_group, metric_no_threshold, user_requested_cpu_list, system_wide, metric_list, table);
        if ret == -EINVAL {
            pr_err(cstr!("Fail to parse metric or group `%s'\n"), metric_name);
        } else if ret == -ENOENT {
            pr_err(cstr!("Cannot find metric or group `%s'\n"), metric_name);
        }
        if ret != 0 {
            break;
        }
        count += 1;
    }
    free(list_copy as *mut c_void);
    if ret == 0 {
        metric__watchdog_constraint_hint(ptr::null(), true);
        if count == 0 {
            return -EINVAL;
        }
    }
    ret
}

unsafe fn metricgroup__free_metrics(metric_list: *mut list_head) {
    let mut m = metric_list_first(metric_list);
    while !m.is_null() {
        let tmp = metric_next(m, metric_list);
        list_del_init(&mut (*m).nd);
        metric__free(m);
        m = tmp;
    }
}

unsafe fn find_tool_events(metric_list: *const list_head, tool_events: *mut bool) {
    let mut m = metric_list_first(metric_list);
    while !m.is_null() {
        let mut i = 0;
        while i < TOOL_PMU__EVENT_MAX as c_int {
            let mut data: *mut expr_id_data = ptr::null_mut();
            if !*tool_events.add(i as usize)
                && expr__get_id((*m).pctx, tool_pmu__event_to_str(i), &mut data) == 0
            {
                *tool_events.add(i as usize) = true;
            }
            i += 1;
        }
        m = metric_next(m, metric_list);
    }
}

unsafe fn build_combined_expr_ctx(metric_list: *const list_head, combined: *mut *mut expr_parse_ctx) -> c_int {
    *combined = expr__ctx_new();
    if (*combined).is_null() {
        return -ENOMEM;
    }
    let mut ret = 0;
    let mut m = metric_list_first(metric_list);
    while !m.is_null() {
        if !(*m).group_events && (*m).modifier.is_null() {
            #[repr(C)]
            struct Data {
                combined: *mut expr_parse_ctx,
                ret: c_int,
            }
            unsafe fn cb(cur: *mut hashmap_entry, data: *mut c_void) -> c_int {
                let d = data as *mut Data;
                let dup = strdup((*cur).pkey);
                if dup.is_null() {
                    (*d).ret = -ENOMEM;
                    return 1;
                }
                (*d).ret = expr__add_id((*d).combined, dup);
                if (*d).ret != 0 { 1 } else { 0 }
            }
            let mut data = Data { combined: *combined, ret: 0 };
            let _ = hashmap_for_each_entry((*(*m).pctx).ids, cb, &mut data as *mut _ as *mut c_void);
            ret = data.ret;
            if ret != 0 {
                expr__ctx_free(*combined);
                *combined = ptr::null_mut();
                return ret;
            }
        }
        m = metric_next(m, metric_list);
    }
    ret
}

unsafe fn parse_ids(
    metric_no_merge: bool,
    fake_pmu: bool,
    ids: *mut expr_parse_ctx,
    modifier: *const c_char,
    group_events: bool,
    tool_events: *const bool,
    out_evlist: *mut *mut evlist,
    filter_pmu: *const c_char,
    cputype_filter: bool,
) -> c_int {
    let mut parse_error: parse_events_error = mem::zeroed();
    let mut parsed_evlist: *mut evlist;
    let mut events = strbuf { buf: ptr::null_mut(), len: 0, alloc: 0 };
    let mut ret;
    *out_evlist = ptr::null_mut();
    if !metric_no_merge || hashmap__size((*ids).ids) == 0 {
        let mut added_event = false;
        let mut i = 0;
        while i < TOOL_PMU__EVENT_MAX as c_int {
            if *tool_events.add(i as usize) {
                let tmp = strdup(tool_pmu__event_to_str(i));
                if tmp.is_null() {
                    return -ENOMEM;
                }
                ids__insert((*ids).ids, tmp);
                added_event = true;
            }
            i += 1;
        }
        if !added_event && hashmap__size((*ids).ids) == 0 {
            let tmp = strdup(cstr!("duration_time"));
            if tmp.is_null() {
                return -ENOMEM;
            }
            ids__insert((*ids).ids, tmp);
        }
    }
    ret = metricgroup__build_event_string(&mut events, ids, modifier, group_events);
    if ret != 0 {
        return ret;
    }
    parsed_evlist = evlist__new();
    if parsed_evlist.is_null() {
        ret = -ENOMEM;
        parse_events_error__exit(&mut parse_error);
        strbuf_release(&mut events);
        return ret;
    }
    pr_debug(cstr!("Parsing metric events '%s'\n"), events.buf);
    parse_events_error__init(&mut parse_error);
    ret = __parse_events(parsed_evlist, events.buf, filter_pmu, cputype_filter, &mut parse_error, fake_pmu, false, false);
    if ret != 0 {
        parse_events_error__print(&mut parse_error, events.buf);
    } else {
        ret = decode_all_metric_ids(parsed_evlist, modifier);
        if ret == 0 {
            *out_evlist = parsed_evlist;
            parsed_evlist = ptr::null_mut();
        }
    }
    parse_events_error__exit(&mut parse_error);
    evlist__put(parsed_evlist);
    strbuf_release(&mut events);
    ret
}

unsafe fn count_uses(metric_list: *mut list_head, evsel: *mut evsel) -> c_int {
    let metric_id = evsel__metric_id(evsel);
    let mut uses = 0;
    let mut m = metric_list_first(metric_list);
    while !m.is_null() {
        if hashmap__find((*(*m).pctx).ids, metric_id, ptr::null_mut()) {
            uses += 1;
        }
        m = metric_next(m, metric_list);
    }
    uses
}

unsafe fn pick_display_evsel(metric_list: *mut list_head, metric_events: *mut *mut evsel) -> *mut evsel {
    let mut selected = *metric_events;
    if selected.is_null() {
        return ptr::null_mut();
    }
    let mut selected_uses = count_uses(metric_list, selected) as usize;
    let mut selected_is_tool = evsel__is_tool(selected);
    let mut i = 1usize;
    while !(*metric_events.add(i)).is_null() {
        let candidate = *metric_events.add(i);
        let candidate_uses = count_uses(metric_list, candidate) as usize;
        if (selected_is_tool && !evsel__is_tool(candidate)) || candidate_uses < selected_uses {
            selected = candidate;
            selected_uses = candidate_uses;
            selected_is_tool = evsel__is_tool(selected);
        }
        i += 1;
    }
    selected
}

unsafe fn parse_groups(
    perf_evlist: *mut evlist,
    pmu: *const c_char,
    cputype_filter: bool,
    str_: *const c_char,
    metric_no_group: bool,
    metric_no_merge: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    fake_pmu: bool,
    table: *const pmu_metrics_table,
) -> c_int {
    let mut combined_evlist: *mut evlist = ptr::null_mut();
    let mut metric_list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    INIT_LIST_HEAD(&mut metric_list);
    let mut tool_events = [false; TOOL_PMU__EVENT_MAX];
    let is_default = strcmp(str_, cstr!("Default")) == 0;
    let mut ret = metricgroup__add_metric_list(pmu, str_, metric_no_group, metric_no_threshold, user_requested_cpu_list, system_wide, &mut metric_list, table);
    if ret != 0 {
        metricgroup__free_metrics(&mut metric_list);
        return ret;
    }
    list_sort(ptr::null_mut(), &mut metric_list, Some(metric_list_cmp));
    if !metric_no_merge {
        let mut combined: *mut expr_parse_ctx = ptr::null_mut();
        find_tool_events(&metric_list, tool_events.as_mut_ptr());
        ret = build_combined_expr_ctx(&metric_list, &mut combined);
        if ret == 0 && !combined.is_null() && hashmap__size((*combined).ids) != 0 {
            let filter = if !pmu.is_null() && strcmp(pmu, cstr!("all")) == 0 { ptr::null() } else { pmu };
            ret = parse_ids(metric_no_merge, fake_pmu, combined, ptr::null(), false, tool_events.as_ptr(), &mut combined_evlist, filter, cputype_filter);
        }
        if !combined.is_null() {
            expr__ctx_free(combined);
        }
        if ret != 0 {
            if !combined_evlist.is_null() { evlist__put(combined_evlist); }
            metricgroup__free_metrics(&mut metric_list);
            return ret;
        }
    }
    if is_default {
        list_sort(ptr::null_mut(), &mut metric_list, Some(default_metricgroup_cmp));
    }
    let mut m = metric_list_first(&metric_list);
    while !m.is_null() {
        let mut metric_evlist: *mut evlist = ptr::null_mut();
        if !combined_evlist.is_null() && !(*m).group_events {
            metric_evlist = combined_evlist;
        } else if !metric_no_merge {
            let mut n = metric_list_first(&metric_list);
            while !n.is_null() {
                if m == n { break; }
                if !(*n).evlist.is_null()
                    && !(((*m).modifier.is_null() && !(*n).modifier.is_null())
                        || (!(*m).modifier.is_null() && (*n).modifier.is_null())
                        || (!(*m).modifier.is_null() && !(*n).modifier.is_null() && strcmp((*m).modifier, (*n).modifier) != 0))
                    && !(((*m).pmu.is_null() && !(*n).pmu.is_null())
                        || (!(*m).pmu.is_null() && (*n).pmu.is_null())
                        || (!(*m).pmu.is_null() && !(*n).pmu.is_null() && strcmp((*m).pmu, (*n).pmu) != 0))
                    && expr__subset_of_ids((*n).pctx, (*m).pctx)
                {
                    pr_debug(cstr!("Events in '%s' fully contained within '%s'\n"), (*m).metric_name, (*n).metric_name);
                    metric_evlist = (*n).evlist;
                    break;
                }
                n = metric_next(n, &metric_list);
            }
        }
        if metric_evlist.is_null() {
            let filter = if !pmu.is_null() && strcmp(pmu, cstr!("all")) == 0 { ptr::null() } else { pmu };
            ret = parse_ids(metric_no_merge, fake_pmu, (*m).pctx, (*m).modifier, (*m).group_events, tool_events.as_ptr(), &mut (*m).evlist, filter, cputype_filter);
            if ret != 0 { break; }
            metric_evlist = (*m).evlist;
        }
        let mut metric_events: *mut *mut evsel = ptr::null_mut();
        ret = setup_metric_events(if fake_pmu { cstr!("all") } else { (*m).pmu }, (*(*m).pctx).ids, metric_evlist, &mut metric_events);
        if ret != 0 {
            pr_err(cstr!("Cannot resolve IDs for %s: %s\n"), (*m).metric_name, (*m).metric_expr);
            break;
        }
        let me = metricgroup__lookup(evlist__metric_events(perf_evlist), pick_display_evsel(&mut metric_list, metric_events), true);
        let expr = malloc(size_of::<metric_expr>()) as *mut metric_expr;
        if expr.is_null() {
            ret = -ENOMEM;
            free(metric_events as *mut c_void);
            break;
        }
        (*expr).metric_refs = (*m).metric_refs;
        (*m).metric_refs = ptr::null_mut();
        (*expr).metric_expr = (*m).metric_expr;
        if !(*m).modifier.is_null() {
            let mut tmp: *mut c_char = ptr::null_mut();
            if asprintf(&mut tmp, cstr!("%s:%s"), (*m).metric_name, (*m).modifier) < 0 {
                (*expr).metric_name = ptr::null_mut();
            } else {
                (*expr).metric_name = tmp;
            }
        } else {
            (*expr).metric_name = strdup((*m).metric_name);
        }
        if (*expr).metric_name.is_null() {
            ret = -ENOMEM;
            free(expr as *mut c_void);
            free(metric_events as *mut c_void);
            break;
        }
        if (*m).default_show_events {
            let mut i = 0usize;
            while !(*metric_events.add(i)).is_null() {
                (**metric_events.add(i)).default_show_events = true;
                i += 1;
            }
            #[repr(C)]
            struct D;
            unsafe fn cb(pos: *mut evsel, _data: *mut c_void) -> c_int {
                if !(*pos).metric_leader.is_null() && (*(*pos).metric_leader).default_show_events {
                    (*pos).default_show_events = true;
                }
                0
            }
            let _ = evlist_for_each_entry(metric_evlist, cb, ptr::null_mut());
        }
        (*expr).metric_threshold = (*m).metric_threshold;
        (*expr).metric_unit = (*m).metric_unit;
        (*expr).metric_events = metric_events;
        (*expr).runtime = (*(*m).pctx).sctx.runtime;
        (*expr).default_metricgroup_name = (*m).default_metricgroup_name;
        (*me).is_default = is_default;
        list_add(&mut (*expr).nd, &mut (*me).head);
        m = metric_next(m, &metric_list);
    }
    if ret == 0 {
        if !combined_evlist.is_null() {
            evlist__splice_list_tail(perf_evlist, &mut (*evlist__core(combined_evlist)).entries);
            evlist__put(combined_evlist);
            combined_evlist = ptr::null_mut();
        }
        let mut mm = metric_list_first(&metric_list);
        while !mm.is_null() {
            if !(*mm).evlist.is_null() {
                evlist__splice_list_tail(perf_evlist, &mut (*evlist__core((*mm).evlist)).entries);
            }
            mm = metric_next(mm, &metric_list);
        }
    }
    if !combined_evlist.is_null() {
        evlist__put(combined_evlist);
    }
    metricgroup__free_metrics(&mut metric_list);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__parse_groups(
    perf_evlist: *mut evlist,
    pmu: *const c_char,
    cputype_filter: bool,
    str_: *const c_char,
    metric_no_group: bool,
    metric_no_merge: bool,
    metric_no_threshold: bool,
    user_requested_cpu_list: *const c_char,
    system_wide: bool,
    hardware_aware_grouping: bool,
) -> c_int {
    let table = pmu_metrics_table__find();
    if hardware_aware_grouping {
        pr_debug(cstr!("Use hardware aware grouping instead of traditional metric grouping method\n"));
    }
    parse_groups(perf_evlist, pmu, cputype_filter, str_, metric_no_group, metric_no_merge, metric_no_threshold, user_requested_cpu_list, system_wide, false, table)
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__parse_groups_test(
    evlist: *mut evlist,
    table: *const pmu_metrics_table,
    str_: *const c_char,
    cputype_filter: bool,
) -> c_int {
    parse_groups(evlist, cstr!("all"), cputype_filter, str_, false, false, false, ptr::null(), false, true, table)
}

#[repr(C)]
struct metricgroup__has_metric_data {
    pmu: *const c_char,
    metric_or_groups: *const c_char,
}

unsafe extern "C" fn metricgroup__has_metric_or_groups_callback(
    pm: *const pmu_metric,
    _table: *const pmu_metrics_table,
    vdata: *mut c_void,
) -> c_int {
    let data = vdata as *mut metricgroup__has_metric_data;
    if match_pm_metric_or_groups(pm, (*data).pmu, (*data).metric_or_groups) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__has_metric_or_groups(
    pmu: *const c_char,
    metric_or_groups: *const c_char,
) -> bool {
    let table = pmu_metrics_table__find();
    let mut data = metricgroup__has_metric_data { pmu, metric_or_groups };
    metricgroup__for_each_metric(table, Some(metricgroup__has_metric_or_groups_callback), &mut data as *mut _ as *mut c_void) != 0
}

unsafe extern "C" fn metricgroup__topdown_max_level_callback(
    pm: *const pmu_metric,
    _table: *const pmu_metrics_table,
    data: *mut c_void,
) -> c_int {
    let max_level = data as *mut c_uint;
    let group = if !(*pm).metric_group.is_null() { (*pm).metric_group } else { cstr!("") };
    let p = strstr(group, cstr!("TopdownL"));
    if p.is_null() || *p.add(8) == 0 {
        return 0;
    }
    let level = (*p.add(8) - b'0' as c_char) as c_uint;
    if level > *max_level {
        *max_level = level;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn metricgroups__topdown_max_level() -> c_uint {
    let mut max_level: c_uint = 0;
    let table = pmu_metrics_table__find();
    if table.is_null() {
        return 0;
    }
    pmu_metrics_table__for_each_metric(table, Some(metricgroup__topdown_max_level_callback), &mut max_level as *mut _ as *mut c_void);
    max_level
}

#[no_mangle]
pub unsafe extern "C" fn metricgroup__copy_metric_events(
    evlist: *mut evlist,
    cgrp: *mut cgroup,
    new_metric_events: *mut rblist,
    old_metric_events: *mut rblist,
) -> c_int {
    let mut i: c_uint = 0;
    while i < rblist__nr_entries(old_metric_events) {
        let nd = rblist__entry(old_metric_events, i);
        let old_me = container_of::<metric_event>(nd as *mut c_void, offset_of!(metric_event, nd));
        let mut evsel = evlist__find_evsel(evlist, (*(*old_me).evsel).core.idx);
        if evsel.is_null() {
            return -EINVAL;
        }
        let new_me = metricgroup__lookup(new_metric_events, evsel, true);
        if new_me.is_null() {
            return -ENOMEM;
        }
        pr_debug(cstr!("copying metric event for cgroup '%s': %s (idx=%d)\n"),
            if !cgrp.is_null() { (*cgrp).name } else { cstr!("root") },
            (*evsel).name,
            (*evsel).core.idx);
        (*new_me).is_default = (*old_me).is_default;
        let mut old_expr = metric_expr_first(&(*old_me).head);
        while !old_expr.is_null() {
            let new_expr = malloc(size_of::<metric_expr>()) as *mut metric_expr;
            if new_expr.is_null() {
                return -ENOMEM;
            }
            (*new_expr).metric_expr = (*old_expr).metric_expr;
            (*new_expr).metric_threshold = (*old_expr).metric_threshold;
            (*new_expr).metric_name = strdup((*old_expr).metric_name);
            if (*new_expr).metric_name.is_null() {
                free(new_expr as *mut c_void);
                return -ENOMEM;
            }
            (*new_expr).metric_unit = (*old_expr).metric_unit;
            (*new_expr).runtime = (*old_expr).runtime;
            (*new_expr).default_metricgroup_name = (*old_expr).default_metricgroup_name;
            if !(*old_expr).metric_refs.is_null() {
                let mut nr = 0;
                while !(*(*old_expr).metric_refs.add(nr as usize)).metric_name.is_null() {
                    nr += 1;
                }
                let alloc_size = size_of::<metric_ref>();
                (*new_expr).metric_refs = calloc((nr + 1) as usize, alloc_size) as *mut metric_ref;
                if (*new_expr).metric_refs.is_null() {
                    zfree(&mut (*new_expr).metric_name);
                    free(new_expr as *mut c_void);
                    return -ENOMEM;
                }
                memcpy((*new_expr).metric_refs as *mut c_void, (*old_expr).metric_refs as *const c_void, nr as usize * alloc_size);
            } else {
                (*new_expr).metric_refs = ptr::null_mut();
            }
            let mut nr = 0;
            while !(*(*old_expr).metric_events.add(nr as usize)).is_null() {
                nr += 1;
            }
            let alloc_size = size_of::<*mut evsel>();
            (*new_expr).metric_events = calloc((nr + 1) as usize, alloc_size) as *mut *mut evsel;
            if (*new_expr).metric_events.is_null() {
                zfree(&mut (*new_expr).metric_name);
                zfree(&mut (*new_expr).metric_refs);
                free(new_expr as *mut c_void);
                return -ENOMEM;
            }
            let mut idx = 0;
            while idx < nr {
                evsel = *(*old_expr).metric_events.add(idx as usize);
                evsel = evlist__find_evsel(evlist, (*evsel).core.idx);
                if evsel.is_null() {
                    zfree(&mut (*new_expr).metric_name);
                    zfree(&mut (*new_expr).metric_events);
                    zfree(&mut (*new_expr).metric_refs);
                    free(new_expr as *mut c_void);
                    return -EINVAL;
                }
                *(*new_expr).metric_events.add(idx as usize) = evsel;
                idx += 1;
            }
            list_add(&mut (*new_expr).nd, &mut (*new_me).head);
            old_expr = metric_expr_next(old_expr, &(*old_me).head);
        }
        i += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
