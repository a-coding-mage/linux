// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// Linux clock, platform, device-tree, and MediaTek clock framework dependencies
// are supplied by the surrounding kernel Rust bindings.

static mt8135_clk_lock: SpinLock = DEFINE_SPINLOCK!();

static top_divs: &[MtkFixedFactor] = &[
    FACTOR!(CLK_DUMMY, "top_divs_dummy", "clk_null", 1, 1),
    FACTOR!(CLK_TOP_DSI0_LNTC_DSICLK, "dsi0_lntc_dsiclk", "clk_null", 1, 1),
    FACTOR!(CLK_TOP_HDMITX_CLKDIG_CTS, "hdmitx_clkdig_cts", "clk_null", 1, 1),
    FACTOR!(CLK_TOP_CLKPH_MCK, "clkph_mck", "clk_null", 1, 1),
    FACTOR!(CLK_TOP_CPUM_TCK_IN, "cpum_tck_in", "clk_null", 1, 1),
    FACTOR!(CLK_TOP_MAINPLL_806M, "mainpll_806m", "mainpll", 1, 2),
    FACTOR!(CLK_TOP_MAINPLL_537P3M, "mainpll_537p3m", "mainpll", 1, 3),
    FACTOR!(CLK_TOP_MAINPLL_322P4M, "mainpll_322p4m", "mainpll", 1, 5),
    FACTOR!(CLK_TOP_MAINPLL_230P3M, "mainpll_230p3m", "mainpll", 1, 7),
    FACTOR!(CLK_TOP_UNIVPLL_624M, "univpll_624m", "univpll", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL_416M, "univpll_416m", "univpll", 1, 3),
    FACTOR!(CLK_TOP_UNIVPLL_249P6M, "univpll_249p6m", "univpll", 1, 5),
    FACTOR!(CLK_TOP_UNIVPLL_178P3M, "univpll_178p3m", "univpll", 1, 7),
    FACTOR!(CLK_TOP_UNIVPLL_48M, "univpll_48m", "univpll", 1, 26),
    FACTOR!(CLK_TOP_MMPLL_D2, "mmpll_d2", "mmpll", 1, 2),
    FACTOR!(CLK_TOP_MMPLL_D3, "mmpll_d3", "mmpll", 1, 3),
    FACTOR!(CLK_TOP_MMPLL_D5, "mmpll_d5", "mmpll", 1, 5),
    FACTOR!(CLK_TOP_MMPLL_D7, "mmpll_d7", "mmpll", 1, 7),
    FACTOR!(CLK_TOP_MMPLL_D4, "mmpll_d4", "mmpll_d2", 1, 2),
    FACTOR!(CLK_TOP_MMPLL_D6, "mmpll_d6", "mmpll_d3", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL_D2, "syspll_d2", "mainpll_806m", 1, 1),
    FACTOR!(CLK_TOP_SYSPLL_D4, "syspll_d4", "mainpll_806m", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL_D6, "syspll_d6", "mainpll_806m", 1, 3),
    FACTOR!(CLK_TOP_SYSPLL_D8, "syspll_d8", "mainpll_806m", 1, 4),
    FACTOR!(CLK_TOP_SYSPLL_D10, "syspll_d10", "mainpll_806m", 1, 5),
    FACTOR!(CLK_TOP_SYSPLL_D12, "syspll_d12", "mainpll_806m", 1, 6),
    FACTOR!(CLK_TOP_SYSPLL_D16, "syspll_d16", "mainpll_806m", 1, 8),
    FACTOR!(CLK_TOP_SYSPLL_D24, "syspll_d24", "mainpll_806m", 1, 12),
    FACTOR!(CLK_TOP_SYSPLL_D3, "syspll_d3", "mainpll_537p3m", 1, 1),
    FACTOR!(CLK_TOP_SYSPLL_D2P5, "syspll_d2p5", "mainpll_322p4m", 2, 1),
    FACTOR!(CLK_TOP_SYSPLL_D5, "syspll_d5", "mainpll_322p4m", 1, 1),
    FACTOR!(CLK_TOP_SYSPLL_D3P5, "syspll_d3p5", "mainpll_230p3m", 2, 1),
    FACTOR!(CLK_TOP_UNIVPLL1_D2, "univpll1_d2", "univpll_624m", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL1_D4, "univpll1_d4", "univpll_624m", 1, 4),
    FACTOR!(CLK_TOP_UNIVPLL1_D6, "univpll1_d6", "univpll_624m", 1, 6),
    FACTOR!(CLK_TOP_UNIVPLL1_D8, "univpll1_d8", "univpll_624m", 1, 8),
    FACTOR!(CLK_TOP_UNIVPLL1_D10, "univpll1_d10", "univpll_624m", 1, 10),
    FACTOR!(CLK_TOP_UNIVPLL2_D2, "univpll2_d2", "univpll_416m", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL2_D4, "univpll2_d4", "univpll_416m", 1, 4),
    FACTOR!(CLK_TOP_UNIVPLL2_D6, "univpll2_d6", "univpll_416m", 1, 6),
    FACTOR!(CLK_TOP_UNIVPLL2_D8, "univpll2_d8", "univpll_416m", 1, 8),
    FACTOR!(CLK_TOP_UNIVPLL_D3, "univpll_d3", "univpll_416m", 1, 1),
    FACTOR!(CLK_TOP_UNIVPLL_D5, "univpll_d5", "univpll_249p6m", 1, 1),
    FACTOR!(CLK_TOP_UNIVPLL_D7, "univpll_d7", "univpll_178p3m", 1, 1),
    FACTOR!(CLK_TOP_UNIVPLL_D10, "univpll_d10", "univpll_249p6m", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL_D26, "univpll_d26", "univpll_48m", 1, 1),
    FACTOR!(CLK_TOP_APLL, "apll_ck", "audpll", 1, 1),
    FACTOR!(CLK_TOP_APLL_D4, "apll_d4", "audpll", 1, 4),
    FACTOR!(CLK_TOP_APLL_D8, "apll_d8", "audpll", 1, 8),
    FACTOR!(CLK_TOP_APLL_D16, "apll_d16", "audpll", 1, 16),
    FACTOR!(CLK_TOP_APLL_D24, "apll_d24", "audpll", 1, 24),
    FACTOR!(CLK_TOP_LVDSPLL_D2, "lvdspll_d2", "lvdspll", 1, 2),
    FACTOR!(CLK_TOP_LVDSPLL_D4, "lvdspll_d4", "lvdspll", 1, 4),
    FACTOR!(CLK_TOP_LVDSPLL_D8, "lvdspll_d8", "lvdspll", 1, 8),
    FACTOR!(CLK_TOP_LVDSTX_CLKDIG_CT, "lvdstx_clkdig_cts", "lvdspll", 1, 1),
    FACTOR!(CLK_TOP_VPLL_DPIX, "vpll_dpix_ck", "lvdspll", 1, 1),
    FACTOR!(CLK_TOP_TVHDMI_H, "tvhdmi_h_ck", "tvdpll", 1, 1),
    FACTOR!(CLK_TOP_HDMITX_CLKDIG_D2, "hdmitx_clkdig_d2", "hdmitx_clkdig_cts", 1, 2),
    FACTOR!(CLK_TOP_HDMITX_CLKDIG_D3, "hdmitx_clkdig_d3", "hdmitx_clkdig_cts", 1, 3),
    FACTOR!(CLK_TOP_TVHDMI_D2, "tvhdmi_d2", "tvhdmi_h_ck", 1, 2),
    FACTOR!(CLK_TOP_TVHDMI_D4, "tvhdmi_d4", "tvhdmi_h_ck", 1, 4),
    FACTOR!(CLK_TOP_MEMPLL_MCK_D4, "mempll_mck_d4", "clkph_mck", 1, 4),
];

macro_rules! parents { ($($x:expr),* $(,)?) => { &[$($x),*] }; }
static axi_parents: &[&str] = parents!["clk26m", "syspll_d3", "syspll_d4", "syspll_d6", "univpll_d5", "univpll2_d2", "syspll_d3p5"];
static smi_parents: &[&str] = parents!["clk26m", "clkph_mck", "syspll_d2p5", "syspll_d3", "syspll_d8", "univpll_d5", "univpll1_d2", "univpll1_d6", "mmpll_d3", "mmpll_d4", "mmpll_d5", "mmpll_d6", "mmpll_d7", "vdecpll", "lvdspll"];
static mfg_parents: &[&str] = parents!["clk26m", "univpll1_d4", "syspll_d2", "syspll_d2p5", "syspll_d3", "univpll_d5", "univpll1_d2", "mmpll_d2", "mmpll_d3", "mmpll_d4", "mmpll_d5", "mmpll_d6", "mmpll_d7"];
static irda_parents: &[&str] = parents!["clk26m", "univpll2_d8", "univpll1_d6"];
static cam_parents: &[&str] = parents!["clk26m", "syspll_d3", "syspll_d3p5", "syspll_d4", "univpll_d5", "univpll2_d2", "univpll_d7", "univpll1_d4"];
static aud_intbus_parents: &[&str] = parents!["clk26m", "syspll_d6", "univpll_d10"];
static jpg_parents: &[&str] = parents!["clk26m", "syspll_d5", "syspll_d4", "syspll_d3", "univpll_d7", "univpll2_d2", "univpll_d5"];
static disp_parents: &[&str] = parents!["clk26m", "syspll_d3p5", "syspll_d3", "univpll2_d2", "univpll_d5", "univpll1_d2", "lvdspll", "vdecpll"];
static msdc30_parents: &[&str] = parents!["clk26m", "syspll_d6", "syspll_d5", "univpll1_d4", "univpll2_d4", "msdcpll"];
static usb20_parents: &[&str] = parents!["clk26m", "univpll2_d6", "univpll1_d10"];
static venc_parents: &[&str] = parents!["clk26m", "syspll_d3", "syspll_d8", "univpll_d5", "univpll1_d6", "mmpll_d4", "mmpll_d5", "mmpll_d6"];
static spi_parents: &[&str] = parents!["clk26m", "syspll_d6", "syspll_d8", "syspll_d10", "univpll1_d6", "univpll1_d8"];
static uart_parents: &[&str] = parents!["clk26m", "univpll2_d8"];
static mem_parents: &[&str] = parents!["clk26m", "clkph_mck"];
static camtg_parents: &[&str] = parents!["clk26m", "univpll_d26", "univpll1_d6", "syspll_d16", "syspll_d8"];
static audio_parents: &[&str] = parents!["clk26m", "syspll_d24"];
static fix_parents: &[&str] = parents!["rtc32k", "clk26m", "univpll_d5", "univpll_d7", "univpll1_d2", "univpll1_d4", "univpll1_d6", "univpll1_d8"];
static vdec_parents: &[&str] = parents!["clk26m", "vdecpll", "clkph_mck", "syspll_d2p5", "syspll_d3", "syspll_d3p5", "syspll_d4", "syspll_d5", "syspll_d6", "syspll_d8", "univpll1_d2", "univpll2_d2", "univpll_d7", "univpll_d10", "univpll2_d4", "lvdspll"];
static ddrphycfg_parents: &[&str] = parents!["clk26m", "axi_sel", "syspll_d12"];
static dpilvds_parents: &[&str] = parents!["clk26m", "lvdspll", "lvdspll_d2", "lvdspll_d4", "lvdspll_d8"];
static pmicspi_parents: &[&str] = parents!["clk26m", "univpll2_d6", "syspll_d8", "syspll_d10", "univpll1_d10", "mempll_mck_d4", "univpll_d26", "syspll_d24"];
static smi_mfg_as_parents: &[&str] = parents!["clk26m", "smi_sel", "mfg_sel", "mem_sel"];
static gcpu_parents: &[&str] = parents!["clk26m", "syspll_d4", "univpll_d7", "syspll_d5", "syspll_d6"];
static dpi1_parents: &[&str] = parents!["clk26m", "tvhdmi_h_ck", "tvhdmi_d2", "tvhdmi_d4"];
static cci_parents: &[&str] = parents!["clk26m", "mainpll_537p3m", "univpll_d3", "syspll_d2p5", "syspll_d3", "syspll_d5"];
static apll_parents: &[&str] = parents!["clk26m", "apll_ck", "apll_d4", "apll_d8", "apll_d16", "apll_d24"];
static hdmipll_parents: &[&str] = parents!["clk26m", "hdmitx_clkdig_cts", "hdmitx_clkdig_d2", "hdmitx_clkdig_d3"];

static top_muxes: &[MtkComposite] = &[
    MUX_GATE!(CLK_TOP_AXI_SEL, "axi_sel", axi_parents, 0x0140, 0, 3, INVALID_MUX_GATE_BIT),
    MUX_GATE!(CLK_TOP_SMI_SEL, "smi_sel", smi_parents, 0x0140, 8, 4, 15),
    MUX_GATE!(CLK_TOP_MFG_SEL, "mfg_sel", mfg_parents, 0x0140, 16, 4, 23),
    MUX_GATE!(CLK_TOP_IRDA_SEL, "irda_sel", irda_parents, 0x0140, 24, 2, 31),
    MUX_GATE!(CLK_TOP_CAM_SEL, "cam_sel", cam_parents, 0x0144, 0, 3, 7),
    MUX_GATE!(CLK_TOP_AUD_INTBUS_SEL, "aud_intbus_sel", aud_intbus_parents, 0x0144, 8, 2, 15),
    MUX_GATE!(CLK_TOP_JPG_SEL, "jpg_sel", jpg_parents, 0x0144, 16, 3, 23),
    MUX_GATE!(CLK_TOP_DISP_SEL, "disp_sel", disp_parents, 0x0144, 24, 3, 31),
    MUX_GATE!(CLK_TOP_MSDC30_1_SEL, "msdc30_1_sel", msdc30_parents, 0x0148, 0, 3, 7),
    MUX_GATE!(CLK_TOP_MSDC30_2_SEL, "msdc30_2_sel", msdc30_parents, 0x0148, 8, 3, 15),
    MUX_GATE!(CLK_TOP_MSDC30_3_SEL, "msdc30_3_sel", msdc30_parents, 0x0148, 16, 3, 23),
    MUX_GATE!(CLK_TOP_MSDC30_4_SEL, "msdc30_4_sel", msdc30_parents, 0x0148, 24, 3, 31),
    MUX_GATE!(CLK_TOP_USB20_SEL, "usb20_sel", usb20_parents, 0x014c, 0, 2, 7),
    MUX_GATE!(CLK_TOP_VENC_SEL, "venc_sel", venc_parents, 0x0150, 8, 3, 15),
    MUX_GATE!(CLK_TOP_SPI_SEL, "spi_sel", spi_parents, 0x0150, 16, 3, 23),
    MUX_GATE!(CLK_TOP_UART_SEL, "uart_sel", uart_parents, 0x0150, 24, 2, 31),
    MUX_GATE!(CLK_TOP_MEM_SEL, "mem_sel", mem_parents, 0x0158, 0, 2, 7),
    MUX_GATE!(CLK_TOP_CAMTG_SEL, "camtg_sel", camtg_parents, 0x0158, 8, 3, 15),
    MUX_GATE!(CLK_TOP_AUDIO_SEL, "audio_sel", audio_parents, 0x0158, 24, 2, 31),
    MUX_GATE!(CLK_TOP_FIX_SEL, "fix_sel", fix_parents, 0x015c, 0, 3, 7),
    MUX_GATE!(CLK_TOP_VDEC_SEL, "vdec_sel", vdec_parents, 0x015c, 8, 4, 15),
    MUX_GATE!(CLK_TOP_DDRPHYCFG_SEL, "ddrphycfg_sel", ddrphycfg_parents, 0x015c, 16, 2, 23),
    MUX_GATE!(CLK_TOP_DPILVDS_SEL, "dpilvds_sel", dpilvds_parents, 0x015c, 24, 3, 31),
    MUX_GATE!(CLK_TOP_PMICSPI_SEL, "pmicspi_sel", pmicspi_parents, 0x0164, 0, 3, 7),
    MUX_GATE!(CLK_TOP_MSDC30_0_SEL, "msdc30_0_sel", msdc30_parents, 0x0164, 8, 3, 15),
    MUX_GATE!(CLK_TOP_SMI_MFG_AS_SEL, "smi_mfg_as_sel", smi_mfg_as_parents, 0x0164, 16, 2, 23),
    MUX_GATE!(CLK_TOP_GCPU_SEL, "gcpu_sel", gcpu_parents, 0x0164, 24, 3, 31),
    MUX_GATE!(CLK_TOP_DPI1_SEL, "dpi1_sel", dpi1_parents, 0x0168, 0, 2, 7),
    MUX_GATE_FLAGS!(CLK_TOP_CCI_SEL, "cci_sel", cci_parents, 0x0168, 8, 3, 15, CLK_IS_CRITICAL),
    MUX_GATE!(CLK_TOP_APLL_SEL, "apll_sel", apll_parents, 0x0168, 16, 3, 23),
    MUX_GATE!(CLK_TOP_HDMIPLL_SEL, "hdmipll_sel", hdmipll_parents, 0x0168, 24, 2, 31),
];

static infra_cg_regs: MtkGateRegs = MtkGateRegs { set_ofs: 0x0040, clr_ofs: 0x0044, sta_ofs: 0x0048 };
static peri0_cg_regs: MtkGateRegs = MtkGateRegs { set_ofs: 0x0008, clr_ofs: 0x0010, sta_ofs: 0x0018 };
static peri1_cg_regs: MtkGateRegs = MtkGateRegs { set_ofs: 0x000c, clr_ofs: 0x0014, sta_ofs: 0x001c };

static infra_clks: &[MtkGate] = &[
    GATE_DUMMY!(CLK_DUMMY, "infra_dummy"),
    GATE_MTK!(CLK_INFRA_PMIC_WRAP, "pmic_wrap_ck", "axi_sel", &infra_cg_regs, 23, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_PMICSPI, "pmicspi_ck", "pmicspi_sel", &infra_cg_regs, 22, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CCIF1_AP_CTRL, "ccif1_ap_ctrl", "axi_sel", &infra_cg_regs, 21, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CCIF0_AP_CTRL, "ccif0_ap_ctrl", "axi_sel", &infra_cg_regs, 20, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_KP, "kp_ck", "axi_sel", &infra_cg_regs, 16, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_CPUM, "cpum_ck", "cpum_tck_in", &infra_cg_regs, 15, &mtk_clk_gate_ops_setclr),
    GATE_MTK_FLAGS!(CLK_INFRA_M4U, "m4u_ck", "mem_sel", &infra_cg_regs, 8, &mtk_clk_gate_ops_setclr, CLK_IS_CRITICAL),
    GATE_MTK!(CLK_INFRA_MFGAXI, "mfgaxi_ck", "axi_sel", &infra_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_DEVAPC, "devapc_ck", "axi_sel", &infra_cg_regs, 6, &mtk_clk_gate_ops_setclr_inv),
    GATE_MTK!(CLK_INFRA_AUDIO, "audio_ck", "aud_intbus_sel", &infra_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_MFG_BUS, "mfg_bus_ck", "axi_sel", &infra_cg_regs, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_SMI, "smi_ck", "smi_sel", &infra_cg_regs, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_INFRA_DBGCLK, "dbgclk_ck", "axi_sel", &infra_cg_regs, 0, &mtk_clk_gate_ops_setclr),
];

static peri_gates: &[MtkGate] = &[
    GATE_DUMMY!(CLK_DUMMY, "peri_dummy"),
    GATE_MTK!(CLK_PERI_I2C5, "i2c5_ck", "axi_sel", &peri0_cg_regs, 31, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C4, "i2c4_ck", "axi_sel", &peri0_cg_regs, 30, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C3, "i2c3_ck", "axi_sel", &peri0_cg_regs, 29, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C2, "i2c2_ck", "axi_sel", &peri0_cg_regs, 28, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C1, "i2c1_ck", "axi_sel", &peri0_cg_regs, 27, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C0, "i2c0_ck", "axi_sel", &peri0_cg_regs, 26, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_UART3, "uart3_ck", "axi_sel", &peri0_cg_regs, 25, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_UART2, "uart2_ck", "axi_sel", &peri0_cg_regs, 24, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_UART1, "uart1_ck", "axi_sel", &peri0_cg_regs, 23, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_UART0, "uart0_ck", "axi_sel", &peri0_cg_regs, 22, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_IRDA, "irda_ck", "irda_sel", &peri0_cg_regs, 21, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_NLI, "nli_ck", "axi_sel", &peri0_cg_regs, 20, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MD_HIF, "md_hif_ck", "axi_sel", &peri0_cg_regs, 19, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_AP_HIF, "ap_hif_ck", "axi_sel", &peri0_cg_regs, 18, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MSDC30_3, "msdc30_3_ck", "msdc30_4_sel", &peri0_cg_regs, 17, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MSDC30_2, "msdc30_2_ck", "msdc30_3_sel", &peri0_cg_regs, 16, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MSDC30_1, "msdc30_1_ck", "msdc30_2_sel", &peri0_cg_regs, 15, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MSDC20_2, "msdc20_2_ck", "msdc30_1_sel", &peri0_cg_regs, 14, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_MSDC20_1, "msdc20_1_ck", "msdc30_0_sel", &peri0_cg_regs, 13, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_AP_DMA, "ap_dma_ck", "axi_sel", &peri0_cg_regs, 12, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_USB1, "usb1_ck", "usb20_sel", &peri0_cg_regs, 11, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_USB0, "usb0_ck", "usb20_sel", &peri0_cg_regs, 10, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM, "pwm_ck", "axi_sel", &peri0_cg_regs, 9, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM7, "pwm7_ck", "axi_sel", &peri0_cg_regs, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM6, "pwm6_ck", "axi_sel", &peri0_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM5, "pwm5_ck", "axi_sel", &peri0_cg_regs, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM4, "pwm4_ck", "axi_sel", &peri0_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM3, "pwm3_ck", "axi_sel", &peri0_cg_regs, 4, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM2, "pwm2_ck", "axi_sel", &peri0_cg_regs, 3, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PWM1, "pwm1_ck", "axi_sel", &peri0_cg_regs, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_THERM, "therm_ck", "axi_sel", &peri0_cg_regs, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_NFI, "nfi_ck", "axi_sel", &peri0_cg_regs, 0, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_USBSLV, "usbslv_ck", "axi_sel", &peri1_cg_regs, 8, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_USB1_MCU, "usb1_mcu_ck", "axi_sel", &peri1_cg_regs, 7, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_USB0_MCU, "usb0_mcu_ck", "axi_sel", &peri1_cg_regs, 6, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_GCPU, "gcpu_ck", "gcpu_sel", &peri1_cg_regs, 5, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_FHCTL, "fhctl_ck", "clk26m", &peri1_cg_regs, 4, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_SPI1, "spi1_ck", "spi_sel", &peri1_cg_regs, 3, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_AUXADC, "auxadc_ck", "clk26m", &peri1_cg_regs, 2, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_PERI_PWRAP, "peri_pwrap_ck", "axi_sel", &peri1_cg_regs, 1, &mtk_clk_gate_ops_setclr),
    GATE_MTK!(CLK_PERI_I2C6, "i2c6_ck", "axi_sel", &peri1_cg_regs, 0, &mtk_clk_gate_ops_setclr),
];

static uart_ck_sel_parents: &[&str] = parents!["clk26m", "uart_sel"];
static peri_clks: &[MtkComposite] = &[
    MUX!(CLK_PERI_UART0_SEL, "uart0_ck_sel", uart_ck_sel_parents, 0x40c, 0, 1),
    MUX!(CLK_PERI_UART1_SEL, "uart1_ck_sel", uart_ck_sel_parents, 0x40c, 1, 1),
    MUX!(CLK_PERI_UART2_SEL, "uart2_ck_sel", uart_ck_sel_parents, 0x40c, 2, 1),
    MUX!(CLK_PERI_UART3_SEL, "uart3_ck_sel", uart_ck_sel_parents, 0x40c, 3, 1),
];

static infrasys_rst_ofs: &[u16] = &[0x30, 0x34];
static pericfg_rst_ofs: &[u16] = &[0x0, 0x4];
static clk_rst_desc: &[MtkClkRstDesc] = &[
    MtkClkRstDesc { version: MTK_RST_SIMPLE, rst_bank_ofs: infrasys_rst_ofs, rst_bank_nr: ARRAY_SIZE!(infrasys_rst_ofs) },
    MtkClkRstDesc { version: MTK_RST_SIMPLE, rst_bank_ofs: pericfg_rst_ofs, rst_bank_nr: ARRAY_SIZE!(pericfg_rst_ofs) },
];
static infra_desc: MtkClkDesc = MtkClkDesc { clks: infra_clks, num_clks: ARRAY_SIZE!(infra_clks), rst_desc: &clk_rst_desc[0] };
static peri_desc: MtkClkDesc = MtkClkDesc { clks: peri_gates, num_clks: ARRAY_SIZE!(peri_gates), composite_clks: peri_clks, num_composite_clks: ARRAY_SIZE!(peri_clks), clk_lock: &mt8135_clk_lock, rst_desc: &clk_rst_desc[1] };
static topck_desc: MtkClkDesc = MtkClkDesc { factor_clks: top_divs, num_factor_clks: ARRAY_SIZE!(top_divs), composite_clks: top_muxes, num_composite_clks: ARRAY_SIZE!(top_muxes), clk_lock: &mt8135_clk_lock };

static of_match_clk_mt8135: &[OfDeviceId] = &[
    OfDeviceId { compatible: "mediatek,mt8135-infracfg", data: &infra_desc },
    OfDeviceId { compatible: "mediatek,mt8135-pericfg", data: &peri_desc },
    OfDeviceId { compatible: "mediatek,mt8135-topckgen", data: &topck_desc },
    OfDeviceId::SENTINEL,
];
MODULE_DEVICE_TABLE!(of, of_match_clk_mt8135);
static mut clk_mt8135_drv: PlatformDriver = PlatformDriver {
    driver: Driver { name: "clk-mt8135", of_match_table: of_match_clk_mt8135 },
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
};
module_platform_driver!(clk_mt8135_drv);
MODULE_DESCRIPTION!("MediaTek MT8135 clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
