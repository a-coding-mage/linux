/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt712-sdca-dmic.h -- RT712 SDCA DMIC ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

/* Depends on Linux regmap and SoundWire SDW register definitions. */

#[repr(C)]
pub struct rt712_sdca_dmic_priv {
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub fu1e_dapm_mute: bool,
    pub fu1e_mixer_mute: [bool; 4],
}

#[repr(C)]
pub struct rt712_sdca_dmic_kctrl_priv {
    pub reg_base: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub invert: ::core::ffi::c_uint,
}

/* SDCA (Channel) */
pub const CH_01: u32 = 0x01;
pub const CH_02: u32 = 0x02;
pub const CH_03: u32 = 0x03;
pub const CH_04: u32 = 0x04;

/* must stay sorted by register address, regcache_lookup_reg() does a bsearch() */
pub static rt712_sdca_dmic_reg_defaults: [reg_default; 43] = [
    reg_default { reg: 0x201a, def: 0x00 },
    reg_default { reg: 0x201b, def: 0x00 },
    reg_default { reg: 0x201c, def: 0x00 },
    reg_default { reg: 0x201d, def: 0x00 },
    reg_default { reg: 0x201e, def: 0x00 },
    reg_default { reg: 0x201f, def: 0x00 },
    reg_default { reg: 0x2029, def: 0x00 },
    reg_default { reg: 0x202a, def: 0x00 },
    reg_default { reg: 0x202d, def: 0x00 },
    reg_default { reg: 0x202e, def: 0x00 },
    reg_default { reg: 0x202f, def: 0x00 },
    reg_default { reg: 0x2030, def: 0x00 },
    reg_default { reg: 0x2031, def: 0x00 },
    reg_default { reg: 0x2032, def: 0x00 },
    reg_default { reg: 0x2033, def: 0x00 },
    reg_default { reg: 0x2034, def: 0x00 },
    reg_default { reg: 0x2230, def: 0x00 },
    reg_default { reg: 0x2231, def: 0x2f },
    reg_default { reg: 0x2232, def: 0x80 },
    reg_default { reg: 0x2f01, def: 0x00 },
    reg_default { reg: 0x2f02, def: 0x09 },
    reg_default { reg: 0x2f03, def: 0x00 },
    reg_default { reg: 0x2f04, def: 0x00 },
    reg_default { reg: 0x2f05, def: 0x0b },
    reg_default { reg: 0x2f06, def: 0x01 },
    reg_default { reg: 0x2f08, def: 0x00 },
    reg_default { reg: 0x2f09, def: 0x00 },
    reg_default { reg: 0x2f0a, def: 0x01 },
    reg_default { reg: 0x2f35, def: 0x02 },
    reg_default { reg: 0x2f36, def: 0xcf },
    reg_default { reg: 0x2f52, def: 0x08 },
    reg_default { reg: 0x2f58, def: 0x07 },
    reg_default { reg: 0x2f59, def: 0x07 },
    reg_default { reg: 0x3201, def: 0x01 },
    reg_default { reg: 0x320c, def: 0x00 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_01), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_02), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_03), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_04), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1C, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1F, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_IT26, RT712_SDCA_CTL_VENDOR_DEF, 0), def: 0x00 },
];

/* must stay sorted by register address, regcache_lookup_reg() does a bsearch() */
pub static rt712_sdca_dmic_mbq_defaults: [reg_default; 20] = [
    reg_default { reg: 0x0590001e, def: 0x0020 },
    reg_default { reg: 0x06100000, def: 0x0010 },
    reg_default { reg: 0x06100006, def: 0x0055 },
    reg_default { reg: 0x06100010, def: 0x2630 },
    reg_default { reg: 0x06100011, def: 0x152f },
    reg_default { reg: 0x06100013, def: 0x0102 },
    reg_default { reg: 0x06100015, def: 0x2219 },
    reg_default { reg: 0x06100018, def: 0x0102 },
    reg_default { reg: 0x06100026, def: 0x2c29 },
    reg_default { reg: 0x06100027, def: 0x2d2b },
    reg_default { reg: 0x0610002b, def: 0x2a32 },
    reg_default { reg: 0x0610002f, def: 0x3355 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_03), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_04), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_03), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_04), def: 0x0000 },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
