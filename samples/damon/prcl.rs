// SPDX-License-Identifier: GPL-2.0
/*
 * proactive reclamation: monitor access pattern of a given process, find
 * regions that seems not accessed, and proactively page out the regions.
 */

// #define pr_fmt(fmt) "damon_sample_prcl: " fmt
// Dependencies supplied by the Linux kernel and other translation units are
// intentionally left as external Rust declarations.

// #ifdef MODULE_PARAM_PREFIX
// #undef MODULE_PARAM_PREFIX
// #endif
// #define MODULE_PARAM_PREFIX "damon_sample_prcl."

static mut target_pid: i32 = 0;
// module_param(target_pid, int, 0600);

unsafe extern "C" {
    fn damon_sample_prcl_enable_store(
        val: *const core::ffi::c_char,
        kp: *const kernel_param,
    ) -> i32;
    fn param_get_bool(
        kp: *const kernel_param,
        val: *mut core::ffi::c_char,
    ) -> i32;
}

#[repr(C)]
struct kernel_param_ops {
    set: Option<unsafe extern "C" fn(*const core::ffi::c_char, *const kernel_param) -> i32>,
    get: Option<unsafe extern "C" fn(*const kernel_param, *mut core::ffi::c_char) -> i32>,
}

#[repr(C)]
struct kernel_param;

static enabled_param_ops: kernel_param_ops = kernel_param_ops {
    set: Some(damon_sample_prcl_enable_store),
    get: Some(param_get_bool),
};

static mut enabled: bool = false;
// module_param_cb(enabled, &enabled_param_ops, &enabled, 0600);
// MODULE_PARM_DESC(enabled, "Enable or disable DAMON_SAMPLE_PRCL");

static mut ctx: *mut damon_ctx = core::ptr::null_mut();
static mut target_pidp: *mut pid = core::ptr::null_mut();

#[repr(C)]
struct damon_ctx;
#[repr(C)]
struct damon_target {
    pid: *mut pid,
}
#[repr(C)]
struct damon_region {
    nr_accesses: u32,
    ar: damon_addr_range,
}
#[repr(C)]
struct damon_addr_range {
    start: usize,
    end: usize,
}
#[repr(C)]
struct pid;
#[repr(C)]
struct damos;
#[repr(C)]
struct damon_call_control {
    fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    repeat: bool,
    data: *mut core::ffi::c_void,
}
#[repr(C)]
struct damos_access_pattern {
    min_sz_region: usize,
    max_sz_region: usize,
    min_nr_accesses: u32,
    max_nr_accesses: u32,
    min_age_region: u32,
    max_age_region: u32,
}
#[repr(C)]
struct damos_quota;
#[repr(C)]
struct damos_watermarks;

unsafe extern "C" {
    fn damon_for_each_target(c: *mut damon_ctx, target: *mut damon_target);
    fn damon_for_each_region(r: *mut damon_region, t: *mut damon_target);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_select_ops(ctx: *mut damon_ctx, ops: i32) -> i32;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_new_target() -> *mut damon_target;
    fn damon_add_target(ctx: *mut damon_ctx, target: *mut damon_target);
    fn find_get_pid(nr: i32) -> *mut pid;
    fn damon_new_scheme(
        pattern: *const damos_access_pattern,
        action: i32,
        apply_interval: usize,
        quota: *const damos_quota,
        watermarks: *const damos_watermarks,
        nid: i32,
    ) -> *mut damos;
    fn damon_set_schemes(ctx: *mut damon_ctx, schemes: *mut *mut damos, nr_schemes: usize);
    fn damon_start(ctx: *mut *mut damon_ctx, nr_ctxs: usize, exclusive: bool) -> i32;
    fn damon_call(ctx: *mut damon_ctx, control: *mut damon_call_control) -> i32;
    fn damon_stop(ctx: *mut *mut damon_ctx, nr_ctxs: usize);
    fn damon_initialized() -> bool;
    fn kstrtobool(val: *const core::ffi::c_char, result: *mut bool) -> i32;
}

const DAMON_OPS_VADDR: i32 = 0;
const DAMOS_PAGEOUT: i32 = 0;
const NUMA_NO_NODE: i32 = -1;
const PAGE_SIZE: usize = 4096;

unsafe extern "C" fn damon_sample_prcl_repeat_call_fn(data: *mut core::ffi::c_void) -> i32 {
    let c = data as *mut damon_ctx;
    let mut t: *mut damon_target = core::ptr::null_mut();

    damon_for_each_target(c, t);
    {
        let mut r: *mut damon_region = core::ptr::null_mut();
        let mut wss: usize = 0;

        damon_for_each_region(r, t);
        if (*r).nr_accesses > 0 {
            wss = wss.wrapping_add((*r).ar.end.wrapping_sub((*r).ar.start));
        }
        pr_info(b"wss: %lu\0".as_ptr() as *const _, wss);
    }
    0
}

static mut repeat_call_control: damon_call_control = damon_call_control {
    fn_: Some(damon_sample_prcl_repeat_call_fn),
    repeat: true,
    data: core::ptr::null_mut(),
};

unsafe fn damon_sample_prcl_start() -> i32 {
    let target: *mut damon_target;
    let scheme: *mut damos;
    let mut err: i32;

    pr_info(b"start\0".as_ptr() as *const _);

    ctx = damon_new_ctx();
    if ctx.is_null() { return -12; }
    if damon_select_ops(ctx, DAMON_OPS_VADDR) != 0 {
        damon_destroy_ctx(ctx); return -22;
    }
    target = damon_new_target();
    if target.is_null() { damon_destroy_ctx(ctx); return -12; }
    damon_add_target(ctx, target);
    target_pidp = find_get_pid(target_pid);
    if target_pidp.is_null() { damon_destroy_ctx(ctx); return -22; }
    (*target).pid = target_pidp;

    let pattern = damos_access_pattern { min_sz_region: PAGE_SIZE, max_sz_region: usize::MAX,
        min_nr_accesses: 0, max_nr_accesses: 0, min_age_region: 50, max_age_region: u32::MAX };
    let quota = damos_quota {};
    let watermarks = damos_watermarks {};
    scheme = damon_new_scheme(&pattern, DAMOS_PAGEOUT, 0, &quota, &watermarks, NUMA_NO_NODE);
    if scheme.is_null() { damon_destroy_ctx(ctx); return -12; }
    damon_set_schemes(ctx, &scheme, 1);
    err = damon_start(&mut ctx, 1, true);
    if err != 0 { damon_destroy_ctx(ctx); return err; }
    repeat_call_control.data = ctx as *mut core::ffi::c_void;
    err = damon_call(ctx, &mut repeat_call_control);
    if err != 0 { damon_destroy_ctx(ctx); }
    err
}

unsafe fn damon_sample_prcl_stop() {
    pr_info(b"stop\0".as_ptr() as *const _);
    if !ctx.is_null() { damon_stop(&mut ctx, 1); damon_destroy_ctx(ctx); }
}

unsafe extern "C" fn damon_sample_prcl_enable_store(val: *const core::ffi::c_char, _kp: *const kernel_param) -> i32 {
    let is_enabled = enabled;
    let err = kstrtobool(val, &mut enabled);
    if err != 0 { return err; }
    if enabled == is_enabled { return 0; }
    if !damon_initialized() { return 0; }
    if enabled { let err = damon_sample_prcl_start(); if err != 0 { enabled = false; } return err; }
    damon_sample_prcl_stop();
    0
}

unsafe extern "C" fn damon_sample_prcl_init() -> i32 {
    let mut err = 0;
    if !damon_initialized() { if enabled { enabled = false; } return -12; }
    if enabled { err = damon_sample_prcl_start(); if err != 0 { enabled = false; } }
    let _ = err;
    0
}

// module_init(damon_sample_prcl_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
