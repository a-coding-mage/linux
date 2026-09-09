// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// Linux clock-provider, platform-device, MediaTek clock, mux, and clock
// binding dependencies are supplied by the surrounding translation unit.

const CLK_CFG_0: u32 = 0x40;
const CLK_CFG_0_SET: u32 = 0x44;
const CLK_CFG_0_CLR: u32 = 0x48;
const CLK_CFG_1: u32 = 0x50;
const CLK_CFG_1_SET: u32 = 0x54;
const CLK_CFG_1_CLR: u32 = 0x58;
const CLK_CFG_2: u32 = 0x60;
const CLK_CFG_2_SET: u32 = 0x64;
const CLK_CFG_2_CLR: u32 = 0x68;
const CLK_CFG_3: u32 = 0x70;
const CLK_CFG_3_SET: u32 = 0x74;
const CLK_CFG_3_CLR: u32 = 0x78;
const CLK_CFG_4: u32 = 0x80;
const CLK_CFG_4_SET: u32 = 0x84;
const CLK_CFG_4_CLR: u32 = 0x88;
const CLK_CFG_5: u32 = 0x90;
const CLK_CFG_5_SET: u32 = 0x94;
const CLK_CFG_5_CLR: u32 = 0x98;
const CLK_CFG_6: u32 = 0xa0;
const CLK_CFG_6_SET: u32 = 0xa4;
const CLK_CFG_6_CLR: u32 = 0xa8;
const CLK_CFG_7: u32 = 0xb0;
const CLK_CFG_7_SET: u32 = 0xb4;
const CLK_CFG_7_CLR: u32 = 0xb8;

static mt6735_topckgen_lock: SpinLock = DEFINE_SPINLOCK!();

/* Some clocks with unknown details are modeled as fixed clocks. */
static topckgen_fixed_clks: [MtkFixedClk; 5] = [
    FIXED_CLK!(CLK_TOP_AD_SYS_26M_CK, "ad_sys_26m_ck", Some("clk26m"), 26 * MHZ),
    FIXED_CLK!(CLK_TOP_CLKPH_MCK_O, "clkph_mck_o", None, 0),
    FIXED_CLK!(CLK_TOP_DMPLL, "dmpll", Some("clkph_mck_o"), 0),
    FIXED_CLK!(CLK_TOP_DPI_CK, "dpi_ck", None, 0),
    FIXED_CLK!(CLK_TOP_WHPLL_AUDIO_CK, "whpll_audio_ck", None, 0),
];

static topckgen_factors: [MtkFixedFactor; 35] = [
    FACTOR!(CLK_TOP_SYSPLL_D2,"syspll_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL_D3,"syspll_d3","mainpll",1,3), FACTOR!(CLK_TOP_SYSPLL_D5,"syspll_d5","mainpll",1,5),
    FACTOR!(CLK_TOP_SYSPLL1_D2,"syspll1_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL1_D4,"syspll1_d4","mainpll",1,4), FACTOR!(CLK_TOP_SYSPLL1_D8,"syspll1_d8","mainpll",1,8), FACTOR!(CLK_TOP_SYSPLL1_D16,"syspll1_d16","mainpll",1,16),
    FACTOR!(CLK_TOP_SYSPLL2_D2,"syspll2_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL2_D4,"syspll2_d4","mainpll",1,4), FACTOR!(CLK_TOP_SYSPLL3_D2,"syspll3_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL3_D4,"syspll3_d4","mainpll",1,4), FACTOR!(CLK_TOP_SYSPLL4_D2,"syspll4_d2","mainpll",1,2), FACTOR!(CLK_TOP_SYSPLL4_D4,"syspll4_d4","mainpll",1,4),
    FACTOR!(CLK_TOP_UNIVPLL_D2,"univpll_d2","univpll",1,2), FACTOR!(CLK_TOP_UNIVPLL_D3,"univpll_d3","univpll",1,3), FACTOR!(CLK_TOP_UNIVPLL_D5,"univpll_d5","univpll",1,5), FACTOR!(CLK_TOP_UNIVPLL_D26,"univpll_d26","univpll",1,26), FACTOR!(CLK_TOP_UNIVPLL1_D2,"univpll1_d2","univpll",1,2), FACTOR!(CLK_TOP_UNIVPLL1_D4,"univpll1_d4","univpll",1,4), FACTOR!(CLK_TOP_UNIVPLL1_D8,"univpll1_d8","univpll",1,8), FACTOR!(CLK_TOP_UNIVPLL2_D2,"univpll2_d2","univpll",1,2), FACTOR!(CLK_TOP_UNIVPLL2_D4,"univpll2_d4","univpll",1,4), FACTOR!(CLK_TOP_UNIVPLL2_D8,"univpll2_d8","univpll",1,8), FACTOR!(CLK_TOP_UNIVPLL3_D2,"univpll3_d2","univpll",1,2), FACTOR!(CLK_TOP_UNIVPLL3_D4,"univpll3_d4","univpll",1,4),
    FACTOR!(CLK_TOP_MSDCPLL_D2,"msdcpll_d2","msdcpll",1,2), FACTOR!(CLK_TOP_MSDCPLL_D4,"msdcpll_d4","msdcpll",1,4), FACTOR!(CLK_TOP_MSDCPLL_D8,"msdcpll_d8","msdcpll",1,8), FACTOR!(CLK_TOP_MSDCPLL_D16,"msdcpll_d16","msdcpll",1,16), FACTOR!(CLK_TOP_VENCPLL_D3,"vencpll_d3","vencpll",1,3), FACTOR!(CLK_TOP_TVDPLL_D2,"tvdpll_d2","tvdpll",1,2), FACTOR!(CLK_TOP_TVDPLL_D4,"tvdpll_d4","tvdpll",1,4), FACTOR!(CLK_TOP_DMPLL_D2,"dmpll_d2","clkph_mck_o",1,2), FACTOR!(CLK_TOP_DMPLL_D4,"dmpll_d4","clkph_mck_o",1,4), FACTOR!(CLK_TOP_DMPLL_D8,"dmpll_d8","clkph_mck_o",1,8), FACTOR!(CLK_TOP_AD_SYS_26M_D2,"ad_sys_26m_d2","clk26m",1,2),
];

macro_rules! parents { ($($x:expr),* $(,)?) => { &[$($x),*] }; }
static axi_sel_parents: &[&str] = parents!["clk26m","syspll1_d2","syspll_d5","syspll1_d4","univpll_d5","univpll2_d2","dmpll","dmpll_d2"];
static mem_sel_parents: &[&str] = parents!["clk26m","dmpll"];
static ddrphycfg_parents: &[&str] = parents!["clk26m","syspll1_d8"];
static mm_sel_parents: &[&str] = parents!["clk26m","vencpll","syspll1_d2","syspll_d5","syspll1_d4","univpll_d5","univpll2_d2","dmpll"];
static pwm_sel_parents: &[&str] = parents!["clk26m","univpll2_d4","univpll3_d2","univpll1_d4"];
static vdec_sel_parents: &[&str] = parents!["clk26m","syspll1_d2","syspll_d5","syspll1_d4","univpll_d5","syspll_d2","syspll2_d2","msdcpll_d2"];
static mfg_sel_parents: &[&str] = parents!["clk26m","mmpll","clk26m","clk26m","clk26m","clk26m","clk26m","clk26m","clk26m","syspll_d3","syspll1_d2","syspll_d5","univpll_d3","univpll1_d2"];
static camtg_sel_parents: &[&str] = parents!["clk26m","univpll_d26","univpll2_d2","syspll3_d2","syspll3_d4","msdcpll_d4"];
static uart_sel_parents: &[&str] = parents!["clk26m","univpll2_d8"];
static spi_sel_parents: &[&str] = parents!["clk26m","syspll3_d2","msdcpll_d8","syspll2_d4","syspll4_d2","univpll2_d4","univpll1_d8"];
static usb20_sel_parents: &[&str] = parents!["clk26m","univpll1_d8","univpll3_d4"];
static msdc50_0_sel_parents: &[&str] = parents!["clk26m","syspll1_d2","syspll2_d2","syspll4_d2","univpll_d5","univpll1_d4"];
static msdc30_0_sel_parents: &[&str] = parents!["clk26m","msdcpll","msdcpll_d2","msdcpll_d4","syspll2_d2","syspll1_d4","univpll1_d4","univpll_d3","univpll_d26","syspll2_d4","univpll_d2"];
static msdc30_1_2_sel_parents: &[&str] = parents!["clk26m","univpll2_d2","msdcpll_d4","syspll2_d2","syspll1_d4","univpll1_d4","univpll_d26","syspll2_d4"];
static msdc30_3_sel_parents: &[&str] = parents!["clk26m","univpll2_d2","msdcpll_d4","syspll2_d2","syspll1_d4","univpll1_d4","univpll_d26","msdcpll_d16","syspll2_d4"];
static audio_sel_parents: &[&str] = parents!["clk26m","syspll3_d4","syspll4_d4","syspll1_d16"];
static aud_intbus_sel_parents: &[&str] = parents!["clk26m","syspll1_d4","syspll4_d2","dmpll_d4"];
static pmicspi_sel_parents: &[&str] = parents!["clk26m","syspll1_d8","syspll3_d4","syspll1_d16","univpll3_d4","univpll_d26","dmpll_d4","dmpll_d8"];
static scp_sel_parents: &[&str] = parents!["clk26m","syspll1_d8","dmpll_d2","dmpll_d4"];
static atb_sel_parents: &[&str] = parents!["clk26m","syspll1_d2","syspll_d5","dmpll"];
static dpi0_sel_parents: &[&str] = parents!["clk26m","tvdpll","tvdpll_d2","tvdpll_d4","dpi_ck"];
static scam_sel_parents: &[&str] = parents!["clk26m","syspll3_d2","univpll2_d4","vencpll_d3"];
static mfg13m_sel_parents: &[&str] = parents!["clk26m","ad_sys_26m_d2"];
static aud_1_2_sel_parents: &[&str] = parents!["clk26m","apll1"];
static irda_sel_parents: &[&str] = parents!["clk26m","univpll2_d4"];
static irtx_sel_parents: &[&str] = parents!["clk26m","ad_sys_26m_ck"];
static disppwm_sel_parents: &[&str] = parents!["clk26m","univpll2_d4","syspll4_d2_d8","ad_sys_26m_ck"];

static topckgen_muxes: [MtkMux; 28] = [
    MUX_CLR_SET_UPD!(CLK_TOP_AXI_SEL,"axi_sel",axi_sel_parents,CLK_CFG_0,CLK_CFG_0_SET,CLK_CFG_0_CLR,0,3,0,0), MUX_CLR_SET_UPD!(CLK_TOP_MEM_SEL,"mem_sel",mem_sel_parents,CLK_CFG_0,CLK_CFG_0_SET,CLK_CFG_0_CLR,8,1,0,0), MUX_CLR_SET_UPD!(CLK_TOP_DDRPHY_SEL,"ddrphycfg_sel",ddrphycfg_parents,CLK_CFG_0,CLK_CFG_0_SET,CLK_CFG_0_CLR,16,1,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MM_SEL,"mm_sel",mm_sel_parents,CLK_CFG_0,CLK_CFG_0_SET,CLK_CFG_0_CLR,24,3,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PWM_SEL,"pwm_sel",pwm_sel_parents,CLK_CFG_1,CLK_CFG_1_SET,CLK_CFG_1_CLR,0,2,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_VDEC_SEL,"vdec_sel",vdec_sel_parents,CLK_CFG_1,CLK_CFG_1_SET,CLK_CFG_1_CLR,8,3,15,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MFG_SEL,"mfg_sel",mfg_sel_parents,CLK_CFG_1,CLK_CFG_1_SET,CLK_CFG_1_CLR,16,4,23,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_CAMTG_SEL,"camtg_sel",camtg_sel_parents,CLK_CFG_1,CLK_CFG_1_SET,CLK_CFG_1_CLR,24,3,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_UART_SEL,"uart_sel",uart_sel_parents,CLK_CFG_2,CLK_CFG_2_SET,CLK_CFG_2_CLR,0,1,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPI_SEL,"spi_sel",spi_sel_parents,CLK_CFG_2,CLK_CFG_2_SET,CLK_CFG_2_CLR,8,3,15,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB20_SEL,"usb20_sel",usb20_sel_parents,CLK_CFG_2,CLK_CFG_2_SET,CLK_CFG_2_CLR,16,2,23,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC50_0_SEL,"msdc50_0_sel",msdc50_0_sel_parents,CLK_CFG_2,CLK_CFG_2_SET,CLK_CFG_2_CLR,24,3,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC30_0_SEL,"msdc30_0_sel",msdc30_0_sel_parents,CLK_CFG_3,CLK_CFG_3_SET,CLK_CFG_3_CLR,0,4,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC30_1_SEL,"msdc30_1_sel",msdc30_1_2_sel_parents,CLK_CFG_3,CLK_CFG_3_SET,CLK_CFG_3_CLR,8,3,15,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC30_2_SEL,"msdc30_2_sel",msdc30_1_2_sel_parents,CLK_CFG_3,CLK_CFG_3_SET,CLK_CFG_3_CLR,16,3,23,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MSDC30_3_SEL,"msdc30_3_sel",msdc30_3_sel_parents,CLK_CFG_3,CLK_CFG_3_SET,CLK_CFG_3_CLR,24,4,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUDIO_SEL,"audio_sel",audio_sel_parents,CLK_CFG_4,CLK_CFG_4_SET,CLK_CFG_4_CLR,0,2,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUDINTBUS_SEL,"aud_intbus_sel",aud_intbus_sel_parents,CLK_CFG_4,CLK_CFG_4_SET,CLK_CFG_4_CLR,8,2,15,0,0), MUX_CLR_SET_UPD!(CLK_TOP_PMICSPI_SEL,"pmicspi_sel",pmicspi_sel_parents,CLK_CFG_4,CLK_CFG_4_SET,CLK_CFG_4_CLR,16,3,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SCP_SEL,"scp_sel",scp_sel_parents,CLK_CFG_4,CLK_CFG_4_SET,CLK_CFG_4_CLR,24,2,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ATB_SEL,"atb_sel",atb_sel_parents,CLK_CFG_5,CLK_CFG_5_SET,CLK_CFG_5_CLR,0,2,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_DPI0_SEL,"dpi0_sel",dpi0_sel_parents,CLK_CFG_5,CLK_CFG_5_SET,CLK_CFG_5_CLR,8,3,15,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_SCAM_SEL,"scam_sel",scam_sel_parents,CLK_CFG_5,CLK_CFG_5_SET,CLK_CFG_5_CLR,16,2,23,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_MFG13M_SEL,"mfg13m_sel",mfg13m_sel_parents,CLK_CFG_5,CLK_CFG_5_SET,CLK_CFG_5_CLR,24,1,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD1_SEL,"aud_1_sel",aud_1_2_sel_parents,CLK_CFG_6,CLK_CFG_6_SET,CLK_CFG_6_CLR,0,1,7,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD2_SEL,"aud_2_sel",aud_1_2_sel_parents,CLK_CFG_6,CLK_CFG_6_SET,CLK_CFG_6_CLR,8,1,15,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_IRDA_SEL,"irda_sel",irda_sel_parents,CLK_CFG_6,CLK_CFG_6_SET,CLK_CFG_6_CLR,16,1,23,0,0), MUX_GATE_CLR_SET_UPD!(CLK_TOP_IRTX_SEL,"irtx_sel",irtx_sel_parents,CLK_CFG_6,CLK_CFG_6_SET,CLK_CFG_6_CLR,24,1,31,0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DISPPWM_SEL,"disppwm_sel",disppwm_sel_parents,CLK_CFG_7,CLK_CFG_7_SET,CLK_CFG_7_CLR,0,2,7,0,0),
];

static topckgen_desc: MtkClkDesc = MtkClkDesc { fixed_clks: &topckgen_fixed_clks, num_fixed_clks: topckgen_fixed_clks.len(), factor_clks: &topckgen_factors, num_factor_clks: topckgen_factors.len(), mux_clks: &topckgen_muxes, num_mux_clks: topckgen_muxes.len(), clk_lock: &mt6735_topckgen_lock };

static of_match_mt6735_topckgen: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "mediatek,mt6735-topckgen", data: &topckgen_desc },
    OfDeviceId::sentinel(),
];

static mut clk_mt6735_topckgen: PlatformDriver = PlatformDriver {
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
    driver: Driver { name: "clk-mt6735-topckgen", of_match_table: &of_match_mt6735_topckgen },
};

module_platform_driver!(clk_mt6735_topckgen);

module_author!("Yassine Oudjana <y.oudjana@protonmail.com>");
module_description!("MediaTek MT6735 topckgen clock driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
