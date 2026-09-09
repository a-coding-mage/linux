// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Tarek Dakhran <t.dakhran@samsung.com>
 *
 * Common Clock Framework support for Exynos5410 SoC.
 */

// External Linux clock-provider, device-tree, and local clock definitions are
// supplied by the surrounding translation unit.

const APLL_LOCK: u32 = 0x0;
const APLL_CON0: u32 = 0x100;
const CPLL_LOCK: u32 = 0x10020;
const CPLL_CON0: u32 = 0x10120;
const EPLL_LOCK: u32 = 0x10040;
const EPLL_CON0: u32 = 0x10130;
const MPLL_LOCK: u32 = 0x4000;
const MPLL_CON0: u32 = 0x4100;
const BPLL_LOCK: u32 = 0x20010;
const BPLL_CON0: u32 = 0x20110;
const KPLL_LOCK: u32 = 0x28000;
const KPLL_CON0: u32 = 0x28100;
const SRC_CPU: u32 = 0x200;
const DIV_CPU0: u32 = 0x500;
const SRC_CPERI1: u32 = 0x4204;
const GATE_IP_G2D: u32 = 0x8800;
const DIV_TOP0: u32 = 0x10510;
const DIV_TOP1: u32 = 0x10514;
const DIV_FSYS0: u32 = 0x10548;
const DIV_FSYS1: u32 = 0x1054c;
const DIV_FSYS2: u32 = 0x10550;
const DIV_PERIC0: u32 = 0x10558;
const DIV_PERIC3: u32 = 0x10564;
const SRC_TOP0: u32 = 0x10210;
const SRC_TOP1: u32 = 0x10214;
const SRC_TOP2: u32 = 0x10218;
const SRC_FSYS: u32 = 0x10244;
const SRC_PERIC0: u32 = 0x10250;
const SRC_MASK_FSYS: u32 = 0x10340;
const SRC_MASK_PERIC0: u32 = 0x10350;
const GATE_BUS_FSYS0: u32 = 0x10740;
const GATE_TOP_SCLK_FSYS: u32 = 0x10840;
const GATE_TOP_SCLK_PERIC: u32 = 0x10850;
const GATE_IP_FSYS: u32 = 0x10944;
const GATE_IP_PERIC: u32 = 0x10950;
const GATE_IP_PERIS: u32 = 0x10960;
const SRC_CDREX: u32 = 0x20200;
const SRC_KFC: u32 = 0x28200;
const DIV_KFC0: u32 = 0x28500;
const CLKS_NR: usize = 512;

enum Exynos5410Plls { Apll, Cpll, Epll, Mpll, Bpll, Kpll, NrPlls }

static APLL_P: &[&str] = &["fin_pll", "fout_apll"];
static BPLL_P: &[&str] = &["fin_pll", "fout_bpll"];
static CPLL_P: &[&str] = &["fin_pll", "fout_cpll"];
static EPLL_P: &[&str] = &["fin_pll", "fout_epll"];
static MPLL_P: &[&str] = &["fin_pll", "fout_mpll"];
static KPLL_P: &[&str] = &["fin_pll", "fout_kpll"];
static MOUT_CPU_P: &[&str] = &["mout_apll", "sclk_mpll"];
static MOUT_KFC_P: &[&str] = &["mout_kpll", "sclk_mpll"];
static MPLL_USER_P: &[&str] = &["fin_pll", "sclk_mpll"];
static BPLL_USER_P: &[&str] = &["fin_pll", "sclk_bpll"];
static MPLL_BPLL_P: &[&str] = &["sclk_mpll_muxed", "sclk_bpll_muxed"];
static SCLK_MPLL_BPLL_P: &[&str] = &["sclk_mpll_bpll", "fin_pll"];
static GROUP2_P: &[&str] = &["fin_pll", "fin_pll", "none", "none", "none", "none", "sclk_mpll_bpll", "none", "none", "sclk_cpll"];

static EXYNOS5410_MUX_CLKS: &[SamsungMuxClock] = &[
    MUX!(0, "mout_apll", APLL_P, SRC_CPU, 0, 1), MUX!(0, "mout_cpu", MOUT_CPU_P, SRC_CPU, 16, 1),
    MUX!(0, "mout_kpll", KPLL_P, SRC_KFC, 0, 1), MUX!(0, "mout_kfc", MOUT_KFC_P, SRC_KFC, 16, 1),
    MUX!(0, "sclk_mpll", MPLL_P, SRC_CPERI1, 8, 1), MUX!(0, "sclk_mpll_muxed", MPLL_USER_P, SRC_TOP2, 20, 1),
    MUX!(0, "sclk_bpll", BPLL_P, SRC_CDREX, 0, 1), MUX!(0, "sclk_bpll_muxed", BPLL_USER_P, SRC_TOP2, 24, 1),
    MUX!(0, "sclk_epll", EPLL_P, SRC_TOP2, 12, 1), MUX!(0, "sclk_cpll", CPLL_P, SRC_TOP2, 8, 1),
    MUX!(0, "sclk_mpll_bpll", MPLL_BPLL_P, SRC_TOP1, 20, 1),
    MUX!(0, "mout_mmc0", GROUP2_P, SRC_FSYS, 0, 4), MUX!(0, "mout_mmc1", GROUP2_P, SRC_FSYS, 4, 4), MUX!(0, "mout_mmc2", GROUP2_P, SRC_FSYS, 8, 4),
    MUX!(0, "mout_usbd300", SCLK_MPLL_BPLL_P, SRC_FSYS, 28, 1), MUX!(0, "mout_usbd301", SCLK_MPLL_BPLL_P, SRC_FSYS, 29, 1),
    MUX!(0, "mout_uart0", GROUP2_P, SRC_PERIC0, 0, 4), MUX!(0, "mout_uart1", GROUP2_P, SRC_PERIC0, 4, 4), MUX!(0, "mout_uart2", GROUP2_P, SRC_PERIC0, 8, 4), MUX!(0, "mout_uart3", GROUP2_P, SRC_PERIC0, 12, 4), MUX!(0, "mout_pwm", GROUP2_P, SRC_PERIC0, 24, 4),
    MUX!(0, "mout_aclk200", MPLL_BPLL_P, SRC_TOP0, 12, 1), MUX!(0, "mout_aclk400", MPLL_BPLL_P, SRC_TOP0, 20, 1),
];

static EXYNOS5410_DIV_CLKS: &[SamsungDivClock] = &[
    DIV!(0, "div_arm", "mout_cpu", DIV_CPU0, 0, 3), DIV!(0, "div_arm2", "div_arm", DIV_CPU0, 28, 3), DIV!(0, "div_acp", "div_arm2", DIV_CPU0, 8, 3), DIV!(0, "div_cpud", "div_arm2", DIV_CPU0, 4, 3), DIV!(0, "div_atb", "div_arm2", DIV_CPU0, 16, 3), DIV!(0, "pclk_dbg", "div_arm2", DIV_CPU0, 20, 3),
    DIV!(0, "div_kfc", "mout_kfc", DIV_KFC0, 0, 3), DIV!(0, "div_aclk", "div_kfc", DIV_KFC0, 4, 3), DIV!(0, "div_pclk", "div_kfc", DIV_KFC0, 20, 3),
    DIV!(0, "aclk66_pre", "sclk_mpll_muxed", DIV_TOP1, 24, 3), DIV!(0, "aclk66", "aclk66_pre", DIV_TOP0, 0, 3),
    DIV!(0, "dout_usbphy300", "mout_usbd300", DIV_FSYS0, 16, 4), DIV!(0, "dout_usbphy301", "mout_usbd301", DIV_FSYS0, 20, 4), DIV!(0, "dout_usbd300", "mout_usbd300", DIV_FSYS0, 24, 4), DIV!(0, "dout_usbd301", "mout_usbd301", DIV_FSYS0, 28, 4),
    DIV!(0, "div_mmc0", "mout_mmc0", DIV_FSYS1, 0, 4), DIV!(0, "div_mmc1", "mout_mmc1", DIV_FSYS1, 16, 4), DIV!(0, "div_mmc2", "mout_mmc2", DIV_FSYS2, 0, 4),
    DIV_F!(0, "div_mmc_pre0", "div_mmc0", DIV_FSYS1, 8, 8, CLK_SET_RATE_PARENT, 0), DIV_F!(0, "div_mmc_pre1", "div_mmc1", DIV_FSYS1, 24, 8, CLK_SET_RATE_PARENT, 0), DIV_F!(0, "div_mmc_pre2", "div_mmc2", DIV_FSYS2, 8, 8, CLK_SET_RATE_PARENT, 0),
    DIV!(0, "div_uart0", "mout_uart0", DIV_PERIC0, 0, 4), DIV!(0, "div_uart1", "mout_uart1", DIV_PERIC0, 4, 4), DIV!(0, "div_uart2", "mout_uart2", DIV_PERIC0, 8, 4), DIV!(0, "div_uart3", "mout_uart3", DIV_PERIC0, 12, 4), DIV!(0, "dout_pwm", "mout_pwm", DIV_PERIC3, 0, 4),
    DIV!(0, "aclk200", "mout_aclk200", DIV_TOP0, 12, 3), DIV!(0, "aclk266", "mpll_user_p", DIV_TOP0, 16, 3), DIV!(0, "aclk400", "mout_aclk400", DIV_TOP0, 24, 3),
];

// Gate declarations are retained as the original external clock-description macro calls.
static EXYNOS5410_GATE_CLKS: &[SamsungGateClock] = &[
    GATE!(CLK_SSS, "sss", "aclk266", GATE_IP_G2D, 2, 0, 0), GATE!(CLK_MCT, "mct", "aclk66", GATE_IP_PERIS, 18, 0, 0), GATE!(CLK_WDT, "wdt", "aclk66", GATE_IP_PERIS, 19, 0, 0), GATE!(CLK_RTC, "rtc", "aclk66", GATE_IP_PERIS, 20, 0, 0), GATE!(CLK_TMU, "tmu", "aclk66", GATE_IP_PERIS, 21, 0, 0),
    GATE!(CLK_SCLK_MMC0, "sclk_mmc0", "div_mmc_pre0", SRC_MASK_FSYS, 0, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_MMC1, "sclk_mmc1", "div_mmc_pre1", SRC_MASK_FSYS, 4, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_MMC2, "sclk_mmc2", "div_mmc_pre2", SRC_MASK_FSYS, 8, CLK_SET_RATE_PARENT, 0),
    GATE!(CLK_MMC0, "sdmmc0", "aclk200", GATE_BUS_FSYS0, 12, 0, 0), GATE!(CLK_MMC1, "sdmmc1", "aclk200", GATE_BUS_FSYS0, 13, 0, 0), GATE!(CLK_MMC2, "sdmmc2", "aclk200", GATE_BUS_FSYS0, 14, 0, 0), GATE!(CLK_PDMA1, "pdma1", "aclk200", GATE_BUS_FSYS0, 2, 0, 0), GATE!(CLK_PDMA0, "pdma0", "aclk200", GATE_BUS_FSYS0, 1, 0, 0),
    GATE!(CLK_SCLK_USBPHY301, "sclk_usbphy301", "dout_usbphy301", GATE_TOP_SCLK_FSYS, 7, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_USBPHY300, "sclk_usbphy300", "dout_usbphy300", GATE_TOP_SCLK_FSYS, 8, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_USBD300, "sclk_usbd300", "dout_usbd300", GATE_TOP_SCLK_FSYS, 9, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_USBD301, "sclk_usbd301", "dout_usbd301", GATE_TOP_SCLK_FSYS, 10, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_PWM, "sclk_pwm", "dout_pwm", GATE_TOP_SCLK_PERIC, 11, CLK_SET_RATE_PARENT, 0),
    GATE!(CLK_UART0, "uart0", "aclk66", GATE_IP_PERIC, 0, 0, 0), GATE!(CLK_UART1, "uart1", "aclk66", GATE_IP_PERIC, 1, 0, 0), GATE!(CLK_UART2, "uart2", "aclk66", GATE_IP_PERIC, 2, 0, 0), GATE!(CLK_UART3, "uart3", "aclk66", GATE_IP_PERIC, 3, 0, 0), GATE!(CLK_I2C0, "i2c0", "aclk66", GATE_IP_PERIC, 6, 0, 0), GATE!(CLK_I2C1, "i2c1", "aclk66", GATE_IP_PERIC, 7, 0, 0), GATE!(CLK_I2C2, "i2c2", "aclk66", GATE_IP_PERIC, 8, 0, 0), GATE!(CLK_I2C3, "i2c3", "aclk66", GATE_IP_PERIC, 9, 0, 0), GATE!(CLK_USI0, "usi0", "aclk66", GATE_IP_PERIC, 10, 0, 0), GATE!(CLK_USI1, "usi1", "aclk66", GATE_IP_PERIC, 11, 0, 0), GATE!(CLK_USI2, "usi2", "aclk66", GATE_IP_PERIC, 12, 0, 0), GATE!(CLK_USI3, "usi3", "aclk66", GATE_IP_PERIC, 13, 0, 0), GATE!(CLK_TSADC, "tsadc", "aclk66", GATE_IP_PERIC, 15, 0, 0), GATE!(CLK_PWM, "pwm", "aclk66", GATE_IP_PERIC, 24, 0, 0),
    GATE!(CLK_SCLK_UART0, "sclk_uart0", "div_uart0", SRC_MASK_PERIC0, 0, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_UART1, "sclk_uart1", "div_uart1", SRC_MASK_PERIC0, 4, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_UART2, "sclk_uart2", "div_uart2", SRC_MASK_PERIC0, 8, CLK_SET_RATE_PARENT, 0), GATE!(CLK_SCLK_UART3, "sclk_uart3", "div_uart3", SRC_MASK_PERIC0, 12, CLK_SET_RATE_PARENT, 0),
    GATE!(CLK_USBH20, "usbh20", "aclk200_fsys", GATE_IP_FSYS, 18, 0, 0), GATE!(CLK_USBD300, "usbd300", "aclk200_fsys", GATE_IP_FSYS, 19, 0, 0), GATE!(CLK_USBD301, "usbd301", "aclk200_fsys", GATE_IP_FSYS, 20, 0, 0),
];

static EXYNOS5410_PLL2550X_24MHZ_TBL: &[SamsungPllRateTable] = &[
    PLL_36XX_RATE!(24 * MHZ, 400000000u32, 200, 3, 2, 0), PLL_36XX_RATE!(24 * MHZ, 333000000u32, 111, 2, 2, 0), PLL_36XX_RATE!(24 * MHZ, 300000000u32, 100, 2, 2, 0), PLL_36XX_RATE!(24 * MHZ, 266000000u32, 266, 3, 3, 0), PLL_36XX_RATE!(24 * MHZ, 200000000u32, 200, 3, 3, 0), PLL_36XX_RATE!(24 * MHZ, 192000000u32, 192, 3, 3, 0), PLL_36XX_RATE!(24 * MHZ, 166000000u32, 166, 3, 3, 0), PLL_36XX_RATE!(24 * MHZ, 133000000u32, 266, 3, 4, 0), PLL_36XX_RATE!(24 * MHZ, 100000000u32, 200, 3, 4, 0), PLL_36XX_RATE!(24 * MHZ, 66000000u32, 176, 2, 5, 0),
];

static mut EXYNOS5410_PLLS: [SamsungPllClock; Exynos5410Plls::NrPlls as usize] = [
    PLL!(pll_35xx, CLK_FOUT_APLL, "fout_apll", "fin_pll", APLL_LOCK, APLL_CON0, None),
    PLL!(pll_35xx, CLK_FOUT_CPLL, "fout_cpll", "fin_pll", CPLL_LOCK, CPLL_CON0, None),
    PLL!(pll_2650x, CLK_FOUT_EPLL, "fout_epll", "fin_pll", EPLL_LOCK, EPLL_CON0, None),
    PLL!(pll_35xx, CLK_FOUT_MPLL, "fout_mpll", "fin_pll", MPLL_LOCK, MPLL_CON0, None),
    PLL!(pll_35xx, CLK_FOUT_BPLL, "fout_bpll", "fin_pll", BPLL_LOCK, BPLL_CON0, None),
    PLL!(pll_35xx, CLK_FOUT_KPLL, "fout_kpll", "fin_pll", KPLL_LOCK, KPLL_CON0, None),
];

static CMU: SamsungCmuInfo = SamsungCmuInfo {
    pll_clks: unsafe { &EXYNOS5410_PLLS }, nr_pll_clks: EXYNOS5410_PLLS.len(),
    mux_clks: EXYNOS5410_MUX_CLKS, nr_mux_clks: EXYNOS5410_MUX_CLKS.len(),
    div_clks: EXYNOS5410_DIV_CLKS, nr_div_clks: EXYNOS5410_DIV_CLKS.len(),
    gate_clks: EXYNOS5410_GATE_CLKS, nr_gate_clks: EXYNOS5410_GATE_CLKS.len(), nr_clk_ids: CLKS_NR,
};

// register exynos5410 clocks
unsafe fn exynos5410_clk_init(np: *mut device_node) {
    let xxti = of_clk_get(np, 0);
    if !IS_ERR(xxti) && clk_get_rate(xxti) == 24 * MHZ {
        EXYNOS5410_PLLS[Exynos5410Plls::Epll as usize].rate_table = EXYNOS5410_PLL2550X_24MHZ_TBL.as_ptr();
    }
    samsung_cmu_register_one(np, &CMU);
    pr_debug!("Exynos5410: clock setup completed.\n");
}

CLK_OF_DECLARE!(exynos5410_clk, "samsung,exynos5410-clock", exynos5410_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
