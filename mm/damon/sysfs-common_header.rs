/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common Code for DAMON Sysfs Interface
 */

// C dependencies: <linux/damon.h>, <linux/kobject.h>

use core::ffi::c_char;

// Opaque types supplied by the corresponding Linux/DAMON dependencies.
pub enum mutex {}
pub enum kobject {}
pub enum kobj_type {}
pub enum damon_ctx {}
pub enum damon_sysfs_scheme {}
pub enum damon_target {}
pub enum damon_region {}
pub enum damos {}

extern "C" {
    pub static mut damon_sysfs_lock: mutex;
}

#[repr(C)]
pub struct damon_sysfs_ul_range {
    pub kobj: kobject,
    pub min: usize,
    pub max: usize,
}

extern "C" {
    pub fn damon_sysfs_ul_range_alloc(min: usize, max: usize)
        -> *mut damon_sysfs_ul_range;
    pub fn damon_sysfs_ul_range_release(kobj: *mut kobject);

    pub static damon_sysfs_ul_range_ktype: kobj_type;
}

/*
 * schemes directory
 */

#[repr(C)]
pub struct damon_sysfs_schemes {
    pub kobj: kobject,
    pub schemes_arr: *mut *mut damon_sysfs_scheme,
    pub nr: core::ffi::c_int,
}

extern "C" {
    pub fn damon_sysfs_schemes_alloc() -> *mut damon_sysfs_schemes;
    pub fn damon_sysfs_schemes_rm_dirs(schemes: *mut damon_sysfs_schemes);

    pub static damon_sysfs_schemes_ktype: kobj_type;

    pub fn damon_sysfs_add_schemes(
        ctx: *mut damon_ctx,
        sysfs_schemes: *mut damon_sysfs_schemes,
    ) -> core::ffi::c_int;

    pub fn damon_sysfs_schemes_update_stats(
        sysfs_schemes: *mut damon_sysfs_schemes,
        ctx: *mut damon_ctx,
    );

    pub fn damos_sysfs_populate_region_dir(
        sysfs_schemes: *mut damon_sysfs_schemes,
        ctx: *mut damon_ctx,
        t: *mut damon_target,
        r: *mut damon_region,
        s: *mut damos,
        total_bytes_only: bool,
        sz_filter_passed: usize,
    );

    pub fn damon_sysfs_schemes_clear_regions(
        sysfs_schemes: *mut damon_sysfs_schemes,
    ) -> core::ffi::c_int;

    pub fn damos_sysfs_set_quota_scores(
        sysfs_schemes: *mut damon_sysfs_schemes,
        ctx: *mut damon_ctx,
    ) -> core::ffi::c_int;

    pub fn damos_sysfs_update_effective_quotas(
        sysfs_schemes: *mut damon_sysfs_schemes,
        ctx: *mut damon_ctx,
    );

    pub fn damon_sysfs_memcg_path_to_id(
        memcg_path: *mut c_char,
        id: *mut u64,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
