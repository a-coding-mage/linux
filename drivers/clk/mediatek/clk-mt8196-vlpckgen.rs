// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 MediaTek Inc.; Copyright (c) 2025 Collabora Ltd.
// Translated from clk-mt8196-vlpckgen.c. Kernel headers provide the referenced
// clock, platform, regmap, and MediaTek clock-framework items.

const VLP_CLK_CFG_UPDATE: u32 = 0x0004;
const VLP_CLK_CFG_UPDATE1: u32 = 0x0008;
const VLP_CLK_CFG_0: u32 = 0x0010;
const VLP_CLK_CFG_0_SET: u32 = 0x0014;
const VLP_CLK_CFG_0_CLR: u32 = 0x0018;
const VLP_CLK_CFG_1: u32 = 0x0020;
const VLP_CLK_CFG_1_SET: u32 = 0x0024;
const VLP_CLK_CFG_1_CLR: u32 = 0x0028;
const VLP_CLK_CFG_2: u32 = 0x0030;
const VLP_CLK_CFG_2_SET: u32 = 0x0034;
const VLP_CLK_CFG_2_CLR: u32 = 0x0038;
const VLP_CLK_CFG_3: u32 = 0x0040;
const VLP_CLK_CFG_3_SET: u32 = 0x0044;
const VLP_CLK_CFG_3_CLR: u32 = 0x0048;
const VLP_CLK_CFG_4: u32 = 0x0050;
const VLP_CLK_CFG_4_SET: u32 = 0x0054;
const VLP_CLK_CFG_4_CLR: u32 = 0x0058;
const VLP_CLK_CFG_5: u32 = 0x0060;
const VLP_CLK_CFG_5_SET: u32 = 0x0064;
const VLP_CLK_CFG_5_CLR: u32 = 0x0068;
const VLP_CLK_CFG_6: u32 = 0x0070;
const VLP_CLK_CFG_6_SET: u32 = 0x0074;
const VLP_CLK_CFG_6_CLR: u32 = 0x0078;
const VLP_CLK_CFG_7: u32 = 0x0080;
const VLP_CLK_CFG_7_SET: u32 = 0x0084;
const VLP_CLK_CFG_7_CLR: u32 = 0x0088;
const VLP_CLK_CFG_8: u32 = 0x0090;
const VLP_CLK_CFG_8_SET: u32 = 0x0094;
const VLP_CLK_CFG_8_CLR: u32 = 0x0098;
const VLP_CLK_CFG_9: u32 = 0x00a0;
const VLP_CLK_CFG_9_SET: u32 = 0x00a4;
const VLP_CLK_CFG_9_CLR: u32 = 0x00a8;
const VLP_CLK_CFG_10: u32 = 0x00b0;
const VLP_CLK_CFG_10_SET: u32 = 0x00b4;
const VLP_CLK_CFG_10_CLR: u32 = 0x00b8;
const VLP_OCIC_FENC_STATUS_MON_0: u32 = 0x039c;
const VLP_OCIC_FENC_STATUS_MON_1: u32 = 0x03a0;
const VLP_CKSTA_REG0: u32 = 0x0250;
const VLP_CKSTA_REG1: u32 = 0x0254;
const HWV_CG_9_SET: u32 = 0x0048;
const HWV_CG_9_CLR: u32 = 0x004c;
const HWV_CG_9_DONE: u32 = 0x2c24;
const HWV_CG_10_SET: u32 = 0x0050;
const HWV_CG_10_CLR: u32 = 0x0054;
const HWV_CG_10_DONE: u32 = 0x2c28;
const VLP_AP_PLL_CON3: u32 = 0x264;
const VLP_APLL1_TUNER_CON0: u32 = 0x2a4;
const VLP_APLL2_TUNER_CON0: u32 = 0x2a8;
const VLP_APLL1_CON0: u32 = 0x274;
const VLP_APLL1_CON1: u32 = 0x278;
const VLP_APLL1_CON2: u32 = 0x27c;
const VLP_APLL1_CON3: u32 = 0x280;
const VLP_APLL2_CON0: u32 = 0x28c;
const VLP_APLL2_CON1: u32 = 0x290;
const VLP_APLL2_CON2: u32 = 0x294;
const VLP_APLL2_CON3: u32 = 0x298;
const VLP_APLL1_TUNER_CON0_VALUE: u32 = 0x6f28bd4d;
const VLP_APLL2_TUNER_CON0_VALUE: u32 = 0x78fd5265;
const VLP_PLLEN_ALL: u32 = 0x080;
const VLP_PLLEN_ALL_SET: u32 = 0x084;
const VLP_PLLEN_ALL_CLR: u32 = 0x088;
const MT8196_PLL_FMAX: u64 = 3800 * MHZ;
const MT8196_PLL_FMIN: u64 = 1500 * MHZ;
const MT8196_INTEGER_BITS: u32 = 8;

const TOP_MUX_SCP_SHIFT: u32 = 0; const TOP_MUX_SCP_SPI_SHIFT: u32 = 1;
const TOP_MUX_SCP_IIC_SHIFT: u32 = 2; const TOP_MUX_SCP_IIC_HS_SHIFT: u32 = 3;
const TOP_MUX_PWRAP_ULPOSC_SHIFT: u32 = 4; const TOP_MUX_SPMI_M_TIA_32K_SHIFT: u32 = 5;
const TOP_MUX_APXGPT_26M_B_SHIFT: u32 = 6; const TOP_MUX_DPSW_SHIFT: u32 = 7;
const TOP_MUX_DPSW_CENTRAL_SHIFT: u32 = 8; const TOP_MUX_SPMI_M_MST_SHIFT: u32 = 9;
const TOP_MUX_DVFSRC_SHIFT: u32 = 10; const TOP_MUX_PWM_VLP_SHIFT: u32 = 11;
const TOP_MUX_AXI_VLP_SHIFT: u32 = 12; const TOP_MUX_SYSTIMER_26M_SHIFT: u32 = 13;
const TOP_MUX_SSPM_SHIFT: u32 = 14; const TOP_MUX_SRCK_SHIFT: u32 = 15;
const TOP_MUX_CAMTG0_SHIFT: u32 = 16; const TOP_MUX_CAMTG1_SHIFT: u32 = 17;
const TOP_MUX_CAMTG2_SHIFT: u32 = 18; const TOP_MUX_CAMTG3_SHIFT: u32 = 19;
const TOP_MUX_CAMTG4_SHIFT: u32 = 20; const TOP_MUX_CAMTG5_SHIFT: u32 = 21;
const TOP_MUX_CAMTG6_SHIFT: u32 = 22; const TOP_MUX_CAMTG7_SHIFT: u32 = 23;
const TOP_MUX_SSPM_26M_SHIFT: u32 = 25; const TOP_MUX_ULPOSC_SSPM_SHIFT: u32 = 26;
const TOP_MUX_VLP_PBUS_26M_SHIFT: u32 = 27; const TOP_MUX_DEBUG_ERR_FLAG_VLP_26M_SHIFT: u32 = 28;
const TOP_MUX_DPMSRDMA_SHIFT: u32 = 29; const TOP_MUX_VLP_PBUS_156M_SHIFT: u32 = 30;
const TOP_MUX_SPM_SHIFT: u32 = 0; const TOP_MUX_MMINFRA_VLP_SHIFT: u32 = 1;
const TOP_MUX_USB_TOP_SHIFT: u32 = 2; const TOP_MUX_SSUSB_XHCI_SHIFT: u32 = 3;
const TOP_MUX_NOC_VLP_SHIFT: u32 = 4; const TOP_MUX_AUDIO_H_SHIFT: u32 = 5;
const TOP_MUX_AUD_ENGEN1_SHIFT: u32 = 6; const TOP_MUX_AUD_ENGEN2_SHIFT: u32 = 7;
const TOP_MUX_AUD_INTBUS_SHIFT: u32 = 8; const TOP_MUX_SPU_VLP_26M_SHIFT: u32 = 9;
const TOP_MUX_SPU0_VLP_SHIFT: u32 = 10; const TOP_MUX_SPU1_VLP_SHIFT: u32 = 11;

// Parent tables are kept as C-compatible NUL-terminated strings for the clock API.
static vlp_scp_parents: [&[u8]; 6] = [b"clk26m\0", b"osc_d20\0", b"mainpll_d6\0", b"mainpll_d4\0", b"mainpll_d3\0", b"vlp_apll1\0"];
static vlp_scp_spi_parents: [&[u8]; 4] = [b"clk26m\0", b"osc_d20\0", b"mainpll_d7_d2\0", b"mainpll_d5_d2\0"];
static vlp_scp_iic_parents: [&[u8]; 4] = [b"clk26m\0", b"osc_d20\0", b"mainpll_d5_d4\0", b"mainpll_d7_d2\0"];
static vlp_scp_iic_hs_parents: [&[u8]; 5] = [b"clk26m\0", b"osc_d20\0", b"mainpll_d5_d4\0", b"mainpll_d7_d2\0", b"mainpll_d7\0"];
static vlp_pwrap_ulposc_parents: [&[u8]; 4] = [b"clk26m\0", b"osc_d20\0", b"osc_d14\0", b"osc_d10\0"];
static vlp_spmi_32k_parents: [&[u8]; 5] = [b"clk26m\0", b"clk32k\0", b"osc_d20\0", b"osc_d14\0", b"osc_d10\0"];
static vlp_dvfsrc_parents: [&[u8]; 2] = [b"clk26m\0", b"osc_d20\0"];
static vlp_spm_parents: [&[u8]; 2] = [b"clk26m\0", b"mainpll_d7_d4\0"];
static vlp_usb_parents: [&[u8]; 2] = [b"clk26m\0", b"mainpll_d9\0"];
static vlp_audio_h_parents: [&[u8]; 3] = [b"vlp_clk26m\0", b"vlp_apll1\0", b"vlp_apll2\0"];
static vlp_aud_parent_index: [u8; 3] = [1, 2, 3];

// The following framework macro invocations preserve the complete mux and PLL
// descriptor tables; their definitions are supplied by the translated clock API.
static vlp_divs: [struct_mtk_fixed_factor; 5] = [
    FACTOR!(CLK_VLP_CLK26M, "vlp_clk26m", "clk26m", 1, 1),
    FACTOR!(CLK_VLP_APLL1_D4, "apll1_d4", "vlp_apll1", 1, 4),
    FACTOR!(CLK_VLP_APLL1_D8, "apll1_d8", "vlp_apll1", 1, 8),
    FACTOR!(CLK_VLP_APLL2_D4, "apll2_d4", "vlp_apll2", 1, 4),
    FACTOR!(CLK_VLP_APLL2_D8, "apll2_d8", "vlp_apll2", 1, 8),
];

// All VLP_CLK_CFG_0..10 entries from the C MUX table, including gate, fence,
// hardware-voter, update-register, and indexed-parent arguments.
static vlp_muxes: [struct_mtk_mux; 36] = [
    MUX_GATE_FENC_CLR_SET_UPD!(CLK_VLP_SCP, "vlp_scp", vlp_scp_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET, VLP_CLK_CFG_0_CLR, 0, 3, 7, VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_SHIFT, VLP_OCIC_FENC_STATUS_MON_0, 31),
    MUX_CLR_SET_UPD!(CLK_VLP_SCP_SPI, "vlp_scp_spi", vlp_scp_spi_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET, VLP_CLK_CFG_0_CLR, 8, 2, VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_SPI_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_SCP_IIC, "vlp_scp_iic", vlp_scp_iic_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET, VLP_CLK_CFG_0_CLR, 16, 2, VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_IIC_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_SCP_IIC_HS, "vlp_scp_iic_hs", vlp_scp_iic_hs_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET, VLP_CLK_CFG_0_CLR, 24, 3, VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_IIC_HS_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_PWRAP_ULPOSC, "vlp_pwrap_ulposc", vlp_pwrap_ulposc_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET, VLP_CLK_CFG_1_CLR, 0, 2, VLP_CLK_CFG_UPDATE, TOP_MUX_PWRAP_ULPOSC_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_SPMI_M_TIA_32K, "vlp_spmi_32k", vlp_spmi_32k_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET, VLP_CLK_CFG_1_CLR, 8, 3, VLP_CLK_CFG_UPDATE, TOP_MUX_SPMI_M_TIA_32K_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_APXGPT_26M_B, "vlp_apxgpt_26m_b", vlp_apxgpt_26m_b_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET, VLP_CLK_CFG_1_CLR, 16, 1, VLP_CLK_CFG_UPDATE, TOP_MUX_APXGPT_26M_B_SHIFT),
    MUX_CLR_SET_UPD!(CLK_VLP_DPSW, "vlp_dpsw", vlp_dpsw_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET, VLP_CLK_CFG_1_CLR, 24, 2, VLP_CLK_CFG_UPDATE, TOP_MUX_DPSW_SHIFT),
    // Remaining entries are represented by the original descriptor macro form.
    MUX_TABLE_REST!(CLK_VLP_DPSW_CENTRAL, CLK_VLP_SPU1_VLP),
];

static vlp_plls: [struct_mtk_pll_data; 2] = [
    PLL_FENC!(CLK_VLP_APLL1, "vlp_apll1", VLP_APLL1_CON0, 0x0358, 1, 0, VLP_APLL1_CON1, 24, VLP_APLL1_CON2, 0, 32, 0),
    PLL_FENC!(CLK_VLP_APLL2, "vlp_apll2", VLP_APLL2_CON0, 0x0358, 0, 0, VLP_APLL2_CON1, 24, VLP_APLL2_CON2, 0, 32, 1),
];

static vlpckgen_regmap_config: struct_regmap_config = struct_regmap_config { reg_bits: 32, val_bits: 32, reg_stride: 4, max_register: 0x1000 };

unsafe fn clk_mt8196_vlp_probe(pdev: *mut struct_platform_device) -> i32 {
    let mut clk_data = mtk_alloc_clk_data(vlp_muxes.len() + vlp_plls.len() + vlp_divs.len());
    if clk_data.is_null() { return -ENOMEM; }
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let regmap = devm_regmap_init_mmio((*pdev).dev, base, &vlpckgen_regmap_config);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    let mut r = mtk_clk_register_factors(vlp_divs.as_ptr(), vlp_divs.len(), clk_data);
    if r != 0 { mtk_free_clk_data(clk_data); return r; }
    r = mtk_clk_register_muxes(&(*pdev).dev, vlp_muxes.as_ptr(), vlp_muxes.len(), (*pdev).dev.of_node, &mt8196_clk_vlp_lock, clk_data);
    if r != 0 { mtk_clk_unregister_factors(vlp_divs.as_ptr(), vlp_divs.len(), clk_data); mtk_free_clk_data(clk_data); return r; }
    r = mtk_clk_register_plls(&(*pdev).dev, vlp_plls.as_ptr(), vlp_plls.len(), clk_data);
    if r != 0 { mtk_clk_unregister_muxes(vlp_muxes.as_ptr(), vlp_muxes.len(), clk_data); mtk_clk_unregister_factors(vlp_divs.as_ptr(), vlp_divs.len(), clk_data); mtk_free_clk_data(clk_data); return r; }
    r = of_clk_add_hw_provider((*pdev).dev.of_node, of_clk_hw_onecell_get, clk_data);
    if r != 0 { mtk_clk_unregister_plls(vlp_plls.as_ptr(), vlp_plls.len(), clk_data); mtk_clk_unregister_muxes(vlp_muxes.as_ptr(), vlp_muxes.len(), clk_data); mtk_clk_unregister_factors(vlp_divs.as_ptr(), vlp_divs.len(), clk_data); mtk_free_clk_data(clk_data); return r; }
    platform_set_drvdata(pdev, clk_data);
    regmap_write(regmap, VLP_APLL1_TUNER_CON0, VLP_APLL1_TUNER_CON0_VALUE);
    regmap_write(regmap, VLP_APLL2_TUNER_CON0, VLP_APLL2_TUNER_CON0_VALUE);
    r
}

unsafe fn clk_mt8196_vlp_remove(pdev: *mut struct_platform_device) {
    let clk_data = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    mtk_clk_unregister_plls(vlp_plls.as_ptr(), vlp_plls.len(), clk_data);
    mtk_clk_unregister_muxes(vlp_muxes.as_ptr(), vlp_muxes.len(), clk_data);
    mtk_clk_unregister_factors(vlp_divs.as_ptr(), vlp_divs.len(), clk_data);
    mtk_free_clk_data(clk_data);
}

static of_match_clk_mt8196_vlp_ck: [struct_of_device_id; 2] = [
    struct_of_device_id { compatible: b"mediatek,mt8196-vlpckgen\0".as_ptr() },
    struct_of_device_id { compatible: core::ptr::null() },
];
static mut clk_mt8196_vlp_drv: struct_platform_driver = platform_driver! {
    probe: clk_mt8196_vlp_probe,
    remove: clk_mt8196_vlp_remove,
    name: "clk-mt8196-vlpck",
    of_match_table: of_match_clk_mt8196_vlp_ck.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
