// SPDX-License-Identifier: GPL-2.0-only
//
// aw87390.h  --  aw87390 ALSA SoC Audio driver
//
// Copyright (c) 2023 awinic Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

pub const AW87390_ID_REG: u32 = 0x00;
pub const AW87390_SYSCTRL_REG: u32 = 0x01;
pub const AW87390_MDCTRL_REG: u32 = 0x02;
pub const AW87390_CPOVP_REG: u32 = 0x03;
pub const AW87390_CPP_REG: u32 = 0x04;
pub const AW87390_PAG_REG: u32 = 0x05;
pub const AW87390_AGC3P_REG: u32 = 0x06;
pub const AW87390_AGC3PA_REG: u32 = 0x07;
pub const AW87390_AGC2P_REG: u32 = 0x08;
pub const AW87390_AGC2PA_REG: u32 = 0x09;
pub const AW87390_AGC1PA_REG: u32 = 0x0A;
pub const AW87390_SYSST_REG: u32 = 0x59;
pub const AW87390_SYSINT_REG: u32 = 0x60;
pub const AW87390_DFT_SYSCTRL_REG: u32 = 0x61;
pub const AW87390_DFT_MDCTRL_REG: u32 = 0x62;
pub const AW87390_DFT_CPADP_REG: u32 = 0x63;
pub const AW87390_DFT_AGCPA_REG: u32 = 0x64;
pub const AW87390_DFT_POFR_REG: u32 = 0x65;
pub const AW87390_DFT_OC_REG: u32 = 0x66;
pub const AW87390_DFT_ADP1_REG: u32 = 0x67;
pub const AW87390_DFT_REF_REG: u32 = 0x68;
pub const AW87390_DFT_LDO_REG: u32 = 0x69;
pub const AW87390_ADP1_REG: u32 = 0x70;
pub const AW87390_ADP2_REG: u32 = 0x71;
pub const AW87390_NG1_REG: u32 = 0x72;
pub const AW87390_NG2_REG: u32 = 0x73;
pub const AW87390_NG3_REG: u32 = 0x74;
pub const AW87390_CP_REG: u32 = 0x75;
pub const AW87390_AB_REG: u32 = 0x76;
pub const AW87390_TEST_REG: u32 = 0x77;
pub const AW87390_ENCR_REG: u32 = 0x78;
pub const AW87390_DELAY_REG_ADDR: u32 = 0xFE;

pub const AW87390_SOFT_RESET_VALUE: u32 = 0xAA;
pub const AW87390_POWER_DOWN_VALUE: u32 = 0x00;
pub const AW87390_REG_MAX: u32 = 0xFF;
pub const AW87390_DEV_DEFAULT_CH: u32 = 0;
pub const AW87390_INIT_PROFILE: u32 = 0;
pub const AW87390_REG_DELAY_TIME: u32 = 1000;
pub const AW87390_I2C_NAME: &str = "aw87390";
pub const AW87390_ACF_FILE: &str = "aw87390_acf.bin";

pub const AW87391_SYSCTRL_REG: u32 = 0x01;
pub const AW87391_REG_VER_SEL_LOW: u32 = 0 << 6;
pub const AW87391_REG_VER_SEL_NORMAL: u32 = 1 << 6;
pub const AW87391_REG_VER_SEL_SUPER: u32 = 2 << 6;
pub const AW87391_REG_EN_ADAP: u32 = 1 << 5;
pub const AW87391_REG_EN_2X: u32 = 1 << 4;
pub const AW87391_EN_SPK: u32 = 1 << 3;
pub const AW87391_EN_PA: u32 = 1 << 2;
pub const AW87391_REG_EN_CP: u32 = 1 << 1;
pub const AW87391_EN_SW: u32 = 1 << 0;

pub const AW87391_CP_REG: u32 = 0x02;
pub const AW87391_REG_CP_OVP_6_50V: u32 = 0;
pub const AW87391_REG_CP_OVP_6_75V: u32 = 1;
pub const AW87391_REG_CP_OVP_7_00V: u32 = 2;
pub const AW87391_REG_CP_OVP_7_25V: u32 = 3;
pub const AW87391_REG_CP_OVP_7_50V: u32 = 4;
pub const AW87391_REG_CP_OVP_7_75V: u32 = 5;
pub const AW87391_REG_CP_OVP_8_00V: u32 = 6;
pub const AW87391_REG_CP_OVP_8_25V: u32 = 7;
pub const AW87391_REG_CP_OVP_8_50V: u32 = 8;

pub const AW87391_PAG_REG: u32 = 0x03;
pub const AW87391_GAIN_12DB: u32 = 0;
pub const AW87391_GAIN_15DB: u32 = 1;
pub const AW87391_GAIN_18DB: u32 = 2;
pub const AW87391_GAIN_21DB: u32 = 3;
pub const AW87391_GAIN_24DB: u32 = 4;

pub const AW87391_AGCPO_REG: u32 = 0x04;
pub const AW87391_AK1_S_016: u32 = 2 << 5;
pub const AW87391_AK1_S_032: u32 = 3 << 5;
pub const AW87391_PD_AGC1_PWRDN: u32 = 1 << 4;
/* AGC2PO supports values between 500mW (0000) to 1600mW (1011) */
#[macro_export]
macro_rules! AW87391_AGC2PO_MW {
    ($n:expr) => {
        (($n / 100) - 5)
    };
}

pub const AW87391_AGC2PA_REG: u32 = 0x05;
pub const AW87391_RK_S_5_12: u32 = 0 << 5;
pub const AW87391_RK_S_10_24: u32 = 1 << 5;
pub const AW87391_RK_S_20_48: u32 = 2 << 5;
pub const AW87391_RK_S_41: u32 = 3 << 5;
pub const AW87391_RK_S_82: u32 = 4 << 5;
pub const AW87391_RK_S_164: u32 = 5 << 5;
pub const AW87391_RK_S_328: u32 = 6 << 5;
pub const AW87391_RK_S_656: u32 = 7 << 5;
pub const AW87391_AK2_S_1_28: u32 = 0 << 2;
pub const AW87391_AK2_S_2_56: u32 = 1 << 2;
pub const AW87391_AK2_S_10_24: u32 = 2 << 2;
pub const AW87391_AK2_S_41: u32 = 3 << 2;
pub const AW87391_AK2_S_82: u32 = 4 << 2;
pub const AW87391_AK2_S_164: u32 = 5 << 2;
pub const AW87391_AK2_S_328: u32 = 6 << 2;
pub const AW87391_AK2_S_656: u32 = 7 << 2;
pub const AW87391_AK2F_S_10_24: u32 = 0;
pub const AW87391_AK2F_S_20_48: u32 = 1;
pub const AW87391_AK2F_S_41: u32 = 2;
pub const AW87391_AK2F_S_82: u32 = 3;

pub const AW87391_SYSST_REG: u32 = 0x06;
pub const AW87391_UVLO: u32 = 1 << 7;
pub const AW87391_OTN: u32 = 1 << 6;
pub const AW87391_OC_FLAG: u32 = 1 << 5;
pub const AW87391_ADAP_CP: u32 = 1 << 4;
pub const AW87391_STARTOK: u32 = 1 << 3;
pub const AW87391_CP_OVP: u32 = 1 << 2;
pub const AW87391_PORN: u32 = 1 << 1;

pub const AW87391_SYSINT_REG: u32 = 0x07;
pub const AW87391_UVLOI: u32 = 1 << 7;
pub const AW87391_ONTI: u32 = 1 << 6;
pub const AW87391_OC_FLAGI: u32 = 1 << 5;
pub const AW87391_ADAP_CPI: u32 = 1 << 4;
pub const AW87391_STARTOKI: u32 = 1 << 3;
pub const AW87391_CP_OVPI: u32 = 1 << 2;
pub const AW87391_PORNI: u32 = 1 << 1;

pub const AW87391_DFT_THGEN0_REG: u32 = 0x63;
pub const AW87391_ADAPVTH_01W: u32 = 0 << 2;
pub const AW87391_ADAPVTH_02W: u32 = 1 << 2;
pub const AW87391_ADAPVTH_03W: u32 = 2 << 2;
pub const AW87391_ADAPVTH_04W: u32 = 3 << 2;

pub const AW87391_I2C_NAME: &str = "aw87391";

#[macro_export]
macro_rules! AW87390_PROFILE_EXT {
    ($xname:expr, $profile_info:expr, $profile_get:expr, $profile_set:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: $profile_info,
            get: $profile_get,
            put: $profile_set,
        }
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum aw87390_id {
    AW87390_CHIP_ID = 0x76,
    AW87391_CHIP_ID = 0xc1,
}

pub const AW87390_DEV_FW_FAILED: u32 = 0;
pub const AW87390_DEV_FW_OK: u32 = 1;

pub const AW87390_DEV_PW_OFF: u32 = 0;
pub const AW87390_DEV_PW_ON: u32 = 1;

#[repr(C)]
pub struct aw87390 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub vdd_reg: *mut regulator,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
