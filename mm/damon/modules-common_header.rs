/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common Code for DAMON Modules
 */

// Dependency supplied by the Linux module-parameter environment.

macro_rules! DEFINE_DAMON_MODULES_MON_ATTRS_PARAMS {
    ($attrs:expr) => {
        module_param_named!(sample_interval, $attrs.sample_interval, ulong, 0o600);
        module_param_named!(aggr_interval, $attrs.aggr_interval, ulong, 0o600);
        module_param_named!(min_nr_regions, $attrs.min_nr_regions, ulong, 0o600);
        module_param_named!(max_nr_regions, $attrs.max_nr_regions, ulong, 0o600);
    };
}

macro_rules! DEFINE_DAMON_MODULES_DAMOS_TIME_QUOTA {
    ($quota:expr) => {
        module_param_named!(quota_ms, $quota.ms, ulong, 0o600);
        module_param_named!(quota_reset_interval_ms, $quota.reset_interval, ulong, 0o600);
    };
}

macro_rules! DEFINE_DAMON_MODULES_DAMOS_QUOTAS {
    ($quota:expr) => {
        DEFINE_DAMON_MODULES_DAMOS_TIME_QUOTA!($quota);
        module_param_named!(quota_sz, $quota.sz, ulong, 0o600);
    };
}

macro_rules! DEFINE_DAMON_MODULES_WMARKS_PARAMS {
    ($wmarks:expr) => {
        module_param_named!(wmarks_interval, $wmarks.interval, ulong, 0o600);
        module_param_named!(wmarks_high, $wmarks.high, ulong, 0o600);
        module_param_named!(wmarks_mid, $wmarks.mid, ulong, 0o600);
        module_param_named!(wmarks_low, $wmarks.low, ulong, 0o600);
    };
}

// The C version uses token pasting (##) to form the exported parameter names.
// `module_param_named!` is supplied by the surrounding module-parameter
// environment and receives the equivalent constructed names here.
macro_rules! DEFINE_DAMON_MODULES_DAMOS_STATS_PARAMS {
    ($stat:expr, $try_name:ident, $succ_name:ident, $qt_exceed_name:ident) => {
        module_param_named!(concat!("nr_", stringify!($try_name)), $stat.nr_tried, ulong, 0o400);
        module_param_named!(concat!("bytes_", stringify!($try_name)), $stat.sz_tried, ulong, 0o400);
        module_param_named!(concat!("nr_", stringify!($succ_name)), $stat.nr_applied, ulong, 0o400);
        module_param_named!(concat!("bytes_", stringify!($succ_name)), $stat.sz_applied, ulong, 0o400);
        module_param_named!(concat!("nr_", stringify!($qt_exceed_name)), $stat.qt_exceeds, ulong, 0o400);
    };
}

// Opaque types supplied by the DAMON implementation.
pub enum damon_ctx {}
pub enum damon_target {}

extern "C" {
    pub fn damon_modules_new_paddr_ctx_target(
        ctxp: *mut *mut damon_ctx,
        targetp: *mut *mut damon_target,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
