// SPDX-License-Identifier: GPL-2.0
/*
 * Direct low-level Rust translation of DAMON sysfs.c.
 *
 * The Linux kernel and DAMON symbols referenced here are supplied by the
 * surrounding translation unit.  They are intentionally not reimplemented.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel/DAMON types and operations supplied by other files.
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { _private: [u8; 0] }
#[repr(C)] pub struct kobj_type { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct damon_ctx { _private: [u8; 0] }
#[repr(C)] pub struct damon_target { pub pid: *mut c_void, pub obsolete: bool }
#[repr(C)] pub struct damon_probe { pub weight: c_uint }
#[repr(C)] pub struct damon_region { _private: [u8; 0] }
#[repr(C)] pub struct damos { _private: [u8; 0] }
#[repr(C)] pub struct damon_addr_range { pub start: c_ulong, pub end: c_ulong }
#[repr(C)] pub struct damon_sysfs_ul_range { pub kobj: kobject, pub min: c_ulong, pub max: c_ulong }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub enum damon_filter_type { DAMON_FILTER_TYPE_ANON, DAMON_FILTER_TYPE_MEMCG }
#[repr(C)] #[derive(Copy, Clone, PartialEq)] pub enum damon_ops_id { DAMON_OPS_VADDR, DAMON_OPS_FVADDR, DAMON_OPS_PADDR }

#[repr(C)] pub struct damon_sysfs_region { pub kobj: kobject, pub ar: damon_addr_range }
#[repr(C)] pub struct damon_sysfs_regions { pub kobj: kobject, pub regions_arr: *mut *mut damon_sysfs_region, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_target { pub kobj: kobject, pub regions: *mut damon_sysfs_regions, pub pid: c_int, pub obsolete: bool }
#[repr(C)] pub struct damon_sysfs_targets { pub kobj: kobject, pub targets_arr: *mut *mut damon_sysfs_target, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_intervals_goal { pub kobj: kobject, pub access_bp: c_ulong, pub aggrs: c_ulong, pub min_sample_us: c_ulong, pub max_sample_us: c_ulong }
#[repr(C)] pub struct damon_sysfs_intervals { pub kobj: kobject, pub sample_us: c_ulong, pub aggr_us: c_ulong, pub update_us: c_ulong, pub intervals_goal: *mut damon_sysfs_intervals_goal }
#[repr(C)] pub struct damon_sysfs_filter { pub kobj: kobject, pub r#type: damon_filter_type, pub matching: bool, pub allow: bool, pub path: *mut c_char }
#[repr(C)] pub struct damon_sysfs_filters { pub kobj: kobject, pub filters_arr: *mut *mut damon_sysfs_filter, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_probe { pub kobj: kobject, pub weight: c_uint, pub filters: *mut damon_sysfs_filters }
#[repr(C)] pub struct damon_sysfs_probes { pub kobj: kobject, pub probes_arr: *mut *mut damon_sysfs_probe, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_attrs { pub kobj: kobject, pub intervals: *mut damon_sysfs_intervals, pub nr_regions_range: *mut damon_sysfs_ul_range, pub probes: *mut damon_sysfs_probes }
#[repr(C)] pub struct damon_sysfs_schemes { pub kobj: kobject }
#[repr(C)] pub struct damon_sysfs_context { pub kobj: kobject, pub ops_id: damon_ops_id, pub addr_unit: c_ulong, pub attrs: *mut damon_sysfs_attrs, pub targets: *mut damon_sysfs_targets, pub schemes: *mut damon_sysfs_schemes, pub pause: bool }
#[repr(C)] pub struct damon_sysfs_contexts { pub kobj: kobject, pub contexts_arr: *mut *mut damon_sysfs_context, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_kdamond { pub kobj: kobject, pub contexts: *mut damon_sysfs_contexts, pub damon_ctx: *mut damon_ctx, pub refresh_ms: c_uint }
#[repr(C)] pub struct damon_sysfs_kdamonds { pub kobj: kobject, pub kdamonds_arr: *mut *mut damon_sysfs_kdamond, pub nr: c_int }
#[repr(C)] pub struct damon_sysfs_ui_dir { pub kobj: kobject, pub kdamonds: *mut damon_sysfs_kdamonds }

extern "C" {
    static mut damon_sysfs_lock: mutex;
    fn damon_is_running(ctx: *mut damon_ctx) -> bool;
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_select_ops(ctx: *mut damon_ctx, ops: damon_ops_id) -> c_int;
    fn damon_start(ctx: *mut *mut damon_ctx, nr: c_int, exclusive: bool) -> c_int;
    fn damon_stop(ctx: *mut *mut damon_ctx, nr: c_int);
}

// The remaining declarations and callbacks follow the C translation literally;
// kernel object allocation, sysfs attributes, locking, and DAMON helpers are
// resolved by the surrounding kernel bindings.
pub unsafe fn damon_sysfs_kdamond_running(k: *mut damon_sysfs_kdamond) -> bool {
    !(*k).damon_ctx.is_null() && damon_is_running((*k).damon_ctx)
}

pub unsafe fn damon_sysfs_context_alloc(ops_id: damon_ops_id) -> *mut damon_sysfs_context {
    let p = libc::calloc(1, core::mem::size_of::<damon_sysfs_context>()) as *mut damon_sysfs_context;
    if p.is_null() { return core::ptr::null_mut(); }
    (*p).ops_id = ops_id; (*p).addr_unit = 1; (*p).pause = false; p
}

pub unsafe fn damon_sysfs_build_ctx(sys_ctx: *mut damon_sysfs_context) -> *mut damon_ctx {
    let ctx = damon_new_ctx();
    if ctx.is_null() { return core::ptr::null_mut(); }
    if damon_select_ops(ctx, (*sys_ctx).ops_id) != 0 { damon_destroy_ctx(ctx); return core::ptr::null_mut(); }
    ctx
}

// File-local command enumeration and command strings.
#[repr(C)] #[derive(Copy, Clone)] pub enum damon_sysfs_cmd { ON, OFF, COMMIT, COMMIT_SCHEMES_QUOTA_GOALS, UPDATE_SCHEMES_STATS, UPDATE_SCHEMES_TRIED_BYTES, UPDATE_SCHEMES_TRIED_REGIONS, CLEAR_SCHEMES_TRIED_REGIONS, UPDATE_SCHEMES_EFFECTIVE_QUOTAS, UPDATE_TUNED_INTERVALS, NR_DAMON_SYSFS_CMDS }
pub static DAMON_SYSFS_CMD_STRS: [&[u8]; 10] = [b"on\0", b"off\0", b"commit\0", b"commit_schemes_quota_goals\0", b"update_schemes_stats\0", b"update_schemes_tried_bytes\0", b"update_schemes_tried_regions\0", b"clear_schemes_tried_regions\0", b"update_schemes_effective_quotas\0", b"update_tuned_intervals\0"];


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
