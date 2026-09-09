// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 * Copyright (c) 2023 Collabora Ltd.
 */

// Dependencies supplied by the Linux clock-provider, platform-device,
// MediaTek clock, gate, and device-tree binding interfaces.

static aud_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

macro_rules! GATE_AUD {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        GATE_MTK!($id, $name, $parent, &aud_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

static aud_clks: [mtk_gate; 13] = [
    GATE_AUD!(CLK_AUD_AFE, "aud_afe", "clk26m_ck", 2),
    GATE_AUD!(CLK_AUD_I2S, "aud_i2s", "i2s_infra_bck", 6),
    GATE_AUD!(CLK_AUD_22M, "aud_22m", "rg_aud_engen1", 8),
    GATE_AUD!(CLK_AUD_24M, "aud_24m", "rg_aud_engen2", 9),
    GATE_AUD!(CLK_AUD_INTDIR, "aud_intdir", "rg_aud_spdif_in", 15),
    GATE_AUD!(CLK_AUD_APLL2_TUNER, "aud_apll2_tuner", "rg_aud_engen2", 18),
    GATE_AUD!(CLK_AUD_APLL_TUNER, "aud_apll_tuner", "rg_aud_engen1", 19),
    GATE_AUD!(CLK_AUD_HDMI, "aud_hdmi", "apll12_div4", 20),
    GATE_AUD!(CLK_AUD_SPDF, "aud_spdf", "apll12_div6", 21),
    GATE_AUD!(CLK_AUD_ADC, "aud_adc", "aud_afe", 24),
    GATE_AUD!(CLK_AUD_DAC, "aud_dac", "aud_afe", 25),
    GATE_AUD!(CLK_AUD_DAC_PREDIS, "aud_dac_predis", "aud_afe", 26),
    GATE_AUD!(CLK_AUD_TML, "aud_tml", "aud_afe", 27),
];

static aud_desc: mtk_clk_desc = mtk_clk_desc {
    clks: &aud_clks,
    num_clks: aud_clks.len(),
};

static of_match_clk_mt8516_aud: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8516-audsys",
        data: &aud_desc,
    },
    of_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, of_match_clk_mt8516_aud);

static mut clk_mt8516_aud_drv: platform_driver = platform_driver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: driver {
        name: "clk-mt8516-aud",
        of_match_table: &of_match_clk_mt8516_aud,
    },
};

module_platform_driver!(clk_mt8516_aud_drv);

MODULE_DESCRIPTION!("MediaTek MT8516 audiosys clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
