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
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit.

pub const MAX_INSTANCE: usize = 6;
pub const MAX_SEGMENT: usize = 6;
pub const SMU_REGISTER_WRITE_RETRY_COUNT: u32 = 5;

#[repr(C)]
pub struct IP_BASE_INSTANCE {
    pub segment: [u32; MAX_SEGMENT],
}

#[repr(C)]
pub struct IP_BASE {
    pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE],
}

pub static MP0_BASE: IP_BASE = IP_BASE {
    instance: [
        IP_BASE_INSTANCE { segment: [0x00016000, 0x00DC0000, 0x00E00000, 0x00E40000, 0x0243FC00, 0] },
        IP_BASE_INSTANCE { segment: [0; 6] },
        IP_BASE_INSTANCE { segment: [0; 6] },
        IP_BASE_INSTANCE { segment: [0; 6] },
        IP_BASE_INSTANCE { segment: [0; 6] },
        IP_BASE_INSTANCE { segment: [0; 6] },
    ],
};

pub const mmMP1_C2PMSG_3: u32 = 0x3B1050C;
pub const reg__MP1_C2PMSG_3_MASK: u32 = 0xFFFFFFFF;
pub const reg__MP1_C2PMSG_3__SHIFT: u32 = 0;
pub const data_reg_name__MP1_C2PMSG_3_MASK: u32 = 0xFFFFFFFF;
pub const data_reg_name__MP1_C2PMSG_3__SHIFT: u32 = 0;

pub const VBIOSSMC_MSG_TestMessage: u32 = 0x01;
pub const VBIOSSMC_MSG_GetPmfwVersion: u32 = 0x02;
pub const VBIOSSMC_MSG_Spare0: u32 = 0x03;
pub const VBIOSSMC_MSG_SetDispclkFreq: u32 = 0x04;
pub const VBIOSSMC_MSG_Spare1: u32 = 0x05;
pub const VBIOSSMC_MSG_SetDppclkFreq: u32 = 0x06;
pub const VBIOSSMC_MSG_SetHardMinDcfclkByFreq: u32 = 0x07;
pub const VBIOSSMC_MSG_SetMinDeepSleepDcfclk: u32 = 0x08;
pub const VBIOSSMC_MSG_GetDtbclkFreq: u32 = 0x09;
pub const VBIOSSMC_MSG_SetDtbClk: u32 = 0x0A;
pub const VBIOSSMC_MSG_SetDisplayCount: u32 = 0x0B;
pub const VBIOSSMC_MSG_EnableTmdp48MHzRefclkPwrDown: u32 = 0x0C;
pub const VBIOSSMC_MSG_UpdatePmeRestore: u32 = 0x0D;
pub const VBIOSSMC_MSG_SetVbiosDramAddrHigh: u32 = 0x0E;
pub const VBIOSSMC_MSG_SetVbiosDramAddrLow: u32 = 0x0F;
pub const VBIOSSMC_MSG_TransferTableSmu2Dram: u32 = 0x10;
pub const VBIOSSMC_MSG_TransferTableDram2Smu: u32 = 0x11;
pub const VBIOSSMC_MSG_SetDisplayIdleOptimizations: u32 = 0x12;
pub const VBIOSSMC_MSG_GetDprefclkFreq: u32 = 0x13;
pub const VBIOSSMC_Message_Count: u32 = 0x14;

pub const VBIOSSMC_Status_BUSY: u32 = 0x0;
pub const VBIOSSMC_Result_OK: u32 = 0x01;
pub const VBIOSSMC_Result_Failed: u32 = 0xFF;
pub const VBIOSSMC_Result_UnknownCmd: u32 = 0xFE;
pub const VBIOSSMC_Result_CmdRejectedPrereq: u32 = 0xFD;
pub const VBIOSSMC_Result_CmdRejectedBusy: u32 = 0xFC;

unsafe fn dcn315_smu_wait_for_response(clk_mgr: *mut clk_mgr_internal, delay_us: u32, mut max_retries: u32) -> u32 {
    let mut res_val = VBIOSSMC_Status_BUSY;
    loop {
        res_val = REG_READ(clk_mgr, MP1_SMN_C2PMSG_38);
        if res_val != VBIOSSMC_Status_BUSY { break; }
        if delay_us >= 1000 { msleep(delay_us / 1000); }
        else if delay_us > 0 { udelay(delay_us); }
        if max_retries == 0 { break; }
        max_retries -= 1;
    }
    res_val
}

unsafe fn dcn315_smu_send_msg_with_param(clk_mgr: *mut clk_mgr_internal, msg_id: u32, param: u32) -> i32 {
    let mut result = dcn315_smu_wait_for_response(clk_mgr, 10, 200000);
    if result != VBIOSSMC_Result_OK { smu_print!("SMU Response was not OK. SMU response after wait received is: %d\n", result); }
    if result == VBIOSSMC_Status_BUSY { return -1; }
    REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_38, VBIOSSMC_Status_BUSY);
    REG_WRITE(clk_mgr, MP1_SMN_C2PMSG_37, param);
    let mut i = 0u32;
    let mut read_back_data = 0u32;
    while i < SMU_REGISTER_WRITE_RETRY_COUNT {
        IX_REG_SET_SYNC(clk_mgr, mmMP1_C2PMSG_3, 0, MP1_C2PMSG_3, msg_id);
        IX_REG_GET_SYNC(clk_mgr, mmMP1_C2PMSG_3, MP1_C2PMSG_3, &mut read_back_data);
        if read_back_data == msg_id { break; }
        udelay(2);
        smu_print!("SMU msg id write fail %x times. \n", i + 1);
        i += 1;
    }
    result = dcn315_smu_wait_for_response(clk_mgr, 10, 200000);
    if result == VBIOSSMC_Status_BUSY {
        ASSERT!(false);
        dm_helpers_smu_timeout((*clk_mgr).base.ctx, msg_id, param, 10 * 200000);
    }
    REG_READ(clk_mgr, MP1_SMN_C2PMSG_37) as i32
}

pub unsafe fn dcn315_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32 {
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GetPmfwVersion, 0)
}

pub unsafe fn dcn315_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested_dispclk_khz; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetDispclkFreq, khz_to_mhz_ceil(requested_dispclk_khz)) * 1000
}

pub unsafe fn dcn315_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: i32) -> i32 {
    if !(*(*clk_mgr).base.ctx).dc.debug.pstate_enabled { return -1; }
    if !(*clk_mgr).smu_present { return requested_dcfclk_khz; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetHardMinDcfclkByFreq, khz_to_mhz_ceil(requested_dcfclk_khz)) * 1000
}

pub unsafe fn dcn315_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: i32) -> i32 {
    if !(*(*clk_mgr).base.ctx).dc.debug.pstate_enabled { return -1; }
    if !(*clk_mgr).smu_present { return requested_min_ds_dcfclk_khz; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetMinDeepSleepDcfclk, khz_to_mhz_ceil(requested_min_ds_dcfclk_khz)) * 1000
}

pub unsafe fn dcn315_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: i32) -> i32 {
    if !(*clk_mgr).smu_present { return requested_dpp_khz; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetDppclkFreq, khz_to_mhz_ceil(requested_dpp_khz)) * 1000
}

pub unsafe fn dcn315_smu_set_display_idle_optimization(clk_mgr: *mut clk_mgr_internal, idle_info: u32) {
    if !(*(*clk_mgr).base.ctx).dc.debug.pstate_enabled || !(*clk_mgr).smu_present { return; }
    // TODO: Work with smu team to define optimization options.
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetDisplayIdleOptimizations, idle_info);
}

pub unsafe fn dcn315_smu_enable_phy_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    let mut idle_info: display_idle_optimization_u = core::mem::zeroed();
    if !(*clk_mgr).smu_present { return; }
    if enable { idle_info.idle_info.df_request_disabled = 1; idle_info.idle_info.phy_ref_clk_off = 1; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetDisplayIdleOptimizations, idle_info.data);
}

pub unsafe fn dcn315_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_UpdatePmeRestore, 0); } }
pub unsafe fn dcn315_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32) { if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetVbiosDramAddrHigh, addr_high); } }
pub unsafe fn dcn315_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32) { if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetVbiosDramAddrLow, addr_low); } }
pub unsafe fn dcn315_smu_transfer_dpm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TransferTableSmu2Dram, TABLE_DPMCLOCKS); } }
pub unsafe fn dcn315_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal) { if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_TransferTableDram2Smu, TABLE_WATERMARKS); } }

pub unsafe fn dcn315_smu_get_dpref_clk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    let mhz = if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GetDprefclkFreq, 0) } else { -1 };
    mhz * 1000
}
pub unsafe fn dcn315_smu_get_dtbclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    let mhz = if (*clk_mgr).smu_present { dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_GetDtbclkFreq, 0) } else { -1 };
    mhz * 1000
}
pub unsafe fn dcn315_smu_set_dtbclk(clk_mgr: *mut clk_mgr_internal, enable: bool) {
    if !(*clk_mgr).smu_present { return; }
    dcn315_smu_send_msg_with_param(clk_mgr, VBIOSSMC_MSG_SetDtbClk, enable as u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
