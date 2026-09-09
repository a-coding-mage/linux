/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2025 Intel Corporation
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel/driver translation.
#[repr(C)] pub struct vpu_job_queue { _private: [u8; 0] }
#[repr(C)] pub struct ivpu_bo { _private: [u8; 0] }
#[repr(C)] pub struct ivpu_device { _private: [u8; 0] }
#[repr(C)] pub struct ivpu_file_priv { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct llist_node { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_file { _private: [u8; 0] }

/**
 * struct ivpu_cmdq - Represents a command queue for submitting jobs to the VPU.
 * Tracks queue memory, preemption buffers, and metadata for job management.
 */
#[repr(C)]
pub struct ivpu_cmdq {
    pub jobq: *mut vpu_job_queue,
    pub primary_preempt_buf: *mut ivpu_bo,
    pub secondary_preempt_buf: *mut ivpu_bo,
    pub mem: *mut ivpu_bo,
    pub entry_count: u32,
    pub id: u32,
    pub db_id: u32,
    pub priority: u8,
    pub is_legacy: bool,
}

/**
 * struct ivpu_job - Representing a batch or DMA buffer submitted to the VPU.
 * Each job is a unit of execution, tracked by job_id for status reporting from VPU FW.
 * The structure holds all resources and metadata needed for job submission, execution,
 * and completion handling.
 */
#[repr(C)]
pub struct ivpu_job {
    pub vdev: *mut ivpu_device,
    pub file_priv: *mut ivpu_file_priv,
    pub done_fence: *mut dma_fence,
    pub destroy_node: llist_node,
    pub cmd_buf_vpu_addr: u64,
    pub cmdq_id: u32,
    pub job_id: u32,
    pub engine_idx: u32,
    pub job_status: u32,
    pub primary_preempt_buf: *mut ivpu_bo,
    pub secondary_preempt_buf: *mut ivpu_bo,
    pub bo_count: usize,
    // Flexible array member: struct ivpu_bo *bos[] __counted_by(bo_count)
    pub bos: [*mut ivpu_bo; 0],
}

extern "C" {
    pub fn ivpu_submit_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> i32;
    pub fn ivpu_cmdq_create_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> i32;
    pub fn ivpu_cmdq_destroy_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> i32;
    pub fn ivpu_cmdq_submit_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> i32;

    pub fn ivpu_context_abort_locked(file_priv: *mut ivpu_file_priv);

    pub fn ivpu_cmdq_release_all_locked(file_priv: *mut ivpu_file_priv);
    pub fn ivpu_cmdq_reset_all_contexts(vdev: *mut ivpu_device);
    pub fn ivpu_cmdq_abort_all_jobs(vdev: *mut ivpu_device, ctx_id: u32, cmdq_id: u32);

    pub fn ivpu_job_done_consumer_init(vdev: *mut ivpu_device);
    pub fn ivpu_job_done_consumer_fini(vdev: *mut ivpu_device);
    pub fn ivpu_job_handle_engine_error(vdev: *mut ivpu_device, job_id: u32, job_status: u32) -> bool;
    pub fn ivpu_context_abort_work_fn(work: *mut work_struct);
    pub fn ivpu_job_destroy_work_fn(work: *mut work_struct);

    pub fn ivpu_jobs_abort_all(vdev: *mut ivpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
