/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NAU8810 ALSA SoC audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 * Author: David Lin <ctlin0@nuvoton.com>
 */

pub const NAU8810_REG_RESET: u32 = 0x00;
pub const NAU8810_REG_POWER1: u32 = 0x01;
pub const NAU8810_REG_POWER2: u32 = 0x02;
pub const NAU8810_REG_POWER3: u32 = 0x03;
pub const NAU8810_REG_IFACE: u32 = 0x04;
pub const NAU8810_REG_COMP: u32 = 0x05;
pub const NAU8810_REG_CLOCK: u32 = 0x06;
pub const NAU8810_REG_SMPLR: u32 = 0x07;
pub const NAU8810_REG_DAC: u32 = 0x0A;
pub const NAU8810_REG_DACGAIN: u32 = 0x0B;
pub const NAU8810_REG_ADC: u32 = 0x0E;
pub const NAU8810_REG_ADCGAIN: u32 = 0x0F;
pub const NAU8810_REG_EQ1: u32 = 0x12;
pub const NAU8810_REG_EQ2: u32 = 0x13;
pub const NAU8810_REG_EQ3: u32 = 0x14;
pub const NAU8810_REG_EQ4: u32 = 0x15;
pub const NAU8810_REG_EQ5: u32 = 0x16;
pub const NAU8810_REG_DACLIM1: u32 = 0x18;
pub const NAU8810_REG_DACLIM2: u32 = 0x19;
pub const NAU8810_REG_NOTCH1: u32 = 0x1B;
pub const NAU8810_REG_NOTCH2: u32 = 0x1C;
pub const NAU8810_REG_NOTCH3: u32 = 0x1D;
pub const NAU8810_REG_NOTCH4: u32 = 0x1E;
pub const NAU8810_REG_ALC1: u32 = 0x20;
pub const NAU8810_REG_ALC2: u32 = 0x21;
pub const NAU8810_REG_ALC3: u32 = 0x22;
pub const NAU8810_REG_NOISEGATE: u32 = 0x23;
pub const NAU8810_REG_PLLN: u32 = 0x24;
pub const NAU8810_REG_PLLK1: u32 = 0x25;
pub const NAU8810_REG_PLLK2: u32 = 0x26;
pub const NAU8810_REG_PLLK3: u32 = 0x27;
pub const NAU8810_REG_ATTEN: u32 = 0x28;
pub const NAU8810_REG_INPUT_SIGNAL: u32 = 0x2C;
pub const NAU8810_REG_PGAGAIN: u32 = 0x2D;
pub const NAU8810_REG_ADCBOOST: u32 = 0x2F;
pub const NAU8810_REG_OUTPUT: u32 = 0x31;
pub const NAU8810_REG_SPKMIX: u32 = 0x32;
pub const NAU8810_REG_SPKGAIN: u32 = 0x36;
pub const NAU8810_REG_MONOMIX: u32 = 0x38;
pub const NAU8810_REG_POWER4: u32 = 0x3A;
pub const NAU8810_REG_TSLOTCTL1: u32 = 0x3B;
pub const NAU8810_REG_TSLOTCTL2: u32 = 0x3C;
pub const NAU8810_REG_DEVICE_REVID: u32 = 0x3E;
pub const NAU8810_REG_I2C_DEVICEID: u32 = 0x3F;
pub const NAU8810_REG_ADDITIONID: u32 = 0x40;
pub const NAU8810_REG_RESERVE: u32 = 0x41;
pub const NAU8810_REG_OUTCTL: u32 = 0x45;
pub const NAU8810_REG_ALC1ENHAN1: u32 = 0x46;
pub const NAU8810_REG_ALC1ENHAN2: u32 = 0x47;
pub const NAU8810_REG_MISCCTL: u32 = 0x49;
pub const NAU8810_REG_OUTTIEOFF: u32 = 0x4B;
pub const NAU8810_REG_AGCP2POUT: u32 = 0x4C;
pub const NAU8810_REG_AGCPOUT: u32 = 0x4D;
pub const NAU8810_REG_AMTCTL: u32 = 0x4E;
pub const NAU8810_REG_OUTTIEOFFMAN: u32 = 0x4F;
pub const NAU8810_REG_MAX: u32 = NAU8810_REG_OUTTIEOFFMAN;

/* NAU8810_REG_POWER1 (0x1) */
pub const NAU8810_DCBUF_EN: u32 = 0x1 << 8;
pub const NAU8810_AUX_EN_SFT: u32 = 6;
pub const NAU8810_PLL_EN_SFT: u32 = 5;
pub const NAU8810_MICBIAS_EN_SFT: u32 = 4;
pub const NAU8810_ABIAS_EN: u32 = 0x1 << 3;
pub const NAU8810_IOBUF_EN: u32 = 0x1 << 2;
pub const NAU8810_REFIMP_MASK: u32 = 0x3;
pub const NAU8810_REFIMP_DIS: u32 = 0x0;
pub const NAU8810_REFIMP_80K: u32 = 0x1;
pub const NAU8810_REFIMP_300K: u32 = 0x2;
pub const NAU8810_REFIMP_3K: u32 = 0x3;

/* NAU8810_REG_POWER2 (0x2) */
pub const NAU8810_BST_EN_SFT: u32 = 4;
pub const NAU8810_PGA_EN_SFT: u32 = 2;
pub const NAU8810_ADC_EN_SFT: u32 = 0;

/* NAU8810_REG_POWER3 (0x3) */
pub const NAU8810_DAC_EN_SFT: u32 = 0;
pub const NAU8810_SPKMX_EN_SFT: u32 = 2;
pub const NAU8810_MOUTMX_EN_SFT: u32 = 3;
pub const NAU8810_PSPK_EN_SFT: u32 = 5;
pub const NAU8810_NSPK_EN_SFT: u32 = 6;
pub const NAU8810_MOUT_EN_SFT: u32 = 7;

/* NAU8810_REG_IFACE (0x4) */
pub const NAU8810_AIFMT_SFT: u32 = 3;
pub const NAU8810_AIFMT_MASK: u32 = 0x3 << NAU8810_AIFMT_SFT;
pub const NAU8810_AIFMT_RIGHT: u32 = 0x0 << NAU8810_AIFMT_SFT;
pub const NAU8810_AIFMT_LEFT: u32 = 0x1 << NAU8810_AIFMT_SFT;
pub const NAU8810_AIFMT_I2S: u32 = 0x2 << NAU8810_AIFMT_SFT;
pub const NAU8810_AIFMT_PCM_A: u32 = 0x3 << NAU8810_AIFMT_SFT;
pub const NAU8810_WLEN_SFT: u32 = 5;
pub const NAU8810_WLEN_MASK: u32 = 0x3 << NAU8810_WLEN_SFT;
pub const NAU8810_WLEN_16: u32 = 0x0 << NAU8810_WLEN_SFT;
pub const NAU8810_WLEN_20: u32 = 0x1 << NAU8810_WLEN_SFT;
pub const NAU8810_WLEN_24: u32 = 0x2 << NAU8810_WLEN_SFT;
pub const NAU8810_WLEN_32: u32 = 0x3 << NAU8810_WLEN_SFT;
pub const NAU8810_FSP_IF: u32 = 0x1 << 7;
pub const NAU8810_BCLKP_IB: u32 = 0x1 << 8;

/* NAU8810_REG_COMP (0x5) */
pub const NAU8810_ADDAP_SFT: u32 = 0;
pub const NAU8810_ADCCM_SFT: u32 = 1;
pub const NAU8810_DACCM_SFT: u32 = 3;

/* NAU8810_REG_CLOCK (0x6) */
pub const NAU8810_CLKIO_MASK: u32 = 0x1;
pub const NAU8810_CLKIO_SLAVE: u32 = 0x0;
pub const NAU8810_CLKIO_MASTER: u32 = 0x1;
pub const NAU8810_BCLKSEL_SFT: u32 = 2;
pub const NAU8810_BCLKSEL_MASK: u32 = 0x7 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_1: u32 = 0x0 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_2: u32 = 0x1 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_4: u32 = 0x2 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_8: u32 = 0x3 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_16: u32 = 0x4 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_BCLKDIV_32: u32 = 0x5 << NAU8810_BCLKSEL_SFT;
pub const NAU8810_MCLKSEL_SFT: u32 = 5;
pub const NAU8810_MCLKSEL_MASK: u32 = 0x7 << NAU8810_MCLKSEL_SFT;
pub const NAU8810_CLKM_SFT: u32 = 8;
pub const NAU8810_CLKM_MASK: u32 = 0x1 << NAU8810_CLKM_SFT;
pub const NAU8810_CLKM_MCLK: u32 = 0x0 << NAU8810_CLKM_SFT;
pub const NAU8810_CLKM_PLL: u32 = 0x1 << NAU8810_CLKM_SFT;

/* NAU8810_REG_SMPLR (0x7) */
pub const NAU8810_SMPLR_SFT: u32 = 1;
pub const NAU8810_SMPLR_MASK: u32 = 0x7 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_48K: u32 = 0x0 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_32K: u32 = 0x1 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_24K: u32 = 0x2 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_16K: u32 = 0x3 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_12K: u32 = 0x4 << NAU8810_SMPLR_SFT;
pub const NAU8810_SMPLR_8K: u32 = 0x5 << NAU8810_SMPLR_SFT;

/* NAU8810_REG_DAC (0xA) */
pub const NAU8810_DACPL_SFT: u32 = 0;
pub const NAU8810_DACOS_SFT: u32 = 3;
pub const NAU8810_DEEMP_SFT: u32 = 4;

/* NAU8810_REG_DACGAIN (0xB) */
pub const NAU8810_DACGAIN_SFT: u32 = 0;

/* NAU8810_REG_ADC (0xE) */
pub const NAU8810_ADCPL_SFT: u32 = 0;
pub const NAU8810_ADCOS_SFT: u32 = 3;
pub const NAU8810_HPF_SFT: u32 = 4;
pub const NAU8810_HPFEN_SFT: u32 = 8;

/* NAU8810_REG_ADCGAIN (0xF) */
pub const NAU8810_ADCGAIN_SFT: u32 = 0;

/* NAU8810_REG_EQ1 (0x12) */
pub const NAU8810_EQ1GC_SFT: u32 = 0;
pub const NAU8810_EQ1CF_SFT: u32 = 5;
pub const NAU8810_EQM_SFT: u32 = 8;

/* NAU8810_REG_EQ2 (0x13) */
pub const NAU8810_EQ2GC_SFT: u32 = 0;
pub const NAU8810_EQ2CF_SFT: u32 = 5;
pub const NAU8810_EQ2BW_SFT: u32 = 8;

/* NAU8810_REG_EQ3 (0x14) */
pub const NAU8810_EQ3GC_SFT: u32 = 0;
pub const NAU8810_EQ3CF_SFT: u32 = 5;
pub const NAU8810_EQ3BW_SFT: u32 = 8;

/* NAU8810_REG_EQ4 (0x15) */
pub const NAU8810_EQ4GC_SFT: u32 = 0;
pub const NAU8810_EQ4CF_SFT: u32 = 5;
pub const NAU8810_EQ4BW_SFT: u32 = 8;

/* NAU8810_REG_EQ5 (0x16) */
pub const NAU8810_EQ5GC_SFT: u32 = 0;
pub const NAU8810_EQ5CF_SFT: u32 = 5;

/* NAU8810_REG_DACLIM1 (0x18) */
pub const NAU8810_DACLIMATK_SFT: u32 = 0;
pub const NAU8810_DACLIMDCY_SFT: u32 = 4;
pub const NAU8810_DACLIMEN_SFT: u32 = 8;

/* NAU8810_REG_DACLIM2 (0x19) */
pub const NAU8810_DACLIMBST_SFT: u32 = 0;
pub const NAU8810_DACLIMTHL_SFT: u32 = 4;

/* NAU8810_REG_ALC1 (0x20) */
pub const NAU8810_ALCMINGAIN_SFT: u32 = 0;
pub const NAU8810_ALCMXGAIN_SFT: u32 = 3;
pub const NAU8810_ALCEN_SFT: u32 = 8;

/* NAU8810_REG_ALC2 (0x21) */
pub const NAU8810_ALCSL_SFT: u32 = 0;
pub const NAU8810_ALCHT_SFT: u32 = 4;
pub const NAU8810_ALCZC_SFT: u32 = 8;

/* NAU8810_REG_ALC3 (0x22) */
pub const NAU8810_ALCATK_SFT: u32 = 0;
pub const NAU8810_ALCDCY_SFT: u32 = 4;
pub const NAU8810_ALCM_SFT: u32 = 8;

/* NAU8810_REG_NOISEGATE (0x23) */
pub const NAU8810_ALCNTH_SFT: u32 = 0;
pub const NAU8810_ALCNEN_SFT: u32 = 3;

/* NAU8810_REG_PLLN (0x24) */
pub const NAU8810_PLLN_MASK: u32 = 0xF;
pub const NAU8810_PLLMCLK_DIV2: u32 = 0x1 << 4;

/* NAU8810_REG_PLLK1 (0x25) */
pub const NAU8810_PLLK1_SFT: u32 = 18;
pub const NAU8810_PLLK1_MASK: u32 = 0x3F;

/* NAU8810_REG_PLLK2 (0x26) */
pub const NAU8810_PLLK2_SFT: u32 = 9;
pub const NAU8810_PLLK2_MASK: u32 = 0x1FF;

/* NAU8810_REG_PLLK3 (0x27) */
pub const NAU8810_PLLK3_MASK: u32 = 0x1FF;

/* NAU8810_REG_INPUT_SIGNAL (0x2C) */
pub const NAU8810_PMICPGA_SFT: u32 = 0;
pub const NAU8810_PMICPGA_EN: u32 = 0x1 << NAU8810_PMICPGA_SFT;
pub const NAU8810_NMICPGA_SFT: u32 = 1;
pub const NAU8810_NMICPGA_EN: u32 = 0x1 << NAU8810_NMICPGA_SFT;
pub const NAU8810_AUXPGA_SFT: u32 = 2;

/* NAU8810_REG_PGAGAIN (0x2D) */
pub const NAU8810_PGAGAIN_SFT: u32 = 0;
pub const NAU8810_PGAMT_SFT: u32 = 6;
pub const NAU8810_PGAZC_SFT: u32 = 7;

/* NAU8810_REG_ADCBOOST (0x2F) */
pub const NAU8810_AUXBSTGAIN_SFT: u32 = 0;
pub const NAU8810_PMICBSTGAIN_SFT: u32 = 4;
pub const NAU8810_PMICBSTGAIN_MASK: u32 = 0x7 << NAU8810_PMICBSTGAIN_SFT;
pub const NAU8810_PGABST_SFT: u32 = 8;

/* NAU8810_REG_SPKMIX (0x32) */
pub const NAU8810_DACSPK_SFT: u32 = 0;
pub const NAU8810_BYPSPK_SFT: u32 = 1;
pub const NAU8810_AUXSPK_SFT: u32 = 5;

/* NAU8810_REG_SPKGAIN (0x36) */
pub const NAU8810_SPKGAIN_SFT: u32 = 0;
pub const NAU8810_SPKMT_SFT: u32 = 6;
pub const NAU8810_SPKZC_SFT: u32 = 7;

/* NAU8810_REG_MONOMIX (0x38) */
pub const NAU8810_DACMOUT_SFT: u32 = 0;
pub const NAU8810_BYPMOUT_SFT: u32 = 1;
pub const NAU8810_AUXMOUT_SFT: u32 = 2;
pub const NAU8810_MOUTMXMT_SFT: u32 = 6;

/* System Clock Source */
pub const NAU8810_SCLK_MCLK: u32 = 0;
pub const NAU8810_SCLK_PLL: u32 = 1;

#[repr(C)]
pub struct nau8810_pll {
    pub pre_factor: ::core::ffi::c_int,
    pub mclk_scaler: ::core::ffi::c_int,
    pub pll_frac: ::core::ffi::c_int,
    pub pll_int: ::core::ffi::c_int,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nau8810 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pll: nau8810_pll,
    pub sysclk: ::core::ffi::c_int,
    pub clk_id: ::core::ffi::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
