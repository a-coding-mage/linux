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

// Dependencies supplied by the surrounding driver are intentionally external.

const VBIOSSMC_MSG_GET_SMU_VERSION: u32 = 0x2;
const VBIOSSMC_MSG_SET_DISPCLK_FREQ: u32 = 0x4;
const VBIOSSMC_MSG_SET_DPREFCLK_FREQ: u32 = 0x5;
const VBIOSSMC_MSG_SET_DPPCLK_FREQ: u32 = 0x6;
const VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ: u32 = 0x7;
const VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK: u32 = 0x8;
const VBIOSSMC_MSG_GET_FCLK_FREQUENCY: u32 = 0xA;
const VBIOSSMC_MSG_UPDATE_PME_RESTORE: u32 = 0xD;
const VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_HIGH: u32 = 0xE;
const VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_LOW: u32 = 0xF;
const VBIOSSMC_MSG_TRANSFER_TABLE_SMU2DRAM: u32 = 0x10;
const VBIOSSMC_MSG_TRANSFER_TABLE_DRAM2SMU: u32 = 0x11;
const VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS: u32 = 0x12;

const VBIOSSMC_STATUS_BUSY: u32 = 0x0;
const VBIOSSMC_RESULT_OK: u32 = 0x1;

const TABLE_DPMCLOCKS: u32 = 0;
const TABLE_WATERMARKS: u32 = 0;

#[repr(C)]
pub struct clk_mgr_internal {
    pub base: clk_mgr_base,
}

#[repr(C)]
pub struct clk_mgr_base {
    pub dprefclk_khz: i32,
}

#[repr(C)]
pub struct display_idle_optimization_fields {
    pub df_request_disabled: u32,
    pub phy_ref_clk_off: u32,
}

#[repr(C)]
pub union display_idle_optimization_u {
    pub data: u32,
    pub idle_info: display_idle_optimization_fields,
}

extern "C" {
    fn REG_READ(reg: u32) -> u32;
    fn REG_WRITE(reg: u32, value: u32);
    fn khz_to_mhz_ceil(value: i32) -> u32;
    fn msleep(value: u32);
    fn udelay(value: u32);
    fn IS_SMU_TIMEOUT(value: u32) -> bool;
    fn ASSERT(value: i32);
    fn dm_helpers_smu_timeout(ctx: *mut core::ffi::c_void, msg_id: u32, param: u32, timeout: u32);
}

unsafe fn dcn301_smu_wait_for_response(
    _clk_mgr: *mut clk_mgr_internal,
    delay_us: u32,
    mut max_retries: u32,
) -> u32 {
    let mut res_val = VBIOSSMC_STATUS_BUSY;
    loop {
        res_val = REG_READ(MP1_SMN_C2PMSG_91);
        if res_val != VBIOSSMC_STATUS_BUSY {
            break;
        }
        if delay_us >= 1000 {
            msleep(delay_us / 1000);
        } else if delay_us > 0 {
            udelay(delay_us);
        }
        if max_retries == 0 {
            break;
        }
        max_retries = max_retries.wrapping_sub(1);
    }
    res_val
}

unsafe fn dcn301_smu_send_msg_with_param(
    clk_mgr: *mut clk_mgr_internal,
    msg_id: u32,
    param: u32,
) -> i32 {
    let mut result = dcn301_smu_wait_for_response(clk_mgr, 10, 200000);
    if result != VBIOSSMC_RESULT_OK {
        // DC_LOG_SMU("SMU Response was not OK...", result);
    }
    if result == VBIOSSMC_STATUS_BUSY {
        return -1;
    }
    REG_WRITE(MP1_SMN_C2PMSG_91, VBIOSSMC_STATUS_BUSY);
    REG_WRITE(MP1_SMN_C2PMSG_83, param);
    REG_WRITE(MP1_SMN_C2PMSG_67, msg_id);
    result = dcn301_smu_wait_for_response(clk_mgr, 10, 200000);
    if IS_SMU_TIMEOUT(result) {
        ASSERT(0);
        dm_helpers_smu_timeout(core::ptr::null_mut(), msg_id, param, 10 * 200000);
    }
    REG_READ(MP1_SMN_C2PMSG_83) as i32
}

pub unsafe fn dcn301_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GET_SMU_VERSION, 0)
}

pub unsafe fn dcn301_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPCLK_FREQ, khz_to_mhz_ceil(requested_dispclk_khz)) * 1000
}

pub unsafe fn dcn301_smu_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DPREFCLK_FREQ, khz_to_mhz_ceil((*clk_mgr).base.dprefclk_khz)) * 1000
}

pub unsafe fn dcn301_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_HARD_MIN_DCFCLK_BY_FREQ, khz_to_mhz_ceil(requested_dcfclk_khz)) * 1000
}

pub unsafe fn dcn301_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_MIN_DEEP_SLEEP_DCFCLK, khz_to_mhz_ceil(requested_min_ds_dcfclk_khz)) * 1000
}

pub unsafe fn dcn301_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32 {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DPPCLK_FREQ, khz_to_mhz_ceil(requested_dpp_khz)) * 1000
}

pub unsafe fn dcn301_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info);
}

pub unsafe fn dcn301_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    let mut idle_info = display_idle_optimization_u { data: 0 };
    if enable {
        idle_info.data = 0x3;
    }
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_DISPLAY_IDLE_OPTIMIZATIONS, idle_info.data);
}

pub unsafe fn dcn301_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_UPDATE_PME_RESTORE, 0);
}

pub unsafe fn dcn301_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_HIGH, addr_high);
}

pub unsafe fn dcn301_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SET_VBIOS_DRAM_ADDR_LOW, addr_low);
}

pub unsafe fn dcn301_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TRANSFER_TABLE_SMU2DRAM, TABLE_DPMCLOCKS);
}

pub unsafe fn dcn301_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) {
    dcn301_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TRANSFER_TABLE_DRAM2SMU, TABLE_WATERMARKS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
