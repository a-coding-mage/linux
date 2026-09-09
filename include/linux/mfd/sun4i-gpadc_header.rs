/* SPDX-License-Identifier: GPL-2.0-only */
/* Header of ADC MFD core driver for sunxi platforms
 *
 * Copyright (c) 2016 Quentin Schulz <quentin.schulz@free-electrons.com>
 */

// C header guard: __SUN4I_GPADC__H__

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 {
    (((1u32 << (high - low + 1)) - 1) << low)
}

pub const SUN4I_GPADC_CTRL0: u32 = 0x00;
pub const fn SUN4I_GPADC_CTRL0_ADC_FIRST_DLY(x: u32) -> u32 { (genmask(7, 0) & x) << 24 }
pub const SUN4I_GPADC_CTRL0_ADC_FIRST_DLY_MODE: u32 = bit(23);
pub const SUN4I_GPADC_CTRL0_ADC_CLK_SELECT: u32 = bit(22);
pub const fn SUN4I_GPADC_CTRL0_ADC_CLK_DIVIDER(x: u32) -> u32 { (genmask(1, 0) & x) << 20 }
pub const fn SUN4I_GPADC_CTRL0_FS_DIV(x: u32) -> u32 { (genmask(3, 0) & x) << 16 }
pub const fn SUN4I_GPADC_CTRL0_T_ACQ(x: u32) -> u32 { genmask(15, 0) & x }

pub const SUN4I_GPADC_CTRL1: u32 = 0x04;
pub const fn SUN4I_GPADC_CTRL1_STYLUS_UP_DEBOUNCE(x: u32) -> u32 { (genmask(7, 0) & x) << 12 }
pub const SUN4I_GPADC_CTRL1_STYLUS_UP_DEBOUNCE_EN: u32 = bit(9);
pub const SUN4I_GPADC_CTRL1_TOUCH_PAN_CALI_EN: u32 = bit(6);
pub const SUN4I_GPADC_CTRL1_TP_DUAL_EN: u32 = bit(5);
pub const SUN4I_GPADC_CTRL1_TP_MODE_EN: u32 = bit(4);
pub const SUN4I_GPADC_CTRL1_TP_ADC_SELECT: u32 = bit(3);
pub const fn SUN4I_GPADC_CTRL1_ADC_CHAN_SELECT(x: u32) -> u32 { genmask(2, 0) & x }
pub const SUN4I_GPADC_CTRL1_ADC_CHAN_MASK: u32 = genmask(2, 0);

/* TP_CTRL1 bits for sun6i SOCs */
pub const SUN6I_GPADC_CTRL1_TOUCH_PAN_CALI_EN: u32 = bit(7);
pub const SUN6I_GPADC_CTRL1_TP_DUAL_EN: u32 = bit(6);
pub const SUN6I_GPADC_CTRL1_TP_MODE_EN: u32 = bit(5);
pub const SUN6I_GPADC_CTRL1_TP_ADC_SELECT: u32 = bit(4);
pub const fn SUN6I_GPADC_CTRL1_ADC_CHAN_SELECT(x: u32) -> u32 { genmask(3, 0) & bit(x) }
pub const SUN6I_GPADC_CTRL1_ADC_CHAN_MASK: u32 = genmask(3, 0);

/* TP_CTRL1 bits for sun8i SoCs */
pub const SUN8I_GPADC_CTRL1_CHOP_TEMP_EN: u32 = bit(8);
pub const SUN8I_GPADC_CTRL1_GPADC_CALI_EN: u32 = bit(7);

pub const SUN4I_GPADC_CTRL2: u32 = 0x08;
pub const fn SUN4I_GPADC_CTRL2_TP_SENSITIVE_ADJUST(x: u32) -> u32 { (genmask(3, 0) & x) << 28 }
pub const fn SUN4I_GPADC_CTRL2_TP_MODE_SELECT(x: u32) -> u32 { (genmask(1, 0) & x) << 26 }
pub const SUN4I_GPADC_CTRL2_PRE_MEA_EN: u32 = bit(24);
pub const fn SUN4I_GPADC_CTRL2_PRE_MEA_THRE_CNT(x: u32) -> u32 { genmask(23, 0) & x }

pub const SUN4I_GPADC_CTRL3: u32 = 0x0c;
pub const SUN4I_GPADC_CTRL3_FILTER_EN: u32 = bit(2);
pub const fn SUN4I_GPADC_CTRL3_FILTER_TYPE(x: u32) -> u32 { genmask(1, 0) & x }

pub const SUN4I_GPADC_TPR: u32 = 0x18;
pub const SUN4I_GPADC_TPR_TEMP_ENABLE: u32 = bit(16);
pub const fn SUN4I_GPADC_TPR_TEMP_PERIOD(x: u32) -> u32 { genmask(15, 0) & x }

pub const SUN4I_GPADC_INT_FIFOC: u32 = 0x10;
pub const SUN4I_GPADC_INT_FIFOC_TEMP_IRQ_EN: u32 = bit(18);
pub const SUN4I_GPADC_INT_FIFOC_TP_OVERRUN_IRQ_EN: u32 = bit(17);
pub const SUN4I_GPADC_INT_FIFOC_TP_DATA_IRQ_EN: u32 = bit(16);
pub const SUN4I_GPADC_INT_FIFOC_TP_DATA_XY_CHANGE: u32 = bit(13);
pub const fn SUN4I_GPADC_INT_FIFOC_TP_FIFO_TRIG_LEVEL(x: u32) -> u32 { (genmask(4, 0) & x) << 8 }
pub const SUN4I_GPADC_INT_FIFOC_TP_DATA_DRQ_EN: u32 = bit(7);
pub const SUN4I_GPADC_INT_FIFOC_TP_FIFO_FLUSH: u32 = bit(4);
pub const SUN4I_GPADC_INT_FIFOC_TP_UP_IRQ_EN: u32 = bit(1);
pub const SUN4I_GPADC_INT_FIFOC_TP_DOWN_IRQ_EN: u32 = bit(0);

pub const SUN4I_GPADC_INT_FIFOS: u32 = 0x14;
pub const SUN4I_GPADC_INT_FIFOS_TEMP_DATA_PENDING: u32 = bit(18);
pub const SUN4I_GPADC_INT_FIFOS_FIFO_OVERRUN_PENDING: u32 = bit(17);
pub const SUN4I_GPADC_INT_FIFOS_FIFO_DATA_PENDING: u32 = bit(16);
pub const SUN4I_GPADC_INT_FIFOS_TP_IDLE_FLG: u32 = bit(2);
pub const SUN4I_GPADC_INT_FIFOS_TP_UP_PENDING: u32 = bit(1);
pub const SUN4I_GPADC_INT_FIFOS_TP_DOWN_PENDING: u32 = bit(0);

pub const SUN4I_GPADC_CDAT: u32 = 0x1c;
pub const SUN4I_GPADC_TEMP_DATA: u32 = 0x20;
pub const SUN4I_GPADC_DATA: u32 = 0x24;
pub const SUN4I_GPADC_IRQ_FIFO_DATA: u32 = 1;
pub const SUN4I_GPADC_IRQ_TEMP_DATA: u32 = 2;

/* 10s delay before suspending the IP */
pub const SUN4I_GPADC_AUTOSUSPEND_DELAY: u32 = 10000;

#[repr(C)]
pub struct sun4i_gpadc_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irqc: *mut regmap_irq_chip_data,
    pub base: *mut core::ffi::c_void,
}

// External C types supplied by other headers.
pub enum device {}
pub enum regmap {}
pub enum regmap_irq_chip_data {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
