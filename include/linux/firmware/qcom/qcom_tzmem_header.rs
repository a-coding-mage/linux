/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023-2024 Linaro Ltd.
 */

//! Rust translation of `qcom_tzmem.h`.

use core::ffi::c_void;

// C forward declarations.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qcom_tzmem_pool {
    _private: [u8; 0],
}

/**
 * enum qcom_tzmem_policy - Policy for pool growth.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_tzmem_policy {
    /** Static pool, never grow above initial size. */
    QCOM_TZMEM_POLICY_STATIC = 1,
    /** When out of memory, add increment * current size of memory. */
    QCOM_TZMEM_POLICY_MULTIPLIER,
    /** When out of memory add as much as is needed until max_size. */
    QCOM_TZMEM_POLICY_ON_DEMAND,
}

/** TZ memory pool configuration. */
#[repr(C)]
pub struct qcom_tzmem_pool_config {
    /** Number of bytes to allocate for the pool during its creation. */
    pub initial_size: usize,
    /** Pool size growth policy. */
    pub policy: qcom_tzmem_policy,
    /** Used with policies that allow pool growth. */
    pub increment: usize,
    /** Size above which the pool will never grow. */
    pub max_size: usize,
}

extern "C" {
    pub fn qcom_tzmem_pool_new(
        config: *const qcom_tzmem_pool_config,
    ) -> *mut qcom_tzmem_pool;
    pub fn qcom_tzmem_pool_free(pool: *mut qcom_tzmem_pool);
    pub fn devm_qcom_tzmem_pool_new(
        dev: *mut device,
        config: *const qcom_tzmem_pool_config,
    ) -> *mut qcom_tzmem_pool;

    pub fn qcom_tzmem_alloc(
        pool: *mut qcom_tzmem_pool,
        size: usize,
        gfp: gfp_t,
    ) -> *mut c_void;
    pub fn qcom_tzmem_free(ptr: *mut c_void);

    pub fn qcom_tzmem_to_phys(ptr: *mut c_void) -> phys_addr_t;
}

// `gfp_t` and `phys_addr_t` are supplied by the translated Linux type layer.
extern "C" {
    // These declarations intentionally refer to externally supplied C types.
    // They preserve the source header's dependency on linux/types.h.
}

/// Cleanup equivalent of `DEFINE_FREE(qcom_tzmem, void *, if (_T) qcom_tzmem_free(_T))`.
#[inline]
pub unsafe fn qcom_tzmem_cleanup(ptr: *mut c_void) {
    if !ptr.is_null() {
        qcom_tzmem_free(ptr);
    }
}

// CONFIG_QCOM_TZMEM_MODE_SHMBRIDGE is a build-time kernel configuration.
#[cfg(feature = "CONFIG_QCOM_TZMEM_MODE_SHMBRIDGE")]
extern "C" {
    pub fn qcom_tzmem_shm_bridge_create(
        paddr: phys_addr_t,
        size: usize,
        handle: *mut u64,
    ) -> i32;
    pub fn qcom_tzmem_shm_bridge_delete(handle: u64);
}

#[cfg(not(feature = "CONFIG_QCOM_TZMEM_MODE_SHMBRIDGE"))]
#[inline]
pub unsafe fn qcom_tzmem_shm_bridge_create(
    _paddr: phys_addr_t,
    _size: usize,
    _handle: *mut u64,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_QCOM_TZMEM_MODE_SHMBRIDGE"))]
#[inline]
pub unsafe fn qcom_tzmem_shm_bridge_delete(_handle: u64) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
