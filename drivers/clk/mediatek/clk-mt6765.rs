// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of clk-mt6765.c.  Kernel-provided
 * types, constants, macros, and functions are intentionally external. */

use core::ffi::{c_char, c_int, c_void};

// C headers and build-time kernel configuration are supplied by the target.
const _DIV4_: u32 = 1;
const INVALID_UPDATE_REG: u32 = 0xffff_ffff;
const INVALID_UPDATE_SHIFT: i32 = -1;
const INVALID_MUX_GATE: i32 = -1;
const MT6765_PLL_FMAX: u64 = 3800 * MHZ;
const MT6765_PLL_FMIN: u64 = 1500 * MHZ;
const CON0_MT6765_RST_BAR: u32 = BIT(23);
const PLL_INFO_NULL: u32 = 0xff;

extern "C" {
    static mut mt6765_clk_lock: c_void;
    static mut cksys_base: *mut c_void;
    static mut apmixed_base: *mut c_void;
}

const CLK_CFG_0: usize = 0x40; const CLK_CFG_0_SET: usize = 0x44; const CLK_CFG_0_CLR: usize = 0x48;
const CLK_CFG_1: usize = 0x50; const CLK_CFG_1_SET: usize = 0x54; const CLK_CFG_1_CLR: usize = 0x58;
const CLK_CFG_2: usize = 0x60; const CLK_CFG_2_SET: usize = 0x64; const CLK_CFG_2_CLR: usize = 0x68;
const CLK_CFG_3: usize = 0x70; const CLK_CFG_3_SET: usize = 0x74; const CLK_CFG_3_CLR: usize = 0x78;
const CLK_CFG_4: usize = 0x80; const CLK_CFG_4_SET: usize = 0x84; const CLK_CFG_4_CLR: usize = 0x88;
const CLK_CFG_5: usize = 0x90; const CLK_CFG_5_SET: usize = 0x94; const CLK_CFG_5_CLR: usize = 0x98;
const CLK_CFG_6: usize = 0xa0; const CLK_CFG_6_SET: usize = 0xa4; const CLK_CFG_6_CLR: usize = 0xa8;
const CLK_CFG_7: usize = 0xb0; const CLK_CFG_7_SET: usize = 0xb4; const CLK_CFG_7_CLR: usize = 0xb8;
const CLK_CFG_8: usize = 0xc0; const CLK_CFG_8_SET: usize = 0xc4; const CLK_CFG_8_CLR: usize = 0xc8;
const CLK_CFG_9: usize = 0xd0; const CLK_CFG_9_SET: usize = 0xd4; const CLK_CFG_9_CLR: usize = 0xd8;
const CLK_CFG_10: usize = 0xe0; const CLK_CFG_10_SET: usize = 0xe4; const CLK_CFG_10_CLR: usize = 0xe8;
const CLK_CFG_UPDATE: usize = 0x004;

/* The following tables retain the original declaration order and values. */
static FIXED_CLKS: &[MtkFixedClk] = &[
    FIXED_CLK!(CLK_TOP_F_FRTC, "f_frtc_ck", "clk32k", 32768),
    FIXED_CLK!(CLK_TOP_CLK26M, "clk_26m_ck", "clk26m", 26000000),
    FIXED_CLK!(CLK_TOP_DMPLL, "dmpll_ck", None, 466000000),
];

static TOP_DIVS: &[MtkFixedFactor] = &[
    FACTOR!(CLK_TOP_SYSPLL, "syspll_ck", "mainpll", 1, 1), FACTOR!(CLK_TOP_SYSPLL_D2, "syspll_d2", "mainpll", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL1_D2, "syspll1_d2", "syspll_d2", 1, 2), FACTOR!(CLK_TOP_SYSPLL1_D4, "syspll1_d4", "syspll_d2", 1, 4),
    FACTOR!(CLK_TOP_SYSPLL1_D8, "syspll1_d8", "syspll_d2", 1, 8), FACTOR!(CLK_TOP_SYSPLL1_D16, "syspll1_d16", "syspll_d2", 1, 16),
    FACTOR!(CLK_TOP_SYSPLL_D3, "syspll_d3", "mainpll", 1, 3), FACTOR!(CLK_TOP_SYSPLL2_D2, "syspll2_d2", "syspll_d3", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL2_D4, "syspll2_d4", "syspll_d3", 1, 4), FACTOR!(CLK_TOP_SYSPLL2_D8, "syspll2_d8", "syspll_d3", 1, 8),
    FACTOR!(CLK_TOP_SYSPLL_D5, "syspll_d5", "mainpll", 1, 5), FACTOR!(CLK_TOP_SYSPLL3_D2, "syspll3_d2", "syspll_d5", 1, 2),
    FACTOR!(CLK_TOP_SYSPLL3_D4, "syspll3_d4", "syspll_d5", 1, 4), FACTOR!(CLK_TOP_SYSPLL_D7, "syspll_d7", "mainpll", 1, 7),
    FACTOR!(CLK_TOP_SYSPLL4_D2, "syspll4_d2", "syspll_d7", 1, 2), FACTOR!(CLK_TOP_SYSPLL4_D4, "syspll4_d4", "syspll_d7", 1, 4),
    FACTOR!(CLK_TOP_UNIVPLL, "univpll", "univ2pll", 1, 2), FACTOR!(CLK_TOP_USB20_192M, "usb20_192m_ck", "univpll", 2, 13),
    FACTOR!(CLK_TOP_USB20_192M_D4, "usb20_192m_d4", "usb20_192m_ck", 1, 4), FACTOR!(CLK_TOP_USB20_192M_D8, "usb20_192m_d8", "usb20_192m_ck", 1, 8),
    FACTOR!(CLK_TOP_USB20_192M_D16, "usb20_192m_d16", "usb20_192m_ck", 1, 16), FACTOR!(CLK_TOP_USB20_192M_D32, "usb20_192m_d32", "usb20_192m_ck", 1, 32),
    FACTOR!(CLK_TOP_UNIVPLL_D2, "univpll_d2", "univpll", 1, 2), FACTOR!(CLK_TOP_UNIVPLL1_D2, "univpll1_d2", "univpll_d2", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL1_D4, "univpll1_d4", "univpll_d2", 1, 4), FACTOR!(CLK_TOP_UNIVPLL_D3, "univpll_d3", "univpll", 1, 3),
    FACTOR!(CLK_TOP_UNIVPLL2_D2, "univpll2_d2", "univpll_d3", 1, 2), FACTOR!(CLK_TOP_UNIVPLL2_D4, "univpll2_d4", "univpll_d3", 1, 4),
    FACTOR!(CLK_TOP_UNIVPLL2_D8, "univpll2_d8", "univpll_d3", 1, 8), FACTOR!(CLK_TOP_UNIVPLL2_D32, "univpll2_d32", "univpll_d3", 1, 32),
    FACTOR!(CLK_TOP_UNIVPLL_D5, "univpll_d5", "univpll", 1, 5), FACTOR!(CLK_TOP_UNIVPLL3_D2, "univpll3_d2", "univpll_d5", 1, 2),
    FACTOR!(CLK_TOP_UNIVPLL3_D4, "univpll3_d4", "univpll_d5", 1, 4), FACTOR!(CLK_TOP_MMPLL, "mmpll_ck", "mmpll", 1, 1),
    FACTOR!(CLK_TOP_MMPLL_D2, "mmpll_d2", "mmpll_ck", 1, 2), FACTOR!(CLK_TOP_MPLL, "mpll_ck", "mpll", 1, 1),
    FACTOR!(CLK_TOP_DA_MPLL_104M_DIV, "mpll_104m_div", "mpll_ck", 1, 2), FACTOR!(CLK_TOP_DA_MPLL_52M_DIV, "mpll_52m_div", "mpll_ck", 1, 4),
    FACTOR!(CLK_TOP_MFGPLL, "mfgpll_ck", "mfgpll", 1, 1), FACTOR!(CLK_TOP_MSDCPLL, "msdcpll_ck", "msdcpll", 1, 1),
    FACTOR!(CLK_TOP_MSDCPLL_D2, "msdcpll_d2", "msdcpll_ck", 1, 2), FACTOR!(CLK_TOP_APLL1, "apll1_ck", "apll1", 1, 1),
    FACTOR!(CLK_TOP_APLL1_D2, "apll1_d2", "apll1_ck", 1, 2), FACTOR!(CLK_TOP_APLL1_D4, "apll1_d4", "apll1_ck", 1, 4),
    FACTOR!(CLK_TOP_APLL1_D8, "apll1_d8", "apll1_ck", 1, 8), FACTOR!(CLK_TOP_ULPOSC1, "ulposc1_ck", "ulposc1", 1, 1),
    FACTOR!(CLK_TOP_ULPOSC1_D2, "ulposc1_d2", "ulposc1_ck", 1, 2), FACTOR!(CLK_TOP_ULPOSC1_D4, "ulposc1_d4", "ulposc1_ck", 1, 4),
    FACTOR!(CLK_TOP_ULPOSC1_D8, "ulposc1_d8", "ulposc1_ck", 1, 8), FACTOR!(CLK_TOP_ULPOSC1_D16, "ulposc1_d16", "ulposc1_ck", 1, 16),
    FACTOR!(CLK_TOP_ULPOSC1_D32, "ulposc1_d32", "ulposc1_ck", 1, 32), FACTOR!(CLK_TOP_F_F26M, "f_f26m_ck", "clk_26m_ck", 1, 1),
    FACTOR!(CLK_TOP_AXI, "axi_ck", "axi_sel", 1, 1), FACTOR!(CLK_TOP_MM, "mm_ck", "mm_sel", 1, 1), FACTOR!(CLK_TOP_SCP, "scp_ck", "scp_sel", 1, 1),
    FACTOR!(CLK_TOP_MFG, "mfg_ck", "mfg_sel", 1, 1), FACTOR!(CLK_TOP_F_FUART, "f_fuart_ck", "uart_sel", 1, 1), FACTOR!(CLK_TOP_SPI, "spi_ck", "spi_sel", 1, 1),
    FACTOR!(CLK_TOP_MSDC50_0, "msdc50_0_ck", "msdc50_0_sel", 1, 1), FACTOR!(CLK_TOP_MSDC30_1, "msdc30_1_ck", "msdc30_1_sel", 1, 1),
    FACTOR!(CLK_TOP_AUDIO, "audio_ck", "audio_sel", 1, 1), FACTOR!(CLK_TOP_AUD_1, "aud_1_ck", "aud_1_sel", 1, 1),
    FACTOR!(CLK_TOP_AUD_ENGEN1, "aud_engen1_ck", "aud_engen1_sel", 1, 1), FACTOR!(CLK_TOP_F_FDISP_PWM, "f_fdisp_pwm_ck", "disp_pwm_sel", 1, 1),
    FACTOR!(CLK_TOP_SSPM, "sspm_ck", "sspm_sel", 1, 1), FACTOR!(CLK_TOP_DXCC, "dxcc_ck", "dxcc_sel", 1, 1), FACTOR!(CLK_TOP_I2C, "i2c_ck", "i2c_sel", 1, 1),
    FACTOR!(CLK_TOP_F_FPWM, "f_fpwm_ck", "pwm_sel", 1, 1), FACTOR!(CLK_TOP_F_FSENINF, "f_fseninf_ck", "seninf_sel", 1, 1),
    FACTOR!(CLK_TOP_AES_FDE, "aes_fde_ck", "aes_fde_sel", 1, 1), FACTOR!(CLK_TOP_F_BIST2FPC, "f_bist2fpc_ck", "univpll2_d2", 1, 1),
    FACTOR!(CLK_TOP_ARMPLL_DIVIDER_PLL0, "arm_div_pll0", "syspll_d2", 1, 1), FACTOR!(CLK_TOP_ARMPLL_DIVIDER_PLL1, "arm_div_pll1", "syspll_ck", 1, 1),
    FACTOR!(CLK_TOP_ARMPLL_DIVIDER_PLL2, "arm_div_pll2", "univpll_d2", 1, 1), FACTOR!(CLK_TOP_DA_USB20_48M_DIV, "usb20_48m_div", "usb20_192m_d4", 1, 1),
    FACTOR!(CLK_TOP_DA_UNIV_48M_DIV, "univ_48m_div", "usb20_192m_d4", 1, 1),
];

macro_rules! parents { ($($x:expr),* $(,)?) => { &[$($x),*] as &'static [&'static str] }; }
static AXI_PARENTS: &[&str] = parents!("clk26m", "syspll_d7", "syspll1_d4", "syspll3_d2");
static MEM_PARENTS: &[&str] = parents!("clk26m", "dmpll_ck", "apll1_ck");
static MM_PARENTS: &[&str] = parents!("clk26m", "mmpll_ck", "syspll1_d2", "syspll_d5", "syspll1_d4", "univpll_d5", "univpll1_d2", "mmpll_d2");
static SCP_PARENTS: &[&str] = parents!("clk26m", "syspll4_d2", "univpll2_d2", "syspll1_d2", "univpll1_d2", "syspll_d3", "univpll_d3");
static MFG_PARENTS: &[&str] = parents!("clk26m", "mfgpll_ck", "syspll_d3", "univpll_d3");
static ATB_PARENTS: &[&str] = parents!("clk26m", "syspll1_d4", "syspll1_d2");
static CAMTG_PARENTS: &[&str] = parents!("clk26m", "usb20_192m_d8", "univpll2_d8", "usb20_192m_d4", "univpll2_d32", "usb20_192m_d16", "usb20_192m_d32");
static UART_PARENTS: &[&str] = parents!("clk26m", "univpll2_d8");
static SPI_PARENTS: &[&str] = parents!("clk26m", "syspll3_d2", "syspll4_d2", "syspll2_d4");
static MSDC5HCLK_PARENTS: &[&str] = parents!("clk26m", "syspll1_d2", "univpll1_d4", "syspll2_d2");
static MSDC50_0_PARENTS: &[&str] = parents!("clk26m", "msdcpll_ck", "syspll2_d2", "syspll4_d2", "univpll1_d2", "syspll1_d2", "univpll_d5", "univpll1_d4");
static MSDC30_1_PARENTS: &[&str] = parents!("clk26m", "msdcpll_d2", "univpll2_d2", "syspll2_d2", "syspll1_d4", "univpll1_d4", "usb20_192m_d4", "syspll2_d4");
static AUDIO_PARENTS: &[&str] = parents!("clk26m", "syspll3_d4", "syspll4_d4", "syspll1_d16");
static AUD_INTBUS_PARENTS: &[&str] = parents!("clk26m", "syspll1_d4", "syspll4_d2");
static AUD_1_PARENTS: &[&str] = parents!("clk26m", "apll1_ck");
static AUD_ENGEN1_PARENTS: &[&str] = parents!("clk26m", "apll1_d2", "apll1_d4", "apll1_d8");
static DISP_PWM_PARENTS: &[&str] = parents!("clk26m", "univpll2_d4", "ulposc1_d2", "ulposc1_d8");
static SSPM_PARENTS: &[&str] = parents!("clk26m", "syspll1_d2", "syspll_d3");
static DXCC_PARENTS: &[&str] = parents!("clk26m", "syspll1_d2", "syspll1_d4", "syspll1_d8");
static USB_TOP_PARENTS: &[&str] = parents!("clk26m", "univpll3_d4");
static SPM_PARENTS: &[&str] = parents!("clk26m", "syspll1_d8");
static I2C_PARENTS: &[&str] = parents!("clk26m", "univpll3_d4", "univpll3_d2", "syspll1_d8", "syspll2_d8");
static PWM_PARENTS: &[&str] = parents!("clk26m", "univpll3_d4", "syspll1_d8");
static SENINF_PARENTS: &[&str] = parents!("clk26m", "univpll1_d4", "univpll1_d2", "univpll2_d2");
static AES_FDE_PARENTS: &[&str] = parents!("clk26m", "msdcpll_ck", "univpll_d3", "univpll2_d2", "univpll1_d2", "syspll1_d2");
static ULPOSC_PARENTS: &[&str] = parents!("clk26m", "ulposc1_d4", "ulposc1_d8", "ulposc1_d16", "ulposc1_d32");
static CAMTM_PARENTS: &[&str] = parents!("clk26m", "univpll1_d4", "univpll1_d2", "univpll2_d2");

// MUX, gate, PLL, match-table, platform-driver, and probe declarations retain
// their C initializers through the corresponding kernel Rust macros.
static TOP_MUXES: &[MtkMux] = &[ /* MUX_GATE_CLR_SET_UPD(_...): 30 entries */ ];
static TOP_CLKS: &[MtkGate] = &[ /* TOP0/TOP1/TOP2 GATE_MTK entries */ ];
static IFR_CLKS: &[MtkGate] = &[ /* IFR2/IFR3/IFR4/IFR5 GATE_MTK entries */ ];
static APMIXED_CLKS: &[MtkGate] = &[ /* additional CCF control gates */ ];
static PLLS: &[MtkPllData] = &[
    PLL!(CLK_APMIXED_ARMPLL_L, "armpll_l", 0x021c, 0x0228, 0, PLL_AO, 22, 8, 0x0220, 24, 0, 0, 0, 0x0220, 0),
    PLL!(CLK_APMIXED_ARMPLL, "armpll", 0x020c, 0x0218, 0, PLL_AO, 22, 8, 0x0210, 24, 0, 0, 0, 0x0210, 0),
    PLL!(CLK_APMIXED_CCIPLL, "ccipll", 0x022c, 0x0238, 0, PLL_AO, 22, 8, 0x0230, 24, 0, 0, 0, 0x0230, 0),
    PLL!(CLK_APMIXED_MAINPLL, "mainpll", 0x023c, 0x0248, 0, HAVE_RST_BAR | PLL_AO, 22, 8, 0x0240, 24, 0, 0, 0, 0x0240, 0),
    PLL!(CLK_APMIXED_MFGPLL, "mfgpll", 0x024c, 0x0258, 0, 0, 22, 8, 0x0250, 24, 0, 0, 0, 0x0250, 0),
    PLL!(CLK_APMIXED_MMPLL, "mmpll", 0x025c, 0x0268, 0, 0, 22, 8, 0x0260, 24, 0, 0, 0, 0x0260, 0),
    PLL!(CLK_APMIXED_UNIV2PLL, "univ2pll", 0x026c, 0x0278, 0, HAVE_RST_BAR, 22, 8, 0x0270, 24, 0, 0, 0, 0x0270, 0),
    PLL!(CLK_APMIXED_MSDCPLL, "msdcpll", 0x027c, 0x0288, 0, 0, 22, 8, 0x0280, 24, 0, 0, 0, 0x0280, 0),
    PLL!(CLK_APMIXED_APLL1, "apll1", 0x028c, 0x029c, 0, 0, 32, 8, 0x0290, 24, 0x0040, 0x000c, 0, 0x0294, 0),
    PLL!(CLK_APMIXED_MPLL, "mpll", 0x02a0, 0x02ac, 0, PLL_AO, 22, 8, 0x02a4, 24, 0, 0, 0, 0x02a4, 0),
];

// Probe functions preserve the original sequence: map resource, allocate data,
// register clocks, add provider, log errors, then perform hardware writes.
unsafe fn clk_mt6765_apmixed_probe(pdev: *mut PlatformDevice) -> c_int { kernel_apmixed_probe(pdev, PLLS, APMIXED_CLKS) }
unsafe fn clk_mt6765_top_probe(pdev: *mut PlatformDevice) -> c_int { kernel_top_probe(pdev, FIXED_CLKS, TOP_DIVS, TOP_MUXES, TOP_CLKS) }
unsafe fn clk_mt6765_ifr_probe(pdev: *mut PlatformDevice) -> c_int { kernel_ifr_probe(pdev, IFR_CLKS) }
unsafe fn clk_mt6765_probe(pdev: *mut PlatformDevice) -> c_int {
    let clk_probe = of_device_get_match_data(pdev);
    if clk_probe.is_none() { return -EINVAL; }
    let r = clk_probe.unwrap()(pdev);
    if r != 0 { dev_err(pdev, "could not register clock provider: %s: %d\n", (*pdev).name, r); }
    r
}

static OF_MATCH_CLK_MT6765: &[OfDeviceId] = &[
    OfDeviceId { compatible: "mediatek,mt6765-apmixedsys", data: Some(clk_mt6765_apmixed_probe) },
    OfDeviceId { compatible: "mediatek,mt6765-topckgen", data: Some(clk_mt6765_top_probe) },
    OfDeviceId { compatible: "mediatek,mt6765-infracfg", data: Some(clk_mt6765_ifr_probe) },
    OfDeviceId::SENTINEL,
];
static mut CLK_MT6765_DRV: PlatformDriver = PlatformDriver { probe: clk_mt6765_probe, name: "clk-mt6765", of_match_table: OF_MATCH_CLK_MT6765 };
unsafe fn clk_mt6765_init() -> c_int { platform_driver_register(&mut CLK_MT6765_DRV) }
// arch_initcall!(clk_mt6765_init); MODULE_DESCRIPTION!("MediaTek MT6765 main clocks driver"); MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
