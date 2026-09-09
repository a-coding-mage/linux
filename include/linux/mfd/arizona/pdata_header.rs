/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for Arizona devices
 *
 * Copyright 2012 Wolfson Microelectronics. PLC.
 */

// Dependencies supplied by the corresponding Arizona kernel headers.

pub const ARIZONA_GPN_DIR_MASK: u16 = 0x8000;
pub const ARIZONA_GPN_DIR_SHIFT: u32 = 15;
pub const ARIZONA_GPN_DIR_WIDTH: u32 = 1;
pub const ARIZONA_GPN_PU_MASK: u16 = 0x4000;
pub const ARIZONA_GPN_PU_SHIFT: u32 = 14;
pub const ARIZONA_GPN_PU_WIDTH: u32 = 1;
pub const ARIZONA_GPN_PD_MASK: u16 = 0x2000;
pub const ARIZONA_GPN_PD_SHIFT: u32 = 13;
pub const ARIZONA_GPN_PD_WIDTH: u32 = 1;
pub const ARIZONA_GPN_LVL_MASK: u16 = 0x0800;
pub const ARIZONA_GPN_LVL_SHIFT: u32 = 11;
pub const ARIZONA_GPN_LVL_WIDTH: u32 = 1;
pub const ARIZONA_GPN_POL_MASK: u16 = 0x0400;
pub const ARIZONA_GPN_POL_SHIFT: u32 = 10;
pub const ARIZONA_GPN_POL_WIDTH: u32 = 1;
pub const ARIZONA_GPN_OP_CFG_MASK: u16 = 0x0200;
pub const ARIZONA_GPN_OP_CFG_SHIFT: u32 = 9;
pub const ARIZONA_GPN_OP_CFG_WIDTH: u32 = 1;
pub const ARIZONA_GPN_DB_MASK: u16 = 0x0100;
pub const ARIZONA_GPN_DB_SHIFT: u32 = 8;
pub const ARIZONA_GPN_DB_WIDTH: u32 = 1;
pub const ARIZONA_GPN_FN_MASK: u16 = 0x007F;
pub const ARIZONA_GPN_FN_SHIFT: u32 = 0;
pub const ARIZONA_GPN_FN_WIDTH: u32 = 7;

pub const ARIZONA_MAX_GPIO: usize = 5;
pub const ARIZONA_MAX_INPUT: usize = 4;
pub const ARIZONA_MAX_MICBIAS: usize = 3;
pub const ARIZONA_MAX_OUTPUT: usize = 6;
pub const ARIZONA_MAX_AIF: usize = 3;
pub const ARIZONA_HAP_ACT_ERM: u32 = 0;
pub const ARIZONA_HAP_ACT_LRA: u32 = 2;
pub const ARIZONA_MAX_PDM_SPK: usize = 2;

#[repr(C)]
pub struct regulator_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct arizona_micbias {
    pub mV: i32,
    // C bit-fields packed into one unsigned-int storage unit.
    pub flags: u32,
}

#[repr(C)]
pub struct arizona_micd_config {
    pub src: u32,
    pub bias: u32,
    pub gpio: bool,
}

#[repr(C)]
pub struct arizona_micd_range {
    pub max: i32,
    pub key: i32,
}

#[repr(C)]
pub struct arizona_pdata {
    pub reset: *mut gpio_desc,
    pub micvdd: arizona_micsupp_pdata,
    pub ldo1: arizona_ldo1_pdata,
    pub clk32k_src: i32,
    pub irq_flags: u32,
    pub gpio_base: i32,
    pub gpio_defaults: [u32; ARIZONA_MAX_GPIO],
    pub max_channels_clocked: [u32; ARIZONA_MAX_AIF],
    pub jd_gpio5: bool,
    pub jd_gpio5_nopull: bool,
    pub jd_invert: bool,
    pub hpdet_acc_id: bool,
    pub hpdet_acc_id_line: bool,
    pub hpdet_channel: u32,
    pub micd_software_compare: bool,
    pub micd_detect_debounce: u32,
    pub micd_bias_start_time: u32,
    pub micd_rate: u32,
    pub micd_dbtime: u32,
    pub micd_timeout: u32,
    pub micd_force_micbias: bool,
    pub micd_ranges: *const arizona_micd_range,
    pub num_micd_ranges: i32,
    pub micd_configs: *mut arizona_micd_config,
    pub num_micd_configs: i32,
    pub dmic_ref: [i32; ARIZONA_MAX_INPUT],
    pub micbias: [arizona_micbias; ARIZONA_MAX_MICBIAS],
    pub inmode: [i32; ARIZONA_MAX_INPUT],
    pub out_mono: [i32; ARIZONA_MAX_OUTPUT],
    pub out_vol_limit: [u32; 2 * ARIZONA_MAX_OUTPUT],
    pub spk_mute: [u32; ARIZONA_MAX_PDM_SPK],
    pub spk_fmt: [u32; ARIZONA_MAX_PDM_SPK],
    pub hap_act: u32,
    // CONFIG_GPIOLIB_LEGACY conditional field.
    #[cfg(feature = "CONFIG_GPIOLIB_LEGACY")]
    pub irq_gpio: i32,
    pub gpsw: u32,
}

// External types supplied by linux/regulator/arizona-micsupp.h and
// linux/regulator/arizona-ldo1.h.
pub type arizona_micsupp_pdata = crate::arizona_micsupp_pdata;
pub type arizona_ldo1_pdata = crate::arizona_ldo1_pdata;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
