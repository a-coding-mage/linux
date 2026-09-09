/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
 *
 * Lockdep annotation interface for AMDGPU
 */

// Dependency intent: declarations supplied by the Linux lockdep interface.

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[cfg(CONFIG_LOCKDEP)]
extern "C" {
    /**
     * amdgpu_lockdep_init - Train lockdep on correct lock ordering
     *
     * Call once during module init to establish the lock dependency chain.
     */
    pub fn amdgpu_lockdep_init() -> ::core::ffi::c_int;

    /**
     * amdgpu_lockdep_set_class - Associate lock class keys with real locks
     * @adev: AMDGPU device
     *
     * Call during device init to associate lock classes with actual locks.
     */
    pub fn amdgpu_lockdep_set_class(adev: *mut amdgpu_device);
}

#[cfg(not(CONFIG_LOCKDEP))]
#[inline]
pub fn amdgpu_lockdep_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_LOCKDEP))]
#[inline]
pub fn amdgpu_lockdep_set_class(_adev: *mut amdgpu_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
