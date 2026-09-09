/* SPDX-License-Identifier: MIT */
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

pub const DALSMC_VERSION: u32 = 0x1;

// SMU Response Codes:
pub const DALSMC_Result_OK: u32 = 0x1;
pub const DALSMC_Result_Failed: u32 = 0xFF;
pub const DALSMC_Result_UnknownCmd: u32 = 0xFE;
pub const DALSMC_Result_CmdRejectedPrereq: u32 = 0xFD;
pub const DALSMC_Result_CmdRejectedBusy: u32 = 0xFC;

// Message Definitions:
pub const DALSMC_MSG_TestMessage: u32 = 0x1;
pub const DALSMC_MSG_GetSmuVersion: u32 = 0x2;
pub const DALSMC_MSG_GetDriverIfVersion: u32 = 0x3;
pub const DALSMC_MSG_GetMsgHeaderVersion: u32 = 0x4;
pub const DALSMC_MSG_SetDalDramAddrHigh: u32 = 0x5;
pub const DALSMC_MSG_SetDalDramAddrLow: u32 = 0x6;
pub const DALSMC_MSG_TransferTableSmu2Dram: u32 = 0x7;
pub const DALSMC_MSG_TransferTableDram2Smu: u32 = 0x8;
pub const DALSMC_MSG_SetHardMinByFreq: u32 = 0x9;
pub const DALSMC_MSG_SetHardMaxByFreq: u32 = 0xA;
pub const DALSMC_MSG_GetDpmFreqByIndex: u32 = 0xB;
pub const DALSMC_MSG_GetDcModeMaxDpmFreq: u32 = 0xC;
pub const DALSMC_MSG_SetMinDeepSleepDcfclk: u32 = 0xD;
pub const DALSMC_MSG_NumOfDisplays: u32 = 0xE;
pub const DALSMC_MSG_SetExternalClientDfCstateAllow: u32 = 0xF;
pub const DALSMC_MSG_BacoAudioD3PME: u32 = 0x10;
pub const DALSMC_MSG_SetFclkSwitchAllow: u32 = 0x11;
pub const DALSMC_MSG_SetCabForUclkPstate: u32 = 0x12;
pub const DALSMC_MSG_SetWorstCaseUclkLatency: u32 = 0x13;
pub const DALSMC_MSG_SetAlwaysWaitDmcubResp: u32 = 0x14;
pub const DALSMC_MSG_ReturnHardMinStatus: u32 = 0x15;
pub const DALSMC_Message_Count: u32 = 0x16;

pub const CHECK_HARD_MIN_CLK_DISPCLK: u32 = 0x1;
pub const CHECK_HARD_MIN_CLK_DPPCLK: u32 = 0x2;
pub const CHECK_HARD_MIN_CLK_DPREFCLK: u32 = 0x4;
pub const CHECK_HARD_MIN_CLK_DCFCLK: u32 = 0x8;
pub const CHECK_HARD_MIN_CLK_DTBCLK: u32 = 0x10;
pub const CHECK_HARD_MIN_CLK_UCLK: u32 = 0x20;

#[repr(C)]
pub enum FclkSwitchAllow_e {
    FCLK_SWITCH_DISALLOW,
    FCLK_SWITCH_ALLOW,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
