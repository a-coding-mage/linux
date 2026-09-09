/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2023 Intel Corporation. */

/* Dependency supplied externally by the telemetry implementation. */
pub struct adf_tl_hw_data;

/* Computation constants. */
pub const ADF_GEN4_CPP_NS_PER_CYCLE: u32 = 2;
pub const ADF_GEN4_TL_BW_HW_UNITS_TO_BYTES: u32 = 64;

/* Maximum aggregation time. Value in milliseconds. */
pub const ADF_GEN4_TL_MAX_AGGR_TIME_MS: u32 = 4000;
/* Num of buffers to store historic values. */
pub const ADF_GEN4_TL_NUM_HIST_BUFFS: u32 =
    ADF_GEN4_TL_MAX_AGGR_TIME_MS / ADF_TL_DATA_WR_INTERVAL_MS;

/* Max number of HW resources of one type. */
pub const ADF_GEN4_TL_MAX_SLICES_PER_TYPE: usize = 24;

/* Max number of simultaneously monitored ring pairs. */
pub const ADF_GEN4_TL_MAX_RP_NUM: usize = 4;

#[repr(C)]
pub struct adf_gen4_tl_slice_data_regs {
    pub reg_tm_slice_exec_cnt: u32,
    pub reg_tm_slice_util: u32,
}

pub const ADF_GEN4_TL_SLICE_REG_SZ: usize =
    core::mem::size_of::<adf_gen4_tl_slice_data_regs>();

#[repr(C)]
pub struct adf_gen4_tl_device_data_regs {
    pub reg_tl_rd_lat_acc: u64,
    pub reg_tl_gp_lat_acc: u64,
    pub reg_tl_at_page_req_lat_acc: u64,
    pub reg_tl_at_trans_lat_acc: u64,
    pub reg_tl_re_acc: u64,
    pub reg_tl_pci_trans_cnt: u32,
    pub reg_tl_rd_lat_max: u32,
    pub reg_tl_rd_cmpl_cnt: u32,
    pub reg_tl_gp_lat_max: u32,
    pub reg_tl_ae_put_cnt: u32,
    pub reg_tl_bw_in: u32,
    pub reg_tl_bw_out: u32,
    pub reg_tl_at_page_req_cnt: u32,
    pub reg_tl_at_trans_lat_cnt: u32,
    pub reg_tl_at_max_tlb_used: u32,
    pub reg_tl_re_cnt: u32,
    pub reserved: u32,
    pub ath_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub cph_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub cpr_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub xlt_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub dcpr_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub pke_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub ucs_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub wat_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
    pub wcp_slices: [adf_gen4_tl_slice_data_regs; ADF_GEN4_TL_MAX_SLICES_PER_TYPE],
}

#[repr(C)]
pub struct adf_gen4_tl_ring_pair_data_regs {
    pub reg_tl_gp_lat_acc: u64,
    pub reserved: u64,
    pub reg_tl_pci_trans_cnt: u32,
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

pub const ADF_GEN4_TL_RP_REG_SZ: usize =
    core::mem::size_of::<adf_gen4_tl_ring_pair_data_regs>();

#[repr(C)]
pub struct adf_gen4_tl_layout {
    pub tl_device_data_regs: adf_gen4_tl_device_data_regs,
    pub tl_ring_pairs_data_regs: [adf_gen4_tl_ring_pair_data_regs; ADF_GEN4_TL_MAX_RP_NUM],
    pub reg_tl_msg_cnt: u32,
    pub reserved: u32,
}

pub const ADF_GEN4_TL_LAYOUT_SZ: usize = core::mem::size_of::<adf_gen4_tl_layout>();
pub const ADF_GEN4_TL_MSG_CNT_OFF: usize =
    core::mem::offset_of!(adf_gen4_tl_layout, reg_tl_msg_cnt);

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn adf_gen4_init_tl_data(tl_data: *mut adf_tl_hw_data);
}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn adf_gen4_init_tl_data(_tl_data: *mut adf_tl_hw_data) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
