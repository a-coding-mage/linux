// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by the Linux clock-provider, platform-device,
// dt-bindings, clk-gate, and clk-mtk headers are intentionally external.

static IMG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// Equivalent of GATE_IMG(_id, _name, _parent, _shift), which expands to
// GATE_MTK(_id, _name, _parent, &img_cg_regs, _shift, &mtk_clk_gate_ops_setclr).
unsafe fn gate_img(id: u32, name: &'static str, parent: &'static str, shift: u32) -> MtkGate {
    gate_mtk(
        id,
        name,
        parent,
        &IMG_CG_REGS,
        shift,
        &mtk_clk_gate_ops_setclr,
    )
}

static IMG1_CLKS: [MtkGate; 4] = [
    unsafe { gate_img(CLK_IMG1_LARB9_IMG1, "img1_larb9_img1", "top_img1", 0) },
    unsafe { gate_img(CLK_IMG1_LARB10_IMG1, "img1_larb10_img1", "top_img1", 1) },
    unsafe { gate_img(CLK_IMG1_DIP, "img1_dip", "top_img1", 2) },
    unsafe { gate_img(CLK_IMG1_GALS_IMG1, "img1_gals_img1", "top_img1", 12) },
];

static IMG2_CLKS: [MtkGate; 6] = [
    unsafe { gate_img(CLK_IMG2_LARB9_IMG2, "img2_larb9_img2", "top_img1", 0) },
    unsafe { gate_img(CLK_IMG2_LARB10_IMG2, "img2_larb10_img2", "top_img1", 1) },
    unsafe { gate_img(CLK_IMG2_MFB, "img2_mfb", "top_img1", 6) },
    unsafe { gate_img(CLK_IMG2_WPE, "img2_wpe", "top_img1", 7) },
    unsafe { gate_img(CLK_IMG2_MSS, "img2_mss", "top_img1", 8) },
    unsafe { gate_img(CLK_IMG2_GALS_IMG2, "img2_gals_img2", "top_img1", 12) },
];

static IMG1_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMG1_CLKS.as_ptr(),
    num_clks: IMG1_CLKS.len(),
};

static IMG2_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMG2_CLKS.as_ptr(),
    num_clks: IMG2_CLKS.len(),
};

static OF_MATCH_CLK_MT8186_IMG: [OfDeviceId; 3] = [
    OfDeviceId {
        compatible: "mediatek,mt8186-imgsys1",
        data: &IMG1_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        compatible: "mediatek,mt8186-imgsys2",
        data: &IMG2_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8186_img);
static CLK_MT8186_IMG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8186-img",
        of_match_table: OF_MATCH_CLK_MT8186_IMG.as_ptr(),
    },
};

// module_platform_driver(clk_mt8186_img_drv);
// MODULE_DESCRIPTION("MediaTek MT8186 imgsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
