// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

// Dependencies supplied by the surrounding kernel clock framework and headers.

static audio0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static audio1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x4,
    sta_ofs: 0x4,
};

// C macro GATE_AUDIO0(_id, _name, _parent, _shift):
// GATE_MTK(_id, _name, _parent, &audio0_cg_regs, _shift,
//     &mtk_clk_gate_ops_no_setclr)
// C macro GATE_AUDIO1(_id, _name, _parent, _shift):
// GATE_MTK(_id, _name, _parent, &audio1_cg_regs, _shift,
//     &mtk_clk_gate_ops_no_setclr)

static audio_clks: [mtk_gate; 15] = [
    // AUDIO0
    GATE_MTK!(CLK_AUDIO_AFE, "aud_afe", "audio_sel", &audio0_cg_regs, 2,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_22M, "aud_22m", "aud_eng1_sel", &audio0_cg_regs, 8,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_24M, "aud_24m", "aud_eng2_sel", &audio0_cg_regs, 9,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_APLL2_TUNER, "aud_apll2_tuner", "aud_eng2_sel", &audio0_cg_regs, 18,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_APLL_TUNER, "aud_apll_tuner", "aud_eng1_sel", &audio0_cg_regs, 19,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_TDM, "aud_tdm", "apll12_divb", &audio0_cg_regs, 20,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_ADC, "aud_adc", "audio_sel", &audio0_cg_regs, 24,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_DAC, "aud_dac", "audio_sel", &audio0_cg_regs, 25,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_DAC_PREDIS, "aud_dac_predis", "audio_sel", &audio0_cg_regs, 26,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_TML, "aud_tml", "audio_sel", &audio0_cg_regs, 27,
        &mtk_clk_gate_ops_no_setclr),
    // AUDIO1
    GATE_MTK!(CLK_AUDIO_I2S1, "aud_i2s1", "audio_sel", &audio1_cg_regs, 4,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_I2S2, "aud_i2s2", "audio_sel", &audio1_cg_regs, 5,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_I2S3, "aud_i2s3", "audio_sel", &audio1_cg_regs, 6,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_I2S4, "aud_i2s4", "audio_sel", &audio1_cg_regs, 7,
        &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_AUDIO_PDN_ADDA6_ADC, "aud_pdn_adda6_adc", "audio_sel", &audio1_cg_regs, 20,
        &mtk_clk_gate_ops_no_setclr),
];

static audio_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &audio_clks,
    num_clks: audio_clks.len(),
};

unsafe fn clk_mt8183_audio_probe(pdev: *mut platform_device) -> c_int {
    let mut r: c_int;

    r = mtk_clk_simple_probe(pdev);
    if r != 0 {
        return r;
    }

    r = devm_of_platform_populate(&mut (*pdev).dev);
    if r != 0 {
        mtk_clk_simple_remove(pdev);
    }

    r
}

unsafe fn clk_mt8183_audio_remove(pdev: *mut platform_device) {
    of_platform_depopulate(&mut (*pdev).dev);
    mtk_clk_simple_remove(pdev);
}

static of_match_clk_mt8183_audio: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8183-audiosys",
        data: &audio_desc,
    },
    of_device_id { /* sentinel */ },
];

static mut clk_mt8183_audio_drv: platform_driver = platform_driver {
    probe: Some(clk_mt8183_audio_probe),
    remove: Some(clk_mt8183_audio_remove),
    driver: device_driver {
        name: "clk-mt8183-audio",
        of_match_table: &of_match_clk_mt8183_audio,
    },
};

module_platform_driver!(clk_mt8183_audio_drv);

module_description!("MediaTek MT8183 audio clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
