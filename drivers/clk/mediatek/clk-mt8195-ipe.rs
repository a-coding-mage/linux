// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies supplied by clk-gate.h, clk-mtk.h, and the Linux clock/platform
// headers are referenced here as external Rust items.

static IPE_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static IPE_CLKS: [MtkGate; 5] = [
    MtkGate {
        id: CLK_IPE_DPE,
        name: "ipe_dpe",
        parent_name: "top_ipe",
        regs: &IPE_CG_REGS,
        shift: 0,
        ops: &mtk_clk_gate_ops_no_setclr,
    },
    MtkGate {
        id: CLK_IPE_FDVT,
        name: "ipe_fdvt",
        parent_name: "top_ipe",
        regs: &IPE_CG_REGS,
        shift: 1,
        ops: &mtk_clk_gate_ops_no_setclr,
    },
    MtkGate {
        id: CLK_IPE_ME,
        name: "ipe_me",
        parent_name: "top_ipe",
        regs: &IPE_CG_REGS,
        shift: 2,
        ops: &mtk_clk_gate_ops_no_setclr,
    },
    MtkGate {
        id: CLK_IPE_TOP,
        name: "ipe_top",
        parent_name: "top_ipe",
        regs: &IPE_CG_REGS,
        shift: 3,
        ops: &mtk_clk_gate_ops_no_setclr,
    },
    MtkGate {
        id: CLK_IPE_SMI_LARB12,
        name: "ipe_smi_larb12",
        parent_name: "top_ipe",
        regs: &IPE_CG_REGS,
        shift: 4,
        ops: &mtk_clk_gate_ops_no_setclr,
    },
];

static IPE_DESC: MtkClkDesc = MtkClkDesc {
    clks: IPE_CLKS.as_ptr(),
    num_clks: IPE_CLKS.len(),
};

static OF_MATCH_CLK_MT8195_IPE: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: "mediatek,mt8195-ipesys",
        data: &IPE_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: "",
        data: core::ptr::null(),
    },
];

// Equivalent of MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_ipe).

static mut CLK_MT8195_IPE_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: "clk-mt8195-ipe",
        of_match_table: OF_MATCH_CLK_MT8195_IPE.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt8195_ipe_drv).
register_platform_driver!(CLK_MT8195_IPE_DRV);

module_description!("MediaTek MT8195 Image Processing Engine clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
