// SPDX-License-Identifier: GPL-2.0
/*
 * Shows data access monitoring results in simple metrics.
 */

// pr_fmt(fmt) = "damon-stat: " fmt

// Dependencies supplied by the kernel DAMON environment are intentionally
// referenced here rather than implemented in this translation.

static mut ENABLED: bool = false;
static mut ESTIMATED_MEMORY_BANDWIDTH: usize = 0;
static mut MEMORY_IDLE_MS_PERCENTILES: [isize; 101] = [0; 101];
static mut AGGR_INTERVAL_US: usize = 0;
static mut DAMON_STAT_CONTEXT: *mut damon_ctx = core::ptr::null_mut();
static mut DAMON_STAT_LAST_REFRESH_JIFFIES: usize = 0;

unsafe fn damon_stat_set_estimated_memory_bandwidth(c: *mut damon_ctx) {
    let mut access_bytes: usize = 0;
    // damon_for_each_target(t, c)
    for t in damon_targets(c) {
        // damon_for_each_region(r, t)
        for r in damon_regions(t) {
            access_bytes = access_bytes.wrapping_add(
                (r.ar.end.wrapping_sub(r.ar.start)).wrapping_mul(r.nr_accesses),
            );
        }
    }
    ESTIMATED_MEMORY_BANDWIDTH = access_bytes
        .wrapping_mul(USEC_PER_MSEC)
        .wrapping_mul(MSEC_PER_SEC)
        / (*(*c).attrs.aggr_interval);
}

unsafe fn damon_stat_idletime(r: *const damon_region) -> isize {
    if (*r).nr_accesses != 0 {
        -1 * ((*r).age as isize + 1)
    } else {
        (*r).age as isize + 1
    }
}

unsafe extern "C" fn damon_stat_cmp_regions(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let ra = *(a as *const *const damon_region);
    let rb = *(b as *const *const damon_region);
    damon_stat_idletime(ra).wrapping_sub(damon_stat_idletime(rb)) as i32
}

unsafe fn damon_stat_sort_regions(
    c: *mut damon_ctx,
    sorted_ptr: *mut *mut *mut damon_region,
    nr_regions_ptr: *mut i32,
    total_sz_ptr: *mut usize,
) -> i32 {
    let mut region_pointers: *mut *mut damon_region = core::ptr::null_mut();
    let mut nr_regions: u32 = 0;
    let mut total_sz: usize = 0;
    for t in damon_targets(c) {
        region_pointers = kmalloc_objs(damon_nr_regions(t));
        if region_pointers.is_null() { return -ENOMEM; }
        for r in damon_regions(t) {
            *region_pointers.add(nr_regions as usize) = r;
            nr_regions += 1;
            total_sz = total_sz.wrapping_add(r.ar.end.wrapping_sub(r.ar.start));
        }
    }
    sort(region_pointers, nr_regions as usize, core::mem::size_of::<*mut damon_region>(), damon_stat_cmp_regions, core::ptr::null_mut());
    *sorted_ptr = region_pointers;
    *nr_regions_ptr = nr_regions as i32;
    *total_sz_ptr = total_sz;
    0
}

unsafe fn damon_stat_set_idletime_percentiles(c: *mut damon_ctx) {
    let mut sorted_regions: *mut *mut damon_region = core::ptr::null_mut();
    let mut nr_regions = 0i32;
    let mut total_sz = 0usize;
    if damon_stat_sort_regions(c, &mut sorted_regions, &mut nr_regions, &mut total_sz) != 0 { return; }
    let mut accounted_bytes = 0usize;
    let mut next_percentile = 0usize;
    for i in 0..nr_regions as usize {
        let region = *sorted_regions.add(i);
        accounted_bytes = accounted_bytes.wrapping_add(region.ar.end.wrapping_sub(region.ar.start));
        while next_percentile <= accounted_bytes * 100 / total_sz {
            MEMORY_IDLE_MS_PERCENTILES[next_percentile] = damon_stat_idletime(region)
                * (*c).attrs.aggr_interval as isize / USEC_PER_MSEC as isize;
            next_percentile += 1;
        }
    }
    kfree(sorted_regions as *mut core::ffi::c_void);
}

unsafe extern "C" fn damon_stat_damon_call_fn(data: *mut core::ffi::c_void) -> i32 {
    let c = data as *mut damon_ctx;
    if time_before_eq(jiffies(), DAMON_STAT_LAST_REFRESH_JIFFIES + secs_to_jiffies(5)) { return 0; }
    DAMON_STAT_LAST_REFRESH_JIFFIES = jiffies();
    AGGR_INTERVAL_US = (*c).attrs.aggr_interval;
    damon_stat_set_estimated_memory_bandwidth(c);
    damon_stat_set_idletime_percentiles(c);
    0
}

unsafe fn damon_stat_build_ctx() -> *mut damon_ctx {
    let ctx = damon_new_ctx();
    if ctx.is_null() { return core::ptr::null_mut(); }
    let attrs = damon_attrs { sample_interval: 5 * USEC_PER_MSEC, aggr_interval: 100 * USEC_PER_MSEC, ops_update_interval: 60 * USEC_PER_MSEC * MSEC_PER_SEC, min_nr_regions: 10, max_nr_regions: 1000, ..core::mem::zeroed() };
    let intervals_goal = damon_intervals_goal { access_bp: 400, aggrs: 3, min_sample_us: 5000, max_sample_us: 10000000 };
    (*ctx).attrs.intervals_goal = intervals_goal;
    if damon_set_attrs(ctx, &attrs) != 0 || damon_select_ops(ctx, DAMON_OPS_PADDR) != 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    let target = damon_new_target();
    if target.is_null() { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    damon_add_target(ctx, target);
    let mut start = 0usize; let mut end = 0usize;
    if damon_set_region_system_rams_default(target, &mut start, &mut end, (*ctx).addr_unit, (*ctx).min_region_sz) != 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    ctx
}

// The remaining module entry points preserve the C lifecycle and parameter interfaces.
unsafe fn damon_stat_start() -> i32 {
    if !DAMON_STAT_CONTEXT.is_null() { if damon_is_running(DAMON_STAT_CONTEXT) { return -EAGAIN; } damon_destroy_ctx(DAMON_STAT_CONTEXT); }
    DAMON_STAT_CONTEXT = damon_stat_build_ctx();
    if DAMON_STAT_CONTEXT.is_null() { return -ENOMEM; }
    let err = damon_start(&mut DAMON_STAT_CONTEXT, 1, true);
    if err != 0 { damon_destroy_ctx(DAMON_STAT_CONTEXT); DAMON_STAT_CONTEXT = core::ptr::null_mut(); return err; }
    DAMON_STAT_LAST_REFRESH_JIFFIES = jiffies();
    damon_call(DAMON_STAT_CONTEXT, &mut CALL_CONTROL)
}

unsafe fn damon_stat_stop() { damon_stop(&mut DAMON_STAT_CONTEXT, 1); damon_destroy_ctx(DAMON_STAT_CONTEXT); DAMON_STAT_CONTEXT = core::ptr::null_mut(); }
unsafe fn damon_stat_enabled() -> bool { !DAMON_STAT_CONTEXT.is_null() && damon_is_running(DAMON_STAT_CONTEXT) }

unsafe fn damon_stat_enabled_store(val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let err = kstrtobool(val, &mut ENABLED);
    if err != 0 { return err; }
    if damon_stat_enabled() == ENABLED { return 0; }
    if !damon_initialized() { return 0; }
    if ENABLED { damon_stat_start() } else { damon_stat_stop(); 0 }
}

unsafe fn damon_stat_enabled_load(buffer: *mut core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    sprintf(buffer, "%c\n", if damon_stat_enabled() { 'Y' } else { 'N' })
}

unsafe fn damon_stat_kdamond_pid_store(_val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 { 0 }

unsafe fn damon_stat_kdamond_pid_load(buffer: *mut core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let mut pid = -1;
    if !DAMON_STAT_CONTEXT.is_null() {
        pid = damon_kdamond_pid(DAMON_STAT_CONTEXT);
        if pid < 1 { pid = -1; }
    }
    sprintf(buffer, "%d\n", pid)
}

static mut CALL_CONTROL: damon_call_control = damon_call_control { fn_: Some(damon_stat_damon_call_fn), repeat: true, data: core::ptr::null_mut() };

unsafe fn damon_stat_init() -> i32 {
    let mut err = 0;
    if !damon_initialized() { err = -ENOMEM; }
    else if ENABLED { err = damon_stat_start(); }
    if err != 0 && ENABLED { ENABLED = false; }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
