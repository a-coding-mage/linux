/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt1320-sdw.h -- RT1320 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2024 Realtek Semiconductor Corp.
 */

/*
 * C header dependencies:
 * <linux/regmap.h>
 * <linux/soundwire/sdw.h>
 * <linux/soundwire/sdw_type.h>
 * <linux/soundwire/sdw_registers.h>
 * <sound/soc.h>
 * "../../../drivers/soundwire/bus.h"
 */

pub const RT1320_DEV_ID: u32 = 0x6981;
pub const RT1321_DEV_ID: u32 = 0x7045;
pub const RT1321_DEV_HV_VA0_ID: u32 = 0x6997;
pub const RT1321_DEV_HV_VA1_ID: u32 = 0x7071;

/* imp-defined registers */
pub const RT1320_DEV_VERSION_ID_1: u32 = 0xc404;
pub const RT1320_DEV_ID_1: u32 = 0xc405;
pub const RT1320_DEV_ID_0: u32 = 0xc406;

pub const RT1320_HV_DEV_ID_0: u32 = 0xf622;
pub const RT1320_HV_DEV_ID_1: u32 = 0xf623;

pub const RT1320_POWER_STATE: u32 = 0xc560;

pub const RT1321_PATCH_MAIN_VER: u32 = 0x1000cffe;
pub const RT1321_PATCH_BETA_VER: u32 = 0x1000cfff;

pub const RT1320_KR0_STATUS_CNT: u32 = 0x1000f008;
pub const RT1320_KR0_INT_READY: u32 = 0x1000f021;
pub const RT1320_HIFI_VER_0: u32 = 0x3fe2e000;
pub const RT1320_HIFI_VER_1: u32 = 0x3fe2e001;
pub const RT1320_HIFI_VER_2: u32 = 0x3fe2e002;
pub const RT1320_HIFI_VER_3: u32 = 0x3fe2e003;

/* RT1320 SDCA Control - function number */
pub const FUNC_NUM_AMP: u32 = 0x04;
pub const FUNC_NUM_MIC: u32 = 0x02;

/* RT1320 SDCA entity */
pub const RT1320_SDCA_ENT0: u32 = 0x00;
pub const RT1320_SDCA_ENT_PDE11: u32 = 0x2a;
pub const RT1320_SDCA_ENT_PDE23: u32 = 0x33;
pub const RT1320_SDCA_ENT_PDE27: u32 = 0x27;
pub const RT1320_SDCA_ENT_FU14: u32 = 0x32;
pub const RT1320_SDCA_ENT_FU21: u32 = 0x03;
pub const RT1320_SDCA_ENT_FU113: u32 = 0x30;
pub const RT1320_SDCA_ENT_CS14: u32 = 0x13;
pub const RT1320_SDCA_ENT_CS21: u32 = 0x21;
pub const RT1320_SDCA_ENT_CS113: u32 = 0x12;
pub const RT1320_SDCA_ENT_SAPU: u32 = 0x29;
pub const RT1320_SDCA_ENT_PPU21: u32 = 0x04;

/* RT1320 SDCA control */
pub const RT1320_SDCA_CTL_SAMPLE_FREQ_INDEX: u32 = 0x10;
pub const RT1320_SDCA_CTL_REQ_POWER_STATE: u32 = 0x01;
pub const RT1320_SDCA_CTL_ACTUAL_POWER_STATE: u32 = 0x10;
pub const RT1320_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const RT1320_SDCA_CTL_FU_VOLUME: u32 = 0x02;
pub const RT1320_SDCA_CTL_SAPU_PROTECTION_MODE: u32 = 0x10;
pub const RT1320_SDCA_CTL_SAPU_PROTECTION_STATUS: u32 = 0x11;
pub const RT1320_SDCA_CTL_POSTURE_NUMBER: u32 = 0x10;
pub const RT1320_SDCA_CTL_FUNC_STATUS: u32 = 0x10;

/* RT1320 SDCA channel */
pub const CH_01: u32 = 0x01;
pub const CH_02: u32 = 0x02;

/* Function_Status */
pub const FUNCTION_NEEDS_INITIALIZATION: u32 = 1u32 << 5;

/* Sample Frequency Index */
pub const RT1320_SDCA_RATE_16000HZ: u32 = 0x04;
pub const RT1320_SDCA_RATE_32000HZ: u32 = 0x07;
pub const RT1320_SDCA_RATE_44100HZ: u32 = 0x08;
pub const RT1320_SDCA_RATE_48000HZ: u32 = 0x09;
pub const RT1320_SDCA_RATE_96000HZ: u32 = 0x0b;
pub const RT1320_SDCA_RATE_192000HZ: u32 = 0x0d;

pub const RT1320_AIF1: u32 = 0;
pub const RT1320_AIF2: u32 = 1;

/*
 * The version id will be useful to distinguish the capability between the different IC versions.
 * Currently, VA and VB have different DSP FW versions.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1320_version_id {
    RT1320_VA = 0,
    RT1320_VB = 1,
    RT1320_VC = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1321_version_id {
    RT1321_VA0 = 0,
    RT1321_VA1 = 1,
    RT1321_VA2 = 2,
}

pub const RT1320_VER_B_ID: u32 = 0x07392238;
pub const RT1320_VAB_MCU_PATCH: &[u8] = b"realtek/rt1320/rt1320-patch-code-vab.bin\0";
pub const RT1320_VC_MCU_PATCH: &[u8] = b"realtek/rt1320/rt1320-patch-code-vc.bin\0";
pub const RT1321_VA_MCU_PATCH: &[u8] = b"realtek/rt1320/rt1321-patch-code-va.bin\0";

pub const RT1320_FW_PARAM_ADDR: u32 = 0x3fc2ab80;
pub const RT1320_CMD_ID: u32 = 0x3fc2ab81;
pub const RT1320_CMD_PARAM_ADDR: u32 = 0x3fc2ab90;
pub const RT1320_DSPFW_STATUS_ADDR: u32 = 0x3fc2bfc4;

pub const RT1321_FW_PARAM_ADDR: u32 = 0x3fc2d300;
pub const RT1321_CMD_ID: u32 = 0x3fc2d301;
pub const RT1321_CMD_PARAM_ADDR: u32 = 0x3fc2d310;
pub const RT1321_DSPFW_STATUS_ADDR: u32 = 0x3fc2dfc4;

/* FW parameter id 6, 7 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rt1320_datafixpoint {
    pub silencedetect: i32,
    pub r0: i32,
    pub meanr0: i32,
    pub advancegain: i32,
    pub ts: i32,
    pub re: i32,
    pub t: i32,
    pub invrs: i32,
}

/* FW parameter id 1300 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct FwPara_HwSwGain {
    pub SwAdvGain: u32,
    pub SwBasGain: u32,
    pub HwAdvGain: u32,
    pub HwBasGain: u32,
    pub reserve0: u32,
    pub reserve1: u32,
    pub reserve2: u32,
    pub reserve3: u32,
    pub reserve4: u32,
    pub reserve5: u32,
}

pub type FwPara_Get_HwSwGain = FwPara_HwSwGain;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rt1320_paramcmd {
    pub moudleid: u8,
    pub commandtype: u8,
    pub reserved1: u16,
    pub commandlength: u32,
    pub reserved2: i64,
    pub paramid: u32,
    pub paramlength: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1320_fw_cmdid {
    RT1320_FW_READY = 0,
    RT1320_SET_PARAM = 1,
    RT1320_GET_PARAM = 2,
    RT1320_GET_POOLSIZE = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1320_power_state {
    RT1320_NORMAL_STATE = 0x18,
    RT1320_K_R0_STATE = 0x1b,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt1320_rw_type {
    RT1320_BRA_WRITE = 0,
    RT1320_BRA_READ = 1,
    RT1320_PARAM_WRITE = 2,
    RT1320_PARAM_READ = 3,
}

pub const RT1320_CLK_FREQ_19_2_MHZ: u32 = 1;
pub const RT1320_CLK_FREQ_24MHZ: u32 = 2;
pub const RT1320_CLK_FREQ_24_576MHZ: u32 = 3;
pub const RT1320_CLK_FREQ_22_5792MHZ: u32 = 4;

#[repr(C)]
pub struct rt1320_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub version_id: i32,
    pub brown_out: i32,
    pub dev_id: u32,
    pub fu_dapm_mute: bool,
    pub fu_mixer_mute: [bool; 4],
    pub r0_l_reg: u64,
    pub r0_r_reg: u64,
    pub r0_l_calib: u32,
    pub r0_r_calib: u32,
    pub temp_l_calib: u32,
    pub temp_r_calib: u32,
    pub dspfw_name: *const core::ffi::c_char,
    pub cali_done: bool,
    pub fw_load_done: bool,
    pub rae_update_done: bool,
    pub load_dspfw_work: work_struct,
    pub bra_msg: sdw_bpt_msg,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
