/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */
/* Copyright 2025 Arm, Ltd. */

/* Declarations supplied by the surrounding kernel/driver dependencies. */
#[repr(C)]
pub struct ethosu_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ethosu_file_priv {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_sched_job {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_gem_object {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ethosu_perfmon {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dma_fence {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct kref {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct drm_file {
    _opaque: [u8; 0],
}

/* Supplied by the NPU dependency. */
pub const NPU_BASEP_REGION_MAX: usize = 0; // TODO: use the dependency-provided value.

#[repr(C)]
pub struct ethosu_job {
    pub base: drm_sched_job,
    pub dev: *mut ethosu_device,

    pub cmd_bo: *mut drm_gem_object,
    pub region_bo: [*mut drm_gem_object; NPU_BASEP_REGION_MAX],
    pub region_bo_num: [u8; NPU_BASEP_REGION_MAX],
    pub region_cnt: u8,
    pub sram_size: u32,

    pub perfmon: *mut ethosu_perfmon,

    /* Fence to be signaled by drm-sched once its done with the job */
    pub inference_done_fence: *mut dma_fence,

    /* Fence to be signaled by IRQ handler when the job is complete. */
    pub done_fence: *mut dma_fence,

    pub refcount: kref,
}

unsafe extern "C" {
    pub fn ethosu_ioctl_submit(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file: *mut drm_file,
    ) -> core::ffi::c_int;

    pub fn ethosu_job_init(dev: *mut ethosu_device) -> core::ffi::c_int;
    pub fn ethosu_job_fini(dev: *mut ethosu_device);
    pub fn ethosu_job_open(ethosu_priv: *mut ethosu_file_priv) -> core::ffi::c_int;
    pub fn ethosu_job_close(ethosu_priv: *mut ethosu_file_priv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
