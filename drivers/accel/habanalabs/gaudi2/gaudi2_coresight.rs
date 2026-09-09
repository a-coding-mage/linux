// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2019-2022 HabanaLabs, Ltd.
// All Rights Reserved.

// Translated from gaudi2_coresight.c.  Register definitions and driver types
// are supplied by the surrounding kernel translation unit.

pub const GAUDI2_PLDM_CORESIGHT_TIMEOUT_USEC: u64 = CORESIGHT_TIMEOUT_USEC * 2000;
pub const SPMU_MAX_COUNTERS: usize = 6;
pub const COMPONENT_ID_INVALID: u32 = u32::MAX;
pub const MAX_BMONS_PER_UNIT: usize = 8;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Gaudi2HifHmmuId {
    HMMU_ID_DCORE0_HMMU0, HMMU_ID_DCORE0_HMMU1, HMMU_ID_DCORE0_HMMU2,
    HMMU_ID_DCORE0_HMMU3, HMMU_ID_DCORE1_HMMU0, HMMU_ID_DCORE1_HMMU1,
    HMMU_ID_DCORE1_HMMU2, HMMU_ID_DCORE1_HMMU3, HMMU_ID_DCORE2_HMMU0,
    HMMU_ID_DCORE2_HMMU1, HMMU_ID_DCORE2_HMMU2, HMMU_ID_DCORE2_HMMU3,
    HMMU_ID_DCORE3_HMMU0, HMMU_ID_DCORE3_HMMU1, HMMU_ID_DCORE3_HMMU2,
    HMMU_ID_DCORE3_HMMU3, HMMU_ID_SIZE,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Gaudi2XbarEdgeId {
    XBAR_EDGE_ID_DCORE0, XBAR_EDGE_ID_DCORE1, XBAR_EDGE_ID_DCORE2,
    XBAR_EDGE_ID_DCORE3, XBAR_EDGE_ID_SIZE,
}

#[repr(C)]
pub struct ComponentConfigOffsets {
    pub funnel_id: u32,
    pub etf_id: u32,
    pub stm_id: u32,
    pub spmu_id: u32,
    pub bmon_count: u32,
    pub bmon_ids: [u32; MAX_BMONS_PER_UNIT],
}

// The following tables are indexed by the corresponding GAUDI2_* enum values.
// Their register constants are intentionally kept as external dependencies.
extern "C" {
    static mut debug_stm_regs: [u64; GAUDI2_STM_LAST as usize + 1];
    static mut debug_etf_regs: [u64; GAUDI2_ETF_LAST as usize + 1];
    static mut debug_funnel_regs: [u64; GAUDI2_FUNNEL_LAST as usize + 1];
    static mut debug_bmon_regs: [u64; GAUDI2_BMON_LAST as usize + 1];
    static mut debug_spmu_regs: [u64; GAUDI2_SPMU_LAST as usize + 1];
}

extern "C" {
    fn gaudi2_coresight_timeout(hdev: *mut hl_device, addr: u64, val: u32) -> i32;
    fn gaudi2_unlock_coresight_unit(hdev: *mut hl_device, addr: u64) -> i32;
    fn gaudi2_config_stm(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32;
    fn gaudi2_config_etf(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32;
    fn gaudi2_etr_validate_address(hdev: *mut hl_device, addr: u64, size: u64) -> i32;
    fn gaudi2_config_etr(hdev: *mut hl_device, ctx: *mut hl_ctx, params: *mut hl_debug_params) -> i32;
    fn gaudi2_config_funnel(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32;
    fn gaudi2_config_bmon(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32;
    fn gaudi2_config_spmu(hdev: *mut hl_device, params: *mut hl_debug_params) -> i32;
}

extern "C" {
    pub fn gaudi2_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32;
    pub fn gaudi2_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
    pub fn gaudi2_coresight_init(hdev: *mut hl_device) -> i32;
}

// External kernel declarations used by this translation.
#[allow(non_camel_case_types)] pub enum hl_device {}
#[allow(non_camel_case_types)] pub enum hl_ctx {}
#[allow(non_camel_case_types)] pub enum hl_debug_params {}
extern "C" { static CORESIGHT_TIMEOUT_USEC: u64; }
extern "C" { static GAUDI2_STM_LAST: u32; static GAUDI2_ETF_LAST: u32; static GAUDI2_FUNNEL_LAST: u32; static GAUDI2_BMON_LAST: u32; static GAUDI2_SPMU_LAST: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
