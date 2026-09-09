// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// C dependencies: clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

static scp_adsp_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x180,
    clr_ofs: 0x180,
    sta_ofs: 0x180,
};

// C macro:
// GATE_SCP_ADSP(_id, _name, _parent, _shift) =>
//     GATE_MTK(_id, _name, _parent, &scp_adsp_cg_regs, _shift,
//              &mtk_clk_gate_ops_no_setclr)

static scp_adsp_clks: [mtk_gate; 1] = [GATE_MTK!(
    CLK_SCP_ADSP_AUDIODSP,
    "scp_adsp_audiodsp",
    "top_adsp",
    &scp_adsp_cg_regs,
    0,
    &mtk_clk_gate_ops_no_setclr
)];

static scp_adsp_desc: mtk_clk_desc = mtk_clk_desc {
    clks: scp_adsp_clks.as_ptr(),
    num_clks: scp_adsp_clks.len(),
};

static of_match_clk_mt8195_scp_adsp: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8195-scp_adsp",
        data: &scp_adsp_desc,
    },
    of_device_id {
        // sentinel
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_scp_adsp);

static mut clk_mt8195_scp_adsp_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt8195-scp_adsp",
        of_match_table: of_match_clk_mt8195_scp_adsp.as_ptr(),
    },
};

// module_platform_driver(clk_mt8195_scp_adsp_drv);
// MODULE_DESCRIPTION("MediaTek MT8195 SCP AudioDSP clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
