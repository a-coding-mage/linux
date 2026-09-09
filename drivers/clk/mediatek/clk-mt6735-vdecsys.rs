// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// External Linux clock-provider, platform-device, MediaTek clock, and
// device-tree binding declarations are supplied by other dependencies.

const VDEC_CKEN_SET: u32 = 0x00;
const VDEC_CKEN_CLR: u32 = 0x04;
const SMI_LARB1_CKEN_SET: u32 = 0x08;
const SMI_LARB1_CKEN_CLR: u32 = 0x0c;
const VDEC_RESETB_CON: u32 = 0x10;
const SMI_LARB1_RESETB_CON: u32 = 0x14;

const RST_NR_PER_BANK: u16 = 32;

static mut vdec_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: VDEC_CKEN_SET,
    clr_ofs: VDEC_CKEN_CLR,
    sta_ofs: VDEC_CKEN_SET,
};

static mut smi_larb1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: SMI_LARB1_CKEN_SET,
    clr_ofs: SMI_LARB1_CKEN_CLR,
    sta_ofs: SMI_LARB1_CKEN_SET,
};

static vdecsys_gates: [mtk_gate; 2] = [
    GATE_MTK!(CLK_VDEC_VDEC, "vdec", "vdec_sel", &vdec_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK!(CLK_VDEC_SMI_LARB1, "smi_larb1", "vdec_sel", &smi_larb1_cg_regs, 0, &mtk_clk_gate_ops_setclr_inv),
];

static mut vdecsys_rst_bank_ofs: [u16; 2] = [VDEC_RESETB_CON as u16, SMI_LARB1_RESETB_CON as u16];

static mut vdecsys_rst_idx_map: [u16; 2] = [
    MT6735_VDEC_RST0_VDEC as u16,
    MT6735_VDEC_RST1_SMI_LARB1 as u16,
];

static vdecsys_resets: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: vdecsys_rst_bank_ofs.as_ptr(),
    rst_bank_nr: vdecsys_rst_bank_ofs.len(),
    rst_idx_map: vdecsys_rst_idx_map.as_ptr(),
    rst_idx_map_nr: vdecsys_rst_idx_map.len(),
};

static vdecsys_clks: mtk_clk_desc = mtk_clk_desc {
    clks: vdecsys_gates.as_ptr(),
    num_clks: vdecsys_gates.len(),
    rst_desc: &vdecsys_resets,
};

static of_match_mt6735_vdecsys: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt6735-vdecsys",
        data: &vdecsys_clks,
    },
    of_device_id { /* sentinel */ },
];

MODULE_DEVICE_TABLE!(of, of_match_mt6735_vdecsys);

static mut clk_mt6735_vdecsys: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: device_driver {
        name: "clk-mt6735-vdecsys",
        of_match_table: of_match_mt6735_vdecsys.as_ptr(),
    },
};

module_platform_driver!(clk_mt6735_vdecsys);

MODULE_AUTHOR!("Yassine Oudjana <y.oudjana@protonmail.com>");
MODULE_DESCRIPTION!("MediaTek MT6735 vdecsys clock and reset driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
