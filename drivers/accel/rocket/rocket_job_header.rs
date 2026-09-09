/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependencies supplied by the corresponding DRM and Rocket headers.

#[repr(C)]
pub struct drm_sched_job {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_gem_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rocket_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rocket_core {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rocket_file_priv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rocket_iommu_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rocket_task {
    pub regcmd: u64,
    pub regcmd_count: u32,
}

#[repr(C)]
pub struct rocket_job {
    pub base: drm_sched_job,

    pub rdev: *mut rocket_device,

    pub in_bos: *mut *mut drm_gem_object,
    pub out_bos: *mut *mut drm_gem_object,

    pub in_bo_count: u32,
    pub out_bo_count: u32,

    pub tasks: *mut rocket_task,
    pub task_count: u32,
    pub next_task_idx: u32,

    /* Fence to be signaled by drm-sched once its done with the job */
    pub inference_done_fence: *mut dma_fence,

    /* Fence to be signaled by IRQ handler when the job is complete. */
    pub done_fence: *mut dma_fence,

    pub domain: *mut rocket_iommu_domain,

    pub refcount: kref,
}

unsafe extern "C" {
    pub fn rocket_ioctl_submit(
        dev: *mut drm_device,
        data: *mut core::ffi::c_void,
        file: *mut drm_file,
    ) -> i32;

    pub fn rocket_job_init(core: *mut rocket_core) -> i32;
    pub fn rocket_job_fini(core: *mut rocket_core);
    pub fn rocket_job_open(rocket_priv: *mut rocket_file_priv) -> i32;
    pub fn rocket_job_close(rocket_priv: *mut rocket_file_priv);
    pub fn rocket_job_is_idle(core: *mut rocket_core) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
