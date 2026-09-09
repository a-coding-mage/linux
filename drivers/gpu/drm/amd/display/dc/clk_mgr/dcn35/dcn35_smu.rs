/* Copyright 2022 Advanced Micro Devices, Inc. */
/* SPDX-License-Identifier: MIT */

// External driver/kernel declarations supplied by the surrounding translation unit.
use core::ffi::c_int;

const MP1_BASE__INST0_SEG0: u32 = 0x00016000;
const MP1_BASE__INST0_SEG1: u32 = 0x0243FC00;
const MP1_BASE__INST0_SEG2: u32 = 0x00DC0000;
const MP1_BASE__INST0_SEG3: u32 = 0x00E00000;
const MP1_BASE__INST0_SEG4: u32 = 0x00E40000;
const MP1_BASE__INST0_SEG5: u32 = 0;

const VBIOSSMC_MSG_TestMessage: u32 = 0x1;
const VBIOSSMC_MSG_GetSmuVersion: u32 = 0x2;
const VBIOSSMC_MSG_PowerUpGfx: u32 = 0x3;
const VBIOSSMC_MSG_SetDispclkFreq: u32 = 0x4;
const VBIOSSMC_MSG_SetDprefclkFreq: u32 = 0x5;
const VBIOSSMC_MSG_SetDppclkFreq: u32 = 0x6;
const VBIOSSMC_MSG_SetHardMinDcfclkByFreq: u32 = 0x7;
const VBIOSSMC_MSG_SetMinDeepSleepDcfclk: u32 = 0x8;
const VBIOSSMC_MSG_SetPhyclkVoltageByFreq: u32 = 0x9;
const VBIOSSMC_MSG_GetFclkFrequency: u32 = 0xA;
const VBIOSSMC_MSG_SetDisplayCount: u32 = 0xB;
const VBIOSSMC_MSG_EnableTmdp48MHzRefclkPwrDown: u32 = 0xC;
const VBIOSSMC_MSG_UpdatePmeRestore: u32 = 0xD;
const VBIOSSMC_MSG_SetVbiosDramAddrHigh: u32 = 0xE;
const VBIOSSMC_MSG_SetVbiosDramAddrLow: u32 = 0xF;
const VBIOSSMC_MSG_TransferTableSmu2Dram: u32 = 0x10;
const VBIOSSMC_MSG_TransferTableDram2Smu: u32 = 0x11;
const VBIOSSMC_MSG_SetDisplayIdleOptimizations: u32 = 0x12;
const VBIOSSMC_MSG_GetDprefclkFreq: u32 = 0x13;
const VBIOSSMC_MSG_GetDtbclkFreq: u32 = 0x14;
const VBIOSSMC_MSG_AllowZstatesEntry: u32 = 0x15;
const VBIOSSMC_MSG_DisallowZstatesEntry: u32 = 0x16;
const VBIOSSMC_MSG_SetDtbClk: u32 = 0x17;
const VBIOSSMC_MSG_DispIPS2Entry: u32 = 0x18;
const VBIOSSMC_MSG_DispIPS2Exit: u32 = 0x19;
const VBIOSSMC_MSG_DisableLSdma: u32 = 0x1A;
const VBIOSSMC_MSG_DpControllerPhyStatus: u32 = 0x1B;
const VBIOSSMC_MSG_QueryIPS2Support: u32 = 0x1C;
const VBIOSSMC_MSG_NotifyHostRouterBW: u32 = 0x1D;
const VBIOSSMC_Message_Count: u32 = 0x1E;
const VBIOSSMC_Status_BUSY: u32 = 0x0;
const VBIOSSMC_Result_OK: u32 = 0x1;
const VBIOSSMC_Result_Failed: u32 = 0xFF;
const VBIOSSMC_Result_UnknownCmd: u32 = 0xFE;
const VBIOSSMC_Result_CmdRejectedPrereq: u32 = 0xFD;
const VBIOSSMC_Result_CmdRejectedBusy: u32 = 0xFC;

#[repr(C)]
pub union dcn35_dpia_host_router_bw {
    pub bits: dcn35_dpia_host_router_bw_bits,
    pub all: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn35_dpia_host_router_bw_bits { pub hr_id: u16, pub bw_mbps: u16 }

extern "C" {
    fn REG_READ(reg: u32) -> u32;
    fn REG_WRITE(reg: u32, value: u32);
    fn msleep(ms: u32);
    fn udelay(us: u32);
    fn khz_to_mhz_ceil(khz: c_int) -> u32;
    fn ASSERT(condition: bool);
    fn IS_SMU_TIMEOUT(value: u32) -> bool;
    fn DC_LOG_WARNING(fmt: *const i8, ...);
    fn smu_print(fmt: *const i8, ...);
}

// Register identifiers are provided by the generated register headers.
extern "C" {
    static MP1_SMN_C2PMSG_91: u32;
    static MP1_SMN_C2PMSG_83: u32;
    static MP1_SMN_C2PMSG_67: u32;
}

#[inline]
unsafe fn dcn35_smu_wait_for_response(clk_mgr: *mut clk_mgr_internal, delay_us: u32, mut max_retries: u32) -> u32 {
    let mut res_val = VBIOSSMC_Status_BUSY;
    loop {
        res_val = REG_READ(MP1_SMN_C2PMSG_91);
        if res_val != VBIOSSMC_Status_BUSY { break; }
        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        if (*clk_mgr).base.ctx->dc->debug.disable_timeout { max_retries = max_retries.wrapping_add(1); }
        if max_retries == 0 { break; }
        max_retries = max_retries.wrapping_sub(1);
    }
    res_val
}

unsafe fn dcn35_smu_send_msg_with_param(clk_mgr: *mut clk_mgr_internal, msg_id: u32, param: u32) -> c_int {
    let mut result = dcn35_smu_wait_for_response(clk_mgr, 10, 2_000_000);
    ASSERT(result == VBIOSSMC_Result_OK);
    if result != VBIOSSMC_Result_OK {
        if result == VBIOSSMC_Status_BUSY { return -1; }
    }
    REG_WRITE(MP1_SMN_C2PMSG_91, VBIOSSMC_Status_BUSY);
    REG_WRITE(MP1_SMN_C2PMSG_83, param);
    REG_WRITE(MP1_SMN_C2PMSG_67, msg_id);
    result = dcn35_smu_wait_for_response(clk_mgr, 10, 2_000_000);
    if result == VBIOSSMC_Result_Failed {
        if !(msg_id == VBIOSSMC_MSG_TransferTableDram2Smu && param == TABLE_WATERMARKS) { ASSERT(false); }
        REG_WRITE(MP1_SMN_C2PMSG_91, VBIOSSMC_Result_OK);
        return -1;
    }
    if IS_SMU_TIMEOUT(result) { ASSERT(false); result = dcn35_smu_wait_for_response(clk_mgr, 10, 2_000_000); }
    REG_READ(MP1_SMN_C2PMSG_83) as c_int
}

pub unsafe fn dcn35_smu_get_smu_version(m: *mut clk_mgr_internal) -> c_int { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_GetSmuVersion, 0) }
pub unsafe fn dcn35_smu_set_dispclk(m: *mut clk_mgr_internal, requested: c_int) -> c_int { if !(*m).smu_present { return requested; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDispclkFreq, khz_to_mhz_ceil(requested)) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_set_dprefclk(m: *mut clk_mgr_internal) -> c_int { if !(*m).smu_present { return (*m).base.dprefclk_khz; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDprefclkFreq, khz_to_mhz_ceil((*m).base.dprefclk_khz)) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_set_hard_min_dcfclk(m: *mut clk_mgr_internal, requested: c_int) -> c_int { if !(*m).smu_present { return requested; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetHardMinDcfclkByFreq, khz_to_mhz_ceil(requested)) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_set_min_deep_sleep_dcfclk(m: *mut clk_mgr_internal, requested: c_int) -> c_int { if !(*m).smu_present { return requested; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetMinDeepSleepDcfclk, khz_to_mhz_ceil(requested)) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_set_dppclk(m: *mut clk_mgr_internal, requested: c_int) -> c_int { if !(*m).smu_present { return requested; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDppclkFreq, khz_to_mhz_ceil(requested)) as i64 * 1000) as c_int }

// The remaining declarations and structure definitions are supplied by dcn35_smu.h.
// Function bodies below retain the original driver-visible behavior.
pub unsafe fn dcn35_smu_set_display_idle_optimization(m: *mut clk_mgr_internal, idle_info: u32) { if !(*m).base.ctx->dc->debug.pstate_enabled || !(*m).smu_present { return; } dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDisplayIdleOptimizations, idle_info); }
pub unsafe fn dcn35_smu_enable_phy_refclk_pwrdwn(m: *mut clk_mgr_internal, enable: bool) { if !(*m).smu_present { return; } let data = if enable { (1u32 << 0) | (1u32 << 1) } else { 0 }; dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDisplayIdleOptimizations, data); }
pub unsafe fn dcn35_smu_enable_pme_wa(m: *mut clk_mgr_internal) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_UpdatePmeRestore, 0); } }
pub unsafe fn dcn35_smu_set_dram_addr_high(m: *mut clk_mgr_internal, v: u32) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetVbiosDramAddrHigh, v); } }
pub unsafe fn dcn35_smu_set_dram_addr_low(m: *mut clk_mgr_internal, v: u32) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetVbiosDramAddrLow, v); } }
pub unsafe fn dcn35_smu_transfer_dpm_table_smu_2_dram(m: *mut clk_mgr_internal) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_TransferTableSmu2Dram, TABLE_DPMCLOCKS); } }
pub unsafe fn dcn35_smu_transfer_wm_table_dram_2_smu(m: *mut clk_mgr_internal) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_TransferTableDram2Smu, TABLE_WATERMARKS); } }

pub unsafe fn dcn35_smu_set_zstate_support(m: *mut clk_mgr_internal, support: enum_dcn_zstate_support_state) {
    if !(*m).smu_present { return; }
    let param = match support {
        DCN_ZSTATE_SUPPORT_ALLOW => (1u32 << 10) | (1u32 << 9) | (1u32 << 8),
        DCN_ZSTATE_SUPPORT_ALLOW_Z10_ONLY => 1u32 << 10,
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_Z10_ONLY => (1u32 << 10) | (1u32 << 8),
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY => 1u32 << 8,
        DCN_ZSTATE_SUPPORT_DISALLOW => 0,
        _ => 0,
    };
    dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_AllowZstatesEntry, param);
}

pub unsafe fn dcn35_smu_get_dprefclk(m: *mut clk_mgr_internal) -> c_int { if !(*m).smu_present { return 0; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_GetDprefclkFreq, 0) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_get_dtbclk(m: *mut clk_mgr_internal) -> c_int { if !(*m).smu_present { return 0; } (dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_GetDtbclkFreq, 0) as i64 * 1000) as c_int }
pub unsafe fn dcn35_smu_set_dtbclk(m: *mut clk_mgr_internal, enable: bool) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_SetDtbClk, enable as u32); } }
pub unsafe fn dcn35_vbios_smu_enable_48mhz_tmdp_refclk_pwrdwn(m: *mut clk_mgr_internal, enable: bool) { if (*m).smu_present { dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_EnableTmdp48MHzRefclkPwrDown, enable as u32); } }
pub unsafe fn dcn35_smu_exit_low_power_state(m: *mut clk_mgr_internal) -> c_int { if !(*m).smu_present { return 0; } dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_DispIPS2Exit, 0) }
pub unsafe fn dcn35_smu_get_ips_supported(m: *mut clk_mgr_internal) -> c_int { if !(*m).smu_present { return 0; } dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_QueryIPS2Support, 0) }
pub unsafe fn dcn35_smu_notify_host_router_bw(m: *mut clk_mgr_internal, hr_id: u32, bw_kbps: u32) { let data = (hr_id & 0xffff) | (((bw_kbps / 1000) & 0xffff) << 16); dcn35_smu_send_msg_with_param(m, VBIOSSMC_MSG_NotifyHostRouterBW, data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
