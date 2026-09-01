// SPDX-License-Identifier: GPL-2.0
/*
 * wm9713.h  --  WM9713 Soc Audio driver
 */

/* clock inputs */
pub const WM9713_CLKA_PIN: u32 = 0;
pub const WM9713_CLKB_PIN: u32 = 1;

/* clock divider ID's */
pub const WM9713_PCMCLK_DIV: u32 = 0;
pub const WM9713_CLKA_MULT: u32 = 1;
pub const WM9713_CLKB_MULT: u32 = 2;
pub const WM9713_HIFI_DIV: u32 = 3;
pub const WM9713_PCMBCLK_DIV: u32 = 4;
pub const WM9713_PCMCLK_PLL_DIV: u32 = 5;
pub const WM9713_HIFI_PLL_DIV: u32 = 6;

/* Calculate the appropriate bit mask for the external PCM clock divider */
pub const fn WM9713_PCMDIV(x: u32) -> u32 {
    (x.wrapping_sub(1)) << 8
}

/* Calculate the appropriate bit mask for the external HiFi clock divider */
pub const fn WM9713_HIFIDIV(x: u32) -> u32 {
    (x.wrapping_sub(1)) << 12
}

/* MCLK clock mulitipliers */
pub const WM9713_CLKA_X1: u32 = 0 << 1;
pub const WM9713_CLKA_X2: u32 = 1 << 1;
pub const WM9713_CLKB_X1: u32 = 0 << 2;
pub const WM9713_CLKB_X2: u32 = 1 << 2;

/* MCLK clock MUX */
pub const WM9713_CLK_MUX_A: u32 = 0 << 0;
pub const WM9713_CLK_MUX_B: u32 = 1 << 0;

/* Voice DAI BCLK divider */
pub const WM9713_PCMBCLK_DIV_1: u32 = 0 << 9;
pub const WM9713_PCMBCLK_DIV_2: u32 = 1 << 9;
pub const WM9713_PCMBCLK_DIV_4: u32 = 2 << 9;
pub const WM9713_PCMBCLK_DIV_8: u32 = 3 << 9;
pub const WM9713_PCMBCLK_DIV_16: u32 = 4 << 9;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
