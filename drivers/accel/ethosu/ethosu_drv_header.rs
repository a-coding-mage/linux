/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/* Copyright 2025-2026 Arm, Ltd. */

// The C header depends on Linux kernel and DRM declarations supplied by other
// translation units.  Those names are intentionally referenced here rather
// than reimplemented.

pub struct ethosu_device;
pub struct drm_device;
pub struct drm_file;

#[repr(C)]
pub struct ethosu_file_priv {
    pub edev: *mut ethosu_device,
    pub sched_entity: drm_sched_entity,
    pub perfmons: xarray,
}

/* Performance monitor object. The perfmon lifetime is controlled by userspace
 * using perfmon related ioctls. A perfmon can be attached to a DRM_ETHOSU_SUBMIT
 * request, and when this is the case, HW perf counters will be activated just
 * before the job is submitted to the NPU and disabled when the job is done.
 * This way, only events related to a specific job will be counted.
 */
#[repr(C)]
pub struct ethosu_perfmon {
    /* Tracks the number of users of the perfmon, when this counter reaches
     * zero the perfmon is destroyed.
     */
    pub refcnt: refcount_t,

    /* Number of counters activated in this perfmon instance
     * (should be less than or equal to DRM_ETHOSU_MAX_PERF_COUNTERS).
     */
    pub ncounters: u8,

    /* Events counted by the HW perf counters. */
    pub counters: [u16; DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS],

    /*
     * Storage for counter values. Counters are incremented by the HW
     * perf counter values every time the perfmon is attached to an
     * NPU job. This way, perfmon users don't have to retrieve the
     * results after each job if they want to track events covering
     * several submissions. Note that counter values can't be reset,
     * but you can fake a reset by destroying the perfmon and
     * creating a new one.
     */
    // C flexible array member: values[] __counted_by(ncounters)
    pub values: [u64; 0],
}

/* ethosu_perfmon.c */
unsafe extern "C" {
    pub fn ethosu_perfmon_get(perfmon: *mut ethosu_perfmon);
    pub fn ethosu_perfmon_put(perfmon: *mut ethosu_perfmon);
    pub fn ethosu_perfmon_start(
        ethosu: *mut ethosu_device,
        perfmon: *mut ethosu_perfmon,
    );
    pub fn ethosu_perfmon_stop(
        ethosu: *mut ethosu_device,
        perfmon: *mut ethosu_perfmon,
        capture: bool,
    );
    pub fn ethosu_perfmon_stop_locked(
        ethosu: *mut ethosu_device,
        perfmon: *mut ethosu_perfmon,
        capture: bool,
    );
    pub fn ethosu_perfmon_find(
        ethosu_priv: *mut ethosu_file_priv,
        id: i32,
    ) -> *mut ethosu_perfmon;
    pub fn ethosu_perfmon_open_file(ethosu_priv: *mut ethosu_file_priv);
    pub fn ethosu_perfmon_close_file(ethosu_priv: *mut ethosu_file_priv);
    pub fn ethosu_ioctl_perfmon_create(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file_priv: *mut drm_file,
    ) -> i32;
    pub fn ethosu_ioctl_perfmon_destroy(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file_priv: *mut drm_file,
    ) -> i32;
    pub fn ethosu_ioctl_perfmon_get_values(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file_priv: *mut drm_file,
    ) -> i32;
    pub fn ethosu_ioctl_perfmon_set_global(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file_priv: *mut drm_file,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
