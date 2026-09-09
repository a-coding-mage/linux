/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Definitions for CS4231 & InterWave chips & compatible chips registers
 */

/* IO ports */

/* C token-pasting macro; Rust callers provide the corresponding identifier. */
macro_rules! CS4231P { ($x:ident) => { c_d_c_CS4231$x }; }

pub const c_d_c_CS4231REGSEL: u8 = 0;
pub const c_d_c_CS4231REG: u8 = 1;
pub const c_d_c_CS4231STATUS: u8 = 2;
pub const c_d_c_CS4231PIO: u8 = 3;

/* codec registers */
pub const CS4231_LEFT_INPUT: u8 = 0x00;
pub const CS4231_RIGHT_INPUT: u8 = 0x01;
pub const CS4231_AUX1_LEFT_INPUT: u8 = 0x02;
pub const CS4231_AUX1_RIGHT_INPUT: u8 = 0x03;
pub const CS4231_AUX2_LEFT_INPUT: u8 = 0x04;
pub const CS4231_AUX2_RIGHT_INPUT: u8 = 0x05;
pub const CS4231_LEFT_OUTPUT: u8 = 0x06;
pub const CS4231_RIGHT_OUTPUT: u8 = 0x07;
pub const CS4231_PLAYBK_FORMAT: u8 = 0x08;
pub const CS4231_IFACE_CTRL: u8 = 0x09;
pub const CS4231_PIN_CTRL: u8 = 0x0a;
pub const CS4231_TEST_INIT: u8 = 0x0b;
pub const CS4231_MISC_INFO: u8 = 0x0c;
pub const CS4231_LOOPBACK: u8 = 0x0d;
pub const CS4231_PLY_UPR_CNT: u8 = 0x0e;
pub const CS4231_PLY_LWR_CNT: u8 = 0x0f;
pub const CS4231_ALT_FEATURE_1: u8 = 0x10;
pub const AD1845_AF1_MIC_LEFT: u8 = 0x10;
pub const CS4231_ALT_FEATURE_2: u8 = 0x11;
pub const AD1845_AF2_MIC_RIGHT: u8 = 0x11;
pub const CS4231_LEFT_LINE_IN: u8 = 0x12;
pub const CS4231_RIGHT_LINE_IN: u8 = 0x13;
pub const CS4231_TIMER_LOW: u8 = 0x14;
pub const CS4231_TIMER_HIGH: u8 = 0x15;
pub const CS4231_LEFT_MIC_INPUT: u8 = 0x16;
pub const AD1845_UPR_FREQ_SEL: u8 = 0x16;
pub const CS4231_RIGHT_MIC_INPUT: u8 = 0x17;
pub const AD1845_LWR_FREQ_SEL: u8 = 0x17;
pub const CS4236_EXT_REG: u8 = 0x17;
pub const CS4231_IRQ_STATUS: u8 = 0x18;
pub const CS4231_LINE_LEFT_OUTPUT: u8 = 0x19;
pub const CS4231_VERSION: u8 = 0x19;
pub const CS4231_MONO_CTRL: u8 = 0x1a;
pub const CS4231_LINE_RIGHT_OUTPUT: u8 = 0x1b;
pub const AD1845_PWR_DOWN: u8 = 0x1b;
pub const CS4235_LEFT_MASTER: u8 = 0x1b;
pub const CS4231_REC_FORMAT: u8 = 0x1c;
pub const AD1845_CLOCK: u8 = 0x1d;
pub const CS4235_RIGHT_MASTER: u8 = 0x1d;
pub const CS4231_REC_UPR_CNT: u8 = 0x1e;
pub const CS4231_REC_LWR_CNT: u8 = 0x1f;

/* definitions for codec register select port - CODECP( REGSEL ) */
pub const CS4231_INIT: u8 = 0x80;
pub const CS4231_MCE: u8 = 0x40;
pub const CS4231_TRD: u8 = 0x20;
/* definitions for codec status register - CODECP( STATUS ) */
pub const CS4231_GLOBALIRQ: u8 = 0x01;
/* definitions for codec irq status */
pub const CS4231_PLAYBACK_IRQ: u8 = 0x10;
pub const CS4231_RECORD_IRQ: u8 = 0x20;
pub const CS4231_TIMER_IRQ: u8 = 0x40;
pub const CS4231_ALL_IRQS: u8 = 0x70;
pub const CS4231_REC_UNDERRUN: u8 = 0x08;
pub const CS4231_REC_OVERRUN: u8 = 0x04;
pub const CS4231_PLY_OVERRUN: u8 = 0x02;
pub const CS4231_PLY_UNDERRUN: u8 = 0x01;

pub const CS4231_ENABLE_MIC_GAIN: u8 = 0x20;
pub const CS4231_MIXS_LINE: u8 = 0x00;
pub const CS4231_MIXS_AUX1: u8 = 0x40;
pub const CS4231_MIXS_MIC: u8 = 0x80;
pub const CS4231_MIXS_ALL: u8 = 0xc0;

pub const CS4231_LINEAR_8: u8 = 0x00;
pub const CS4231_ALAW_8: u8 = 0x60;
pub const CS4231_ULAW_8: u8 = 0x20;
pub const CS4231_LINEAR_16: u8 = 0x40;
pub const CS4231_LINEAR_16_BIG: u8 = 0xc0;
pub const CS4231_ADPCM_16: u8 = 0xa0;
pub const CS4231_STEREO: u8 = 0x10;
pub const CS4231_XTAL1: u8 = 0x00;
pub const CS4231_XTAL2: u8 = 0x01;

pub const CS4231_RECORD_PIO: u8 = 0x80;
pub const CS4231_PLAYBACK_PIO: u8 = 0x40;
pub const CS4231_CALIB_MODE: u8 = 0x18;
pub const CS4231_AUTOCALIB: u8 = 0x08;
pub const CS4231_SINGLE_DMA: u8 = 0x04;
pub const CS4231_RECORD_ENABLE: u8 = 0x02;
pub const CS4231_PLAYBACK_ENABLE: u8 = 0x01;
pub const CS4231_IRQ_ENABLE: u8 = 0x02;
pub const CS4231_XCTL1: u8 = 0x40;
pub const CS4231_XCTL0: u8 = 0x80;
pub const CS4231_CALIB_IN_PROGRESS: u8 = 0x20;
pub const CS4231_DMA_REQUEST: u8 = 0x10;
pub const CS4231_MODE2: u8 = 0x40;
pub const CS4231_IW_MODE3: u8 = 0x6c;
pub const CS4231_4236_MODE3: u8 = 0xe0;
pub const CS4231_DACZ: u8 = 0x01;
pub const CS4231_TIMER_ENABLE: u8 = 0x40;
pub const CS4231_OLB: u8 = 0x80;

#[inline]
pub const fn CS4236_REG(i23val: u8) -> u8 {
    ((i23val << 2) & 0x10) | ((i23val >> 4) & 0x0f)
}

#[inline]
pub const fn CS4236_I23VAL(reg: u8) -> u8 {
    (((reg & 0xf) << 4) | ((reg & 0x10) >> 2) | 0x8)
}

pub const CS4236_LEFT_LINE: u8 = 0x08;
pub const CS4236_RIGHT_LINE: u8 = 0x18;
pub const CS4236_LEFT_MIC: u8 = 0x28;
pub const CS4236_RIGHT_MIC: u8 = 0x38;
pub const CS4236_LEFT_MIX_CTRL: u8 = 0x48;
pub const CS4236_RIGHT_MIX_CTRL: u8 = 0x58;
pub const CS4236_LEFT_FM: u8 = 0x68;
pub const CS4236_RIGHT_FM: u8 = 0x78;
pub const CS4236_LEFT_DSP: u8 = 0x88;
pub const CS4236_RIGHT_DSP: u8 = 0x98;
pub const CS4236_RIGHT_LOOPBACK: u8 = 0xa8;
pub const CS4236_DAC_MUTE: u8 = 0xb8;
pub const CS4236_ADC_RATE: u8 = 0xc8;
pub const CS4236_DAC_RATE: u8 = 0xd8;
pub const CS4236_LEFT_MASTER: u8 = 0xe8;
pub const CS4236_RIGHT_MASTER: u8 = 0xf8;
pub const CS4236_LEFT_WAVE: u8 = 0x0c;
pub const CS4236_RIGHT_WAVE: u8 = 0x1c;
pub const CS4236_VERSION: u8 = 0x9c;

/* definitions for extended registers - OPTI93X */
pub const OPTi931_AUX_LEFT_INPUT: u8 = 0x10;
pub const OPTi931_AUX_RIGHT_INPUT: u8 = 0x11;
pub const OPTi93X_MIC_LEFT_INPUT: u8 = 0x14;
pub const OPTi93X_MIC_RIGHT_INPUT: u8 = 0x15;
pub const OPTi93X_OUT_LEFT: u8 = 0x16;
pub const OPTi93X_OUT_RIGHT: u8 = 0x17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
