// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

// External kernel clock-provider, platform-device, MediaTek clock, gate,
// and MT8167 clock-binding definitions are supplied by other dependencies.

static VDEC0_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0,
    clr_ofs: 0x4,
    sta_ofs: 0x0,
};

static VDEC1_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x8,
    clr_ofs: 0xc,
    sta_ofs: 0x8,
};

// GATE_VDEC0_I(_id, _name, _parent, _shift)
//     GATE_MTK(_id, _name, _parent, &vdec0_cg_regs, _shift,
//              &mtk_clk_gate_ops_setclr_inv)
// GATE_VDEC1_I(_id, _name, _parent, _shift)
//     GATE_MTK(_id, _name, _parent, &vdec1_cg_regs, _shift,
//              &mtk_clk_gate_ops_setclr_inv)

static VDEC_CLKS: [MtkGate; 2] = [
    // VDEC0
    gate_mtk(
        CLK_VDEC_CKEN,
        "vdec_cken",
        "rg_vdec",
        &VDEC0_CG_REGS,
        0,
        &mtk_clk_gate_ops_setclr_inv,
    ),
    // VDEC1
    gate_mtk(
        CLK_VDEC_LARB1_CKEN,
        "vdec_larb1_cken",
        "smi_mm",
        &VDEC1_CG_REGS,
        0,
        &mtk_clk_gate_ops_setclr_inv,
    ),
];

static VDEC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: VDEC_CLKS.len(),
};

static OF_MATCH_CLK_MT8167_VDEC: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8167-vdecsys",
        data: &VDEC_DESC,
    },
    OfDeviceId {
        // sentinel
        ..OfDeviceId::default()
    },
];

static mut CLK_MT8167_VDEC_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: PlatformDriverInner {
        name: "clk-mt8167-vdecsys",
        of_match_table: OF_MATCH_CLK_MT8167_VDEC.as_ptr(),
    },
};

module_platform_driver!(CLK_MT8167_VDEC_DRV);

module_description!("MediaTek MT8167 Video Decoders clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
