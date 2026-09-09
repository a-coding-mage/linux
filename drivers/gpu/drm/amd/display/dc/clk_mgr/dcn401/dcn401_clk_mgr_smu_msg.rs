// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dcn401_clk_mgr_smu_msg.h, clk_mgr_internal.h, reg_helper.h, dalsmc.h,
// dcn401_smu14_driver_if.h, and logger_types.h.

type c_uint = u32;
type c_int = i32;

const MM_DAL_MSG_REG: u32 = 0x1628A;
const MM_DAL_ARG_REG: u32 = 0x16273;
const MM_DAL_RESP_REG: u32 = 0x16274;

// temporary defines; retain the source names used by the external interface.
const DALSMC_MSG_SubvpUclkFclk: u32 = 0x1B;
const DALSMC_MSG_GetNumUmcChannels: u32 = 0x1C;

unsafe fn dcn401_smu_wait_for_response(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: c_uint,
    mut max_retries: c_uint,
) -> u32 {
    let mut reg: u32 = 0;

    loop {
        reg = REG_READ(DAL_RESP_REG);
        if reg != 0 {
            break;
        }

        if delay_us >= 1000 {
            msleep(delay_us / 1000);
        } else if delay_us > 0 {
            udelay(delay_us);
        }

        let old = max_retries;
        max_retries = max_retries.wrapping_sub(1);
        if old == 0 {
            break;
        }
    }

    reg
}

unsafe fn dcn401_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param_in: u32,
    param_out: *mut u32,
) -> bool {
    dcn401_smu_wait_for_response(clk_mgr, 10, 200000);

    TRACE_SMU_MSG_ENTER(msg_id, param_in, (*clk_mgr).base.ctx);
    REG_WRITE(DAL_RESP_REG, 0);
    REG_WRITE(DAL_ARG_REG, param_in);
    REG_WRITE(DAL_MSG_REG, msg_id);

    if dcn401_smu_wait_for_response(clk_mgr, 10, 200000) == DALSMC_Result_OK {
        if !param_out.is_null() {
            *param_out = REG_READ(DAL_ARG_REG);
        }
        TRACE_SMU_MSG_EXIT(true, if !param_out.is_null() { *param_out } else { 0 }, (*clk_mgr).base.ctx);
        return true;
    }

    TRACE_SMU_MSG_EXIT(false, 0, (*clk_mgr).base.ctx);
    false
}

unsafe fn dcn401_smu_wait_for_response_delay(
    clk_mgr: *mut clk_mgr_internal,
    delay_us: c_uint,
    mut max_retries: c_uint,
    total_delay_us: *mut c_uint,
) -> u32 {
    let mut reg: u32 = 0;
    *total_delay_us = 0;

    loop {
        reg = REG_READ(DAL_RESP_REG);
        if reg != 0 {
            break;
        }
        if delay_us >= 1000 {
            msleep(delay_us / 1000);
        } else if delay_us > 0 {
            udelay(delay_us);
        }
        *total_delay_us += delay_us;
        let old = max_retries;
        max_retries = max_retries.wrapping_sub(1);
        if old == 0 {
            break;
        }
    }
    reg
}

unsafe fn dcn401_smu_send_msg_with_param_delay(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param_in: u32,
    param_out: *mut u32,
    total_delay_us: *mut c_uint,
) -> bool {
    let mut delay1_us: c_uint = 0;
    let mut delay2_us: c_uint = 0;
    *total_delay_us = 0;
    dcn401_smu_wait_for_response_delay(clk_mgr, 10, 200000, &mut delay1_us);
    TRACE_SMU_MSG_ENTER(msg_id, param_in, (*clk_mgr).base.ctx);
    REG_WRITE(DAL_RESP_REG, 0);
    REG_WRITE(DAL_ARG_REG, param_in);
    REG_WRITE(DAL_MSG_REG, msg_id);

    if dcn401_smu_wait_for_response_delay(clk_mgr, 10, 200000, &mut delay2_us) == DALSMC_Result_OK {
        if !param_out.is_null() {
            *param_out = REG_READ(DAL_ARG_REG);
        }
        *total_delay_us = delay1_us + delay2_us;
        TRACE_SMU_MSG_EXIT(true, if !param_out.is_null() { *param_out } else { 0 }, (*clk_mgr).base.ctx);
        return true;
    }
    *total_delay_us = delay1_us + 2000000;
    TRACE_SMU_MSG_EXIT(false, 0, (*clk_mgr).base.ctx);
    false
}

pub unsafe fn dcn401_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal, version: *mut c_int) -> bool {
    smu_print!("SMU Get SMU version\n");
    if dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetSmuVersion, 0, version as *mut u32) {
        smu_print!("SMU version: %d\n", *version);
        return true;
    }
    false
}

pub unsafe fn dcn401_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool {
    let mut response = 0u32;
    smu_print!("SMU Check driver if version\n");
    if dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetDriverIfVersion, 0, &mut response) {
        smu_print!("SMU driver if version: %d\n", response);
        if response == SMU14_DRIVER_IF_VERSION { return true; }
    }
    false
}

pub unsafe fn dcn401_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool {
    let mut response = 0u32;
    smu_print!("SMU Check msg header version\n");
    if dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetMsgHeaderVersion, 0, &mut response) {
        smu_print!("SMU msg header version: %d\n", response);
        if response == DALSMC_VERSION { return true; }
    }
    false
}

pub unsafe fn dcn401_smu_send_fclk_pstate_message(clk_mgr: *mut clk_mgr_internal, support: bool) {
    smu_print!("FCLK P-state support value is : %d\n", support);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetFclkSwitchAllow, support as u32, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_send_uclk_pstate_message(clk_mgr: *mut clk_mgr_internal, support: bool) {
    smu_print!("UCLK P-state support value is : %d\n", support);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetUclkPstateAllow, support as u32, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_send_cab_for_uclk_message(clk_mgr: *mut clk_mgr_internal, num_ways: c_uint) {
    let param = (num_ways << 1) | (num_ways > 0) as c_uint;
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetCabForUclkPstate, param, core::ptr::null_mut());
    smu_print!("Numways for SubVP : %d\n", num_ways);
}

pub unsafe fn dcn401_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32) {
    smu_print!("SMU Set DRAM addr high: %d\n", addr_high);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetDalDramAddrHigh, addr_high, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32) {
    smu_print!("SMU Set DRAM addr low: %d\n", addr_low);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetDalDramAddrLow, addr_low, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) {
    smu_print!("SMU Transfer WM table DRAM 2 SMU\n");
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_TransferTableDram2Smu, TABLE_WATERMARKS, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal) {
    smu_print!("SMU Set PME workaround\n");
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_BacoAudioD3PME, 0, core::ptr::null_mut());
}

unsafe fn dcn401_smu_get_hard_min_status(clk_mgr: *mut clk_mgr_internal, no_timeout: *mut bool, total_delay_us: *mut c_uint) -> c_uint {
    let mut response = 0u32;
    let param = 0u32;
    *no_timeout = dcn401_smu_send_msg_with_param_delay(clk_mgr, DALSMC_MSG_ReturnHardMinStatus, param, &mut response, total_delay_us);
    smu_print!("SMU Get hard min status: no_timeout %d delay %d us clk bits %x\n", *no_timeout, *total_delay_us, response);
    response
}

unsafe fn dcn401_smu_wait_hard_min_status(clk_mgr: *mut clk_mgr_internal, ppclk: u32) -> bool {
    let max_delay_us = 1000000u32;
    let hardmin_status_mask = 1u32 << ppclk;
    let mut total_delay_us = 0u32;
    let mut hardmin_done = false;
    while !hardmin_done && total_delay_us < max_delay_us {
        let mut read_total_delay_us = 0u32;
        let mut no_timeout = false;
        if !hardmin_done && total_delay_us > 0 {
            udelay(500);
            total_delay_us += 500;
            smu_print!("SMU Wait hard min status for %d us\n", total_delay_us);
        }
        let hardmin_status = dcn401_smu_get_hard_min_status(clk_mgr, &mut no_timeout, &mut read_total_delay_us);
        total_delay_us += read_total_delay_us;
        hardmin_done = (hardmin_status & hardmin_status_mask) != 0;
    }
    hardmin_done
}

pub unsafe fn dcn401_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint {
    let mut response = 0u32;
    let param = (clk << 16) | freq_mhz as u32;
    smu_print!("SMU Set hard min by freq: clk = %d, freq_mhz = %d MHz\n", clk, freq_mhz);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetHardMinByFreq, param, &mut response);
    let hard_min_done = dcn401_smu_wait_hard_min_status(clk_mgr, clk);
    smu_print!("SMU Frequency set = %d KHz hard_min_done %d\n", response, hard_min_done);
    response
}

pub unsafe fn dcn401_smu_wait_for_dmub_ack_mclk(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    smu_print!("SMU to wait for DMCUB ack for MCLK : %d\n", enable);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetAlwaysWaitDmcubResp, if enable { 1 } else { 0 }, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_indicate_drr_status(clk_mgr: *mut clk_mgr_internal, mod_drr_for_pstate: bool) {
    smu_print!("SMU Set indicate drr status = %d\n", mod_drr_for_pstate);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_IndicateDrrStatus, if mod_drr_for_pstate { 1 } else { 0 }, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_set_idle_uclk_fclk_hardmin(clk_mgr: *mut clk_mgr_internal, uclk_freq_mhz: u16, fclk_freq_mhz: u16) -> bool {
    let mut response = 0u32;
    let param = ((fclk_freq_mhz as u32) << 16) | uclk_freq_mhz as u32;
    smu_print!("SMU Set idle hardmin by freq: uclk_freq_mhz = %d MHz, fclk_freq_mhz = %d MHz\n", uclk_freq_mhz, fclk_freq_mhz);
    let mut success = dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_IdleUclkFclk, param, &mut response);
    success &= dcn401_smu_wait_hard_min_status(clk_mgr, PPCLK_UCLK);
    smu_print!("SMU hard_min_done %d\n", success);
    success
}

pub unsafe fn dcn401_smu_set_active_uclk_fclk_hardmin(clk_mgr: *mut clk_mgr_internal, uclk_freq_mhz: u16, fclk_freq_mhz: u16) -> bool {
    let mut response = 0u32;
    let param = ((fclk_freq_mhz as u32) << 16) | uclk_freq_mhz as u32;
    smu_print!("SMU Set active hardmin by freq: uclk_freq_mhz = %d MHz, fclk_freq_mhz = %d MHz\n", uclk_freq_mhz, fclk_freq_mhz);
    let mut success = dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_ActiveUclkFclk, param, &mut response);
    success &= dcn401_smu_wait_hard_min_status(clk_mgr, PPCLK_UCLK);
    smu_print!("SMU hard_min_done %d\n", success);
    success
}

pub unsafe fn dcn401_smu_set_subvp_uclk_fclk_hardmin(clk_mgr: *mut clk_mgr_internal, uclk_freq_mhz: u16, fclk_freq_mhz: u16) -> bool {
    let mut response = 0u32;
    let param = ((fclk_freq_mhz as u32) << 16) | uclk_freq_mhz as u32;
    smu_print!("SMU Set active hardmin by freq: uclk_freq_mhz = %d MHz, fclk_freq_mhz = %d MHz\n", uclk_freq_mhz, fclk_freq_mhz);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SubvpUclkFclk, param, &mut response)
}

pub unsafe fn dcn401_smu_set_min_deep_sleep_dcef_clk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32) {
    smu_print!("SMU Set min deep sleep dcef clk: freq_mhz = %d MHz\n", freq_mhz);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_SetMinDeepSleepDcfclk, freq_mhz, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_set_num_of_displays(clk_mgr: *mut clk_mgr_internal, num_displays: u32) {
    smu_print!("SMU Set num of displays: num_displays = %d\n", num_displays);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_NumOfDisplays, num_displays, core::ptr::null_mut());
}

pub unsafe fn dcn401_smu_get_num_of_umc_channels(clk_mgr: *mut clk_mgr_internal) -> c_uint {
    let mut response = 0u32;
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetNumUmcChannels, 0, &mut response);
    smu_print!("SMU Get Num UMC Channels: num_umc_channels = %d\n", response);
    response
}

/*
 * Frequency in MHz returned in lower 16 bits for valid DPM level
 *
 * Call with dpm_level = 0xFF to query features, return value will be:
 *     Bits 7:0 - number of DPM levels
 *     Bit   28 - 1 = auto DPM on
 *     Bit   29 - 1 = sweep DPM on
 *     Bit   30 - 1 = forced DPM on
 *     Bit   31 - 0 = discrete, 1 = fine-grained
 *
 * With fine-grained DPM, only min and max frequencies will be reported
 *
 * Returns 0 on failure
 */
pub unsafe fn dcn401_smu_get_dpm_freq_by_index(clk_mgr: *mut clk_mgr_internal, clk: u32, dpm_level: u8) -> c_uint {
    let mut response = 0u32;
    let param = (clk << 16) | dpm_level as u32;
    smu_print!("SMU Get dpm freq by index: clk = %d, dpm_level = %d\n", clk, dpm_level);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetDpmFreqByIndex, param, &mut response);
    smu_print!("SMU dpm freq: %d MHz\n", response);
    response
}

pub unsafe fn dcn401_smu_get_dc_mode_max_dpm_freq(clk_mgr: *mut clk_mgr_internal, clk: u32) -> c_uint {
    let mut response = 0u32;
    let param = clk << 16;
    smu_print!("SMU Get DC mode max DPM freq: clk = %d\n", clk);
    dcn401_smu_send_msg_with_param(clk_mgr, DALSMC_MSG_GetDcModeMaxDpmFreq, param, &mut response);
    smu_print!("SMU DC mode max DMP freq: %d MHz\n", response);
    response
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
