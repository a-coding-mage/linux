/* SPDX-License-Identifier: MIT */
/* Copyright (C) 2025 Arm, Ltd. */

// Dependency: names supplied by the DRM header are intentionally external.

/** IOCTL IDs. New ioctls must be appended, not reordered, replaced, or removed. */
#[repr(u32)]
pub enum drm_ethosu_ioctl_id {
    DRM_ETHOSU_DEV_QUERY = 0,
    DRM_ETHOSU_BO_CREATE,
    DRM_ETHOSU_BO_WAIT,
    DRM_ETHOSU_BO_MMAP_OFFSET,
    DRM_ETHOSU_CMDSTREAM_BO_CREATE,
    DRM_ETHOSU_SUBMIT,
    DRM_ETHOSU_PERFMON_CREATE,
    DRM_ETHOSU_PERFMON_DESTROY,
    DRM_ETHOSU_PERFMON_GET_VALUES,
    DRM_ETHOSU_PERFMON_SET_GLOBAL,
}

#[repr(u32)]
pub enum drm_ethosu_dev_query_type {
    DRM_ETHOSU_DEV_QUERY_NPU_INFO = 0,
}

#[repr(C)]
pub struct drm_ethosu_npu_info {
    pub id: __u32,
    pub config: __u32,
    pub sram_size: __u32,
    pub pmu_counters: __u32,
}

#[inline]
pub const fn DRM_ETHOSU_ARCH_MAJOR(x: __u32) -> __u32 { x >> 28 }
#[inline]
pub const fn DRM_ETHOSU_ARCH_MINOR(x: __u32) -> __u32 { (x >> 20) & 0xff }
#[inline]
pub const fn DRM_ETHOSU_ARCH_PATCH(x: __u32) -> __u32 { (x >> 16) & 0xf }
#[inline]
pub const fn DRM_ETHOSU_PRODUCT_MAJOR(x: __u32) -> __u32 { (x >> 12) & 0xf }
#[inline]
pub const fn DRM_ETHOSU_VERSION_MAJOR(x: __u32) -> __u32 { (x >> 8) & 0xf }
#[inline]
pub const fn DRM_ETHOSU_VERSION_MINOR(x: __u32) -> __u32 { (x >> 4) & 0xff }
#[inline]
pub const fn DRM_ETHOSU_VERSION_STATUS(x: __u32) -> __u32 { x & 0xf }

#[repr(C)]
pub struct drm_ethosu_dev_query {
    pub r#type: __u32,
    pub size: __u32,
    pub pointer: __u64,
}

pub const DRM_ETHOSU_BO_NO_MMAP: __u32 = 1 << 0;

#[repr(C)]
pub struct drm_ethosu_bo_create {
    pub size: __u64,
    pub flags: __u32,
    pub handle: __u32,
}

#[repr(C)]
pub struct drm_ethosu_bo_mmap_offset {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
}

#[repr(C)]
pub struct drm_ethosu_bo_wait {
    pub handle: __u32,
    pub pad: __u32,
    pub timeout_ns: __s64, // absolute
}

#[repr(C)]
pub struct drm_ethosu_cmdstream_bo_create {
    pub size: __u32,
    pub flags: __u32,
    pub data: __u64,
    pub handle: __u32,
    pub pad: __u32,
}

pub const ETHOSU_MAX_REGIONS: usize = 8;

#[repr(C)]
pub struct drm_ethosu_job {
    pub cmd_bo: __u32,
    pub sram_size: __u32,
    pub region_bo_handles: [__u32; ETHOSU_MAX_REGIONS],
}

#[repr(C)]
pub struct drm_ethosu_submit {
    pub jobs: __u64,
    pub job_count: __u32,
    pub perfmon_id: __u32,
}

pub const DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS: usize = 8;
pub const DRM_ETHOSU_MAX_PERF_COUNTERS: usize = DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS + 1;

#[repr(C)]
pub struct drm_ethosu_perfmon_create {
    pub id: __u32,
    pub ncounters: __u32,
    pub counters: [__u16; DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS],
}

#[repr(C)]
pub struct drm_ethosu_perfmon_destroy {
    pub id: __u32,
    pub pad: __u32,
}

#[repr(C)]
pub struct drm_ethosu_perfmon_get_values {
    pub id: __u32,
    pub pad: __u32,
    pub values_ptr: __u64,
}

pub const DRM_ETHOSU_PERFMON_CLEAR_GLOBAL: __u32 = 0x0001;

#[repr(C)]
pub struct drm_ethosu_perfmon_set_global {
    pub flags: __u32,
    pub id: __u32,
}

// Equivalent of DRM_IOCTL_ETHOSU(); DRM_IOWR and DRM_COMMAND_BASE are supplied by drm.h.
#[macro_export]
macro_rules! DRM_IOCTL_ETHOSU {
    ($access:ident, $id:ident, $ty:ident) => {
        DRM_IOWR!(DRM_COMMAND_BASE + concat_idents!(DRM_ETHOSU_, $id), drm_ethosu_$ty)
    };
}

pub const DRM_IOCTL_ETHOSU_DEV_QUERY: _ = DRM_IOCTL_ETHOSU!(WR, DEV_QUERY, dev_query);
pub const DRM_IOCTL_ETHOSU_BO_CREATE: _ = DRM_IOCTL_ETHOSU!(WR, BO_CREATE, bo_create);
pub const DRM_IOCTL_ETHOSU_BO_WAIT: _ = DRM_IOCTL_ETHOSU!(WR, BO_WAIT, bo_wait);
pub const DRM_IOCTL_ETHOSU_BO_MMAP_OFFSET: _ = DRM_IOCTL_ETHOSU!(WR, BO_MMAP_OFFSET, bo_mmap_offset);
pub const DRM_IOCTL_ETHOSU_CMDSTREAM_BO_CREATE: _ = DRM_IOCTL_ETHOSU!(WR, CMDSTREAM_BO_CREATE, cmdstream_bo_create);
pub const DRM_IOCTL_ETHOSU_SUBMIT: _ = DRM_IOCTL_ETHOSU!(WR, SUBMIT, submit);
pub const DRM_IOCTL_ETHOSU_PERFMON_CREATE: _ = DRM_IOCTL_ETHOSU!(WR, PERFMON_CREATE, perfmon_create);
pub const DRM_IOCTL_ETHOSU_PERFMON_DESTROY: _ = DRM_IOCTL_ETHOSU!(WR, PERFMON_DESTROY, perfmon_destroy);
pub const DRM_IOCTL_ETHOSU_PERFMON_GET_VALUES: _ = DRM_IOCTL_ETHOSU!(WR, PERFMON_GET_VALUES, perfmon_get_values);
pub const DRM_IOCTL_ETHOSU_PERFMON_SET_GLOBAL: _ = DRM_IOCTL_ETHOSU!(WR, PERFMON_SET_GLOBAL, perfmon_set_global);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
