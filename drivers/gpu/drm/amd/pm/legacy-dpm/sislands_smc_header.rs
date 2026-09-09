/*
 * Copyright 2013 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: ppsmc.h

pub const SISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: usize = 16;

#[repr(C, packed)]
pub struct PP_SIslands_Dpm2PerfLevel { pub MaxPS: u8, pub TgtAct: u8, pub MaxPS_StepInc: u8, pub MaxPS_StepDec: u8, pub PSSamplingTime: u8, pub NearTDPDec: u8, pub AboveSafeInc: u8, pub BelowSafeInc: u8, pub PSDeltaLimit: u8, pub PSDeltaWin: u8, pub PwrEfficiencyRatio: u16, pub Reserved: [u8; 4] }
pub type PP_SIslands_Dpm2PerfLevel_t = PP_SIslands_Dpm2PerfLevel;

#[repr(C, packed)]
pub struct PP_SIslands_DPM2Status { pub dpm2Flags: u32, pub CurrPSkip: u8, pub CurrPSkipPowerShift: u8, pub CurrPSkipTDP: u8, pub CurrPSkipOCP: u8, pub MaxSPLLIndex: u8, pub MinSPLLIndex: u8, pub CurrSPLLIndex: u8, pub InfSweepMode: u8, pub InfSweepDir: u8, pub TDPexceeded: u8, pub reserved: u8, pub SwitchDownThreshold: u8, pub SwitchDownCounter: u32, pub SysScalingFactor: u32 }
pub type PP_SIslands_DPM2Status_t = PP_SIslands_DPM2Status;

#[repr(C, packed)]
pub struct PP_SIslands_DPM2Parameters { pub TDPLimit: u32, pub NearTDPLimit: u32, pub SafePowerLimit: u32, pub PowerBoostLimit: u32, pub MinLimitDelta: u32 }
pub type PP_SIslands_DPM2Parameters_t = PP_SIslands_DPM2Parameters;

#[repr(C, packed)]
pub struct PP_SIslands_PAPMStatus { pub EstimatedDGPU_T: u32, pub EstimatedDGPU_P: u32, pub EstimatedAPU_T: u32, pub EstimatedAPU_P: u32, pub dGPU_T_Limit_Exceeded: u8, pub reserved: [u8; 3] }
pub type PP_SIslands_PAPMStatus_t = PP_SIslands_PAPMStatus;
#[repr(C, packed)]
pub struct PP_SIslands_PAPMParameters { pub NearTDPLimitTherm: u32, pub NearTDPLimitPAPM: u32, pub PlatformPowerLimit: u32, pub dGPU_T_Limit: u32, pub dGPU_T_Warning: u32, pub dGPU_T_Hysteresis: u32 }
pub type PP_SIslands_PAPMParameters_t = PP_SIslands_PAPMParameters;

#[repr(C, packed)] pub struct SISLANDS_SMC_SCLK_VALUE { pub vCG_SPLL_FUNC_CNTL:u32,pub vCG_SPLL_FUNC_CNTL_2:u32,pub vCG_SPLL_FUNC_CNTL_3:u32,pub vCG_SPLL_FUNC_CNTL_4:u32,pub vCG_SPLL_SPREAD_SPECTRUM:u32,pub vCG_SPLL_SPREAD_SPECTRUM_2:u32,pub sclk_value:u32 }
pub type SISLANDS_SMC_SCLK_VALUE_t = SISLANDS_SMC_SCLK_VALUE;
#[repr(C, packed)] pub struct SISLANDS_SMC_MCLK_VALUE { pub vMPLL_FUNC_CNTL:u32,pub vMPLL_FUNC_CNTL_1:u32,pub vMPLL_FUNC_CNTL_2:u32,pub vMPLL_AD_FUNC_CNTL:u32,pub vMPLL_DQ_FUNC_CNTL:u32,pub vMCLK_PWRMGT_CNTL:u32,pub vDLL_CNTL:u32,pub vMPLL_SS:u32,pub vMPLL_SS2:u32,pub mclk_value:u32 }
pub type SISLANDS_SMC_MCLK_VALUE_t = SISLANDS_SMC_MCLK_VALUE;
#[repr(C, packed)] pub struct SISLANDS_SMC_VOLTAGE_VALUE { pub value:u16,pub index:u8,pub phase_settings:u8 }
pub type SISLANDS_SMC_VOLTAGE_VALUE_t = SISLANDS_SMC_VOLTAGE_VALUE;

#[repr(C, packed)] pub struct SISLANDS_SMC_HW_PERFORMANCE_LEVEL { pub ACIndex:u8,pub displayWatermark:u8,pub gen2PCIE:u8,pub UVDWatermark:u8,pub VCEWatermark:u8,pub strobeMode:u8,pub mcFlags:u8,pub padding:u8,pub aT:u32,pub bSP:u32,pub sclk:SISLANDS_SMC_SCLK_VALUE,pub mclk:SISLANDS_SMC_MCLK_VALUE,pub vddc:SISLANDS_SMC_VOLTAGE_VALUE,pub mvdd:SISLANDS_SMC_VOLTAGE_VALUE,pub vddci:SISLANDS_SMC_VOLTAGE_VALUE,pub std_vddc:SISLANDS_SMC_VOLTAGE_VALUE,pub hysteresisUp:u8,pub hysteresisDown:u8,pub stateFlags:u8,pub arbRefreshState:u8,pub SQPowerThrottle:u32,pub SQPowerThrottle_2:u32,pub MaxPoweredUpCU:u32,pub high_temp_vddc:SISLANDS_SMC_VOLTAGE_VALUE,pub low_temp_vddc:SISLANDS_SMC_VOLTAGE_VALUE,pub reserved:[u32;2],pub dpm2:PP_SIslands_Dpm2PerfLevel }

pub const SISLANDS_SMC_STROBE_RATIO:u32=0x0F; pub const SISLANDS_SMC_STROBE_ENABLE:u32=0x10; pub const SISLANDS_SMC_MC_EDC_RD_FLAG:u32=0x01; pub const SISLANDS_SMC_MC_EDC_WR_FLAG:u32=0x02; pub const SISLANDS_SMC_MC_RTT_ENABLE:u32=0x04; pub const SISLANDS_SMC_MC_STUTTER_EN:u32=0x08; pub const SISLANDS_SMC_MC_PG_EN:u32=0x10;

#[repr(C, packed)] pub struct SISLANDS_SMC_SWSTATE { pub flags:u8,pub levelCount:u8,pub padding2:u8,pub padding3:u8,pub levels:[SISLANDS_SMC_HW_PERFORMANCE_LEVEL;0] }
pub type SISLANDS_SMC_SWSTATE_t = SISLANDS_SMC_SWSTATE;
#[repr(C, packed)] pub struct SISLANDS_SMC_SWSTATE_SINGLE { pub flags:u8,pub levelCount:u8,pub padding2:u8,pub padding3:u8,pub level:SISLANDS_SMC_HW_PERFORMANCE_LEVEL }

pub const SISLANDS_SMC_VOLTAGEMASK_VDDC:u32=0; pub const SISLANDS_SMC_VOLTAGEMASK_MVDD:u32=1; pub const SISLANDS_SMC_VOLTAGEMASK_VDDCI:u32=2; pub const SISLANDS_SMC_VOLTAGEMASK_VDDC_PHASE_SHEDDING:u32=3; pub const SISLANDS_SMC_VOLTAGEMASK_MAX:usize=4;
#[repr(C, packed)] pub struct SISLANDS_SMC_VOLTAGEMASKTABLE { pub lowMask:[u32;SISLANDS_SMC_VOLTAGEMASK_MAX] }
pub type SISLANDS_SMC_VOLTAGEMASKTABLE_t=SISLANDS_SMC_VOLTAGEMASKTABLE;
pub const SISLANDS_MAX_NO_VREG_STEPS:usize=32;
#[repr(C, packed)] pub struct SISLANDS_SMC_STATETABLE { pub thermalProtectType:u8,pub systemFlags:u8,pub maxVDDCIndexInPPTable:u8,pub extraFlags:u8,pub lowSMIO:[u32;SISLANDS_MAX_NO_VREG_STEPS],pub voltageMaskTable:SISLANDS_SMC_VOLTAGEMASKTABLE,pub phaseMaskTable:SISLANDS_SMC_VOLTAGEMASKTABLE,pub dpm2Params:PP_SIslands_DPM2Parameters,pub initialState:SISLANDS_SMC_SWSTATE_SINGLE,pub ACPIState:SISLANDS_SMC_SWSTATE_SINGLE,pub ULVState:SISLANDS_SMC_SWSTATE_SINGLE,pub driverState:SISLANDS_SMC_SWSTATE,pub dpmLevels:[SISLANDS_SMC_HW_PERFORMANCE_LEVEL;SISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE] }
pub type SISLANDS_SMC_STATETABLE_t=SISLANDS_SMC_STATETABLE;

#[repr(C, packed)] pub struct PP_SIslands_FanTable { pub fdo_mode:u8,pub padding:u8,pub temp_min:i16,pub temp_med:i16,pub temp_max:i16,pub slope1:i16,pub slope2:i16,pub fdo_min:i16,pub hys_up:i16,pub hys_down:i16,pub hys_slope:i16,pub temp_resp_lim:i16,pub temp_curr:i16,pub slope_curr:i16,pub pwm_curr:i16,pub refresh_period:u32,pub fdo_max:i16,pub temp_src:u8,pub padding2:i8 }
pub type PP_SIslands_FanTable_t=PP_SIslands_FanTable;
pub const SI_SMC_SOFT_REGISTER_mclk_chg_timeout:u32=0x0; pub const SI_SMC_SOFT_REGISTER_delay_vreg:u32=0xC; pub const SI_SMC_SOFT_REGISTER_delay_acpi:u32=0x28; pub const SI_SMC_SOFT_REGISTER_seq_index:u32=0x5C; pub const SI_SMC_SOFT_REGISTER_mvdd_chg_time:u32=0x60; pub const SI_SMC_SOFT_REGISTER_mclk_switch_lim:u32=0x70; pub const SI_SMC_SOFT_REGISTER_watermark_threshold:u32=0x78; pub const SI_SMC_SOFT_REGISTER_phase_shedding_delay:u32=0x88; pub const SI_SMC_SOFT_REGISTER_ulv_volt_change_delay:u32=0x8C; pub const SI_SMC_SOFT_REGISTER_mc_block_delay:u32=0x98; pub const SI_SMC_SOFT_REGISTER_ticks_per_us:u32=0xA8; pub const SI_SMC_SOFT_REGISTER_crtc_index:u32=0xC4; pub const SI_SMC_SOFT_REGISTER_mclk_change_block_cp_min:u32=0xC8; pub const SI_SMC_SOFT_REGISTER_mclk_change_block_cp_max:u32=0xCC; pub const SI_SMC_SOFT_REGISTER_non_ulv_pcie_link_width:u32=0xF4; pub const SI_SMC_SOFT_REGISTER_tdr_is_about_to_happen:u32=0xFC; pub const SI_SMC_SOFT_REGISTER_vr_hot_gpio:u32=0x100; pub const SI_SMC_SOFT_REGISTER_svi_rework_plat_type:u32=0x118; pub const SI_SMC_SOFT_REGISTER_svi_rework_gpio_id_svd:u32=0x11C; pub const SI_SMC_SOFT_REGISTER_svi_rework_gpio_id_svc:u32=0x120;

pub const SMC_SISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES:usize=16; pub const SMC_SISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES:usize=32; pub const SMC_SISLANDS_SCALE_I:u32=7; pub const SMC_SISLANDS_SCALE_R:u32=12;
#[repr(C, packed)] pub struct PP_SIslands_CacConfig { pub cac_lkge_lut:[[u16;SMC_SISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES];SMC_SISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES],pub lkge_lut_V0:u32,pub lkge_lut_Vstep:u32,pub WinTime:u32,pub R_LL:u32,pub calculation_repeats:u32,pub l2numWin_TDP:u32,pub dc_cac:u32,pub lts_truncate_n:u8,pub SHIFT_N:u8,pub log2_PG_LKG_SCALE:u8,pub cac_temp:u8,pub lkge_lut_T0:u32,pub lkge_lut_Tstep:u32 }
pub type PP_SIslands_CacConfig_t=PP_SIslands_CacConfig;

pub const SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE:usize=16; pub const SMC_SISLANDS_MC_REGISTER_ARRAY_SET_COUNT:usize=20;
#[repr(C, packed)] pub struct SMC_SIslands_MCRegisterAddress { pub s0:u16,pub s1:u16 }
pub type SMC_SIslands_MCRegisterAddress_t=SMC_SIslands_MCRegisterAddress;
#[repr(C, packed)] pub struct SMC_SIslands_MCRegisterSet { pub value:[u32;SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE] }
pub type SMC_SIslands_MCRegisterSet_t=SMC_SIslands_MCRegisterSet;
#[repr(C, packed)] pub struct SMC_SIslands_MCRegisters { pub last:u8,pub reserved:[u8;3],pub address:[SMC_SIslands_MCRegisterAddress;SMC_SISLANDS_MC_REGISTER_ARRAY_SIZE],pub data:[SMC_SIslands_MCRegisterSet;SMC_SISLANDS_MC_REGISTER_ARRAY_SET_COUNT] }
pub type SMC_SIslands_MCRegisters_t=SMC_SIslands_MCRegisters;
#[repr(C, packed)] pub struct SMC_SIslands_MCArbDramTimingRegisterSet { pub mc_arb_dram_timing:u32,pub mc_arb_dram_timing2:u32,pub mc_arb_rfsh_rate:u8,pub mc_arb_burst_time:u8,pub padding:[u8;2] }
pub type SMC_SIslands_MCArbDramTimingRegisterSet_t=SMC_SIslands_MCArbDramTimingRegisterSet;
#[repr(C, packed)] pub struct SMC_SIslands_MCArbDramTimingRegisters { pub arb_current:u8,pub reserved:[u8;3],pub data:[SMC_SIslands_MCArbDramTimingRegisterSet;16] }
pub type SMC_SIslands_MCArbDramTimingRegisters_t=SMC_SIslands_MCArbDramTimingRegisters;
#[repr(C, packed)] pub struct SMC_SISLANDS_SPLL_DIV_TABLE { pub freq:[u32;256],pub ss:[u32;256] }
pub type SMC_SISLANDS_SPLL_DIV_TABLE_t=SMC_SISLANDS_SPLL_DIV_TABLE;
pub const SMC_SISLANDS_SPLL_DIV_TABLE_FBDIV_MASK:u32=0x01ffffff; pub const SMC_SISLANDS_SPLL_DIV_TABLE_FBDIV_SHIFT:u32=0; pub const SMC_SISLANDS_SPLL_DIV_TABLE_PDIV_MASK:u32=0xfe000000; pub const SMC_SISLANDS_SPLL_DIV_TABLE_PDIV_SHIFT:u32=25; pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKV_MASK:u32=0x000fffff; pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKV_SHIFT:u32=0; pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKS_MASK:u32=0xfff00000; pub const SMC_SISLANDS_SPLL_DIV_TABLE_CLKS_SHIFT:u32=20;

pub const SMC_SISLANDS_DTE_MAX_FILTER_STAGES:usize=5; pub const SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE:usize=16;
#[repr(C, packed)] pub struct Smc_SIslands_DTE_Configuration { pub tau:[u32;SMC_SISLANDS_DTE_MAX_FILTER_STAGES],pub R:[u32;SMC_SISLANDS_DTE_MAX_FILTER_STAGES],pub K:u32,pub T0:u32,pub MaxT:u32,pub WindowSize:u8,pub Tdep_count:u8,pub temp_select:u8,pub DTE_mode:u8,pub T_limits:[u8;SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],pub Tdep_tau:[u32;SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],pub Tdep_R:[u32;SMC_SISLANDS_DTE_MAX_TEMPERATURE_DEPENDENT_ARRAY_SIZE],pub Tthreshold:u32 }
pub type Smc_SIslands_DTE_Configuration_t=Smc_SIslands_DTE_Configuration;
pub const SMC_SISLANDS_DTE_STATUS_FLAG_DTE_ON:u32=1;
pub const SISLANDS_SMC_FIRMWARE_HEADER_LOCATION:u32=0x10000;
pub const SISLANDS_SMC_FIRMWARE_HEADER_version:u32=0x0; pub const SISLANDS_SMC_FIRMWARE_HEADER_flags:u32=0x4; pub const SISLANDS_SMC_FIRMWARE_HEADER_softRegisters:u32=0xC; pub const SISLANDS_SMC_FIRMWARE_HEADER_stateTable:u32=0x10; pub const SISLANDS_SMC_FIRMWARE_HEADER_fanTable:u32=0x14; pub const SISLANDS_SMC_FIRMWARE_HEADER_CacConfigTable:u32=0x18; pub const SISLANDS_SMC_FIRMWARE_HEADER_mcRegisterTable:u32=0x24; pub const SISLANDS_SMC_FIRMWARE_HEADER_mcArbDramAutoRefreshTable:u32=0x30; pub const SISLANDS_SMC_FIRMWARE_HEADER_spllTable:u32=0x38; pub const SISLANDS_SMC_FIRMWARE_HEADER_DteConfiguration:u32=0x40; pub const SISLANDS_SMC_FIRMWARE_HEADER_PAPMParameters:u32=0x48;

// External declarations; types are supplied by the surrounding translation unit.
extern "C" { pub fn amdgpu_si_copy_bytes_to_smc(adev:*mut amdgpu_device, smc_start_address:u32, src:*const u8, byte_count:u32, limit:u32)->i32; pub fn amdgpu_si_start_smc(adev:*mut amdgpu_device); pub fn amdgpu_si_reset_smc(adev:*mut amdgpu_device); pub fn amdgpu_si_program_jump_on_start(adev:*mut amdgpu_device)->i32; pub fn amdgpu_si_smc_clock(adev:*mut amdgpu_device, enable:bool); pub fn amdgpu_si_is_smc_running(adev:*mut amdgpu_device)->bool; pub fn amdgpu_si_send_msg_to_smc(adev:*mut amdgpu_device, msg:PPSMC_Msg)->PPSMC_Result; pub fn amdgpu_si_wait_for_smc_inactive(adev:*mut amdgpu_device)->PPSMC_Result; pub fn amdgpu_si_load_smc_ucode(adev:*mut amdgpu_device, limit:u32)->i32; pub fn amdgpu_si_read_smc_sram_dword(adev:*mut amdgpu_device, smc_address:u32, value:*mut u32, limit:u32)->i32; pub fn amdgpu_si_write_smc_sram_dword(adev:*mut amdgpu_device, smc_address:u32, value:u32, limit:u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
