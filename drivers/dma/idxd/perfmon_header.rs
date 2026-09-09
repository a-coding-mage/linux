/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2020 Intel Corporation. All rights rsvd. */

// Declarations supplied by the Linux kernel and registers.h are intentionally
// left as external dependencies, as in the original header.

#[inline]
pub unsafe fn event_to_pmu(event: *mut perf_event) -> *mut idxd_pmu {
    let pmu = (*event).pmu;
    container_of_pmu(pmu)
}

#[inline]
pub unsafe fn event_to_idxd(event: *mut perf_event) -> *mut idxd_device {
    let pmu = (*event).pmu;
    (*container_of_pmu(pmu)).idxd
}

#[inline]
pub unsafe fn pmu_to_idxd(pmu: *mut pmu) -> *mut idxd_device {
    (*container_of_pmu(pmu)).idxd
}

#[repr(C)]
pub enum dsa_perf_events {
    DSA_PERF_EVENT_WQ = 0,
    DSA_PERF_EVENT_ENGINE,
    DSA_PERF_EVENT_ADDR_TRANS,
    DSA_PERF_EVENT_OP,
    DSA_PERF_EVENT_COMPL,
    DSA_PERF_EVENT_MAX,
}

#[repr(C)]
pub enum filter_enc {
    FLT_WQ = 0,
    FLT_TC,
    FLT_PG_SZ,
    FLT_XFER_SZ,
    FLT_ENG,
    FLT_MAX,
}

pub const CONFIG_RESET: u64 = 0x0000000000000001;
pub const CNTR_RESET: u64 = 0x0000000000000002;
pub const CNTR_ENABLE: u64 = 0x0000000000000001;
pub const INTR_OVFL: u64 = 0x0000000000000002;

pub const COUNTER_FREEZE: u64 = 0x00000000FFFFFFFF;
pub const COUNTER_UNFREEZE: u64 = 0x0000000000000000;
pub const OVERFLOW_SIZE: u32 = 32;

pub const CNTRCFG_ENABLE: u64 = 1 << 0;
pub const CNTRCFG_IRQ_OVERFLOW: u64 = 1 << 1;
pub const CNTRCFG_CATEGORY_SHIFT: u32 = 8;
pub const CNTRCFG_EVENT_SHIFT: u32 = 32;

#[inline]
pub unsafe fn perfmon_table_offset(idxd: *mut idxd_device) -> usize {
    (*idxd).reg_base + (*idxd).perfmon_offset
}

#[inline]
pub unsafe fn perfmon_reg_offset(idxd: *mut idxd_device, offset: usize) -> usize {
    perfmon_table_offset(idxd) + offset
}

#[inline]
pub unsafe fn perfcap_reg(idxd: *mut idxd_device) -> usize {
    perfmon_reg_offset(idxd, IDXD_PERFCAP_OFFSET)
}

#[inline]
pub unsafe fn perfrst_reg(idxd: *mut idxd_device) -> usize {
    perfmon_reg_offset(idxd, IDXD_PERFRST_OFFSET)
}

#[inline]
pub unsafe fn ovfstatus_reg(idxd: *mut idxd_device) -> usize {
    perfmon_reg_offset(idxd, IDXD_OVFSTATUS_OFFSET)
}

#[inline]
pub unsafe fn perffrz_reg(idxd: *mut idxd_device) -> usize {
    perfmon_reg_offset(idxd, IDXD_PERFFRZ_OFFSET)
}

#[inline]
pub unsafe fn fltcfg_reg(idxd: *mut idxd_device, cntr: usize, flt: usize) -> usize {
    perfmon_reg_offset(idxd, IDXD_FLTCFG_OFFSET) + (cntr * 32) + (flt * 4)
}

#[inline]
pub unsafe fn cntrcfg_reg(idxd: *mut idxd_device, cntr: usize) -> usize {
    perfmon_reg_offset(idxd, IDXD_CNTRCFG_OFFSET) + (cntr * 8)
}

#[inline]
pub unsafe fn cntrdata_reg(idxd: *mut idxd_device, cntr: usize) -> usize {
    perfmon_reg_offset(idxd, IDXD_CNTRDATA_OFFSET) + (cntr * 8)
}

#[inline]
pub unsafe fn cntrcap_reg(idxd: *mut idxd_device, cntr: usize) -> usize {
    perfmon_reg_offset(idxd, IDXD_CNTRCAP_OFFSET) + (cntr * 8)
}

#[inline]
pub unsafe fn evntcap_reg(idxd: *mut idxd_device, category: usize) -> usize {
    perfmon_reg_offset(idxd, IDXD_EVNTCAP_OFFSET) + (category * 8)
}

// DEFINE_PERFMON_FORMAT_ATTR generates a read-only kobject format attribute
// whose show function writes the supplied format string followed by a newline.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
