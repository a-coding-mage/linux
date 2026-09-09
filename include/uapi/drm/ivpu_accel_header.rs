/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/* Copyright (C) 2020-2025 Intel Corporation */

// Translated from uapi/drm/ivpu_accel.h. The DRM ioctl encoding helpers are
// supplied by the corresponding DRM bindings.

pub const DRM_IVPU_GET_PARAM: u32 = 0x00;
pub const DRM_IVPU_SET_PARAM: u32 = 0x01;
pub const DRM_IVPU_BO_CREATE: u32 = 0x02;
pub const DRM_IVPU_BO_INFO: u32 = 0x03;
pub const DRM_IVPU_SUBMIT: u32 = 0x05;
pub const DRM_IVPU_BO_WAIT: u32 = 0x06;
pub const DRM_IVPU_METRIC_STREAMER_START: u32 = 0x07;
pub const DRM_IVPU_METRIC_STREAMER_STOP: u32 = 0x08;
pub const DRM_IVPU_METRIC_STREAMER_GET_DATA: u32 = 0x09;
pub const DRM_IVPU_METRIC_STREAMER_GET_INFO: u32 = 0x0a;
pub const DRM_IVPU_CMDQ_CREATE: u32 = 0x0b;
pub const DRM_IVPU_CMDQ_DESTROY: u32 = 0x0c;
pub const DRM_IVPU_CMDQ_SUBMIT: u32 = 0x0d;
pub const DRM_IVPU_BO_CREATE_FROM_USERPTR: u32 = 0x0e;

// DRM_IOCTL_IVPU_* are encoded with DRM_IOW/DRM_IOWR using the structs below.
// (The C preprocessor expressions are retained here as dependency comments.)
// DRM_IOWR(DRM_COMMAND_BASE + DRM_IVPU_GET_PARAM, drm_ivpu_param)
// DRM_IOW(DRM_COMMAND_BASE + DRM_IVPU_SET_PARAM, drm_ivpu_param)
// DRM_IOWR(DRM_COMMAND_BASE + DRM_IVPU_BO_CREATE, drm_ivpu_bo_create)
// DRM_IOWR(DRM_COMMAND_BASE + DRM_IVPU_BO_INFO, drm_ivpu_bo_info)
// DRM_IOW(DRM_COMMAND_BASE + DRM_IVPU_SUBMIT, drm_ivpu_submit)
// DRM_IOWR(DRM_COMMAND_BASE + DRM_IVPU_BO_WAIT, drm_ivpu_bo_wait)
// DRM_IOWR/DRM_IOW metric streamer and command queue ioctl encodings use the
// matching struct and command number above.

pub const DRM_IVPU_PARAM_DEVICE_ID: u32 = 0;
pub const DRM_IVPU_PARAM_DEVICE_REVISION: u32 = 1;
pub const DRM_IVPU_PARAM_PLATFORM_TYPE: u32 = 2;
pub const DRM_IVPU_PARAM_CORE_CLOCK_RATE: u32 = 3;
pub const DRM_IVPU_PARAM_NUM_CONTEXTS: u32 = 4;
pub const DRM_IVPU_PARAM_CONTEXT_BASE_ADDRESS: u32 = 5;
pub const DRM_IVPU_PARAM_CONTEXT_PRIORITY: u32 = 6; // Deprecated
pub const DRM_IVPU_PARAM_CONTEXT_ID: u32 = 7;
pub const DRM_IVPU_PARAM_FW_API_VERSION: u32 = 8;
pub const DRM_IVPU_PARAM_ENGINE_HEARTBEAT: u32 = 9;
pub const DRM_IVPU_PARAM_UNIQUE_INFERENCE_ID: u32 = 10;
pub const DRM_IVPU_PARAM_TILE_CONFIG: u32 = 11;
pub const DRM_IVPU_PARAM_SKU: u32 = 12;
pub const DRM_IVPU_PARAM_CAPABILITIES: u32 = 13;
pub const DRM_IVPU_PARAM_PREEMPT_BUFFER_SIZE: u32 = 14;
pub const DRM_IVPU_PLATFORM_TYPE_SILICON: u32 = 0;

pub const DRM_IVPU_CONTEXT_PRIORITY_IDLE: u32 = 0;
pub const DRM_IVPU_CONTEXT_PRIORITY_NORMAL: u32 = 1;
pub const DRM_IVPU_CONTEXT_PRIORITY_FOCUS: u32 = 2;
pub const DRM_IVPU_CONTEXT_PRIORITY_REALTIME: u32 = 3;
pub const DRM_IVPU_JOB_PRIORITY_DEFAULT: u32 = 0;
pub const DRM_IVPU_JOB_PRIORITY_IDLE: u32 = 1;
pub const DRM_IVPU_JOB_PRIORITY_NORMAL: u32 = 2;
pub const DRM_IVPU_JOB_PRIORITY_FOCUS: u32 = 3;
pub const DRM_IVPU_JOB_PRIORITY_REALTIME: u32 = 4;

pub const DRM_IVPU_CAP_METRIC_STREAMER: u32 = 1;
pub const DRM_IVPU_CAP_DMA_MEMORY_RANGE: u32 = 2;
pub const DRM_IVPU_CAP_MANAGE_CMDQ: u32 = 3;
pub const DRM_IVPU_CAP_BO_CREATE_FROM_USERPTR: u32 = 4;

#[repr(C)]
pub struct drm_ivpu_param { pub param: u32, pub index: u32, pub value: u64 }

pub const DRM_IVPU_BO_SHAVE_MEM: u32 = 0x00000001;
pub const DRM_IVPU_BO_HIGH_MEM: u32 = DRM_IVPU_BO_SHAVE_MEM;
pub const DRM_IVPU_BO_MAPPABLE: u32 = 0x00000002;
pub const DRM_IVPU_BO_DMA_MEM: u32 = 0x00000004;
pub const DRM_IVPU_BO_READ_ONLY: u32 = 0x00000008;
pub const DRM_IVPU_BO_CACHED: u32 = 0x00000000;
pub const DRM_IVPU_BO_UNCACHED: u32 = 0x00010000;
pub const DRM_IVPU_BO_WC: u32 = 0x00020000;
pub const DRM_IVPU_BO_CACHE_MASK: u32 = 0x00030000;
pub const DRM_IVPU_BO_FLAGS: u32 = DRM_IVPU_BO_HIGH_MEM | DRM_IVPU_BO_MAPPABLE |
    DRM_IVPU_BO_DMA_MEM | DRM_IVPU_BO_READ_ONLY | DRM_IVPU_BO_CACHE_MASK;

#[repr(C)]
pub struct drm_ivpu_bo_create { pub size: u64, pub flags: u32, pub handle: u32, pub vpu_addr: u64 }
#[repr(C)]
pub struct drm_ivpu_bo_create_from_userptr { pub user_ptr: u64, pub size: u64, pub flags: u32, pub handle: u32, pub vpu_addr: u64 }
#[repr(C)]
pub struct drm_ivpu_bo_info { pub handle: u32, pub flags: u32, pub vpu_addr: u64, pub mmap_offset: u64, pub size: u64 }

pub const DRM_IVPU_ENGINE_COMPUTE: u32 = 0;
pub const DRM_IVPU_ENGINE_COPY: u32 = 1;
#[repr(C)]
pub struct drm_ivpu_submit { pub buffers_ptr: u64, pub buffer_count: u32, pub engine: u32, pub flags: u32, pub commands_offset: u32, pub priority: u32 }
#[repr(C)]
pub struct drm_ivpu_cmdq_submit { pub buffers_ptr: u64, pub buffer_count: u32, pub cmdq_id: u32, pub flags: u32, pub commands_offset: u32, pub preempt_buffer_index: u32, pub reserved: u32 }

pub const DRM_IVPU_JOB_STATUS_SUCCESS: u32 = 0;
pub const DRM_IVPU_JOB_STATUS_ABORTED: u32 = 256;
#[repr(C)]
pub struct drm_ivpu_bo_wait { pub handle: u32, pub flags: u32, pub timeout_ns: i64, pub job_status: u32, pub pad: u32 }

#[repr(C)]
pub struct drm_ivpu_metric_streamer_start { pub metric_group_mask: u64, pub sampling_period_ns: u64, pub read_period_samples: u32, pub sample_size: u32, pub max_data_size: u32 }
#[repr(C)]
pub struct drm_ivpu_metric_streamer_get_data { pub metric_group_mask: u64, pub buffer_ptr: u64, pub buffer_size: u64, pub data_size: u64 }

pub const DRM_IVPU_CMDQ_FLAG_TURBO: u32 = 0x00000001;
#[repr(C)]
pub struct drm_ivpu_cmdq_create { pub cmdq_id: u32, pub priority: u32, pub flags: u32 }
#[repr(C)]
pub struct drm_ivpu_cmdq_destroy { pub cmdq_id: u32 }
#[repr(C)]
pub struct drm_ivpu_metric_streamer_stop { pub metric_group_mask: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
