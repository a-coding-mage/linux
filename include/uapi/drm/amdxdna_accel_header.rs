/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (C) 2022-2024, Advanced Micro Devices, Inc. */

// Translated from uapi/drm/amdxdna_accel.h.
// Required external Linux/DRM definitions are intentionally left as dependencies.

pub const AMDXDNA_INVALID_CMD_HANDLE: ::core::ffi::c_ulong = !0;
pub const AMDXDNA_INVALID_ADDR: ::core::ffi::c_ulong = !0;
pub const AMDXDNA_INVALID_CTX_HANDLE: u32 = 0;
pub const AMDXDNA_INVALID_BO_HANDLE: u32 = 0;
pub const AMDXDNA_INVALID_FENCE_HANDLE: u32 = 0;
pub const AMDXDNA_INVALID_DOORBELL_OFFSET: u32 = !0;

pub const AMDXDNA_QOS_REALTIME_PRIORITY: u32 = 0x100;
pub const AMDXDNA_QOS_HIGH_PRIORITY: u32 = 0x180;
pub const AMDXDNA_QOS_NORMAL_PRIORITY: u32 = 0x200;
pub const AMDXDNA_QOS_LOW_PRIORITY: u32 = 0x280;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_device_type {
    AMDXDNA_DEV_TYPE_UNKNOWN = -1,
    AMDXDNA_DEV_TYPE_KMQ = 0,
    AMDXDNA_DEV_TYPE_UMQ = 1,
    AMDXDNA_DEV_TYPE_PF = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_drm_ioctl_id {
    DRM_AMDXDNA_CREATE_HWCTX = 0,
    DRM_AMDXDNA_DESTROY_HWCTX,
    DRM_AMDXDNA_CONFIG_HWCTX,
    DRM_AMDXDNA_CREATE_BO,
    DRM_AMDXDNA_GET_BO_INFO,
    DRM_AMDXDNA_SYNC_BO,
    DRM_AMDXDNA_EXEC_CMD,
    DRM_AMDXDNA_GET_INFO,
    DRM_AMDXDNA_SET_STATE,
    DRM_AMDXDNA_WAIT_CMD,
    DRM_AMDXDNA_GET_ARRAY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_qos_info { pub gops: u32, pub fps: u32, pub dma_bandwidth: u32, pub latency: u32, pub frame_exec_time: u32, pub priority: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_create_hwctx { pub ext: u64, pub ext_flags: u64, pub qos_p: u64, pub umq_bo: u32, pub log_buf_bo: u32, pub max_opc: u32, pub num_tiles: u32, pub mem_size: u32, pub umq_doorbell: u32, pub handle: u32, pub syncobj_handle: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_destroy_hwctx { pub handle: u32, pub pad: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cu_config { pub cu_bo: u32, pub cu_func: u8, pub pad: [u8; 3] }
#[repr(C)]
pub struct amdxdna_hwctx_param_config_cu { pub num_cus: u16, pub pad: [u16; 3], pub cu_configs: [amdxdna_cu_config; 0] }

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_drm_config_hwctx_param { DRM_AMDXDNA_HWCTX_CONFIG_CU = 0, DRM_AMDXDNA_HWCTX_ASSIGN_DBG_BUF, DRM_AMDXDNA_HWCTX_REMOVE_DBG_BUF }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_config_hwctx { pub handle: u32, pub param_type: u32, pub param_val: u64, pub param_val_size: u32, pub pad: u32 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_bo_type { AMDXDNA_BO_INVALID = 0, AMDXDNA_BO_SHMEM = 1, AMDXDNA_BO_SHARE = 1, AMDXDNA_BO_DEV_HEAP = 2, AMDXDNA_BO_DEV = 3, AMDXDNA_BO_CMD = 4 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_va_entry { pub vaddr: u64, pub len: u64 }
#[repr(C)]
pub struct amdxdna_drm_va_tbl { pub dmabuf_fd: i32, pub num_entries: u32, pub va_entries: [amdxdna_drm_va_entry; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_create_bo { pub flags: u64, pub vaddr: u64, pub size: u64, pub r#type: u32, pub handle: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_bo_info { pub ext: u64, pub ext_flags: u64, pub handle: u32, pub pad: u32, pub map_offset: u64, pub vaddr: u64, pub xdna_addr: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_sync_bo { pub handle: u32, pub direction: u32, pub offset: u64, pub size: u64 }
pub const SYNC_DIRECT_TO_DEVICE: u32 = 0;
pub const SYNC_DIRECT_FROM_DEVICE: u32 = 1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_cmd_type { AMDXDNA_CMD_SUBMIT_EXEC_BUF = 0, AMDXDNA_CMD_SUBMIT_DEPENDENCY, AMDXDNA_CMD_SUBMIT_SIGNAL }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_exec_cmd { pub ext: u64, pub ext_flags: u64, pub hwctx: u32, pub r#type: u32, pub cmd_handles: u64, pub args: u64, pub cmd_count: u32, pub arg_count: u32, pub seq: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_wait_cmd { pub hwctx: u32, pub timeout: u32, pub seq: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_status { pub buffer: u64, pub buffer_size: u32, pub cols_filled: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_version { pub major: u32, pub minor: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_tile_metadata { pub row_count: u16, pub row_start: u16, pub dma_channel_count: u16, pub lock_count: u16, pub event_reg_count: u16, pub pad: [u16; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_aie_metadata { pub col_size: u32, pub cols: u16, pub rows: u16, pub version: amdxdna_drm_query_aie_version, pub core: amdxdna_drm_query_aie_tile_metadata, pub mem: amdxdna_drm_query_aie_tile_metadata, pub shim: amdxdna_drm_query_aie_tile_metadata }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_clock { pub name: [u8; 16], pub freq_mhz: u32, pub pad: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_clock_metadata { pub mp_npu_clock: amdxdna_drm_query_clock, pub h_clock: amdxdna_drm_query_clock }

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_sensor_type { AMDXDNA_SENSOR_TYPE_POWER = 0, AMDXDNA_SENSOR_TYPE_COLUMN_UTILIZATION }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_sensor { pub label: [u8; 64], pub input: u32, pub max: u32, pub average: u32, pub highest: u32, pub status: [u8; 64], pub units: [u8; 16], pub unitm: i8, pub r#type: u8, pub pad: [u8; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_hwctx { pub context_id: u32, pub start_col: u32, pub num_col: u32, pub pad: u32, pub pid: i64, pub command_submissions: u64, pub command_completions: u64, pub migrations: u64, pub preemptions: u64, pub errors: u64 }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_power_mode_type { POWER_MODE_DEFAULT = 0, POWER_MODE_LOW, POWER_MODE_MEDIUM, POWER_MODE_HIGH, POWER_MODE_TURBO }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_power_mode { pub power_mode: u8, pub pad: [u8; 7] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_query_firmware_version { pub major: u32, pub minor: u32, pub patch: u32, pub build: u32 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_drm_get_param { DRM_AMDXDNA_QUERY_AIE_STATUS = 0, DRM_AMDXDNA_QUERY_AIE_METADATA, DRM_AMDXDNA_QUERY_AIE_VERSION, DRM_AMDXDNA_QUERY_CLOCK_METADATA, DRM_AMDXDNA_QUERY_SENSORS, DRM_AMDXDNA_QUERY_HW_CONTEXTS, DRM_AMDXDNA_QUERY_FIRMWARE_VERSION = 8, DRM_AMDXDNA_GET_POWER_MODE, DRM_AMDXDNA_QUERY_TELEMETRY, DRM_AMDXDNA_GET_FORCE_PREEMPT_STATE, DRM_AMDXDNA_QUERY_RESOURCE_INFO, DRM_AMDXDNA_GET_FRAME_BOUNDARY_PREEMPT_STATE }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_resource_info { pub npu_clk_max: u64, pub npu_tops_max: u64, pub npu_task_max: u64, pub npu_tops_curr: u64, pub npu_task_curr: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_attribute_state { pub state: u8, pub pad: [u8; 7] }
#[repr(C)]
pub struct amdxdna_drm_query_telemetry_header { pub major: u32, pub minor: u32, pub r#type: u32, pub map_num_elements: u32, pub map: [u32; 0] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_info { pub param: u32, pub buffer_size: u32, pub buffer: u64 }

pub const AMDXDNA_HWCTX_STATE_IDLE: u32 = 0;
pub const AMDXDNA_HWCTX_STATE_ACTIVE: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_hwctx_entry { pub context_id: u32, pub start_col: u32, pub num_col: u32, pub hwctx_id: u32, pub pid: i64, pub command_submissions: u64, pub command_completions: u64, pub migrations: u64, pub preemptions: u64, pub errors: u64, pub priority: u64, pub heap_usage: u64, pub suspensions: u64, pub state: u32, pub pasid: u32, pub gops: u32, pub fps: u32, pub dma_bandwidth: u32, pub latency: u32, pub frame_exec_time: u32, pub txn_op_idx: u32, pub ctx_pc: u32, pub fatal_error_type: u32, pub fatal_error_exception_type: u32, pub fatal_error_exception_pc: u32, pub fatal_error_app_module: u32, pub pad: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_async_error { pub err_code: u64, pub ts_us: u64, pub ex_err_code: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_bo_usage { pub pid: i64, pub total_usage: u64, pub internal_usage: u64, pub heap_usage: u64 }

pub const DRM_AMDXDNA_HW_CONTEXT_ALL: u32 = 0;
pub const DRM_AMDXDNA_HW_LAST_ASYNC_ERR: u32 = 2;
pub const DRM_AMDXDNA_BO_USAGE: u32 = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_get_array { pub param: u32, pub element_size: u32, pub num_element: u32, pub pad: u32, pub buffer: u64 }
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdxdna_drm_set_param { DRM_AMDXDNA_SET_POWER_MODE = 0, DRM_AMDXDNA_WRITE_AIE_MEM, DRM_AMDXDNA_WRITE_AIE_REG, DRM_AMDXDNA_SET_FORCE_PREEMPT, DRM_AMDXDNA_SET_FRAME_BOUNDARY_PREEMPT }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_set_state { pub param: u32, pub buffer_size: u32, pub buffer: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drm_set_power_mode { pub power_mode: u8, pub pad: [u8; 7] }

// DRM ioctl encodings depend on external DRM_COMMAND_BASE, DRM_IOWR, and DRM_IOW definitions.
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_CREATE_HWCTX { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_CREATE_HWCTX, amdxdna_drm_create_hwctx) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_DESTROY_HWCTX { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_DESTROY_HWCTX, amdxdna_drm_destroy_hwctx) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_CONFIG_HWCTX { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_CONFIG_HWCTX, amdxdna_drm_config_hwctx) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_CREATE_BO { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_CREATE_BO, amdxdna_drm_create_bo) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_GET_BO_INFO { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_GET_BO_INFO, amdxdna_drm_get_bo_info) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_SYNC_BO { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_SYNC_BO, amdxdna_drm_sync_bo) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_EXEC_CMD { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_EXEC_CMD, amdxdna_drm_exec_cmd) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_GET_INFO { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_GET_INFO, amdxdna_drm_get_info) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_SET_STATE { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_SET_STATE, amdxdna_drm_set_state) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_GET_ARRAY { () => { DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDXDNA_GET_ARRAY, amdxdna_drm_get_array) }; }
#[macro_export] macro_rules! DRM_IOCTL_AMDXDNA_WAIT_CMD { () => { DRM_IOW(DRM_COMMAND_BASE + DRM_AMDXDNA_WAIT_CMD, amdxdna_drm_wait_cmd) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
