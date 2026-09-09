// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static AUDIO0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static AUDIO1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x4,
    sta_ofs: 0x4,
};

unsafe fn gate_audio0(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    gate_mtk(id, name, parent, &AUDIO0_CG_REGS, shift, &mtk_clk_gate_ops_no_setclr)
}

unsafe fn gate_audio1(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    gate_mtk(id, name, parent, &AUDIO1_CG_REGS, shift, &mtk_clk_gate_ops_no_setclr)
}

static AUDIO_CLKS: &[mtk_gate] = &[
    // AUDIO0
    gate_audio0(CLK_AUDIO_AFE, "aud_afe", "audio_ck", 2),
    gate_audio0(CLK_AUDIO_22M, "aud_22m", "aud_engen1_ck", 8),
    gate_audio0(
        CLK_AUDIO_APLL_TUNER,
        "aud_apll_tuner",
        "aud_engen1_ck",
        19,
    ),
    gate_audio0(CLK_AUDIO_ADC, "aud_adc", "audio_ck", 24),
    gate_audio0(CLK_AUDIO_DAC, "aud_dac", "audio_ck", 25),
    gate_audio0(
        CLK_AUDIO_DAC_PREDIS,
        "aud_dac_predis",
        "audio_ck",
        26,
    ),
    gate_audio0(CLK_AUDIO_TML, "aud_tml", "audio_ck", 27),
    // AUDIO1
    gate_audio1(
        CLK_AUDIO_I2S1_BCLK,
        "aud_i2s1_bclk",
        "audio_ck",
        4,
    ),
    gate_audio1(
        CLK_AUDIO_I2S2_BCLK,
        "aud_i2s2_bclk",
        "audio_ck",
        5,
    ),
    gate_audio1(
        CLK_AUDIO_I2S3_BCLK,
        "aud_i2s3_bclk",
        "audio_ck",
        6,
    ),
    gate_audio1(
        CLK_AUDIO_I2S4_BCLK,
        "aud_i2s4_bclk",
        "audio_ck",
        7,
    ),
];

static AUDIO_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: AUDIO_CLKS,
    num_clks: AUDIO_CLKS.len(),
};

static OF_MATCH_CLK_MT6765_AUDIO: &[of_device_id] = &[
    of_device_id {
        compatible: "mediatek,mt6765-audsys",
        data: &AUDIO_DESC,
    },
    of_device_id {
        // sentinel
    },
];

static mut CLK_MT6765_AUDIO_DRV: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt6765-audio",
        of_match_table: OF_MATCH_CLK_MT6765_AUDIO,
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6765_audio);
// module_platform_driver(clk_mt6765_audio_drv);
// MODULE_DESCRIPTION("MediaTek MT6765 audio clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
