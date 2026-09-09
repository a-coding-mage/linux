// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Translated dependencies supplied by the Linux clock-provider, platform-device,
// device-tree, clk-gate, and clk-mtk headers.

static WPE_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

// Equivalent of GATE_WPE(_id, _name, _parent, _shift), using the MTK gate
// operation mtk_clk_gate_ops_no_setclr_inv supplied by clk-mtk.
const fn gate_wpe(
    id: u32,
    name: &'static str,
    parent: &'static str,
    shift: u32,
) -> mtk_gate {
    GATE_MTK(
        id,
        name,
        parent,
        &WPE_CG_REGS,
        shift,
        &mtk_clk_gate_ops_no_setclr_inv,
    )
}

static WPE_CLKS: [mtk_gate; 4] = [
    gate_wpe(CLK_WPE_CK_EN, "wpe", "top_wpe", 17),
    gate_wpe(
        CLK_WPE_SMI_LARB8_CK_EN,
        "wpe_smi_larb8",
        "top_wpe",
        19,
    ),
    gate_wpe(
        CLK_WPE_SYS_EVENT_TX_CK_EN,
        "wpe_sys_event_tx",
        "top_wpe",
        20,
    ),
    gate_wpe(
        CLK_WPE_SMI_LARB8_PCLK_EN,
        "wpe_smi_larb8_p_en",
        "top_wpe",
        25,
    ),
];

static WPE_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: WPE_CLKS.as_ptr(),
    num_clks: WPE_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_WPE: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8186-wpesys",
        data: &WPE_DESC,
    },
    of_device_id {
        // sentinel
        ..of_device_id::default()
    },
];

// Equivalent of MODULE_DEVICE_TABLE(of, of_match_clk_mt8186_wpe).

static mut CLK_MT8186_WPE_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8186-wpe",
        of_match_table: OF_MATCH_CLK_MT8186_WPE.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt8186_wpe_drv).
module_platform_driver!(CLK_MT8186_WPE_DRV);

// MODULE_DESCRIPTION("MediaTek MT8186 Warp Engine clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
