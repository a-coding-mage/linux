/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2023 Intel Corporation. */

// The C header includes linux/types.h and refers to declarations supplied by
// other translation units.

use core::ffi::c_char;

pub struct adf_accel_dev;

pub const MAX_COUNT_NAME_SIZE: usize = 32;
pub const SNAPSHOT_CNT_MSG: &str = "sample_cnt";
pub const RP_NUM_INDEX: &str = "rp_num";
pub const PCI_TRANS_CNT_NAME: &str = "pci_trans_cnt";
pub const MAX_RD_LAT_NAME: &str = "max_rd_lat";
pub const RD_LAT_ACC_NAME: &str = "rd_lat_acc_avg";
pub const MAX_LAT_NAME: &str = "max_gp_lat";
pub const LAT_ACC_NAME: &str = "gp_lat_acc_avg";
pub const BW_IN_NAME: &str = "bw_in";
pub const BW_OUT_NAME: &str = "bw_out";
pub const RE_ACC_NAME: &str = "re_acc_avg";
pub const PAGE_REQ_LAT_NAME: &str = "at_page_req_lat_avg";
pub const AT_TRANS_LAT_NAME: &str = "at_trans_lat_avg";
pub const AT_MAX_UTLB_USED_NAME: &str = "at_max_tlb_used";
pub const AT_GLOB_DTLB_HIT_NAME: &str = "at_glob_devtlb_hit";
pub const AT_GLOB_DTLB_MISS_NAME: &str = "at_glob_devtlb_miss";
pub const AT_PAYLD_DTLB_HIT_NAME: &str = "tl_at_payld_devtlb_hit";
pub const AT_PAYLD_DTLB_MISS_NAME: &str = "tl_at_payld_devtlb_miss";
pub const RP_SERVICE_TYPE: &str = "service_type";

#[inline]
pub const fn ADF_TL_DBG_RP_ALPHA_INDEX(index: i32) -> i32 { index + b'A' as i32 }

#[inline]
pub const fn ADF_TL_DBG_RP_INDEX_ALPHA(alpha: i32) -> i32 { alpha - b'A' as i32 }

pub const ADF_TL_RP_REGS_FNAME: &str = "rp_%c_data";
pub const ADF_TL_RP_REGS_FNAME_SIZE: usize = 16;

// These C macros use token-pasted, generation-specific struct names and
// offsetof; the equivalent offset_of! expressions require those external
// struct definitions and are retained as macros for dependent translation.
#[macro_export]
macro_rules! ADF_TL_DATA_REG_OFF { ($layout:ty, $reg:tt) => { core::mem::offset_of!($layout, $reg) }; }
#[macro_export]
macro_rules! ADF_TL_DEV_REG_OFF { ($layout:ty, $device_regs:ty, $reg:tt, $device_reg:tt) => { core::mem::offset_of!($layout, $device_reg) + core::mem::offset_of!($device_regs, $reg) }; }
#[macro_export]
macro_rules! ADF_TL_SLICE_REG_OFF { ($layout:ty, $device_regs:ty, $slice_regs:ty, $slice:tt, $reg:tt) => { core::mem::offset_of!($layout, $slice) + core::mem::offset_of!($device_regs, $slice) + core::mem::offset_of!($slice_regs, $reg) }; }
#[macro_export]
macro_rules! ADF_TL_CMDQ_REG_OFF { ($layout:ty, $device_regs:ty, $cmdq_regs:ty, $cmdq:tt, $reg:tt) => { core::mem::offset_of!($layout, $cmdq) + core::mem::offset_of!($device_regs, $cmdq) + core::mem::offset_of!($cmdq_regs, $reg) }; }
#[macro_export]
macro_rules! ADF_TL_RP_REG_OFF { ($layout:ty, $ring_pair_regs:ty, $reg:tt) => { core::mem::offset_of!($layout, $ring_pair_regs) + core::mem::offset_of!($ring_pair_regs, $reg) }; }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adf_tl_counter_type {
    ADF_TL_COUNTER_UNSUPPORTED,
    ADF_TL_SIMPLE_COUNT,
    ADF_TL_COUNTER_NS,
    ADF_TL_COUNTER_NS_AVG,
    ADF_TL_COUNTER_MBPS,
}

#[repr(C)]
pub struct adf_tl_dbg_counter {
    pub name: *const c_char,
    pub type_: adf_tl_counter_type,
    pub offset1: usize,
    pub offset2: usize,
}

#[macro_export]
macro_rules! ADF_TL_COUNTER { ($name:expr, $type_:expr, $offset:expr) => { $crate::adf_tl_dbg_counter { name: $name, type_: $type_, offset1: $offset, offset2: 0 } }; }

#[macro_export]
macro_rules! ADF_TL_COUNTER_LATENCY { ($name:expr, $type_:expr, $offset1:expr, $offset2:expr) => { $crate::adf_tl_dbg_counter { name: $name, type_: $type_, offset1: $offset1, offset2: $offset2 } }; }

/* Telemetry counter aggregated values. */
#[repr(C)]
pub struct adf_tl_dbg_aggr_values {
    pub curr: u64,
    pub min: u64,
    pub max: u64,
    pub avg: u64,
}

extern "C" {
    /** Add telemetry's debug fs entries. */
    pub fn adf_tl_dbgfs_add(accel_dev: *mut adf_accel_dev);

    /** Remove telemetry's debug fs entries. */
    pub fn adf_tl_dbgfs_rm(accel_dev: *mut adf_accel_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
