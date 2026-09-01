/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Driver for the PCM512x CODECs
 *
 * Author:	Mark Brown <broonie@kernel.org>
 *		Copyright 2014 Linaro Ltd
 */

/* Dependencies from C header includes: <linux/pm.h>, <linux/regmap.h>. */

pub const PCM512x_VIRT_BASE: u32 = 0x100;
pub const PCM512x_PAGE_LEN: u32 = 0x100;
pub const fn PCM512x_PAGE_BASE(n: u32) -> u32 {
    PCM512x_VIRT_BASE + (PCM512x_PAGE_LEN * n)
}

pub const PCM512x_PAGE: u32 = 0;

pub const PCM512x_RESET: u32 = PCM512x_PAGE_BASE(0) + 1;
pub const PCM512x_POWER: u32 = PCM512x_PAGE_BASE(0) + 2;
pub const PCM512x_MUTE: u32 = PCM512x_PAGE_BASE(0) + 3;
pub const PCM512x_PLL_EN: u32 = PCM512x_PAGE_BASE(0) + 4;
pub const PCM512x_SPI_MISO_FUNCTION: u32 = PCM512x_PAGE_BASE(0) + 6;
pub const PCM512x_DSP: u32 = PCM512x_PAGE_BASE(0) + 7;
pub const PCM512x_GPIO_EN: u32 = PCM512x_PAGE_BASE(0) + 8;
pub const PCM512x_BCLK_LRCLK_CFG: u32 = PCM512x_PAGE_BASE(0) + 9;
pub const PCM512x_DSP_GPIO_INPUT: u32 = PCM512x_PAGE_BASE(0) + 10;
pub const PCM512x_MASTER_MODE: u32 = PCM512x_PAGE_BASE(0) + 12;
pub const PCM512x_PLL_REF: u32 = PCM512x_PAGE_BASE(0) + 13;
pub const PCM512x_DAC_REF: u32 = PCM512x_PAGE_BASE(0) + 14;
pub const PCM512x_GPIO_DACIN: u32 = PCM512x_PAGE_BASE(0) + 16;
pub const PCM512x_GPIO_PLLIN: u32 = PCM512x_PAGE_BASE(0) + 18;
pub const PCM512x_SYNCHRONIZE: u32 = PCM512x_PAGE_BASE(0) + 19;
pub const PCM512x_PLL_COEFF_0: u32 = PCM512x_PAGE_BASE(0) + 20;
pub const PCM512x_PLL_COEFF_1: u32 = PCM512x_PAGE_BASE(0) + 21;
pub const PCM512x_PLL_COEFF_2: u32 = PCM512x_PAGE_BASE(0) + 22;
pub const PCM512x_PLL_COEFF_3: u32 = PCM512x_PAGE_BASE(0) + 23;
pub const PCM512x_PLL_COEFF_4: u32 = PCM512x_PAGE_BASE(0) + 24;
pub const PCM512x_DSP_CLKDIV: u32 = PCM512x_PAGE_BASE(0) + 27;
pub const PCM512x_DAC_CLKDIV: u32 = PCM512x_PAGE_BASE(0) + 28;
pub const PCM512x_NCP_CLKDIV: u32 = PCM512x_PAGE_BASE(0) + 29;
pub const PCM512x_OSR_CLKDIV: u32 = PCM512x_PAGE_BASE(0) + 30;
pub const PCM512x_MASTER_CLKDIV_1: u32 = PCM512x_PAGE_BASE(0) + 32;
pub const PCM512x_MASTER_CLKDIV_2: u32 = PCM512x_PAGE_BASE(0) + 33;
pub const PCM512x_FS_SPEED_MODE: u32 = PCM512x_PAGE_BASE(0) + 34;
pub const PCM512x_IDAC_1: u32 = PCM512x_PAGE_BASE(0) + 35;
pub const PCM512x_IDAC_2: u32 = PCM512x_PAGE_BASE(0) + 36;
pub const PCM512x_ERROR_DETECT: u32 = PCM512x_PAGE_BASE(0) + 37;
pub const PCM512x_I2S_1: u32 = PCM512x_PAGE_BASE(0) + 40;
pub const PCM512x_I2S_2: u32 = PCM512x_PAGE_BASE(0) + 41;
pub const PCM512x_DAC_ROUTING: u32 = PCM512x_PAGE_BASE(0) + 42;
pub const PCM512x_DSP_PROGRAM: u32 = PCM512x_PAGE_BASE(0) + 43;
pub const PCM512x_CLKDET: u32 = PCM512x_PAGE_BASE(0) + 44;
pub const PCM512x_AUTO_MUTE: u32 = PCM512x_PAGE_BASE(0) + 59;
pub const PCM512x_DIGITAL_VOLUME_1: u32 = PCM512x_PAGE_BASE(0) + 60;
pub const PCM512x_DIGITAL_VOLUME_2: u32 = PCM512x_PAGE_BASE(0) + 61;
pub const PCM512x_DIGITAL_VOLUME_3: u32 = PCM512x_PAGE_BASE(0) + 62;
pub const PCM512x_DIGITAL_MUTE_1: u32 = PCM512x_PAGE_BASE(0) + 63;
pub const PCM512x_DIGITAL_MUTE_2: u32 = PCM512x_PAGE_BASE(0) + 64;
pub const PCM512x_DIGITAL_MUTE_3: u32 = PCM512x_PAGE_BASE(0) + 65;
pub const PCM512x_GPIO_OUTPUT_1: u32 = PCM512x_PAGE_BASE(0) + 80;
pub const PCM512x_GPIO_OUTPUT_2: u32 = PCM512x_PAGE_BASE(0) + 81;
pub const PCM512x_GPIO_OUTPUT_3: u32 = PCM512x_PAGE_BASE(0) + 82;
pub const PCM512x_GPIO_OUTPUT_4: u32 = PCM512x_PAGE_BASE(0) + 83;
pub const PCM512x_GPIO_OUTPUT_5: u32 = PCM512x_PAGE_BASE(0) + 84;
pub const PCM512x_GPIO_OUTPUT_6: u32 = PCM512x_PAGE_BASE(0) + 85;
pub const PCM512x_GPIO_CONTROL_1: u32 = PCM512x_PAGE_BASE(0) + 86;
pub const PCM512x_GPIO_CONTROL_2: u32 = PCM512x_PAGE_BASE(0) + 87;
pub const PCM512x_OVERFLOW: u32 = PCM512x_PAGE_BASE(0) + 90;
pub const PCM512x_RATE_DET_1: u32 = PCM512x_PAGE_BASE(0) + 91;
pub const PCM512x_RATE_DET_2: u32 = PCM512x_PAGE_BASE(0) + 92;
pub const PCM512x_RATE_DET_3: u32 = PCM512x_PAGE_BASE(0) + 93;
pub const PCM512x_RATE_DET_4: u32 = PCM512x_PAGE_BASE(0) + 94;
pub const PCM512x_CLOCK_STATUS: u32 = PCM512x_PAGE_BASE(0) + 95;
pub const PCM512x_ANALOG_MUTE_DET: u32 = PCM512x_PAGE_BASE(0) + 108;
pub const PCM512x_GPIN: u32 = PCM512x_PAGE_BASE(0) + 119;
pub const PCM512x_DIGITAL_MUTE_DET: u32 = PCM512x_PAGE_BASE(0) + 120;

pub const PCM512x_OUTPUT_AMPLITUDE: u32 = PCM512x_PAGE_BASE(1) + 1;
pub const PCM512x_ANALOG_GAIN_CTRL: u32 = PCM512x_PAGE_BASE(1) + 2;
pub const PCM512x_UNDERVOLTAGE_PROT: u32 = PCM512x_PAGE_BASE(1) + 5;
pub const PCM512x_ANALOG_MUTE_CTRL: u32 = PCM512x_PAGE_BASE(1) + 6;
pub const PCM512x_ANALOG_GAIN_BOOST: u32 = PCM512x_PAGE_BASE(1) + 7;
pub const PCM512x_VCOM_CTRL_1: u32 = PCM512x_PAGE_BASE(1) + 8;
pub const PCM512x_VCOM_CTRL_2: u32 = PCM512x_PAGE_BASE(1) + 9;

pub const PCM512x_CRAM_CTRL: u32 = PCM512x_PAGE_BASE(44) + 1;

pub const PCM512x_FLEX_A: u32 = PCM512x_PAGE_BASE(253) + 63;
pub const PCM512x_FLEX_B: u32 = PCM512x_PAGE_BASE(253) + 64;

pub const PCM512x_MAX_REGISTER: u32 = PCM512x_PAGE_BASE(253) + 64;

/* Page 0, Register 1 - reset */
pub const PCM512x_RSTR: u32 = 1 << 0;
pub const PCM512x_RSTM: u32 = 1 << 4;

/* Page 0, Register 2 - power */
pub const PCM512x_RQPD: u32 = 1 << 0;
pub const PCM512x_RQPD_SHIFT: u32 = 0;
pub const PCM512x_RQST: u32 = 1 << 4;
pub const PCM512x_RQST_SHIFT: u32 = 4;

/* Page 0, Register 3 - mute */
pub const PCM512x_RQMR: u32 = 1 << 0;
pub const PCM512x_RQMR_SHIFT: u32 = 0;
pub const PCM512x_RQML: u32 = 1 << 4;
pub const PCM512x_RQML_SHIFT: u32 = 4;

/* Page 0, Register 4 - PLL */
pub const PCM512x_PLLE: u32 = 1 << 0;
pub const PCM512x_PLLE_SHIFT: u32 = 0;
pub const PCM512x_PLCK: u32 = 1 << 4;
pub const PCM512x_PLCK_SHIFT: u32 = 4;

/* Page 0, Register 7 - DSP */
pub const PCM512x_SDSL: u32 = 1 << 0;
pub const PCM512x_SDSL_SHIFT: u32 = 0;
pub const PCM512x_DEMP: u32 = 1 << 4;
pub const PCM512x_DEMP_SHIFT: u32 = 4;

/* Page 0, Register 8 - GPIO output enable */
pub const PCM512x_G1OE: u32 = 1 << 0;
pub const PCM512x_G2OE: u32 = 1 << 1;
pub const PCM512x_G3OE: u32 = 1 << 2;
pub const PCM512x_G4OE: u32 = 1 << 3;
pub const PCM512x_G5OE: u32 = 1 << 4;
pub const PCM512x_G6OE: u32 = 1 << 5;

/* Page 0, Register 9 - BCK, LRCLK configuration */
pub const PCM512x_LRKO: u32 = 1 << 0;
pub const PCM512x_LRKO_SHIFT: u32 = 0;
pub const PCM512x_BCKO: u32 = 1 << 4;
pub const PCM512x_BCKO_SHIFT: u32 = 4;
pub const PCM512x_BCKP: u32 = 1 << 5;
pub const PCM512x_BCKP_SHIFT: u32 = 5;

/* Page 0, Register 12 - Master mode BCK, LRCLK reset */
pub const PCM512x_RLRK: u32 = 1 << 0;
pub const PCM512x_RLRK_SHIFT: u32 = 0;
pub const PCM512x_RBCK: u32 = 1 << 1;
pub const PCM512x_RBCK_SHIFT: u32 = 1;

/* Page 0, Register 13 - PLL reference */
pub const PCM512x_SREF: u32 = 7 << 4;
pub const PCM512x_SREF_SHIFT: u32 = 4;
pub const PCM512x_SREF_SCK: u32 = 0 << 4;
pub const PCM512x_SREF_BCK: u32 = 1 << 4;
pub const PCM512x_SREF_GPIO: u32 = 3 << 4;

/* Page 0, Register 14 - DAC reference */
pub const PCM512x_SDAC: u32 = 7 << 4;
pub const PCM512x_SDAC_SHIFT: u32 = 4;
pub const PCM512x_SDAC_MCK: u32 = 0 << 4;
pub const PCM512x_SDAC_PLL: u32 = 1 << 4;
pub const PCM512x_SDAC_SCK: u32 = 3 << 4;
pub const PCM512x_SDAC_BCK: u32 = 4 << 4;
pub const PCM512x_SDAC_GPIO: u32 = 5 << 4;

/* Page 0, Register 16, 18 - GPIO source for DAC, PLL */
pub const PCM512x_GREF: u32 = 7 << 0;
pub const PCM512x_GREF_SHIFT: u32 = 0;
pub const PCM512x_GREF_GPIO1: u32 = 0 << 0;
pub const PCM512x_GREF_GPIO2: u32 = 1 << 0;
pub const PCM512x_GREF_GPIO3: u32 = 2 << 0;
pub const PCM512x_GREF_GPIO4: u32 = 3 << 0;
pub const PCM512x_GREF_GPIO5: u32 = 4 << 0;
pub const PCM512x_GREF_GPIO6: u32 = 5 << 0;

/* Page 0, Register 19 - synchronize */
pub const PCM512x_RQSY: u32 = 1 << 0;
pub const PCM512x_RQSY_RESUME: u32 = 0 << 0;
pub const PCM512x_RQSY_HALT: u32 = 1 << 0;

/* Page 0, Register 34 - fs speed mode */
pub const PCM512x_FSSP: u32 = 3 << 0;
pub const PCM512x_FSSP_SHIFT: u32 = 0;
pub const PCM512x_FSSP_48KHZ: u32 = 0 << 0;
pub const PCM512x_FSSP_96KHZ: u32 = 1 << 0;
pub const PCM512x_FSSP_192KHZ: u32 = 2 << 0;
pub const PCM512x_FSSP_384KHZ: u32 = 3 << 0;

/* Page 0, Register 37 - Error detection */
pub const PCM512x_IPLK: u32 = 1 << 0;
pub const PCM512x_DCAS: u32 = 1 << 1;
pub const PCM512x_IDCM: u32 = 1 << 2;
pub const PCM512x_IDCH: u32 = 1 << 3;
pub const PCM512x_IDSK: u32 = 1 << 4;
pub const PCM512x_IDBK: u32 = 1 << 5;
pub const PCM512x_IDFS: u32 = 1 << 6;

/* Page 0, Register 40 - I2S configuration */
pub const PCM512x_ALEN: u32 = 3 << 0;
pub const PCM512x_ALEN_SHIFT: u32 = 0;
pub const PCM512x_ALEN_16: u32 = 0 << 0;
pub const PCM512x_ALEN_20: u32 = 1 << 0;
pub const PCM512x_ALEN_24: u32 = 2 << 0;
pub const PCM512x_ALEN_32: u32 = 3 << 0;
pub const PCM512x_AFMT: u32 = 3 << 4;
pub const PCM512x_AFMT_SHIFT: u32 = 4;
pub const PCM512x_AFMT_I2S: u32 = 0 << 4;
pub const PCM512x_AFMT_DSP: u32 = 1 << 4;
pub const PCM512x_AFMT_RTJ: u32 = 2 << 4;
pub const PCM512x_AFMT_LTJ: u32 = 3 << 4;

/* Page 0, Register 42 - DAC routing */
pub const PCM512x_AUPR_SHIFT: u32 = 0;
pub const PCM512x_AUPL_SHIFT: u32 = 4;

/* Page 0, Register 59 - auto mute */
pub const PCM512x_ATMR_SHIFT: u32 = 0;
pub const PCM512x_ATML_SHIFT: u32 = 4;

/* Page 0, Register 63 - ramp rates */
pub const PCM512x_VNDF_SHIFT: u32 = 6;
pub const PCM512x_VNDS_SHIFT: u32 = 4;
pub const PCM512x_VNUF_SHIFT: u32 = 2;
pub const PCM512x_VNUS_SHIFT: u32 = 0;

/* Page 0, Register 64 - emergency ramp rates */
pub const PCM512x_VEDF_SHIFT: u32 = 6;
pub const PCM512x_VEDS_SHIFT: u32 = 4;

/* Page 0, Register 65 - Digital mute enables */
pub const PCM512x_ACTL_SHIFT: u32 = 2;
pub const PCM512x_AMLE_SHIFT: u32 = 1;
pub const PCM512x_AMRE_SHIFT: u32 = 0;

/* Page 0, Register 80-85, GPIO output selection */
pub const PCM512x_GxSL: u32 = 31 << 0;
pub const PCM512x_GxSL_SHIFT: u32 = 0;
pub const PCM512x_GxSL_OFF: u32 = 0 << 0;
pub const PCM512x_GxSL_DSP: u32 = 1 << 0;
pub const PCM512x_GxSL_REG: u32 = 2 << 0;
pub const PCM512x_GxSL_AMUTB: u32 = 3 << 0;
pub const PCM512x_GxSL_AMUTL: u32 = 4 << 0;
pub const PCM512x_GxSL_AMUTR: u32 = 5 << 0;
pub const PCM512x_GxSL_CLKI: u32 = 6 << 0;
pub const PCM512x_GxSL_SDOUT: u32 = 7 << 0;
pub const PCM512x_GxSL_ANMUL: u32 = 8 << 0;
pub const PCM512x_GxSL_ANMUR: u32 = 9 << 0;
pub const PCM512x_GxSL_PLLLK: u32 = 10 << 0;
pub const PCM512x_GxSL_CPCLK: u32 = 11 << 0;
pub const PCM512x_GxSL_UV0_7: u32 = 14 << 0;
pub const PCM512x_GxSL_UV0_3: u32 = 15 << 0;
pub const PCM512x_GxSL_PLLCK: u32 = 16 << 0;

/* Page 1, Register 2 - analog volume control */
pub const PCM512x_RAGN_SHIFT: u32 = 0;
pub const PCM512x_LAGN_SHIFT: u32 = 4;

/* Page 1, Register 7 - analog boost control */
pub const PCM512x_AGBR_SHIFT: u32 = 0;
pub const PCM512x_AGBL_SHIFT: u32 = 4;

extern "C" {
    pub static pcm512x_pm_ops: dev_pm_ops;
    pub static pcm512x_regmap: regmap_config;

    pub fn pcm512x_probe(dev: *mut device, regmap: *mut regmap) -> ::std::os::raw::c_int;
    pub fn pcm512x_remove(dev: *mut device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
