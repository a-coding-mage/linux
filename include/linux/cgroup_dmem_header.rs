/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2023-2024 Intel Corporation
 */

use core::ffi::c_void;

#[repr(C)]
pub struct dmem_cgroup_pool_state {
    _private: [u8; 0],
}

/* Opaque definition of a cgroup region, used internally */
#[repr(C)]
pub struct dmem_cgroup_region {
    _private: [u8; 0],
}

/**
 * struct dmem_cgroup_ops - Operations for a dmem cgroup region.
 * @reclaim: Optional callback invoked when dmem.max is set below the current
 *           usage of a pool. The driver should attempt to free at least
 *           @target_bytes from @pool. May be called multiple times if usage
 *           remains above the limit after returning.
 *
 *           Return: 0 if some progress was made (even if less than
 *           @target_bytes was freed), -ENOSPC if no progress could be made
 *           (the caller will retry up to a bounded number of times), or
 *           another negative error code if a fatal error occurred (stops
 *           further reclaim attempts immediately).
 */
#[repr(C)]
pub struct dmem_cgroup_ops {
    pub reclaim: Option<unsafe extern "C" fn(
        pool: *mut dmem_cgroup_pool_state,
        target_bytes: u64,
        priv_: *mut c_void,
    ) -> i32>,
}

/**
 * struct dmem_cgroup_init - Initialization parameters for a dmem cgroup region.
 * @size: Size of the region in bytes.
 * @ops: Optional operations for this region. May be NULL.
 * @reclaim_priv: Opaque pointer passed to @ops->reclaim. May be NULL.
 */
#[repr(C)]
pub struct dmem_cgroup_init {
    pub size: u64,
    pub ops: *const dmem_cgroup_ops,
    pub reclaim_priv: *mut c_void,
}

#[cfg(feature = "CONFIG_CGROUP_DMEM")]
extern "C" {
    pub fn dmem_cgroup_register_region(
        init: *const dmem_cgroup_init,
        name_fmt: *const i8,
        ...,
    ) -> *mut dmem_cgroup_region;
    pub fn dmem_cgroup_unregister_region(region: *mut dmem_cgroup_region);
    pub fn dmem_cgroup_try_charge(
        region: *mut dmem_cgroup_region,
        size: u64,
        ret_pool: *mut *mut dmem_cgroup_pool_state,
        ret_limit_pool: *mut *mut dmem_cgroup_pool_state,
    ) -> i32;
    pub fn dmem_cgroup_uncharge(pool: *mut dmem_cgroup_pool_state, size: u64);
    pub fn dmem_cgroup_state_evict_valuable(
        limit_pool: *mut dmem_cgroup_pool_state,
        test_pool: *mut dmem_cgroup_pool_state,
        ignore_low: bool,
        ret_hit_low: *mut bool,
    ) -> bool;
    pub fn dmem_cgroup_below_min(
        root: *mut dmem_cgroup_pool_state,
        test: *mut dmem_cgroup_pool_state,
    ) -> bool;
    pub fn dmem_cgroup_below_low(
        root: *mut dmem_cgroup_pool_state,
        test: *mut dmem_cgroup_pool_state,
    ) -> bool;
    pub fn dmem_cgroup_get_common_ancestor(
        a: *mut dmem_cgroup_pool_state,
        b: *mut dmem_cgroup_pool_state,
    ) -> *mut dmem_cgroup_pool_state;
    pub fn dmem_cgroup_pool_state_put(pool: *mut dmem_cgroup_pool_state);
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_register_region(
    _init: *const dmem_cgroup_init,
    _name_fmt: *const i8,
    ...,
) -> *mut dmem_cgroup_region {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_unregister_region(_region: *mut dmem_cgroup_region) {}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_try_charge(
    _region: *mut dmem_cgroup_region,
    _size: u64,
    ret_pool: *mut *mut dmem_cgroup_pool_state,
    ret_limit_pool: *mut *mut dmem_cgroup_pool_state,
) -> i32 {
    *ret_pool = core::ptr::null_mut();
    if !ret_limit_pool.is_null() {
        *ret_limit_pool = core::ptr::null_mut();
    }
    0
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_uncharge(_pool: *mut dmem_cgroup_pool_state, _size: u64) {}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_state_evict_valuable(
    _limit_pool: *mut dmem_cgroup_pool_state,
    _test_pool: *mut dmem_cgroup_pool_state,
    _ignore_low: bool,
    _ret_hit_low: *mut bool,
) -> bool {
    true
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_below_min(
    _root: *mut dmem_cgroup_pool_state,
    _test: *mut dmem_cgroup_pool_state,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_below_low(
    _root: *mut dmem_cgroup_pool_state,
    _test: *mut dmem_cgroup_pool_state,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_get_common_ancestor(
    _a: *mut dmem_cgroup_pool_state,
    _b: *mut dmem_cgroup_pool_state,
) -> *mut dmem_cgroup_pool_state {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_CGROUP_DMEM"))]
pub unsafe fn dmem_cgroup_pool_state_put(_pool: *mut dmem_cgroup_pool_state) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
