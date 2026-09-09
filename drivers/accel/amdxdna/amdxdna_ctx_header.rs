/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
 */

// Translated from amdxdna_ctx.h. C header dependencies are supplied externally.

#[repr(u32)]
pub enum ert_cmd_opcode {
    ERT_START_CU = 0,
    ERT_START_DPU = 18,
    ERT_CMD_CHAIN = 19,
    ERT_START_NPU = 20,
    ERT_START_NPU_PREEMPT = 21,
    ERT_START_NPU_PREEMPT_ELF = 22,
    ERT_INVALID_CMD = u32::MAX,
}

#[repr(u32)]
pub enum ert_cmd_state {
    ERT_CMD_STATE_INVALID,
    ERT_CMD_STATE_NEW,
    ERT_CMD_STATE_QUEUED,
    ERT_CMD_STATE_RUNNING,
    ERT_CMD_STATE_COMPLETED,
    ERT_CMD_STATE_ERROR,
    ERT_CMD_STATE_ABORT,
    ERT_CMD_STATE_SUBMITTED,
    ERT_CMD_STATE_TIMEOUT,
    ERT_CMD_STATE_NORESPONSE,
}

/* Interpretation of the beginning of data payload for ERT_START_NPU. */
#[repr(C)]
pub struct amdxdna_cmd_start_npu {
    pub buffer: u64,
    pub buffer_size: u32,
    pub prop_count: u32,
    pub prop_args: [u32; 0],
}

/* Interpretation of the beginning of data payload for ERT_CMD_CHAIN. */
#[repr(C)]
pub struct amdxdna_cmd_chain {
    pub command_count: u32,
    pub submit_index: u32,
    pub error_index: u32,
    pub reserved: [u32; 3],
    pub data: [u64; 0],
}

/* Interpretation of the beginning of data payload for ERT_START_NPU_PREEMPT. */
#[repr(C)]
pub struct amdxdna_cmd_preempt_data {
    pub inst_buf: u64,
    pub save_buf: u64,
    pub restore_buf: u64,
    pub inst_size: u32,
    pub save_size: u32,
    pub restore_size: u32,
    pub inst_prop_cnt: u32,
    pub prop_args: [u32; 0],
}

pub const AMDXDNA_CMD_CTX_HEALTH_V1: u32 = 1;
pub const AMDXDNA_CMD_CTX_HEALTH_AIE2: u32 = 0;

#[repr(C)]
pub struct amdxdna_ctx_health {
    pub version: u32,
    pub npu_gen: u32,
}

/* Exec buffer command header format */
pub const AMDXDNA_CMD_STATE: u32 = 0x0000_000f;
pub const AMDXDNA_CMD_EXTRA_CU_MASK: u32 = 0x0000_0c00;
pub const AMDXDNA_CMD_COUNT: u32 = 0x007f_f000;
pub const AMDXDNA_CMD_OPCODE: u32 = 0x0f80_0000;

#[repr(C)]
pub struct amdxdna_cmd {
    pub header: u32,
    pub data: [u32; 0],
}

pub const INVALID_CU_IDX: u32 = u32::MAX;

#[repr(C)]
pub struct amdxdna_hwctx {
    pub client: *mut amdxdna_client,
    pub priv_: *mut amdxdna_hwctx_priv,
    pub name: *mut core::ffi::c_char,
    pub id: u32,
    pub max_opc: u32,
    pub num_tiles: u32,
    pub mem_size: u32,
    pub fw_ctx_id: u32,
    pub col_list_len: u32,
    pub col_list: *mut u32,
    pub start_col: u32,
    pub num_col: u32,
    pub umq_bo_hdl: u32,
    pub doorbell_offset: u32,
    pub num_unused_col: u32,
    pub last_attached_heap: u32,
    pub qos: amdxdna_qos_info,
    pub cus: *mut amdxdna_hwctx_param_config_cu,
    pub syncobj_hdl: u32,
    pub job_submit_cnt: atomic64_t,
    pub job_free_cnt: atomic64_t,
}

#[repr(u32)]
pub enum amdxdna_job_opcode {
    DEFAULT_IO,
    SYNC_DEBUG_BO,
    ATTACH_DEBUG_BO,
    DETACH_DEBUG_BO,
}

#[repr(C)]
pub struct amdxdna_drv_cmd {
    pub opcode: amdxdna_job_opcode,
    pub result: u32,
    pub refcnt: kref,
}

#[repr(C)]
pub union amdxdna_job_priv {
    pub aie2_health: *mut app_health_report,
}

#[repr(C)]
pub struct amdxdna_sched_job {
    pub base: drm_sched_job,
    pub refcnt: kref,
    pub hwctx: *mut amdxdna_hwctx,
    pub mm: *mut mm_struct,
    pub fence: *mut dma_fence,
    pub out_fence: *mut dma_fence,
    pub job_done: bool,
    pub job_timeout: bool,
    pub seq: u64,
    pub drv_cmd: *mut amdxdna_drv_cmd,
    pub cmd_bo: *mut amdxdna_gem_obj,
    pub priv_: amdxdna_job_priv,
    pub bo_cnt: usize,
    pub bos: [*mut drm_gem_object; 0],
}

// C macro: aie2_job_health expands to priv.aie2_health.

#[inline]
pub unsafe fn amdxdna_cmd_get_op(abo: *mut amdxdna_gem_obj) -> u32 {
    let cmd = amdxdna_gem_vmap(abo);
    if cmd.is_null() { return ert_cmd_opcode::ERT_INVALID_CMD as u32; }
    ((*cmd).header & AMDXDNA_CMD_OPCODE) >> 23
}

#[inline]
pub unsafe fn amdxdna_cmd_set_state(abo: *mut amdxdna_gem_obj, s: ert_cmd_state) {
    let cmd = amdxdna_gem_vmap(abo);
    if cmd.is_null() { return; }
    (*cmd).header = ((*cmd).header & !AMDXDNA_CMD_STATE) | ((s as u32) & AMDXDNA_CMD_STATE);
}

#[inline]
pub unsafe fn amdxdna_cmd_get_state(abo: *mut amdxdna_gem_obj) -> ert_cmd_state {
    let cmd = amdxdna_gem_vmap(abo);
    if cmd.is_null() { return ert_cmd_state::ERT_CMD_STATE_INVALID; }
    match (*cmd).header & AMDXDNA_CMD_STATE {
        0 => ert_cmd_state::ERT_CMD_STATE_INVALID,
        1 => ert_cmd_state::ERT_CMD_STATE_NEW,
        2 => ert_cmd_state::ERT_CMD_STATE_QUEUED,
        3 => ert_cmd_state::ERT_CMD_STATE_RUNNING,
        4 => ert_cmd_state::ERT_CMD_STATE_COMPLETED,
        5 => ert_cmd_state::ERT_CMD_STATE_ERROR,
        6 => ert_cmd_state::ERT_CMD_STATE_ABORT,
        7 => ert_cmd_state::ERT_CMD_STATE_SUBMITTED,
        8 => ert_cmd_state::ERT_CMD_STATE_TIMEOUT,
        9 => ert_cmd_state::ERT_CMD_STATE_NORESPONSE,
        _ => ert_cmd_state::ERT_CMD_STATE_INVALID,
    }
}

extern "C" {
    pub fn amdxdna_cmd_get_payload(abo: *mut amdxdna_gem_obj, size: *mut u32) -> *mut core::ffi::c_void;
    pub fn amdxdna_cmd_get_cu_idx(abo: *mut amdxdna_gem_obj) -> u32;
    pub fn amdxdna_cmd_set_error(abo: *mut amdxdna_gem_obj, job: *mut amdxdna_sched_job, cmd_idx: u32, error_state: ert_cmd_state, err_data: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn amdxdna_sched_job_cleanup(job: *mut amdxdna_sched_job);
    pub fn amdxdna_hwctx_remove_all(client: *mut amdxdna_client);
    pub fn amdxdna_hwctx_walk(client: *mut amdxdna_client, arg: *mut core::ffi::c_void, walk: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, *mut core::ffi::c_void) -> i32>) -> i32;
    pub fn amdxdna_hwctx_sync_debug_bo(client: *mut amdxdna_client, debug_bo_hdl: u32) -> i32;
    pub fn amdxdna_update_heap(client: *mut amdxdna_client, hwctx: *mut amdxdna_hwctx) -> i32;
    pub fn amdxdna_cmd_submit(client: *mut amdxdna_client, drv_cmd: *mut amdxdna_drv_cmd, cmd_bo_hdls: u32, arg_bo_hdls: *mut u32, arg_bo_cnt: u32, hwctx_hdl: u32, seq: *mut u64) -> i32;
    pub fn amdxdna_drm_create_hwctx_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdxdna_drm_config_hwctx_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdxdna_drm_destroy_hwctx_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdxdna_drm_submit_cmd_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdxdna_drm_wait_cmd_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
