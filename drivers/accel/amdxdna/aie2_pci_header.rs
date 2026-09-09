/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023-2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding translation unit:
// drm/amdxdna_accel.h, linux/limits.h, linux/semaphore.h, aie.h,
// aie2_msg_priv.h, and amdxdna_mailbox.h.

/* Firmware determines device memory base address and size */
pub const AIE2_DEVM_BASE: u32 = 0x4000000;
pub const AIE2_DEVM_SIZE: usize = 64 * 1024 * 1024;
pub const AIE2_DEVM_MAX_SIZE: usize = 512 * 1024 * 1024;

pub const CHAN_SLOT_SZ: usize = 8 * 1024;

#[inline]
pub unsafe fn aie2_sram_off(ndev: *mut amdxdna_dev_hdl, addr: u64) -> u64 { addr.wrapping_sub((*ndev).priv_.as_ref().unwrap().sram_dev_addr as u64) }
#[inline]
pub unsafe fn aie2_mbox_off(ndev: *mut amdxdna_dev_hdl, addr: u64) -> u64 { addr.wrapping_sub((*ndev).priv_.as_ref().unwrap().mbox_dev_addr as u64) }
// SRAM_REG_OFF, SRAM_GET_ADDR, NDEV2PDEV, and MBOX_SIZE retain their C macro
// semantics and depend on the surrounding kernel/device definitions.

pub const HWCTX_MAX_CMDS: u64 = 4;

#[inline]
pub const fn get_job_idx(seq: u64) -> u64 {
    seq & (HWCTX_MAX_CMDS - 1)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum aie2_sram_reg_idx {
    MBOX_CHANN_OFF = 0,
    FW_ALIVE_OFF,
    SRAM_MAX_INDEX, /* Keep this at the end */
}

pub enum amdxdna_client {}
pub enum amdxdna_fw_ver {}
pub enum amdxdna_hwctx {}
pub enum amdxdna_sched_job {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rt_config_category {
    AIE2_RT_CFG_INIT,
    AIE2_RT_CFG_CLK_GATING,
    AIE2_RT_CFG_FORCE_PREEMPT,
    AIE2_RT_CFG_FRAME_BOUNDARY_PREEMPT,
}

#[repr(C)]
pub struct rt_config {
    pub type_: u32,
    pub value: u32,
    pub category: u32,
    pub feature_mask: usize,
}

#[repr(C)]
pub struct dpm_clk_freq {
    pub npuclk: u32,
    pub hclk: u32,
}

#[repr(C)]
pub struct amdxdna_hwctx_priv {
    pub heap: *mut amdxdna_gem_obj,
    pub mbox_chann: *mut core::ffi::c_void,
    pub sched: drm_gpu_scheduler,
    pub entity: drm_sched_entity,
    pub io_lock: mutex,
    pub job_free_wq: wait_queue_head,
    pub num_pending: u32,
    pub seq: u64,
    pub job_sem: semaphore,
    pub job_done: bool,
    /* Completed job counter */
    pub completed: u64,
    pub cmd_buf: [*mut amdxdna_gem_obj; HWCTX_MAX_CMDS as usize],
    pub syncobj: *mut drm_syncobj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum aie2_dev_status {
    AIE2_DEV_UNINIT,
    AIE2_DEV_INIT,
    AIE2_DEV_START,
}

#[repr(C)]
pub struct aie2_exec_msg_ops {
    pub init_cu_req: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize, *mut u32) -> i32>,
    pub init_dpu_req: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize, *mut u32) -> i32>,
    pub init_chain_req: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u64, usize, u32)>,
    pub fill_cf_slot: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize) -> i32>,
    pub fill_dpu_slot: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize) -> i32>,
    pub fill_preempt_slot: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize) -> i32>,
    pub fill_elf_slot: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, *mut core::ffi::c_void, *mut usize) -> i32>,
    pub get_chain_msg_op: Option<unsafe extern "C" fn(u32) -> u32>,
}

#[repr(C)]
pub struct amdxdna_dev_hdl {
    pub aie: aie_device,
    pub priv_: *const amdxdna_dev_priv,
    pub sram_base: *mut core::ffi::c_void,
    pub mbox_base: *mut core::ffi::c_void,
    pub total_col: u32,
    pub version: amdxdna_drm_query_aie_version,
    pub exec_msg_ops: *mut aie2_exec_msg_ops,
    /* power management and clock*/
    pub pw_mode: amdxdna_power_mode_type,
    pub dpm_level: u32,
    pub dft_dpm_level: u32,
    pub max_dpm_level: u32,
    pub clk_gating: u32,
    pub npuclk_freq: u32,
    pub hclk_freq: u32,
    pub max_tops: u32,
    pub curr_tops: u32,
    pub force_preempt_enabled: u32,
    pub frame_boundary_preempt: u32,
    /* Mailbox and the management channel */
    pub mbox: *mut mailbox,
    pub async_events: *mut async_events,
    pub dev_status: aie2_dev_status,
    pub hwctx_num: u32,
    pub last_async_err: amdxdna_async_error,
    pub last_signal_ts: usize,
}

#[repr(C)]
pub struct aie2_hw_ops {
    pub set_dpm: Option<unsafe extern "C" fn(*mut amdxdna_dev_hdl, u32) -> i32>,
    pub update_counters: Option<unsafe extern "C" fn(*mut amdxdna_dev_hdl) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum aie2_fw_feature {
    AIE2_NPU_COMMAND,
    AIE2_PREEMPT,
    AIE2_TEMPORAL_ONLY,
    AIE2_APP_HEALTH,
    AIE2_ADD_HOST_BUFFER,
    AIE2_UPDATE_PROPERTY,
    AIE2_GET_DEV_REVISION,
    AIE2_FEATURE_MAX,
}

pub const AIE2_ALL_FEATURES: u64 = (1u64 << (aie2_fw_feature::AIE2_FEATURE_MAX as u32)) - 1;

pub const COL_ALIGN_NONE: u32 = 0;
pub const COL_ALIGN_NATURE: u32 = 1;

#[repr(C)]
pub struct amdxdna_dev_priv {
    pub fw_path: *const i8,
    pub rt_config: *const rt_config,
    pub dpm_clk_tbl: *const dpm_clk_freq,
    pub col_align: u32,
    pub col_opc: u32,
    pub mbox_dev_addr: u32,
    /* If mbox_size is 0, use BAR size. See MBOX_SIZE macro */
    pub mbox_size: u32,
    pub hwctx_limit: u32,
    pub sram_dev_addr: u32,
    pub sram_offs: [aie_bar_off_pair; SRAM_MAX_INDEX as usize],
    pub psp_regs_off: [aie_bar_off_pair; PSP_MAX_REGS as usize],
    pub smu_regs_off: [aie_bar_off_pair; SMU_MAX_REGS as usize],
    pub hw_ops: *const aie2_hw_ops,
}

extern "C" {
    pub static aie2_ops: amdxdna_dev_ops;
    pub static npu1_dpm_clk_table: dpm_clk_freq;
    pub static npu4_dpm_clk_table: dpm_clk_freq;
    pub static npu1_default_rt_cfg: rt_config;
    pub static npu4_default_rt_cfg: rt_config;
    pub static npu4_fw_feature_table: amdxdna_fw_feature_tbl;
    pub static npu4_rev_vbnv_tbl: amdxdna_rev_vbnv;
    pub static npu4_hw_ops: aie2_hw_ops;

    pub fn aie2_runtime_cfg(ndev: *mut amdxdna_dev_hdl, category: rt_config_category, val: *mut u32) -> i32;
    pub fn aie2_pm_init(ndev: *mut amdxdna_dev_hdl) -> i32;
    pub fn aie2_pm_set_mode(ndev: *mut amdxdna_dev_hdl, target: amdxdna_power_mode_type) -> i32;
    pub fn aie2_pm_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> i32;
    pub fn aie2_error_async_events_alloc(ndev: *mut amdxdna_dev_hdl) -> i32;
    pub fn aie2_error_async_events_free(ndev: *mut amdxdna_dev_hdl);
    pub fn aie2_error_async_msg_thread(data: *mut core::ffi::c_void) -> i32;
    pub fn aie2_get_array_async_error(ndev: *mut amdxdna_dev_hdl, args: *mut amdxdna_drm_get_array) -> i32;
    pub fn aie2_msg_init(ndev: *mut amdxdna_dev_hdl);
    pub fn aie2_destroy_mgmt_chann(ndev: *mut amdxdna_dev_hdl);
    pub fn aie2_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> i32;
    pub fn aie2_resume_fw(ndev: *mut amdxdna_dev_hdl) -> i32;
    pub fn aie2_set_runtime_cfg(ndev: *mut amdxdna_dev_hdl, type_: u32, value: u64) -> i32;
    pub fn aie2_get_runtime_cfg(ndev: *mut amdxdna_dev_hdl, type_: u32, value: *mut u64) -> i32;
    pub fn aie2_assign_mgmt_pasid(ndev: *mut amdxdna_dev_hdl, pasid: u16) -> i32;
    pub fn aie2_query_aie_version(ndev: *mut amdxdna_dev_hdl, version: *mut amdxdna_drm_query_aie_version) -> i32;
    pub fn aie2_query_aie_metadata(ndev: *mut amdxdna_dev_hdl, metadata: *mut amdxdna_drm_query_aie_metadata) -> i32;
    pub fn aie2_query_firmware_version(ndev: *mut amdxdna_dev_hdl, fw_ver: *mut amdxdna_fw_ver) -> i32;
    pub fn aie2_query_app_health(ndev: *mut amdxdna_dev_hdl, context_id: u32, report: *mut app_health_report) -> i32;
    pub fn aie2_get_dev_revision(ndev: *mut amdxdna_dev_hdl, rev: *mut aie2_dev_revision) -> i32;
    pub fn aie2_create_context(ndev: *mut amdxdna_dev_hdl, hwctx: *mut amdxdna_hwctx) -> i32;
    pub fn aie2_destroy_context(ndev: *mut amdxdna_dev_hdl, hwctx: *mut amdxdna_hwctx) -> i32;
    pub fn aie2_map_host_buf(ndev: *mut amdxdna_dev_hdl, context_id: u32, addr: u64, size: u64) -> i32;
    pub fn aie2_add_host_buf(ndev: *mut amdxdna_dev_hdl, context_id: u32, addr: u64, size: u64) -> i32;
    pub fn aie2_query_status(ndev: *mut amdxdna_dev_hdl, buf: *mut i8, size: u32, cols_filled: *mut u32) -> i32;
    pub fn aie2_query_telemetry(ndev: *mut amdxdna_dev_hdl, buf: *mut i8, size: u32, header: *mut amdxdna_drm_query_telemetry_header) -> i32;
    pub fn aie2_register_asyn_event_msg(ndev: *mut amdxdna_dev_hdl, addr: dma_addr_t, size: u32, handle: *mut core::ffi::c_void, cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_config_cu(hwctx: *mut amdxdna_hwctx, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_execbuf(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_cmdlist_single_execbuf(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_cmdlist_multi_execbuf(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_sync_bo(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_config_debug_bo(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, notify_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> i32>) -> i32;
    pub fn aie2_update_prop_time_quota(ndev: *mut amdxdna_dev_hdl, us: u32) -> i32;
    pub fn aie2_hwctx_init(hwctx: *mut amdxdna_hwctx) -> i32;
    pub fn aie2_hwctx_fini(hwctx: *mut amdxdna_hwctx);
    pub fn aie2_hwctx_config(hwctx: *mut amdxdna_hwctx, type_: u32, value: u64, buf: *mut core::ffi::c_void, size: u32) -> i32;
    pub fn aie2_hwctx_sync_debug_bo(hwctx: *mut amdxdna_hwctx, debug_bo_hdl: u32) -> i32;
    pub fn aie2_hwctx_suspend(client: *mut amdxdna_client);
    pub fn aie2_hwctx_resume(client: *mut amdxdna_client) -> i32;
    pub fn aie2_cmd_submit(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, seq: *mut u64) -> i32;
    pub fn aie2_hmm_invalidate(abo: *mut amdxdna_gem_obj, cur_seq: usize);
    pub fn aie2_hwctx_heap_expand(hwctx: *mut amdxdna_hwctx, heap: *mut amdxdna_gem_obj) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
