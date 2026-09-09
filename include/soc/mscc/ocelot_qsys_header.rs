/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/* Microsemi Ocelot Switch driver; translated from the C header. */

const fn genmask(hi: u32, lo: u32) -> u32 {
    (((1u32 << (hi - lo + 1)) - 1) << lo)
}

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

pub const QSYS_PORT_MODE_RSZ: u32 = 0x4;
pub const QSYS_PORT_MODE_DEQUEUE_DIS: u32 = bit!(1);
pub const QSYS_PORT_MODE_DEQUEUE_LATE: u32 = bit!(0);
pub const QSYS_STAT_CNT_CFG_TX_GREEN_CNT_MODE: u32 = bit!(5);
pub const QSYS_STAT_CNT_CFG_TX_YELLOW_CNT_MODE: u32 = bit!(4);
pub const QSYS_STAT_CNT_CFG_DROP_GREEN_CNT_MODE: u32 = bit!(3);
pub const QSYS_STAT_CNT_CFG_DROP_YELLOW_CNT_MODE: u32 = bit!(2);
pub const QSYS_STAT_CNT_CFG_DROP_COUNT_ONCE: u32 = bit!(1);
pub const QSYS_STAT_CNT_CFG_DROP_COUNT_EGRESS: u32 = bit!(0);
pub const QSYS_EEE_CFG_RSZ: u32 = 0x4;

macro_rules! QSYS_EEE_THRES_EEE_HIGH_BYTES { ($x:expr) => { (($x << 8) & genmask(15, 8)) }; }
pub const QSYS_EEE_THRES_EEE_HIGH_BYTES_M: u32 = genmask(15, 8);
macro_rules! QSYS_EEE_THRES_EEE_HIGH_BYTES_X { ($x:expr) => { (($x & genmask(15, 8)) >> 8) }; }
macro_rules! QSYS_EEE_THRES_EEE_HIGH_FRAMES { ($x:expr) => { ($x & genmask(7, 0)) }; }
pub const QSYS_EEE_THRES_EEE_HIGH_FRAMES_M: u32 = genmask(7, 0);
pub const QSYS_SW_STATUS_RSZ: u32 = 0x4;

macro_rules! qsys_shift_mask { ($x:expr, $s:expr, $h:expr, $l:expr) => { (($x << $s) & genmask($h, $l)) }; }
macro_rules! qsys_extract { ($x:expr, $s:expr, $h:expr, $l:expr) => { (($x & genmask($h, $l)) >> $s) }; }

macro_rules! QSYS_EXT_CPU_CFG_EXT_CPU_PORT { ($x:expr) => { qsys_shift_mask!($x, 8, 12, 8) }; }
pub const QSYS_EXT_CPU_CFG_EXT_CPU_PORT_M: u32 = genmask(12, 8);
macro_rules! QSYS_EXT_CPU_CFG_EXT_CPU_PORT_X { ($x:expr) => { qsys_extract!($x, 8, 12, 8) }; }
macro_rules! QSYS_EXT_CPU_CFG_EXT_CPUQ_MSK { ($x:expr) => { $x & genmask(7, 0) }; }
pub const QSYS_EXT_CPU_CFG_EXT_CPUQ_MSK_M: u32 = genmask(7, 0);
pub const QSYS_QMAP_GSZ: u32 = 0x4;
macro_rules! QSYS_QMAP_SE_BASE { ($x:expr) => { qsys_shift_mask!($x, 5, 12, 5) }; }
pub const QSYS_QMAP_SE_BASE_M: u32 = genmask(12, 5);
macro_rules! QSYS_QMAP_SE_BASE_X { ($x:expr) => { qsys_extract!($x, 5, 12, 5) }; }
macro_rules! QSYS_QMAP_SE_IDX_SEL { ($x:expr) => { qsys_shift_mask!($x, 2, 4, 2) }; }
pub const QSYS_QMAP_SE_IDX_SEL_M: u32 = genmask(4, 2);
macro_rules! QSYS_QMAP_SE_IDX_SEL_X { ($x:expr) => { qsys_extract!($x, 2, 4, 2) }; }
macro_rules! QSYS_QMAP_SE_INP_SEL { ($x:expr) => { $x & genmask(1, 0) }; }
pub const QSYS_QMAP_SE_INP_SEL_M: u32 = genmask(1, 0);

pub const QSYS_ISDX_SGRP_GSZ: u32 = 0x4;
pub const QSYS_TIMED_FRAME_ENTRY_GSZ: u32 = 0x4;
macro_rules! QSYS_TFRM_MISC_TIMED_CANCEL_SLOT { ($x:expr) => { qsys_shift_mask!($x, 9, 18, 9) }; }
pub const QSYS_TFRM_MISC_TIMED_CANCEL_SLOT_M: u32 = genmask(18, 9);
macro_rules! QSYS_TFRM_MISC_TIMED_CANCEL_SLOT_X { ($x:expr) => { qsys_extract!($x, 9, 18, 9) }; }
pub const QSYS_TFRM_MISC_TIMED_CANCEL_1SHOT: u32 = bit!(8);
pub const QSYS_TFRM_MISC_TIMED_SLOT_MODE_MC: u32 = bit!(7);
macro_rules! QSYS_TFRM_MISC_TIMED_ENTRY_FAST_CNT { ($x:expr) => { $x & genmask(6, 0) }; }
pub const QSYS_TFRM_MISC_TIMED_ENTRY_FAST_CNT_M: u32 = genmask(6, 0);

pub const QSYS_RED_PROFILE_RSZ: u32 = 0x4;
macro_rules! QSYS_RED_PROFILE_WM_RED_LOW { ($x:expr) => { qsys_shift_mask!($x, 8, 15, 8) }; }
pub const QSYS_RED_PROFILE_WM_RED_LOW_M: u32 = genmask(15, 8);
macro_rules! QSYS_RED_PROFILE_WM_RED_LOW_X { ($x:expr) => { qsys_extract!($x, 8, 15, 8) }; }
macro_rules! QSYS_RED_PROFILE_WM_RED_HIGH { ($x:expr) => { $x & genmask(7, 0) }; }
pub const QSYS_RED_PROFILE_WM_RED_HIGH_M: u32 = genmask(7, 0);
pub const QSYS_RES_CFG_GSZ: u32 = 0x8;
pub const QSYS_RES_STAT_GSZ: u32 = 0x8;
macro_rules! QSYS_MMGT_EQ_CTRL_FP_FREE_CNT { ($x:expr) => { $x & genmask(15, 0) }; }
pub const QSYS_MMGT_EQ_CTRL_FP_FREE_CNT_M: u32 = genmask(15, 0);
macro_rules! QSYS_EVENTS_CORE_EV_FDC { ($x:expr) => { qsys_shift_mask!($x, 2, 4, 2) }; }
pub const QSYS_EVENTS_CORE_EV_FDC_M: u32 = genmask(4, 2);
macro_rules! QSYS_EVENTS_CORE_EV_FDC_X { ($x:expr) => { qsys_extract!($x, 2, 4, 2) }; }
macro_rules! QSYS_EVENTS_CORE_EV_FRD { ($x:expr) => { $x & genmask(1, 0) }; }
pub const QSYS_EVENTS_CORE_EV_FRD_M: u32 = genmask(1, 0);

/* Remaining register fields retain the same masks and shifts as the source. */
macro_rules! QSYS_FIELD { ($x:expr, $shift:expr, $hi:expr, $lo:expr) => { qsys_shift_mask!($x, $shift, $hi, $lo) }; }
macro_rules! QSYS_FIELD_X { ($x:expr, $shift:expr, $hi:expr, $lo:expr) => { qsys_extract!($x, $shift, $hi, $lo) }; }

pub const QSYS_QMAXSDU_CFG_0_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_1_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_2_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_3_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_4_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_5_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_6_RSZ: u32 = 0x4;
pub const QSYS_QMAXSDU_CFG_7_RSZ: u32 = 0x4;
pub const QSYS_PREEMPTION_CFG_RSZ: u32 = 0x4;
macro_rules! QSYS_PREEMPTION_CFG_P_QUEUES { ($x:expr) => { $x & genmask(7,0) }; }
pub const QSYS_PREEMPTION_CFG_P_QUEUES_M: u32 = genmask(7,0);
macro_rules! QSYS_PREEMPTION_CFG_MM_ADD_FRAG_SIZE { ($x:expr) => { qsys_shift_mask!($x,8,9,8) }; }
pub const QSYS_PREEMPTION_CFG_MM_ADD_FRAG_SIZE_M: u32 = genmask(9,8);
macro_rules! QSYS_PREEMPTION_CFG_MM_ADD_FRAG_SIZE_X { ($x:expr) => { qsys_extract!($x,8,9,8) }; }
macro_rules! QSYS_PREEMPTION_CFG_STRICT_IPG { ($x:expr) => { qsys_shift_mask!($x,12,13,12) }; }
pub const QSYS_PREEMPTION_CFG_STRICT_IPG_M: u32 = genmask(13,12);
macro_rules! QSYS_PREEMPTION_CFG_STRICT_IPG_X { ($x:expr) => { qsys_extract!($x,12,13,12) }; }
macro_rules! QSYS_PREEMPTION_CFG_HOLD_ADVANCE { ($x:expr) => { qsys_shift_mask!($x,16,31,16) }; }
pub const QSYS_PREEMPTION_CFG_HOLD_ADVANCE_M: u32 = genmask(31,16);
macro_rules! QSYS_PREEMPTION_CFG_HOLD_ADVANCE_X { ($x:expr) => { qsys_extract!($x,16,31,16) }; }

pub const QSYS_CIR_CFG_GSZ: u32 = 0x80;
macro_rules! QSYS_CIR_CFG_CIR_RATE { ($x:expr) => { qsys_shift_mask!($x,6,20,6) }; }
pub const QSYS_CIR_CFG_CIR_RATE_M: u32 = genmask(20,6);
macro_rules! QSYS_CIR_CFG_CIR_RATE_X { ($x:expr) => { qsys_extract!($x,6,20,6) }; }
macro_rules! QSYS_CIR_CFG_CIR_BURST { ($x:expr) => { $x & genmask(5,0) }; }
pub const QSYS_CIR_CFG_CIR_BURST_M: u32 = genmask(5,0);
pub const QSYS_EIR_CFG_GSZ: u32 = 0x80;
macro_rules! QSYS_EIR_CFG_EIR_RATE { ($x:expr) => { qsys_shift_mask!($x,7,21,7) }; }
pub const QSYS_EIR_CFG_EIR_RATE_M: u32 = genmask(21,7);
macro_rules! QSYS_EIR_CFG_EIR_RATE_X { ($x:expr) => { qsys_extract!($x,7,21,7) }; }
macro_rules! QSYS_EIR_CFG_EIR_BURST { ($x:expr) => { qsys_shift_mask!($x,1,6,1) }; }
pub const QSYS_EIR_CFG_EIR_BURST_M: u32 = genmask(6,1);
macro_rules! QSYS_EIR_CFG_EIR_BURST_X { ($x:expr) => { qsys_extract!($x,1,6,1) }; }
pub const QSYS_EIR_CFG_EIR_MARK_ENA: u32 = bit!(0);

pub const QSYS_SE_CFG_GSZ: u32 = 0x80;
macro_rules! QSYS_SE_CFG_SE_DWRR_CNT { ($x:expr) => { qsys_shift_mask!($x,6,9,6) }; }
pub const QSYS_SE_CFG_SE_DWRR_CNT_M: u32 = genmask(9,6);
macro_rules! QSYS_SE_CFG_SE_DWRR_CNT_X { ($x:expr) => { qsys_extract!($x,6,9,6) }; }
pub const QSYS_SE_CFG_SE_RR_ENA: u32 = bit!(5);
pub const QSYS_SE_CFG_SE_AVB_ENA: u32 = bit!(4);
macro_rules! QSYS_SE_CFG_SE_FRM_MODE { ($x:expr) => { qsys_shift_mask!($x,2,3,2) }; }
pub const QSYS_SE_CFG_SE_FRM_MODE_M: u32 = genmask(3,2);
macro_rules! QSYS_SE_CFG_SE_FRM_MODE_X { ($x:expr) => { qsys_extract!($x,2,3,2) }; }
pub const QSYS_SE_CFG_SE_EXC_ENA: u32 = bit!(1);
pub const QSYS_SE_CFG_SE_EXC_FWD: u32 = bit!(0);
pub const QSYS_SE_DWRR_CFG_GSZ: u32 = 0x80;
pub const QSYS_SE_DWRR_CFG_RSZ: u32 = 0x4;
pub const QSYS_SE_CONNECT_GSZ: u32 = 0x80;

macro_rules! QSYS_SE_CONNECT_SE_OUTP_IDX { ($x:expr) => { qsys_shift_mask!($x,17,24,17) }; }
pub const QSYS_SE_CONNECT_SE_OUTP_IDX_M: u32 = genmask(24,17);
macro_rules! QSYS_SE_CONNECT_SE_OUTP_IDX_X { ($x:expr) => { qsys_extract!($x,17,24,17) }; }
macro_rules! QSYS_SE_CONNECT_SE_INP_IDX { ($x:expr) => { qsys_shift_mask!($x,9,16,9) }; }
pub const QSYS_SE_CONNECT_SE_INP_IDX_M: u32 = genmask(16,9);
macro_rules! QSYS_SE_CONNECT_SE_INP_IDX_X { ($x:expr) => { qsys_extract!($x,9,16,9) }; }
macro_rules! QSYS_SE_CONNECT_SE_OUTP_CON { ($x:expr) => { qsys_shift_mask!($x,5,8,5) }; }
pub const QSYS_SE_CONNECT_SE_OUTP_CON_M: u32 = genmask(8,5);
macro_rules! QSYS_SE_CONNECT_SE_OUTP_CON_X { ($x:expr) => { qsys_extract!($x,5,8,5) }; }
macro_rules! QSYS_SE_CONNECT_SE_INP_CNT { ($x:expr) => { qsys_shift_mask!($x,1,4,1) }; }
pub const QSYS_SE_CONNECT_SE_INP_CNT_M: u32 = genmask(4,1);
macro_rules! QSYS_SE_CONNECT_SE_INP_CNT_X { ($x:expr) => { qsys_extract!($x,1,4,1) }; }
pub const QSYS_SE_CONNECT_SE_TERMINAL: u32 = bit!(0);

pub const QSYS_SE_DLB_SENSE_GSZ: u32 = 0x80;
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_PRIO { ($x:expr) => { qsys_shift_mask!($x,11,13,11) }; }
pub const QSYS_SE_DLB_SENSE_SE_DLB_PRIO_M: u32 = genmask(13,11);
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_PRIO_X { ($x:expr) => { qsys_extract!($x,11,13,11) }; }
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_SPORT { ($x:expr) => { qsys_shift_mask!($x,7,10,7) }; }
pub const QSYS_SE_DLB_SENSE_SE_DLB_SPORT_M: u32 = genmask(10,7);
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_SPORT_X { ($x:expr) => { qsys_extract!($x,7,10,7) }; }
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_DPORT { ($x:expr) => { qsys_shift_mask!($x,3,6,3) }; }
pub const QSYS_SE_DLB_SENSE_SE_DLB_DPORT_M: u32 = genmask(6,3);
macro_rules! QSYS_SE_DLB_SENSE_SE_DLB_DPORT_X { ($x:expr) => { qsys_extract!($x,3,6,3) }; }
pub const QSYS_SE_DLB_SENSE_SE_DLB_PRIO_ENA: u32 = bit!(2);
pub const QSYS_SE_DLB_SENSE_SE_DLB_SPORT_ENA: u32 = bit!(1);
pub const QSYS_SE_DLB_SENSE_SE_DLB_DPORT_ENA: u32 = bit!(0);

pub const QSYS_CIR_STATE_GSZ: u32 = 0x80;
macro_rules! QSYS_CIR_STATE_CIR_LVL { ($x:expr) => { qsys_shift_mask!($x,4,25,4) }; }
pub const QSYS_CIR_STATE_CIR_LVL_M: u32 = genmask(25,4);
macro_rules! QSYS_CIR_STATE_CIR_LVL_X { ($x:expr) => { qsys_extract!($x,4,25,4) }; }
macro_rules! QSYS_CIR_STATE_SHP_TIME { ($x:expr) => { $x & genmask(3,0) }; }
pub const QSYS_CIR_STATE_SHP_TIME_M: u32 = genmask(3,0);
pub const QSYS_EIR_STATE_GSZ: u32 = 0x80;
pub const QSYS_SE_STATE_GSZ: u32 = 0x80;
macro_rules! QSYS_SE_STATE_SE_OUTP_LVL { ($x:expr) => { qsys_shift_mask!($x,1,2,1) }; }
pub const QSYS_SE_STATE_SE_OUTP_LVL_M: u32 = genmask(2,1);
macro_rules! QSYS_SE_STATE_SE_OUTP_LVL_X { ($x:expr) => { qsys_extract!($x,1,2,1) }; }
pub const QSYS_SE_STATE_SE_WAS_YEL: u32 = bit!(0);

pub const QSYS_HSCH_MISC_CFG_SE_CONNECT_VLD: u32 = bit!(8);
macro_rules! QSYS_HSCH_MISC_CFG_FRM_ADJ { ($x:expr) => { qsys_shift_mask!($x,3,7,3) }; }
pub const QSYS_HSCH_MISC_CFG_FRM_ADJ_M: u32 = genmask(7,3);
macro_rules! QSYS_HSCH_MISC_CFG_FRM_ADJ_X { ($x:expr) => { qsys_extract!($x,3,7,3) }; }
pub const QSYS_HSCH_MISC_CFG_LEAK_DIS: u32 = bit!(2);
pub const QSYS_HSCH_MISC_CFG_QSHP_EXC_ENA: u32 = bit!(1);
pub const QSYS_HSCH_MISC_CFG_PFC_BYP_UPD: u32 = bit!(0);

pub const QSYS_TAG_CONFIG_RSZ: u32 = 0x4;
pub const QSYS_TAG_CONFIG_ENABLE: u32 = bit!(0);
macro_rules! QSYS_TAG_CONFIG_LINK_SPEED { ($x:expr) => { qsys_shift_mask!($x,4,5,4) }; }
pub const QSYS_TAG_CONFIG_LINK_SPEED_M: u32 = genmask(5,4);
macro_rules! QSYS_TAG_CONFIG_LINK_SPEED_X { ($x:expr) => { qsys_extract!($x,4,5,4) }; }
macro_rules! QSYS_TAG_CONFIG_INIT_GATE_STATE { ($x:expr) => { qsys_shift_mask!($x,8,15,8) }; }
pub const QSYS_TAG_CONFIG_INIT_GATE_STATE_M: u32 = genmask(15,8);
macro_rules! QSYS_TAG_CONFIG_INIT_GATE_STATE_X { ($x:expr) => { qsys_extract!($x,8,15,8) }; }
macro_rules! QSYS_TAG_CONFIG_SCH_TRAFFIC_QUEUES { ($x:expr) => { qsys_shift_mask!($x,16,23,16) }; }
pub const QSYS_TAG_CONFIG_SCH_TRAFFIC_QUEUES_M: u32 = genmask(23,16);
macro_rules! QSYS_TAG_CONFIG_SCH_TRAFFIC_QUEUES_X { ($x:expr) => { qsys_extract!($x,16,23,16) }; }

macro_rules! QSYS_TAS_PARAM_CFG_CTRL_PORT_NUM { ($x:expr) => { $x & genmask(7,0) }; }
pub const QSYS_TAS_PARAM_CFG_CTRL_PORT_NUM_M: u32 = genmask(7,0);
pub const QSYS_TAS_PARAM_CFG_CTRL_ALWAYS_GUARD_BAND_SCH_Q: u32 = bit!(8);
pub const QSYS_TAS_PARAM_CFG_CTRL_CONFIG_CHANGE: u32 = bit!(16);
pub const QSYS_PORT_MAX_SDU_RSZ: u32 = 0x4;
macro_rules! QSYS_PARAM_CFG_REG_3_BASE_TIME_SEC_MSB { ($x:expr) => { $x & genmask(15,0) }; }
pub const QSYS_PARAM_CFG_REG_3_BASE_TIME_SEC_MSB_M: u32 = genmask(15,0);
macro_rules! QSYS_PARAM_CFG_REG_3_LIST_LENGTH { ($x:expr) => { qsys_shift_mask!($x,16,31,16) }; }
pub const QSYS_PARAM_CFG_REG_3_LIST_LENGTH_M: u32 = genmask(31,16);
macro_rules! QSYS_PARAM_CFG_REG_3_LIST_LENGTH_X { ($x:expr) => { qsys_extract!($x,16,31,16) }; }
macro_rules! QSYS_GCL_CFG_REG_1_GCL_ENTRY_NUM { ($x:expr) => { $x & genmask(5,0) }; }
pub const QSYS_GCL_CFG_REG_1_GCL_ENTRY_NUM_M: u32 = genmask(5,0);
macro_rules! QSYS_GCL_CFG_REG_1_GATE_STATE { ($x:expr) => { qsys_shift_mask!($x,8,15,8) }; }
pub const QSYS_GCL_CFG_REG_1_GATE_STATE_M: u32 = genmask(15,8);
macro_rules! QSYS_GCL_CFG_REG_1_GATE_STATE_X { ($x:expr) => { qsys_extract!($x,8,15,8) }; }
macro_rules! QSYS_PARAM_STATUS_REG_3_BASE_TIME_SEC_MSB { ($x:expr) => { $x & genmask(15,0) }; }
pub const QSYS_PARAM_STATUS_REG_3_BASE_TIME_SEC_MSB_M: u32 = genmask(15,0);
macro_rules! QSYS_PARAM_STATUS_REG_3_LIST_LENGTH { ($x:expr) => { qsys_shift_mask!($x,16,31,16) }; }
pub const QSYS_PARAM_STATUS_REG_3_LIST_LENGTH_M: u32 = genmask(31,16);
macro_rules! QSYS_PARAM_STATUS_REG_3_LIST_LENGTH_X { ($x:expr) => { qsys_extract!($x,16,31,16) }; }
macro_rules! QSYS_PARAM_STATUS_REG_8_CFG_CHG_TIME_SEC_MSB { ($x:expr) => { $x & genmask(15,0) }; }
pub const QSYS_PARAM_STATUS_REG_8_CFG_CHG_TIME_SEC_MSB_M: u32 = genmask(15,0);
macro_rules! QSYS_PARAM_STATUS_REG_8_OPER_GATE_STATE { ($x:expr) => { qsys_shift_mask!($x,16,23,16) }; }
pub const QSYS_PARAM_STATUS_REG_8_OPER_GATE_STATE_M: u32 = genmask(23,16);
macro_rules! QSYS_PARAM_STATUS_REG_8_OPER_GATE_STATE_X { ($x:expr) => { qsys_extract!($x,16,23,16) }; }
pub const QSYS_PARAM_STATUS_REG_8_CONFIG_PENDING: u32 = bit!(24);
macro_rules! QSYS_GCL_STATUS_REG_1_GCL_ENTRY_NUM { ($x:expr) => { $x & genmask(5,0) }; }
pub const QSYS_GCL_STATUS_REG_1_GCL_ENTRY_NUM_M: u32 = genmask(5,0);
macro_rules! QSYS_GCL_STATUS_REG_1_GATE_STATE { ($x:expr) => { qsys_shift_mask!($x,8,15,8) }; }
pub const QSYS_GCL_STATUS_REG_1_GATE_STATE_M: u32 = genmask(15,8);
macro_rules! QSYS_GCL_STATUS_REG_1_GATE_STATE_X { ($x:expr) => { qsys_extract!($x,8,15,8) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
