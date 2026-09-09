/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI Touch Screen / ADC MFD driver
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// C dependencies: linux/bitfield.h, linux/mfd/core.h, linux/units.h

pub const REG_RAWIRQSTATUS: u32 = 0x024;
pub const REG_IRQSTATUS: u32 = 0x028;
pub const REG_IRQENABLE: u32 = 0x02C;
pub const REG_IRQCLR: u32 = 0x030;
pub const REG_IRQWAKEUP: u32 = 0x034;
pub const REG_DMAENABLE_SET: u32 = 0x038;
pub const REG_DMAENABLE_CLEAR: u32 = 0x03c;
pub const REG_CTRL: u32 = 0x040;
pub const REG_ADCFSM: u32 = 0x044;
pub const REG_CLKDIV: u32 = 0x04C;
pub const REG_SE: u32 = 0x054;
pub const REG_IDLECONFIG: u32 = 0x058;
pub const REG_CHARGECONFIG: u32 = 0x05C;
pub const REG_CHARGEDELAY: u32 = 0x060;
pub const REG_FIFO0CNT: u32 = 0xE4;
pub const REG_FIFO0THR: u32 = 0xE8;
pub const REG_FIFO1CNT: u32 = 0xF0;
pub const REG_FIFO1THR: u32 = 0xF4;
pub const REG_DMA1REQ: u32 = 0xF8;
pub const REG_FIFO0: u32 = 0x100;
pub const REG_FIFO1: u32 = 0x200;

pub const fn reg_stepconfig(n: u32) -> u32 { 0x64 + n * 8 }
pub const fn reg_stepdelay(n: u32) -> u32 { 0x68 + n * 8 }

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 { ((1u32 << (high - low + 1)) - 1) << low }
const fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }

/* Register Bitfields */
/* IRQ wakeup enable */
pub const IRQWKUP_ENB: u32 = bit(0);

/* IRQ enable */
pub const IRQENB_HW_PEN: u32 = bit(0);
pub const IRQENB_EOS: u32 = bit(1);
pub const IRQENB_FIFO0THRES: u32 = bit(2);
pub const IRQENB_FIFO0OVRRUN: u32 = bit(3);
pub const IRQENB_FIFO0UNDRFLW: u32 = bit(4);
pub const IRQENB_FIFO1THRES: u32 = bit(5);
pub const IRQENB_FIFO1OVRRUN: u32 = bit(6);
pub const IRQENB_FIFO1UNDRFLW: u32 = bit(7);
pub const IRQENB_PENUP: u32 = bit(9);

/* Step Configuration */
pub const fn stepconfig_mode(val: u32) -> u32 { field_prep(genmask(1, 0), val) }
pub const STEPCONFIG_MODE_SWCNT: u32 = stepconfig_mode(1);
pub const STEPCONFIG_MODE_HWSYNC: u32 = stepconfig_mode(2);
pub const fn stepconfig_avg(val: u32) -> u32 { field_prep(genmask(4, 2), val) }
pub const STEPCONFIG_AVG_16: u32 = stepconfig_avg(4);
pub const STEPCONFIG_XPP: u32 = bit(5);
pub const STEPCONFIG_XNN: u32 = bit(6);
pub const STEPCONFIG_YPP: u32 = bit(7);
pub const STEPCONFIG_YNN: u32 = bit(8);
pub const STEPCONFIG_XNP: u32 = bit(9);
pub const STEPCONFIG_YPN: u32 = bit(10);
pub const fn stepconfig_rfp(val: u32) -> u32 { field_prep(genmask(13, 12), val) }
pub const STEPCONFIG_RFP_VREFP: u32 = stepconfig_rfp(3);
pub const fn stepconfig_inm(val: u32) -> u32 { field_prep(genmask(18, 15), val) }
pub const STEPCONFIG_INM_ADCREFM: u32 = stepconfig_inm(8);
pub const fn stepconfig_inp(val: u32) -> u32 { field_prep(genmask(22, 19), val) }
pub const STEPCONFIG_INP_AN4: u32 = stepconfig_inp(4);
pub const STEPCONFIG_INP_ADCREFM: u32 = stepconfig_inp(8);
pub const STEPCONFIG_FIFO1: u32 = bit(26);
pub const fn stepconfig_rfm(val: u32) -> u32 { field_prep(genmask(24, 23), val) }
pub const STEPCONFIG_RFM_VREFN: u32 = stepconfig_rfm(3);

/* Delay register */
pub const fn stepdelay_open(val: u32) -> u32 { field_prep(genmask(17, 0), val) }
pub const STEPCONFIG_OPENDLY: u32 = stepdelay_open(0x098);
pub const STEPCONFIG_MAX_OPENDLY: u32 = genmask(17, 0);
pub const fn stepdelay_sample(val: u32) -> u32 { field_prep(genmask(31, 24), val) }
pub const STEPCONFIG_SAMPLEDLY: u32 = stepdelay_sample(0);
pub const STEPCONFIG_MAX_SAMPLE: u32 = genmask(7, 0);

/* Charge Config */
pub const fn stepcharge_rfp(val: u32) -> u32 { field_prep(genmask(14, 12), val) }
pub const STEPCHARGE_RFP_XPUL: u32 = stepcharge_rfp(1);
pub const fn stepcharge_inm(val: u32) -> u32 { field_prep(genmask(18, 15), val) }
pub const STEPCHARGE_INM_AN1: u32 = stepcharge_inm(1);
pub const fn stepcharge_inp(val: u32) -> u32 { field_prep(genmask(22, 19), val) }
pub const fn stepcharge_rfm(val: u32) -> u32 { field_prep(genmask(24, 23), val) }
pub const STEPCHARGE_RFM_XNUR: u32 = stepcharge_rfm(1);

/* Charge delay */
pub const fn chargedly_open(val: u32) -> u32 { field_prep(genmask(17, 0), val) }
pub const CHARGEDLY_OPENDLY: u32 = chargedly_open(0x400);

/* Control register */
pub const CNTRLREG_SSENB: u32 = bit(0);
pub const CNTRLREG_STEPID: u32 = bit(1);
pub const CNTRLREG_TSC_STEPCONFIGWRT: u32 = bit(2);
pub const CNTRLREG_POWERDOWN: u32 = bit(4);
pub const fn cntrlreg_tsc_afe_ctrl(val: u32) -> u32 { field_prep(genmask(6, 5), val) }
pub const CNTRLREG_TSC_4WIRE: u32 = cntrlreg_tsc_afe_ctrl(1);
pub const CNTRLREG_TSC_5WIRE: u32 = cntrlreg_tsc_afe_ctrl(2);
pub const CNTRLREG_TSC_ENB: u32 = bit(7);

/*Control registers bitfields  for MAGADC IP */
pub const CNTRLREG_MAGADCENB: u32 = bit(0);
pub const CNTRLREG_MAG_PREAMP_PWRDOWN: u32 = bit(5);
pub const CNTRLREG_MAG_PREAMP_BYPASS: u32 = bit(6);

/* FIFO READ Register */
pub const FIFOREAD_DATA_MASK: u32 = genmask(11, 0);
pub const FIFOREAD_CHNLID_MASK: u32 = genmask(19, 16);

/* DMA ENABLE/CLEAR Register */
pub const DMA_FIFO0: u32 = bit(0);
pub const DMA_FIFO1: u32 = bit(1);

/* Sequencer Status */
pub const SEQ_STATUS: u32 = bit(5);
pub const CHARGE_STEP: u32 = 0x11;

// HZ_PER_MHZ is supplied by the Linux units dependency.
pub const TSC_ADC_CLK: u32 = 3 * HZ_PER_MHZ;
pub const MAG_ADC_CLK: u32 = 13 * HZ_PER_MHZ;
pub const TOTAL_STEPS: u32 = 16;
pub const TOTAL_CHANNELS: u32 = 8;
pub const FIFO1_THRESHOLD: u32 = 19;

/*
 * time in us for processing a single channel, calculated as follows:
 *
 * max num cycles = open delay + (sample delay + conv time) * averaging
 *
 * max num cycles: 262143 + (255 + 13) * 16 = 266431
 *
 * clock frequency: 26MHz / 8 = 3.25MHz
 * clock period: 1 / 3.25MHz = 308ns
 *
 * max processing time: 266431 * 308ns = 83ms(approx)
 */
pub const IDLE_TIMEOUT_MS: u32 = 83; /* milliseconds */
pub const TSCADC_CELLS: usize = 2;

#[repr(C)]
pub struct ti_tscadc_data {
    pub adc_feature_name: *mut i8,
    pub adc_feature_compatible: *mut i8,
    pub secondary_feature_name: *mut i8,
    pub secondary_feature_compatible: *mut i8,
    pub target_clk_rate: u32,
}

#[repr(C)]
pub struct ti_tscadc_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub tscadc_base: *mut core::ffi::c_void,
    pub tscadc_phys_base: phys_addr_t,
    pub data: *const ti_tscadc_data,
    pub irq: i32,
    pub cells: [mfd_cell; TSCADC_CELLS],
    pub ctrl: u32,
    pub reg_se_cache: u32,
    pub adc_waiting: bool,
    pub adc_in_use: bool,
    pub reg_se_wait: wait_queue_head_t,
    pub reg_lock: spinlock_t,
    pub clk_div: u32,
    /* tsc device */
    pub tsc: *mut titsc,
    /* adc device */
    pub adc: *mut adc_device,
}

pub unsafe fn ti_tscadc_dev_get(p: *mut platform_device) -> *mut ti_tscadc_dev {
    let tscadc_dev = (*p).dev.platform_data as *mut *mut ti_tscadc_dev;
    *tscadc_dev
}

pub unsafe fn ti_adc_with_touchscreen(tscadc: *mut ti_tscadc_dev) -> bool {
    of_device_is_compatible((*tscadc).dev.as_ref().unwrap().of_node,
                            b"ti,am3359-tscadc\0".as_ptr() as *const i8)
}

extern "C" {
    pub fn am335x_tsc_se_set_cache(tsadc: *mut ti_tscadc_dev, val: u32);
    pub fn am335x_tsc_se_set_once(tsadc: *mut ti_tscadc_dev, val: u32);
    pub fn am335x_tsc_se_clr(tsadc: *mut ti_tscadc_dev, val: u32);
    pub fn am335x_tsc_se_adc_done(tsadc: *mut ti_tscadc_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
