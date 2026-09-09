/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Freescale MXS Low Resolution Analog-to-Digital Converter driver
 *
 * Copyright (c) 2012 DENX Software Engineering, GmbH.
 * Copyright (c) 2016 Ksenija Stanojevic <ksenija.stanojevic@gmail.com>
 *
 * Author: Marek Vasut <marex@denx.de>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const LRADC_MAX_DELAY_CHANS: u32 = 4;
pub const LRADC_MAX_MAPPED_CHANS: u32 = 8;
pub const LRADC_MAX_TOTAL_CHANS: u32 = 16;

pub const LRADC_DELAY_TIMER_HZ: u32 = 2000;

pub const LRADC_CTRL0: u32 = 0x00;
pub const LRADC_CTRL0_MX28_TOUCH_DETECT_ENABLE: u32 = 1 << 23;
pub const LRADC_CTRL0_MX28_TOUCH_SCREEN_TYPE: u32 = 1 << 22;
pub const LRADC_CTRL0_MX28_YNNSW: u32 = 1 << 21;
pub const LRADC_CTRL0_MX28_YPNSW: u32 = 1 << 20;
pub const LRADC_CTRL0_MX28_YPPSW: u32 = 1 << 19;
pub const LRADC_CTRL0_MX28_XNNSW: u32 = 1 << 18;
pub const LRADC_CTRL0_MX28_XNPSW: u32 = 1 << 17;
pub const LRADC_CTRL0_MX28_XPPSW: u32 = 1 << 16;
pub const LRADC_CTRL0_MX23_TOUCH_DETECT_ENABLE: u32 = 1 << 20;
pub const LRADC_CTRL0_MX23_YM: u32 = 1 << 19;
pub const LRADC_CTRL0_MX23_XM: u32 = 1 << 18;
pub const LRADC_CTRL0_MX23_YP: u32 = 1 << 17;
pub const LRADC_CTRL0_MX23_XP: u32 = 1 << 16;
pub const LRADC_CTRL0_MX28_PLATE_MASK: u32 = LRADC_CTRL0_MX28_TOUCH_DETECT_ENABLE | LRADC_CTRL0_MX28_YNNSW | LRADC_CTRL0_MX28_YPNSW | LRADC_CTRL0_MX28_YPPSW | LRADC_CTRL0_MX28_XNNSW | LRADC_CTRL0_MX28_XNPSW | LRADC_CTRL0_MX28_XPPSW;
pub const LRADC_CTRL0_MX23_PLATE_MASK: u32 = LRADC_CTRL0_MX23_TOUCH_DETECT_ENABLE | LRADC_CTRL0_MX23_YM | LRADC_CTRL0_MX23_XM | LRADC_CTRL0_MX23_YP | LRADC_CTRL0_MX23_XP;

pub const LRADC_CTRL1: u32 = 0x10;
pub const LRADC_CTRL1_TOUCH_DETECT_IRQ_EN: u32 = 1 << 24;
#[inline] pub const fn LRADC_CTRL1_LRADC_IRQ_EN(n: u32) -> u32 { 1 << (n + 16) }
pub const LRADC_CTRL1_MX28_LRADC_IRQ_EN_MASK: u32 = 0x1fff << 16;
pub const LRADC_CTRL1_MX23_LRADC_IRQ_EN_MASK: u32 = 0x01ff << 16;
pub const LRADC_CTRL1_LRADC_IRQ_EN_OFFSET: u32 = 16;
pub const LRADC_CTRL1_TOUCH_DETECT_IRQ: u32 = 1 << 8;
#[inline] pub const fn LRADC_CTRL1_LRADC_IRQ(n: u32) -> u32 { 1 << n }
pub const LRADC_CTRL1_MX28_LRADC_IRQ_MASK: u32 = 0x1fff;
pub const LRADC_CTRL1_MX23_LRADC_IRQ_MASK: u32 = 0x01ff;
pub const LRADC_CTRL1_LRADC_IRQ_OFFSET: u32 = 0;

pub const LRADC_CTRL2: u32 = 0x20;
pub const LRADC_CTRL2_DIVIDE_BY_TWO_OFFSET: u32 = 24;
pub const LRADC_CTRL2_TEMPSENSE_PWD: u32 = 1 << 15;
pub const LRADC_STATUS: u32 = 0x40;
pub const LRADC_STATUS_TOUCH_DETECT_RAW: u32 = 1;

#[inline] pub const fn LRADC_CH(n: u32) -> u32 { 0x50 + 0x10 * n }
pub const LRADC_CH_ACCUMULATE: u32 = 1 << 29;
pub const LRADC_CH_NUM_SAMPLES_MASK: u32 = 0x1f << 24;
pub const LRADC_CH_NUM_SAMPLES_OFFSET: u32 = 24;
#[inline] pub const fn LRADC_CH_NUM_SAMPLES(x: u32) -> u32 { x << LRADC_CH_NUM_SAMPLES_OFFSET }
pub const LRADC_CH_VALUE_MASK: u32 = 0x3ffff;
pub const LRADC_CH_VALUE_OFFSET: u32 = 0;

#[inline] pub const fn LRADC_DELAY(n: u32) -> u32 { 0xd0 + 0x10 * n }
pub const LRADC_DELAY_TRIGGER_LRADCS_MASK: u32 = 0xff << 24;
pub const LRADC_DELAY_TRIGGER_LRADCS_OFFSET: u32 = 24;
#[inline] pub const fn LRADC_DELAY_TRIGGER(x: u32) -> u32 { (x << 24) & LRADC_DELAY_TRIGGER_LRADCS_MASK }
pub const LRADC_DELAY_KICK: u32 = 1 << 20;
pub const LRADC_DELAY_TRIGGER_DELAYS_MASK: u32 = 0xf << 16;
pub const LRADC_DELAY_TRIGGER_DELAYS_OFFSET: u32 = 16;
#[inline] pub const fn LRADC_DELAY_TRIGGER_DELAYS(x: u32) -> u32 { (x << 16) & LRADC_DELAY_TRIGGER_DELAYS_MASK }
pub const LRADC_DELAY_LOOP_COUNT_MASK: u32 = 0x1f << 11;
pub const LRADC_DELAY_LOOP_COUNT_OFFSET: u32 = 11;
#[inline] pub const fn LRADC_DELAY_LOOP(x: u32) -> u32 { (x << 11) & LRADC_DELAY_LOOP_COUNT_MASK }
pub const LRADC_DELAY_DELAY_MASK: u32 = 0x7ff;
pub const LRADC_DELAY_DELAY_OFFSET: u32 = 0;
#[inline] pub const fn LRADC_DELAY_DELAY(x: u32) -> u32 { x & LRADC_DELAY_DELAY_MASK }

pub const LRADC_CTRL4: u32 = 0x140;
#[inline] pub const fn LRADC_CTRL4_LRADCSELECT_MASK(n: u32) -> u32 { 0xf << (n * 4) }
#[inline] pub const fn LRADC_CTRL4_LRADCSELECT_OFFSET(n: u32) -> u32 { n * 4 }
#[inline] pub const fn LRADC_CTRL4_LRADCSELECT(n: u32, x: u32) -> u32 { (x << (n * 4)) & LRADC_CTRL4_LRADCSELECT_MASK(n) }

pub const LRADC_RESOLUTION: u32 = 12;
pub const LRADC_SINGLE_SAMPLE_MASK: u32 = (1 << LRADC_RESOLUTION) - 1;
pub const BUFFER_VCHANS_LIMITED: u32 = 0x3f;
pub const BUFFER_VCHANS_ALL: u32 = 0xff;
pub const CHAN_MASK_TOUCHBUTTON: u32 = (1 << 1) | (1 << 0);
pub const CHAN_MASK_TOUCHSCREEN_4WIRE: u32 = 0xf << 2;
pub const CHAN_MASK_TOUCHSCREEN_5WIRE: u32 = 0x1f << 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mxs_lradc_id { IMX23_LRADC, IMX28_LRADC }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mxs_lradc_ts_wires {
    MXS_LRADC_TOUCHSCREEN_NONE = 0,
    MXS_LRADC_TOUCHSCREEN_4WIRE,
    MXS_LRADC_TOUCHSCREEN_5WIRE,
}

#[repr(C)]
pub struct clk { _private: [u8; 0] }

#[repr(C)]
pub struct mxs_lradc {
    pub soc: mxs_lradc_id,
    pub clk: *mut clk,
    pub buffer_vchans: u8,
    pub touchscreen_wire: mxs_lradc_ts_wires,
    pub use_touchbutton: bool,
}

#[inline]
pub unsafe fn mxs_lradc_irq_mask(lradc: *mut mxs_lradc) -> u32 {
    match (*lradc).soc {
        mxs_lradc_id::IMX23_LRADC => LRADC_CTRL1_MX23_LRADC_IRQ_MASK,
        mxs_lradc_id::IMX28_LRADC => LRADC_CTRL1_MX28_LRADC_IRQ_MASK,
        _ => 0,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
