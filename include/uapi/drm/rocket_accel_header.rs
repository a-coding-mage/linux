/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2024 Tomeu Vizoso
 */

// Dependency: drm.h supplies DRM_COMMAND_BASE and the DRM_IOW/DRM_IOWR
// ioctl encoding macros used below.

pub const DRM_ROCKET_CREATE_BO: u32 = 0x00;
pub const DRM_ROCKET_SUBMIT: u32 = 0x01;
pub const DRM_ROCKET_PREP_BO: u32 = 0x02;
pub const DRM_ROCKET_FINI_BO: u32 = 0x03;

pub const DRM_IOCTL_ROCKET_CREATE_BO: _ = DRM_IOWR!(DRM_COMMAND_BASE + DRM_ROCKET_CREATE_BO, drm_rocket_create_bo);
pub const DRM_IOCTL_ROCKET_SUBMIT: _ = DRM_IOW!(DRM_COMMAND_BASE + DRM_ROCKET_SUBMIT, drm_rocket_submit);
pub const DRM_IOCTL_ROCKET_PREP_BO: _ = DRM_IOW!(DRM_COMMAND_BASE + DRM_ROCKET_PREP_BO, drm_rocket_prep_bo);
pub const DRM_IOCTL_ROCKET_FINI_BO: _ = DRM_IOW!(DRM_COMMAND_BASE + DRM_ROCKET_FINI_BO, drm_rocket_fini_bo);

/**
 * struct drm_rocket_create_bo - ioctl argument for creating Rocket BOs.
 */
#[repr(C)]
pub struct drm_rocket_create_bo {
    /// @size: Input: Size of the requested BO.
    pub size: u32,
    /// @handle: Output: GEM handle for the BO.
    pub handle: u32,
    /// @dma_address: Output: DMA address for the BO in the NPU address
    /// space. This address is private to the DRM fd and is valid for
    /// the lifetime of the GEM handle.
    pub dma_address: u64,
    /// @offset: Output: Offset into the drm node to use for subsequent
    /// mmap call.
    pub offset: u64,
}

/**
 * struct drm_rocket_prep_bo - ioctl argument for starting CPU ownership of the BO.
 *
 * Takes care of waiting for any NPU jobs that might still use the NPU and performs cache
 * synchronization.
 */
#[repr(C)]
pub struct drm_rocket_prep_bo {
    /// @handle: Input: GEM handle of the buffer object.
    pub handle: u32,
    /// @reserved: Reserved, must be zero.
    pub reserved: u32,
    /// @timeout_ns: Input: Amount of time to wait for NPU jobs.
    pub timeout_ns: i64,
}

/**
 * struct drm_rocket_fini_bo - ioctl argument for finishing CPU ownership of the BO.
 *
 * Synchronize caches for NPU access.
 */
#[repr(C)]
pub struct drm_rocket_fini_bo {
    /// @handle: Input: GEM handle of the buffer object.
    pub handle: u32,
    /// @reserved: Reserved, must be zero.
    pub reserved: u32,
}

/**
 * struct drm_rocket_task - A task to be run on the NPU
 *
 * A task is the smallest unit of work that can be run on the NPU.
 */
#[repr(C)]
pub struct drm_rocket_task {
    /// @regcmd: Input: DMA address to NPU mapping of register command buffer
    pub regcmd: u32,
    /// @regcmd_count: Input: Number of commands in the register command
    /// buffer
    pub regcmd_count: u32,
}

/**
 * struct drm_rocket_job - A job to be run on the NPU
 *
 * The kernel will schedule the execution of this job taking into account its
 * dependencies with other jobs. All tasks in the same job will be executed
 * sequentially on the same core, to benefit from memory residency in SRAM.
 */
#[repr(C)]
pub struct drm_rocket_job {
    /// @tasks: Input: Pointer to an array of struct drm_rocket_task.
    pub tasks: u64,
    /// @in_bo_handles: Input: Pointer to a u32 array of the BOs that
    /// are read by the job.
    pub in_bo_handles: u64,
    /// @out_bo_handles: Input: Pointer to a u32 array of the BOs that
    /// are written to by the job.
    pub out_bo_handles: u64,
    /// @task_count: Input: Number of tasks passed in.
    pub task_count: u32,
    /// @task_struct_size: Input: Size in bytes of the structs in the
    /// @tasks field.
    pub task_struct_size: u32,
    /// @in_bo_handle_count: Input: Number of input BO handles passed in
    /// (size is that times 4).
    pub in_bo_handle_count: u32,
    /// @out_bo_handle_count: Input: Number of output BO handles passed in
    /// (size is that times 4).
    pub out_bo_handle_count: u32,
}

/**
 * struct drm_rocket_submit - ioctl argument for submitting commands to the NPU.
 *
 * The kernel will schedule the execution of these jobs in dependency order.
 */
#[repr(C)]
pub struct drm_rocket_submit {
    /// @jobs: Input: Pointer to an array of struct drm_rocket_job.
    pub jobs: u64,
    /// @job_count: Input: Number of jobs passed in.
    pub job_count: u32,
    /// @job_struct_size: Input: Size in bytes of the structs in the
    /// @jobs field.
    pub job_struct_size: u32,
    /// @reserved: Reserved, must be zero.
    pub reserved: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
