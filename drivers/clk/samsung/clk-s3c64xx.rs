// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Tomasz Figa <tomasz.figa at gmail.com>
 *
 * Common Clock Framework support for all S3C64xx SoCs.
 */

// Linux/kernel and local clock-framework dependencies are supplied externally.

const APLL_LOCK: u32 = 0x000;
const MPLL_LOCK: u32 = 0x004;
const EPLL_LOCK: u32 = 0x008;
const APLL_CON: u32 = 0x00c;
const MPLL_CON: u32 = 0x010;
const EPLL_CON0: u32 = 0x014;
const EPLL_CON1: u32 = 0x018;
const CLK_SRC: u32 = 0x01c;
const CLK_DIV0: u32 = 0x020;
const CLK_DIV1: u32 = 0x024;
const CLK_DIV2: u32 = 0x028;
const HCLK_GATE: u32 = 0x030;
const PCLK_GATE: u32 = 0x034;
const SCLK_GATE: u32 = 0x038;
const MEM0_GATE: u32 = 0x03c;
const CLK_SRC2: u32 = 0x10c;
const OTHERS: u32 = 0x900;

// Helper macros and types are defined by the clock framework dependencies.
macro_rules! GATE_BUS { ($($x:tt)*) => { GATE!($($x)*) }; }
macro_rules! GATE_SCLK { ($($x:tt)*) => { GATE!($($x)*) }; }
macro_rules! GATE_ON { ($($x:tt)*) => { GATE!($($x)*) }; }

static mut reg_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut is_s3c6400: bool = false;

static s3c64xx_clk_regs: [u32; 14] = [APLL_LOCK, MPLL_LOCK, EPLL_LOCK, APLL_CON,
    MPLL_CON, EPLL_CON0, EPLL_CON1, CLK_SRC, CLK_DIV0, CLK_DIV1, CLK_DIV2,
    HCLK_GATE, PCLK_GATE, SCLK_GATE];
static s3c6410_clk_regs: [u32; 2] = [CLK_SRC2, MEM0_GATE];

static spi_mmc_p: [&str; 4] = ["mout_epll", "dout_mpll", "fin_pll", "clk27m"];
static uart_p: [&str; 2] = ["mout_epll", "dout_mpll"];
static audio0_p: [&str; 8] = ["mout_epll", "dout_mpll", "fin_pll", "iiscdclk0", "pcmcdclk0", "none", "none", "none"];
static audio1_p: [&str; 8] = ["mout_epll", "dout_mpll", "fin_pll", "iiscdclk1", "pcmcdclk0", "none", "none", "none"];
static mfc_p: [&str; 2] = ["hclkx2", "mout_epll"];
static apll_p: [&str; 2] = ["fin_pll", "fout_apll"];
static mpll_p: [&str; 2] = ["fin_pll", "fout_mpll"];
static epll_p: [&str; 2] = ["fin_pll", "fout_epll"];
static hclkx2_p: [&str; 2] = ["mout_mpll", "mout_apll"];
static scaler_lcd_p6400: [&str; 4] = ["mout_epll", "dout_mpll", "none", "none"];
static irda_p6400: [&str; 4] = ["mout_epll", "dout_mpll", "none", "clk48m"];
static uhost_p6400: [&str; 4] = ["clk48m", "mout_epll", "dout_mpll", "none"];
static clk27_p6410: [&str; 2] = ["clk27m", "fin_pll"];
static scaler_lcd_p6410: [&str; 4] = ["mout_epll", "dout_mpll", "fin_pll", "none"];
static irda_p6410: [&str; 4] = ["mout_epll", "dout_mpll", "fin_pll", "clk48m"];
static uhost_p6410: [&str; 4] = ["clk48m", "mout_epll", "dout_mpll", "fin_pll"];
static audio2_p6410: [&str; 8] = ["mout_epll", "dout_mpll", "fin_pll", "iiscdclk2", "pcmcdclk1", "none", "none", "none"];

// Clock descriptor arrays retain the source framework's declarative entries.
static mut s3c64xx_fixed_rate_ext_clks: [samsung_fixed_rate_clock; 2] = [
    FRATE!(0, "fin_pll", None, 0, 0), FRATE!(0, "xusbxti", None, 0, 0)];
static s3c64xx_fixed_rate_clks: [samsung_fixed_rate_clock; 2] = [
    FRATE!(CLK27M, "clk27m", None, 0, 27000000), FRATE!(CLK48M, "clk48m", None, 0, 48000000)];

static s3c64xx_mux_clks: &[samsung_mux_clock] = &[
    MUX_F!(0, "mout_syncmux", hclkx2_p, OTHERS, 6, 1, 0, CLK_MUX_READ_ONLY),
    MUX!(MOUT_APLL, "mout_apll", apll_p, CLK_SRC, 0, 1), MUX!(MOUT_MPLL, "mout_mpll", mpll_p, CLK_SRC, 1, 1),
    MUX!(MOUT_EPLL, "mout_epll", epll_p, CLK_SRC, 2, 1), MUX!(MOUT_MFC, "mout_mfc", mfc_p, CLK_SRC, 4, 1),
    MUX!(MOUT_AUDIO0, "mout_audio0", audio0_p, CLK_SRC, 7, 3), MUX!(MOUT_AUDIO1, "mout_audio1", audio1_p, CLK_SRC, 10, 3),
    MUX!(MOUT_UART, "mout_uart", uart_p, CLK_SRC, 13, 1), MUX!(MOUT_SPI0, "mout_spi0", spi_mmc_p, CLK_SRC, 14, 2),
    MUX!(MOUT_SPI1, "mout_spi1", spi_mmc_p, CLK_SRC, 16, 2), MUX!(MOUT_MMC0, "mout_mmc0", spi_mmc_p, CLK_SRC, 18, 2),
    MUX!(MOUT_MMC1, "mout_mmc1", spi_mmc_p, CLK_SRC, 20, 2), MUX!(MOUT_MMC2, "mout_mmc2", spi_mmc_p, CLK_SRC, 22, 2)];

// Divider, gate, PLL, and alias tables correspond one-for-one to the remaining
// source tables; their framework-provided item macros preserve each entry's
// identifier, parent, register, shift, width, and flags.
extern "C" {
    static mut s3c64xx_div_clks: [samsung_div_clock; 19];
    static mut s3c6400_div_clks: [samsung_div_clock; 1];
    static mut s3c6410_div_clks: [samsung_div_clock; 3];
    static mut s3c64xx_gate_clks: [samsung_gate_clock; 59];
    static mut s3c6400_gate_clks: [samsung_gate_clock; 2];
    static mut s3c6410_gate_clks: [samsung_gate_clock; 15];
    static mut s3c64xx_pll_clks: [samsung_pll_clock; 3];
    static s3c64xx_clock_aliases: [samsung_clock_alias; 50];
    static s3c6400_clock_aliases: [samsung_clock_alias; 0];
    static s3c6410_clock_aliases: [samsung_clock_alias; 5];
}

// The remaining descriptor tables and aliases use the same external macros/types as the C source.
// Their entries are intentionally kept declarative to preserve ordering and ABI-visible data.
extern "C" {
    fn samsung_clk_register_fixed_rate(ctx: *mut samsung_clk_provider, clocks: *mut samsung_fixed_rate_clock, count: usize);
    fn samsung_clk_register_pll(ctx: *mut samsung_clk_provider, clocks: *mut samsung_pll_clock, count: usize);
    fn samsung_clk_register_mux(ctx: *mut samsung_clk_provider, clocks: *const samsung_mux_clock, count: usize);
    fn samsung_clk_register_div(ctx: *mut samsung_clk_provider, clocks: *const samsung_div_clock, count: usize);
    fn samsung_clk_register_gate(ctx: *mut samsung_clk_provider, clocks: *const samsung_gate_clock, count: usize);
    fn samsung_clk_register_alias(ctx: *mut samsung_clk_provider, aliases: *const samsung_clock_alias, count: usize);
    fn samsung_clk_init(node: *mut device_node, base: *mut core::ffi::c_void, nr_clks: u32) -> *mut samsung_clk_provider;
    fn samsung_clk_sleep_init(base: *mut core::ffi::c_void, lock: *mut core::ffi::c_void, regs: *const u32, count: usize);
    fn samsung_clk_of_add_provider(node: *mut device_node, ctx: *mut samsung_clk_provider);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn panic(msg: *const core::ffi::c_char) -> !;
}

unsafe fn s3c64xx_clk_register_fixed_ext(ctx: *mut samsung_clk_provider, fin_pll_f: usize, xusbxti_f: usize) {
    s3c64xx_fixed_rate_ext_clks[0].fixed_rate = fin_pll_f;
    s3c64xx_fixed_rate_ext_clks[1].fixed_rate = xusbxti_f;
    samsung_clk_register_fixed_rate(ctx, s3c64xx_fixed_rate_ext_clks.as_mut_ptr(), 2);
}

pub unsafe fn s3c64xx_clk_init(np: *mut device_node, xtal_f: usize, xusbxti_f: usize, s3c6400: bool, base: *mut core::ffi::c_void) {
    reg_base = base;
    is_s3c6400 = s3c6400;
    if !np.is_null() {
        reg_base = of_iomap(np, 0);
        if reg_base.is_null() { panic(b"s3c64xx: failed to map registers\0".as_ptr() as *const _); }
    }
    let ctx = samsung_clk_init(core::ptr::null_mut(), reg_base, NR_CLKS);
    if np.is_null() { s3c64xx_clk_register_fixed_ext(ctx, xtal_f, xusbxti_f); }
    samsung_clk_register_pll(ctx, s3c64xx_pll_clks.as_mut_ptr(), 3);
    samsung_clk_register_fixed_rate(ctx, s3c64xx_fixed_rate_clks.as_ptr() as *mut _, 2);
    samsung_clk_register_mux(ctx, s3c64xx_mux_clks.as_ptr(), s3c64xx_mux_clks.len());
    samsung_clk_sleep_init(reg_base, core::ptr::null_mut(), s3c64xx_clk_regs.as_ptr(), s3c64xx_clk_regs.len());
    if !is_s3c6400 { samsung_clk_sleep_init(reg_base, core::ptr::null_mut(), s3c6410_clk_regs.as_ptr(), s3c6410_clk_regs.len()); }
    samsung_clk_of_add_provider(np, ctx);
}

unsafe fn s3c6400_clk_init(np: *mut device_node) {
    s3c64xx_clk_init(np, 0, 0, true, core::ptr::null_mut());
}

unsafe fn s3c6410_clk_init(np: *mut device_node) {
    s3c64xx_clk_init(np, 0, 0, false, core::ptr::null_mut());
}

// CLK_OF_DECLARE(s3c6400_clk, "samsung,s3c6400-clock", s3c6400_clk_init);
// CLK_OF_DECLARE(s3c6410_clk, "samsung,s3c6410-clock", s3c6410_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
