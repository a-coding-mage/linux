// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt712-sdca-sdw.h -- RT712 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

// C dependencies: <linux/regmap.h>, <linux/soundwire/sdw_registers.h>

/* must stay sorted by register address, regcache_lookup_reg() does a bsearch() */
pub const rt712_sdca_reg_defaults: [reg_default; 19] = [
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_MUTE, CH_01), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_MUTE, CH_02), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_MUTE, CH_01), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_MUTE, CH_02), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE12, RT712_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_CS01, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_CS11, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PDE40, RT712_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_01), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_02), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_03), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_MUTE, CH_04), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1C, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_CS1F, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_MUTE, CH_01), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_MUTE, CH_02), def: 0x01 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_PDE23, RT712_SDCA_CTL_REQ_POWER_STATE, 0), def: 0x03 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_CS31, RT712_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), def: 0x09 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_OT23, RT712_SDCA_CTL_VENDOR_DEF, 0), def: 0x00 },
];

/* must stay sorted by register address, regcache_lookup_reg() does a bsearch() */
pub const rt712_sdca_mbq_defaults: [reg_default; 36] = [
    reg_default { reg: 0x2000004, def: 0xaa01 },
    reg_default { reg: 0x200000e, def: 0x21e0 },
    reg_default { reg: 0x200004a, def: 0x8830 },
    reg_default { reg: 0x2000067, def: 0xf100 },
    reg_default { reg: 0x5800000, def: 0x1893 },
    reg_default { reg: 0x5b00000, def: 0x0407 },
    reg_default { reg: 0x5b00005, def: 0x0000 },
    reg_default { reg: 0x5b00029, def: 0x3fff },
    reg_default { reg: 0x5b0002a, def: 0xf000 },
    reg_default { reg: 0x6100000, def: 0x04e4 },
    reg_default { reg: 0x610000e, def: 0x0007 },
    reg_default { reg: 0x6100045, def: 0x0860 },
    reg_default { reg: 0x6100046, def: 0x0029 },
    reg_default { reg: 0x6100053, def: 0x3fff },
    reg_default { reg: 0x6100055, def: 0x0000 },
    reg_default { reg: 0x6100060, def: 0x0000 },
    reg_default { reg: 0x6100064, def: 0x8000 },
    reg_default { reg: 0x6100065, def: 0x0000 },
    reg_default { reg: 0x6100067, def: 0xff12 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_VOLUME, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU05, RT712_SDCA_CTL_FU_VOLUME, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_VOLUME, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_USER_FU0F, RT712_SDCA_CTL_FU_VOLUME, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PLATFORM_FU44, RT712_SDCA_CTL_FU_CH_GAIN, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT712_SDCA_ENT_PLATFORM_FU44, RT712_SDCA_CTL_FU_CH_GAIN, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_03), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_PLATFORM_FU15, RT712_SDCA_CTL_FU_CH_GAIN, CH_04), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_02), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_03), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT712_SDCA_ENT_USER_FU1E, RT712_SDCA_CTL_FU_VOLUME, CH_04), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_VOLUME, CH_01), def: 0x0000 },
    reg_default { reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT712_SDCA_ENT_USER_FU06, RT712_SDCA_CTL_FU_VOLUME, CH_02), def: 0x0000 },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
