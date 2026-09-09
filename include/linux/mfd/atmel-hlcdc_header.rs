/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Free Electrons
 * Copyright (C) 2014 Atmel
 *
 * Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
 */

// The C header includes linux/clk.h and linux/regmap.h; those dependencies
// are supplied by the surrounding translation unit.

pub const fn bit(n: u32) -> u32 {
    1u32 << n
}

pub const fn genmask(high: u32, low: u32) -> u32 {
    ((1u32 << (high - low + 1)) - 1) << low
}

pub const fn ATMEL_HLCDC_CFG(i: u32) -> u32 {
    i * 0x4
}

// LCDCFG is supplied by another header in the original source.
pub const ATMEL_HLCDC_SIG_CFG: u32 = ATMEL_HLCDC_CFG(5);
pub const ATMEL_HLCDC_HSPOL: u32 = bit(0);
pub const ATMEL_HLCDC_VSPOL: u32 = bit(1);
pub const ATMEL_HLCDC_VSPDLYS: u32 = bit(2);
pub const ATMEL_HLCDC_VSPDLYE: u32 = bit(3);
pub const ATMEL_HLCDC_DISPPOL: u32 = bit(4);
pub const ATMEL_HLCDC_DITHER: u32 = bit(6);
pub const ATMEL_HLCDC_DISPDLY: u32 = bit(7);
pub const ATMEL_HLCDC_MODE_MASK: u32 = genmask(9, 8);
pub const ATMEL_XLCDC_MODE_MASK: u32 = genmask(10, 8);
pub const ATMEL_XLCDC_DPI: u32 = bit(11);
pub const ATMEL_HLCDC_PP: u32 = bit(10);
pub const ATMEL_HLCDC_VSPSU: u32 = bit(12);
pub const ATMEL_HLCDC_VSPHO: u32 = bit(13);
pub const ATMEL_HLCDC_GUARDTIME_MASK: u32 = genmask(20, 16);

pub const ATMEL_HLCDC_EN: u32 = 0x20;
pub const ATMEL_HLCDC_DIS: u32 = 0x24;
pub const ATMEL_HLCDC_SR: u32 = 0x28;
pub const ATMEL_HLCDC_IER: u32 = 0x2c;
pub const ATMEL_HLCDC_IDR: u32 = 0x30;
pub const ATMEL_HLCDC_IMR: u32 = 0x34;
pub const ATMEL_HLCDC_ISR: u32 = 0x38;
pub const ATMEL_XLCDC_ATTRE: u32 = 0x3c;

pub const ATMEL_XLCDC_BASE_UPDATE: u32 = bit(0);
pub const ATMEL_XLCDC_OVR1_UPDATE: u32 = bit(1);
pub const ATMEL_XLCDC_OVR3_UPDATE: u32 = bit(2);
pub const ATMEL_XLCDC_HEO_UPDATE: u32 = bit(3);

pub const ATMEL_HLCDC_CLKPOL: u32 = bit(0);
pub const ATMEL_HLCDC_CLKSEL: u32 = bit(2);
pub const ATMEL_HLCDC_CLKPWMSEL: u32 = bit(3);
pub const fn ATMEL_HLCDC_CGDIS(i: u32) -> u32 { bit(8 + i) }
pub const ATMEL_HLCDC_CLKDIV_SHFT: u32 = 16;
pub const ATMEL_HLCDC_CLKDIV_MASK: u32 = genmask(23, 16);
pub const fn ATMEL_HLCDC_CLKDIV(div: u32) -> u32 {
    (div - 2) << ATMEL_HLCDC_CLKDIV_SHFT
}

pub const ATMEL_HLCDC_PIXEL_CLK: u32 = bit(0);
pub const ATMEL_HLCDC_SYNC: u32 = bit(1);
pub const ATMEL_HLCDC_DISP: u32 = bit(2);
pub const ATMEL_HLCDC_PWM: u32 = bit(3);
pub const ATMEL_HLCDC_SIP: u32 = bit(4);
pub const ATMEL_XLCDC_SD: u32 = bit(5);
pub const ATMEL_XLCDC_CM: u32 = bit(6);

pub const ATMEL_HLCDC_SOF: u32 = bit(0);
pub const ATMEL_HLCDC_SYNCDIS: u32 = bit(1);
pub const ATMEL_HLCDC_FIFOERR: u32 = bit(4);
pub const fn ATMEL_HLCDC_LAYER_STATUS(x: u32) -> u32 { bit(x + 8) }

/**
 * Structure shared by the MFD device and its subdevices.
 *
 * @regmap: register map used to access HLCDC IP registers
 * @periph_clk: the hlcdc peripheral clock
 * @sys_clk: the hlcdc system clock
 * @slow_clk: the system slow clk
 * @irq: the hlcdc irq
 */
#[repr(C)]
pub struct atmel_hlcdc {
    pub regmap: *mut regmap,
    pub lvds_pll_clk: *mut clk,
    pub periph_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub slow_clk: *mut clk,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
