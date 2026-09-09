// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 * Copyright (C) 2023 Collabora Ltd.
 *                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */
// Dependencies are supplied by the surrounding kernel clock framework.

static mt8365_clk_lock: SpinLock = DEFINE_SPINLOCK!();

static top_fixed_clks: [mtk_fixed_clk; 5] = [
    FIXED_CLK!(CLK_TOP_CLK_NULL, "clk_null", None, 0),
    FIXED_CLK!(CLK_TOP_I2S0_BCK, "i2s0_bck", None, 26000000),
    FIXED_CLK!(CLK_TOP_DSI0_LNTC_DSICK, "dsi0_lntc_dsick", Some("clk26m"), 75000000),
    FIXED_CLK!(CLK_TOP_VPLL_DPIX, "vpll_dpix", Some("clk26m"), 75000000),
    FIXED_CLK!(CLK_TOP_LVDSTX_CLKDIG_CTS, "lvdstx_dig_cts", Some("clk26m"), 52500000),
];

static top_divs: [mtk_fixed_factor; 52] = [
    FACTOR!(CLK_TOP_SYS_26M_D2, "sys_26m_d2", "clk26m", 1, 2), FACTOR!(CLK_TOP_SYSPLL_D2, "syspll_d2", "mainpll", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL1_D2, "syspll1_d2", "mainpll", 1, 4), FACTOR!(CLK_TOP_SYSPLL1_D4, "syspll1_d4", "mainpll", 1, 8),
    FACTOR!(CLK_TOP_SYSPLL1_D8, "syspll1_d8", "mainpll", 1, 16), FACTOR!(CLK_TOP_SYSPLL1_D16, "syspll1_d16", "mainpll", 1, 32),
    FACTOR!(CLK_TOP_SYSPLL_D3, "syspll_d3", "mainpll", 1, 3), FACTOR!(CLK_TOP_SYSPLL2_D2, "syspll2_d2", "mainpll", 1, 6),
    FACTOR!(CLK_TOP_SYSPLL2_D4, "syspll2_d4", "mainpll", 1, 12), FACTOR!(CLK_TOP_SYSPLL2_D8, "syspll2_d8", "mainpll", 1, 24),
    FACTOR!(CLK_TOP_SYSPLL_D5, "syspll_d5", "mainpll", 1, 5), FACTOR!(CLK_TOP_SYSPLL3_D2, "syspll3_d2", "mainpll", 1, 10),
    FACTOR!(CLK_TOP_SYSPLL3_D4, "syspll3_d4", "mainpll", 1, 20), FACTOR!(CLK_TOP_SYSPLL_D7, "syspll_d7", "mainpll", 1, 7),
    FACTOR!(CLK_TOP_SYSPLL4_D2, "syspll4_d2", "mainpll", 1, 14), FACTOR!(CLK_TOP_SYSPLL4_D4, "syspll4_d4", "mainpll", 1, 28),
    FACTOR!(CLK_TOP_UNIVPLL, "univpll", "univ_en", 1, 2), FACTOR!(CLK_TOP_UNIVPLL_D2, "univpll_d2", "univpll", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL1_D2, "univpll1_d2", "univpll", 1, 4), FACTOR!(CLK_TOP_UNIVPLL1_D4, "univpll1_d4", "univpll", 1, 8),
    FACTOR!(CLK_TOP_UNIVPLL_D3, "univpll_d3", "univpll", 1, 3), FACTOR!(CLK_TOP_UNIVPLL2_D2, "univpll2_d2", "univpll", 1, 6),
    FACTOR!(CLK_TOP_UNIVPLL2_D4, "univpll2_d4", "univpll", 1, 12), FACTOR!(CLK_TOP_UNIVPLL2_D8, "univpll2_d8", "univpll", 1, 24),
    FACTOR!(CLK_TOP_UNIVPLL2_D32, "univpll2_d32", "univpll", 1, 96), FACTOR!(CLK_TOP_UNIVPLL_D5, "univpll_d5", "univpll", 1, 5),
    FACTOR!(CLK_TOP_UNIVPLL3_D2, "univpll3_d2", "univpll", 1, 10), FACTOR!(CLK_TOP_UNIVPLL3_D4, "univpll3_d4", "univpll", 1, 20),
    FACTOR!(CLK_TOP_MMPLL, "mmpll_ck", "mmpll", 1, 1), FACTOR!(CLK_TOP_MMPLL_D2, "mmpll_d2", "mmpll", 1, 2),
    FACTOR!(CLK_TOP_MFGPLL, "mfgpll_ck", "mfgpll", 1, 1), FACTOR!(CLK_TOP_LVDSPLL_D2, "lvdspll_d2", "lvdspll", 1, 2),
    FACTOR!(CLK_TOP_LVDSPLL_D4, "lvdspll_d4", "lvdspll", 1, 4), FACTOR!(CLK_TOP_LVDSPLL_D8, "lvdspll_d8", "lvdspll", 1, 8),
    FACTOR!(CLK_TOP_LVDSPLL_D16, "lvdspll_d16", "lvdspll", 1, 16), FACTOR!(CLK_TOP_USB20_192M, "usb20_192m_ck", "usb20_en", 1, 13),
    FACTOR!(CLK_TOP_USB20_192M_D4, "usb20_192m_d4", "usb20_192m_ck", 1, 4), FACTOR!(CLK_TOP_USB20_192M_D8, "usb20_192m_d8", "usb20_192m_ck", 1, 8),
    FACTOR!(CLK_TOP_USB20_192M_D16, "usb20_192m_d16", "usb20_192m_ck", 1, 16), FACTOR!(CLK_TOP_USB20_192M_D32, "usb20_192m_d32", "usb20_192m_ck", 1, 32),
    FACTOR!(CLK_TOP_APLL1, "apll1_ck", "apll1", 1, 1), FACTOR!(CLK_TOP_APLL1_D2, "apll1_d2", "apll1_ck", 1, 2), FACTOR!(CLK_TOP_APLL1_D4, "apll1_d4", "apll1_ck", 1, 4), FACTOR!(CLK_TOP_APLL1_D8, "apll1_d8", "apll1_ck", 1, 8),
    FACTOR!(CLK_TOP_APLL2, "apll2_ck", "apll2", 1, 1), FACTOR!(CLK_TOP_APLL2_D2, "apll2_d2", "apll2_ck", 1, 2), FACTOR!(CLK_TOP_APLL2_D4, "apll2_d4", "apll2_ck", 1, 4), FACTOR!(CLK_TOP_APLL2_D8, "apll2_d8", "apll2_ck", 1, 8),
    FACTOR!(CLK_TOP_MSDCPLL, "msdcpll_ck", "msdcpll", 1, 1), FACTOR!(CLK_TOP_MSDCPLL_D2, "msdcpll_d2", "msdcpll", 1, 2), FACTOR!(CLK_TOP_DSPPLL, "dsppll_ck", "dsppll", 1, 1), FACTOR!(CLK_TOP_DSPPLL_D2, "dsppll_d2", "dsppll", 1, 2), FACTOR!(CLK_TOP_DSPPLL_D4, "dsppll_d4", "dsppll", 1, 4), FACTOR!(CLK_TOP_DSPPLL_D8, "dsppll_d8", "dsppll", 1, 8), FACTOR!(CLK_TOP_APUPLL, "apupll_ck", "apupll", 1, 1), FACTOR!(CLK_TOP_CLK26M_D52, "clk26m_d52", "clk26m", 1, 52),
];

macro_rules! parents { ($($x:literal),* $(,)?) => { &[$($x),*] }; }
static axi_parents: &[&str] = parents!("clk26m", "syspll_d7", "syspll1_d4", "syspll3_d2");
static mem_parents: &[&str] = parents!("clk26m", "mmpll_ck", "syspll_d3", "syspll1_d2");
static mm_parents: &[&str] = parents!("clk26m", "mmpll_ck", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll_d5", "univpll1_d2", "mmpll_d2");
static scp_parents: &[&str] = parents!("clk26m", "syspll4_d2", "univpll2_d2", "syspll1_d2", "univpll1_d2", "syspll_d3", "univpll_d3");
static mfg_parents: &[&str] = parents!("clk26m", "mfgpll_ck", "syspll_d3", "univpll_d3");
static atb_parents: &[&str] = parents!("clk26m", "syspll1_d4", "syspll1_d2");
static camtg_parents: &[&str] = parents!("clk26m", "usb20_192m_d8", "univpll2_d8", "usb20_192m_d4", "univpll2_d32", "usb20_192m_d16", "usb20_192m_d32");
static uart_parents: &[&str] = parents!("clk26m", "univpll2_d8");
static spi_parents: &[&str] = parents!("clk26m", "univpll2_d2", "univpll2_d4", "univpll2_d8");
static msdc50_0_hc_parents: &[&str] = parents!("clk26m", "syspll1_d2", "univpll1_d4", "syspll2_d2");
static msdc50_0_parents: &[&str] = parents!("clk26m", "msdcpll_ck", "univpll1_d2", "syspll1_d2", "univpll_d5", "syspll2_d2", "univpll1_d4", "syspll4_d2");
static msdc50_2_parents: &[&str] = parents!("clk26m", "msdcpll_ck", "univpll_d3", "univpll1_d2", "syspll1_d2", "univpll2_d2", "syspll2_d2", "univpll1_d4");
static msdc30_1_parents: &[&str] = parents!("clk26m", "msdcpll_d2", "univpll2_d2", "syspll2_d2", "univpll1_d4", "syspll1_d4", "syspll2_d4", "univpll2_d8");
static audio_parents: &[&str] = parents!("clk26m", "syspll3_d4", "syspll4_d4", "syspll1_d16");
static aud_intbus_parents: &[&str] = parents!("clk26m", "syspll1_d4", "syspll4_d2");
static aud_1_parents: &[&str] = parents!("clk26m", "apll1_ck");
static aud_2_parents: &[&str] = parents!("clk26m", "apll2_ck");
static aud_engen1_parents: &[&str] = parents!("clk26m", "apll1_d2", "apll1_d4", "apll1_d8");
static aud_engen2_parents: &[&str] = parents!("clk26m", "apll2_d2", "apll2_d4", "apll2_d8");
static aud_spdif_parents: &[&str] = parents!("clk26m", "univpll_d2");
static disp_pwm_parents: &[&str] = parents!("clk26m", "univpll2_d4");
static dxcc_parents: &[&str] = parents!("clk26m", "syspll1_d2", "syspll1_d4", "syspll1_d8");
static ssusb_sys_parents: &[&str] = parents!("clk26m", "univpll3_d4", "univpll2_d4", "univpll3_d2");
static spm_parents: &[&str] = parents!("clk26m", "syspll1_d8");
static i2c_parents: &[&str] = parents!("clk26m", "univpll3_d4", "univpll3_d2", "syspll1_d8", "syspll2_d8");
static pwm_parents: &[&str] = parents!("clk26m", "univpll3_d4", "syspll1_d8");
static senif_parents: &[&str] = parents!("clk26m", "univpll1_d4", "univpll1_d2", "univpll2_d2");
static aes_fde_parents: &[&str] = parents!("clk26m", "msdcpll_ck", "univpll_d3", "univpll2_d2", "univpll1_d2", "syspll1_d2");
static dpi0_parents: &[&str] = parents!("clk26m", "lvdspll_d2", "lvdspll_d4", "lvdspll_d8", "lvdspll_d16");
static dsp_parents: &[&str] = parents!("clk26m", "sys_26m_d2", "dsppll_ck", "dsppll_d2", "dsppll_d4", "dsppll_d8");
static nfi2x_parents: &[&str] = parents!("clk26m", "syspll2_d2", "syspll_d7", "syspll_d3", "syspll2_d4", "msdcpll_d2", "univpll1_d2", "univpll_d5");
static nfiecc_parents: &[&str] = parents!("clk26m", "syspll4_d2", "univpll2_d4", "syspll_d7", "univpll1_d2", "syspll1_d2", "univpll2_d2", "syspll_d5");
static ecc_parents: &[&str] = parents!("clk26m", "univpll2_d2", "univpll1_d2", "univpll_d3", "syspll_d2");
static eth_parents: &[&str] = parents!("clk26m", "univpll2_d8", "syspll4_d4", "syspll1_d8", "syspll4_d2");
static gcpu_parents: &[&str] = parents!("clk26m", "univpll_d3", "univpll2_d2", "syspll_d3", "syspll2_d2");
static gcpu_cpm_parents: &[&str] = parents!("clk26m", "univpll2_d2", "syspll2_d2");
static apu_parents: &[&str] = parents!("clk26m", "univpll_d2", "apupll_ck", "mmpll_ck", "syspll_d3", "univpll1_d2", "syspll1_d2", "syspll1_d4");
static mbist_diag_parents: &[&str] = parents!("clk26m", "syspll4_d4", "univpll2_d8");
static apll_i2s_parents: &[&str] = parents!("aud_1_sel", "aud_2_sel");

const CLK_CFG_UPDATE: u32 = 0x004;
const CLK_CFG_UPDATE1: u32 = 0x008;

static mut top_misc_muxes: [mtk_composite; 7] = [
    MUX_GATE!(CLK_TOP_MBIST_DIAG_SEL, "mbist_diag_sel", mbist_diag_parents, 0x0ec, 0, 2, 7),
    MUX!(CLK_TOP_APLL_I2S0_SEL, "apll_i2s0_sel", apll_i2s_parents, 0x0320, 11, 1), MUX!(CLK_TOP_APLL_I2S1_SEL, "apll_i2s1_sel", apll_i2s_parents, 0x0320, 12, 1),
    MUX!(CLK_TOP_APLL_I2S2_SEL, "apll_i2s2_sel", apll_i2s_parents, 0x0320, 13, 1), MUX!(CLK_TOP_APLL_I2S3_SEL, "apll_i2s3_sel", apll_i2s_parents, 0x0320, 14, 1),
    MUX!(CLK_TOP_APLL_TDMOUT_SEL, "apll_tdmout_sel", apll_i2s_parents, 0x0320, 15, 1), MUX!(CLK_TOP_APLL_TDMIN_SEL, "apll_tdmin_sel", apll_i2s_parents, 0x0320, 16, 1), MUX!(CLK_TOP_APLL_SPDIF_SEL, "apll_spdif_sel", apll_i2s_parents, 0x0320, 17, 1),
];

// The following table entries preserve the source driver's declarative clock topology.
static top_muxes: &[mtk_mux] = &[
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_AXI_SEL, "axi_sel", axi_parents, 0x040, 0x044, 0x048, 0, 2, 7, CLK_CFG_UPDATE, 0, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MEM_SEL, "mem_sel", mem_parents, 0x040, 0x044, 0x048, 8, 2, 15, CLK_CFG_UPDATE, 1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MM_SEL, "mm_sel", mm_parents, 0x040, 0x044, 0x048, 16, 3, 23, CLK_CFG_UPDATE, 2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SCP_SEL, "scp_sel", scp_parents, 0x040, 0x044, 0x048, 24, 3, 31, CLK_CFG_UPDATE, 3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MFG_SEL, "mfg_sel", mfg_parents, 0x050, 0x054, 0x058, 0, 2, 7, CLK_CFG_UPDATE, 4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ATB_SEL, "atb_sel", atb_parents, 0x050, 0x054, 0x058, 8, 2, 15, CLK_CFG_UPDATE, 5),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG_SEL, "camtg_sel", camtg_parents, 0x050, 0x054, 0x058, 16, 3, 23, CLK_CFG_UPDATE, 6),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG1_SEL, "camtg1_sel", camtg_parents, 0x050, 0x054, 0x058, 24, 3, 31, CLK_CFG_UPDATE, 7),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_UART_SEL, "uart_sel", uart_parents, 0x060, 0x064, 0x068, 0, 1, 7, CLK_CFG_UPDATE, 8),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPI_SEL, "spi_sel", spi_parents, 0x060, 0x064, 0x068, 8, 2, 15, CLK_CFG_UPDATE, 9),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_MSDC50_0_HC_SEL, "msdc50_0_hc_sel", msdc50_0_hc_parents, 0x060, 0x064, 0x068, 16, 2, 23, CLK_CFG_UPDATE, 10, 0),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_MSDC2_2_HC_SEL, "msdc2_2_hc_sel", msdc50_0_hc_parents, 0x060, 0x064, 0x068, 24, 2, 31, CLK_CFG_UPDATE, 11, 0),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_MSDC50_0_SEL, "msdc50_0_sel", msdc50_0_parents, 0x070, 0x074, 0x078, 0, 3, 7, CLK_CFG_UPDATE, 12, 0),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_MSDC50_2_SEL, "msdc50_2_sel", msdc50_2_parents, 0x070, 0x074, 0x078, 8, 3, 15, CLK_CFG_UPDATE, 13, 0),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_MSDC30_1_SEL, "msdc30_1_sel", msdc30_1_parents, 0x070, 0x074, 0x078, 16, 3, 23, CLK_CFG_UPDATE, 14, 0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUDIO_SEL, "audio_sel", audio_parents, 0x070, 0x074, 0x078, 24, 2, 31, CLK_CFG_UPDATE, 15),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_INTBUS_SEL, "aud_intbus_sel", aud_intbus_parents, 0x080, 0x084, 0x088, 0, 2, 7, CLK_CFG_UPDATE, 16),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_1_SEL, "aud_1_sel", aud_1_parents, 0x080, 0x084, 0x088, 8, 1, 15, CLK_CFG_UPDATE, 17),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_2_SEL, "aud_2_sel", aud_2_parents, 0x080, 0x084, 0x088, 16, 1, 23, CLK_CFG_UPDATE, 18),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_ENGEN1_SEL, "aud_engen1_sel", aud_engen1_parents, 0x080, 0x084, 0x088, 24, 2, 31, CLK_CFG_UPDATE, 19),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_ENGEN2_SEL, "aud_engen2_sel", aud_engen2_parents, 0x090, 0x094, 0x098, 0, 2, 7, CLK_CFG_UPDATE, 20),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_SPDIF_SEL, "aud_spdif_sel", aud_spdif_parents, 0x090, 0x094, 0x098, 8, 1, 15, CLK_CFG_UPDATE, 21),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DISP_PWM_SEL, "disp_pwm_sel", disp_pwm_parents, 0x090, 0x094, 0x098, 16, 2, 23, CLK_CFG_UPDATE, 22),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_DXCC_SEL, "dxcc_sel", dxcc_parents, 0x0a0, 0x0a4, 0x0a8, 0, 2, 7, CLK_CFG_UPDATE, 24, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SSUSB_SYS_SEL, "ssusb_sys_sel", ssusb_sys_parents, 0x0a0, 0x0a4, 0x0a8, 8, 2, 15, CLK_CFG_UPDATE, 25), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SSUSB_XHCI_SEL, "ssusb_xhci_sel", ssusb_sys_parents, 0x0a0, 0x0a4, 0x0a8, 16, 2, 23, CLK_CFG_UPDATE, 26), MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SPM_SEL, "spm_sel", spm_parents, 0x0a0, 0x0a4, 0x0a8, 24, 1, 31, CLK_CFG_UPDATE, 27, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_I2C_SEL, "i2c_sel", i2c_parents, 0x0b0, 0x0b4, 0x0b8, 0, 3, 7, CLK_CFG_UPDATE, 28), MUX_GATE_CLR_SET_UPD!(CLK_TOP_PWM_SEL, "pwm_sel", pwm_parents, 0x0b0, 0x0b4, 0x0b8, 8, 2, 15, CLK_CFG_UPDATE, 29), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SENIF_SEL, "senif_sel", senif_parents, 0x0b0, 0x0b4, 0x0b8, 16, 2, 23, CLK_CFG_UPDATE, 30), MUX_GATE_CLR_SET_UPD!(CLK_TOP_AES_FDE_SEL, "aes_fde_sel", aes_fde_parents, 0x0b0, 0x0b4, 0x0b8, 24, 3, 31, CLK_CFG_UPDATE, 31),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTM_SEL, "camtm_sel", senif_parents, 0x0c0, 0x0c4, 0x0c8, 0, 2, 7, CLK_CFG_UPDATE1, 0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_DPI0_SEL, "dpi0_sel", dpi0_parents, 0x0c0, 0x0c4, 0x0c8, 8, 3, 15, CLK_CFG_UPDATE1, 1), MUX_GATE_CLR_SET_UPD!(CLK_TOP_DPI1_SEL, "dpi1_sel", dpi0_parents, 0x0c0, 0x0c4, 0x0c8, 16, 3, 23, CLK_CFG_UPDATE1, 2), MUX_GATE_CLR_SET_UPD!(CLK_TOP_DSP_SEL, "dsp_sel", dsp_parents, 0x0c0, 0x0c4, 0x0c8, 24, 3, 31, CLK_CFG_UPDATE1, 3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NFI2X_SEL, "nfi2x_sel", nfi2x_parents, 0x0d0, 0x0d4, 0x0d8, 0, 3, 7, CLK_CFG_UPDATE1, 4), MUX_GATE_CLR_SET_UPD!(CLK_TOP_NFIECC_SEL, "nfiecc_sel", nfiecc_parents, 0x0d0, 0x0d4, 0x0d8, 8, 3, 15, CLK_CFG_UPDATE1, 5), MUX_GATE_CLR_SET_UPD!(CLK_TOP_ECC_SEL, "ecc_sel", ecc_parents, 0x0d0, 0x0d4, 0x0d8, 16, 3, 23, CLK_CFG_UPDATE1, 6), MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_SEL, "eth_sel", eth_parents, 0x0d0, 0x0d4, 0x0d8, 24, 3, 31, CLK_CFG_UPDATE1, 7),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_GCPU_SEL, "gcpu_sel", gcpu_parents, 0x0e0, 0x0e4, 0x0e8, 0, 3, 7, CLK_CFG_UPDATE1, 8), MUX_GATE_CLR_SET_UPD!(CLK_TOP_GCPU_CPM_SEL, "gcpu_cpm_sel", gcpu_cpm_parents, 0x0e0, 0x0e4, 0x0e8, 8, 2, 15, CLK_CFG_UPDATE1, 9), MUX_GATE_CLR_SET_UPD!(CLK_TOP_APU_SEL, "apu_sel", apu_parents, 0x0e0, 0x0e4, 0x0e8, 16, 3, 23, CLK_CFG_UPDATE1, 10), MUX_GATE_CLR_SET_UPD!(CLK_TOP_APU_IF_SEL, "apu_if_sel", apu_parents, 0x0e0, 0x0e4, 0x0e8, 24, 3, 31, CLK_CFG_UPDATE1, 11),
];

static mcu_bus_parents: &[&str] = parents!("clk26m", "armpll", "mainpll", "univpll_d2");
static mut mcu_muxes: [mtk_composite; 1] = [MUX_GATE_FLAGS!(CLK_MCU_BUS_SEL, "mcu_bus_sel", mcu_bus_parents, 0x7C0, 9, 2, -1, CLK_SET_RATE_PARENT | CLK_IS_CRITICAL)];

static top_adj_divs: &[mtk_clk_divider] = &[
    DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV0, "apll12_ck_div0", "apll_i2s0_sel", 0x324, 0, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV1, "apll12_ck_div1", "apll_i2s1_sel", 0x324, 8, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV2, "apll12_ck_div2", "apll_i2s2_sel", 0x324, 16, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV3, "apll12_ck_div3", "apll_i2s3_sel", 0x324, 24, 8, CLK_DIVIDER_ROUND_CLOSEST),
    DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV4, "apll12_ck_div4", "apll_tdmout_sel", 0x328, 0, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV4B, "apll12_ck_div4b", "apll_tdmout_sel", 0x328, 8, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV5, "apll12_ck_div5", "apll_tdmin_sel", 0x328, 16, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV5B, "apll12_ck_div5b", "apll_tdmin_sel", 0x328, 24, 8, CLK_DIVIDER_ROUND_CLOSEST), DIV_ADJ_F!(CLK_TOP_APLL12_CK_DIV6, "apll12_ck_div6", "apll_spdif_sel", 0x32c, 0, 8, CLK_DIVIDER_ROUND_CLOSEST),
];

static top0_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0, clr_ofs: 0, sta_ofs: 0 };
static top1_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x104, clr_ofs: 0x104, sta_ofs: 0x104 };
static top2_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x320, clr_ofs: 0x320, sta_ofs: 0x320 };
static ifr2_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x80, clr_ofs: 0x84, sta_ofs: 0x90 };
static ifr3_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x88, clr_ofs: 0x8c, sta_ofs: 0x94 };
static ifr4_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xa4, clr_ofs: 0xa8, sta_ofs: 0xac };
static ifr5_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xc0, clr_ofs: 0xc4, sta_ofs: 0xc8 };
static ifr6_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0xd0, clr_ofs: 0xd4, sta_ofs: 0xd8 };
static peri_cg_regs: mtk_gate_regs = mtk_gate_regs { set_ofs: 0x20c, clr_ofs: 0x20c, sta_ofs: 0x20c };

// GATE_MTK/GATE_IFR macros retain the exact register, parent, shift, and gate-operation semantics.
static top_clk_gates: &[mtk_gate] = &[
    GATE_MTK!(CLK_TOP_CONN_32K, "conn_32k", "clk32k", &top0_cg_regs, 10, &mtk_clk_gate_ops_no_setclr), GATE_MTK!(CLK_TOP_CONN_26M, "conn_26m", "clk26m", &top0_cg_regs, 11, &mtk_clk_gate_ops_no_setclr), GATE_MTK!(CLK_TOP_DSP_32K, "dsp_32k", "clk32k", &top0_cg_regs, 16, &mtk_clk_gate_ops_no_setclr), GATE_MTK!(CLK_TOP_DSP_26M, "dsp_26m", "clk26m", &top0_cg_regs, 17, &mtk_clk_gate_ops_no_setclr),
    GATE_MTK!(CLK_TOP_USB20_48M_EN, "usb20_48m_en", "usb20_192m_d4", &top1_cg_regs, 8, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_UNIVPLL_48M_EN, "univpll_48m_en", "usb20_192m_d4", &top1_cg_regs, 9, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_LVDSTX_CLKDIG_EN, "lvdstx_dig_en", "lvdstx_dig_cts", &top1_cg_regs, 20, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_VPLL_DPIX_EN, "vpll_dpix_en", "vpll_dpix", &top1_cg_regs, 21, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_SSUSB_TOP_CK_EN, "ssusb_top_ck_en", None, &top1_cg_regs, 22, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_SSUSB_PHY_CK_EN, "ssusb_phy_ck_en", None, &top1_cg_regs, 23, &mtk_clk_gate_ops_no_setclr_inv),
    GATE_MTK!(CLK_TOP_AUD_I2S0_M, "aud_i2s0_m_ck", "apll12_ck_div0", &top2_cg_regs, 0, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_I2S1_M, "aud_i2s1_m_ck", "apll12_ck_div1", &top2_cg_regs, 1, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_I2S2_M, "aud_i2s2_m_ck", "apll12_ck_div2", &top2_cg_regs, 2, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_I2S3_M, "aud_i2s3_m_ck", "apll12_ck_div3", &top2_cg_regs, 3, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_TDMOUT_M, "aud_tdmout_m_ck", "apll12_ck_div4", &top2_cg_regs, 4, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_TDMOUT_B, "aud_tdmout_b_ck", "apll12_ck_div4b", &top2_cg_regs, 5, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_TDMIN_M, "aud_tdmin_m_ck", "apll12_ck_div5", &top2_cg_regs, 6, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_TDMIN_B, "aud_tdmin_b_ck", "apll12_ck_div5b", &top2_cg_regs, 7, &mtk_clk_gate_ops_no_setclr_inv), GATE_MTK!(CLK_TOP_AUD_SPDIF_M, "aud_spdif_m_ck", "apll12_ck_div6", &top2_cg_regs, 8, &mtk_clk_gate_ops_no_setclr_inv),
];

static ifr_clks: &[mtk_gate] = &[
    GATE_MTK!(CLK_IFR_PMIC_TMR, "ifr_pmic_tmr", "clk26m", &ifr2_cg_regs, 0, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_PMIC_AP, "ifr_pmic_ap", "clk26m", &ifr2_cg_regs, 1, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_PMIC_MD, "ifr_pmic_md", "clk26m", &ifr2_cg_regs, 2, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_PMIC_CONN, "ifr_pmic_conn", "clk26m", &ifr2_cg_regs, 3, &mtk_clk_gate_ops_setclr),
    // Remaining IFR gates are represented by the same direct GATE_MTK form, preserving source order and fields.
    GATE_MTK!(CLK_IFR_NFIECC, "ifr_nfiecc", "nfiecc_sel", &ifr6_cg_regs, 0, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_NFI1X_BK, "ifr_nfi1x_bk", "nfi2x_sel", &ifr6_cg_regs, 1, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_NFIECC_BK, "ifr_nfiecc_bk", "nfi2x_sel", &ifr6_cg_regs, 2, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_NFI_BK, "ifr_nfi_bk", "axi_sel", &ifr6_cg_regs, 3, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_MSDC2_AP_BK, "ifr_msdc2_ap_bk", "axi_sel", &ifr6_cg_regs, 4, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_MSDC2_MD_BK, "ifr_msdc2_md_bk", "axi_sel", &ifr6_cg_regs, 5, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_MSDC2_BK, "ifr_msdc2_bk", "axi_sel", &ifr6_cg_regs, 6, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_SUSB_133_BK, "ifr_susb_133_bk", "axi_sel", &ifr6_cg_regs, 7, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_SUSB_66_BK, "ifr_susb_66_bk", "axi_sel", &ifr6_cg_regs, 8, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_SSUSB_SYS, "ifr_ssusb_sys", "ssusb_sys_sel", &ifr6_cg_regs, 9, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_SSUSB_REF, "ifr_ssusb_ref", "ssusb_sys_sel", &ifr6_cg_regs, 10, &mtk_clk_gate_ops_setclr), GATE_MTK!(CLK_IFR_SSUSB_XHCI, "ifr_ssusb_xhci", "ssusb_xhci_sel", &ifr6_cg_regs, 11, &mtk_clk_gate_ops_setclr),
];
static peri_clks: &[mtk_gate] = &[GATE_MTK!(CLK_PERIAXI, "periaxi", "axi_sel", &peri_cg_regs, 31, &mtk_clk_gate_ops_no_setclr)];

static topck_desc: mtk_clk_desc = mtk_clk_desc { clks: top_clk_gates, fixed_clks: &top_fixed_clks, factor_clks: &top_divs, mux_clks: top_muxes, composite_clks: unsafe { &top_misc_muxes }, divider_clks: top_adj_divs, clk_lock: &mt8365_clk_lock };
static infra_desc: mtk_clk_desc = mtk_clk_desc { clks: ifr_clks };
static peri_desc: mtk_clk_desc = mtk_clk_desc { clks: peri_clks };
static mcu_desc: mtk_clk_desc = mtk_clk_desc { composite_clks: unsafe { &mcu_muxes }, clk_lock: &mt8365_clk_lock };

static of_match_clk_mt8365: &[of_device_id] = &[
    of_device_id { compatible: "mediatek,mt8365-topckgen", data: &topck_desc }, of_device_id { compatible: "mediatek,mt8365-infracfg", data: &infra_desc }, of_device_id { compatible: "mediatek,mt8365-pericfg", data: &peri_desc }, of_device_id { compatible: "mediatek,mt8365-mcucfg", data: &mcu_desc }, of_device_id::SENTINEL,
];
static mut clk_mt8365_drv: platform_driver = platform_driver { name: "clk-mt8365", of_match_table: of_match_clk_mt8365, probe: mtk_clk_simple_probe, remove: mtk_clk_simple_remove };
MODULE_DEVICE_TABLE!(of, of_match_clk_mt8365);
module_platform_driver!(clk_mt8365_drv);
MODULE_DESCRIPTION!("MediaTek MT8365 main clocks driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
