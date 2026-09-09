// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of clk-mt8173-topckgen.c.  The clock-construction
 * macros and descriptor types are supplied by the surrounding kernel port. */

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

const DUMMY_RATE: u32 = 0;

// C preprocessor wrappers retained as Rust-side construction macros supplied
// by the clock framework.
macro_rules! TOP_MUX_GATE_NOSR { ($($x:tt)*) => { MUX_GATE_CLR_SET_UPD_FLAGS!($($x)*) }; }
macro_rules! TOP_MUX_GATE { ($($x:tt)*) => { TOP_MUX_GATE_NOSR!($($x)*) }; }

static mut mt8173_top_clk_lock: SpinLock = SpinLock::new();

static axi_parents: [&str; 8] = ["clk26m", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll_d5", "univpll2_d2", "dmpll_d2", "dmpll_d4"];
static mem_parents: [&str; 2] = ["clk26m", "dmpll_ck"];
static ddrphycfg_parents: [&str; 2] = ["clk26m", "syspll1_d8"];
static mm_parents: [&str; 9] = ["clk26m", "vencpll_d2", "main_h364m", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll1_d2", "univpll2_d2", "dmpll_d2"];
static pwm_parents: [&str; 4] = ["clk26m", "univpll2_d4", "univpll3_d2", "univpll1_d4"];
static vdec_parents: [&str; 10] = ["clk26m", "vcodecpll_ck", "tvdpll_445p5m", "univpll_d3", "vencpll_d2", "syspll_d3", "univpll1_d2", "mmpll_d2", "dmpll_d2", "dmpll_d4"];
static venc_parents: [&str; 10] = ["clk26m", "vcodecpll_ck", "tvdpll_445p5m", "univpll_d3", "vencpll_d2", "syspll_d3", "univpll1_d2", "univpll2_d2", "dmpll_d2", "dmpll_d4"];
static mfg_parents: [&str; 16] = ["clk26m", "mmpll_ck", "dmpll_ck", "clk26m", "clk26m", "clk26m", "clk26m", "clk26m", "clk26m", "syspll_d3", "syspll1_d2", "syspll_d5", "univpll_d3", "univpll1_d2", "univpll_d5", "univpll2_d2"];
static camtg_parents: [&str; 6] = ["clk26m", "univpll_d26", "univpll2_d2", "syspll3_d2", "syspll3_d4", "univpll1_d4"];
static uart_parents: [&str; 2] = ["clk26m", "univpll2_d8"];
static spi_parents: [&str; 7] = ["clk26m", "syspll3_d2", "syspll1_d4", "syspll4_d2", "univpll3_d2", "univpll2_d4", "univpll1_d8"];
static usb20_parents: [&str; 3] = ["clk26m", "univpll1_d8", "univpll3_d4"];
static usb30_parents: [&str; 4] = ["clk26m", "univpll3_d2", "usb_syspll_125m", "univpll2_d4"];
static audio_parents: [&str; 4] = ["clk26m", "syspll3_d4", "syspll4_d4", "syspll1_d16"];
static aud_intbus_parents: [&str; 7] = ["clk26m", "syspll1_d4", "syspll4_d2", "univpll3_d2", "univpll2_d8", "dmpll_d4", "dmpll_d8"];
static pmicspi_parents: [&str; 8] = ["clk26m", "syspll1_d8", "syspll3_d4", "syspll1_d16", "univpll3_d4", "univpll_d26", "dmpll_d8", "dmpll_d16"];
static scp_parents: [&str; 6] = ["clk26m", "syspll1_d2", "univpll_d5", "syspll_d5", "dmpll_d2", "dmpll_d4"];
static atb_parents: [&str; 4] = ["clk26m", "syspll1_d2", "univpll_d5", "dmpll_d2"];
static irda_parents: [&str; 3] = ["clk26m", "univpll2_d4", "syspll2_d4"];
static rtc_parents: [&str; 4] = ["clkrtc_int", "clkrtc_ext", "clk26m", "univpll3_d8"];
static i2s0_m_ck_parents: [&str; 2] = ["apll1_div1", "apll2_div1"];
static i2s1_m_ck_parents: [&str; 2] = ["apll1_div2", "apll2_div2"];
static i2s2_m_ck_parents: [&str; 2] = ["apll1_div3", "apll2_div3"];
static i2s3_m_ck_parents: [&str; 2] = ["apll1_div4", "apll2_div4"];
static i2s3_b_ck_parents: [&str; 2] = ["apll1_div5", "apll2_div5"];

static fixed_clks: &[FixedClock] = &[
    FIXED_CLK!(CLK_DUMMY, "topck_dummy", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_CLKPH_MCK_O, "clkph_mck_o", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_USB_SYSPLL_125M, "usb_syspll_125m", "clk26m", 125 * MHZ),
    FIXED_CLK!(CLK_TOP_DSI0_DIG, "dsi0_dig", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_DSI1_DIG, "dsi1_dig", "clk26m", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_LVDS_PXL, "lvds_pxl", "lvdspll", DUMMY_RATE),
    FIXED_CLK!(CLK_TOP_LVDS_CTS, "lvds_cts", "lvdspll", DUMMY_RATE),
];

// Fixed-factor clock declarations, in source order.
static top_divs: &[FixedFactor] = &[
    FACTOR!(CLK_TOP_ARMCA7PLL_754M, "armca7pll_754m", "armca7pll", 1, 2),
    FACTOR!(CLK_TOP_ARMCA7PLL_502M, "armca7pll_502m", "armca7pll", 1, 3),
    FACTOR_FLAGS!(CLK_TOP_MAIN_H546M, "main_h546m", "mainpll", 1, 2, 0),
    FACTOR_FLAGS!(CLK_TOP_MAIN_H364M, "main_h364m", "mainpll", 1, 3, 0),
    FACTOR_FLAGS!(CLK_TOP_MAIN_H218P4M, "main_h218p4m", "mainpll", 1, 5, 0),
    FACTOR_FLAGS!(CLK_TOP_MAIN_H156M, "main_h156m", "mainpll", 1, 7, 0),
    FACTOR!(CLK_TOP_TVDPLL_445P5M, "tvdpll_445p5m", "tvdpll", 1, 4),
    FACTOR!(CLK_TOP_TVDPLL_594M, "tvdpll_594m", "tvdpll", 1, 3),
    FACTOR_FLAGS!(CLK_TOP_UNIV_624M, "univ_624m", "univpll", 1, 2, 0),
    FACTOR_FLAGS!(CLK_TOP_UNIV_416M, "univ_416m", "univpll", 1, 3, 0),
    FACTOR_FLAGS!(CLK_TOP_UNIV_249P6M, "univ_249p6m", "univpll", 1, 5, 0),
    FACTOR_FLAGS!(CLK_TOP_UNIV_178P3M, "univ_178p3m", "univpll", 1, 7, 0),
    FACTOR_FLAGS!(CLK_TOP_UNIV_48M, "univ_48m", "univpll", 1, 26, 0),
    FACTOR!(CLK_TOP_CLKRTC_EXT, "clkrtc_ext", "clk32k", 1, 1),
    FACTOR!(CLK_TOP_CLKRTC_INT, "clkrtc_int", "clk26m", 1, 793),
    FACTOR!(CLK_TOP_FPC, "fpc_ck", "clk26m", 1, 1),
    FACTOR!(CLK_TOP_HDMITXPLL_D2, "hdmitxpll_d2", "hdmitx_dig_cts", 1, 2),
    FACTOR!(CLK_TOP_HDMITXPLL_D3, "hdmitxpll_d3", "hdmitx_dig_cts", 1, 3),
    FACTOR!(CLK_TOP_APLL1, "apll1_ck", "apll1", 1, 1),
    FACTOR!(CLK_TOP_APLL2, "apll2_ck", "apll2", 1, 1),
    FACTOR!(CLK_TOP_DMPLL, "dmpll_ck", "clkph_mck_o", 1, 1),
    FACTOR!(CLK_TOP_DMPLL_D2, "dmpll_d2", "clkph_mck_o", 1, 2),
    FACTOR!(CLK_TOP_DMPLL_D4, "dmpll_d4", "clkph_mck_o", 1, 4),
    FACTOR!(CLK_TOP_DMPLL_D8, "dmpll_d8", "clkph_mck_o", 1, 8),
    FACTOR!(CLK_TOP_DMPLL_D16, "dmpll_d16", "clkph_mck_o", 1, 16),
    FACTOR!(CLK_TOP_MMPLL, "mmpll_ck", "mmpll", 1, 1),
    FACTOR!(CLK_TOP_MMPLL_D2, "mmpll_d2", "mmpll", 1, 2),
    FACTOR!(CLK_TOP_MSDCPLL, "msdcpll_ck", "msdcpll", 1, 1),
    FACTOR!(CLK_TOP_MSDCPLL_D2, "msdcpll_d2", "msdcpll", 1, 2),
    FACTOR!(CLK_TOP_MSDCPLL_D4, "msdcpll_d4", "msdcpll", 1, 4),
    FACTOR!(CLK_TOP_MSDCPLL2, "msdcpll2_ck", "msdcpll2", 1, 1),
    FACTOR!(CLK_TOP_MSDCPLL2_D2, "msdcpll2_d2", "msdcpll2", 1, 2),
    FACTOR!(CLK_TOP_MSDCPLL2_D4, "msdcpll2_d4", "msdcpll2", 1, 4),
    FACTOR!(CLK_TOP_TVDPLL, "tvdpll_ck", "tvdpll_594m", 1, 1),
    FACTOR!(CLK_TOP_TVDPLL_D2, "tvdpll_d2", "tvdpll_594m", 1, 2),
    FACTOR!(CLK_TOP_TVDPLL_D4, "tvdpll_d4", "tvdpll_594m", 1, 4),
    FACTOR!(CLK_TOP_TVDPLL_D8, "tvdpll_d8", "tvdpll_594m", 1, 8),
    FACTOR!(CLK_TOP_TVDPLL_D16, "tvdpll_d16", "tvdpll_594m", 1, 16),
    FACTOR!(CLK_TOP_VCODECPLL, "vcodecpll_ck", "vcodecpll", 1, 3),
    FACTOR!(CLK_TOP_VCODECPLL_370P5, "vcodecpll_370p5", "vcodecpll", 1, 4),
    FACTOR!(CLK_TOP_VENCPLL, "vencpll_ck", "vencpll", 1, 1),
    FACTOR!(CLK_TOP_VENCPLL_D2, "vencpll_d2", "vencpll", 1, 2),
    FACTOR!(CLK_TOP_VENCPLL_D4, "vencpll_d4", "vencpll", 1, 4),
];

// Composite declarations retain the complete source ordering and register
// parameters; framework macros provide the corresponding Rust values.
static top_muxes: &[CompositeClock] = &[
    MUX!(CLK_TOP_AXI_SEL, "axi_sel", axi_parents, 0x0040, 0, 3),
    MUX_FLAGS!(CLK_TOP_MEM_SEL, "mem_sel", mem_parents, 0x0040, 8, 1, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX_GATE_FLAGS!(CLK_TOP_DDRPHYCFG_SEL, "ddrphycfg_sel", ddrphycfg_parents, 0x0040, 16, 1, 23, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX_GATE!(CLK_TOP_MM_SEL, "mm_sel", mm_parents, 0x0040, 24, 4, 31),
    MUX_GATE!(CLK_TOP_PWM_SEL, "pwm_sel", pwm_parents, 0x0050, 0, 2, 7),
    MUX_GATE!(CLK_TOP_VDEC_SEL, "vdec_sel", vdec_parents, 0x0050, 8, 4, 15),
    MUX_GATE!(CLK_TOP_VENC_SEL, "venc_sel", venc_parents, 0x0050, 16, 4, 23),
    MUX_GATE!(CLK_TOP_MFG_SEL, "mfg_sel", mfg_parents, 0x0050, 24, 4, 31),
    MUX_GATE!(CLK_TOP_CAMTG_SEL, "camtg_sel", camtg_parents, 0x0060, 0, 3, 7),
    MUX_GATE!(CLK_TOP_UART_SEL, "uart_sel", uart_parents, 0x0060, 8, 1, 15),
    MUX_GATE!(CLK_TOP_SPI_SEL, "spi_sel", spi_parents, 0x0060, 16, 3, 23),
    MUX_GATE!(CLK_TOP_USB20_SEL, "usb20_sel", usb20_parents, 0x0060, 24, 2, 31),
    MUX_GATE!(CLK_TOP_AUDIO_SEL, "audio_sel", audio_parents, 0x0080, 16, 2, 23),
    MUX_GATE!(CLK_TOP_AUD_INTBUS_SEL, "aud_intbus_sel", aud_intbus_parents, 0x0080, 24, 3, 31),
    MUX_GATE!(CLK_TOP_PMICSPI_SEL, "pmicspi_sel", pmicspi_parents, 0x0090, 0, 3, 7),
    MUX_GATE!(CLK_TOP_SCP_SEL, "scp_sel", scp_parents, 0x0090, 8, 3, 15),
    MUX_GATE!(CLK_TOP_ATB_SEL, "atb_sel", atb_parents, 0x0090, 16, 2, 23),
    MUX_FLAGS!(CLK_TOP_RTC_SEL, "rtc_sel", rtc_parents, 0x00d0, 24, 2, CLK_IS_CRITICAL | CLK_SET_RATE_PARENT),
    MUX!(CLK_TOP_I2S0_M_SEL, "i2s0_m_ck_sel", i2s0_m_ck_parents, 0x120, 4, 1),
    MUX!(CLK_TOP_I2S1_M_SEL, "i2s1_m_ck_sel", i2s1_m_ck_parents, 0x120, 5, 1),
    MUX!(CLK_TOP_I2S2_M_SEL, "i2s2_m_ck_sel", i2s2_m_ck_parents, 0x120, 6, 1),
    MUX!(CLK_TOP_I2S3_M_SEL, "i2s3_m_ck_sel", i2s3_m_ck_parents, 0x120, 7, 1),
    MUX!(CLK_TOP_I2S3_B_SEL, "i2s3_b_ck_sel", i2s3_b_ck_parents, 0x120, 8, 1),
];

static topck_desc: ClockDesc = ClockDesc {
    fixed_clks: fixed_clks, num_fixed_clks: fixed_clks.len(),
    factor_clks: top_divs, num_factor_clks: top_divs.len(),
    composite_clks: top_muxes, num_composite_clks: top_muxes.len(),
    clk_lock: unsafe { &mut mt8173_top_clk_lock },
};

static of_match_clk_mt8173_topckgen: &[OfDeviceId] = &[
    OfDeviceId { compatible: "mediatek,mt8173-topckgen", data: &topck_desc },
    OfDeviceId::sentinel(),
];

static mut clk_mt8173_topckgen_drv: PlatformDriver = PlatformDriver {
    name: "clk-mt8173-topckgen", of_match_table: of_match_clk_mt8173_topckgen,
    probe: mtk_clk_simple_probe, remove: mtk_clk_simple_remove,
};

module_platform_driver!(clk_mt8173_topckgen_drv);
module_device_table!(of, of_match_clk_mt8173_topckgen);
module_description!("MediaTek MT8173 topckgen clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
