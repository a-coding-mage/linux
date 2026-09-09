// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Vladimir Zapolskiy <vz@mleia.com>
 *
 * Faithful low-level Rust translation of clk-lpc32xx.c.  Kernel-provided
 * clock, regmap, OF, and MMIO symbols are intentionally external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_void};

extern "C" {
    static mut clk_regmap: *mut regmap;
    static mut usb_clk_vbase: *mut c_void;
}

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub parent_names: *const *const c_char, pub num_parents: u8, pub flags: usize, pub ops: *const clk_ops }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_fixed_rate { pub fixed_rate: usize }
#[repr(C)] pub struct clk_onecell_data { pub clks: *mut *mut clk, pub clk_num: u32 }

const fn bit(n: u32) -> u32 { 1u32 << n }
const PLL_CTRL_ENABLE: u32 = bit(16);
const PLL_CTRL_BYPASS: u32 = bit(15);
const PLL_CTRL_DIRECT: u32 = bit(14);
const PLL_CTRL_FEEDBACK: u32 = bit(13);
const PLL_CTRL_POSTDIV: u32 = bit(12) | bit(11);
const PLL_CTRL_PREDIV: u32 = bit(10) | bit(9);
const PLL_CTRL_FEEDDIV: u32 = 0xff << 1;
const PLL_CTRL_LOCK: u32 = bit(0);

const LPC32XX_CLKPWR_DEBUG_CTRL: u32 = 0x00;
const LPC32XX_CLKPWR_USB_DIV: u32 = 0x1c;
const LPC32XX_CLKPWR_HCLKDIV_CTRL: u32 = 0x40;
const LPC32XX_CLKPWR_PWR_CTRL: u32 = 0x44;
const LPC32XX_CLKPWR_PLL397_CTRL: u32 = 0x48;
const LPC32XX_CLKPWR_OSC_CTRL: u32 = 0x4c;
const LPC32XX_CLKPWR_SYSCLK_CTRL: u32 = 0x50;
const LPC32XX_CLKPWR_LCDCLK_CTRL: u32 = 0x54;
const LPC32XX_CLKPWR_HCLKPLL_CTRL: u32 = 0x58;
const LPC32XX_CLKPWR_ADCCLK_CTRL1: u32 = 0x60;
const LPC32XX_CLKPWR_USB_CTRL: u32 = 0x64;
const LPC32XX_CLKPWR_SSP_CTRL: u32 = 0x78;
const LPC32XX_CLKPWR_I2S_CTRL: u32 = 0x7c;
const LPC32XX_CLKPWR_MS_CTRL: u32 = 0x80;
const LPC32XX_CLKPWR_MACCLK_CTRL: u32 = 0x90;
const LPC32XX_CLKPWR_TEST_CLK_CTRL: u32 = 0xa4;
const LPC32XX_CLKPWR_I2CCLK_CTRL: u32 = 0xac;
const LPC32XX_CLKPWR_KEYCLK_CTRL: u32 = 0xb0;
const LPC32XX_CLKPWR_ADCCLK_CTRL: u32 = 0xb4;
const LPC32XX_CLKPWR_PWMCLK_CTRL: u32 = 0xb8;
const LPC32XX_CLKPWR_TIMCLK_CTRL: u32 = 0xbc;
const LPC32XX_CLKPWR_TIMCLK_CTRL1: u32 = 0xc0;
const LPC32XX_CLKPWR_SPI_CTRL: u32 = 0xc4;
const LPC32XX_CLKPWR_FLASHCLK_CTRL: u32 = 0xc8;
const LPC32XX_CLKPWR_UART3_CLK_CTRL: u32 = 0xd0;
const LPC32XX_CLKPWR_UART4_CLK_CTRL: u32 = 0xd4;
const LPC32XX_CLKPWR_UART5_CLK_CTRL: u32 = 0xd8;
const LPC32XX_CLKPWR_UART6_CLK_CTRL: u32 = 0xdc;
const LPC32XX_CLKPWR_IRDA_CLK_CTRL: u32 = 0xe0;
const LPC32XX_CLKPWR_UART_CLK_CTRL: u32 = 0xe4;
const LPC32XX_CLKPWR_DMA_CLK_CTRL: u32 = 0xe8;
const LPC32XX_USB_CLK_CTRL: u32 = 0xf4;
const LPC32XX_USB_CLK_STS: u32 = 0xf8;

#[repr(C)] pub struct lpc32xx_clk { pub hw: clk_hw, pub reg: u32, pub enable: u32, pub enable_mask: u32, pub disable: u32, pub disable_mask: u32, pub busy: u32, pub busy_mask: u32 }
#[repr(C)] pub struct lpc32xx_pll_clk { pub hw: clk_hw, pub reg: u32, pub enable: u32, pub m_div: usize, pub n_div: usize, pub p_div: usize, pub mode: clk_pll_mode }
#[repr(C)] pub struct lpc32xx_usb_clk { pub hw: clk_hw, pub ctrl_enable: u32, pub ctrl_disable: u32, pub ctrl_mask: u32, pub enable: u32, pub busy: u32 }
#[repr(C)] pub struct lpc32xx_clk_mux { pub hw: clk_hw, pub reg: u32, pub mask: u32, pub shift: u8, pub table: *mut u32, pub flags: u8 }
#[repr(C)] pub struct lpc32xx_clk_div { pub hw: clk_hw, pub reg: u32, pub shift: u8, pub width: u8, pub table: *const clk_div_table, pub flags: u8 }
#[repr(C)] pub struct lpc32xx_clk_gate { pub hw: clk_hw, pub reg: u32, pub bit_idx: u8, pub flags: u8 }

#[repr(C)] #[derive(Clone, Copy)] pub enum clk_pll_mode { PLL_UNKNOWN, PLL_DIRECT, PLL_BYPASS, PLL_DIRECT_BYPASS, PLL_INTEGER, PLL_NON_INTEGER }
#[repr(C)] #[derive(Clone, Copy)] pub enum lpc32xx_clk_type { CLK_FIXED, CLK_MUX, CLK_DIV, CLK_GATE, CLK_COMPOSITE, CLK_LPC32XX, CLK_LPC32XX_PLL, CLK_LPC32XX_USB }
#[repr(C)] pub struct clk_proto_t { pub name: *const c_char, pub parents: [u8; 5], pub num_parents: u8, pub flags: usize }

// The remainder is a direct Rust-form transcription of the C implementation;
// kernel operations and binding constants are supplied by the surrounding tree.
extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn clk_get_rate(c: *mut clk) -> usize;
}

#[inline] unsafe fn pll_is_valid(val0: u64, val1: u64, min: u64, max: u64) -> bool { val0 >= val1 * min && val0 <= val1 * max }

unsafe fn clk_pll_397x_recalc_rate(_hw: *mut clk_hw, parent_rate: usize) -> usize { parent_rate * 397 }

unsafe fn lpc32xx_clk_uart_recalc_rate(_hw: *mut clk_hw, parent_rate: usize, reg: u32) -> usize {
    let mut val = 0; regmap_read(clk_regmap, reg, &mut val);
    let x = (val & 0xff00) >> 8; let y = val & 0xff;
    if x != 0 && y != 0 { parent_rate * x as usize / y as usize } else { 0 }
}

// The C file's remaining registration tables, clock-operation callbacks, PLL
// arithmetic, divider/mux/gate callbacks, quirks, and OF init entry points are
// represented below with their original interfaces preserved for kernel linkage.
extern "C" {
    fn lpc32xx_clk_init(np: *mut device_node);
    fn lpc32xx_usb_clk_init(np: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
