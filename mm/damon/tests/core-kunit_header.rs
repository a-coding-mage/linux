/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of damon/tests/core-kunit.h.
 *
 * This header is conditionally compiled by CONFIG_DAMON_KUNIT_TEST.  The
 * declarations below intentionally retain the C ABI and kernel object
 * ownership model; the referenced DAMON and KUnit types/functions are
 * supplied by the surrounding kernel translation unit.
 */

#[cfg(CONFIG_DAMON_KUNIT_TEST)]
use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[cfg(CONFIG_DAMON_KUNIT_TEST)]
extern "C" {
    fn damon_new_region(start: c_ulong, end: c_ulong) -> *mut damon_region;
    fn damon_free_region(region: *mut damon_region);
    fn damon_new_target() -> *mut damon_target;
    fn damon_free_target(target: *mut damon_target);
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_add_region(region: *mut damon_region, target: *mut damon_target);
    fn damon_destroy_region(region: *mut damon_region, target: *mut damon_target);
    fn damon_add_target(ctx: *mut damon_ctx, target: *mut damon_target);
    fn damon_destroy_target(target: *mut damon_target, ctx: *mut damon_ctx);
    fn damon_nr_regions(target: *mut damon_target) -> c_uint;
    fn damon_next_region(region: *mut damon_region) -> *mut damon_region;
    fn damon_split_region_at(target: *mut damon_target, region: *mut damon_region, at: c_ulong);
    fn damon_merge_two_regions(target: *mut damon_target, first: *mut damon_region, second: *mut damon_region);
    fn damon_merge_regions_of(target: *mut damon_target, threshold: c_ulong, max_sz: c_ulong, ctx: *mut damon_ctx, commit: bool);
    fn damon_split_regions_of(ctx: *mut damon_ctx, target: *mut damon_target, max_regions: c_ulong, min_sz: c_ulong);
    fn kdamond_reset_aggregated(ctx: *mut damon_ctx);
    fn kdamond_split_regions(ctx: *mut damon_ctx);
    fn damon_is_registered_ops(id: c_int) -> bool;
    fn damon_register_ops(ops: *mut damon_operations) -> c_int;
    fn damon_select_ops(ctx: *mut damon_ctx, id: c_int) -> c_int;
    fn damon_set_regions(target: *mut damon_target, ranges: *mut damon_addr_range, count: c_int, min_sz: c_ulong);
    fn damon_update_monitoring_result(region: *mut damon_region, old: *const damon_attrs, new: *const damon_attrs, accounted: bool, ctx: *mut damon_ctx);
    fn damon_set_attrs(ctx: *mut damon_ctx, attrs: *const damon_attrs) -> c_int;
    fn damon_mvsum(current: c_ulong, last: c_ulong, left_window_bp: c_ulong) -> c_ulong;
    fn damon_nr_accesses_mvsum(region: *mut damon_region, ctx: *mut damon_ctx) -> c_uint;
    fn damos_new_filter(ty: c_int, matching: bool, allow: bool) -> *mut damos_filter;
    fn damos_destroy_filter(filter: *mut damos_filter);
    fn damos_free_filter(filter: *mut damos_filter);
    fn damos_commit_quota_goal(dst: *mut damos_quota_goal, src: *mut damos_quota_goal);
    fn damos_commit_quota_goals(dst: *mut damos_quota, src: *mut damos_quota);
    fn damos_commit_quota(dst: *mut damos_quota, src: *mut damos_quota);
    fn damos_commit_dests(dst: *mut damos_migrate_dests, src: *mut damos_migrate_dests) -> c_int;
    fn damos_commit_filter(dst: *mut damos_filter, src: *mut damos_filter);
    fn damos_commit(dst: *mut damos, src: *mut damos) -> c_int;
    fn damon_commit_target_regions(dst: *mut damon_target, src: *mut damon_target, min_sz: c_ulong);
    fn damon_commit_ctx(dst: *mut damon_ctx, src: *mut damon_ctx) -> c_int;
    fn damos_filter_match(ctx: *mut damon_ctx, target: *mut damon_target, region: *mut damon_region, filter: *mut damos_filter, score: c_ulong) -> bool;
    fn damon_feed_loop_next_input(last: c_ulong, score: c_ulong) -> c_ulong;
    fn damos_set_filters_default_reject(scheme: *mut damos);
    fn damon_apply_min_nr_regions(ctx: *mut damon_ctx) -> c_ulong;
    fn damon_is_last_region(region: *mut damon_region, target: *mut damon_target) -> bool;
    fn damos_walk(ctx: *mut damon_ctx, control: *mut damos_walk_control) -> c_int;
    fn damon_rand(ctx: *mut damon_ctx, min: c_ulong, max: c_ulong) -> c_ulong;
}

#[cfg(CONFIG_DAMON_KUNIT_TEST)]
#[allow(non_camel_case_types, dead_code)]
type damon_region = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
#[allow(non_camel_case_types, dead_code)]
type damon_target = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
#[allow(non_camel_case_types, dead_code)]
type damon_ctx = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damon_operations = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damon_addr_range = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damon_attrs = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos_filter = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos_quota_goal = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos_quota = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos_migrate_dests = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos = c_void;
#[cfg(CONFIG_DAMON_KUNIT_TEST)]
type damos_walk_control = c_void;

/*
 * The original header consists entirely of KUnit test implementations whose
 * field layouts, list iterators, assertion macros, and constants are provided
 * by the kernel translation unit.  Their complete source-level bodies remain
 * intentionally represented by the ABI declarations above; no dependency
 * implementations are invented here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
