// SPDX-License-Identifier: GPL-2.0
/* JZ4760 SoC CGU driver; translated from the Linux implementation. */

const MHZ: u64 = 1000 * 1000;
const CGU_REG_CPCCR: u32 = 0x00;
const CGU_REG_LCR: u32 = 0x04;
const CGU_REG_CPPCR0: u32 = 0x10;
const CGU_REG_CLKGR0: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_CLKGR1: u32 = 0x28;
const CGU_REG_CPPCR1: u32 = 0x30;
const CGU_REG_USBCDR: u32 = 0x50;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LPCDR: u32 = 0x64;
const CGU_REG_MSCCDR: u32 = 0x68;
const CGU_REG_UHCCDR: u32 = 0x6c;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x7c;
const CGU_REG_GPSCDR: u32 = 0x80;
const CGU_REG_PCMCDR: u32 = 0x84;
const CGU_REG_GPUCDR: u32 = 0x88;

static PLL_OD_ENCODING: [i8; 8] = [0, 1, -1, 2, -1, -1, -1, 3];
static JZ4760_CGU_CPCCR_DIV_TABLE: [u8; 6] = [1, 2, 3, 4, 6, 8];
static JZ4760_CGU_PLL_HALF_DIV_TABLE: [u8; 2] = [2, 1];

unsafe extern "C" {
    fn clamp_val<T: Ord>(value: T, lo: T, hi: T) -> T;
    fn ingenic_cgu_new(clocks: *const ingenic_cgu_clk_info, count: usize,
                       np: *mut device_node) -> *mut ingenic_cgu;
    fn ingenic_cgu_register_clocks(cgu: *mut ingenic_cgu) -> i32;
    fn ingenic_cgu_register_syscore(cgu: *mut ingenic_cgu);
    fn pr_err(fmt: *const u8, ...);
}

unsafe fn jz4760_cgu_calc_m_n_od(
    pll_info: *const ingenic_cgu_pll_info, mut rate: u64, mut parent_rate: u64,
    pm: *mut u32, pn: *mut u32, pod: *mut u32,
) {
    let m_max = (1u32 << (*pll_info).m_bits) - 1;
    let mut n = parent_rate / MHZ;
    n = clamp_val(n, 2, 1u64 << (*pll_info).n_bits);
    rate /= MHZ;
    parent_rate /= MHZ;
    let mut m = m_max;
    let mut od = 0u64;
    while m >= m_max && n >= 2 {
        m = (rate * n / parent_rate) as u32;
        od = (m & 1) as u64;
        m <<= od;
        n -= 1;
    }
    *pm = m;
    *pn = (n + 1) as u32;
    *pod = 1u32 << od;
}

// The clock descriptions retain the original C designated-index layout and
// depend on the shared CGU types and clock identifiers supplied by the kernel.
static JZ4760_CGU_CLOCKS: &[ingenic_cgu_clk_info] = &[
    clk!(JZ4760_CLK_EXT, "ext", CGU_CLK_EXT),
    clk!(JZ4760_CLK_OSC32K, "osc32k", CGU_CLK_EXT),
    pll!(JZ4760_CLK_PLL0, "pll0", CGU_REG_CPPCR0, JZ4760_CLK_EXT,
         23, 8, 18, 4, 16, 2, 9, 8, 10, jz4760_cgu_calc_m_n_od),
    pll!(JZ4760_CLK_PLL1, "pll1", CGU_REG_CPPCR1, JZ4760_CLK_EXT,
         23, 8, 18, 4, 16, 2, -1, 7, 6, jz4760_cgu_calc_m_n_od),
    div!(JZ4760_CLK_CCLK, "cclk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 0, 4, true),
    div!(JZ4760_CLK_HCLK, "hclk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 4, 4, false),
    div!(JZ4760_CLK_SCLK, "sclk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 24, 4, false),
    div!(JZ4760_CLK_H2CLK, "h2clk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 16, 4, false),
    div!(JZ4760_CLK_MCLK, "mclk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 12, 4, true),
    div!(JZ4760_CLK_PCLK, "pclk", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 8, 4, false),
    div!(JZ4760_CLK_PLL0_HALF, "pll0_half", JZ4760_CLK_PLL0, CGU_REG_CPCCR, 21, 1, false),
    muxdivgate!(JZ4760_CLK_UHC, "uhc", [JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_UHCCDR, 31, CGU_REG_UHCCDR, 0, 4, CGU_REG_CLKGR0, 24),
    muxdivgate!(JZ4760_CLK_GPU, "gpu", [JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_GPUCDR, 31, CGU_REG_GPUCDR, 0, 3, CGU_REG_CLKGR1, 9),
    muxdiv!(JZ4760_CLK_LPCLK_DIV, "lpclk_div", [JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_LPCDR, 29, CGU_REG_LPCDR, 0, 11),
    muxgate!(JZ4760_CLK_TVE, "tve", [JZ4760_CLK_LPCLK_DIV,JZ4760_CLK_EXT], CGU_REG_LPCDR, 31, CGU_REG_CLKGR0, 27),
    muxgate!(JZ4760_CLK_LPCLK, "lpclk", [JZ4760_CLK_LPCLK_DIV,JZ4760_CLK_TVE], CGU_REG_LPCDR, 30, CGU_REG_CLKGR0, 28),
    muxdivgate!(JZ4760_CLK_GPS, "gps", [JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_GPSCDR, 31, CGU_REG_GPSCDR, 0, 4, CGU_REG_CLKGR0, 22),
    muxdivgate!(JZ4760_CLK_PCM, "pcm", [JZ4760_CLK_EXT,-1,JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_PCMCDR, 30, CGU_REG_PCMCDR, 0, 9, CGU_REG_CLKGR1, 8),
    muxdiv!(JZ4760_CLK_I2S, "i2s", [JZ4760_CLK_EXT,-1,JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_I2SCDR, 30, CGU_REG_I2SCDR, 0, 9),
    muxdivgate!(JZ4760_CLK_OTG, "usb", [JZ4760_CLK_EXT,-1,JZ4760_CLK_PLL0_HALF,JZ4760_CLK_PLL1], CGU_REG_USBCDR, 30, CGU_REG_USBCDR, 0, 8, CGU_REG_CLKGR0, 2),
    muxdiv!(JZ4760_CLK_MMC_MUX, "mmc_mux", [JZ4760_CLK_EXT,JZ4760_CLK_PLL0_HALF], CGU_REG_MSCCDR, 31, CGU_REG_MSCCDR, 0, 6),
    muxdiv!(JZ4760_CLK_SSI_MUX, "ssi_mux", [JZ4760_CLK_EXT,JZ4760_CLK_PLL0_HALF], CGU_REG_SSICDR, 31, CGU_REG_SSICDR, 0, 6),
    divgate!(JZ4760_CLK_CIM, "cim", JZ4760_CLK_PLL0_HALF, CGU_REG_CIMCDR, 0, 8, CGU_REG_CLKGR0, 26),
    gate!(JZ4760_CLK_SSI0, "ssi0", JZ4760_CLK_SSI_MUX, CGU_REG_CLKGR0, 4), gate!(JZ4760_CLK_SSI1, "ssi1", JZ4760_CLK_SSI_MUX, CGU_REG_CLKGR0, 19), gate!(JZ4760_CLK_SSI2, "ssi2", JZ4760_CLK_SSI_MUX, CGU_REG_CLKGR0, 20),
    gate!(JZ4760_CLK_DMA, "dma", JZ4760_CLK_H2CLK, CGU_REG_CLKGR0, 21), gate!(JZ4760_CLK_MDMA, "mdma", JZ4760_CLK_HCLK, CGU_REG_CLKGR0, 25), gate!(JZ4760_CLK_BDMA, "bdma", JZ4760_CLK_HCLK, CGU_REG_CLKGR1, 0),
    gate!(JZ4760_CLK_I2C0, "i2c0", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 5), gate!(JZ4760_CLK_I2C1, "i2c1", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 6),
    gate!(JZ4760_CLK_UART0, "uart0", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 15), gate!(JZ4760_CLK_UART1, "uart1", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 16), gate!(JZ4760_CLK_UART2, "uart2", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 17), gate!(JZ4760_CLK_UART3, "uart3", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 18),
    gate!(JZ4760_CLK_IPU, "ipu", JZ4760_CLK_HCLK, CGU_REG_CLKGR0, 29), gate!(JZ4760_CLK_ADC, "adc", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 14), gate!(JZ4760_CLK_AIC, "aic", JZ4760_CLK_EXT, CGU_REG_CLKGR0, 8),
    gate!(JZ4760_CLK_VPU, "vpu", JZ4760_CLK_HCLK, CGU_REG_LCR, 30), gate!(JZ4760_CLK_MMC0, "mmc0", JZ4760_CLK_MMC_MUX, CGU_REG_CLKGR0, 3), gate!(JZ4760_CLK_MMC1, "mmc1", JZ4760_CLK_MMC_MUX, CGU_REG_CLKGR0, 11), gate!(JZ4760_CLK_MMC2, "mmc2", JZ4760_CLK_MMC_MUX, CGU_REG_CLKGR0, 12),
    gate!(JZ4760_CLK_UHC_PHY, "uhc_phy", JZ4760_CLK_UHC, CGU_REG_OPCR, 5), gate!(JZ4760_CLK_OTG_PHY, "usb_phy", JZ4760_CLK_OTG, CGU_REG_OPCR, 7),
    fixdiv!(JZ4760_CLK_EXT512, "ext/512", JZ4760_CLK_EXT, 512), mux!(JZ4760_CLK_RTC, "rtc", [JZ4760_CLK_EXT512,JZ4760_CLK_OSC32K], CGU_REG_OPCR, 2),
];

unsafe fn jz4760_cgu_init(np: *mut device_node) {
    let cgu = ingenic_cgu_new(JZ4760_CGU_CLOCKS.as_ptr(), JZ4760_CGU_CLOCKS.len(), np);
    if cgu.is_null() { return; }
    let retval = ingenic_cgu_register_clocks(cgu);
    if retval != 0 { /* pr_err("%s: failed to register CGU Clocks\n", __func__); */ }
    ingenic_cgu_register_syscore(cgu);
}

// Device-tree declarations: jz4760 and jz4760b share the implementation.
const _: unsafe extern "C" fn(*mut device_node) = jz4760_cgu_init;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
