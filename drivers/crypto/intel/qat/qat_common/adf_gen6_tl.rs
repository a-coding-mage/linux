// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Intel Corporation. */

// Dependencies supplied by the surrounding kernel translation.

macro_rules! ADF_GEN6_TL_DEV_REG_OFF { ($reg:ident) => { ADF_TL_DEV_REG_OFF!($reg, gen6) }; }
macro_rules! ADF_GEN6_TL_RP_REG_OFF { ($reg:ident) => { ADF_TL_RP_REG_OFF!($reg, gen6) }; }
macro_rules! ADF_GEN6_TL_SL_UTIL_COUNTER {
    ($name:ident) => { ADF_TL_COUNTER!(concat!("util_", stringify!($name)), ADF_TL_SIMPLE_COUNT, ADF_TL_SLICE_REG_OFF!($name, reg_tm_slice_util, gen6)) };
}
macro_rules! ADF_GEN6_TL_SL_EXEC_COUNTER {
    ($name:ident) => { ADF_TL_COUNTER!(concat!("exec_", stringify!($name)), ADF_TL_SIMPLE_COUNT, ADF_TL_SLICE_REG_OFF!($name, reg_tm_slice_exec_cnt, gen6)) };
}
macro_rules! SLICE_IDX { ($sl:ident) => { offset_of!(icp_qat_fw_init_admin_slice_cnt, $sl##_cnt) }; }
macro_rules! ADF_GEN6_TL_CMDQ_WAIT_COUNTER {
    ($name:ident) => { ADF_TL_COUNTER!(concat!("cmdq_wait_", stringify!($name)), ADF_TL_SIMPLE_COUNT, ADF_TL_CMDQ_REG_OFF!($name, reg_tm_cmdq_wait_cnt, gen6)) };
}
macro_rules! ADF_GEN6_TL_CMDQ_EXEC_COUNTER {
    ($name:ident) => { ADF_TL_COUNTER!(concat!("cmdq_exec_", stringify!($name)), ADF_TL_SIMPLE_COUNT, ADF_TL_CMDQ_REG_OFF!($name, reg_tm_cmdq_exec_cnt, gen6)) };
}
macro_rules! ADF_GEN6_TL_CMDQ_DRAIN_COUNTER {
    ($name:ident) => { ADF_TL_COUNTER!(concat!("cmdq_drain_", stringify!($name)), ADF_TL_SIMPLE_COUNT, ADF_TL_CMDQ_REG_OFF!($name, reg_tm_cmdq_drain_cnt, gen6)) };
}

const CPR_QUEUE_COUNT: usize = 5;
const DCPR_QUEUE_COUNT: usize = 3;
const PKE_QUEUE_COUNT: usize = 1;
const WAT_QUEUE_COUNT: usize = 7;
const WCP_QUEUE_COUNT: usize = 7;
const USC_QUEUE_COUNT: usize = 3;
const ATH_QUEUE_COUNT: usize = 2;

/* Device level counters. */
static DEV_COUNTERS: [adf_tl_dbg_counter; 11] = [
    ADF_TL_COUNTER!(PCI_TRANS_CNT_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_prt_trans_cnt)),
    ADF_TL_COUNTER!(MAX_RD_LAT_NAME, ADF_TL_COUNTER_NS, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_rd_lat_max)),
    ADF_TL_COUNTER_LATENCY!(RD_LAT_ACC_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_rd_lat_acc), ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_rd_cmpl_cnt)),
    ADF_TL_COUNTER!(MAX_LAT_NAME, ADF_TL_COUNTER_NS, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_gp_lat_max)),
    ADF_TL_COUNTER_LATENCY!(LAT_ACC_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_gp_lat_acc), ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_ae_put_cnt)),
    ADF_TL_COUNTER!(BW_IN_NAME, ADF_TL_COUNTER_MBPS, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_bw_in)),
    ADF_TL_COUNTER!(BW_OUT_NAME, ADF_TL_COUNTER_MBPS, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_bw_out)),
    ADF_TL_COUNTER_LATENCY!(PAGE_REQ_LAT_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_at_page_req_lat_acc), ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_at_page_req_cnt)),
    ADF_TL_COUNTER_LATENCY!(AT_TRANS_LAT_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_at_trans_lat_acc), ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_at_trans_lat_cnt)),
    ADF_TL_COUNTER!(AT_MAX_UTLB_USED_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_at_max_utlb_used)),
    ADF_TL_COUNTER_LATENCY!(RE_ACC_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_re_acc), ADF_GEN6_TL_DEV_REG_OFF!(reg_tl_re_cnt)),
];

/* Accelerator utilization and execution counters. */
static SL_UTIL_COUNTERS: [adf_tl_dbg_counter; ADF_TL_SL_CNT_COUNT] = [
    ADF_GEN6_TL_SL_UTIL_COUNTER!(cnv), ADF_GEN6_TL_SL_UTIL_COUNTER!(dcprz), ADF_GEN6_TL_SL_UTIL_COUNTER!(pke),
    ADF_GEN6_TL_SL_UTIL_COUNTER!(wat), ADF_GEN6_TL_SL_UTIL_COUNTER!(wcp), ADF_GEN6_TL_SL_UTIL_COUNTER!(ucs), ADF_GEN6_TL_SL_UTIL_COUNTER!(ath),
];
static SL_EXEC_COUNTERS: [adf_tl_dbg_counter; ADF_TL_SL_CNT_COUNT] = [
    ADF_GEN6_TL_SL_EXEC_COUNTER!(cnv), ADF_GEN6_TL_SL_EXEC_COUNTER!(dcprz), ADF_GEN6_TL_SL_EXEC_COUNTER!(pke),
    ADF_GEN6_TL_SL_EXEC_COUNTER!(wat), ADF_GEN6_TL_SL_EXEC_COUNTER!(wcp), ADF_GEN6_TL_SL_EXEC_COUNTER!(ucs), ADF_GEN6_TL_SL_EXEC_COUNTER!(ath),
];

macro_rules! CMDQ_COUNTERS { ($name:ident) => { [ADF_GEN6_TL_CMDQ_WAIT_COUNTER!($name), ADF_GEN6_TL_CMDQ_EXEC_COUNTER!($name), ADF_GEN6_TL_CMDQ_DRAIN_COUNTER!($name)] }; }
static CNV_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(cnv);
static DCPRZ_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(dcprz);
static PKE_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(pke);
static WAT_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(wat);
static WCP_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(wcp);
static UCS_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(ucs);
static ATH_CMDQ_COUNTERS: [adf_tl_dbg_counter; 3] = CMDQ_COUNTERS!(ath);
static CMDQ_COUNTERS_TABLE: [*const adf_tl_dbg_counter; ADF_TL_SL_CNT_COUNT] = [
    CNV_CMDQ_COUNTERS.as_ptr(), DCPRZ_CMDQ_COUNTERS.as_ptr(), PKE_CMDQ_COUNTERS.as_ptr(),
    WAT_CMDQ_COUNTERS.as_ptr(), WCP_CMDQ_COUNTERS.as_ptr(), UCS_CMDQ_COUNTERS.as_ptr(), ATH_CMDQ_COUNTERS.as_ptr(),
];

static RP_COUNTERS: [adf_tl_dbg_counter; 9] = [
    ADF_TL_COUNTER!(PCI_TRANS_CNT_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_prt_trans_cnt)),
    ADF_TL_COUNTER_LATENCY!(LAT_ACC_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_gp_lat_acc), ADF_GEN6_TL_RP_REG_OFF!(reg_tl_ae_put_cnt)),
    ADF_TL_COUNTER!(BW_IN_NAME, ADF_TL_COUNTER_MBPS, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_bw_in)),
    ADF_TL_COUNTER!(BW_OUT_NAME, ADF_TL_COUNTER_MBPS, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_bw_out)),
    ADF_TL_COUNTER!(AT_GLOB_DTLB_HIT_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_at_glob_devtlb_hit)),
    ADF_TL_COUNTER!(AT_GLOB_DTLB_MISS_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_at_glob_devtlb_miss)),
    ADF_TL_COUNTER!(AT_PAYLD_DTLB_HIT_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_at_payld_devtlb_hit)),
    ADF_TL_COUNTER!(AT_PAYLD_DTLB_MISS_NAME, ADF_TL_SIMPLE_COUNT, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_at_payld_devtlb_miss)),
    ADF_TL_COUNTER_LATENCY!(RE_ACC_NAME, ADF_TL_COUNTER_NS_AVG, ADF_GEN6_TL_RP_REG_OFF!(reg_tl_re_acc), ADF_GEN6_TL_RP_REG_OFF!(reg_tl_re_cnt)),
];

pub unsafe fn adf_gen6_init_tl_data(tl_data: *mut adf_tl_hw_data) {
    (*tl_data).layout_sz = ADF_GEN6_TL_LAYOUT_SZ;
    (*tl_data).slice_reg_sz = ADF_GEN6_TL_SLICE_REG_SZ;
    (*tl_data).cmdq_reg_sz = ADF_GEN6_TL_CMDQ_REG_SZ;
    (*tl_data).rp_reg_sz = ADF_GEN6_TL_RP_REG_SZ;
    (*tl_data).num_hbuff = ADF_GEN6_TL_NUM_HIST_BUFFS;
    (*tl_data).max_rp = ADF_GEN6_TL_MAX_RP_NUM;
    (*tl_data).msg_cnt_off = ADF_GEN6_TL_MSG_CNT_OFF;
    (*tl_data).cpp_ns_per_cycle = ADF_GEN6_CPP_NS_PER_CYCLE;
    (*tl_data).bw_units_to_bytes = ADF_GEN6_TL_BW_HW_UNITS_TO_BYTES;
    (*tl_data).dev_counters = DEV_COUNTERS.as_ptr();
    (*tl_data).num_dev_counters = DEV_COUNTERS.len();
    (*tl_data).sl_util_counters = SL_UTIL_COUNTERS.as_ptr();
    (*tl_data).sl_exec_counters = SL_EXEC_COUNTERS.as_ptr();
    (*tl_data).cmdq_counters = CMDQ_COUNTERS_TABLE.as_ptr();
    (*tl_data).num_cmdq_counters = 3;
    (*tl_data).rp_counters = RP_COUNTERS.as_ptr();
    (*tl_data).num_rp_counters = RP_COUNTERS.len();
    (*tl_data).max_sl_cnt = ADF_GEN6_TL_MAX_SLICES_PER_TYPE;
    (*tl_data).multiplier.cpr_cnt = CPR_QUEUE_COUNT;
    (*tl_data).multiplier.dcpr_cnt = DCPR_QUEUE_COUNT;
    (*tl_data).multiplier.pke_cnt = PKE_QUEUE_COUNT;
    (*tl_data).multiplier.wat_cnt = WAT_QUEUE_COUNT;
    (*tl_data).multiplier.wcp_cnt = WCP_QUEUE_COUNT;
    (*tl_data).multiplier.ucs_cnt = USC_QUEUE_COUNT;
    (*tl_data).multiplier.ath_cnt = ATH_QUEUE_COUNT;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
