/*
 * Copyright 2011 Advanced Micro Devices, Inc.
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

pub const PPSMC_SWSTATE_FLAG_DC: u8 = 0x01;
pub const PPSMC_SWSTATE_FLAG_UVD: u8 = 0x02;
pub const PPSMC_SWSTATE_FLAG_VCE: u8 = 0x04;
pub const PPSMC_SWSTATE_FLAG_PCIE_X1: u8 = 0x08;

pub const PPSMC_THERMAL_PROTECT_TYPE_INTERNAL: u8 = 0x00;
pub const PPSMC_THERMAL_PROTECT_TYPE_EXTERNAL: u8 = 0x01;
pub const PPSMC_THERMAL_PROTECT_TYPE_NONE: u8 = 0xff;

pub const PPSMC_SYSTEMFLAG_GPIO_DC: u8 = 0x01;
pub const PPSMC_SYSTEMFLAG_STEPVDDC: u8 = 0x02;
pub const PPSMC_SYSTEMFLAG_GDDR5: u8 = 0x04;
pub const PPSMC_SYSTEMFLAG_DISABLE_BABYSTEP: u8 = 0x08;
pub const PPSMC_SYSTEMFLAG_REGULATOR_HOT: u8 = 0x10;
pub const PPSMC_SYSTEMFLAG_REGULATOR_HOT_ANALOG: u8 = 0x20;
pub const PPSMC_SYSTEMFLAG_REGULATOR_HOT_PROG_GPIO: u8 = 0x40;

pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_MASK: u8 = 0x07;
pub const PPSMC_EXTRAFLAGS_AC2DC_DONT_WAIT_FOR_VBLANK: u8 = 0x08;
pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_GOTODPMLOWSTATE: u8 = 0x00;
pub const PPSMC_EXTRAFLAGS_AC2DC_ACTION_GOTOINITIALSTATE: u8 = 0x01;
pub const PPSMC_EXTRAFLAGS_AC2DC_GPIO5_POLARITY_HIGH: u8 = 0x02;

pub const PPSMC_DISPLAY_WATERMARK_LOW: u8 = 0;
pub const PPSMC_DISPLAY_WATERMARK_HIGH: u8 = 1;

pub const PPSMC_STATEFLAG_AUTO_PULSE_SKIP: u8 = 0x01;
pub const PPSMC_STATEFLAG_POWERBOOST: u8 = 0x02;
pub const PPSMC_STATEFLAG_DEEPSLEEP_THROTTLE: u8 = 0x20;
pub const PPSMC_STATEFLAG_DEEPSLEEP_BYPASS: u8 = 0x40;

pub const FDO_MODE_HARDWARE: u8 = 0;
pub const FDO_MODE_PIECE_WISE_LINEAR: u8 = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FAN_CONTROL {
    FAN_CONTROL_FUZZY,
    FAN_CONTROL_TABLE,
}

pub const PPSMC_Result_OK: u8 = 0x01;
pub const PPSMC_Result_Failed: u8 = 0xFF;
pub type PPSMC_Result = u8;

pub const PPSMC_MSG_Halt: u8 = 0x10;
pub const PPSMC_MSG_Resume: u8 = 0x11;
pub const PPSMC_MSG_ZeroLevelsDisabled: u8 = 0x13;
pub const PPSMC_MSG_OneLevelsDisabled: u8 = 0x14;
pub const PPSMC_MSG_TwoLevelsDisabled: u8 = 0x15;
pub const PPSMC_MSG_EnableThermalInterrupt: u8 = 0x16;
pub const PPSMC_MSG_RunningOnAC: u8 = 0x17;
pub const PPSMC_MSG_SwitchToSwState: u8 = 0x20;
pub const PPSMC_MSG_SwitchToInitialState: u8 = 0x40;
pub const PPSMC_MSG_NoForcedLevel: u8 = 0x41;
pub const PPSMC_MSG_ForceHigh: u8 = 0x42;
pub const PPSMC_MSG_ForceMediumOrHigh: u8 = 0x43;
pub const PPSMC_MSG_SwitchToMinimumPower: u8 = 0x51;
pub const PPSMC_MSG_ResumeFromMinimumPower: u8 = 0x52;
pub const PPSMC_MSG_EnableCac: u8 = 0x53;
pub const PPSMC_MSG_DisableCac: u8 = 0x54;
pub const PPSMC_TDPClampingActive: u8 = 0x59;
pub const PPSMC_TDPClampingInactive: u8 = 0x5A;
pub const PPSMC_StartFanControl: u8 = 0x5B;
pub const PPSMC_StopFanControl: u8 = 0x5C;
pub const PPSMC_MSG_NoDisplay: u8 = 0x5D;
pub const PPSMC_NoDisplay: u8 = 0x5D;
pub const PPSMC_MSG_HasDisplay: u8 = 0x5E;
pub const PPSMC_HasDisplay: u8 = 0x5E;
pub const PPSMC_MSG_UVDPowerOFF: u8 = 0x60;
pub const PPSMC_MSG_UVDPowerON: u8 = 0x61;
pub const PPSMC_MSG_EnableULV: u8 = 0x62;
pub const PPSMC_MSG_DisableULV: u8 = 0x63;
pub const PPSMC_MSG_EnterULV: u8 = 0x64;
pub const PPSMC_MSG_ExitULV: u8 = 0x65;
pub const PPSMC_CACLongTermAvgEnable: u8 = 0x6E;
pub const PPSMC_CACLongTermAvgDisable: u8 = 0x6F;
pub const PPSMC_MSG_CollectCAC_PowerCorreln: u8 = 0x7A;
pub const PPSMC_FlushDataCache: u8 = 0x80;
pub const PPSMC_MSG_SetEnabledLevels: u8 = 0x82;
pub const PPSMC_MSG_SetForcedLevels: u8 = 0x83;
pub const PPSMC_MSG_ResetToDefaults: u8 = 0x84;
pub const PPSMC_MSG_EnableDTE: u8 = 0x87;
pub const PPSMC_MSG_DisableDTE: u8 = 0x88;
pub const PPSMC_MSG_ThrottleOVRDSCLKDS: u8 = 0x96;
pub const PPSMC_MSG_CancelThrottleOVRDSCLKDS: u8 = 0x97;
pub const PPSMC_MSG_EnableACDCGPIOInterrupt: u16 = 0x149;

/* CI/KV/KB */
pub const PPSMC_MSG_UVDDPM_SetEnabledMask: u16 = 0x12D;
pub const PPSMC_MSG_VCEDPM_SetEnabledMask: u16 = 0x12E;
pub const PPSMC_MSG_ACPDPM_SetEnabledMask: u16 = 0x12F;
pub const PPSMC_MSG_SAMUDPM_SetEnabledMask: u16 = 0x130;
pub const PPSMC_MSG_MCLKDPM_ForceState: u16 = 0x131;
pub const PPSMC_MSG_MCLKDPM_NoForcedLevel: u16 = 0x132;
pub const PPSMC_MSG_Thermal_Cntl_Disable: u16 = 0x133;
pub const PPSMC_MSG_Voltage_Cntl_Disable: u16 = 0x135;
pub const PPSMC_MSG_PCIeDPM_Enable: u16 = 0x136;
pub const PPSMC_MSG_PCIeDPM_Disable: u16 = 0x13d;
pub const PPSMC_MSG_ACPPowerOFF: u16 = 0x137;
pub const PPSMC_MSG_ACPPowerON: u16 = 0x138;
pub const PPSMC_MSG_SAMPowerOFF: u16 = 0x139;
pub const PPSMC_MSG_SAMPowerON: u16 = 0x13a;
// Duplicate declaration in the C header.
pub const PPSMC_MSG_NBDPM_Enable: u16 = 0x140;
pub const PPSMC_MSG_NBDPM_Disable: u16 = 0x141;
pub const PPSMC_MSG_SCLKDPM_SetEnabledMask: u16 = 0x145;
pub const PPSMC_MSG_MCLKDPM_SetEnabledMask: u16 = 0x146;
pub const PPSMC_MSG_PCIeDPM_ForceLevel: u16 = 0x147;
pub const PPSMC_MSG_PCIeDPM_UnForceLevel: u16 = 0x148;
pub const PPSMC_MSG_EnableVRHotGPIOInterrupt: u16 = 0x14a;
pub const PPSMC_MSG_DPM_Enable: u16 = 0x14e;
pub const PPSMC_MSG_DPM_Disable: u16 = 0x14f;
pub const PPSMC_MSG_MCLKDPM_Enable: u16 = 0x150;
pub const PPSMC_MSG_MCLKDPM_Disable: u16 = 0x151;
pub const PPSMC_MSG_UVDDPM_Enable: u16 = 0x154;
pub const PPSMC_MSG_UVDDPM_Disable: u16 = 0x155;
pub const PPSMC_MSG_SAMUDPM_Enable: u16 = 0x156;
pub const PPSMC_MSG_SAMUDPM_Disable: u16 = 0x157;
pub const PPSMC_MSG_ACPDPM_Enable: u16 = 0x158;
pub const PPSMC_MSG_ACPDPM_Disable: u16 = 0x159;
pub const PPSMC_MSG_VCEDPM_Enable: u16 = 0x15a;
pub const PPSMC_MSG_VCEDPM_Disable: u16 = 0x15b;
pub const PPSMC_MSG_VddC_Request: u16 = 0x15f;
pub const PPSMC_MSG_SCLKDPM_GetEnabledMask: u16 = 0x162;
pub const PPSMC_MSG_PCIeDPM_SetEnabledMask: u16 = 0x167;
pub const PPSMC_MSG_TDCLimitEnable: u16 = 0x169;
pub const PPSMC_MSG_TDCLimitDisable: u16 = 0x16a;
pub const PPSMC_MSG_PkgPwrLimitEnable: u16 = 0x185;
pub const PPSMC_MSG_PkgPwrLimitDisable: u16 = 0x186;
pub const PPSMC_MSG_PkgPwrSetLimit: u16 = 0x187;
pub const PPSMC_MSG_OverDriveSetTargetTdp: u16 = 0x188;
pub const PPSMC_MSG_SCLKDPM_FreezeLevel: u16 = 0x189;
pub const PPSMC_MSG_SCLKDPM_UnfreezeLevel: u16 = 0x18A;
pub const PPSMC_MSG_MCLKDPM_FreezeLevel: u16 = 0x18B;
pub const PPSMC_MSG_MCLKDPM_UnfreezeLevel: u16 = 0x18C;
pub const PPSMC_MSG_MASTER_DeepSleep_ON: u16 = 0x18F;
pub const PPSMC_MSG_MASTER_DeepSleep_OFF: u16 = 0x190;
pub const PPSMC_MSG_Remove_DC_Clamp: u16 = 0x191;
pub const PPSMC_MSG_SetFanPwmMax: u16 = 0x19A;
pub const PPSMC_MSG_SetFanRpmMax: u16 = 0x205;
pub const PPSMC_MSG_ENABLE_THERMAL_DPM: u16 = 0x19C;
pub const PPSMC_MSG_DISABLE_THERMAL_DPM: u16 = 0x19D;
pub const PPSMC_MSG_API_GetSclkFrequency: u16 = 0x200;
pub const PPSMC_MSG_API_GetMclkFrequency: u16 = 0x201;

/* TN */
pub const PPSMC_MSG_DPM_Config: u32 = 0x102;
pub const PPSMC_MSG_DPM_ForceState: u32 = 0x104;
pub const PPSMC_MSG_PG_SIMD_Config: u32 = 0x108;
pub const PPSMC_MSG_Voltage_Cntl_Enable: u32 = 0x109;
pub const PPSMC_MSG_Thermal_Cntl_Enable: u32 = 0x10a;
pub const PPSMC_MSG_VCEPowerOFF: u32 = 0x10e;
pub const PPSMC_MSG_VCEPowerON: u32 = 0x10f;
pub const PPSMC_MSG_DPM_N_LevelsDisabled: u32 = 0x112;
pub const PPSMC_MSG_DCE_RemoveVoltageAdjustment: u32 = 0x11d;
pub const PPSMC_MSG_DCE_AllowVoltageAdjustment: u32 = 0x11e;
pub const PPSMC_MSG_EnableBAPM: u32 = 0x120;
pub const PPSMC_MSG_DisableBAPM: u32 = 0x121;
pub const PPSMC_MSG_UVD_DPM_Config: u32 = 0x124;

pub const PPSMC_MSG_DRV_DRAM_ADDR_HI: u16 = 0x250;
pub const PPSMC_MSG_DRV_DRAM_ADDR_LO: u16 = 0x251;
pub const PPSMC_MSG_SMU_DRAM_ADDR_HI: u16 = 0x252;
pub const PPSMC_MSG_SMU_DRAM_ADDR_LO: u16 = 0x253;
pub const PPSMC_MSG_LoadUcodes: u16 = 0x254;

pub type PPSMC_Msg = u16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
