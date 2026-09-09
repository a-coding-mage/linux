// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// dt-bindings/clock/mediatek,mt6795-clk.h
// dt-bindings/reset/mediatek,mt6795-resets.h
// linux/module.h, linux/platform_device.h, clk-cpumux.h, clk-gate.h,
// clk-mtk.h, reset.h

const INFRA_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0040,
    clr_ofs: 0x0044,
    sta_ofs: 0x0048,
};

static CA53_C0_PARENTS: [&'static str; 4] = ["clk26m", "armca53pll", "mainpll", "univpll"];
static CA53_C1_PARENTS: [&'static str; 4] = ["clk26m", "armca53pll", "mainpll", "univpll"];

static CPU_MUXES: [mtk_composite; 2] = [
    MUX(CLK_INFRA_CA53_C0_SEL, "infra_ca53_c0_sel", &CA53_C0_PARENTS, 0x00, 0, 2),
    MUX(CLK_INFRA_CA53_C1_SEL, "infra_ca53_c1_sel", &CA53_C1_PARENTS, 0x00, 2, 2),
];

static INFRA_GATES: [mtk_gate; 14] = [
    GATE_MTK(CLK_INFRA_DBGCLK, "infra_dbgclk", "axi_sel", &INFRA_CG_REGS, 0, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_SMI, "infra_smi", "mm_sel", &INFRA_CG_REGS, 1, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_AUDIO, "infra_audio", "aud_intbus_sel", &INFRA_CG_REGS, 5, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_GCE, "infra_gce", "axi_sel", &INFRA_CG_REGS, 6, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_L2C_SRAM, "infra_l2c_sram", "axi_sel", &INFRA_CG_REGS, 7, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_M4U, "infra_m4u", "mem_sel", &INFRA_CG_REGS, 8, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_MD1MCU, "infra_md1mcu", "clk26m", &INFRA_CG_REGS, 9, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_MD1BUS, "infra_md1bus", "axi_sel", &INFRA_CG_REGS, 10, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_MD1DBB, "infra_dbb", "axi_sel", &INFRA_CG_REGS, 11, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_DEVICE_APC, "infra_devapc", "clk26m", &INFRA_CG_REGS, 12, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_TRNG, "infra_trng", "axi_sel", &INFRA_CG_REGS, 13, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_MD1LTE, "infra_md1lte", "axi_sel", &INFRA_CG_REGS, 14, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_CPUM, "infra_cpum", "cpum_ck", &INFRA_CG_REGS, 15, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK(CLK_INFRA_KP, "infra_kp", "axi_sel", &INFRA_CG_REGS, 16, &mtk_clk_gate_ops_no_setclr),
];

static mut INFRA_AO_RST_OFS: [u16; 2] = [0x30, 0x34];

static mut INFRA_AO_IDX_MAP: [u16; 16] = {
    let mut map = [0u16; 16];
    map[MT6795_INFRA_RST0_SCPSYS_RST as usize] = 0 * RST_NR_PER_BANK + 5;
    map[MT6795_INFRA_RST0_PMIC_WRAP_RST as usize] = 0 * RST_NR_PER_BANK + 7;
    map[MT6795_INFRA_RST1_MIPI_DSI_RST as usize] = 1 * RST_NR_PER_BANK + 4;
    map[MT6795_INFRA_RST1_MIPI_CSI_RST as usize] = 1 * RST_NR_PER_BANK + 7;
    map[MT6795_INFRA_RST1_MM_IOMMU_RST as usize] = 1 * RST_NR_PER_BANK + 15;
    map
};

static CLK_RST_DESC: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SET_CLR,
    rst_bank_ofs: &INFRA_AO_RST_OFS,
    rst_bank_nr: INFRA_AO_RST_OFS.len(),
    rst_idx_map: &INFRA_AO_IDX_MAP,
    rst_idx_map_nr: INFRA_AO_IDX_MAP.len(),
};

static OF_MATCH_CLK_MT6795_INFRACFG: [of_device_id; 2] = [
    of_device_id { compatible: "mediatek,mt6795-infracfg" },
    of_device_id { /* sentinel */ },
];

unsafe fn clk_mt6795_infracfg_probe(pdev: *mut platform_device) -> c_int {
    let mut clk_data: *mut clk_hw_onecell_data;
    let node = (*(*pdev).dev.of_node);
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }

    clk_data = mtk_alloc_clk_data(CLK_INFRA_NR_CLK);
    if clk_data.is_null() { return -ENOMEM; }

    let mut ret = mtk_register_reset_controller_with_dev(&mut (*pdev).dev, &CLK_RST_DESC);
    if ret != 0 { mtk_free_clk_data(clk_data); return ret; }
    ret = mtk_clk_register_gates(&mut (*pdev).dev, node, &INFRA_GATES, INFRA_GATES.len(), clk_data);
    if ret != 0 { mtk_free_clk_data(clk_data); return ret; }
    ret = mtk_clk_register_cpumuxes(&mut (*pdev).dev, node, &CPU_MUXES, CPU_MUXES.len(), clk_data);
    if ret != 0 {
        mtk_clk_unregister_gates(&INFRA_GATES, INFRA_GATES.len(), clk_data);
        mtk_free_clk_data(clk_data);
        return ret;
    }
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if ret != 0 {
        mtk_clk_unregister_cpumuxes(&CPU_MUXES, CPU_MUXES.len(), clk_data);
        mtk_clk_unregister_gates(&INFRA_GATES, INFRA_GATES.len(), clk_data);
        mtk_free_clk_data(clk_data);
        return ret;
    }
    0
}

unsafe fn clk_mt6795_infracfg_remove(pdev: *mut platform_device) {
    let node = (*(*pdev).dev.of_node);
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider(node);
    mtk_clk_unregister_cpumuxes(&CPU_MUXES, CPU_MUXES.len(), clk_data);
    mtk_clk_unregister_gates(&INFRA_GATES, INFRA_GATES.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static mut CLK_MT6795_INFRACFG_DRV: platform_driver = platform_driver {
    driver: driver {
        name: "clk-mt6795-infracfg",
        of_match_table: &OF_MATCH_CLK_MT6795_INFRACFG,
    },
    probe: clk_mt6795_infracfg_probe,
    remove: clk_mt6795_infracfg_remove,
};

module_platform_driver!(CLK_MT6795_INFRACFG_DRV);
module_description!("MediaTek MT6795 infracfg clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
