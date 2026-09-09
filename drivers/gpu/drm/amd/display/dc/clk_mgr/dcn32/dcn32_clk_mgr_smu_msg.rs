/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

const MM_DAL_MSG_REG: u32 = 0x1628A;
const MM_DAL_ARG_REG: u32 = 0x16273;
const MM_DAL_RESP_REG: u32 = 0x16274;

/* External symbols and register helpers are supplied by the translated dependencies. */
extern "C" {
    fn REG_READ(reg: u32) -> u32;
    fn REG_WRITE(reg: u32, value: u32);
    fn msleep(milliseconds: u32);
    fn udelay(microseconds: u32);
    fn TRACE_SMU_MSG_DELAY(a: u32, b: u32, delay: u32, ctx: *mut core::ffi::c_void);
    fn TRACE_SMU_MSG(msg_id: u32, param: u32, ctx: *mut core::ffi::c_void);
    fn DC_LOG_SMU(format: *const core::ffi::c_char, ...);
    fn ASICREV_IS_GC_11_0_0(rev: u32) -> bool;
    fn ASICREV_IS_GC_11_0_2(rev: u32) -> bool;
}

#[allow(non_snake_case)]
unsafe fn smu_print(_format: *const core::ffi::c_char) {}

unsafe fn dcn32_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let initial_max_retries = max_retries;
    let mut reg = 0u32;

    loop {
        reg = REG_READ(MM_DAL_RESP_REG);
        if reg != 0 { break; }

        if delay_us >= 1000 {
            msleep(delay_us / 1000);
        } else if delay_us > 0 {
            udelay(delay_us);
        }
        let old = max_retries;
        max_retries = max_retries.wrapping_sub(1);
        if old == 0 { break; }
    }

    TRACE_SMU_MSG_DELAY(0, 0, delay_us * (initial_max_retries - max_retries), (*clk_mgr).base.ctx);
    reg
}

unsafe fn dcn32_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal, msg_id: u32, param_in: u32, param_out: *mut u32,
) -> bool {
    dcn32_smu_wait_for_response(clk_mgr, 10, 200000);
    REG_WRITE(MM_DAL_RESP_REG, 0);
    REG_WRITE(MM_DAL_ARG_REG, param_in);
    REG_WRITE(MM_DAL_MSG_REG, msg_id);
    TRACE_SMU_MSG(msg_id, param_in, (*clk_mgr).base.ctx);
    if dcn32_smu_wait_for_response(clk_mgr, 10, 200000) == DALSMC_Result_OK {
        if !param_out.is_null() { *param_out = REG_READ(MM_DAL_ARG_REG); }
        return true;
    }
    false
}

unsafe fn dcn32_smu_wait_for_response_delay(
    clk_mgr: *mut clk_mgr_internal, delay_us: u32, mut max_retries: u32,
    total_delay_us: *mut u32,
) -> u32 {
    let mut reg = 0u32;
    *total_delay_us = 0;
    loop {
        reg = REG_READ(MM_DAL_RESP_REG);
        if reg != 0 { break; }
        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        *total_delay_us += delay_us;
        let old = max_retries;
        max_retries = max_retries.wrapping_sub(1);
        if old == 0 { break; }
    }
    TRACE_SMU_MSG_DELAY(0, 0, *total_delay_us, (*clk_mgr).base.ctx);
    reg
}

unsafe fn dcn32_smu_send_msg_with_param_delay(
    clk_mgr: *mut clk_mgr_internal, msg_id: u32, param_in: u32, param_out: *mut u32,
    total_delay_us: *mut u32,
) -> bool {
    let mut delay1_us = 0u32;
    let mut delay2_us = 0u32;
    *total_delay_us = 0;
    dcn32_smu_wait_for_response_delay(clk_mgr, 10, 200000, &mut delay1_us);
    REG_WRITE(MM_DAL_RESP_REG, 0);
    REG_WRITE(MM_DAL_ARG_REG, param_in);
    REG_WRITE(MM_DAL_MSG_REG, msg_id);
    TRACE_SMU_MSG(msg_id, param_in, (*clk_mgr).base.ctx);
    if dcn32_smu_wait_for_response_delay(clk_mgr, 10, 200000, &mut delay2_us) == DALSMC_Result_OK {
        if !param_out.is_null() { *param_out = REG_READ(MM_DAL_ARG_REG); }
        *total_delay_us = delay1_us + delay2_us;
        return true;
    }
    *total_delay_us = delay1_us + 2000000;
    false
}

pub unsafe fn dcn32_smu_send_fclk_pstate_message(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    dcn32_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetFclkSwitchAllow,
        if enable { FCLK_PSTATE_SUPPORTED } else { FCLK_PSTATE_NOTSUPPORTED }, core::ptr::null_mut());
}

pub unsafe fn dcn32_smu_send_cab_for_uclk_message(clk_mgr: *mut clk_mgr_internal, num_ways: u32) {
    let param = (num_ways << 1) | (num_ways > 0) as u32;
    dcn32_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetCabForUclkPstate, param, core::ptr::null_mut());
}

pub unsafe fn dcn32_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) {
    dcn32_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_TransferTableDram2Smu, TABLE_WATERMARKS, core::ptr::null_mut());
}

pub unsafe fn dcn32_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal) {
    dcn32_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_BacoAudioD3PME, 0, core::ptr::null_mut());
}

unsafe fn dcn32_get_hard_min_status_supported(clk_mgr: *mut clk_mgr_internal) -> bool {
    if ASICREV_IS_GC_11_0_0((*clk_mgr).base.ctx->asic_id.hw_internal_rev) {
        if (*clk_mgr).smu_ver >= 0x4e6a00 { return true; }
    } else if ASICREV_IS_GC_11_0_2((*clk_mgr).base.ctx->asic_id.hw_internal_rev) {
        if (*clk_mgr).smu_ver >= 0x524e00 { return true; }
    } else if (*clk_mgr).smu_ver >= 0x503900 { return true; }
    false
}

unsafe fn dcn32_smu_get_hard_min_status(clk_mgr: *mut clk_mgr_internal, no_timeout: *mut bool, total_delay_us: *mut u32) -> u32 {
    let mut response = 0u32;
    *no_timeout = dcn32_smu_send_msg_with_param_delay(clk_mgr, DALSMC_MSG_ReturnHardMinStatus, 0, &mut response, total_delay_us);
    response
}

unsafe fn dcn32_smu_wait_get_hard_min_status(clk_mgr: *mut clk_mgr_internal, clk: u32) -> bool {
    let mut check = CHECK_HARD_MIN_CLK_DPREFCLK;
    if clk == PPCLK_DISPCLK { check |= CHECK_HARD_MIN_CLK_DISPCLK; }
    if clk == PPCLK_DPPCLK { check |= CHECK_HARD_MIN_CLK_DPPCLK; }
    if clk == PPCLK_DCFCLK { check |= CHECK_HARD_MIN_CLK_DCFCLK; }
    if clk == PPCLK_DTBCLK { check |= CHECK_HARD_MIN_CLK_DTBCLK; }
    if clk == PPCLK_UCLK { check |= CHECK_HARD_MIN_CLK_UCLK; }
    if check == CHECK_HARD_MIN_CLK_DPREFCLK { return false; }
    let mut total = 0u32;
    loop {
        let mut no_timeout = false; let mut read_delay = 0u32;
        let read = dcn32_smu_get_hard_min_status(clk_mgr, &mut no_timeout, &mut read_delay);
        total += read_delay;
        if check == (read & check) { return true; }
        if total >= 2000000 { break; }
        msleep(1); total += 1000;
    }
    false
}

pub unsafe fn dcn32_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> u32 {
    let mut response = 0u32;
    let param = (clk << 16) | freq_mhz as u32;
    dcn32_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetHardMinByFreq, param, &mut response);
    if dcn32_get_hard_min_status_supported(clk_mgr) { let _ = dcn32_smu_wait_get_hard_min_status(clk_mgr, clk); }
    response
}

pub unsafe fn dcn32_smu_wait_for_dmub_ack_mclk(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    dcn32_smu_send_msg_with_param(clk_mgr, 0x14, if enable { 1 } else { 0 }, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
