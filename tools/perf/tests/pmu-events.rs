// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/pmu-events.c. C include dependencies are expected
// to be supplied by the surrounding translated repository.

use core::ffi::{c_char, c_double, c_int, c_ulong, c_void};

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const ENOMEM: c_int = 12;
const PERF_PMU_TYPE_FAKE: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct pmu_event {
    pub name: *const c_char,
    pub compat: *const c_char,
    pub event: *const c_char,
    pub desc: *const c_char,
    pub topic: *const c_char,
    pub long_desc: *const c_char,
    pub pmu: *const c_char,
    pub unit: *const c_char,
    pub perpkg: bool,
    pub deprecated: bool,
}

#[repr(C)]
pub struct pmu_event_info {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub topic: *const c_char,
    pub str_: *const c_char,
    pub pmu_name: *const c_char,
    pub pmu: *mut perf_pmu,
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
    pub format: list_head,
    pub is_core: bool,
    pub is_uncore: bool,
    pub id: *mut c_char,
    pub events_table: *const pmu_events_table,
    pub cpu_aliases_added: bool,
    pub sysfs_aliases_loaded: bool,
}

#[repr(C)]
pub struct pmu_events_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_metrics_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_metric {
    pub metric_name: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aggr_counts {
    pub val: c_int,
}

#[repr(C)]
pub struct aggr_stat {
    pub counts: aggr_counts,
}

#[repr(C)]
pub struct evsel_stats {
    pub aggr: *mut aggr_stat,
}

#[repr(C)]
pub struct evsel {
    pub stats: *mut evsel_stats,
}

#[repr(C)]
pub struct metric_event {
    pub head: list_head,
}

#[repr(C)]
pub struct metric_expr {
    pub nd: list_head,
    pub metric_name: *const c_char,
}

#[repr(C)]
pub struct metric_ref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct expr_parse_ctx {
    pub sctx: expr_scanner_ctx,
    pub ids: *mut hashmap,
}

#[repr(C)]
pub struct expr_scanner_ctx {
    pub is_test: bool,
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
pub struct test_case {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub priv_: *mut c_void,
    pub skip_reason: *const c_char,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
    pub setup: Option<unsafe extern "C" fn(*mut test_suite) -> c_int>,
}

#[repr(C)]
struct perf_pmu_test_event {
    /* used for matching against events from generated pmu-events.c */
    event: pmu_event,

    /*
     * Note: For when PublicDescription does not exist in the JSON, we
     * will have no long_desc in pmu_event.long_desc, but long_desc may
     * be set in the alias.
     */
    alias_long_desc: *const c_char,

    /* PMU which we should match against */
    matching_pmu: *const c_char,
}

#[repr(C)]
struct perf_pmu_test_pmu {
    pmu_name: *const c_char,
    pmu_is_uncore: bool,
    pmu_id: *const c_char,
    aliases: [*const perf_pmu_test_event; 10],
}

#[repr(C)]
struct test_core_pmu_event_aliases_cb_args {
    test_event: *const perf_pmu_test_event,
    count: *mut c_int,
}

#[repr(C)]
struct metric {
    list: list_head,
    metric_ref: metric_ref,
}

#[repr(C)]
struct test_metric {
    str_: *const c_char,
}

#[repr(C)]
struct populate_cb_data {
    test_cases: *mut test_case,
    curr: usize,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn find_sys_events_table(name: *const c_char) -> *const pmu_events_table;
    fn find_core_events_table(arch: *const c_char, cpu: *const c_char) -> *const pmu_events_table;
    fn pmu_events_table__for_each_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        cb: unsafe extern "C" fn(*const pmu_event, *const pmu_events_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_events_table__find_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        name: *const c_char,
        event: *mut *const pmu_event,
        info: *mut *mut pmu_event_info,
    ) -> c_int;
    fn pmu_add_cpu_aliases_table(pmu: *mut perf_pmu, table: *const pmu_events_table);
    fn pmu_add_sys_aliases(pmu: *mut perf_pmu);
    fn perf_pmu__init(pmu: *mut perf_pmu, typ: c_int, name: *const c_char) -> c_int;
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_pmu__find_event(
        pmu: *mut perf_pmu,
        name: *const c_char,
        state: *mut c_void,
        cb: unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int,
    ) -> c_int;
    fn perf_pmu__num_events(pmu: *mut perf_pmu) -> c_int;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn list_empty(head: *const list_head) -> bool;
    fn zalloc(size: usize) -> *mut c_void;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut c_void;
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_evlist__set_maps(core: *mut c_void, cpus: *mut perf_cpu_map, threads: *mut c_void);
    fn __parse_events(
        evlist: *mut evlist,
        str_: *const c_char,
        pmu_filter: *mut c_void,
        cputype_filter: bool,
        error: *mut parse_events_error,
        fake_pmu: bool,
        warn_if_reordered: bool,
        fake_tp: bool,
    ) -> c_int;
    fn parse_events_error__init(error: *mut parse_events_error);
    fn parse_events_error__exit(error: *mut parse_events_error);
    fn metricgroup__parse_groups_test(
        evlist: *mut evlist,
        table: *const pmu_metrics_table,
        metric_name: *const c_char,
        cputype_filter: bool,
    ) -> c_int;
    fn evlist__alloc_stats(config: *mut c_void, evlist: *mut evlist, alloc_raw: bool) -> c_int;
    fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr: c_int);
    fn evlist__free_stats(evlist: *mut evlist);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    fn evlist__metric_events(evlist: *mut evlist) -> *mut c_void;
    fn metricgroup__lookup(events: *mut c_void, evsel: *mut evsel, create: bool) -> *mut metric_event;
    fn metric_event__first(head: *mut list_head) -> *mut metric_expr;
    fn metric_event__next(head: *mut list_head, mexp: *mut metric_expr) -> *mut metric_expr;
    fn test_generic_metric(mexp: *mut metric_expr, cpu: c_int) -> c_double;

    fn expr__ctx_new() -> *mut expr_parse_ctx;
    fn expr__ctx_free(ctx: *mut expr_parse_ctx);
    fn expr__find_ids(str_: *const c_char, map: *mut c_void, ctx: *mut expr_parse_ctx) -> c_int;
    fn expr__add_id_val(ctx: *mut expr_parse_ctx, id: *mut c_char, val: c_int);
    fn expr__parse(result: *mut c_double, ctx: *mut expr_parse_ctx, str_: *const c_char) -> c_int;
    fn hashmap__first(map: *mut hashmap, bkt: *mut usize) -> *mut hashmap_entry;
    fn hashmap__next(map: *mut hashmap, cur: *mut hashmap_entry, bkt: *mut usize) -> *mut hashmap_entry;

    fn tool_pmu__cpu_slots_per_cycle() -> c_int;
    fn pmu_metrics_table__for_each_metric(
        table: *const pmu_metrics_table,
        cb: unsafe extern "C" fn(*const pmu_metric, *const pmu_metrics_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_for_each_core_metric(
        cb: unsafe extern "C" fn(*const pmu_metric, *const pmu_metrics_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_for_each_sys_metric(
        cb: unsafe extern "C" fn(*const pmu_metric, *const pmu_metrics_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_metrics_table__name(table: *const pmu_metrics_table) -> *const c_char;
    fn pmu_metrics_table__iterate_tables(
        cb: unsafe extern "C" fn(*const pmu_metrics_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
}

static BP_L1_BTB_CORRECT: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("bp_l1_btb_correct"), event: cstr!("event=0x8a"), desc: cstr!("L1 BTB Correction"), topic: cstr!("branch"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(),
    matching_pmu: core::ptr::null(),
};

static BP_L2_BTB_CORRECT: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("bp_l2_btb_correct"), event: cstr!("event=0x8b"), desc: cstr!("L2 BTB Correction"), topic: cstr!("branch"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(),
    matching_pmu: core::ptr::null(),
};

static SEGMENT_REG_LOADS_ANY: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("segment_reg_loads.any"), event: cstr!("event=6,period=200000,umask=0x80"), desc: cstr!("Number of segment register loads"), topic: cstr!("other"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(),
    matching_pmu: core::ptr::null(),
};

static DISPATCH_BLOCKED_ANY: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("dispatch_blocked.any"), event: cstr!("event=9,period=200000,umask=0x20"), desc: cstr!("Memory cluster signals to block micro-op dispatch for any reason"), topic: cstr!("other"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(),
    matching_pmu: core::ptr::null(),
};

static EIST_TRANS: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("eist_trans"), event: cstr!("event=0x3a,period=200000"), desc: cstr!("Number of Enhanced Intel SpeedStep(R) Technology (EIST) transitions"), topic: cstr!("other"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(),
    matching_pmu: core::ptr::null(),
};

static L3_CACHE_RD: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { pmu: cstr!("default_core"), name: cstr!("l3_cache_rd"), event: cstr!("event=0x40"), desc: cstr!("L3 cache access, read"), long_desc: cstr!("Attributable Level 3 cache access, read"), topic: cstr!("cache"), compat: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: cstr!("Attributable Level 3 cache access, read"),
    matching_pmu: core::ptr::null(),
};

static CORE_EVENTS: [*const perf_pmu_test_event; 7] = [
    &BP_L1_BTB_CORRECT, &BP_L2_BTB_CORRECT, &SEGMENT_REG_LOADS_ANY,
    &DISPATCH_BLOCKED_ANY, &EIST_TRANS, &L3_CACHE_RD, core::ptr::null(),
];

static UNCORE_HISI_DDRC_FLUX_WCMD: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("uncore_hisi_ddrc.flux_wcmd"), event: cstr!("event=2"), desc: cstr!("DDRC write commands"), topic: cstr!("uncore"), pmu: cstr!("hisi_sccl,ddrc"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("hisi_sccl1_ddrc2"),
};
static UNC_CBO_XSNP_RESPONSE_MISS_EVICTION: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("unc_cbo_xsnp_response.miss_eviction"), event: cstr!("event=0x22,umask=0x81"), desc: cstr!("A cross-core snoop resulted from L3 Eviction which misses in some processor core"), topic: cstr!("uncore"), pmu: cstr!("uncore_cbox"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_cbox_0"),
};
static UNCORE_HYPHEN: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("event-hyphen"), event: cstr!("event=0xe0"), desc: cstr!("UNC_CBO_HYPHEN"), topic: cstr!("uncore"), pmu: cstr!("uncore_cbox"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_cbox_0"),
};
static UNCORE_TWO_HYPH: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("event-two-hyph"), event: cstr!("event=0xc0"), desc: cstr!("UNC_CBO_TWO_HYPH"), topic: cstr!("uncore"), pmu: cstr!("uncore_cbox"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_cbox_0"),
};
static UNCORE_HISI_L3C_RD_HIT_CPIPE: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("uncore_hisi_l3c.rd_hit_cpipe"), event: cstr!("event=7"), desc: cstr!("Total read hits"), topic: cstr!("uncore"), pmu: cstr!("hisi_sccl,l3c"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("hisi_sccl3_l3c7"),
};
static UNCORE_IMC_FREE_RUNNING_CACHE_MISS: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("uncore_imc_free_running.cache_miss"), event: cstr!("event=0x12"), desc: cstr!("Total cache misses"), topic: cstr!("uncore"), pmu: cstr!("uncore_imc_free_running"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_imc_free_running_0"),
};
static UNCORE_IMC_CACHE_HITS: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("uncore_imc.cache_hits"), event: cstr!("event=0x34"), desc: cstr!("Total cache hits"), topic: cstr!("uncore"), pmu: cstr!("uncore_imc"), compat: core::ptr::null(), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_imc_0"),
};

static UNCORE_EVENTS: [*const perf_pmu_test_event; 8] = [
    &UNCORE_HISI_DDRC_FLUX_WCMD, &UNC_CBO_XSNP_RESPONSE_MISS_EVICTION,
    &UNCORE_HYPHEN, &UNCORE_TWO_HYPH, &UNCORE_HISI_L3C_RD_HIT_CPIPE,
    &UNCORE_IMC_FREE_RUNNING_CACHE_MISS, &UNCORE_IMC_CACHE_HITS, core::ptr::null(),
];

static SYS_DDR_PMU_WRITE_CYCLES: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("sys_ddr_pmu.write_cycles"), event: cstr!("event=0x2b"), desc: cstr!("ddr write-cycles event"), topic: cstr!("uncore"), pmu: cstr!("uncore_sys_ddr_pmu"), compat: cstr!("v8"), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_sys_ddr_pmu0"),
};
static SYS_CCN_PMU_READ_CYCLES: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("sys_ccn_pmu.read_cycles"), event: cstr!("config=0x2c"), desc: cstr!("ccn read-cycles event"), topic: cstr!("uncore"), pmu: cstr!("uncore_sys_ccn_pmu"), compat: cstr!("0x01"), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_sys_ccn_pmu4"),
};
static SYS_CMN_PMU_HNF_CACHE_MISS: perf_pmu_test_event = perf_pmu_test_event {
    event: pmu_event { name: cstr!("sys_cmn_pmu.hnf_cache_miss"), event: cstr!("eventid=1,type=5"), desc: cstr!("Counts total cache misses in first lookup result (high priority)"), topic: cstr!("uncore"), pmu: cstr!("uncore_sys_cmn_pmu"), compat: cstr!("(434|436|43c|43a).*"), long_desc: core::ptr::null(), unit: core::ptr::null(), perpkg: false, deprecated: false },
    alias_long_desc: core::ptr::null(), matching_pmu: cstr!("uncore_sys_cmn_pmu0"),
};
static SYS_EVENTS: [*const perf_pmu_test_event; 4] = [
    &SYS_DDR_PMU_WRITE_CYCLES, &SYS_CCN_PMU_READ_CYCLES, &SYS_CMN_PMU_HNF_CACHE_MISS,
    core::ptr::null(),
];

unsafe fn is_same(reference: *const c_char, test: *const c_char) -> bool {
    if reference.is_null() && test.is_null() { return true; }
    if !reference.is_null() && test.is_null() { return false; }
    if reference.is_null() && !test.is_null() { return false; }
    strcmp(reference, test) == 0
}

unsafe fn compare_pmu_events(e1: *const pmu_event, e2: *const pmu_event) -> c_int {
    if !is_same((*e1).name, (*e2).name) {
        pr_debug2(cstr!("testing event e1 %s: mismatched name string, %s vs %s\n"), (*e1).name, (*e1).name, (*e2).name);
        return -1;
    }
    if !is_same((*e1).compat, (*e2).compat) {
        pr_debug2(cstr!("testing event e1 %s: mismatched compat string, %s vs %s\n"), (*e1).name, (*e1).compat, (*e2).compat);
        return -1;
    }
    if !is_same((*e1).event, (*e2).event) {
        pr_debug2(cstr!("testing event e1 %s: mismatched event, %s vs %s\n"), (*e1).name, (*e1).event, (*e2).event);
        return -1;
    }
    if !is_same((*e1).desc, (*e2).desc) {
        pr_debug2(cstr!("testing event e1 %s: mismatched desc, %s vs %s\n"), (*e1).name, (*e1).desc, (*e2).desc);
        return -1;
    }
    if !is_same((*e1).topic, (*e2).topic) {
        pr_debug2(cstr!("testing event e1 %s: mismatched topic, %s vs %s\n"), (*e1).name, (*e1).topic, (*e2).topic);
        return -1;
    }
    if !is_same((*e1).long_desc, (*e2).long_desc) {
        pr_debug2(cstr!("testing event e1 %s: mismatched long_desc, %s vs %s\n"), (*e1).name, (*e1).long_desc, (*e2).long_desc);
        return -1;
    }
    if !is_same((*e1).pmu, (*e2).pmu) {
        pr_debug2(cstr!("testing event e1 %s: mismatched pmu string, %s vs %s\n"), (*e1).name, (*e1).pmu, (*e2).pmu);
        return -1;
    }
    if !is_same((*e1).unit, (*e2).unit) {
        pr_debug2(cstr!("testing event e1 %s: mismatched unit, %s vs %s\n"), (*e1).name, (*e1).unit, (*e2).unit);
        return -1;
    }
    if (*e1).perpkg != (*e2).perpkg {
        pr_debug2(cstr!("testing event e1 %s: mismatched perpkg, %d vs %d\n"), (*e1).name, (*e1).perpkg as c_int, (*e2).perpkg as c_int);
        return -1;
    }
    if (*e1).deprecated != (*e2).deprecated {
        pr_debug2(cstr!("testing event e1 %s: mismatched deprecated, %d vs %d\n"), (*e1).name, (*e1).deprecated as c_int, (*e2).deprecated as c_int);
        return -1;
    }
    0
}

unsafe fn compare_alias_to_test_event(alias: *mut pmu_event_info, test_event: *const perf_pmu_test_event, pmu_name: *const c_char) -> c_int {
    let event = &(*test_event).event as *const pmu_event;
    if !is_same((*alias).name, (*event).name) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched name, %s vs %s\n"), pmu_name, (*alias).name, (*event).name);
        return -1;
    }
    if !is_same((*alias).desc, (*event).desc) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched desc, %s vs %s\n"), pmu_name, (*alias).desc, (*event).desc);
        return -1;
    }
    if !is_same((*alias).long_desc, (*test_event).alias_long_desc) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched long_desc, %s vs %s\n"), pmu_name, (*alias).long_desc, (*test_event).alias_long_desc);
        return -1;
    }
    if !is_same((*alias).topic, (*event).topic) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched topic, %s vs %s\n"), pmu_name, (*alias).topic, (*event).topic);
        return -1;
    }
    if !is_same((*alias).str_, (*test_event).event.event) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched str, %s vs %s\n"), pmu_name, (*alias).str_, (*test_event).event.event);
        return -1;
    }
    if !is_same((*alias).long_desc, (*test_event).alias_long_desc) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched long desc, %s vs %s\n"), pmu_name, (*alias).str_, (*test_event).alias_long_desc);
        return -1;
    }
    if !is_same((*alias).pmu_name, (*test_event).event.pmu) && !is_same((*alias).pmu_name, cstr!("default_core")) {
        pr_debug(cstr!("testing aliases PMU %s: mismatched pmu_name, %s vs %s\n"), pmu_name, (*alias).pmu_name, (*test_event).event.pmu);
        return -1;
    }
    0
}

unsafe extern "C" fn test__pmu_event_table_core_callback(pe: *const pmu_event, _table: *const pmu_events_table, data: *mut c_void) -> c_int {
    let map_events = data as *mut c_int;
    let test_event_table = if strcmp((*pe).pmu, cstr!("default_core")) != 0 { UNCORE_EVENTS.as_ptr() } else { CORE_EVENTS.as_ptr() };
    let mut found = false;
    let mut p = test_event_table;
    while !(*p).is_null() {
        let test_event = *p;
        let event = &(*test_event).event as *const pmu_event;
        if strcmp((*pe).name, (*event).name) != 0 {
            p = p.add(1);
            continue;
        }
        found = true;
        *map_events += 1;
        if compare_pmu_events(pe, event) != 0 { return -1; }
        pr_debug(cstr!("testing event table %s: pass\n"), (*pe).name);
        p = p.add(1);
    }
    if !found {
        pr_err(cstr!("testing event table: could not find event %s\n"), (*pe).name);
        return -1;
    }
    0
}

unsafe extern "C" fn test__pmu_event_table_sys_callback(pe: *const pmu_event, _table: *const pmu_events_table, data: *mut c_void) -> c_int {
    let map_events = data as *mut c_int;
    let mut p = SYS_EVENTS.as_ptr();
    let mut found = false;
    while !(*p).is_null() {
        let test_event = *p;
        let event = &(*test_event).event as *const pmu_event;
        if strcmp((*pe).name, (*event).name) != 0 {
            p = p.add(1);
            continue;
        }
        found = true;
        *map_events += 1;
        if compare_pmu_events(pe, event) != 0 { return TEST_FAIL; }
        pr_debug(cstr!("testing sys event table %s: pass\n"), (*pe).name);
        p = p.add(1);
    }
    if !found {
        pr_debug(cstr!("testing sys event table: could not find event %s\n"), (*pe).name);
        return TEST_FAIL;
    }
    TEST_OK
}

/* Verify generated events from pmu-events.c are as expected */
unsafe extern "C" fn test__pmu_event_table(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let sys_event_table = find_sys_events_table(cstr!("pmu_events__test_soc_sys"));
    let table = find_core_events_table(cstr!("testarch"), cstr!("testcpu"));
    let mut map_events: c_int = 0;
    let expected_events: c_int = (CORE_EVENTS.len() + UNCORE_EVENTS.len() + SYS_EVENTS.len() - 3) as c_int;
    if table.is_null() || sys_event_table.is_null() { return -1; }
    let mut err = pmu_events_table__for_each_event(table, core::ptr::null_mut(), test__pmu_event_table_core_callback, &mut map_events as *mut _ as *mut c_void);
    if err != 0 { return err; }
    err = pmu_events_table__for_each_event(sys_event_table, core::ptr::null_mut(), test__pmu_event_table_sys_callback, &mut map_events as *mut _ as *mut c_void);
    if err != 0 { return err; }
    if map_events != expected_events {
        pr_err(cstr!("testing event table: found %d, but expected %d\n"), map_events, expected_events);
        return TEST_FAIL;
    }
    0
}

unsafe extern "C" fn test_core_pmu_event_aliases_cb(state: *mut c_void, alias: *mut pmu_event_info) -> c_int {
    let args = state as *mut test_core_pmu_event_aliases_cb_args;
    if compare_alias_to_test_event(alias, (*args).test_event, (*(*alias).pmu).name) != 0 { return -1; }
    *(*args).count += 1;
    pr_debug2(cstr!("testing aliases core PMU %s: matched event %s\n"), (*alias).pmu_name, (*alias).name);
    0
}

/* Verify aliases are as expected */
unsafe fn __test_core_pmu_event_aliases(pmu_name: *const c_char, count: *mut c_int) -> c_int {
    let table = find_core_events_table(cstr!("testarch"), cstr!("testcpu"));
    if table.is_null() { return -1; }
    let pmu = zalloc(core::mem::size_of::<perf_pmu>()) as *mut perf_pmu;
    if pmu.is_null() { return -1; }
    if perf_pmu__init(pmu, PERF_PMU_TYPE_FAKE, pmu_name) != 0 {
        perf_pmu__delete(pmu);
        return -1;
    }
    (*pmu).is_core = true;
    (*pmu).events_table = table;
    pmu_add_cpu_aliases_table(pmu, table);
    (*pmu).cpu_aliases_added = true;
    (*pmu).sysfs_aliases_loaded = true;
    let mut res = pmu_events_table__find_event(table, pmu, cstr!("bp_l1_btb_correct"), core::ptr::null_mut(), core::ptr::null_mut());
    if res != 0 {
        pr_debug(cstr!("Missing test event in test architecture"));
        return res;
    }
    let mut p = CORE_EVENTS.as_ptr();
    while !(*p).is_null() {
        let mut test_event = core::ptr::read(*p);
        test_event.event.pmu = pmu_name;
        let event = &test_event.event as *const pmu_event;
        let mut args = test_core_pmu_event_aliases_cb_args { test_event: &test_event, count };
        let err = perf_pmu__find_event(pmu, (*event).name, &mut args as *mut _ as *mut c_void, test_core_pmu_event_aliases_cb);
        if err != 0 { res = err; }
        p = p.add(1);
    }
    perf_pmu__delete(pmu);
    res
}

unsafe fn __test_uncore_pmu_event_aliases(test_pmu: *mut perf_pmu_test_pmu) -> c_int {
    let mut alias_count: c_int;
    let mut to_match_count: c_int = 0;
    let mut matched_count: c_int = 0;
    let events_table = find_core_events_table(cstr!("testarch"), cstr!("testcpu"));
    if events_table.is_null() { return -1; }
    let pmu = zalloc(core::mem::size_of::<perf_pmu>()) as *mut perf_pmu;
    if pmu.is_null() { return -1; }
    if perf_pmu__init(pmu, PERF_PMU_TYPE_FAKE, (*test_pmu).pmu_name) != 0 {
        perf_pmu__delete(pmu);
        return -1;
    }
    (*pmu).is_uncore = (*test_pmu).pmu_is_uncore;
    if !(*test_pmu).pmu_id.is_null() {
        (*pmu).id = strdup((*test_pmu).pmu_id);
        if (*pmu).id.is_null() {
            perf_pmu__delete(pmu);
            return -1;
        }
    }
    (*pmu).events_table = events_table;
    pmu_add_cpu_aliases_table(pmu, events_table);
    (*pmu).cpu_aliases_added = true;
    (*pmu).sysfs_aliases_loaded = true;
    pmu_add_sys_aliases(pmu);
    /* Count how many aliases we generated */
    alias_count = perf_pmu__num_events(pmu);
    /* Count how many aliases we expect from the known table */
    let mut table = (*test_pmu).aliases.as_ptr();
    while !(*table).is_null() {
        to_match_count += 1;
        table = table.add(1);
    }
    if alias_count != to_match_count {
        pr_debug(cstr!("testing aliases uncore PMU %s: mismatch expected aliases (%d) vs found (%d)\n"), (*pmu).name, to_match_count, alias_count);
        perf_pmu__delete(pmu);
        return -1;
    }
    table = (*test_pmu).aliases.as_ptr();
    let mut res = 0;
    while !(*table).is_null() {
        let mut test_event = core::ptr::read(*table);
        let event = &test_event.event as *const pmu_event;
        let mut args = test_core_pmu_event_aliases_cb_args { test_event: &test_event, count: &mut matched_count };
        if strcmp((*pmu).name, test_event.matching_pmu) != 0 {
            pr_debug(cstr!("testing aliases uncore PMU %s: mismatched matching_pmu, %s vs %s\n"), (*pmu).name, test_event.matching_pmu, (*pmu).name);
            perf_pmu__delete(pmu);
            return -1;
        }
        let err = perf_pmu__find_event(pmu, (*event).name, &mut args as *mut _ as *mut c_void, test_core_pmu_event_aliases_cb);
        if err != 0 {
            res = err;
            pr_debug(cstr!("testing aliases uncore PMU %s: could not match alias %s\n"), (*pmu).name, (*event).name);
            perf_pmu__delete(pmu);
            return -1;
        }
        table = table.add(1);
    }
    if alias_count != matched_count {
        pr_debug(cstr!("testing aliases uncore PMU %s: mismatch found aliases (%d) vs matched (%d)\n"), (*pmu).name, matched_count, alias_count);
        res = -1;
    }
    perf_pmu__delete(pmu);
    res
}

static mut TEST_PMUS: [perf_pmu_test_pmu; 11] = [
    perf_pmu_test_pmu { pmu_name: cstr!("hisi_sccl1_ddrc2"), pmu_is_uncore: true, pmu_id: core::ptr::null(), aliases: [&UNCORE_HISI_DDRC_FLUX_WCMD, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_cbox_0"), pmu_is_uncore: true, pmu_id: core::ptr::null(), aliases: [&UNC_CBO_XSNP_RESPONSE_MISS_EVICTION, &UNCORE_HYPHEN, &UNCORE_TWO_HYPH, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("hisi_sccl3_l3c7"), pmu_is_uncore: true, pmu_id: core::ptr::null(), aliases: [&UNCORE_HISI_L3C_RD_HIT_CPIPE, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_imc_free_running_0"), pmu_is_uncore: true, pmu_id: core::ptr::null(), aliases: [&UNCORE_IMC_FREE_RUNNING_CACHE_MISS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_imc_0"), pmu_is_uncore: true, pmu_id: core::ptr::null(), aliases: [&UNCORE_IMC_CACHE_HITS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_ddr_pmu0"), pmu_is_uncore: true, pmu_id: cstr!("v8"), aliases: [&SYS_DDR_PMU_WRITE_CYCLES, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_ccn_pmu4"), pmu_is_uncore: true, pmu_id: cstr!("0x01"), aliases: [&SYS_CCN_PMU_READ_CYCLES, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_cmn_pmu0"), pmu_is_uncore: true, pmu_id: cstr!("43401"), aliases: [&SYS_CMN_PMU_HNF_CACHE_MISS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_cmn_pmu0"), pmu_is_uncore: true, pmu_id: cstr!("43602"), aliases: [&SYS_CMN_PMU_HNF_CACHE_MISS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_cmn_pmu0"), pmu_is_uncore: true, pmu_id: cstr!("43c03"), aliases: [&SYS_CMN_PMU_HNF_CACHE_MISS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
    perf_pmu_test_pmu { pmu_name: cstr!("uncore_sys_cmn_pmu0"), pmu_is_uncore: true, pmu_id: cstr!("43a01"), aliases: [&SYS_CMN_PMU_HNF_CACHE_MISS, core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null()] },
];

/* Test that aliases generated are as expected */
unsafe extern "C" fn test__aliases(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut pmu: *mut perf_pmu = core::ptr::null_mut();
    loop {
        pmu = perf_pmus__scan_core(pmu);
        if pmu.is_null() { break; }
        let mut count = 0;
        if list_empty(&(*pmu).format) {
            pr_debug2(cstr!("skipping testing core PMU %s\n"), (*pmu).name);
            continue;
        }
        if __test_core_pmu_event_aliases((*pmu).name, &mut count) != 0 {
            pr_debug(cstr!("testing core PMU %s aliases: failed\n"), (*pmu).name);
            return -1;
        }
        if count == 0 {
            pr_debug(cstr!("testing core PMU %s aliases: no events to match\n"), (*pmu).name);
            return -1;
        }
        pr_debug(cstr!("testing core PMU %s aliases: pass\n"), (*pmu).name);
    }
    let mut i: c_ulong = 0;
    while (i as usize) < TEST_PMUS.len() {
        let res = __test_uncore_pmu_event_aliases(&mut TEST_PMUS[i as usize]);
        if res != 0 { return res; }
        i += 1;
    }
    0
}

unsafe fn is_number(str_: *const c_char) -> bool {
    let mut end_ptr: *mut c_char = core::ptr::null_mut();
    errno = 0;
    let v = strtod(str_, &mut end_ptr);
    let _ = v; // We're not interested in this value, only if it is valid
    errno == 0 && end_ptr != str_ as *mut c_char
}

unsafe fn check_parse_id(id: *const c_char, error: *mut parse_events_error) -> c_int {
    /* Numbers are always valid. */
    if is_number(id) { return 0; }
    let evlist = evlist__new();
    if evlist.is_null() { return -ENOMEM; }
    let dup = strdup(id);
    if dup.is_null() { return -ENOMEM; }
    let mut cur = strchr(dup, '@' as c_int);
    while !cur.is_null() {
        *cur = '/' as c_char;
        cur = strchr(cur.add(1), '@' as c_int);
    }
    let ret = __parse_events(evlist, dup, core::ptr::null_mut(), false, error, true, true, false);
    free(dup as *mut c_void);
    evlist__put(evlist);
    ret
}

unsafe fn check_parse_fake(id: *const c_char) -> c_int {
    let mut error = core::mem::MaybeUninit::<parse_events_error>::uninit();
    parse_events_error__init(error.as_mut_ptr());
    let ret = check_parse_id(id, error.as_mut_ptr());
    parse_events_error__exit(error.as_mut_ptr());
    ret
}

unsafe fn is_expected_broken_metric(pm: *const pmu_metric) -> bool {
    if strcmp((*pm).metric_name, cstr!("M1")) == 0 || strcmp((*pm).metric_name, cstr!("M2")) == 0 || strcmp((*pm).metric_name, cstr!("M3")) == 0 {
        return true;
    }
    // C condition preserved: #if defined(__aarch64__)
    #[cfg(target_arch = "aarch64")]
    {
        /*
         * Arm64 platforms may return "#slots == 0", which is treated as a
         * syntax error by the parser. Don't test these metrics when running
         * on such platforms.
         */
        if !strstr((*pm).metric_expr, cstr!("#slots")).is_null() && tool_pmu__cpu_slots_per_cycle() == 0 {
            return true;
        }
    }
    false
}

unsafe extern "C" fn test__parsing_callback(pm: *const pmu_metric, table: *const pmu_metrics_table, data: *mut c_void) -> c_int {
    let failures = data as *mut c_int;
    let mut k: c_int;
    let mut err = 0;
    if (*pm).metric_expr.is_null() { return 0; }
    pr_debug(cstr!("Found metric '%s'\n"), (*pm).metric_name);
    *failures += 1;
    /*
     * We need to prepare evlist for stat mode running on CPU 0
     * because that's where all the stats are going to be created.
     */
    let evlist = evlist__new();
    if evlist.is_null() { return -ENOMEM; }
    let cpus = perf_cpu_map__new(cstr!("0"));
    if cpus.is_null() {
        evlist__put(evlist);
        return -ENOMEM;
    }
    perf_evlist__set_maps(evlist__core(evlist), cpus, core::ptr::null_mut());
    err = metricgroup__parse_groups_test(evlist, table, (*pm).metric_name, false);
    if err != 0 {
        if is_expected_broken_metric(pm) {
            *failures -= 1;
            pr_debug(cstr!("Expected broken metric %s skipping\n"), (*pm).metric_name);
            err = 0;
        }
        goto_out_err(err, evlist, cpus, pm);
        return err;
    }
    err = evlist__alloc_stats(core::ptr::null_mut(), evlist, false);
    if err == 0 {
        /*
         * Add all ids with a made up value. The value may trigger divide by
         * zero when subtracted and so try to make them unique.
         */
        k = 1;
        evlist__alloc_aggr_stats(evlist, 1);
        let mut evsel = evlist__first(evlist);
        while !evsel.is_null() {
            (*(*(*evsel).stats).aggr).counts.val = k;
            k += 1;
            evsel = evlist__next(evlist, evsel);
        }
        evsel = evlist__first(evlist);
        while !evsel.is_null() {
            let me = metricgroup__lookup(evlist__metric_events(evlist), evsel, false);
            if !me.is_null() {
                let mut mexp = metric_event__first(&mut (*me).head);
                while !mexp.is_null() {
                    if strcmp((*mexp).metric_name, (*pm).metric_name) == 0 {
                        pr_debug(cstr!("Result %f\n"), test_generic_metric(mexp, 0));
                        err = 0;
                        *failures -= 1;
                        goto_out_err(err, evlist, cpus, pm);
                        return err;
                    }
                    mexp = metric_event__next(&mut (*me).head, mexp);
                }
            }
            evsel = evlist__next(evlist, evsel);
        }
        pr_debug(cstr!("Didn't find parsed metric %s"), (*pm).metric_name);
        err = 1;
    }
    goto_out_err(err, evlist, cpus, pm);
    err
}

unsafe fn goto_out_err(err: c_int, evlist: *mut evlist, cpus: *mut perf_cpu_map, pm: *const pmu_metric) {
    if err != 0 {
        pr_debug(cstr!("Broken metric %s\n"), (*pm).metric_name);
    }
    /* ... cleanup. */
    evlist__free_stats(evlist);
    perf_cpu_map__put(cpus);
    evlist__put(evlist);
}

unsafe extern "C" fn test__parsing(test: *mut test_suite, subtest: c_int) -> c_int {
    let mut failures = 0;
    let mut table: *const pmu_metrics_table = core::ptr::null();
    if !(*test).test_cases.is_null() {
        table = (*(*test).test_cases.add(subtest as usize)).priv_ as *const pmu_metrics_table;
    }
    if !table.is_null() {
        pmu_metrics_table__for_each_metric(table, test__parsing_callback, &mut failures as *mut _ as *mut c_void);
    } else {
        pmu_for_each_core_metric(test__parsing_callback, &mut failures as *mut _ as *mut c_void);
        pmu_for_each_sys_metric(test__parsing_callback, &mut failures as *mut _ as *mut c_void);
    }
    if failures == 0 { TEST_OK } else { TEST_FAIL }
}

static METRICS: [test_metric; 5] = [
    test_metric { str_: cstr!("(unc_p_power_state_occupancy.cores_c0 / unc_p_clockticks) * 100.") },
    test_metric { str_: cstr!("imx8_ddr0@read\\-cycles@ * 4 * 4") },
    test_metric { str_: cstr!("imx8_ddr0@axid\\-read\\,axi_mask\\=0xffff\\,axi_id\\=0x0000@ * 4") },
    test_metric { str_: cstr!("(cstate_pkg@c2\\-residency@ / msr@tsc@) * 100") },
    test_metric { str_: cstr!("(imx8_ddr0@read\\-cycles@ + imx8_ddr0@write\\-cycles@)") },
];

unsafe fn metric_parse_fake(metric_name: *const c_char, str_: *const c_char) -> c_int {
    let mut result: c_double = 0.0;
    let mut ret = -1;
    pr_debug(cstr!("parsing '%s': '%s'\n"), metric_name, str_);
    let ctx = expr__ctx_new();
    if ctx.is_null() {
        pr_debug(cstr!("expr__ctx_new failed"));
        return TEST_FAIL;
    }
    (*ctx).sctx.is_test = true;
    if expr__find_ids(str_, core::ptr::null_mut(), ctx) < 0 {
        pr_err(cstr!("expr__find_ids failed\n"));
        return -1;
    }
    /*
     * Add all ids with a made up value. The value may
     * trigger divide by zero when subtracted and so try to
     * make them unique.
     */
    let mut i: c_int = 1;
    let mut bkt: usize = 0;
    let mut cur = hashmap__first((*ctx).ids, &mut bkt);
    while !cur.is_null() {
        expr__add_id_val(ctx, strdup((*cur).pkey), i);
        i += 1;
        cur = hashmap__next((*ctx).ids, cur, &mut bkt);
    }
    bkt = 0;
    cur = hashmap__first((*ctx).ids, &mut bkt);
    while !cur.is_null() {
        if check_parse_fake((*cur).pkey) != 0 {
            pr_err(cstr!("check_parse_fake failed\n"));
            expr__ctx_free(ctx);
            return ret;
        }
        cur = hashmap__next((*ctx).ids, cur, &mut bkt);
    }
    ret = 0;
    if expr__parse(&mut result, ctx, str_) != 0 {
        /*
         * Parsing failed, make numbers go from large to small which can
         * resolve divide by zero issues.
         */
        i = 1024;
        bkt = 0;
        cur = hashmap__first((*ctx).ids, &mut bkt);
        while !cur.is_null() {
            expr__add_id_val(ctx, strdup((*cur).pkey), i);
            i -= 1;
            cur = hashmap__next((*ctx).ids, cur, &mut bkt);
        }
        if expr__parse(&mut result, ctx, str_) != 0 {
            pr_err(cstr!("expr__parse failed for %s\n"), metric_name);
            /* The following have hard to avoid divide by zero. */
            if strcmp(metric_name, cstr!("tma_clears_resteers")) == 0 || strcmp(metric_name, cstr!("tma_mispredicts_resteers")) == 0 {
                ret = 0;
            } else {
                ret = -1;
            }
        }
    }
    expr__ctx_free(ctx);
    ret
}

unsafe extern "C" fn test__parsing_fake_callback(pm: *const pmu_metric, _table: *const pmu_metrics_table, _data: *mut c_void) -> c_int {
    metric_parse_fake((*pm).metric_name, (*pm).metric_expr)
}

/*
 * Parse all the metrics for current architecture, or all defined cpus via the
 * 'fake_pmu' in parse_events.
 */
unsafe extern "C" fn test__parsing_fake_static(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut i = 0usize;
    while i < METRICS.len() {
        let err = metric_parse_fake(cstr!(""), METRICS[i].str_);
        if err != 0 { return err; }
        i += 1;
    }
    0
}

unsafe extern "C" fn test__parsing_fake(test: *mut test_suite, subtest: c_int) -> c_int {
    let mut table: *const pmu_metrics_table = core::ptr::null();
    if !(*test).test_cases.is_null() {
        table = (*(*test).test_cases.add(subtest as usize)).priv_ as *const pmu_metrics_table;
    }
    if !table.is_null() {
        return pmu_metrics_table__for_each_metric(table, test__parsing_fake_callback, core::ptr::null_mut());
    }
    let mut i = 0usize;
    while i < METRICS.len() {
        let err = metric_parse_fake(cstr!(""), METRICS[i].str_);
        if err != 0 { return err; }
        i += 1;
    }
    let err = pmu_for_each_core_metric(test__parsing_fake_callback, core::ptr::null_mut());
    if err != 0 { return err; }
    pmu_for_each_sys_metric(test__parsing_fake_callback, core::ptr::null_mut())
}

unsafe extern "C" fn test__parsing_threshold_callback(pm: *const pmu_metric, _table: *const pmu_metrics_table, _data: *mut c_void) -> c_int {
    if (*pm).metric_threshold.is_null() { return 0; }
    metric_parse_fake((*pm).metric_name, (*pm).metric_threshold)
}

unsafe extern "C" fn test__parsing_threshold(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let err = pmu_for_each_core_metric(test__parsing_threshold_callback, core::ptr::null_mut());
    if err != 0 { return err; }
    pmu_for_each_sys_metric(test__parsing_threshold_callback, core::ptr::null_mut())
}

unsafe extern "C" fn count_metrics_tables_cb(_table: *const pmu_metrics_table, data: *mut c_void) -> c_int {
    let count = data as *mut usize;
    *count += 1;
    0
}

unsafe extern "C" fn populate_metrics_tables_cb(table: *const pmu_metrics_table, data: *mut c_void) -> c_int {
    let cb_data = data as *mut populate_cb_data;
    let mut table_name = pmu_metrics_table__name(table);
    let mut desc_real: *mut c_char = core::ptr::null_mut();
    let mut desc_fake: *mut c_char = core::ptr::null_mut();
    if table_name.is_null() {
        table_name = cstr!("unknown");
    }
    if asprintf(&mut desc_real, cstr!("PMU metric parsing: %s"), table_name) < 0 {
        return -ENOMEM;
    }
    if asprintf(&mut desc_fake, cstr!("PMU metric parsing with fake PMU: %s"), table_name) < 0 {
        free(desc_real as *mut c_void);
        return -ENOMEM;
    }
    *(*cb_data).test_cases.add((*cb_data).curr) = test_case {
        name: cstr!("parsing"),
        desc: desc_real,
        run_case: Some(test__parsing),
        priv_: table as *mut c_void,
        skip_reason: cstr!("some metrics failed"),
    };
    (*cb_data).curr += 1;
    *(*cb_data).test_cases.add((*cb_data).curr) = test_case {
        name: cstr!("parsing_fake"),
        desc: desc_fake,
        run_case: Some(test__parsing_fake),
        priv_: table as *mut c_void,
        skip_reason: core::ptr::null(),
    };
    (*cb_data).curr += 1;
    0
}

static mut PMU_EVENTS_TESTS: [test_case; 6] = [
    test_case { name: cstr!("pmu_event_table"), desc: cstr!("PMU event table sanity"), run_case: Some(test__pmu_event_table), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() },
    test_case { name: cstr!("aliases"), desc: cstr!("PMU event map aliases"), run_case: Some(test__aliases), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() },
    test_case { name: cstr!("parsing"), desc: cstr!("Parsing of PMU event table metrics"), run_case: Some(test__parsing), priv_: core::ptr::null_mut(), skip_reason: cstr!("some metrics failed") },
    test_case { name: cstr!("parsing_fake"), desc: cstr!("Parsing of PMU event table metrics with fake PMU"), run_case: Some(test__parsing_fake), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() },
    test_case { name: cstr!("parsing_threshold"), desc: cstr!("Parsing of metric thresholds with fake PMU"), run_case: Some(test__parsing_threshold), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() },
    test_case { name: core::ptr::null(), desc: core::ptr::null(), run_case: None, priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() },
];

unsafe extern "C" fn setup_pmu_events_suite(suite: *mut test_suite) -> c_int {
    let mut num_tables: usize = 0;
    let num_fixed_tests: usize = 4;
    let tests_per_table: usize = 2;
    let mut curr: usize = 0;
    if (*suite).test_cases != PMU_EVENTS_TESTS.as_mut_ptr() {
        return 0;
    }
    let mut ret = pmu_metrics_table__iterate_tables(count_metrics_tables_cb, &mut num_tables as *mut _ as *mut c_void);
    if ret != 0 { return ret; }
    let total_tests = num_fixed_tests + (num_tables * tests_per_table) + 1;
    let test_cases = calloc(total_tests, core::mem::size_of::<test_case>()) as *mut test_case;
    if test_cases.is_null() { return -ENOMEM; }
    *test_cases.add(curr) = test_case { name: cstr!("pmu_event_table"), desc: cstr!("PMU event table sanity"), run_case: Some(test__pmu_event_table), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() };
    curr += 1;
    *test_cases.add(curr) = test_case { name: cstr!("aliases"), desc: cstr!("PMU event map aliases"), run_case: Some(test__aliases), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() };
    curr += 1;
    *test_cases.add(curr) = test_case { name: cstr!("parsing_fake_static"), desc: cstr!("Parsing of static metrics with fake PMU"), run_case: Some(test__parsing_fake_static), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() };
    curr += 1;
    *test_cases.add(curr) = test_case { name: cstr!("parsing_threshold"), desc: cstr!("Parsing of metric thresholds with fake PMU"), run_case: Some(test__parsing_threshold), priv_: core::ptr::null_mut(), skip_reason: core::ptr::null() };
    curr += 1;
    let mut cb_data = populate_cb_data { test_cases, curr };
    ret = pmu_metrics_table__iterate_tables(populate_metrics_tables_cb, &mut cb_data as *mut _ as *mut c_void);
    if ret != 0 {
        let mut i = num_fixed_tests;
        while i < cb_data.curr {
            free((*test_cases.add(i)).desc as *mut c_void);
            i += 1;
        }
        free(test_cases as *mut c_void);
        return ret;
    }
    (*suite).test_cases = test_cases;
    0
}

#[unsafe(no_mangle)]
pub static mut suite__pmu_events: test_suite = test_suite {
    desc: cstr!("PMU JSON event tests"),
    test_cases: unsafe { PMU_EVENTS_TESTS.as_mut_ptr() },
    setup: Some(setup_pmu_events_suite),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
