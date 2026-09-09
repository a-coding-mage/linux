// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies are supplied by the surrounding kernel translation unit.

const MP1_BASE_INST0_SEG0: u32 = 0x00016000;
const MP1_BASE_INST0_SEG1: u32 = 0x00016200;
const MP1_BASE_INST0_SEG2: u32 = 0x00E00000;
const MP1_BASE_INST0_SEG3: u32 = 0x00E80000;
const MP1_BASE_INST0_SEG4: u32 = 0x00EC0000;
const MP1_BASE_INST0_SEG5: u32 = 0x00F00000;
const MP1_BASE_INST0_SEG6: u32 = 0x02400400;
const MP1_BASE_INST0_SEG7: u32 = 0x0243F400;
const MP1_BASE_INST0_SEG8: u32 = 0x3C004000;
const MP1_BASE_INST0_SEG9: u32 = 0x3C3F4000;

const DAL_MSG_REG: u32 = MP1_SMN_C2PMSG_71;
const DAL_RESP_REG: u32 = MP1_SMN_C2PMSG_72;
const DAL_ARG_REG: u32 = MP1_SMN_C2PMSG_73;

const DALSMC_RESULT_OK: u32 = 0x01;
const DALSMC_RESULT_FAILED: u32 = 0xFF;
const DALSMC_RESULT_UNKNOWN_CMD: u32 = 0xFE;
const DALSMC_RESULT_CMD_REJECTED_PREREQ: u32 = 0xFD;
const DALSMC_RESULT_CMD_REJECTED_BUSY: u32 = 0xFC;

const DALSMC_MSG_TEST_MESSAGE: u32 = 0x01;
const DALSMC_MSG_GET_PMFW_VERSION: u32 = 0x02;
const DALSMC_MSG_SET_DISPCLK_FREQ: u32 = 0x03;
const DALSMC_MSG_SET_DPPCLK_FREQ: u32 = 0x04;
const DALSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ: u32 = 0x05;
const DALSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK: u32 = 0x06;
const DALSMC_MSG_UPDATE_PME_RESTORE: u32 = 0x07;
const DALSMC_MSG_SET_DRAM_ADDR_HIGH: u32 = 0x08;
const DALSMC_MSG_SET_DRAM_ADDR_LOW: u32 = 0x09;
const DALSMC_MSG_TRANSFER_TABLE_SMU2DRAM: u32 = 0x0A;
const DALSMC_MSG_TRANSFER_TABLE_DRAM2SMU: u32 = 0x0B;
const DALSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS: u32 = 0x0C;
const DALSMC_MSG_GET_DPREFCLK_FREQ: u32 = 0x0D;
const DALSMC_MSG_GET_DTBCLK_FREQ: u32 = 0x0E;
const DALSMC_MSG_ALLOW_ZSTATES_ENTRY: u32 = 0x0F;
const DALSMC_MSG_SET_DTB_CLK: u32 = 0x10;
const DALSMC_MSG_DISP_IPS2_EXIT: u32 = 0x11;
const DALSMC_MSG_QUERY_IPS2_SUPPORT: u32 = 0x12;
const DALSMC_MSG_DF_CSTATE_DISABLE: u32 = 0x13;
const DALSMC_MESSAGE_COUNT: u32 = 0x14;

#[repr(C)]
pub union Dcn42DpiaHostRouterBw {
    pub bits: Dcn42DpiaHostRouterBwBits,
    pub all: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dcn42DpiaHostRouterBwBits { pub hr_id: u16, pub bw_mbps: u16 }

unsafe fn dcn42_smu_wait_for_response(clk_mgr: *mut clk_mgr_internal, delay_us: u32, mut max_retries: u32) -> u32 {
    let mut res_val: u32;
    loop {
        res_val = REG_READ!(DAL_RESP_REG);
        if res_val != DALSMC_RESULT_CMD_REJECTED_BUSY { break; }
        if delay_us >= 1000 { msleep!(delay_us / 1000); }
        else if delay_us > 0 { udelay!(delay_us); }
        if (*clk_mgr).base.ctx->dc->debug.disable_timeout { max_retries += 1; }
        if max_retries == 0 { break; }
        max_retries -= 1;
    }
    res_val
}

unsafe fn dcn42_smu_send_msg_with_param(clk_mgr: *mut clk_mgr_internal, msg_id: u32, param: u32) -> i32 {
    let mut result = dcn42_smu_wait_for_response(clk_mgr, 10, 2000000);
    if result != DALSMC_RESULT_OK {
        DC_LOG_WARNING!("SMU response after wait: %d, msg id = %d\n", result, msg_id);
        if result == DALSMC_RESULT_CMD_REJECTED_BUSY { return -1; }
    }
    REG_WRITE!(DAL_RESP_REG, DALSMC_RESULT_CMD_REJECTED_BUSY);
    REG_WRITE!(DAL_ARG_REG, param);
    REG_WRITE!(DAL_MSG_REG, msg_id);
    result = dcn42_smu_wait_for_response(clk_mgr, 10, 2000000);
    if result == DALSMC_RESULT_FAILED {
        if msg_id == DALSMC_MSG_TRANSFER_TABLE_DRAM2SMU && param == TABLE_WATERMARKS { DC_LOG_WARNING!("Watermarks table not configured properly by SMU"); }
        REG_WRITE!(DAL_RESP_REG, DALSMC_RESULT_OK);
        DC_LOG_WARNING!("SMU response after wait: %d, msg id = %d\n", result, msg_id);
        return -1;
    }
    if IS_SMU_TIMEOUT!(result) {
        ASSERT!(0);
        result = dcn42_smu_wait_for_response(clk_mgr, 10, 2000000);
        DC_LOG_WARNING!("SMU response after wait: %d, msg id = %d\n", result, msg_id);
    }
    REG_READ!(DAL_ARG_REG) as i32
}

pub unsafe fn dcn42_smu_get_pmfw_version(clk_mgr: *mut clk_mgr_internal) -> i32 { dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GET_PMFW_VERSION, 0) }

pub unsafe fn dcn42_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested_dispclk_khz; }
    let actual = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DISPCLK_FREQ, khz_to_mhz_ceil!(requested_dispclk_khz));
    smu_print!("requested_dispclk_khz = %d, actual_dispclk_set_mhz: %d\n", requested_dispclk_khz, actual);
    actual.wrapping_mul(1000)
}

pub unsafe fn dcn42_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested; }
    let actual = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ, khz_to_mhz_ceil!(requested));
    smu_print!("requested_dcfclk_khz = %d, actual_dcfclk_set_mhz: %d\n", requested, actual); actual.wrapping_mul(1000)
}
pub unsafe fn dcn42_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested; }
    let actual = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK, khz_to_mhz_ceil!(requested));
    smu_print!("requested_min_ds_dcfclk_khz = %d, actual_min_ds_dcfclk_mhz: %d\n", requested, actual); actual.wrapping_mul(1000)
}
pub unsafe fn dcn42_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested; }
    let actual = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DPPCLK_FREQ, khz_to_mhz_ceil!(requested));
    smu_print!("requested_dpp_khz = %d, actual_dppclk_set_mhz: %d\n", requested, actual); actual.wrapping_mul(1000)
}

pub unsafe fn dcn42_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32) {
    if !(*clk_mgr).base.ctx->dc->debug.pstate_enabled || !(*clk_mgr).smu_present { return; }
    dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info);
    smu_print!("%s: SMC_MSG_SetDisplayIdleOptimizations idle_info  = %x\n", __func__, idle_info);
}
pub unsafe fn dcn42_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    if !(*clk_mgr).smu_present { return; }
    let mut idle_info: display_idle_optimization_u = core::mem::zeroed();
    if enable { idle_info.idle_info.df_request_disabled = 1; idle_info.idle_info.phy_ref_clk_off = 1; }
    dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info.data);
    smu_print!("%s smu_enable_phy_refclk_pwrdwn  = %d\n", __func__, if enable { 1 } else { 0 });
}
pub unsafe fn dcn42_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal) {
    if !(*clk_mgr).smu_present { return; }
    dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_UPDATE_PME_RESTORE, 0);
    smu_print!("%s: SMC_MSG_UpdatePmeRestore\n", __func__);
}
pub unsafe fn dcn42_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, value: u32) { if (*clk_mgr).smu_present { dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DRAM_ADDR_HIGH, value); } }
pub unsafe fn dcn42_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, value: u32) { if (*clk_mgr).smu_present { dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DRAM_ADDR_LOW, value); } }
pub unsafe fn dcn42_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_TRANSFER_TABLE_SMU2DRAM, TABLE_DPMCLOCKS); } }
pub unsafe fn dcn42_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_TRANSFER_TABLE_DRAM2SMU, TABLE_WATERMARKS); } }

pub unsafe fn dcn42_smu_get_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    if !(*clk_mgr).smu_present { return 0; }
    let v = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GET_DPREFCLK_FREQ, 0);
    smu_print!("%s:  SMU DPREF clk  = %d mhz\n", __func__, v); v.wrapping_mul(1000)
}
pub unsafe fn dcn42_smu_get_dtbclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    if !(*clk_mgr).smu_present { return 0; }
    let v = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GET_DTBCLK_FREQ, 0);
    smu_print!("%s: get_dtbclk  = %dmhz\n", __func__, v); v.wrapping_mul(1000)
}
pub unsafe fn dcn42_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    if !(*clk_mgr).smu_present { return; }
    dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SET_DTB_CLK, enable as u32);
    smu_print!("%s: smu_set_dtbclk = %d\n", __func__, if enable { 1 } else { 0 });
}

pub unsafe fn dcn42_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state) {
    if !(*clk_mgr).smu_present { return; }
    let (msg_id, param) = match support {
        DCN_ZSTATE_SUPPORT_ALLOW => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, (1 << 10) | (1 << 9) | (1 << 8)),
        DCN_ZSTATE_SUPPORT_DISALLOW => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, 0),
        DCN_ZSTATE_SUPPORT_ALLOW_Z10_ONLY => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, 1 << 10),
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_Z10_ONLY => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, (1 << 10) | (1 << 8)),
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, 1 << 8),
        _ => (DALSMC_MSG_ALLOW_ZSTATES_ENTRY, 0),
    };
    let retv = dcn42_smu_send_msg_with_param(clk_mgr, msg_id, param);
    smu_print!("%s:  msg_id = %d, param = 0x%x, return = 0x%x\n", __func__, msg_id, param, retv);
}

pub unsafe fn dcn42_smu_set_df_cstate_disable(clk_mgr: *mut clk_mgr_internal, disable: bool) -> bool {
    if !(*clk_mgr).smu_present { return true; }
    let retv = dcn42_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_DF_CSTATE_DISABLE, if disable { 1 } else { 0 });
    smu_print!("%s: DfCstateDisable param = %d, return = %d\n", __func__, if disable { 1 } else { 0 }, retv);
    retv != -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
