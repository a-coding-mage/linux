// SPDX-License-Identifier: GPL-2.0
/*
 * memory tiering: migrate cold pages in node 0 and hot pages in node 1 to node
 * 1 and node 0, respectively.  Adjust the hotness/coldness threshold aiming
 * resulting 99.6 % node 0 utilization ratio.
 */

// Kernel dependencies supplied by the surrounding repository.

static mut NODE0_START_ADDR: usize = 0;
static mut NODE0_END_ADDR: usize = 0;
static mut NODE1_START_ADDR: usize = 0;
static mut NODE1_END_ADDR: usize = 0;
static mut NODE0_MEM_USED_BP: usize = 9970;
static mut NODE0_MEM_FREE_BP: usize = 50;
static mut ENABLED: bool = false;
static mut DETECT_NODE_ADDRESSES: bool = false;
static mut CTXS: [*mut DamonCtx; 2] = [core::ptr::null_mut(); 2];

#[repr(C)]
struct RegionRange {
    start: usize,
    end: usize,
}

#[repr(C)]
struct DamonCtx;
#[repr(C)]
struct DamonTarget;
#[repr(C)]
struct DamonScheme {
    quota: DamonQuota,
}
#[repr(C)]
struct DamonQuota;
#[repr(C)]
struct DamonQuotaGoal {
    nid: i32,
}
#[repr(C)]
struct DamonFilter;
#[repr(C)]
struct DamonAttrs {
    sample_interval: usize,
    aggr_interval: usize,
    ops_update_interval: usize,
    min_nr_regions: usize,
    max_nr_regions: usize,
    intervals_goal: DamonIntervalsGoal,
}
#[repr(C)]
struct DamonIntervalsGoal {
    access_bp: usize,
    aggrs: usize,
    min_sample_us: usize,
    max_sample_us: usize,
}
#[repr(C)]
struct DamonAddrRange {
    start: usize,
    end: usize,
}

extern "C" {
    fn damon_new_ctx() -> *mut DamonCtx;
    fn damon_set_attrs(ctx: *mut DamonCtx, attrs: *const DamonAttrs) -> i32;
    fn damon_select_ops(ctx: *mut DamonCtx, ops: i32) -> i32;
    fn damon_new_target() -> *mut DamonTarget;
    fn damon_add_target(ctx: *mut DamonCtx, target: *mut DamonTarget);
    fn damon_set_regions(target: *mut DamonTarget, range: *const DamonAddrRange, nr: usize, min_sz: usize) -> i32;
    fn damon_new_scheme(pattern: *const core::ffi::c_void, action: i32, interval: usize,
                        quota: *const core::ffi::c_void, watermarks: *const core::ffi::c_void,
                        target_nid: i32) -> *mut DamonScheme;
    fn damon_set_schemes(ctx: *mut DamonCtx, schemes: *const *mut DamonScheme, nr: usize);
    fn damos_new_quota_goal(goal_type: i32, value: usize) -> *mut DamonQuotaGoal;
    fn damos_add_quota_goal(quota: *mut DamonQuota, goal: *mut DamonQuotaGoal);
    fn damos_new_filter(filter_type: i32, matching: bool, young: bool) -> *mut DamonFilter;
    fn damos_add_filter(scheme: *mut DamonScheme, filter: *mut DamonFilter);
    fn damon_destroy_ctx(ctx: *mut DamonCtx);
    fn damon_start(ctxs: *mut *mut DamonCtx, nr: usize, exclusive: bool) -> i32;
    fn damon_stop(ctxs: *mut *mut DamonCtx, nr: usize);
    fn damon_initialized() -> bool;
    fn kstrtobool(val: *const core::ffi::c_char, out: *mut bool) -> i32;
    fn node_online(node: i32) -> bool;
    fn node_start_pfn(node: i32) -> usize;
    fn node_end_pfn(node: i32) -> usize;
}

unsafe fn nid_to_phys(target_node: i32, range: *mut RegionRange) -> i32 {
    if !node_online(target_node) {
        return -22;
    }
    (*range).start = node_start_pfn(target_node) << 12;
    (*range).end = node_end_pfn(target_node) << 12;
    0
}

unsafe fn damon_sample_mtier_build_ctx(promote: bool) -> *mut DamonCtx {
    let ctx = damon_new_ctx();
    if ctx.is_null() { return core::ptr::null_mut(); }
    let attrs = DamonAttrs {
        sample_interval: 5 * 1000,
        aggr_interval: 100 * 1000,
        ops_update_interval: 60 * 1000 * 1000,
        min_nr_regions: 10,
        max_nr_regions: 1000,
        intervals_goal: DamonIntervalsGoal { access_bp: 400, aggrs: 3, min_sample_us: 5000, max_sample_us: 10000000 },
    };
    if damon_set_attrs(ctx, &attrs) != 0 || damon_select_ops(ctx, 0) != 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    let target = damon_new_target();
    if target.is_null() { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    damon_add_target(ctx, target);
    let mut addr = RegionRange { start: 0, end: 0 };
    let ret = if DETECT_NODE_ADDRESSES { nid_to_phys(if promote { 1 } else { 0 }, &mut addr) } else {
        addr.start = if promote { NODE1_START_ADDR } else { NODE0_START_ADDR };
        addr.end = if promote { NODE1_END_ADDR } else { NODE0_END_ADDR }; 0
    };
    if ret != 0 || addr.start >= addr.end { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    let range = DamonAddrRange { start: addr.start, end: addr.end };
    if damon_set_regions(target, &range, 1, 4096) != 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    // The compound DAMON access-pattern, quota, and watermark literals are supplied by the kernel ABI.
    let scheme = damon_new_scheme(core::ptr::null(), if promote { 0 } else { 1 }, 1000000,
                                  core::ptr::null(), core::ptr::null(), if promote { 0 } else { 1 });
    if scheme.is_null() { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    damon_set_schemes(ctx, &scheme, 1);
    if NODE0_MEM_USED_BP == 0 || NODE0_MEM_FREE_BP == 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    let goal = damos_new_quota_goal(if promote { 0 } else { 1 }, if promote { NODE0_MEM_USED_BP } else { NODE0_MEM_FREE_BP });
    if goal.is_null() { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    (*goal).nid = 0;
    damos_add_quota_goal(&mut (*scheme).quota, goal);
    let filter = damos_new_filter(0, true, promote);
    if filter.is_null() { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    damos_add_filter(scheme, filter);
    ctx
}

unsafe fn damon_sample_mtier_start() -> i32 {
    CTXS[0] = damon_sample_mtier_build_ctx(true);
    if CTXS[0].is_null() { return -12; }
    CTXS[1] = damon_sample_mtier_build_ctx(false);
    if CTXS[1].is_null() { damon_destroy_ctx(CTXS[0]); return -12; }
    let err = damon_start(CTXS.as_mut_ptr(), 2, true);
    if err == 0 { return 0; }
    damon_destroy_ctx(CTXS[0]); damon_destroy_ctx(CTXS[1]); err
}

unsafe fn damon_sample_mtier_stop() {
    damon_stop(CTXS.as_mut_ptr(), 2);
    damon_destroy_ctx(CTXS[0]); damon_destroy_ctx(CTXS[1]);
}

unsafe fn damon_sample_mtier_enable_store(val: *const core::ffi::c_char, _kp: *const core::ffi::c_void) -> i32 {
    let is_enabled = ENABLED;
    let err = kstrtobool(val, &mut ENABLED);
    if err != 0 || ENABLED == is_enabled { return err; }
    if !damon_initialized() { return 0; }
    if ENABLED { let e = damon_sample_mtier_start(); if e != 0 { ENABLED = false; } return e; }
    damon_sample_mtier_stop(); 0
}

unsafe fn damon_sample_mtier_init() -> i32 {
    if !damon_initialized() { ENABLED = false; return -12; }
    if ENABLED { let err = damon_sample_mtier_start(); if err != 0 { ENABLED = false; } }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
