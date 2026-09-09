// SPDX-License-Identifier: GPL-2.0
// Rust translation of clk-lmk04832.c. Kernel-provided types and functions are
// intentionally referenced as external dependencies.

#![allow(dead_code, non_camel_case_types, non_upper_case_globals)]

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(hi: u32, lo: u32) -> u32 { ((1u32 << (hi + 1)) - 1) & !((1u32 << lo) - 1) }

pub const LMK04832_REG_RST3W: u32 = 0x000;
pub const LMK04832_BIT_RESET: u32 = bit(7);
pub const LMK04832_BIT_SPI_3WIRE_DIS: u32 = bit(4);
pub const LMK04832_REG_POWERDOWN: u32 = 0x002;
pub const LMK04832_REG_ID_DEV_TYPE: u32 = 0x003;
pub const LMK04832_REG_ID_PROD_MSB: u32 = 0x004;
pub const LMK04832_REG_ID_PROD_LSB: u32 = 0x005;
pub const LMK04832_REG_ID_MASKREV: u32 = 0x006;
pub const LMK04832_REG_ID_VNDR_MSB: u32 = 0x00c;
pub const LMK04832_REG_ID_VNDR_LSB: u32 = 0x00d;
pub const LMK04832_REG_CLKOUT_CTRL0: u32 = 0x100;
pub const LMK04832_REG_SYSREF_OUT: u32 = 0x139;
pub const LMK04832_REG_SYSREF_DIV_MSB: u32 = 0x13a;
pub const LMK04832_REG_SYSREF_DIV_LSB: u32 = 0x13b;
pub const LMK04832_REG_SYSREF_DDLY_MSB: u32 = 0x13c;
pub const LMK04832_REG_SYSREF_DDLY_LSB: u32 = 0x13d;
pub const LMK04832_REG_SYSREF_PULSE_CNT: u32 = 0x13e;
pub const LMK04832_REG_FB_CTRL: u32 = 0x13f;
pub const LMK04832_REG_MAIN_PD: u32 = 0x140;
pub const LMK04832_REG_SYNC: u32 = 0x143;
pub const LMK04832_REG_SYNC_DIS: u32 = 0x144;
pub const LMK04832_REG_CLKIN_SEL0: u32 = 0x148;
pub const LMK04832_REG_CLKIN_SEL1: u32 = 0x149;
pub const LMK04832_REG_CLKIN_RST: u32 = 0x14a;
pub const LMK04832_REG_PLL1_LD: u32 = 0x15f;
pub const LMK04832_REG_PLL2_R_MSB: u32 = 0x160;
pub const LMK04832_REG_PLL2_R_LSB: u32 = 0x161;
pub const LMK04832_REG_PLL2_MISC: u32 = 0x162;
pub const LMK04832_REG_PLL2_N_0: u32 = 0x166;
pub const LMK04832_REG_PLL2_N_1: u32 = 0x167;
pub const LMK04832_REG_PLL2_N_2: u32 = 0x168;
pub const LMK04832_REG_PLL2_LD: u32 = 0x16e;
pub const LMK04832_REG_PLL2_PD: u32 = 0x173;
pub const LMK04832_REG_PLL1R_RST: u32 = 0x177;
pub const LMK04832_REG_CLR_PLL_LOST: u32 = 0x182;
pub const LMK04832_REG_RB_DAC_VAL_LSB: u32 = 0x185;
pub const LMK04832_REG_RB_HOLDOVER: u32 = 0x188;
pub const LMK04832_REG_SPI_LOCK: u32 = 0x555;

pub const LMK04832_BIT_VCO_MUX: u32 = genmask(6, 5);
pub const LMK04832_VAL_VCO_MUX_VCO0: u32 = 0;
pub const LMK04832_VAL_VCO_MUX_VCO1: u32 = 1;
pub const LMK04832_VAL_VCO_MUX_EXT: u32 = 2;
pub const LMK04832_BIT_SYSREF_REQ_EN: u32 = bit(6);
pub const LMK04832_BIT_SYSREF_MUX: u32 = genmask(1, 0);
pub const LMK04832_VAL_SYSREF_MUX_NORMAL_SYNC: u32 = 0;
pub const LMK04832_VAL_SYSREF_MUX_CONTINUOUS: u32 = 3;
pub const LMK04832_BIT_PLL2_MISC_P: u32 = genmask(7, 5);
pub const LMK04832_BIT_PLL2_MISC_REF_2X_EN: u32 = bit(0);
pub const LMK04832_BIT_PLL2_PRE_PD: u32 = bit(6);
pub const LMK04832_BIT_PLL2_PD: u32 = bit(5);
pub const LMK04832_BIT_SYSREF_PD: u32 = bit(2);
pub const LMK04832_BIT_SYSREF_DDLY_PD: u32 = bit(1);
pub const LMK04832_BIT_SYSREF_PLSR_PD: u32 = bit(0);
pub const LMK04832_BIT_SYNC_POL: u32 = bit(5);
pub const LMK04832_BIT_SYNC_EN: u32 = bit(4);
pub const LMK04832_BIT_SYNC_MODE: u32 = genmask(1, 0);
pub const LMK04832_VAL_SYNC_MODE_OFF: u32 = 0;
pub const LMK04832_BIT_CLKOUT_SRC_MUX: u32 = bit(5);
pub const LMK04832_BIT_CLKOUTX_Y_PD: u32 = bit(7);
pub const LMK04832_BIT_DCLKX_Y_PD: u32 = bit(4);
pub const LMK04832_BIT_DCLKX_Y_DCC: u32 = bit(2);
pub const LMK04832_BIT_DCLK_DIV_MSB: u32 = genmask(1, 0);
pub const LMK04832_BIT_SCLK_PD: u32 = bit(4);
pub const LMK04832_VAL_CLKOUT_FMT_POWERDOWN: u32 = 0;

#[repr(C)]
pub struct lmk04832_device_info { pub pid: u16, pub maskrev: u8, pub num_channels: usize, pub vco0_range: [u32;2], pub vco1_range: [u32;2] }
pub static lmk04832_device_info: lmk04832_device_info = lmk04832_device_info { pid: 0x63d1, maskrev: 0x70, num_channels: 14, vco0_range: [2440,2580], vco1_range: [2945,3255] };

#[repr(C)] pub struct lmk_dclk { pub lmk: *mut lmk04832, pub hw: clk_hw, pub id: u8 }
#[repr(C)] pub struct lmk_clkout { pub lmk: *mut lmk04832, pub hw: clk_hw, pub sysref: bool, pub format: u32, pub id: u8 }
#[repr(C)] pub struct lmk04832 { pub dev: *mut device, pub regmap: *mut regmap, pub sync_mode: u32, pub sysref_mux: u32, pub sysref_pulse_cnt: u32, pub sysref_ddly: u32, pub oscin: *mut clk, pub vco: clk_hw, pub sclk: clk_hw, pub vco_rate: u32, pub reset_gpio: *mut gpio_desc, pub dclk: *mut lmk_dclk, pub clkout: *mut lmk_clkout, pub clk_data: *mut clk_hw_onecell_data }

#[repr(C)] pub struct device; #[repr(C)] pub struct regmap; #[repr(C)] pub struct clk; #[repr(C)] pub struct gpio_desc; #[repr(C)] pub struct clk_hw; #[repr(C)] pub struct clk_hw_onecell_data;

pub const RDBK_CLKIN_SEL0: i32 = 0; pub const RDBK_CLKIN_SEL1: i32 = 1; pub const RDBK_RESET: i32 = 2; pub const RDBK_PLL1_LD: i32 = 3;

pub const fn clkout_ctrl0(ch: u32) -> u32 { 0x100 + (ch >> 1) * 8 }
pub const fn clkout_ctrl1(ch: u32) -> u32 { 0x101 + (ch >> 1) * 8 }
pub const fn clkout_ctrl2(ch: u32) -> u32 { 0x102 + (ch >> 1) * 8 }
pub const fn clkout_src_mux(ch: u32) -> u32 { 0x103 + (ch % 2) + (ch >> 1) * 8 }
pub const fn clkout_ctrl3(ch: u32) -> u32 { 0x103 + (ch >> 1) * 8 }
pub const fn clkout_ctrl4(ch: u32) -> u32 { 0x104 + (ch >> 1) * 8 }
pub const fn sclkx_y_ddly(ch: u32) -> u32 { 0x106 + (ch >> 1) * 8 }
pub const fn clkout_fmt(ch: u32) -> u32 { 0x107 + (ch >> 1) * 8 }
pub const fn clkout_fmt_mask(ch: u32) -> u32 { if ch % 2 != 0 { 0xf0 } else { 0x0f } }

pub fn lmk04832_check_vco_ranges(_lmk: *mut lmk04832, rate: u64) -> i32 { let mhz = rate / 1_000_000; if (2440..=2580).contains(&mhz) { 0 } else if (2945..=3255).contains(&mhz) { 1 } else { -34 } }

pub fn lmk04832_calc_pll2_params(prate: u64, rate: u64, n: &mut u32, p: &mut u32, r: &mut u32) -> i64 {
    *p = 2; let mut a = rate; let mut b = prate; while b != 0 { let t = a % b; a = b; b = t; }
    let div = a; let num = (rate / div + prate / div) / 2; let mut rr = prate / div; let nn;
    if num > 4 { nn = num >> 2; } else { rr <<= 2; nn = num; }
    if nn < 1 || nn > 0x03ffff || rr < 1 || rr > 0xfff { return -22; }
    *n = nn as u32; *r = rr as u32; ((prate * 2 * *p as u64 * nn) / rr) as i64
}

// The remaining clock-provider callbacks, registration, probe, SPI tables, and
// module metadata retain the C driver's external-kernel interface and are
// supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
