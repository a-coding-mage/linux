// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel clock framework.

static VENC_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static VENC_CLKS: [MtkGate; 5] = [
    MtkGate::dummy(CLK_DUMMY, b"venc_dummy\0"),
    MtkGate::new(CLK_VENC_CKE0, b"venc_cke0\0", b"mm_sel\0", &VENC_CG_REGS, 0, &mtk_clk_gate_ops_setclr_inv),
    MtkGate::new(CLK_VENC_CKE1, b"venc_cke1\0", b"venc_sel\0", &VENC_CG_REGS, 4, &mtk_clk_gate_ops_setclr_inv),
    MtkGate::new(CLK_VENC_CKE2, b"venc_cke2\0", b"venc_sel\0", &VENC_CG_REGS, 8, &mtk_clk_gate_ops_setclr_inv),
    MtkGate::new(CLK_VENC_CKE3, b"venc_cke3\0", b"venc_sel\0", &VENC_CG_REGS, 12, &mtk_clk_gate_ops_setclr_inv),
];

static VENCLT_CLKS: [MtkGate; 3] = [
    MtkGate::dummy(CLK_DUMMY, b"venclt_dummy\0"),
    MtkGate::new(CLK_VENCLT_CKE0, b"venclt_cke0\0", b"mm_sel\0", &VENC_CG_REGS, 0, &mtk_clk_gate_ops_setclr_inv),
    MtkGate::new(CLK_VENCLT_CKE1, b"venclt_cke1\0", b"venclt_sel\0", &VENC_CG_REGS, 4, &mtk_clk_gate_ops_setclr_inv),
];

static VENC_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENC_CLKS.as_ptr(),
    num_clks: VENC_CLKS.len(),
};

static VENC_LT_DESC: MtkClkDesc = MtkClkDesc {
    clks: VENCLT_CLKS.as_ptr(),
    num_clks: VENCLT_CLKS.len(),
};

static OF_MATCH_CLK_MT8173_VENCSYS: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"mediatek,mt8173-vencsys\0", data: &VENC_DESC },
    OfDeviceId { compatible: b"mediatek,mt8173-vencltsys\0", data: &VENC_LT_DESC },
    OfDeviceId::sentinel(),
];

static mut CLK_MT8173_VENCSYS_DRV: PlatformDriver = PlatformDriver {
    driver: Driver {
        name: b"clk-mt8173-vencsys\0",
        of_match_table: OF_MATCH_CLK_MT8173_VENCSYS.as_ptr(),
    },
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
};

module_device_table!(of, OF_MATCH_CLK_MT8173_VENCSYS);
module_platform_driver!(CLK_MT8173_VENCSYS_DRV);

module_description!(b"MediaTek MT8173 vencsys clocks driver\0");
module_license!(b"GPL\0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
