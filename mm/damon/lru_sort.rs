// SPDX-License-Identifier: GPL-2.0
/*
 * DAMON-based LRU-lists Sorting
 */

// C dependencies supplied by the surrounding kernel translation.

static mut ENABLED: bool = false;
static mut COMMIT_INPUTS: bool = false;
static mut ACTIVE_MEM_BP: usize = 0;
static mut AUTOTUNE_MONITORING_INTERVALS: bool = false;
static mut FILTER_YOUNG_PAGES: bool = false;
static mut HOT_THRES_ACCESS_FREQ: usize = 500;
static mut COLD_MIN_AGE: usize = 120000000;
static mut MONITOR_REGION_START: usize = 0;
static mut MONITOR_REGION_END: usize = 0;
static mut ADDR_UNIT: usize = 1;

static mut DAMON_LRU_SORT_QUOTA: damos_quota = damos_quota {
    ms: 10, sz: 0, reset_interval: 1000,
    weight_sz: 0, weight_nr_accesses: 1, weight_age: 1,
};
static mut DAMON_LRU_SORT_WMARKS: damos_watermarks = damos_watermarks {
    metric: DAMOS_WMARK_FREE_MEM_RATE, interval: 5000000,
    high: 200, mid: 150, low: 50,
};
static mut DAMON_LRU_SORT_MON_ATTRS: damon_attrs = damon_attrs {
    sample_interval: 5000, aggr_interval: 100000, ops_update_interval: 0,
    min_nr_regions: 10, max_nr_regions: 1000,
    ..unsafe { core::mem::zeroed() }
};

static mut DAMON_LRU_SORT_HOT_STAT: damos_stat = unsafe { core::mem::zeroed() };
static mut DAMON_LRU_SORT_COLD_STAT: damos_stat = unsafe { core::mem::zeroed() };
static mut CTX: *mut damon_ctx = core::ptr::null_mut();
static mut TARGET: *mut damon_target = core::ptr::null_mut();
static mut DAMON_LRU_SORT_DAMON_HAS_STARTED: bool = false;

static mut DAMON_LRU_SORT_STUB_PATTERN: damos_access_pattern = damos_access_pattern {
    min_sz_region: PAGE_SIZE, max_sz_region: usize::MAX,
    min_nr_accesses: 0, max_nr_accesses: u32::MAX,
    min_age_region: 0, max_age_region: u32::MAX,
};

unsafe fn damon_lru_sort_new_scheme(
    pattern: *mut damos_access_pattern, action: damos_action,
) -> *mut damos {
    let mut quota = DAMON_LRU_SORT_QUOTA;
    quota.ms /= 2;
    damon_new_scheme(pattern, action, 0, &mut quota,
        &mut DAMON_LRU_SORT_WMARKS, NUMA_NO_NODE)
}

unsafe fn damon_lru_sort_new_hot_scheme(hot_thres: u32) -> *mut damos {
    let mut pattern = DAMON_LRU_SORT_STUB_PATTERN;
    pattern.min_nr_accesses = hot_thres;
    damon_lru_sort_new_scheme(&mut pattern, DAMOS_LRU_PRIO)
}

unsafe fn damon_lru_sort_new_cold_scheme(cold_thres: u32) -> *mut damos {
    let mut pattern = DAMON_LRU_SORT_STUB_PATTERN;
    pattern.max_nr_accesses = 0;
    pattern.min_age_region = cold_thres;
    damon_lru_sort_new_scheme(&mut pattern, DAMOS_LRU_DEPRIO)
}

unsafe fn damon_lru_sort_add_quota_goals(hot_scheme: *mut damos, cold_scheme: *mut damos) -> i32 {
    if ACTIVE_MEM_BP == 0 { return 0; }
    if 10000 < ACTIVE_MEM_BP { return -EINVAL; }
    let mut goal = damos_new_quota_goal(DAMOS_QUOTA_ACTIVE_MEM_BP, ACTIVE_MEM_BP);
    if goal.is_null() { return -ENOMEM; }
    damos_add_quota_goal(&mut (*hot_scheme).quota, goal);
    goal = damos_new_quota_goal(DAMOS_QUOTA_INACTIVE_MEM_BP, 10000 - ACTIVE_MEM_BP + 2);
    if goal.is_null() { return -ENOMEM; }
    damos_add_quota_goal(&mut (*cold_scheme).quota, goal);
    0
}

unsafe fn damon_lru_sort_add_filters(hot_scheme: *mut damos, cold_scheme: *mut damos) -> i32 {
    if !FILTER_YOUNG_PAGES { return 0; }
    let mut filter = damos_new_filter(DAMOS_FILTER_TYPE_YOUNG, false, false);
    if filter.is_null() { return -ENOMEM; }
    damos_add_filter(hot_scheme, filter);
    filter = damos_new_filter(DAMOS_FILTER_TYPE_YOUNG, true, false);
    if filter.is_null() { return -ENOMEM; }
    damos_add_filter(cold_scheme, filter);
    0
}

unsafe fn damon_lru_sort_apply_parameters() -> i32 {
    let (mut param_ctx, mut param_target): (*mut damon_ctx, *mut damon_target) = (core::ptr::null_mut(), core::ptr::null_mut());
    let mut err = damon_modules_new_paddr_ctx_target(&mut param_ctx, &mut param_target);
    if err != 0 { return err; }
    (*param_ctx).addr_unit = ADDR_UNIT;
    (*param_ctx).min_region_sz = core::cmp::max(DAMON_MIN_REGION_SZ / ADDR_UNIT, 1);
    if DAMON_LRU_SORT_MON_ATTRS.sample_interval == 0 { err = -EINVAL; damon_destroy_ctx(param_ctx); return err; }
    let mut attrs = DAMON_LRU_SORT_MON_ATTRS;
    if AUTOTUNE_MONITORING_INTERVALS {
        attrs.sample_interval = 5000; attrs.aggr_interval = 100000;
        attrs.intervals_goal.access_bp = 40; attrs.intervals_goal.aggrs = 3;
        attrs.intervals_goal.min_sample_us = 5000; attrs.intervals_goal.max_sample_us = 10 * 1000 * 1000;
    }
    err = damon_set_attrs(param_ctx, &mut attrs); if err != 0 { damon_destroy_ctx(param_ctx); return err; }
    let hot_thres = damon_nr_samples_per_aggr(&attrs) * HOT_THRES_ACCESS_FREQ / 1000;
    let hot_scheme = damon_lru_sort_new_hot_scheme(hot_thres as u32);
    if hot_scheme.is_null() { damon_destroy_ctx(param_ctx); return -ENOMEM; }
    let cold_scheme = damon_lru_sort_new_cold_scheme((COLD_MIN_AGE / attrs.aggr_interval) as u32);
    if cold_scheme.is_null() { damon_destroy_scheme(hot_scheme); damon_destroy_ctx(param_ctx); return -ENOMEM; }
    damon_set_schemes(param_ctx, &hot_scheme, 1); damon_add_scheme(param_ctx, cold_scheme);
    err = damon_lru_sort_add_quota_goals(hot_scheme, cold_scheme);
    if err == 0 { err = damon_lru_sort_add_filters(hot_scheme, cold_scheme); }
    if err == 0 { err = damon_set_region_system_rams_default(param_target, &mut MONITOR_REGION_START, &mut MONITOR_REGION_END, (*param_ctx).addr_unit, (*param_ctx).min_region_sz); }
    if err == 0 { err = damon_commit_ctx(CTX, param_ctx); }
    damon_destroy_ctx(param_ctx); err
}

unsafe fn damon_lru_sort_commit_inputs_fn(_arg: *mut core::ffi::c_void) -> i32 { damon_lru_sort_apply_parameters() }

unsafe fn damon_lru_sort_turn(on: bool) -> i32 {
    if !on { damon_stop(&mut CTX, 1); return 0; }
    let mut err = damon_lru_sort_apply_parameters(); if err != 0 { return err; }
    err = damon_start(&mut CTX, 1, true); if err != 0 { return err; }
    DAMON_LRU_SORT_DAMON_HAS_STARTED = true;
    damon_call(CTX, &mut CALL_CONTROL)
}

static mut CALL_CONTROL: damon_call_control = damon_call_control { fn_: None, repeat: true, data: core::ptr::null_mut() };

// The remaining kernel parameter callbacks and module initialization retain the C interfaces.
unsafe fn damon_lru_sort_enabled() -> bool { !CTX.is_null() && damon_is_running(CTX) }

unsafe fn damon_lru_sort_commit_inputs_store(
    val: *const core::ffi::c_char, _kp: *const kernel_param,
) -> i32 {
    let requested = if val.is_null() { true } else {
        let mut value = false;
        let err = kstrtobool(val, &mut value);
        if err != 0 { return err; }
        value
    };
    if !requested { return 0; }
    if !DAMON_LRU_SORT_DAMON_HAS_STARTED { return -EINVAL; }
    let mut control = damon_call_control { fn_: Some(damon_lru_sort_commit_inputs_fn), repeat: false, data: core::ptr::null_mut() };
    let err = damon_call(CTX, &mut control);
    if err != 0 { err } else { control.return_code }
}

unsafe fn damon_lru_sort_damon_call_fn(arg: *mut core::ffi::c_void) -> i32 {
    let c = arg as *mut damon_ctx;
    let mut s: *mut damos = core::ptr::null_mut();
    while let Some(next) = damon_next_scheme(c, s) {
        s = next;
        if (*s).action == DAMOS_LRU_PRIO { DAMON_LRU_SORT_HOT_STAT = (*s).stat; }
        else if (*s).action == DAMOS_LRU_DEPRIO { DAMON_LRU_SORT_COLD_STAT = (*s).stat; }
    }
    0
}

unsafe fn damon_lru_sort_addr_unit_store(val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let mut input = 0usize;
    let err = kstrtoul(val, 0, &mut input);
    if err != 0 { return err; }
    if input == 0 { return -EINVAL; }
    ADDR_UNIT = input;
    0
}

unsafe fn damon_lru_sort_enabled_store(val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let mut value = false;
    let err = kstrtobool(val, &mut value);
    if err != 0 { return err; }
    ENABLED = value;
    if damon_lru_sort_enabled() == ENABLED { return 0; }
    if !damon_initialized() { return 0; }
    if CTX.is_null() { return -ENOMEM; }
    damon_lru_sort_turn(ENABLED)
}

unsafe fn damon_lru_sort_enabled_load(buffer: *mut core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    sprintf(buffer, if damon_lru_sort_enabled() { "%c\n" } else { "%c\n" }, if damon_lru_sort_enabled() { b'Y' } else { b'N' })
}

unsafe fn damon_lru_sort_kdamond_pid_store(_val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 { 0 }

unsafe fn damon_lru_sort_kdamond_pid_load(buffer: *mut core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let mut pid = -1;
    if !CTX.is_null() { pid = damon_kdamond_pid(CTX); if pid < 0 { pid = -1; } }
    sprintf(buffer, "%d\n", pid)
}

unsafe fn damon_lru_sort_init() -> i32 {
    if !damon_initialized() { return -ENOMEM; }
    let mut err = damon_modules_new_paddr_ctx_target(&mut CTX, &mut TARGET);
    if err != 0 { return err; }
    CALL_CONTROL.data = CTX;
    if ENABLED { err = damon_lru_sort_turn(true); }
    if err != 0 && ENABLED { ENABLED = false; }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
