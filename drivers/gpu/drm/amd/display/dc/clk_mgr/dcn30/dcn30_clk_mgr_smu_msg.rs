/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies are supplied by the surrounding translation unit.

const MM_DAL_MSG_REG: u32 = 0x1628a;
const MM_DAL_ARG_REG: u32 = 0x16273;
const MM_DAL_RESP_REG: u32 = 0x16274;

#[repr(C)]
pub struct clk_mgr_internal {
    pub base: clk_mgr_base,
}
#[repr(C)]
pub struct clk_mgr_base {
    pub ctx: *mut core::ffi::c_void,
}

extern "C" {
    fn REG_READ(reg: u32) -> u32;
    fn REG_WRITE(reg: u32, value: u32);
    fn msleep(value: u32);
    fn udelay(value: u32);
    fn TRACE_SMU_MSG_DELAY(a: u32, b: u32, delay: u32, ctx: *mut core::ffi::c_void);
    fn TRACE_SMU_MSG(msg: u32, param: u32, ctx: *mut core::ffi::c_void);
    fn dm_helpers_smu_timeout(ctx: *mut core::ffi::c_void, msg: u32, param: u32, timeout: u32);
    fn DC_LOG_SMU(format: *const core::ffi::c_char, ...);
}

extern "C" {
    static DALSMC_Result_OK: u32;
    static DALSMC_MSG_TestMessage: u32;
    static DALSMC_MSG_GetSmuVersion: u32;
    static DALSMC_MSG_GetDriverIfVersion: u32;
    static DALSMC_MSG_GetMsgHeaderVersion: u32;
    static DALSMC_MSG_SetDalDramAddrHigh: u32;
    static DALSMC_MSG_SetDalDramAddrLow: u32;
    static DALSMC_MSG_TransferTableSmu2Dram: u32;
    static DALSMC_MSG_TransferTableDram2Smu: u32;
    static DALSMC_MSG_SetHardMinByFreq: u32;
    static DALSMC_MSG_SetHardMaxByFreq: u32;
    static DALSMC_MSG_GetDpmFreqByIndex: u32;
    static DALSMC_MSG_GetDcModeMaxDpmFreq: u32;
    static DALSMC_MSG_SetMinDeepSleepDcefclk: u32;
    static DALSMC_MSG_NumOfDisplays: u32;
    static DALSMC_MSG_SetDisplayRefreshFromMall: u32;
    static DALSMC_MSG_SetExternalClientDfCstateAllow: u32;
    static DALSMC_MSG_BacoAudioD3PME: u32;
    static SMU11_DRIVER_IF_VERSION: u32;
    static DALSMC_VERSION: u32;
    static TABLE_WATERMARKS: u32;
}

#[inline]
unsafe fn dcn30_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let initial_max_retries = max_retries;
    let mut reg = 0u32;
    loop {
        reg = REG_READ(MM_DAL_RESP_REG);
        if reg != 0 { break; }
        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        let old = max_retries;
        max_retries = max_retries.wrapping_sub(1);
        if old == 0 { break; }
    }
    TRACE_SMU_MSG_DELAY(0, 0, delay_us.wrapping_mul(initial_max_retries.wrapping_sub(max_retries)), (*clk_mgr).base.ctx);
    reg
}

unsafe fn dcn30_smu_send_msg_with_param(clk_mgr: *mut clk_mgr_internal, msg_id: u32, param_in: u32, param_out: *mut u32) -> bool {
    dcn30_smu_wait_for_response(clk_mgr, 10, 200000);
    REG_WRITE(MM_DAL_RESP_REG, 0);
    REG_WRITE(MM_DAL_ARG_REG, param_in);
    REG_WRITE(MM_DAL_MSG_REG, msg_id);
    TRACE_SMU_MSG(msg_id, param_in, (*clk_mgr).base.ctx);
    let result = dcn30_smu_wait_for_response(clk_mgr, 10, 200000);
    if result == 0xffff_ffff { dm_helpers_smu_timeout((*clk_mgr).base.ctx, msg_id, param_in, 10 * 200000); }
    if result == DALSMC_Result_OK {
        if !param_out.is_null() { *param_out = REG_READ(MM_DAL_ARG_REG); }
        return true;
    }
    false
}

pub unsafe fn dcn30_smu_test_message(clk_mgr: *mut clk_mgr_internal, input: u32) -> bool {
    let mut response = 0u32;
    if dcn30_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_TestMessage, input, &mut response) && response == input.wrapping_add(1) { return true; }
    false
}

pub unsafe fn dcn30_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal, version: *mut i32) -> bool {
    if dcn30_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetSmuVersion, 0, version as *mut u32) { return true; }
    false
}

pub unsafe fn dcn30_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool {
    let mut response = 0u32;
    dcn30_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetDriverIfVersion, 0, &mut response) && response == SMU11_DRIVER_IF_VERSION
}

pub unsafe fn dcn30_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool {
    let mut response = 0u32;
    dcn30_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetMsgHeaderVersion, 0, &mut response) && response == DALSMC_VERSION
}

pub unsafe fn dcn30_smu_set_dram_addr_high(c: *mut clk_mgr_internal, v: u32) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetDalDramAddrHigh, v, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_set_dram_addr_low(c: *mut clk_mgr_internal, v: u32) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetDalDramAddrLow, v, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_transfer_wm_table_smu_2_dram(c: *mut clk_mgr_internal) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_TransferTableSmu2Dram, TABLE_WATERMARKS, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_transfer_wm_table_dram_2_smu(c: *mut clk_mgr_internal) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_TransferTableDram2Smu, TABLE_WATERMARKS, core::ptr::null_mut()); }

pub unsafe fn dcn30_smu_set_hard_min_by_freq(c: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> u32 {
    let mut response = 0; let param = (clk << 16) | freq_mhz as u32;
    dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetHardMinByFreq, param, &mut response); response
}
pub unsafe fn dcn30_smu_set_hard_max_by_freq(c: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> u32 {
    let mut response = 0; let param = (clk << 16) | freq_mhz as u32;
    dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetHardMaxByFreq, param, &mut response); response
}
pub unsafe fn dcn30_smu_get_dpm_freq_by_index(c: *mut clk_mgr_internal, clk: u32, level: u8) -> u32 {
    let mut response = 0; dcn30_smu_send_msg_with_param(c, DALSMC_MSG_GetDpmFreqByIndex, (clk << 16) | level as u32, &mut response); response
}
pub unsafe fn dcn30_smu_get_dc_mode_max_dpm_freq(c: *mut clk_mgr_internal, clk: u32) -> u32 {
    let mut response = 0; dcn30_smu_send_msg_with_param(c, DALSMC_MSG_GetDcModeMaxDpmFreq, clk << 16, &mut response); response
}
pub unsafe fn dcn30_smu_set_min_deep_sleep_dcef_clk(c: *mut clk_mgr_internal, freq: u32) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetMinDeepSleepDcefclk, freq, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_set_num_of_displays(c: *mut clk_mgr_internal, n: u32) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_NumOfDisplays, n, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_set_display_refresh_from_mall(c: *mut clk_mgr_internal, enable: bool, delay: u8, scale: u8) { let p = ((scale as u32) << 7) | ((delay as u32) << 1) | enable as u32; dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetDisplayRefreshFromMall, p, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_set_external_client_df_cstate_allow(c: *mut clk_mgr_internal, enable: bool) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_SetExternalClientDfCstateAllow, enable as u32, core::ptr::null_mut()); }
pub unsafe fn dcn30_smu_set_pme_workaround(c: *mut clk_mgr_internal) { dcn30_smu_send_msg_with_param(c, DALSMC_MSG_BacoAudioD3PME, 0, core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
