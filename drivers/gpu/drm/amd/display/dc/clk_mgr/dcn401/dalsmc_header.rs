// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

pub const DALSMC_VERSION: u32 = 0x1;
pub const DALSMC_Result_OK: u32 = 0x1;
pub const DALSMC_Result_Failed: u32 = 0xFF;
pub const DALSMC_Result_UnknownCmd: u32 = 0xFE;
pub const DALSMC_Result_CmdRejectedPrereq: u32 = 0xFD;
pub const DALSMC_Result_CmdRejectedBusy: u32 = 0xFC;

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
pub const DALSMC_MSG_DcnExitReset: u32 = 0x14;
pub const DALSMC_MSG_ReturnHardMinStatus: u32 = 0x15;
pub const DALSMC_MSG_SetAlwaysWaitDmcubResp: u32 = 0x16;
pub const DALSMC_MSG_IndicateDrrStatus: u32 = 0x17; // PMFW 15811
pub const DALSMC_MSG_ActiveUclkFclk: u32 = 0x18;
pub const DALSMC_MSG_IdleUclkFclk: u32 = 0x19;
pub const DALSMC_MSG_SetUclkPstateAllow: u32 = 0x1A;
pub const DALSMC_MSG_SubvpUclkFclk: u32 = 0x1B;
pub const DALSMC_MSG_GetNumUmcChannels: u32 = 0x1C;
pub const DALSMC_Message_Count: u32 = 0x1D;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FclkSwitchAllow_e {
    FCLK_SWITCH_DISALLOW = 0,
    FCLK_SWITCH_ALLOW = 1,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
