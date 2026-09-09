/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2023 Intel Corporation. */

// Dependencies supplied by the surrounding kernel translation.

pub const ADF_TL_SL_CNT_COUNT: usize =
    core::mem::size_of::<icp_qat_fw_init_admin_slice_cnt>() / core::mem::size_of::<u8>();

pub const TL_CAPABILITY_BIT: u32 = 1u32 << 1;
/* Interval within device writes data to DMA region. Value in milliseconds. */
pub const ADF_TL_DATA_WR_INTERVAL_MS: u32 = 1000;
/* Interval within timer interrupt should be handled. Value in milliseconds. */
pub const ADF_TL_TIMER_INT_MS: u32 = ADF_TL_DATA_WR_INTERVAL_MS / 2;

pub const ADF_TL_RP_REGS_DISABLED: u8 = 0xff;

#[repr(C)]
pub struct adf_tl_hw_data {
    pub layout_sz: usize,
    pub slice_reg_sz: usize,
    pub cmdq_reg_sz: usize,
    pub rp_reg_sz: usize,
    pub msg_cnt_off: usize,
    pub dev_counters: *const adf_tl_dbg_counter,
    pub sl_util_counters: *const adf_tl_dbg_counter,
    pub sl_exec_counters: *const adf_tl_dbg_counter,
    pub cmdq_counters: *const *const adf_tl_dbg_counter,
    pub rp_counters: *const adf_tl_dbg_counter,
    pub num_hbuff: u8,
    pub cpp_ns_per_cycle: u8,
    pub bw_units_to_bytes: u8,
    pub num_dev_counters: u8,
    pub num_rp_counters: u8,
    pub num_cmdq_counters: u8,
    pub max_rp: u8,
    pub max_sl_cnt: u8,
    pub multiplier: icp_qat_fw_init_admin_slice_cnt,
}

#[repr(C)]
pub struct adf_telemetry {
    pub accel_dev: *mut adf_accel_dev,
    pub state: atomic_t,
    pub hbuffs: u32,
    pub hb_num: i32,
    pub msg_cnt: u32,
    pub regs_data_p: dma_addr_t, /* bus address for DMA mapping */
    pub regs_data: *mut core::ffi::c_void, /* virtual address for DMA mapping */
    /**
     * @regs_hist_buff: array of pointers to copies of the last @hbuffs
     * values of @regs_data
     */
    pub regs_hist_buff: *mut *mut core::ffi::c_void,
    pub dbg_dir: *mut dentry,
    pub rp_num_indexes: *mut u8,
    /**
     * @regs_hist_lock: protects from race conditions between write and read
     * to the copies referenced by @regs_hist_buff
     */
    pub regs_hist_lock: mutex,
    /**
     * @wr_lock: protects from concurrent writes to debugfs telemetry files
     */
    pub wr_lock: mutex,
    pub work_ctx: delayed_work,
    pub slice_cnt: icp_qat_fw_init_admin_slice_cnt,
    pub cmdq_cnt: icp_qat_fw_init_admin_slice_cnt,
}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn adf_tl_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_tl_start(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_tl_stop(accel_dev: *mut adf_accel_dev);
    pub fn adf_tl_shutdown(accel_dev: *mut adf_accel_dev);
    pub fn adf_tl_run(accel_dev: *mut adf_accel_dev, state: i32) -> i32;
    pub fn adf_tl_halt(accel_dev: *mut adf_accel_dev) -> i32;
}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn adf_tl_init(_accel_dev: *mut adf_accel_dev) -> i32 { 0 }

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn adf_tl_start(_accel_dev: *mut adf_accel_dev) -> i32 { 0 }

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn adf_tl_stop(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn adf_tl_shutdown(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
