// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt722-sdca-sdw.h -- RT722 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

// Depends on the Rust equivalents of:
// - linux/regmap.h: struct reg_default
// - linux/soundwire/sdw_registers.h and codec-local SDCA constants/macros

pub static rt722_sdca_reg_defaults: &[reg_default] = &[
    reg_default { reg: 0x202d, def: 0x00 },
    reg_default { reg: 0x2f01, def: 0x00 },
    reg_default { reg: 0x2f02, def: 0x09 },
    reg_default { reg: 0x2f03, def: 0x00 },
    reg_default { reg: 0x2f04, def: 0x00 },
    reg_default { reg: 0x2f05, def: 0x0b },
    reg_default { reg: 0x2f06, def: 0x01 },
    reg_default { reg: 0x2f08, def: 0x00 },
    reg_default { reg: 0x2f09, def: 0x00 },
    reg_default { reg: 0x2f0a, def: 0x00 },
    reg_default { reg: 0x2f35, def: 0x00 },
    reg_default { reg: 0x2f36, def: 0x00 },
    reg_default { reg: 0x2f50, def: 0xf0 },
    reg_default { reg: 0x2f58, def: 0x07 },
    reg_default { reg: 0x2f59, def: 0x07 },
    reg_default { reg: 0x2f5a, def: 0x07 },
    reg_default { reg: 0x2f5b, def: 0x07 },
    reg_default { reg: 0x2f5c, def: 0x27 },
    reg_default { reg: 0x2f5d, def: 0x07 },
    reg_default { reg: 0x200003c, def: 0xc214 },
    reg_default { reg: 0x2000046, def: 0x8004 },
    reg_default { reg: 0x5810000, def: 0x702d },
    reg_default { reg: 0x6100000, def: 0x0201 },
    reg_default { reg: 0x6100006, def: 0x0005 },
    reg_default { reg: 0x6100010, def: 0x2630 },
    reg_default { reg: 0x6100011, def: 0x152f },
    reg_default { reg: 0x6100013, def: 0x0102 },
    reg_default { reg: 0x6100015, def: 0x2200 },
    reg_default { reg: 0x6100017, def: 0x0102 },
    reg_default { reg: 0x6100025, def: 0x2a29 },
    reg_default { reg: 0x6100026, def: 0x2a00 },
    reg_default { reg: 0x6100028, def: 0x2a2a },
    reg_default { reg: 0x6100029, def: 0x4141 },
    reg_default { reg: 0x6100055, def: 0x0000 },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU05, RT722_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_USER_FU0F, RT722_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE12, RT722_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS01, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_CS11, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PDE40, RT722_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PLATFORM_FU44, RT722_SDCA_CTL_FU_CH_GAIN, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_JACK_CODEC, RT722_SDCA_ENT_PLATFORM_FU44, RT722_SDCA_CTL_FU_CH_GAIN, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_01),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_02),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_03),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_FU15, RT722_SDCA_CTL_FU_CH_GAIN, CH_04),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_01),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_02),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_03),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_MUTE, CH_04),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_01),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_02),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_03),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_USER_FU1E, RT722_SDCA_CTL_FU_VOLUME, CH_04),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_PDE2A, RT722_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_CS1F, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_MIC_ARRAY, RT722_SDCA_ENT_IT26, RT722_SDCA_CTL_VENDOR_DEF, 0),
        def: 0x00,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_USER_FU06, RT722_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_PDE23, RT722_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_CS31, RT722_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(FUNC_NUM_AMP, RT722_SDCA_ENT_OT23, RT722_SDCA_CTL_VENDOR_DEF, 0),
        def: 0x00,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
