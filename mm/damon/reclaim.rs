// SPDX-License-Identifier: GPL-2.0
/*
 * DAMON-based page reclamation
 */

// Linux kernel dependencies and module macros are supplied externally.

static mut ENABLED: bool = false;
static mut COMMIT_INPUTS: bool = false;

static mut MIN_AGE: c_ulong = 120000000;
static mut DAMON_RECLAIM_QUOTA: damos_quota = damos_quota {
    ms: 10,
    sz: 128 * 1024 * 1024,
    reset_interval: 1000,
    weight_sz: 0,
    weight_nr_accesses: 0,
    weight_age: 1,
};

static mut QUOTA_MEM_PRESSURE_US: c_ulong = 0;
static mut QUOTA_AUTOTUNE_FEEDBACK: c_ulong = 0;
static mut AUTOTUNE_MONITORING_INTERVALS: bool = false;

static mut DAMON_RECLAIM_WMARKS: damos_watermarks = damos_watermarks {
    metric: DAMOS_WMARK_FREE_MEM_RATE,
    interval: 5000000,
    high: 500,
    mid: 400,
    low: 200,
};

static mut DAMON_RECLAIM_MON_ATTRS: damon_attrs = damon_attrs {
    sample_interval: 5000,
    aggr_interval: 100000,
    ops_update_interval: 0,
    min_nr_regions: 10,
    max_nr_regions: 1000,
};

static mut MONITOR_REGION_START: c_ulong = 0;
static mut MONITOR_REGION_END: c_ulong = 0;
static mut ADDR_UNIT: c_ulong = 1;
static mut SKIP_ANON: bool = false;
static mut DAMON_RECLAIM_STAT: damos_stat = damos_stat { /* supplied layout */ };
static mut CTX: *mut damon_ctx = core::ptr::null_mut();
static mut TARGET: *mut damon_target = core::ptr::null_mut();

unsafe fn damon_reclaim_new_scheme(aggr_interval: c_ulong) -> *mut damos {
    let pattern = damos_access_pattern {
        min_sz_region: PAGE_SIZE,
        max_sz_region: ULONG_MAX,
        min_nr_accesses: 0,
        max_nr_accesses: 0,
        min_age_region: MIN_AGE / aggr_interval,
        max_age_region: UINT_MAX,
    };

    damon_new_scheme(
        &pattern,
        DAMOS_PAGEOUT,
        0,
        &mut DAMON_RECLAIM_QUOTA,
        &mut DAMON_RECLAIM_WMARKS,
        NUMA_NO_NODE,
    )
}

unsafe fn damon_reclaim_apply_parameters() -> c_int {
    let mut param_ctx: *mut damon_ctx = core::ptr::null_mut();
    let mut param_target: *mut damon_target = core::ptr::null_mut();
    let mut attrs: damon_attrs;
    let mut scheme: *mut damos;
    let mut goal: *mut damos_quota_goal;
    let mut filter: *mut damos_filter;
    let mut err: c_int;

    err = damon_modules_new_paddr_ctx_target(&mut param_ctx, &mut param_target);
    if err != 0 { return err; }

    (*param_ctx).addr_unit = ADDR_UNIT;
    (*param_ctx).min_region_sz = max(DAMON_MIN_REGION_SZ / ADDR_UNIT, 1);

    if DAMON_RECLAIM_MON_ATTRS.aggr_interval == 0 {
        err = -EINVAL;
        goto_out!();
    }

    attrs = DAMON_RECLAIM_MON_ATTRS;
    if AUTOTUNE_MONITORING_INTERVALS {
        attrs.sample_interval = 5000;
        attrs.aggr_interval = 100000;
        attrs.intervals_goal.access_bp = 40;
        attrs.intervals_goal.aggrs = 3;
        attrs.intervals_goal.min_sample_us = 5000;
        attrs.intervals_goal.max_sample_us = 10 * 1000 * 1000;
    }
    err = damon_set_attrs(param_ctx, &attrs);
    if err != 0 { goto_out!(); }

    err = -ENOMEM;
    scheme = damon_reclaim_new_scheme(attrs.aggr_interval);
    if scheme.is_null() { goto_out!(); }
    damon_set_schemes(param_ctx, &mut scheme, 1);

    if QUOTA_MEM_PRESSURE_US != 0 {
        goal = damos_new_quota_goal(DAMOS_QUOTA_SOME_MEM_PSI_US, QUOTA_MEM_PRESSURE_US);
        if goal.is_null() { goto_out!(); }
        damos_add_quota_goal(&mut (*scheme).quota, goal);
    }
    if QUOTA_AUTOTUNE_FEEDBACK != 0 {
        goal = damos_new_quota_goal(DAMOS_QUOTA_USER_INPUT, 10000);
        if goal.is_null() { goto_out!(); }
        (*goal).current_value = QUOTA_AUTOTUNE_FEEDBACK;
        damos_add_quota_goal(&mut (*scheme).quota, goal);
    }
    if SKIP_ANON {
        filter = damos_new_filter(DAMOS_FILTER_TYPE_ANON, true, false);
        if filter.is_null() { goto_out!(); }
        damos_add_filter(scheme, filter);
    }
    err = damon_set_region_system_rams_default(
        param_target, &mut MONITOR_REGION_START, &mut MONITOR_REGION_END,
        (*param_ctx).addr_unit, (*param_ctx).min_region_sz);
    if err != 0 { goto_out!(); }
    err = damon_commit_ctx(CTX, param_ctx);

    goto_cleanup!();
}

unsafe fn damon_reclaim_commit_inputs_fn(_arg: *mut c_void) -> c_int {
    damon_reclaim_apply_parameters()
}

static mut DAMON_RECLAIM_DAMON_HAS_STARTED: bool = false;

unsafe fn damon_reclaim_commit_inputs_store(val: *const c_char, _kp: *const kernel_param) -> c_int {
    let mut commit_inputs_request: bool;
    let mut err: c_int;
    let mut control = damon_call_control { fn_: Some(damon_reclaim_commit_inputs_fn), ..Default::default() };
    if val.is_null() { commit_inputs_request = true; }
    else { err = kstrtobool(val, &mut commit_inputs_request); if err != 0 { return err; } }
    if !commit_inputs_request { return 0; }
    if !DAMON_RECLAIM_DAMON_HAS_STARTED { return -EINVAL; }
    err = damon_call(CTX, &mut control);
    if err != 0 { err } else { control.return_code }
}

unsafe fn damon_reclaim_damon_call_fn(arg: *mut c_void) -> c_int {
    let c = arg as *mut damon_ctx;
    let mut s: *mut damos;
    damon_for_each_scheme!(s, c, {
        DAMON_RECLAIM_STAT = (*s).stat;
    });
    0
}

static mut CALL_CONTROL: damon_call_control = damon_call_control {
    fn_: Some(damon_reclaim_damon_call_fn), repeat: true, ..Default::default()
};

unsafe fn damon_reclaim_turn(on: bool) -> c_int {
    let mut err: c_int;
    if !on { damon_stop(&mut CTX, 1); return 0; }
    err = damon_reclaim_apply_parameters();
    if err != 0 { return err; }
    err = damon_start(&mut CTX, 1, true);
    if err != 0 { return err; }
    if !DAMON_RECLAIM_DAMON_HAS_STARTED { DAMON_RECLAIM_DAMON_HAS_STARTED = true; }
    damon_call(CTX, &mut CALL_CONTROL)
}

unsafe fn damon_reclaim_addr_unit_store(val: *const c_char, _kp: *const kernel_param) -> c_int {
    let mut input_addr_unit: c_ulong = 0;
    let err = kstrtoul(val, 0, &mut input_addr_unit);
    if err != 0 { return err; }
    if input_addr_unit == 0 { return -EINVAL; }
    ADDR_UNIT = input_addr_unit;
    0
}

unsafe fn damon_reclaim_enabled() -> bool {
    if CTX.is_null() { return false; }
    damon_is_running(CTX)
}

unsafe fn damon_reclaim_enabled_store(val: *const c_char, _kp: *const kernel_param) -> c_int {
    let mut err: c_int;
    err = kstrtobool(val, &mut ENABLED);
    if err != 0 { return err; }
    if damon_reclaim_enabled() == ENABLED { return 0; }
    if !damon_initialized() { return 0; }
    if CTX.is_null() { return -ENOMEM; }
    damon_reclaim_turn(ENABLED)
}

unsafe fn damon_reclaim_enabled_load(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    sprintf(buffer, "%c\n", if damon_reclaim_enabled() { 'Y' } else { 'N' })
}

unsafe fn damon_reclaim_kdamond_pid_store(_val: *const c_char, _kp: *const kernel_param) -> c_int { 0 }

unsafe fn damon_reclaim_kdamond_pid_load(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    let mut kdamond_pid = -1;
    if !CTX.is_null() {
        kdamond_pid = damon_kdamond_pid(CTX);
        if kdamond_pid < 0 { kdamond_pid = -1; }
    }
    sprintf(buffer, "%d\n", kdamond_pid)
}

unsafe fn damon_reclaim_init() -> c_int {
    let mut err: c_int;
    if !damon_initialized() { err = -ENOMEM; goto_init_out!(); }
    err = damon_modules_new_paddr_ctx_target(&mut CTX, &mut TARGET);
    if err != 0 { goto_init_out!(); }
    CALL_CONTROL.data = CTX as *mut c_void;
    if ENABLED { err = damon_reclaim_turn(true); }
    else { err = 0; }
    goto_init_out!();
}

// Module parameter declarations, descriptions, and module_init(damon_reclaim_init)
// are provided by the surrounding kernel module integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
