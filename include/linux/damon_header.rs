/* SPDX-License-Identifier: GPL-2.0 */
/* DAMON api -- direct Rust translation of damon.h. */

/* Linux dependencies are supplied by the surrounding kernel/Rust environment. */
use core::ffi::c_void;

pub const DAMON_MIN_REGION_SZ: usize = PAGE_SIZE;
pub const DAMON_MAX_PROBES: usize = 4;
pub const DAMOS_MAX_SCORE: u32 = 99;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct rnd_state { _private: [u8; 0] }
extern "Rust" { static PAGE_SIZE: usize; }

#[repr(C)] pub struct damon_addr_range { pub start: usize, pub end: usize }
#[repr(C)] pub struct damon_size_range { pub min: usize, pub max: usize }

#[repr(C)] pub struct damon_region {
    pub ar: damon_addr_range, pub sampling_addr: usize, pub nr_accesses: u32,
    pub probe_hits: [u8; DAMON_MAX_PROBES], pub age: u32, pub list: list_head,
    pub last_nr_accesses: u32, pub last_probe_hits: [u8; DAMON_MAX_PROBES],
}
#[repr(C)] pub struct damon_target {
    pub pid: *mut pid, pub obsolete: bool, pub nr_regions: u32,
    pub regions_list: list_head, pub list: list_head,
}

#[repr(C)] #[derive(Copy, Clone)] pub enum damos_action { DAMOS_WILLNEED, DAMOS_COLD, DAMOS_PAGEOUT, DAMOS_HUGEPAGE, DAMOS_NOHUGEPAGE, DAMOS_COLLAPSE, DAMOS_LRU_PRIO, DAMOS_LRU_DEPRIO, DAMOS_MIGRATE_HOT, DAMOS_MIGRATE_COLD, DAMOS_STAT, NR_DAMOS_ACTIONS }
#[repr(C)] #[derive(Copy, Clone)] pub enum damos_quota_goal_metric { DAMOS_QUOTA_USER_INPUT, DAMOS_QUOTA_SOME_MEM_PSI_US, DAMOS_QUOTA_NODE_MEM_USED_BP, DAMOS_QUOTA_NODE_MEM_FREE_BP, DAMOS_QUOTA_NODE_MEMCG_USED_BP, DAMOS_QUOTA_NODE_MEMCG_FREE_BP, DAMOS_QUOTA_ACTIVE_MEM_BP, DAMOS_QUOTA_INACTIVE_MEM_BP, DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP, NR_DAMOS_QUOTA_GOAL_METRICS }
#[repr(C)] pub union damos_quota_goal_fields { pub node: damos_quota_goal_node, pub last_psi_total: u64 }
#[repr(C)] pub struct damos_quota_goal_node { pub nid: i32, pub memcg_id: u64 }
#[repr(C)] pub struct damos_quota_goal { pub metric: damos_quota_goal_metric, pub target_value: usize, pub current_value: usize, pub fields: damos_quota_goal_fields, pub list: list_head }
#[repr(C)] pub enum damos_quota_goal_tuner { DAMOS_QUOTA_GOAL_TUNER_CONSIST, DAMOS_QUOTA_GOAL_TUNER_TEMPORAL }
#[repr(C)] pub struct damos_quota {
    pub reset_interval: usize, pub ms: usize, pub sz: usize, pub goal_tuner: damos_quota_goal_tuner, pub esz: usize,
    pub fail_charge_num: u32, pub fail_charge_denom: u32, pub weight_sz: u32, pub weight_nr_accesses: u32, pub weight_age: u32,
    pub goals: list_head, pub total_charged_sz: usize, pub total_charged_ns: usize, pub charged_sz: usize, pub charged_from: usize,
    pub charge_target_from: *mut damon_target, pub charge_addr_from: usize, pub min_score: u32, pub esz_bp: usize,
}
#[repr(C)] pub enum damos_wmark_metric { DAMOS_WMARK_NONE, DAMOS_WMARK_FREE_MEM_RATE, NR_DAMOS_WMARK_METRICS }
#[repr(C)] pub struct damos_watermarks { pub metric: damos_wmark_metric, pub interval: usize, pub high: usize, pub mid: usize, pub low: usize, pub activated: bool }
#[repr(C)] pub struct damos_stat { pub nr_tried: usize, pub sz_tried: usize, pub nr_applied: usize, pub sz_applied: usize, pub sz_ops_filter_passed: usize, pub qt_exceeds: usize, pub nr_snapshots: usize }
#[repr(C)] pub enum damos_filter_type { DAMOS_FILTER_TYPE_ANON, DAMOS_FILTER_TYPE_ACTIVE, DAMOS_FILTER_TYPE_MEMCG, DAMOS_FILTER_TYPE_YOUNG, DAMOS_FILTER_TYPE_HUGEPAGE_SIZE, DAMOS_FILTER_TYPE_UNMAPPED, DAMOS_FILTER_TYPE_ADDR, DAMOS_FILTER_TYPE_TARGET, NR_DAMOS_FILTER_TYPES }
#[repr(C)] pub union damos_filter_fields { pub memcg_id: u64, pub addr_range: damon_addr_range, pub target_idx: i32, pub sz_range: damon_size_range }
#[repr(C)] pub struct damos_filter { pub type_: damos_filter_type, pub matching: bool, pub allow: bool, pub fields: damos_filter_fields, pub list: list_head }

pub enum damon_ctx {}
pub enum damos {}
#[repr(C)] pub struct damos_walk_control { pub walk_fn: Option<unsafe extern "C" fn(*mut c_void, *mut damon_ctx, *mut damon_target, *mut damon_region, *mut damos, usize)>, pub data: *mut c_void, pub completion: completion, pub canceled: bool }
#[repr(C)] pub struct damos_access_pattern { pub min_sz_region: usize, pub max_sz_region: usize, pub min_nr_accesses: u32, pub max_nr_accesses: u32, pub min_age_region: u32, pub max_age_region: u32 }
#[repr(C)] pub struct damos_migrate_dests { pub node_id_arr: *mut u32, pub weight_arr: *mut u32, pub nr_dests: usize }
#[repr(C)] pub struct damos {
    pub pattern: damos_access_pattern, pub action: damos_action, pub apply_interval_us: usize, pub quota: damos_quota, pub wmarks: damos_watermarks,
    pub target_nid: i32, pub migrate_dests: damos_migrate_dests, pub stat: damos_stat, pub max_nr_snapshots: usize, pub next_apply_sis: usize,
    pub walk_completed: bool, pub core_filters_allowed: bool, pub core_filters_default_reject: bool, pub ops_filters_default_reject: bool,
    pub core_filters: list_head, pub ops_filters: list_head, pub last_applied: *mut c_void, pub list: list_head,
}
#[repr(C)] pub enum damon_ops_id { DAMON_OPS_VADDR, DAMON_OPS_FVADDR, DAMON_OPS_PADDR, NR_DAMON_OPS }
#[repr(C)] pub struct damon_operations {
    pub id: damon_ops_id, pub init: Option<unsafe extern "C" fn(*mut damon_ctx)>, pub update: Option<unsafe extern "C" fn(*mut damon_ctx)>,
    pub prepare_access_checks: Option<unsafe extern "C" fn(*mut damon_ctx)>, pub check_accesses: Option<unsafe extern "C" fn(*mut damon_ctx) -> u32>,
    pub apply_probes: Option<unsafe extern "C" fn(*mut damon_ctx, bool, bool) -> u32>, pub get_scheme_score: Option<unsafe extern "C" fn(*mut damon_ctx, *mut damon_region, *mut damos) -> i32>,
    pub apply_scheme: Option<unsafe extern "C" fn(*mut damon_ctx, *mut damon_target, *mut damon_region, *mut damos, *mut usize) -> usize>,
    pub target_valid: Option<unsafe extern "C" fn(*mut damon_target) -> bool>, pub cleanup_target: Option<unsafe extern "C" fn(*mut damon_target)>,
}
#[repr(C)] pub struct damon_call_control { pub fn_: Option<unsafe extern "C" fn(*mut c_void) -> i32>, pub data: *mut c_void, pub repeat: bool, pub return_code: i32, pub dealloc_on_cancel: bool, pub completion: completion, pub canceled: bool, pub list: list_head }
#[repr(C)] pub struct damon_intervals_goal { pub access_bp: usize, pub aggrs: usize, pub min_sample_us: usize, pub max_sample_us: usize }
#[repr(C)] pub enum damon_filter_type { DAMON_FILTER_TYPE_ANON, DAMON_FILTER_TYPE_MEMCG }
#[repr(C)] pub union damon_filter_fields { pub memcg_id: u64 }
#[repr(C)] pub struct damon_filter { pub type_: damon_filter_type, pub matching: bool, pub allow: bool, pub fields: damon_filter_fields, pub list: list_head }
#[repr(C)] pub struct damon_probe { pub weight: u32, pub filters: list_head, pub list: list_head }
#[repr(C)] pub struct damon_attrs { pub sample_interval: usize, pub aggr_interval: usize, pub ops_update_interval: usize, pub intervals_goal: damon_intervals_goal, pub min_nr_regions: usize, pub max_nr_regions: usize, pub aggr_samples: usize }
#[repr(C)] pub struct damon_ctx {
    pub attrs: damon_attrs, pub passed_sample_intervals: usize, pub next_aggregation_sis: usize, pub next_ops_update_sis: usize, pub next_intervals_tune_sis: usize,
    pub kdamond_started: completion, pub regions_score_histogram: *mut usize, pub call_controls: list_head, pub call_controls_obsolete: bool, pub call_controls_lock: mutex,
    pub walk_control: *mut damos_walk_control, pub walk_control_obsolete: bool, pub walk_control_lock: mutex, pub maybe_corrupted: bool, pub kdamond: *mut task_struct,
    pub kdamond_lock: mutex, pub addr_unit: usize, pub min_region_sz: usize, pub pause: bool, pub ops: damon_operations, pub adaptive_targets: list_head,
    pub probes: list_head, pub schemes: list_head, pub rnd_state: rnd_state,
}

/* Inline helpers retain C pointer behavior; list/container helpers are external kernel facilities. */
extern "Rust" {
    fn container_of<T>(p: *mut list_head, member: *const T) -> *mut T;
    fn list_last_entry<T>(h: *mut list_head, member: *const T) -> *mut T;
    fn list_first_entry<T>(h: *mut list_head, member: *const T) -> *mut T;
    fn prandom_u32_state(s: *mut rnd_state) -> u32;
    fn mul_u64_u64_shr(a: u64, b: u64, shift: u32) -> u64;
}
pub unsafe fn damon_rand(ctx: *mut damon_ctx, l: usize, r: usize) -> usize { let span = r.wrapping_sub(l); if span <= u32::MAX as usize { let rnd = prandom_u32_state(&mut (*ctx).rnd_state); return l.wrapping_add(((rnd as u64 * span as u64) >> 32) as usize); } let rnd = ((prandom_u32_state(&mut (*ctx).rnd_state) as u64) << 32) | prandom_u32_state(&mut (*ctx).rnd_state) as u64; l.wrapping_add(mul_u64_u64_shr(rnd, span as u64, 64) as usize) }
pub unsafe fn damon_next_region(r: *mut damon_region) -> *mut damon_region { container_of((*r).list.next, core::ptr::addr_of!((*r).list)) }
pub unsafe fn damon_prev_region(r: *mut damon_region) -> *mut damon_region { container_of((*r).list.prev, core::ptr::addr_of!((*r).list)) }
pub unsafe fn damon_last_region(t: *mut damon_target) -> *mut damon_region { list_last_entry(&mut (*t).regions_list, core::ptr::addr_of!((*t).regions_list)) }
pub unsafe fn damon_first_region(t: *mut damon_target) -> *mut damon_region { list_first_entry(&mut (*t).regions_list, core::ptr::addr_of!((*t).regions_list)) }
pub unsafe fn damon_sz_region(r: *mut damon_region) -> usize { (*r).ar.end.wrapping_sub((*r).ar.start) }

/* C iteration macros are preserved as dependency-facing macro intent. */
/* damon_for_each_filter, damon_for_each_probe, damon_for_each_region,
 * damon_for_each_target, damon_for_each_scheme, and corresponding _safe forms
 * expand to the kernel list_for_each_entry(_safe) primitives. */

/* CONFIG_DAMON-gated external declarations. */
extern "Rust" {
    fn damon_new_filter(type_: damon_filter_type, matching: bool, allow: bool) -> *mut damon_filter;
    fn damon_add_filter(probe: *mut damon_probe, f: *mut damon_filter);
    fn damon_destroy_filter(f: *mut damon_filter);
    fn damon_new_probe() -> *mut damon_probe;
    fn damon_add_probe(ctx: *mut damon_ctx, probe: *mut damon_probe);
    fn damon_new_region(start: usize, end: usize) -> *mut damon_region;
    fn damon_nr_accesses_mvsum(r: *mut damon_region, ctx: *mut damon_ctx) -> u32;
    fn damon_probe_hits_mvsum(probe_idx: i32, r: *mut damon_region, ctx: *mut damon_ctx) -> u8;
    fn damon_probe_hits_wsum(r: *mut damon_region, last: bool, ctx: *mut damon_ctx) -> u32;
    fn damon_set_regions(t: *mut damon_target, ranges: *mut damon_addr_range, nr_ranges: u32, min_region_sz: usize) -> i32;
    fn damon_update_region_access_rate(r: *mut damon_region, accessed: bool);
    fn damos_new_filter(type_: damos_filter_type, matching: bool, allow: bool) -> *mut damos_filter;
    fn damos_add_filter(s: *mut damos, f: *mut damos_filter);
    fn damos_filter_for_ops(type_: damos_filter_type) -> bool;
    fn damos_destroy_filter(f: *mut damos_filter);
    fn damos_new_quota_goal(metric: damos_quota_goal_metric, target_value: usize) -> *mut damos_quota_goal;
    fn damos_add_quota_goal(q: *mut damos_quota, g: *mut damos_quota_goal);
    fn damos_destroy_quota_goal(goal: *mut damos_quota_goal);
    fn damon_new_scheme(pattern: *mut damos_access_pattern, action: damos_action, apply_interval_us: usize, quota: *mut damos_quota, wmarks: *mut damos_watermarks, target_nid: i32) -> *mut damos;
    fn damon_add_scheme(ctx: *mut damon_ctx, s: *mut damos);
    fn damon_destroy_scheme(s: *mut damos);
    fn damos_commit_quota_goals(dst: *mut damos_quota, src: *mut damos_quota) -> i32;
    fn damon_new_target() -> *mut damon_target;
    fn damon_add_target(ctx: *mut damon_ctx, t: *mut damon_target);
    fn damon_targets_empty(ctx: *mut damon_ctx) -> bool;
    fn damon_free_target(t: *mut damon_target);
    fn damon_destroy_target(t: *mut damon_target, ctx: *mut damon_ctx);
    fn damon_nr_regions(t: *mut damon_target) -> u32;
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_set_attrs(ctx: *mut damon_ctx, attrs: *mut damon_attrs) -> i32;
    fn damon_set_schemes(ctx: *mut damon_ctx, schemes: *mut *mut damos, nr_schemes: isize);
    fn damon_commit_ctx(old_ctx: *mut damon_ctx, new_ctx: *mut damon_ctx) -> i32;
    fn damon_nr_running_ctxs() -> i32;
    fn damon_is_registered_ops(id: damon_ops_id) -> bool;
    fn damon_register_ops(ops: *mut damon_operations) -> i32;
    fn damon_select_ops(ctx: *mut damon_ctx, id: damon_ops_id) -> i32;
    fn damon_initialized() -> bool;
    fn damon_start(ctxs: *mut *mut damon_ctx, nr_ctxs: i32, exclusive: bool) -> i32;
    fn damon_stop(ctxs: *mut *mut damon_ctx, nr_ctxs: i32);
    fn damon_is_running(ctx: *mut damon_ctx) -> bool;
    fn damon_kdamond_pid(ctx: *mut damon_ctx) -> i32;
    fn damon_call(ctx: *mut damon_ctx, control: *mut damon_call_control) -> i32;
    fn damos_walk(ctx: *mut damon_ctx, control: *mut damos_walk_control) -> i32;
    fn damon_set_region_system_rams_default(t: *mut damon_target, start: *mut usize, end: *mut usize, addr_unit: usize, min_region_sz: usize) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
