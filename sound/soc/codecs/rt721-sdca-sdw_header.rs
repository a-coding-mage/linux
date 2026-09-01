// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt721-sdca-sdw.h -- RT721 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2024 Realtek Semiconductor Corp.
 */

// C includes translated as dependency intent:
// #include <linux/regmap.h>
// #include <linux/soundwire/sdw_registers.h>

pub static rt721_sdca_reg_defaults: [reg_default; 40] = [
    reg_default { reg: 0x202d, def: 0x00 },
    reg_default { reg: 0x2f01, def: 0x00 },
    reg_default { reg: 0x2f02, def: 0x09 },
    reg_default { reg: 0x2f03, def: 0x08 },
    reg_default { reg: 0x2f04, def: 0x00 },
    reg_default { reg: 0x2f05, def: 0x0e },
    reg_default { reg: 0x2f06, def: 0x01 },
    reg_default { reg: 0x2f09, def: 0x00 },
    reg_default { reg: 0x2f0a, def: 0x00 },
    reg_default { reg: 0x2f35, def: 0x00 },
    reg_default { reg: 0x2f50, def: 0xf0 },
    reg_default { reg: 0x2f58, def: 0x07 },
    reg_default { reg: 0x2f59, def: 0x07 },
    reg_default { reg: 0x2f5a, def: 0x00 },
    reg_default { reg: 0x2f5b, def: 0x07 },
    reg_default { reg: 0x2f5c, def: 0x27 },
    reg_default { reg: 0x2f5d, def: 0x07 },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE12, RT721_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_CS01, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_CS11, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PDE40, RT721_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_MUTE, CH_01),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_MUTE, CH_02),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_MUTE, CH_03),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_MUTE, CH_04),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_PDE2A, RT721_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_CS1F, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_IT26, RT721_SDCA_CTL_VENDOR_DEF, 0),
        def: 0x00,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_L),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_MUTE, CH_R),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_REQ_POWER_STATE, 0),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_01),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_PDE23, RT721_SDCA_CTL_FU_MUTE, CH_02),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_CS31, RT721_SDCA_CTL_SAMPLE_FREQ_INDEX, 0),
        def: 0x09,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_FU55, RT721_SDCA_CTL_FU_MUTE, CH_01),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_FU55, RT721_SDCA_CTL_FU_MUTE, CH_02),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_OT23, RT721_SDCA_CTL_VENDOR_DEF, 0),
        def: 0x00,
    },
];

pub static rt721_sdca_mbq_defaults: [reg_default; 50] = [
    reg_default { reg: 0x0900007, def: 0xc004 },
    reg_default { reg: 0x2000001, def: 0x0000 },
    reg_default { reg: 0x2000002, def: 0x0000 },
    reg_default { reg: 0x2000003, def: 0x0000 },
    reg_default { reg: 0x2000013, def: 0x8001 },
    reg_default { reg: 0x200003c, def: 0x0000 },
    reg_default { reg: 0x2000046, def: 0x3400 },
    reg_default { reg: 0x5f00044, def: 0x6040 },
    reg_default { reg: 0x5f00045, def: 0x3333 },
    reg_default { reg: 0x5f00048, def: 0x0000 },
    reg_default { reg: 0x6100005, def: 0x0005 },
    reg_default { reg: 0x6100006, def: 0x0000 },
    reg_default { reg: 0x610000d, def: 0x0051 },
    reg_default { reg: 0x6100010, def: 0x0180 },
    reg_default { reg: 0x6100011, def: 0x0000 },
    reg_default { reg: 0x6100013, def: 0x0000 },
    reg_default { reg: 0x6100015, def: 0x0000 },
    reg_default { reg: 0x6100017, def: 0x8049 },
    reg_default { reg: 0x6100025, def: 0x1000 },
    reg_default { reg: 0x6100029, def: 0x0809 },
    reg_default { reg: 0x610002c, def: 0x2828 },
    reg_default { reg: 0x610002d, def: 0x2929 },
    reg_default { reg: 0x610002e, def: 0x3529 },
    reg_default { reg: 0x610002f, def: 0x2901 },
    reg_default { reg: 0x6100053, def: 0x2630 },
    reg_default { reg: 0x6100054, def: 0x2a2a },
    reg_default { reg: 0x6100055, def: 0x152f },
    reg_default { reg: 0x6100057, def: 0x2200 },
    reg_default { reg: 0x610005a, def: 0x2a4b },
    reg_default { reg: 0x610005b, def: 0x2a00 },
    reg_default { reg: 0x610006a, def: 0x0102 },
    reg_default { reg: 0x610006d, def: 0x0102 },
    reg_default { reg: 0x6100092, def: 0x4f61 },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU05, RT721_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_USER_FU0F, RT721_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PLATFORM_FU44, RT721_SDCA_CTL_FU_CH_GAIN, CH_L),
        def: 0xfe00,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT721_SDCA_ENT_PLATFORM_FU44, RT721_SDCA_CTL_FU_CH_GAIN, CH_R),
        def: 0xfe00,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_FU15, RT721_SDCA_CTL_FU_CH_GAIN, CH_01),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_FU15, RT721_SDCA_CTL_FU_CH_GAIN, CH_02),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_FU15, RT721_SDCA_CTL_FU_CH_GAIN, CH_03),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_FU15, RT721_SDCA_CTL_FU_CH_GAIN, CH_04),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_VOLUME, CH_01),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_VOLUME, CH_02),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_VOLUME, CH_03),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT721_SDCA_ENT_USER_FU1E, RT721_SDCA_CTL_FU_VOLUME, CH_04),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_VOLUME, CH_L),
        def: 0x0000,
    },
    reg_default {
        reg: SDW_SDCA_CTL(FUNC_NUM_AMP, RT721_SDCA_ENT_USER_FU06, RT721_SDCA_CTL_FU_VOLUME, CH_R),
        def: 0x0000,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
