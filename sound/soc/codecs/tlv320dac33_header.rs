/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC Texas Instruments TLV320DAC33 codec driver
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * Copyright:   (C) 2009 Nokia Corporation
 */

pub const DAC33_PAGE_SELECT: u32 = 0x00;
pub const DAC33_PWR_CTRL: u32 = 0x01;
pub const DAC33_PLL_CTRL_A: u32 = 0x02;
pub const DAC33_PLL_CTRL_B: u32 = 0x03;
pub const DAC33_PLL_CTRL_C: u32 = 0x04;
pub const DAC33_PLL_CTRL_D: u32 = 0x05;
pub const DAC33_PLL_CTRL_E: u32 = 0x06;
pub const DAC33_INT_OSC_CTRL: u32 = 0x07;
pub const DAC33_INT_OSC_FREQ_RAT_A: u32 = 0x08;
pub const DAC33_INT_OSC_FREQ_RAT_B: u32 = 0x09;
pub const DAC33_INT_OSC_DAC_RATIO_SET: u32 = 0x0A;
pub const DAC33_CALIB_TIME: u32 = 0x0B;
pub const DAC33_INT_OSC_CTRL_B: u32 = 0x0C;
pub const DAC33_INT_OSC_CTRL_C: u32 = 0x0D;
pub const DAC33_INT_OSC_STATUS: u32 = 0x0E;
pub const DAC33_INT_OSC_DAC_RATIO_READ: u32 = 0x0F;
pub const DAC33_INT_OSC_FREQ_RAT_READ_A: u32 = 0x10;
pub const DAC33_INT_OSC_FREQ_RAT_READ_B: u32 = 0x11;
pub const DAC33_SER_AUDIOIF_CTRL_A: u32 = 0x12;
pub const DAC33_SER_AUDIOIF_CTRL_B: u32 = 0x13;
pub const DAC33_SER_AUDIOIF_CTRL_C: u32 = 0x14;
pub const DAC33_FIFO_CTRL_A: u32 = 0x15;
pub const DAC33_UTHR_MSB: u32 = 0x16;
pub const DAC33_UTHR_LSB: u32 = 0x17;
pub const DAC33_ATHR_MSB: u32 = 0x18;
pub const DAC33_ATHR_LSB: u32 = 0x19;
pub const DAC33_LTHR_MSB: u32 = 0x1A;
pub const DAC33_LTHR_LSB: u32 = 0x1B;
pub const DAC33_PREFILL_MSB: u32 = 0x1C;
pub const DAC33_PREFILL_LSB: u32 = 0x1D;
pub const DAC33_NSAMPLE_MSB: u32 = 0x1E;
pub const DAC33_NSAMPLE_LSB: u32 = 0x1F;
pub const DAC33_FIFO_WPTR_MSB: u32 = 0x20;
pub const DAC33_FIFO_WPTR_LSB: u32 = 0x21;
pub const DAC33_FIFO_RPTR_MSB: u32 = 0x22;
pub const DAC33_FIFO_RPTR_LSB: u32 = 0x23;
pub const DAC33_FIFO_DEPTH_MSB: u32 = 0x24;
pub const DAC33_FIFO_DEPTH_LSB: u32 = 0x25;
pub const DAC33_SAMPLES_REMAINING_MSB: u32 = 0x26;
pub const DAC33_SAMPLES_REMAINING_LSB: u32 = 0x27;
pub const DAC33_FIFO_IRQ_FLAG: u32 = 0x28;
pub const DAC33_FIFO_IRQ_MASK: u32 = 0x29;
pub const DAC33_FIFO_IRQ_MODE_A: u32 = 0x2A;
pub const DAC33_FIFO_IRQ_MODE_B: u32 = 0x2B;
pub const DAC33_DAC_CTRL_A: u32 = 0x2C;
pub const DAC33_DAC_CTRL_B: u32 = 0x2D;
pub const DAC33_DAC_CTRL_C: u32 = 0x2E;
pub const DAC33_LDAC_DIG_VOL_CTRL: u32 = 0x2F;
pub const DAC33_RDAC_DIG_VOL_CTRL: u32 = 0x30;
pub const DAC33_DAC_STATUS_FLAGS: u32 = 0x31;
pub const DAC33_ASRC_CTRL_A: u32 = 0x32;
pub const DAC33_ASRC_CTRL_B: u32 = 0x33;
pub const DAC33_SRC_REF_CLK_RATIO_A: u32 = 0x34;
pub const DAC33_SRC_REF_CLK_RATIO_B: u32 = 0x35;
pub const DAC33_SRC_EST_REF_CLK_RATIO_A: u32 = 0x36;
pub const DAC33_SRC_EST_REF_CLK_RATIO_B: u32 = 0x37;
pub const DAC33_INTP_CTRL_A: u32 = 0x38;
pub const DAC33_INTP_CTRL_B: u32 = 0x39;
/* Registers 0x3A - 0x3F Reserved */
pub const DAC33_LDAC_PWR_CTRL: u32 = 0x40;
pub const DAC33_RDAC_PWR_CTRL: u32 = 0x41;
pub const DAC33_OUT_AMP_CM_CTRL: u32 = 0x42;
pub const DAC33_OUT_AMP_PWR_CTRL: u32 = 0x43;
pub const DAC33_OUT_AMP_CTRL: u32 = 0x44;
pub const DAC33_LINEL_TO_LLO_VOL: u32 = 0x45;
/* Registers 0x45 - 0x47 Reserved */
pub const DAC33_LINER_TO_RLO_VOL: u32 = 0x48;
pub const DAC33_ANA_VOL_SOFT_STEP_CTRL: u32 = 0x49;
pub const DAC33_OSC_TRIM: u32 = 0x4A;
/* Registers 0x4B - 0x7C Reserved */
pub const DAC33_DEVICE_ID_MSB: u32 = 0x7D;
pub const DAC33_DEVICE_ID_LSB: u32 = 0x7E;
pub const DAC33_DEVICE_REV_ID: u32 = 0x7F;

pub const DAC33_CACHEREGNUM: u32 = 128;

/* Bit definitions */

/* DAC33_PWR_CTRL (0x01) */
pub const DAC33_DACRPDNB: u32 = 0x01 << 0;
pub const DAC33_DACLPDNB: u32 = 0x01 << 1;
pub const DAC33_OSCPDNB: u32 = 0x01 << 2;
pub const DAC33_PLLPDNB: u32 = 0x01 << 3;
pub const DAC33_PDNALLB: u32 = 0x01 << 4;
pub const DAC33_SOFT_RESET: u32 = 0x01 << 7;

/* DAC33_INT_OSC_CTRL (0x07) */
pub const DAC33_REFSEL: u32 = 0x01 << 1;

/* DAC33_INT_OSC_CTRL_B (0x0C) */
pub const fn DAC33_ADJSTEP(x: u32) -> u32 {
    x << 0
}
pub const fn DAC33_ADJTHRSHLD(x: u32) -> u32 {
    x << 4
}

/* DAC33_INT_OSC_CTRL_C (0x0D) */
pub const fn DAC33_REFDIV(x: u32) -> u32 {
    x << 4
}

/* DAC33_INT_OSC_STATUS (0x0E) */
pub const DAC33_OSCSTATUS_IDLE_CALIB: u32 = 0x00;
pub const DAC33_OSCSTATUS_NORMAL: u32 = 0x01;
pub const DAC33_OSCSTATUS_ADJUSTMENT: u32 = 0x03;
pub const DAC33_OSCSTATUS_NOT_USED: u32 = 0x02;

/* DAC33_SER_AUDIOIF_CTRL_A (0x12) */
pub const DAC33_MSWCLK: u32 = 0x01 << 0;
pub const DAC33_MSBCLK: u32 = 0x01 << 1;
pub const DAC33_AFMT_MASK: u32 = 0x03 << 2;
pub const DAC33_AFMT_I2S: u32 = 0x00 << 2;
pub const DAC33_AFMT_DSP: u32 = 0x01 << 2;
pub const DAC33_AFMT_RIGHT_J: u32 = 0x02 << 2;
pub const DAC33_AFMT_LEFT_J: u32 = 0x03 << 2;
pub const DAC33_WLEN_MASK: u32 = 0x03 << 4;
pub const DAC33_WLEN_16: u32 = 0x00 << 4;
pub const DAC33_WLEN_20: u32 = 0x01 << 4;
pub const DAC33_WLEN_24: u32 = 0x02 << 4;
pub const DAC33_WLEN_32: u32 = 0x03 << 4;
pub const DAC33_NCYCL_MASK: u32 = 0x03 << 6;
pub const DAC33_NCYCL_16: u32 = 0x00 << 6;
pub const DAC33_NCYCL_20: u32 = 0x01 << 6;
pub const DAC33_NCYCL_24: u32 = 0x02 << 6;
pub const DAC33_NCYCL_32: u32 = 0x03 << 6;

/* DAC33_SER_AUDIOIF_CTRL_B (0x13) */
pub const DAC33_DATA_DELAY_MASK: u32 = 0x03 << 2;
pub const fn DAC33_DATA_DELAY(x: u32) -> u32 {
    x << 2
}
pub const DAC33_BCLKON: u32 = 0x01 << 5;

/* DAC33_FIFO_CTRL_A (0x15) */
pub const DAC33_WIDTH: u32 = 0x01 << 0;
pub const DAC33_FBYPAS: u32 = 0x01 << 1;
pub const DAC33_FAUTO: u32 = 0x01 << 2;
pub const DAC33_FIFOFLUSH: u32 = 0x01 << 3;

/*
 * UTHR, ATHR, LTHR, PREFILL, NSAMPLE (0x16 - 0x1F)
 * 13-bit values
*/
pub const fn DAC33_THRREG(x: u32) -> u32 {
    ((x) & 0x1FFF) << 3
}

/* DAC33_FIFO_IRQ_MASK (0x29) */
pub const DAC33_MNS: u32 = 0x01 << 0;
pub const DAC33_MPS: u32 = 0x01 << 1;
pub const DAC33_MAT: u32 = 0x01 << 2;
pub const DAC33_MLT: u32 = 0x01 << 3;
pub const DAC33_MUT: u32 = 0x01 << 4;
pub const DAC33_MUF: u32 = 0x01 << 5;
pub const DAC33_MOF: u32 = 0x01 << 6;

pub const DAC33_FIFO_IRQ_MODE_MASK: u32 = 0x03;
pub const DAC33_FIFO_IRQ_MODE_RISING: u32 = 0x00;
pub const DAC33_FIFO_IRQ_MODE_FALLING: u32 = 0x01;
pub const DAC33_FIFO_IRQ_MODE_LEVEL: u32 = 0x02;
pub const DAC33_FIFO_IRQ_MODE_EDGE: u32 = 0x03;

/* DAC33_FIFO_IRQ_MODE_A (0x2A) */
pub const fn DAC33_UTM(x: u32) -> u32 {
    x << 0
}
pub const fn DAC33_UFM(x: u32) -> u32 {
    x << 2
}
pub const fn DAC33_OFM(x: u32) -> u32 {
    x << 4
}

/* DAC33_FIFO_IRQ_MODE_B (0x2B) */
pub const fn DAC33_NSM(x: u32) -> u32 {
    x << 0
}
pub const fn DAC33_PSM(x: u32) -> u32 {
    x << 2
}
pub const fn DAC33_ATM(x: u32) -> u32 {
    x << 4
}
pub const fn DAC33_LTM(x: u32) -> u32 {
    x << 6
}

/* DAC33_DAC_CTRL_A (0x2C) */
pub const fn DAC33_DACRATE(x: u32) -> u32 {
    x << 0
}
pub const DAC33_DACDUAL: u32 = 0x01 << 4;
pub const DAC33_DACLKSEL_MASK: u32 = 0x03 << 5;
pub const DAC33_DACLKSEL_INTSOC: u32 = 0x00 << 5;
pub const DAC33_DACLKSEL_PLL: u32 = 0x01 << 5;
pub const DAC33_DACLKSEL_MCLK: u32 = 0x02 << 5;
pub const DAC33_DACLKSEL_BCLK: u32 = 0x03 << 5;

/* DAC33_DAC_CTRL_B (0x2D) */
pub const DAC33_DACSRCR_MASK: u32 = 0x03 << 0;
pub const DAC33_DACSRCR_MUTE: u32 = 0x00 << 0;
pub const DAC33_DACSRCR_RIGHT: u32 = 0x01 << 0;
pub const DAC33_DACSRCR_LEFT: u32 = 0x02 << 0;
pub const DAC33_DACSRCR_MONOMIX: u32 = 0x03 << 0;
pub const DAC33_DACSRCL_MASK: u32 = 0x03 << 2;
pub const DAC33_DACSRCL_MUTE: u32 = 0x00 << 2;
pub const DAC33_DACSRCL_LEFT: u32 = 0x01 << 2;
pub const DAC33_DACSRCL_RIGHT: u32 = 0x02 << 2;
pub const DAC33_DACSRCL_MONOMIX: u32 = 0x03 << 2;
pub const DAC33_DVOLSTEP_MASK: u32 = 0x03 << 4;
pub const DAC33_DVOLSTEP_SS_PERFS: u32 = 0x00 << 4;
pub const DAC33_DVOLSTEP_SS_PER2FS: u32 = 0x01 << 4;
pub const DAC33_DVOLSTEP_SS_DISABLED: u32 = 0x02 << 4;
pub const DAC33_DVOLCTRL_MASK: u32 = 0x03 << 6;
pub const DAC33_DVOLCTRL_LR_INDEPENDENT1: u32 = 0x00 << 6;
pub const DAC33_DVOLCTRL_LR_RIGHT_CONTROL: u32 = 0x01 << 6;
pub const DAC33_DVOLCTRL_LR_LEFT_CONTROL: u32 = 0x02 << 6;
pub const DAC33_DVOLCTRL_LR_INDEPENDENT2: u32 = 0x03 << 6;

/* DAC33_DAC_CTRL_C (0x2E) */
pub const DAC33_DEEMENR: u32 = 0x01 << 0;
pub const DAC33_EFFENR: u32 = 0x01 << 1;
pub const DAC33_DEEMENL: u32 = 0x01 << 2;
pub const DAC33_EFFENL: u32 = 0x01 << 3;
pub const DAC33_EN3D: u32 = 0x01 << 4;
pub const DAC33_RESYNMUTE: u32 = 0x01 << 5;
pub const DAC33_RESYNEN: u32 = 0x01 << 6;

/* DAC33_ASRC_CTRL_A (0x32) */
pub const DAC33_SRCBYP: u32 = 0x01 << 0;
pub const DAC33_SRCLKSEL_MASK: u32 = 0x03 << 1;
pub const DAC33_SRCLKSEL_INTSOC: u32 = 0x00 << 1;
pub const DAC33_SRCLKSEL_PLL: u32 = 0x01 << 1;
pub const DAC33_SRCLKSEL_MCLK: u32 = 0x02 << 1;
pub const DAC33_SRCLKSEL_BCLK: u32 = 0x03 << 1;
pub const fn DAC33_SRCLKDIV(x: u32) -> u32 {
    x << 3
}

/* DAC33_ASRC_CTRL_B (0x33) */
pub const fn DAC33_SRCSETUP(x: u32) -> u32 {
    x << 0
}
pub const DAC33_SRCREFSEL: u32 = 0x01 << 4;
pub const fn DAC33_SRCREFDIV(x: u32) -> u32 {
    x << 5
}

/* DAC33_INTP_CTRL_A (0x38) */
pub const DAC33_INTPSEL: u32 = 0x01 << 0;
pub const DAC33_INTPM_MASK: u32 = 0x03 << 1;
pub const DAC33_INTPM_ALOW_OPENDRAIN: u32 = 0x00 << 1;
pub const DAC33_INTPM_ALOW: u32 = 0x01 << 1;
pub const DAC33_INTPM_AHIGH: u32 = 0x02 << 1;

/* DAC33_LDAC_PWR_CTRL (0x40) */
/* DAC33_RDAC_PWR_CTRL (0x41) */
pub const DAC33_DACLRNUM: u32 = 0x01 << 2;
pub const fn DAC33_LROUT_GAIN(x: u32) -> u32 {
    x << 0
}

/* DAC33_ANA_VOL_SOFT_STEP_CTRL (0x49) */
pub const DAC33_VOLCLKSEL: u32 = 0x01 << 0;
pub const DAC33_VOLCLKEN: u32 = 0x01 << 1;
pub const DAC33_VOLBYPASS: u32 = 0x01 << 2;

pub const TLV320DAC33_MCLK: u32 = 0;
pub const TLV320DAC33_SLEEPCLK: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
