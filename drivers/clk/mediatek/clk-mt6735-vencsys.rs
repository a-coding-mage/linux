// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/platform_device.h, clk-gate.h, clk-mtk.h,
// and dt-bindings/clock/mediatek,mt6735-vencsys.h.

const VENC_CG_CON: u32 = 0x00;
const VENC_CG_SET: u32 = 0x04;
const VENC_CG_CLR: u32 = 0x08;

static mut venc_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: VENC_CG_SET,
    clr_ofs: VENC_CG_CLR,
    sta_ofs: VENC_CG_CON,
};

static vencsys_gates: [mtk_gate; 4] = [
    GATE_MTK!(CLK_VENC_SMI_LARB3, "smi_larb3", "mm_sel", &venc_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK!(CLK_VENC_VENC, "venc", "mm_sel", &venc_cg_regs, 4, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK!(CLK_VENC_JPGENC, "jpgenc", "mm_sel", &venc_cg_regs, 8, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK!(CLK_VENC_JPGDEC, "jpgdec", "mm_sel", &venc_cg_regs, 12, &mtk_clk_gate_ops_setclr_inv),
];

static vencsys_clks: mtk_clk_desc = mtk_clk_desc {
    clks: vencsys_gates.as_ptr(),
    num_clks: vencsys_gates.len(),
};

static of_match_mt6735_vencsys: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6735-vencsys\0".as_ptr() as *const core::ffi::c_char,
        data: &vencsys_clks as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt6735_vencsys: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt6735-vencsys\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_mt6735_vencsys.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_mt6735_vencsys);
// module_platform_driver(clk_mt6735_vencsys);
// MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
// MODULE_DESCRIPTION("Mediatek MT6735 vencsys clock driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
