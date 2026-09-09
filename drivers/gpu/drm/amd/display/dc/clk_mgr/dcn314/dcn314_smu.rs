// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translated kernel/display tree.

const MP1_BASE_INST0_SEG0: u32 = 0x00016000;
const MP1_BASE_INST0_SEG1: u32 = 0x0243FC00;
const MP1_BASE_INST0_SEG2: u32 = 0x00DC0000;
const MP1_BASE_INST0_SEG3: u32 = 0x00E00000;
const MP1_BASE_INST0_SEG4: u32 = 0x00E40000;
const MP1_BASE_INST0_SEG5: u32 = 0;

const VBIOSSMC_MSG_TEST_MESSAGE: u32 = 0x1;
const VBIOSSMC_MSG_GET_SMU_VERSION: u32 = 0x2;
const VBIOSSMC_MSG_POWER_UP_GFX: u32 = 0x3;
const VBIOSSMC_MSG_SET_DISPCLK_FREQ: u32 = 0x4;
const VBIOSSMC_MSG_SET_DPREFCLK_FREQ: u32 = 0x5;
const VBIOSSMC_MSG_SET_DPPCLK_FREQ: u32 = 0x6;
const VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ: u32 = 0x7;
const VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK: u32 = 0x8;
const VBIOSSMC_MSG_SET_PHYCLK_VOLTAGE_BY_FREQ: u32 = 0x9;
const VBIOSSMC_MSG_GET_FCLK_FREQUENCY: u32 = 0xA;
const VBIOSSMC_MSG_SET_DISPLAY_COUNT: u32 = 0xB;
const VBIOSSMC_MSG_ENABLE_TMDP48_MHZ_REFCLK_PWR_DOWN: u32 = 0xC;
const VBIOSSMC_MSG_UPDATE_PME_RESTORE: u32 = 0xD;
const VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_HIGH: u32 = 0xE;
const VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_LOW: u32 = 0xF;
const VBIOSSMC_MSG_TRANSFER_TABLE_SMU2DRAM: u32 = 0x10;
const VBIOSSMC_MSG_TRANSFER_TABLE_DRAM2SMU: u32 = 0x11;
const VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS: u32 = 0x12;
const VBIOSSMC_MSG_GET_DPREFCLK_FREQ: u32 = 0x13;
const VBIOSSMC_MSG_GET_DTBCLK_FREQ: u32 = 0x14;
const VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY: u32 = 0x15;
const VBIOSSMC_MSG_DISALLOW_ZSTATES_ENTRY: u32 = 0x16;
const VBIOSSMC_MSG_SET_DTB_CLK: u32 = 0x17;
const VBIOSSMC_MESSAGE_COUNT: u32 = 0x18;

const VBIOSSMC_STATUS_BUSY: u32 = 0x0;
const VBIOSSMC_RESULT_OK: u32 = 0x1;
const VBIOSSMC_RESULT_FAILED: u32 = 0xFF;
const VBIOSSMC_RESULT_UNKNOWN_CMD: u32 = 0xFE;
const VBIOSSMC_RESULT_CMD_REJECTED_PREREQ: u32 = 0xFD;
const VBIOSSMC_RESULT_CMD_REJECTED_BUSY: u32 = 0xFC;

unsafe fn dcn314_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let mut res_val = VBIOSSMC_STATUS_BUSY;
    loop {
        res_val = REG_READ(clk_mgr, MP1_SMN_C2PMSG_91);
        if res_val != VBIOSSMC_STATUS_BUSY { break; }
        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        if max_retries == 0 { break; }
        max_retries = max_retries.wrapping_sub(1);
    }
    res_val
}

unsafe fn dcn314_smu_send_msg_with_param(clk_mgr: *mut clk_mgr_internal, msg_id: u32, param: u32) -> i32 {
    let mut result = dcn314_smu_wait_for_response(clk_mgr, 10, 200000);
    if result != VBIOSSMC_RESULT_OK { smu_print!("SMU Response was not OK. SMU response after wait received is: %d\n", result); }
    if result == VBIOSSMC_STATUS_BUSY { return -1; }
    REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_91, VBIOSSMC_STATUS_BUSY);
    REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_83, param);
    REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_67, msg_id);
    result = dcn314_smu_wait_for_response(clk_mgr, 10, 200000);
    if result == VBIOSSMC_RESULT_FAILED {
        if msg_id == VBIOSSMC_MSG_TRANSFER_TABLE_DRAM2SMU && param == TABLE_WATERMARKS { DC_LOG_DEBUG!("Watermarks table not configured properly by SMU"); }
        else if msg_id == VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ || msg_id == VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK { DC_LOG_WARNING!("DCFCLK_DPM is not enabled by BIOS"); }
        else { ASSERT!(false); }
        REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_91, VBIOSSMC_RESULT_OK);
        return -1;
    }
    if IS_SMU_TIMEOUT!(result) { ASSERT!(false); dm_helpers_smu_timeout((*clk_mgr).base.ctx, msg_id, param, 10 * 200000); }
    REG_READ(clk_mgr, MP1_SMN_C2PMSG_83) as i32
}

pub unsafe fn dcn314_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32 { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GET_SMU_VERSION, 0) }

pub unsafe fn dcn314_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested_dispclk_khz; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPCLK_FREQ, khz_to_mhz_ceil(requested_dispclk_khz)) * 1000
}

pub unsafe fn dcn314_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    if !(*clk_mgr).smu_present { return (*clk_mgr).base.dprefclk_khz; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DPREFCLK_FREQ, khz_to_mhz_ceil((*clk_mgr).base.dprefclk_khz)) * 1000
}

pub unsafe fn dcn314_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32 {
    if !(*clk_mgr).base.ctx.dc.debug.pstate_enabled { return -1; }
    if !(*clk_mgr).smu_present { return requested_dcfclk_khz; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ, khz_to_mhz_ceil(requested_dcfclk_khz)) * 1000
}

pub unsafe fn dcn314_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32 {
    if !(*clk_mgr).base.ctx.dc.debug.pstate_enabled { return -1; }
    if !(*clk_mgr).smu_present { return requested_min_ds_dcfclk_khz; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK, khz_to_mhz_ceil(requested_min_ds_dcfclk_khz)) * 1000
}

pub unsafe fn dcn314_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested_dpp_khz; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DPPCLK_FREQ, khz_to_mhz_ceil(requested_dpp_khz)) * 1000
}

pub unsafe fn dcn314_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32) {
    if !(*clk_mgr).base.ctx.dc.debug.pstate_enabled || !(*clk_mgr).smu_present { return; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info);
}

pub unsafe fn dcn314_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    if !(*clk_mgr).smu_present { return; }
    let mut idle_info: display_idle_optimization_u = core::mem::zeroed();
    if enable { idle_info.idle_info.df_request_disabled = 1; idle_info.idle_info.phy_ref_clk_off = 1; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info.data);
}

pub unsafe fn dcn314_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_UPDATE_PME_RESTORE, 0); } }
pub unsafe fn dcn314_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32) { if (*clk_mgr).smu_present { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_HIGH, addr_high); } }
pub unsafe fn dcn314_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32) { if (*clk_mgr).smu_present { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_LOW, addr_low); } }
pub unsafe fn dcn314_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TRANSFER_TABLE_SMU2DRAM, TABLE_DPMCLOCKS); } }
pub unsafe fn dcn314_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TRANSFER_TABLE_DRAM2SMU, TABLE_WATERMARKS); } }

pub unsafe fn dcn314_smu_set_zstate_support(clk_mgr: *mut clk_mgr_internal, support: dcn_zstate_support_state) {
    if !(*clk_mgr).smu_present { return; }
    let (msg_id, param) = match support {
        DCN_ZSTATE_SUPPORT_ALLOW => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, (1 << 10) | (1 << 9) | (1 << 8)),
        DCN_ZSTATE_SUPPORT_DISALLOW => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, 0),
        DCN_ZSTATE_SUPPORT_ALLOW_Z10_ONLY => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, 1 << 10),
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_Z10_ONLY => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, (1 << 10) | (1 << 8)),
        DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, 1 << 8),
        _ => (VBIOSSMC_MSG_ALLOW_ZSTATES_ENTRY, 0),
    };
    dcn314_smu_send_msg_with_param(clk_mgr, msg_id, param);
}

/* Arg = 1: Turn DTB on; 0: Turn DTB CLK OFF. when it is on, it is 600MHZ */
pub unsafe fn dcn314_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    if !(*clk_mgr).smu_present { return; }
    dcn314_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DTB_CLK, enable as u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
