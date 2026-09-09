/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2020-2026 Intel Corporation */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const DRIVER_NAME: &str = "intel_vpu";
pub const DRIVER_DESC: &str = "Driver for Intel NPU (Neural Processing Unit)";

pub const PCI_DEVICE_ID_MTL: u16 = 0x7d1d;
pub const PCI_DEVICE_ID_ARL: u16 = 0xad1d;
pub const PCI_DEVICE_ID_LNL: u16 = 0x643e;
pub const PCI_DEVICE_ID_PTL_P: u16 = 0xb03e;
pub const PCI_DEVICE_ID_WCL: u16 = 0xfd3e;
pub const PCI_DEVICE_ID_NVL: u16 = 0xd71d;

pub const IVPU_HW_IP_37XX: i32 = 37;
pub const IVPU_HW_IP_40XX: i32 = 40;
pub const IVPU_HW_IP_50XX: i32 = 50;
pub const IVPU_HW_IP_60XX: i32 = 60;
pub const IVPU_HW_IP_REV_LNL_B0: i32 = 4;
pub const IVPU_HW_IP_REV_NVL_A0: i32 = 0;
pub const IVPU_HW_BTRS_MTL: i32 = 1;
pub const IVPU_HW_BTRS_LNL: i32 = 2;

pub const IVPU_GLOBAL_CONTEXT_MMU_SSID: u32 = 0;
// SSID 1 is used by the VPU to represent reserved context
pub const IVPU_RESERVED_CONTEXT_MMU_SSID: u32 = 1;
pub const IVPU_USER_CONTEXT_MIN_SSID: u32 = 2;
pub const IVPU_USER_CONTEXT_MAX_SSID: u32 = IVPU_USER_CONTEXT_MIN_SSID + 128;
pub const IVPU_MIN_DB: u32 = 1;
pub const IVPU_MAX_DB: u32 = 255;
pub const IVPU_JOB_ID_JOB_MASK: u32 = 0xff;
pub const IVPU_JOB_ID_CONTEXT_MASK: u32 = 0xffffff00;
pub const IVPU_CMDQ_MIN_ID: u32 = 1;
pub const IVPU_CMDQ_MAX_ID: u32 = 255;
pub const IVPU_PLATFORM_SILICON: u32 = 0;
pub const IVPU_PLATFORM_SIMICS: u32 = 2;
pub const IVPU_PLATFORM_FPGA: u32 = 3;
pub const IVPU_PLATFORM_HSLE: u32 = 4;
pub const IVPU_PLATFORM_INVALID: u32 = 8;
pub const IVPU_SCHED_MODE_AUTO: i32 = -1;

pub const IVPU_DBG_REG: i32 = 1 << 0;
pub const IVPU_DBG_IRQ: i32 = 1 << 1;
pub const IVPU_DBG_MMU: i32 = 1 << 2;
pub const IVPU_DBG_FILE: i32 = 1 << 3;
pub const IVPU_DBG_MISC: i32 = 1 << 4;
pub const IVPU_DBG_FW_BOOT: i32 = 1 << 5;
pub const IVPU_DBG_PM: i32 = 1 << 6;
pub const IVPU_DBG_IPC: i32 = 1 << 7;
pub const IVPU_DBG_BO: i32 = 1 << 8;
pub const IVPU_DBG_JOB: i32 = 1 << 9;
pub const IVPU_DBG_JSM: i32 = 1 << 10;
pub const IVPU_DBG_KREF: i32 = 1 << 11;
pub const IVPU_DBG_RPM: i32 = 1 << 12;
pub const IVPU_DBG_MMU_MAP: i32 = 1 << 13;
pub const IVPU_DBG_IOCTL: i32 = 1 << 14;

#[repr(C)]
pub struct ivpu_wa_table {
    pub punit_disabled: bool,
    pub clear_runtime_mem: bool,
    pub interrupt_clear_with_0: bool,
    pub disable_clock_relinquish: bool,
    pub wp0_during_power_up: bool,
    pub disable_d0i2: bool,
}

#[repr(C)]
pub struct ivpu_user_limits {
    pub hash_node: hlist_node,
    pub vdev: *mut ivpu_device,
    pub ref_: kref,
    pub max_ctx_count: u32,
    pub max_db_count: u32,
    pub uid: u32,
    pub db_count: atomic_t,
}

#[repr(C)]
pub struct ivpu_device {
    pub drm: drm_device,
    pub regb: *mut core::ffi::c_void,
    pub regv: *mut core::ffi::c_void,
    pub platform: u32,
    pub irq: u32,
    pub wa: ivpu_wa_table,
    pub hw: *mut ivpu_hw_info,
    pub mmu: *mut ivpu_mmu_info,
    pub fw: *mut ivpu_fw_info,
    pub ipc: *mut ivpu_ipc_info,
    pub pm: *mut ivpu_pm_info,
    pub gctx: ivpu_mmu_context,
    pub rctx: ivpu_mmu_context,
    pub context_list_lock: mutex,
    pub context_xa: xarray,
    pub context_xa_limit: xa_limit,
    pub user_limits: [u64; 1 << 8],
    pub user_limits_lock: mutex,
    pub db_xa: xarray,
    pub db_limit: xa_limit,
    pub db_next: u32,
    pub irq_dct_work: work_struct,
    pub context_abort_work: work_struct,
    pub job_destroy_list: llist_head,
    pub job_destroy_work: work_struct,
    pub job_destroy_wq: *mut workqueue_struct,
    pub bo_list_lock: mutex,
    pub bo_list: list_head,
    pub submitted_jobs_lock: mutex,
    pub submitted_jobs_xa: xarray,
    pub job_done_consumer: ivpu_ipc_consumer,
    pub job_timeout_counter: atomic_t,
    pub faults_detected: atomic_t,
    pub unique_id_counter: atomic64_t,
    pub busy_start_ts: ktime_t,
    pub busy_time: ktime_t,
    pub timeout: ivpu_timeout,
}

#[repr(C)]
pub struct ivpu_timeout {
    pub boot: i32, pub jsm: i32, pub tdr: i32, pub inference: i32,
    pub autosuspend: i32, pub d0i3_entry_msg: i32, pub state_dump_msg: i32,
}

#[repr(C)]
pub struct ivpu_file_priv {
    pub ref_: kref,
    pub vdev: *mut ivpu_device,
    pub lock: mutex,
    pub cmdq_xa: xarray,
    pub ctx: ivpu_mmu_context,
    pub ms_lock: mutex,
    pub ms_instance_list: list_head,
    pub ms_info_bo: *mut ivpu_bo,
    pub job_limit: xa_limit,
    pub user_limits: *mut ivpu_user_limits,
    pub job_id_next: u32,
    pub cmdq_limit: xa_limit,
    pub cmdq_id_next: u32,
    pub has_mmu_faults: bool,
    pub bound: bool,
    pub aborted: bool,
}

pub static mut ivpu_dbg_mask: i32 = 0;
pub static mut ivpu_pll_min_ratio: u8 = 0;
pub static mut ivpu_pll_max_ratio: u8 = 0;
pub static mut ivpu_sched_mode: i32 = 0;
pub static mut ivpu_disable_mmu_cont_pages: bool = false;
pub static mut ivpu_force_snoop: bool = false;

pub const IVPU_TEST_MODE_FW_TEST: i32 = 1 << 0;
pub const IVPU_TEST_MODE_NULL_HW: i32 = 1 << 1;
pub const IVPU_TEST_MODE_NULL_SUBMISSION: i32 = 1 << 2;
pub const IVPU_TEST_MODE_MIP_DISABLE: i32 = 1 << 6;
pub const IVPU_TEST_MODE_DISABLE_TIMEOUTS: i32 = 1 << 8;
pub const IVPU_TEST_MODE_TURBO_ENABLE: i32 = 1 << 9;
pub const IVPU_TEST_MODE_TURBO_DISABLE: i32 = 1 << 10;
pub const IVPU_TEST_MODE_CLK_RELINQ_DISABLE: i32 = 1 << 11;
pub const IVPU_TEST_MODE_CLK_RELINQ_ENABLE: i32 = 1 << 12;
pub const IVPU_TEST_MODE_D0I2_DISABLE: i32 = 1 << 13;
pub static mut ivpu_test_mode: i32 = 0;

unsafe extern "C" {
    pub fn ivpu_file_priv_get(file_priv: *mut ivpu_file_priv) -> *mut ivpu_file_priv;
    pub fn ivpu_file_priv_put(link: *mut *mut ivpu_file_priv);
    pub fn ivpu_boot(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_shutdown(vdev: *mut ivpu_device) -> i32;
    pub fn ivpu_prepare_for_reset(vdev: *mut ivpu_device);
    pub fn ivpu_is_capable(vdev: *mut ivpu_device, capability: u32) -> bool;
}

#[inline]
pub unsafe fn ivpu_hw_ip_gen(vdev: *mut ivpu_device) -> i32 {
    match ivpu_device_id(vdev) {
        PCI_DEVICE_ID_MTL | PCI_DEVICE_ID_ARL => IVPU_HW_IP_37XX,
        PCI_DEVICE_ID_LNL => IVPU_HW_IP_40XX,
        PCI_DEVICE_ID_PTL_P | PCI_DEVICE_ID_WCL => IVPU_HW_IP_50XX,
        PCI_DEVICE_ID_NVL => IVPU_HW_IP_60XX,
        _ => 0,
    }
}

#[inline]
pub unsafe fn ivpu_hw_btrs_gen(vdev: *mut ivpu_device) -> i32 {
    match ivpu_device_id(vdev) {
        PCI_DEVICE_ID_MTL | PCI_DEVICE_ID_ARL => IVPU_HW_BTRS_MTL,
        PCI_DEVICE_ID_LNL | PCI_DEVICE_ID_PTL_P | PCI_DEVICE_ID_WCL | PCI_DEVICE_ID_NVL => IVPU_HW_BTRS_LNL,
        _ => 0,
    }
}

#[inline]
pub unsafe fn ivpu_get_context_count(vdev: *mut ivpu_device) -> u32 {
    (*vdev).context_xa_limit.max.wrapping_sub((*vdev).context_xa_limit.min).wrapping_add(1)
}

#[inline]
pub unsafe fn ivpu_get_doorbell_count(vdev: *mut ivpu_device) -> u32 {
    (*vdev).db_limit.max.wrapping_sub((*vdev).db_limit.min).wrapping_add(1)
}

#[inline]
pub unsafe fn ivpu_get_platform(vdev: *mut ivpu_device) -> u32 { (*vdev).platform }
#[inline]
pub unsafe fn ivpu_is_silicon(vdev: *mut ivpu_device) -> bool { ivpu_get_platform(vdev) == IVPU_PLATFORM_SILICON }
#[inline]
pub unsafe fn ivpu_is_simics(vdev: *mut ivpu_device) -> bool { ivpu_get_platform(vdev) == IVPU_PLATFORM_SIMICS }
#[inline]
pub unsafe fn ivpu_is_fpga(vdev: *mut ivpu_device) -> bool {
    ivpu_get_platform(vdev) == IVPU_PLATFORM_FPGA || ivpu_get_platform(vdev) == IVPU_PLATFORM_HSLE
}
#[inline]
pub unsafe fn ivpu_is_force_snoop_enabled(_vdev: *mut ivpu_device) -> bool { ivpu_force_snoop }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
