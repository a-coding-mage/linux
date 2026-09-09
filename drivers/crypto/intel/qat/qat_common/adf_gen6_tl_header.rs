/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2025 Intel Corporation. */

// #include <linux/types.h>

pub struct adf_tl_hw_data;

/* Computation constants. */
pub const ADF_GEN6_CPP_NS_PER_CYCLE: u32 = 2;
pub const ADF_GEN6_TL_BW_HW_UNITS_TO_BYTES: u32 = 64;

/* Maximum aggregation time. Value is in milliseconds. */
pub const ADF_GEN6_TL_MAX_AGGR_TIME_MS: u32 = 4000;
/* Number of buffers to store historic values. */
pub const ADF_GEN6_TL_NUM_HIST_BUFFS: u32 =
    ADF_GEN6_TL_MAX_AGGR_TIME_MS / ADF_TL_DATA_WR_INTERVAL_MS;

/* Max number of HW resources of one type */
pub const ADF_GEN6_TL_MAX_SLICES_PER_TYPE: u32 = 32;
pub const MAX_ATH_SL_COUNT: usize = 7;
pub const MAX_CNV_SL_COUNT: usize = 2;
pub const MAX_DCPRZ_SL_COUNT: usize = 2;
pub const MAX_PKE_SL_COUNT: usize = 32;
pub const MAX_UCS_SL_COUNT: usize = 4;
pub const MAX_WAT_SL_COUNT: usize = 5;
pub const MAX_WCP_SL_COUNT: usize = 5;

pub const MAX_ATH_CMDQ_COUNT: usize = 14;
pub const MAX_CNV_CMDQ_COUNT: usize = 6;
pub const MAX_DCPRZ_CMDQ_COUNT: usize = 6;
pub const MAX_PKE_CMDQ_COUNT: usize = 32;
pub const MAX_UCS_CMDQ_COUNT: usize = 12;
pub const MAX_WAT_CMDQ_COUNT: usize = 35;
pub const MAX_WCP_CMDQ_COUNT: usize = 35;

/* Max number of simultaneously monitored ring pairs. */
pub const ADF_GEN6_TL_MAX_RP_NUM: usize = 4;

/**
 * struct adf_gen6_tl_slice_data_regs - HW slice data as populated by FW.
 * @reg_tm_slice_exec_cnt: Slice execution count.
 * @reg_tm_slice_util: Slice utilization.
 */
#[repr(C)]
pub struct adf_gen6_tl_slice_data_regs {
    pub reg_tm_slice_exec_cnt: u32,
    pub reg_tm_slice_util: u32,
}

pub const ADF_GEN6_TL_SLICE_REG_SZ: usize = core::mem::size_of::<adf_gen6_tl_slice_data_regs>();

/**
 * struct adf_gen6_tl_cmdq_data_regs - HW CMDQ data as populated by FW.
 * @reg_tm_cmdq_wait_cnt: CMDQ wait count.
 * @reg_tm_cmdq_exec_cnt: CMDQ execution count.
 * @reg_tm_cmdq_drain_cnt: CMDQ drain count.
 */
#[repr(C)]
pub struct adf_gen6_tl_cmdq_data_regs {
    pub reg_tm_cmdq_wait_cnt: u32,
    pub reg_tm_cmdq_exec_cnt: u32,
    pub reg_tm_cmdq_drain_cnt: u32,
    pub reserved: u32,
}

pub const ADF_GEN6_TL_CMDQ_REG_SZ: usize = core::mem::size_of::<adf_gen6_tl_cmdq_data_regs>();

/**
 * struct adf_gen6_tl_device_data_regs - This structure stores device telemetry
 * counter values as are being populated periodically by device.
 */
#[repr(C)]
pub struct adf_gen6_tl_device_data_regs {
    pub reg_tl_rd_lat_acc: u64,
    pub reg_tl_gp_lat_acc: u64,
    pub reg_tl_at_page_req_lat_acc: u64,
    pub reg_tl_at_trans_lat_acc: u64,
    pub reg_tl_re_acc: u64,
    pub reg_tl_prt_trans_cnt: u32,
    pub reg_tl_rd_lat_max: u32,
    pub reg_tl_rd_cmpl_cnt: u32,
    pub reg_tl_gp_lat_max: u32,
    pub reg_tl_ae_put_cnt: u32,
    pub reg_tl_bw_in: u32,
    pub reg_tl_bw_out: u32,
    pub reg_tl_at_page_req_cnt: u32,
    pub reg_tl_at_trans_lat_cnt: u32,
    pub reg_tl_at_max_utlb_used: u32,
    pub reg_tl_re_cnt: u32,
    pub reserved: u32,
    pub ath_slices: [adf_gen6_tl_slice_data_regs; MAX_ATH_SL_COUNT],
    pub cnv_slices: [adf_gen6_tl_slice_data_regs; MAX_CNV_SL_COUNT],
    pub dcprz_slices: [adf_gen6_tl_slice_data_regs; MAX_DCPRZ_SL_COUNT],
    pub pke_slices: [adf_gen6_tl_slice_data_regs; MAX_PKE_SL_COUNT],
    pub ucs_slices: [adf_gen6_tl_slice_data_regs; MAX_UCS_SL_COUNT],
    pub wat_slices: [adf_gen6_tl_slice_data_regs; MAX_WAT_SL_COUNT],
    pub wcp_slices: [adf_gen6_tl_slice_data_regs; MAX_WCP_SL_COUNT],
    pub ath_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_ATH_CMDQ_COUNT],
    pub cnv_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_CNV_CMDQ_COUNT],
    pub dcprz_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_DCPRZ_CMDQ_COUNT],
    pub pke_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_PKE_CMDQ_COUNT],
    pub ucs_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_UCS_CMDQ_COUNT],
    pub wat_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_WAT_CMDQ_COUNT],
    pub wcp_cmdq: [adf_gen6_tl_cmdq_data_regs; MAX_WCP_CMDQ_COUNT],
}

#[repr(C)]
pub struct adf_gen6_tl_ring_pair_data_regs {
    pub reg_tl_gp_lat_acc: u64,
    pub reg_tl_re_acc: u64,
    pub reg_tl_prt_trans_cnt: u32,
    pub reg_tl_ae_put_cnt: u32,
    pub reg_tl_bw_in: u32,
    pub reg_tl_bw_out: u32,
    pub reg_tl_at_glob_devtlb_hit: u32,
    pub reg_tl_at_glob_devtlb_miss: u32,
    pub reg_tl_at_payld_devtlb_hit: u32,
    pub reg_tl_at_payld_devtlb_miss: u32,
    pub reg_tl_re_cnt: u32,
    pub reserved1: u32,
}

pub const ADF_GEN6_TL_RP_REG_SZ: usize = core::mem::size_of::<adf_gen6_tl_ring_pair_data_regs>();

#[repr(C)]
pub struct adf_gen6_tl_layout {
    pub tl_device_data_regs: adf_gen6_tl_device_data_regs,
    pub tl_ring_pairs_data_regs: [adf_gen6_tl_ring_pair_data_regs; ADF_GEN6_TL_MAX_RP_NUM],
    pub reg_tl_msg_cnt: u32,
    pub reserved: u32,
}

pub const ADF_GEN6_TL_LAYOUT_SZ: usize = core::mem::size_of::<adf_gen6_tl_layout>();
pub const ADF_GEN6_TL_MSG_CNT_OFF: usize = core::mem::offset_of!(adf_gen6_tl_layout, reg_tl_msg_cnt);

// Under CONFIG_DEBUG_FS, this function is externally defined.
#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn adf_gen6_init_tl_data(tl_data: *mut adf_tl_hw_data);
}

// Without CONFIG_DEBUG_FS, the C inline function is an empty operation.
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
pub unsafe fn adf_gen6_init_tl_data(_tl_data: *mut adf_tl_hw_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
