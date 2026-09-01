/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * max98090.h -- MAX98090 ALSA SoC Audio driver
 *
 * Copyright 2011-2012 Maxim Integrated Products
 */


/*
 * The default operating frequency for a DMIC attached to the codec.
 * This can be overridden by a device tree property.
 */
pub const MAX98090_DEFAULT_DMIC_FREQ: u32 = 2500000;

/*
 * MAX98090 Register Definitions
 */

pub const M98090_REG_SOFTWARE_RESET: u32 = 0x00;
pub const M98090_REG_DEVICE_STATUS: u32 = 0x01;
pub const M98090_REG_JACK_STATUS: u32 = 0x02;
pub const M98090_REG_INTERRUPT_S: u32 = 0x03;
pub const M98090_REG_QUICK_SYSTEM_CLOCK: u32 = 0x04;
pub const M98090_REG_QUICK_SAMPLE_RATE: u32 = 0x05;
pub const M98090_REG_DAI_INTERFACE: u32 = 0x06;
pub const M98090_REG_DAC_PATH: u32 = 0x07;
pub const M98090_REG_MIC_DIRECT_TO_ADC: u32 = 0x08;
pub const M98090_REG_LINE_TO_ADC: u32 = 0x09;
pub const M98090_REG_ANALOG_MIC_LOOP: u32 = 0x0A;
pub const M98090_REG_ANALOG_LINE_LOOP: u32 = 0x0B;
pub const M98090_REG_RESERVED: u32 = 0x0C;
pub const M98090_REG_LINE_INPUT_CONFIG: u32 = 0x0D;
pub const M98090_REG_LINE_INPUT_LEVEL: u32 = 0x0E;
pub const M98090_REG_INPUT_MODE: u32 = 0x0F;
pub const M98090_REG_MIC1_INPUT_LEVEL: u32 = 0x10;
pub const M98090_REG_MIC2_INPUT_LEVEL: u32 = 0x11;
pub const M98090_REG_MIC_BIAS_VOLTAGE: u32 = 0x12;
pub const M98090_REG_DIGITAL_MIC_ENABLE: u32 = 0x13;
pub const M98090_REG_DIGITAL_MIC_CONFIG: u32 = 0x14;
pub const M98090_REG_LEFT_ADC_MIXER: u32 = 0x15;
pub const M98090_REG_RIGHT_ADC_MIXER: u32 = 0x16;
pub const M98090_REG_LEFT_ADC_LEVEL: u32 = 0x17;
pub const M98090_REG_RIGHT_ADC_LEVEL: u32 = 0x18;
pub const M98090_REG_ADC_BIQUAD_LEVEL: u32 = 0x19;
pub const M98090_REG_ADC_SIDETONE: u32 = 0x1A;
pub const M98090_REG_SYSTEM_CLOCK: u32 = 0x1B;
pub const M98090_REG_CLOCK_MODE: u32 = 0x1C;
pub const M98090_REG_CLOCK_RATIO_NI_MSB: u32 = 0x1D;
pub const M98090_REG_CLOCK_RATIO_NI_LSB: u32 = 0x1E;
pub const M98090_REG_CLOCK_RATIO_MI_MSB: u32 = 0x1F;
pub const M98090_REG_CLOCK_RATIO_MI_LSB: u32 = 0x20;
pub const M98090_REG_MASTER_MODE: u32 = 0x21;
pub const M98090_REG_INTERFACE_FORMAT: u32 = 0x22;
pub const M98090_REG_TDM_CONTROL: u32 = 0x23;
pub const M98090_REG_TDM_FORMAT: u32 = 0x24;
pub const M98090_REG_IO_CONFIGURATION: u32 = 0x25;
pub const M98090_REG_FILTER_CONFIG: u32 = 0x26;
pub const M98090_REG_DAI_PLAYBACK_LEVEL: u32 = 0x27;
pub const M98090_REG_DAI_PLAYBACK_LEVEL_EQ: u32 = 0x28;
pub const M98090_REG_LEFT_HP_MIXER: u32 = 0x29;
pub const M98090_REG_RIGHT_HP_MIXER: u32 = 0x2A;
pub const M98090_REG_HP_CONTROL: u32 = 0x2B;
pub const M98090_REG_LEFT_HP_VOLUME: u32 = 0x2C;
pub const M98090_REG_RIGHT_HP_VOLUME: u32 = 0x2D;
pub const M98090_REG_LEFT_SPK_MIXER: u32 = 0x2E;
pub const M98090_REG_RIGHT_SPK_MIXER: u32 = 0x2F;
pub const M98090_REG_SPK_CONTROL: u32 = 0x30;
pub const M98090_REG_LEFT_SPK_VOLUME: u32 = 0x31;
pub const M98090_REG_RIGHT_SPK_VOLUME: u32 = 0x32;
pub const M98090_REG_DRC_TIMING: u32 = 0x33;
pub const M98090_REG_DRC_COMPRESSOR: u32 = 0x34;
pub const M98090_REG_DRC_EXPANDER: u32 = 0x35;
pub const M98090_REG_DRC_GAIN: u32 = 0x36;
pub const M98090_REG_RCV_LOUTL_MIXER: u32 = 0x37;
pub const M98090_REG_RCV_LOUTL_CONTROL: u32 = 0x38;
pub const M98090_REG_RCV_LOUTL_VOLUME: u32 = 0x39;
pub const M98090_REG_LOUTR_MIXER: u32 = 0x3A;
pub const M98090_REG_LOUTR_CONTROL: u32 = 0x3B;
pub const M98090_REG_LOUTR_VOLUME: u32 = 0x3C;
pub const M98090_REG_JACK_DETECT: u32 = 0x3D;
pub const M98090_REG_INPUT_ENABLE: u32 = 0x3E;
pub const M98090_REG_OUTPUT_ENABLE: u32 = 0x3F;
pub const M98090_REG_LEVEL_CONTROL: u32 = 0x40;
pub const M98090_REG_DSP_FILTER_ENABLE: u32 = 0x41;
pub const M98090_REG_BIAS_CONTROL: u32 = 0x42;
pub const M98090_REG_DAC_CONTROL: u32 = 0x43;
pub const M98090_REG_ADC_CONTROL: u32 = 0x44;
pub const M98090_REG_DEVICE_SHUTDOWN: u32 = 0x45;
pub const M98090_REG_EQUALIZER_BASE: u32 = 0x46;
pub const M98090_REG_RECORD_BIQUAD_BASE: u32 = 0xAF;
pub const M98090_REG_DMIC3_VOLUME: u32 = 0xBE;
pub const M98090_REG_DMIC4_VOLUME: u32 = 0xBF;
pub const M98090_REG_DMIC34_BQ_PREATTEN: u32 = 0xC0;
pub const M98090_REG_RECORD_TDM_SLOT: u32 = 0xC1;
pub const M98090_REG_SAMPLE_RATE: u32 = 0xC2;
pub const M98090_REG_DMIC34_BIQUAD_BASE: u32 = 0xC3;
pub const M98090_REG_REVISION_ID: u32 = 0xFF;

pub const M98090_REG_CNT: u32 = (0xFF + 1);
pub const MAX98090_MAX_REGISTER: u32 = 0xFF;

/* MAX98090 Register Bit Fields */

/*
 * M98090_REG_SOFTWARE_RESET
 */
pub const M98090_SWRESET_MASK: u32 = (1 << 7);
pub const M98090_SWRESET_SHIFT: u32 = 7;
pub const M98090_SWRESET_WIDTH: u32 = 1;

/*
 * M98090_REG_DEVICE_STATUS
 */
pub const M98090_CLD_MASK: u32 = (1 << 7);
pub const M98090_CLD_SHIFT: u32 = 7;
pub const M98090_CLD_WIDTH: u32 = 1;
pub const M98090_SLD_MASK: u32 = (1 << 6);
pub const M98090_SLD_SHIFT: u32 = 6;
pub const M98090_SLD_WIDTH: u32 = 1;
pub const M98090_ULK_MASK: u32 = (1 << 5);
pub const M98090_ULK_SHIFT: u32 = 5;
pub const M98090_ULK_WIDTH: u32 = 1;
pub const M98090_JDET_MASK: u32 = (1 << 2);
pub const M98090_JDET_SHIFT: u32 = 2;
pub const M98090_JDET_WIDTH: u32 = 1;
pub const M98090_DRCACT_MASK: u32 = (1 << 1);
pub const M98090_DRCACT_SHIFT: u32 = 1;
pub const M98090_DRCACT_WIDTH: u32 = 1;
pub const M98090_DRCCLP_MASK: u32 = (1 << 0);
pub const M98090_DRCCLP_SHIFT: u32 = 0;
pub const M98090_DRCCLP_WIDTH: u32 = 1;

/*
 * M98090_REG_JACK_STATUS
 */
pub const M98090_LSNS_MASK: u32 = (1 << 2);
pub const M98090_LSNS_SHIFT: u32 = 2;
pub const M98090_LSNS_WIDTH: u32 = 1;
pub const M98090_JKSNS_MASK: u32 = (1 << 1);
pub const M98090_JKSNS_SHIFT: u32 = 1;
pub const M98090_JKSNS_WIDTH: u32 = 1;

/*
 * M98090_REG_INTERRUPT_S
 */
pub const M98090_ICLD_MASK: u32 = (1 << 7);
pub const M98090_ICLD_SHIFT: u32 = 7;
pub const M98090_ICLD_WIDTH: u32 = 1;
pub const M98090_ISLD_MASK: u32 = (1 << 6);
pub const M98090_ISLD_SHIFT: u32 = 6;
pub const M98090_ISLD_WIDTH: u32 = 1;
pub const M98090_IULK_MASK: u32 = (1 << 5);
pub const M98090_IULK_SHIFT: u32 = 5;
pub const M98090_IULK_WIDTH: u32 = 1;
pub const M98090_IJDET_MASK: u32 = (1 << 2);
pub const M98090_IJDET_SHIFT: u32 = 2;
pub const M98090_IJDET_WIDTH: u32 = 1;
pub const M98090_IDRCACT_MASK: u32 = (1 << 1);
pub const M98090_IDRCACT_SHIFT: u32 = 1;
pub const M98090_IDRCACT_WIDTH: u32 = 1;
pub const M98090_IDRCCLP_MASK: u32 = (1 << 0);
pub const M98090_IDRCCLP_SHIFT: u32 = 0;
pub const M98090_IDRCCLP_WIDTH: u32 = 1;

/*
 * M98090_REG_QUICK_SYSTEM_CLOCK
 */
pub const M98090_26M_MASK: u32 = (1 << 7);
pub const M98090_26M_SHIFT: u32 = 7;
pub const M98090_26M_WIDTH: u32 = 1;
pub const M98090_19P2M_MASK: u32 = (1 << 6);
pub const M98090_19P2M_SHIFT: u32 = 6;
pub const M98090_19P2M_WIDTH: u32 = 1;
pub const M98090_13M_MASK: u32 = (1 << 5);
pub const M98090_13M_SHIFT: u32 = 5;
pub const M98090_13M_WIDTH: u32 = 1;
pub const M98090_12P288M_MASK: u32 = (1 << 4);
pub const M98090_12P288M_SHIFT: u32 = 4;
pub const M98090_12P288M_WIDTH: u32 = 1;
pub const M98090_12M_MASK: u32 = (1 << 3);
pub const M98090_12M_SHIFT: u32 = 3;
pub const M98090_12M_WIDTH: u32 = 1;
pub const M98090_11P2896M_MASK: u32 = (1 << 2);
pub const M98090_11P2896M_SHIFT: u32 = 2;
pub const M98090_11P2896M_WIDTH: u32 = 1;
pub const M98090_256FS_MASK: u32 = (1 << 0);
pub const M98090_256FS_SHIFT: u32 = 0;
pub const M98090_256FS_WIDTH: u32 = 1;
pub const M98090_CLK_ALL_SHIFT: u32 = 0;
pub const M98090_CLK_ALL_WIDTH: u32 = 8;
pub const M98090_CLK_ALL_NUM: u32 = (1 << M98090_CLK_ALL_WIDTH);

/*
 * M98090_REG_QUICK_SAMPLE_RATE
 */
pub const M98090_SR_96K_MASK: u32 = (1 << 5);
pub const M98090_SR_96K_SHIFT: u32 = 5;
pub const M98090_SR_96K_WIDTH: u32 = 1;
pub const M98090_SR_32K_MASK: u32 = (1 << 4);
pub const M98090_SR_32K_SHIFT: u32 = 4;
pub const M98090_SR_32K_WIDTH: u32 = 1;
pub const M98090_SR_48K_MASK: u32 = (1 << 3);
pub const M98090_SR_48K_SHIFT: u32 = 3;
pub const M98090_SR_48K_WIDTH: u32 = 1;
pub const M98090_SR_44K1_MASK: u32 = (1 << 2);
pub const M98090_SR_44K1_SHIFT: u32 = 2;
pub const M98090_SR_44K1_WIDTH: u32 = 1;
pub const M98090_SR_16K_MASK: u32 = (1 << 1);
pub const M98090_SR_16K_SHIFT: u32 = 1;
pub const M98090_SR_16K_WIDTH: u32 = 1;
pub const M98090_SR_8K_MASK: u32 = (1 << 0);
pub const M98090_SR_8K_SHIFT: u32 = 0;
pub const M98090_SR_8K_WIDTH: u32 = 1;
pub const M98090_SR_MASK: u32 = 0x3F;
pub const M98090_SR_ALL_SHIFT: u32 = 0;
pub const M98090_SR_ALL_WIDTH: u32 = 8;
pub const M98090_SR_ALL_NUM: u32 = (1 << M98090_SR_ALL_WIDTH);

/*
 * M98090_REG_DAI_INTERFACE
 */
pub const M98090_RJ_M_MASK: u32 = (1 << 5);
pub const M98090_RJ_M_SHIFT: u32 = 5;
pub const M98090_RJ_M_WIDTH: u32 = 1;
pub const M98090_RJ_S_MASK: u32 = (1 << 4);
pub const M98090_RJ_S_SHIFT: u32 = 4;
pub const M98090_RJ_S_WIDTH: u32 = 1;
pub const M98090_LJ_M_MASK: u32 = (1 << 3);
pub const M98090_LJ_M_SHIFT: u32 = 3;
pub const M98090_LJ_M_WIDTH: u32 = 1;
pub const M98090_LJ_S_MASK: u32 = (1 << 2);
pub const M98090_LJ_S_SHIFT: u32 = 2;
pub const M98090_LJ_S_WIDTH: u32 = 1;
pub const M98090_I2S_M_MASK: u32 = (1 << 1);
pub const M98090_I2S_M_SHIFT: u32 = 1;
pub const M98090_I2S_M_WIDTH: u32 = 1;
pub const M98090_I2S_S_MASK: u32 = (1 << 0);
pub const M98090_I2S_S_SHIFT: u32 = 0;
pub const M98090_I2S_S_WIDTH: u32 = 1;
pub const M98090_DAI_ALL_SHIFT: u32 = 0;
pub const M98090_DAI_ALL_WIDTH: u32 = 8;
pub const M98090_DAI_ALL_NUM: u32 = (1 << M98090_DAI_ALL_WIDTH);

/*
 * M98090_REG_DAC_PATH
 */
pub const M98090_DIG2_HP_MASK: u32 = (1 << 7);
pub const M98090_DIG2_HP_SHIFT: u32 = 7;
pub const M98090_DIG2_HP_WIDTH: u32 = 1;
pub const M98090_DIG2_EAR_MASK: u32 = (1 << 6);
pub const M98090_DIG2_EAR_SHIFT: u32 = 6;
pub const M98090_DIG2_EAR_WIDTH: u32 = 1;
pub const M98090_DIG2_SPK_MASK: u32 = (1 << 5);
pub const M98090_DIG2_SPK_SHIFT: u32 = 5;
pub const M98090_DIG2_SPK_WIDTH: u32 = 1;
pub const M98090_DIG2_LOUT_MASK: u32 = (1 << 4);
pub const M98090_DIG2_LOUT_SHIFT: u32 = 4;
pub const M98090_DIG2_LOUT_WIDTH: u32 = 1;
pub const M98090_DIG2_ALL_SHIFT: u32 = 0;
pub const M98090_DIG2_ALL_WIDTH: u32 = 8;
pub const M98090_DIG2_ALL_NUM: u32 = (1 << M98090_DIG2_ALL_WIDTH);

/*
 * M98090_REG_MIC_DIRECT_TO_ADC
 */
pub const M98090_IN12_MIC1_MASK: u32 = (1 << 7);
pub const M98090_IN12_MIC1_SHIFT: u32 = 7;
pub const M98090_IN12_MIC1_WIDTH: u32 = 1;
pub const M98090_IN34_MIC2_MASK: u32 = (1 << 6);
pub const M98090_IN34_MIC2_SHIFT: u32 = 6;
pub const M98090_IN34_MIC2_WIDTH: u32 = 1;
pub const M98090_IN56_MIC1_MASK: u32 = (1 << 5);
pub const M98090_IN56_MIC1_SHIFT: u32 = 5;
pub const M98090_IN56_MIC1_WIDTH: u32 = 1;
pub const M98090_IN56_MIC2_MASK: u32 = (1 << 4);
pub const M98090_IN56_MIC2_SHIFT: u32 = 4;
pub const M98090_IN56_MIC2_WIDTH: u32 = 1;
pub const M98090_IN12_DADC_MASK: u32 = (1 << 3);
pub const M98090_IN12_DADC_SHIFT: u32 = 3;
pub const M98090_IN12_DADC_WIDTH: u32 = 1;
pub const M98090_IN34_DADC_MASK: u32 = (1 << 2);
pub const M98090_IN34_DADC_SHIFT: u32 = 2;
pub const M98090_IN34_DADC_WIDTH: u32 = 1;
pub const M98090_IN56_DADC_MASK: u32 = (1 << 1);
pub const M98090_IN56_DADC_SHIFT: u32 = 1;
pub const M98090_IN56_DADC_WIDTH: u32 = 1;
pub const M98090_MIC_ALL_SHIFT: u32 = 0;
pub const M98090_MIC_ALL_WIDTH: u32 = 8;
pub const M98090_MIC_ALL_NUM: u32 = (1 << M98090_MIC_ALL_WIDTH);

/*
 * M98090_REG_LINE_TO_ADC
 */
pub const M98090_IN12S_AB_MASK: u32 = (1 << 7);
pub const M98090_IN12S_AB_SHIFT: u32 = 7;
pub const M98090_IN12S_AB_WIDTH: u32 = 1;
pub const M98090_IN34S_AB_MASK: u32 = (1 << 6);
pub const M98090_IN34S_AB_SHIFT: u32 = 6;
pub const M98090_IN34S_AB_WIDTH: u32 = 1;
pub const M98090_IN56S_AB_MASK: u32 = (1 << 5);
pub const M98090_IN56S_AB_SHIFT: u32 = 5;
pub const M98090_IN56S_AB_WIDTH: u32 = 1;
pub const M98090_IN34D_A_MASK: u32 = (1 << 4);
pub const M98090_IN34D_A_SHIFT: u32 = 4;
pub const M98090_IN34D_A_WIDTH: u32 = 1;
pub const M98090_IN56D_B_MASK: u32 = (1 << 3);
pub const M98090_IN56D_B_SHIFT: u32 = 3;
pub const M98090_IN56D_B_WIDTH: u32 = 1;
pub const M98090_LINE_ALL_SHIFT: u32 = 0;
pub const M98090_LINE_ALL_WIDTH: u32 = 8;
pub const M98090_LINE_ALL_NUM: u32 = (1 << M98090_LINE_ALL_WIDTH);

/*
 * M98090_REG_ANALOG_MIC_LOOP
 */
pub const M98090_IN12_M1HPL_MASK: u32 = (1 << 7);
pub const M98090_IN12_M1HPL_SHIFT: u32 = 7;
pub const M98090_IN12_M1HPL_WIDTH: u32 = 1;
pub const M98090_IN12_M1SPKL_MASK: u32 = (1 << 6);
pub const M98090_IN12_M1SPKL_SHIFT: u32 = 6;
pub const M98090_IN12_M1SPKL_WIDTH: u32 = 1;
pub const M98090_IN12_M1EAR_MASK: u32 = (1 << 5);
pub const M98090_IN12_M1EAR_SHIFT: u32 = 5;
pub const M98090_IN12_M1EAR_WIDTH: u32 = 1;
pub const M98090_IN12_M1LOUTL_MASK: u32 = (1 << 4);
pub const M98090_IN12_M1LOUTL_SHIFT: u32 = 4;
pub const M98090_IN12_M1LOUTL_WIDTH: u32 = 1;
pub const M98090_IN34_M2HPR_MASK: u32 = (1 << 3);
pub const M98090_IN34_M2HPR_SHIFT: u32 = 3;
pub const M98090_IN34_M2HPR_WIDTH: u32 = 1;
pub const M98090_IN34_M2SPKR_MASK: u32 = (1 << 2);
pub const M98090_IN34_M2SPKR_SHIFT: u32 = 2;
pub const M98090_IN34_M2SPKR_WIDTH: u32 = 1;
pub const M98090_IN34_M2EAR_MASK: u32 = (1 << 1);
pub const M98090_IN34_M2EAR_SHIFT: u32 = 1;
pub const M98090_IN34_M2EAR_WIDTH: u32 = 1;
pub const M98090_IN34_M2LOUTR_MASK: u32 = (1 << 0);
pub const M98090_IN34_M2LOUTR_SHIFT: u32 = 0;
pub const M98090_IN34_M2LOUTR_WIDTH: u32 = 1;
pub const M98090_AMIC_ALL_SHIFT: u32 = 0;
pub const M98090_AMIC_ALL_WIDTH: u32 = 8;
pub const M98090_AMIC_ALL_NUM: u32 = (1 << M98090_AMIC_ALL_WIDTH);

/*
 * M98090_REG_ANALOG_LINE_LOOP
 */
pub const M98090_IN12S_ABHP_MASK: u32 = (1 << 7);
pub const M98090_IN12S_ABHP_SHIFT: u32 = 7;
pub const M98090_IN12S_ABHP_WIDTH: u32 = 1;
pub const M98090_IN34D_ASPKL_MASK: u32 = (1 << 6);
pub const M98090_IN34D_ASPKL_SHIFT: u32 = 6;
pub const M98090_IN34D_ASPKL_WIDTH: u32 = 1;
pub const M98090_IN34D_AEAR_MASK: u32 = (1 << 5);
pub const M98090_IN34D_AEAR_SHIFT: u32 = 5;
pub const M98090_IN34D_AEAR_WIDTH: u32 = 1;
pub const M98090_IN12S_ABLOUT_MASK: u32 = (1 << 4);
pub const M98090_IN12S_ABLOUT_SHIFT: u32 = 4;
pub const M98090_IN12S_ABLOUT_WIDTH: u32 = 1;
pub const M98090_IN34S_ABHP_MASK: u32 = (1 << 3);
pub const M98090_IN34S_ABHP_SHIFT: u32 = 3;
pub const M98090_IN34S_ABHP_WIDTH: u32 = 1;
pub const M98090_IN56D_BSPKR_MASK: u32 = (1 << 2);
pub const M98090_IN56D_BSPKR_SHIFT: u32 = 2;
pub const M98090_IN56D_BSPKR_WIDTH: u32 = 1;
pub const M98090_IN56D_BEAR_MASK: u32 = (1 << 1);
pub const M98090_IN56D_BEAR_SHIFT: u32 = 1;
pub const M98090_IN56D_BEAR_WIDTH: u32 = 1;
pub const M98090_IN34S_ABLOUT_MASK: u32 = (1 << 0);
pub const M98090_IN34S_ABLOUT_SHIFT: u32 = 0;
pub const M98090_IN34S_ABLOUT_WIDTH: u32 = 1;
pub const M98090_ALIN_ALL_SHIFT: u32 = 0;
pub const M98090_ALIN_ALL_WIDTH: u32 = 8;
pub const M98090_ALIN_ALL_NUM: u32 = (1 << M98090_ALIN_ALL_WIDTH);

/*
 * M98090_REG_RESERVED
 */

/*
 * M98090_REG_LINE_INPUT_CONFIG
 */
pub const M98090_IN34DIFF_MASK: u32 = (1 << 7);
pub const M98090_IN34DIFF_SHIFT: u32 = 7;
pub const M98090_IN34DIFF_WIDTH: u32 = 1;
pub const M98090_IN56DIFF_MASK: u32 = (1 << 6);
pub const M98090_IN56DIFF_SHIFT: u32 = 6;
pub const M98090_IN56DIFF_WIDTH: u32 = 1;
pub const M98090_IN1SEEN_MASK: u32 = (1 << 5);
pub const M98090_IN1SEEN_SHIFT: u32 = 5;
pub const M98090_IN1SEEN_WIDTH: u32 = 1;
pub const M98090_IN2SEEN_MASK: u32 = (1 << 4);
pub const M98090_IN2SEEN_SHIFT: u32 = 4;
pub const M98090_IN2SEEN_WIDTH: u32 = 1;
pub const M98090_IN3SEEN_MASK: u32 = (1 << 3);
pub const M98090_IN3SEEN_SHIFT: u32 = 3;
pub const M98090_IN3SEEN_WIDTH: u32 = 1;
pub const M98090_IN4SEEN_MASK: u32 = (1 << 2);
pub const M98090_IN4SEEN_SHIFT: u32 = 2;
pub const M98090_IN4SEEN_WIDTH: u32 = 1;
pub const M98090_IN5SEEN_MASK: u32 = (1 << 1);
pub const M98090_IN5SEEN_SHIFT: u32 = 1;
pub const M98090_IN5SEEN_WIDTH: u32 = 1;
pub const M98090_IN6SEEN_MASK: u32 = (1 << 0);
pub const M98090_IN6SEEN_SHIFT: u32 = 0;
pub const M98090_IN6SEEN_WIDTH: u32 = 1;

/*
 * M98090_REG_LINE_INPUT_LEVEL
 */
pub const M98090_MIXG135_MASK: u32 = (1 << 7);
pub const M98090_MIXG135_SHIFT: u32 = 7;
pub const M98090_MIXG135_WIDTH: u32 = 1;
pub const M98090_MIXG135_NUM: u32 = (1 << M98090_MIXG135_WIDTH);
pub const M98090_MIXG246_MASK: u32 = (1 << 6);
pub const M98090_MIXG246_SHIFT: u32 = 6;
pub const M98090_MIXG246_WIDTH: u32 = 1;
pub const M98090_MIXG246_NUM: u32 = (1 << M98090_MIXG246_WIDTH);
pub const M98090_LINAPGA_MASK: u32 = (7 << 3);
pub const M98090_LINAPGA_SHIFT: u32 = 3;
pub const M98090_LINAPGA_WIDTH: u32 = 3;
pub const M98090_LINAPGA_NUM: u32 = 6;
pub const M98090_LINBPGA_MASK: u32 = (7 << 0);
pub const M98090_LINBPGA_SHIFT: u32 = 0;
pub const M98090_LINBPGA_WIDTH: u32 = 3;
pub const M98090_LINBPGA_NUM: u32 = 6;

/*
 * M98090_REG_INPUT_MODE
 */
pub const M98090_EXTBUFA_MASK: u32 = (1 << 7);
pub const M98090_EXTBUFA_SHIFT: u32 = 7;
pub const M98090_EXTBUFA_WIDTH: u32 = 1;
pub const M98090_EXTBUFA_NUM: u32 = (1 << M98090_EXTBUFA_WIDTH);
pub const M98090_EXTBUFB_MASK: u32 = (1 << 6);
pub const M98090_EXTBUFB_SHIFT: u32 = 6;
pub const M98090_EXTBUFB_WIDTH: u32 = 1;
pub const M98090_EXTBUFB_NUM: u32 = (1 << M98090_EXTBUFB_WIDTH);
pub const M98090_EXTMIC_MASK: u32 = (3 << 0);
pub const M98090_EXTMIC_SHIFT: u32 = 0;
pub const M98090_EXTMIC1_SHIFT: u32 = 0;
pub const M98090_EXTMIC2_SHIFT: u32 = 1;
pub const M98090_EXTMIC_WIDTH: u32 = 2;
pub const M98090_EXTMIC_NONE: u32 = (0 << 0);
pub const M98090_EXTMIC_MIC1: u32 = (1 << 0);
pub const M98090_EXTMIC_MIC2: u32 = (2 << 0);

/*
 * M98090_REG_MIC1_INPUT_LEVEL
 */
pub const M98090_MIC_PA1EN_MASK: u32 = (3 << 5);
pub const M98090_MIC_PA1EN_SHIFT: u32 = 5;
pub const M98090_MIC_PA1EN_WIDTH: u32 = 2;
pub const M98090_MIC_PA1EN_NUM: u32 = 3;
pub const M98090_MIC_PGAM1_MASK: u32 = (31 << 0);
pub const M98090_MIC_PGAM1_SHIFT: u32 = 0;
pub const M98090_MIC_PGAM1_WIDTH: u32 = 5;
pub const M98090_MIC_PGAM1_NUM: u32 = 21;

/*
 * M98090_REG_MIC2_INPUT_LEVEL
 */
pub const M98090_MIC_PA2EN_MASK: u32 = (3 << 5);
pub const M98090_MIC_PA2EN_SHIFT: u32 = 5;
pub const M98090_MIC_PA2EN_WIDTH: u32 = 2;
pub const M98090_MIC_PA2EN_NUM: u32 = 3;
pub const M98090_MIC_PGAM2_MASK: u32 = (31 << 0);
pub const M98090_MIC_PGAM2_SHIFT: u32 = 0;
pub const M98090_MIC_PGAM2_WIDTH: u32 = 5;
pub const M98090_MIC_PGAM2_NUM: u32 = 21;

/*
 * M98090_REG_MIC_BIAS_VOLTAGE
 */
pub const M98090_MBVSEL_MASK: u32 = (3 << 0);
pub const M98090_MBVSEL_SHIFT: u32 = 0;
pub const M98090_MBVSEL_WIDTH: u32 = 2;
pub const M98090_MBVSEL_2V8: u32 = (3 << 0);
pub const M98090_MBVSEL_2V55: u32 = (2 << 0);
pub const M98090_MBVSEL_2V4: u32 = (1 << 0);
pub const M98090_MBVSEL_2V2: u32 = (0 << 0);

/*
 * M98090_REG_DIGITAL_MIC_ENABLE
 */
pub const M98090_MICCLK_MASK: u32 = (7 << 4);
pub const M98090_MICCLK_SHIFT: u32 = 4;
pub const M98090_MICCLK_WIDTH: u32 = 3;
pub const M98090_DIGMIC4_MASK: u32 = (1 << 3);
pub const M98090_DIGMIC4_SHIFT: u32 = 3;
pub const M98090_DIGMIC4_WIDTH: u32 = 1;
pub const M98090_DIGMIC4_NUM: u32 = (1 << M98090_DIGMIC4_WIDTH);
pub const M98090_DIGMIC3_MASK: u32 = (1 << 2);
pub const M98090_DIGMIC3_SHIFT: u32 = 2;
pub const M98090_DIGMIC3_WIDTH: u32 = 1;
pub const M98090_DIGMIC3_NUM: u32 = (1 << M98090_DIGMIC3_WIDTH);
pub const M98090_DIGMICR_MASK: u32 = (1 << 1);
pub const M98090_DIGMICR_SHIFT: u32 = 1;
pub const M98090_DIGMICR_WIDTH: u32 = 1;
pub const M98090_DIGMICR_NUM: u32 = (1 << M98090_DIGMICR_WIDTH);
pub const M98090_DIGMICL_MASK: u32 = (1 << 0);
pub const M98090_DIGMICL_SHIFT: u32 = 0;
pub const M98090_DIGMICL_WIDTH: u32 = 1;
pub const M98090_DIGMICL_NUM: u32 = (1 << M98090_DIGMICL_WIDTH);

/*
 * M98090_REG_DIGITAL_MIC_CONFIG
 */
pub const M98090_DMIC_COMP_MASK: u32 = (15 << 4);
pub const M98090_DMIC_COMP_SHIFT: u32 = 4;
pub const M98090_DMIC_COMP_WIDTH: u32 = 4;
pub const M98090_DMIC_COMP_NUM: u32 = (1 << M98090_DMIC_COMP_WIDTH);
pub const M98090_DMIC_FREQ_MASK: u32 = (3 << 0);
pub const M98090_DMIC_FREQ_SHIFT: u32 = 0;
pub const M98090_DMIC_FREQ_WIDTH: u32 = 2;

/*
 * M98090_REG_LEFT_ADC_MIXER
 */
pub const M98090_MIXADL_MIC2_MASK: u32 = (1 << 6);
pub const M98090_MIXADL_MIC2_SHIFT: u32 = 6;
pub const M98090_MIXADL_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXADL_MIC1_MASK: u32 = (1 << 5);
pub const M98090_MIXADL_MIC1_SHIFT: u32 = 5;
pub const M98090_MIXADL_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXADL_LINEB_MASK: u32 = (1 << 4);
pub const M98090_MIXADL_LINEB_SHIFT: u32 = 4;
pub const M98090_MIXADL_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXADL_LINEA_MASK: u32 = (1 << 3);
pub const M98090_MIXADL_LINEA_SHIFT: u32 = 3;
pub const M98090_MIXADL_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXADL_IN65DIFF_MASK: u32 = (1 << 2);
pub const M98090_MIXADL_IN65DIFF_SHIFT: u32 = 2;
pub const M98090_MIXADL_IN65DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADL_IN34DIFF_MASK: u32 = (1 << 1);
pub const M98090_MIXADL_IN34DIFF_SHIFT: u32 = 1;
pub const M98090_MIXADL_IN34DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADL_IN12DIFF_MASK: u32 = (1 << 0);
pub const M98090_MIXADL_IN12DIFF_SHIFT: u32 = 0;
pub const M98090_MIXADL_IN12DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADL_MASK: u32 = (255 << 0);
pub const M98090_MIXADL_SHIFT: u32 = 0;
pub const M98090_MIXADL_WIDTH: u32 = 8;

/*
 * M98090_REG_RIGHT_ADC_MIXER
 */
pub const M98090_MIXADR_MIC2_MASK: u32 = (1 << 6);
pub const M98090_MIXADR_MIC2_SHIFT: u32 = 6;
pub const M98090_MIXADR_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXADR_MIC1_MASK: u32 = (1 << 5);
pub const M98090_MIXADR_MIC1_SHIFT: u32 = 5;
pub const M98090_MIXADR_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXADR_LINEB_MASK: u32 = (1 << 4);
pub const M98090_MIXADR_LINEB_SHIFT: u32 = 4;
pub const M98090_MIXADR_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXADR_LINEA_MASK: u32 = (1 << 3);
pub const M98090_MIXADR_LINEA_SHIFT: u32 = 3;
pub const M98090_MIXADR_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXADR_IN65DIFF_MASK: u32 = (1 << 2);
pub const M98090_MIXADR_IN65DIFF_SHIFT: u32 = 2;
pub const M98090_MIXADR_IN65DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADR_IN34DIFF_MASK: u32 = (1 << 1);
pub const M98090_MIXADR_IN34DIFF_SHIFT: u32 = 1;
pub const M98090_MIXADR_IN34DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADR_IN12DIFF_MASK: u32 = (1 << 0);
pub const M98090_MIXADR_IN12DIFF_SHIFT: u32 = 0;
pub const M98090_MIXADR_IN12DIFF_WIDTH: u32 = 1;
pub const M98090_MIXADR_MASK: u32 = (255 << 0);
pub const M98090_MIXADR_SHIFT: u32 = 0;
pub const M98090_MIXADR_WIDTH: u32 = 8;

/*
 * M98090_REG_LEFT_ADC_LEVEL
 */
pub const M98090_AVLG_MASK: u32 = (7 << 4);
pub const M98090_AVLG_SHIFT: u32 = 4;
pub const M98090_AVLG_WIDTH: u32 = 3;
pub const M98090_AVLG_NUM: u32 = (1 << M98090_AVLG_WIDTH);
pub const M98090_AVL_MASK: u32 = (15 << 0);
pub const M98090_AVL_SHIFT: u32 = 0;
pub const M98090_AVL_WIDTH: u32 = 4;
pub const M98090_AVL_NUM: u32 = (1 << M98090_AVL_WIDTH);

/*
 * M98090_REG_RIGHT_ADC_LEVEL
 */
pub const M98090_AVRG_MASK: u32 = (7 << 4);
pub const M98090_AVRG_SHIFT: u32 = 4;
pub const M98090_AVRG_WIDTH: u32 = 3;
pub const M98090_AVRG_NUM: u32 = (1 << M98090_AVRG_WIDTH);
pub const M98090_AVR_MASK: u32 = (15 << 0);
pub const M98090_AVR_SHIFT: u32 = 0;
pub const M98090_AVR_WIDTH: u32 = 4;
pub const M98090_AVR_NUM: u32 = (1 << M98090_AVR_WIDTH);

/*
 * M98090_REG_ADC_BIQUAD_LEVEL
 */
pub const M98090_AVBQ_MASK: u32 = (15 << 0);
pub const M98090_AVBQ_SHIFT: u32 = 0;
pub const M98090_AVBQ_WIDTH: u32 = 4;
pub const M98090_AVBQ_NUM: u32 = (1 << M98090_AVBQ_WIDTH);

/*
 * M98090_REG_ADC_SIDETONE
 */
pub const M98090_DSTSR_MASK: u32 = (1 << 7);
pub const M98090_DSTSR_SHIFT: u32 = 7;
pub const M98090_DSTSR_WIDTH: u32 = 1;
pub const M98090_DSTSL_MASK: u32 = (1 << 6);
pub const M98090_DSTSL_SHIFT: u32 = 6;
pub const M98090_DSTSL_WIDTH: u32 = 1;
pub const M98090_DVST_MASK: u32 = (31 << 0);
pub const M98090_DVST_SHIFT: u32 = 0;
pub const M98090_DVST_WIDTH: u32 = 5;
pub const M98090_DVST_NUM: u32 = 31;

/*
 * M98090_REG_SYSTEM_CLOCK
 */
pub const M98090_PSCLK_MASK: u32 = (3 << 4);
pub const M98090_PSCLK_SHIFT: u32 = 4;
pub const M98090_PSCLK_WIDTH: u32 = 2;
pub const M98090_PSCLK_DISABLED: u32 = (0 << 4);
pub const M98090_PSCLK_DIV1: u32 = (1 << 4);
pub const M98090_PSCLK_DIV2: u32 = (2 << 4);
pub const M98090_PSCLK_DIV4: u32 = (3 << 4);

/*
 * M98090_REG_CLOCK_MODE
 */
pub const M98090_FREQ_MASK: u32 = (15 << 4);
pub const M98090_FREQ_SHIFT: u32 = 4;
pub const M98090_FREQ_WIDTH: u32 = 4;
pub const M98090_USE_M1_MASK: u32 = (1 << 0);
pub const M98090_USE_M1_SHIFT: u32 = 0;
pub const M98090_USE_M1_WIDTH: u32 = 1;
pub const M98090_USE_M1_NUM: u32 = (1 << M98090_USE_M1_WIDTH);

/*
 * M98090_REG_CLOCK_RATIO_NI_MSB
 */
pub const M98090_NI_HI_MASK: u32 = (127 << 0);
pub const M98090_NI_HI_SHIFT: u32 = 0;
pub const M98090_NI_HI_WIDTH: u32 = 7;
pub const M98090_NI_HI_NUM: u32 = (1 << M98090_NI_HI_WIDTH);

/*
 * M98090_REG_CLOCK_RATIO_NI_LSB
 */
pub const M98090_NI_LO_MASK: u32 = (255 << 0);
pub const M98090_NI_LO_SHIFT: u32 = 0;
pub const M98090_NI_LO_WIDTH: u32 = 8;
pub const M98090_NI_LO_NUM: u32 = (1 << M98090_NI_LO_WIDTH);

/*
 * M98090_REG_CLOCK_RATIO_MI_MSB
 */
pub const M98090_MI_HI_MASK: u32 = (255 << 0);
pub const M98090_MI_HI_SHIFT: u32 = 0;
pub const M98090_MI_HI_WIDTH: u32 = 8;
pub const M98090_MI_HI_NUM: u32 = (1 << M98090_MI_HI_WIDTH);

/*
 * M98090_REG_CLOCK_RATIO_MI_LSB
 */
pub const M98090_MI_LO_MASK: u32 = (255 << 0);
pub const M98090_MI_LO_SHIFT: u32 = 0;
pub const M98090_MI_LO_WIDTH: u32 = 8;
pub const M98090_MI_LO_NUM: u32 = (1 << M98090_MI_LO_WIDTH);

/*
 * M98090_REG_MASTER_MODE
 */
pub const M98090_MAS_MASK: u32 = (1 << 7);
pub const M98090_MAS_SHIFT: u32 = 7;
pub const M98090_MAS_WIDTH: u32 = 1;
pub const M98090_BSEL_MASK: u32 = (1 << 0);
pub const M98090_BSEL_SHIFT: u32 = 0;
pub const M98090_BSEL_WIDTH: u32 = 1;
pub const M98090_BSEL_32: u32 = (1 << 0);
pub const M98090_BSEL_48: u32 = (2 << 0);
pub const M98090_BSEL_64: u32 = (3 << 0);

/*
 * M98090_REG_INTERFACE_FORMAT
 */
pub const M98090_RJ_MASK: u32 = (1 << 5);
pub const M98090_RJ_SHIFT: u32 = 5;
pub const M98090_RJ_WIDTH: u32 = 1;
pub const M98090_WCI_MASK: u32 = (1 << 4);
pub const M98090_WCI_SHIFT: u32 = 4;
pub const M98090_WCI_WIDTH: u32 = 1;
pub const M98090_BCI_MASK: u32 = (1 << 3);
pub const M98090_BCI_SHIFT: u32 = 3;
pub const M98090_BCI_WIDTH: u32 = 1;
pub const M98090_DLY_MASK: u32 = (1 << 2);
pub const M98090_DLY_SHIFT: u32 = 2;
pub const M98090_DLY_WIDTH: u32 = 1;
pub const M98090_WS_MASK: u32 = (3 << 0);
pub const M98090_WS_SHIFT: u32 = 0;
pub const M98090_WS_WIDTH: u32 = 2;
pub const M98090_WS_NUM: u32 = (1 << M98090_WS_WIDTH);

/*
 * M98090_REG_TDM_CONTROL
 */
pub const M98090_FSW_MASK: u32 = (1 << 1);
pub const M98090_FSW_SHIFT: u32 = 1;
pub const M98090_FSW_WIDTH: u32 = 1;
pub const M98090_TDM_MASK: u32 = (1 << 0);
pub const M98090_TDM_SHIFT: u32 = 0;
pub const M98090_TDM_WIDTH: u32 = 1;
pub const M98090_TDM_NUM: u32 = (1 << M98090_TDM_WIDTH);

/*
 * M98090_REG_TDM_FORMAT
 */
pub const M98090_TDM_SLOTL_MASK: u32 = (3 << 6);
pub const M98090_TDM_SLOTL_SHIFT: u32 = 6;
pub const M98090_TDM_SLOTL_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTL_NUM: u32 = (1 << M98090_TDM_SLOTL_WIDTH);
pub const M98090_TDM_SLOTR_MASK: u32 = (3 << 4);
pub const M98090_TDM_SLOTR_SHIFT: u32 = 4;
pub const M98090_TDM_SLOTR_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTR_NUM: u32 = (1 << M98090_TDM_SLOTR_WIDTH);
pub const M98090_TDM_SLOTDLY_MASK: u32 = (15 << 0);
pub const M98090_TDM_SLOTDLY_SHIFT: u32 = 0;
pub const M98090_TDM_SLOTDLY_WIDTH: u32 = 4;
pub const M98090_TDM_SLOTDLY_NUM: u32 = (1 << M98090_TDM_SLOTDLY_WIDTH);

/*
 * M98090_REG_IO_CONFIGURATION
 */
pub const M98090_LTEN_MASK: u32 = (1 << 5);
pub const M98090_LTEN_SHIFT: u32 = 5;
pub const M98090_LTEN_WIDTH: u32 = 1;
pub const M98090_LTEN_NUM: u32 = (1 << M98090_LTEN_WIDTH);
pub const M98090_LBEN_MASK: u32 = (1 << 4);
pub const M98090_LBEN_SHIFT: u32 = 4;
pub const M98090_LBEN_WIDTH: u32 = 1;
pub const M98090_LBEN_NUM: u32 = (1 << M98090_LBEN_WIDTH);
pub const M98090_DMONO_MASK: u32 = (1 << 3);
pub const M98090_DMONO_SHIFT: u32 = 3;
pub const M98090_DMONO_WIDTH: u32 = 1;
pub const M98090_DMONO_NUM: u32 = (1 << M98090_DMONO_WIDTH);
pub const M98090_HIZOFF_MASK: u32 = (1 << 2);
pub const M98090_HIZOFF_SHIFT: u32 = 2;
pub const M98090_HIZOFF_WIDTH: u32 = 1;
pub const M98090_HIZOFF_NUM: u32 = (1 << M98090_HIZOFF_WIDTH);
pub const M98090_SDOEN_MASK: u32 = (1 << 1);
pub const M98090_SDOEN_SHIFT: u32 = 1;
pub const M98090_SDOEN_WIDTH: u32 = 1;
pub const M98090_SDOEN_NUM: u32 = (1 << M98090_SDOEN_WIDTH);
pub const M98090_SDIEN_MASK: u32 = (1 << 0);
pub const M98090_SDIEN_SHIFT: u32 = 0;
pub const M98090_SDIEN_WIDTH: u32 = 1;
pub const M98090_SDIEN_NUM: u32 = (1 << M98090_SDIEN_WIDTH);

/*
 * M98090_REG_FILTER_CONFIG
 */
pub const M98090_MODE_MASK: u32 = (1 << 7);
pub const M98090_MODE_SHIFT: u32 = 7;
pub const M98090_MODE_WIDTH: u32 = 1;
pub const M98090_AHPF_MASK: u32 = (1 << 6);
pub const M98090_AHPF_SHIFT: u32 = 6;
pub const M98090_AHPF_WIDTH: u32 = 1;
pub const M98090_AHPF_NUM: u32 = (1 << M98090_AHPF_WIDTH);
pub const M98090_DHPF_MASK: u32 = (1 << 5);
pub const M98090_DHPF_SHIFT: u32 = 5;
pub const M98090_DHPF_WIDTH: u32 = 1;
pub const M98090_DHPF_NUM: u32 = (1 << M98090_DHPF_WIDTH);
pub const M98090_DHF_MASK: u32 = (1 << 4);
pub const M98090_DHF_SHIFT: u32 = 4;
pub const M98090_DHF_WIDTH: u32 = 1;
pub const M98090_FLT_DMIC34MODE_MASK: u32 = (1 << 3);
pub const M98090_FLT_DMIC34MODE_SHIFT: u32 = 3;
pub const M98090_FLT_DMIC34MODE_WIDTH: u32 = 1;
pub const M98090_FLT_DMIC34HPF_MASK: u32 = (1 << 2);
pub const M98090_FLT_DMIC34HPF_SHIFT: u32 = 2;
pub const M98090_FLT_DMIC34HPF_WIDTH: u32 = 1;
pub const M98090_FLT_DMIC34HPF_NUM: u32 = (1 << M98090_FLT_DMIC34HPF_WIDTH);

/*
 * M98090_REG_DAI_PLAYBACK_LEVEL
 */
pub const M98090_DVM_MASK: u32 = (1 << 7);
pub const M98090_DVM_SHIFT: u32 = 7;
pub const M98090_DVM_WIDTH: u32 = 1;
pub const M98090_DVG_MASK: u32 = (3 << 4);
pub const M98090_DVG_SHIFT: u32 = 4;
pub const M98090_DVG_WIDTH: u32 = 2;
pub const M98090_DVG_NUM: u32 = (1 << M98090_DVG_WIDTH);
pub const M98090_DV_MASK: u32 = (15 << 0);
pub const M98090_DV_SHIFT: u32 = 0;
pub const M98090_DV_WIDTH: u32 = 4;
pub const M98090_DV_NUM: u32 = (1 << M98090_DV_WIDTH);

/*
 * M98090_REG_DAI_PLAYBACK_LEVEL_EQ
 */
pub const M98090_EQCLPN_MASK: u32 = (1 << 4);
pub const M98090_EQCLPN_SHIFT: u32 = 4;
pub const M98090_EQCLPN_WIDTH: u32 = 1;
pub const M98090_EQCLPN_NUM: u32 = (1 << M98090_EQCLPN_WIDTH);
pub const M98090_DVEQ_MASK: u32 = (15 << 0);
pub const M98090_DVEQ_SHIFT: u32 = 0;
pub const M98090_DVEQ_WIDTH: u32 = 4;
pub const M98090_DVEQ_NUM: u32 = (1 << M98090_DVEQ_WIDTH);

/*
 * M98090_REG_LEFT_HP_MIXER
 */
pub const M98090_MIXHPL_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXHPL_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXHPL_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXHPL_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXHPL_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXHPL_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXHPL_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXHPL_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXHPL_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXHPL_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXHPL_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXHPL_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXHPL_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXHPL_DACR_SHIFT: u32 = 1;
pub const M98090_MIXHPL_DACR_WIDTH: u32 = 1;
pub const M98090_MIXHPL_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXHPL_DACL_SHIFT: u32 = 0;
pub const M98090_MIXHPL_DACL_WIDTH: u32 = 1;
pub const M98090_MIXHPL_MASK: u32 = (63 << 0);
pub const M98090_MIXHPL_SHIFT: u32 = 0;
pub const M98090_MIXHPL_WIDTH: u32 = 6;

/*
 * M98090_REG_RIGHT_HP_MIXER
 */
pub const M98090_MIXHPR_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXHPR_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXHPR_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXHPR_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXHPR_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXHPR_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXHPR_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXHPR_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXHPR_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXHPR_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXHPR_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXHPR_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXHPR_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXHPR_DACR_SHIFT: u32 = 1;
pub const M98090_MIXHPR_DACR_WIDTH: u32 = 1;
pub const M98090_MIXHPR_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXHPR_DACL_SHIFT: u32 = 0;
pub const M98090_MIXHPR_DACL_WIDTH: u32 = 1;
pub const M98090_MIXHPR_MASK: u32 = (63 << 0);
pub const M98090_MIXHPR_SHIFT: u32 = 0;
pub const M98090_MIXHPR_WIDTH: u32 = 6;

/*
 * M98090_REG_HP_CONTROL
 */
pub const M98090_MIXHPRSEL_MASK: u32 = (1 << 5);
pub const M98090_MIXHPRSEL_SHIFT: u32 = 5;
pub const M98090_MIXHPRSEL_WIDTH: u32 = 1;
pub const M98090_MIXHPLSEL_MASK: u32 = (1 << 4);
pub const M98090_MIXHPLSEL_SHIFT: u32 = 4;
pub const M98090_MIXHPLSEL_WIDTH: u32 = 1;
pub const M98090_MIXHPRG_MASK: u32 = (3 << 2);
pub const M98090_MIXHPRG_SHIFT: u32 = 2;
pub const M98090_MIXHPRG_WIDTH: u32 = 2;
pub const M98090_MIXHPRG_NUM: u32 = (1 << M98090_MIXHPRG_WIDTH);
pub const M98090_MIXHPLG_MASK: u32 = (3 << 0);
pub const M98090_MIXHPLG_SHIFT: u32 = 0;
pub const M98090_MIXHPLG_WIDTH: u32 = 2;
pub const M98090_MIXHPLG_NUM: u32 = (1 << M98090_MIXHPLG_WIDTH);

/*
 * M98090_REG_LEFT_HP_VOLUME
 */
pub const M98090_HPLM_MASK: u32 = (1 << 7);
pub const M98090_HPLM_SHIFT: u32 = 7;
pub const M98090_HPLM_WIDTH: u32 = 1;
pub const M98090_HPVOLL_MASK: u32 = (31 << 0);
pub const M98090_HPVOLL_SHIFT: u32 = 0;
pub const M98090_HPVOLL_WIDTH: u32 = 5;
pub const M98090_HPVOLL_NUM: u32 = (1 << M98090_HPVOLL_WIDTH);

/*
 * M98090_REG_RIGHT_HP_VOLUME
 */
pub const M98090_HPRM_MASK: u32 = (1 << 7);
pub const M98090_HPRM_SHIFT: u32 = 7;
pub const M98090_HPRM_WIDTH: u32 = 1;
pub const M98090_HPVOLR_MASK: u32 = (31 << 0);
pub const M98090_HPVOLR_SHIFT: u32 = 0;
pub const M98090_HPVOLR_WIDTH: u32 = 5;
pub const M98090_HPVOLR_NUM: u32 = (1 << M98090_HPVOLR_WIDTH);

/*
 * M98090_REG_LEFT_SPK_MIXER
 */
pub const M98090_MIXSPL_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXSPL_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXSPL_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXSPL_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXSPL_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXSPL_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXSPL_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXSPL_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXSPL_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXSPL_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXSPL_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXSPL_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXSPL_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXSPL_DACR_SHIFT: u32 = 1;
pub const M98090_MIXSPL_DACR_WIDTH: u32 = 1;
pub const M98090_MIXSPL_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXSPL_DACL_SHIFT: u32 = 0;
pub const M98090_MIXSPL_DACL_WIDTH: u32 = 1;
pub const M98090_MIXSPL_MASK: u32 = (63 << 0);
pub const M98090_MIXSPL_SHIFT: u32 = 0;
pub const M98090_MIXSPL_WIDTH: u32 = 6;
pub const M98090_MIXSPR_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXSPR_DACR_SHIFT: u32 = 1;
pub const M98090_MIXSPR_DACR_WIDTH: u32 = 1;


/*
 * M98090_REG_RIGHT_SPK_MIXER
 */
pub const M98090_SPK_SLAVE_MASK: u32 = (1 << 6);
pub const M98090_SPK_SLAVE_SHIFT: u32 = 6;
pub const M98090_SPK_SLAVE_WIDTH: u32 = 1;
pub const M98090_MIXSPR_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXSPR_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXSPR_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXSPR_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXSPR_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXSPR_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXSPR_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXSPR_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXSPR_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXSPR_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXSPR_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXSPR_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXSPR_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXSPR_DACR_SHIFT: u32 = 1;
pub const M98090_MIXSPR_DACR_WIDTH: u32 = 1;
pub const M98090_MIXSPR_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXSPR_DACL_SHIFT: u32 = 0;
pub const M98090_MIXSPR_DACL_WIDTH: u32 = 1;
pub const M98090_MIXSPR_MASK: u32 = (63 << 0);
pub const M98090_MIXSPR_SHIFT: u32 = 0;
pub const M98090_MIXSPR_WIDTH: u32 = 6;

/*
 * M98090_REG_SPK_CONTROL
 */
pub const M98090_MIXSPRG_MASK: u32 = (3 << 2);
pub const M98090_MIXSPRG_SHIFT: u32 = 2;
pub const M98090_MIXSPRG_WIDTH: u32 = 2;
pub const M98090_MIXSPRG_NUM: u32 = (1 << M98090_MIXSPRG_WIDTH);
pub const M98090_MIXSPLG_MASK: u32 = (3 << 0);
pub const M98090_MIXSPLG_SHIFT: u32 = 0;
pub const M98090_MIXSPLG_WIDTH: u32 = 2;
pub const M98090_MIXSPLG_NUM: u32 = (1 << M98090_MIXSPLG_WIDTH);

/*
 * M98090_REG_LEFT_SPK_VOLUME
 */
pub const M98090_SPLM_MASK: u32 = (1 << 7);
pub const M98090_SPLM_SHIFT: u32 = 7;
pub const M98090_SPLM_WIDTH: u32 = 1;
pub const M98090_SPVOLL_MASK: u32 = (63 << 0);
pub const M98090_SPVOLL_SHIFT: u32 = 0;
pub const M98090_SPVOLL_WIDTH: u32 = 6;
pub const M98090_SPVOLL_NUM: u32 = 40;

/*
 * M98090_REG_RIGHT_SPK_VOLUME
 */
pub const M98090_SPRM_MASK: u32 = (1 << 7);
pub const M98090_SPRM_SHIFT: u32 = 7;
pub const M98090_SPRM_WIDTH: u32 = 1;
pub const M98090_SPVOLR_MASK: u32 = (63 << 0);
pub const M98090_SPVOLR_SHIFT: u32 = 0;
pub const M98090_SPVOLR_WIDTH: u32 = 6;
pub const M98090_SPVOLR_NUM: u32 = 40;

/*
 * M98090_REG_DRC_TIMING
 */
pub const M98090_DRCEN_MASK: u32 = (1 << 7);
pub const M98090_DRCEN_SHIFT: u32 = 7;
pub const M98090_DRCEN_WIDTH: u32 = 1;
pub const M98090_DRCEN_NUM: u32 = (1 << M98090_DRCEN_WIDTH);
pub const M98090_DRCRLS_MASK: u32 = (7 << 4);
pub const M98090_DRCRLS_SHIFT: u32 = 4;
pub const M98090_DRCRLS_WIDTH: u32 = 3;
pub const M98090_DRCATK_MASK: u32 = (7 << 0);
pub const M98090_DRCATK_SHIFT: u32 = 0;
pub const M98090_DRCATK_WIDTH: u32 = 3;

/*
 * M98090_REG_DRC_COMPRESSOR
 */
pub const M98090_DRCCMP_MASK: u32 = (7 << 5);
pub const M98090_DRCCMP_SHIFT: u32 = 5;
pub const M98090_DRCCMP_WIDTH: u32 = 3;
pub const M98090_DRCTHC_MASK: u32 = (31 << 0);
pub const M98090_DRCTHC_SHIFT: u32 = 0;
pub const M98090_DRCTHC_WIDTH: u32 = 5;
pub const M98090_DRCTHC_NUM: u32 = (1 << M98090_DRCTHC_WIDTH);

/*
 * M98090_REG_DRC_EXPANDER
 */
pub const M98090_DRCEXP_MASK: u32 = (7 << 5);
pub const M98090_DRCEXP_SHIFT: u32 = 5;
pub const M98090_DRCEXP_WIDTH: u32 = 3;
pub const M98090_DRCTHE_MASK: u32 = (31 << 0);
pub const M98090_DRCTHE_SHIFT: u32 = 0;
pub const M98090_DRCTHE_WIDTH: u32 = 5;
pub const M98090_DRCTHE_NUM: u32 = (1 << M98090_DRCTHE_WIDTH);

/*
 * M98090_REG_DRC_GAIN
 */
pub const M98090_DRCG_MASK: u32 = (31 << 0);
pub const M98090_DRCG_SHIFT: u32 = 0;
pub const M98090_DRCG_WIDTH: u32 = 5;
pub const M98090_DRCG_NUM: u32 = 13;

/*
 * M98090_REG_RCV_LOUTL_MIXER
 */
pub const M98090_MIXRCVL_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXRCVL_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXRCVL_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXRCVL_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXRCVL_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXRCVL_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXRCVL_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXRCVL_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXRCVL_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXRCVL_DACR_SHIFT: u32 = 1;
pub const M98090_MIXRCVL_DACR_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXRCVL_DACL_SHIFT: u32 = 0;
pub const M98090_MIXRCVL_DACL_WIDTH: u32 = 1;
pub const M98090_MIXRCVL_MASK: u32 = (63 << 0);
pub const M98090_MIXRCVL_SHIFT: u32 = 0;
pub const M98090_MIXRCVL_WIDTH: u32 = 6;

/*
 * M98090_REG_RCV_LOUTL_CONTROL
 */
pub const M98090_MIXRCVLG_MASK: u32 = (3 << 0);
pub const M98090_MIXRCVLG_SHIFT: u32 = 0;
pub const M98090_MIXRCVLG_WIDTH: u32 = 2;
pub const M98090_MIXRCVLG_NUM: u32 = (1 << M98090_MIXRCVLG_WIDTH);

/*
 * M98090_REG_RCV_LOUTL_VOLUME
 */
pub const M98090_RCVLM_MASK: u32 = (1 << 7);
pub const M98090_RCVLM_SHIFT: u32 = 7;
pub const M98090_RCVLM_WIDTH: u32 = 1;
pub const M98090_RCVLVOL_MASK: u32 = (31 << 0);
pub const M98090_RCVLVOL_SHIFT: u32 = 0;
pub const M98090_RCVLVOL_WIDTH: u32 = 5;
pub const M98090_RCVLVOL_NUM: u32 = (1 << M98090_RCVLVOL_WIDTH);

/*
 * M98090_REG_LOUTR_MIXER
 */
pub const M98090_LINMOD_MASK: u32 = (1 << 7);
pub const M98090_LINMOD_SHIFT: u32 = 7;
pub const M98090_LINMOD_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_MIC2_MASK: u32 = (1 << 5);
pub const M98090_MIXRCVR_MIC2_SHIFT: u32 = 5;
pub const M98090_MIXRCVR_MIC2_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_MIC1_MASK: u32 = (1 << 4);
pub const M98090_MIXRCVR_MIC1_SHIFT: u32 = 4;
pub const M98090_MIXRCVR_MIC1_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_LINEB_MASK: u32 = (1 << 3);
pub const M98090_MIXRCVR_LINEB_SHIFT: u32 = 3;
pub const M98090_MIXRCVR_LINEB_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_LINEA_MASK: u32 = (1 << 2);
pub const M98090_MIXRCVR_LINEA_SHIFT: u32 = 2;
pub const M98090_MIXRCVR_LINEA_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_DACR_MASK: u32 = (1 << 1);
pub const M98090_MIXRCVR_DACR_SHIFT: u32 = 1;
pub const M98090_MIXRCVR_DACR_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_DACL_MASK: u32 = (1 << 0);
pub const M98090_MIXRCVR_DACL_SHIFT: u32 = 0;
pub const M98090_MIXRCVR_DACL_WIDTH: u32 = 1;
pub const M98090_MIXRCVR_MASK: u32 = (63 << 0);
pub const M98090_MIXRCVR_SHIFT: u32 = 0;
pub const M98090_MIXRCVR_WIDTH: u32 = 6;

/*
 * M98090_REG_LOUTR_CONTROL
 */
pub const M98090_MIXRCVRG_MASK: u32 = (3 << 0);
pub const M98090_MIXRCVRG_SHIFT: u32 = 0;
pub const M98090_MIXRCVRG_WIDTH: u32 = 2;
pub const M98090_MIXRCVRG_NUM: u32 = (1 << M98090_MIXRCVRG_WIDTH);

/*
 * M98090_REG_LOUTR_VOLUME
 */
pub const M98090_RCVRM_MASK: u32 = (1 << 7);
pub const M98090_RCVRM_SHIFT: u32 = 7;
pub const M98090_RCVRM_WIDTH: u32 = 1;
pub const M98090_RCVRVOL_MASK: u32 = (31 << 0);
pub const M98090_RCVRVOL_SHIFT: u32 = 0;
pub const M98090_RCVRVOL_WIDTH: u32 = 5;
pub const M98090_RCVRVOL_NUM: u32 = (1 << M98090_RCVRVOL_WIDTH);

/*
 * M98090_REG_JACK_DETECT
 */
pub const M98090_JDETEN_MASK: u32 = (1 << 7);
pub const M98090_JDETEN_SHIFT: u32 = 7;
pub const M98090_JDETEN_WIDTH: u32 = 1;
pub const M98090_JDWK_MASK: u32 = (1 << 6);
pub const M98090_JDWK_SHIFT: u32 = 6;
pub const M98090_JDWK_WIDTH: u32 = 1;
pub const M98090_JDEB_MASK: u32 = (3 << 0);
pub const M98090_JDEB_SHIFT: u32 = 0;
pub const M98090_JDEB_WIDTH: u32 = 2;
pub const M98090_JDEB_25MS: u32 = (0 << 0);
pub const M98090_JDEB_50MS: u32 = (1 << 0);
pub const M98090_JDEB_100MS: u32 = (2 << 0);
pub const M98090_JDEB_200MS: u32 = (3 << 0);

/*
 * M98090_REG_INPUT_ENABLE
 */
pub const M98090_MBEN_MASK: u32 = (1 << 4);
pub const M98090_MBEN_SHIFT: u32 = 4;
pub const M98090_MBEN_WIDTH: u32 = 1;
pub const M98090_LINEAEN_MASK: u32 = (1 << 3);
pub const M98090_LINEAEN_SHIFT: u32 = 3;
pub const M98090_LINEAEN_WIDTH: u32 = 1;
pub const M98090_LINEBEN_MASK: u32 = (1 << 2);
pub const M98090_LINEBEN_SHIFT: u32 = 2;
pub const M98090_LINEBEN_WIDTH: u32 = 1;
pub const M98090_ADREN_MASK: u32 = (1 << 1);
pub const M98090_ADREN_SHIFT: u32 = 1;
pub const M98090_ADREN_WIDTH: u32 = 1;
pub const M98090_ADLEN_MASK: u32 = (1 << 0);
pub const M98090_ADLEN_SHIFT: u32 = 0;
pub const M98090_ADLEN_WIDTH: u32 = 1;

/*
 * M98090_REG_OUTPUT_ENABLE
 */
pub const M98090_HPREN_MASK: u32 = (1 << 7);
pub const M98090_HPREN_SHIFT: u32 = 7;
pub const M98090_HPREN_WIDTH: u32 = 1;
pub const M98090_HPLEN_MASK: u32 = (1 << 6);
pub const M98090_HPLEN_SHIFT: u32 = 6;
pub const M98090_HPLEN_WIDTH: u32 = 1;
pub const M98090_SPREN_MASK: u32 = (1 << 5);
pub const M98090_SPREN_SHIFT: u32 = 5;
pub const M98090_SPREN_WIDTH: u32 = 1;
pub const M98090_SPLEN_MASK: u32 = (1 << 4);
pub const M98090_SPLEN_SHIFT: u32 = 4;
pub const M98090_SPLEN_WIDTH: u32 = 1;
pub const M98090_RCVLEN_MASK: u32 = (1 << 3);
pub const M98090_RCVLEN_SHIFT: u32 = 3;
pub const M98090_RCVLEN_WIDTH: u32 = 1;
pub const M98090_RCVREN_MASK: u32 = (1 << 2);
pub const M98090_RCVREN_SHIFT: u32 = 2;
pub const M98090_RCVREN_WIDTH: u32 = 1;
pub const M98090_DAREN_MASK: u32 = (1 << 1);
pub const M98090_DAREN_SHIFT: u32 = 1;
pub const M98090_DAREN_WIDTH: u32 = 1;
pub const M98090_DALEN_MASK: u32 = (1 << 0);
pub const M98090_DALEN_SHIFT: u32 = 0;
pub const M98090_DALEN_WIDTH: u32 = 1;

/*
 * M98090_REG_LEVEL_CONTROL
 */
pub const M98090_ZDENN_MASK: u32 = (1 << 2);
pub const M98090_ZDENN_SHIFT: u32 = 2;
pub const M98090_ZDENN_WIDTH: u32 = 1;
pub const M98090_ZDENN_NUM: u32 = (1 << M98090_ZDENN_WIDTH);
pub const M98090_VS2ENN_MASK: u32 = (1 << 1);
pub const M98090_VS2ENN_SHIFT: u32 = 1;
pub const M98090_VS2ENN_WIDTH: u32 = 1;
pub const M98090_VS2ENN_NUM: u32 = (1 << M98090_VS2ENN_WIDTH);
pub const M98090_VSENN_MASK: u32 = (1 << 0);
pub const M98090_VSENN_SHIFT: u32 = 0;
pub const M98090_VSENN_WIDTH: u32 = 1;
pub const M98090_VSENN_NUM: u32 = (1 << M98090_VSENN_WIDTH);

/*
 * M98090_REG_DSP_FILTER_ENABLE
 */
pub const M98090_DMIC34BQEN_MASK: u32 = (1 << 4);
pub const M98090_DMIC34BQEN_SHIFT: u32 = 4;
pub const M98090_DMIC34BQEN_WIDTH: u32 = 1;
pub const M98090_DMIC34BQEN_NUM: u32 = (1 << M98090_DMIC34BQEN_WIDTH);
pub const M98090_ADCBQEN_MASK: u32 = (1 << 3);
pub const M98090_ADCBQEN_SHIFT: u32 = 3;
pub const M98090_ADCBQEN_WIDTH: u32 = 1;
pub const M98090_ADCBQEN_NUM: u32 = (1 << M98090_ADCBQEN_WIDTH);
pub const M98090_EQ3BANDEN_MASK: u32 = (1 << 2);
pub const M98090_EQ3BANDEN_SHIFT: u32 = 2;
pub const M98090_EQ3BANDEN_WIDTH: u32 = 1;
pub const M98090_EQ3BANDEN_NUM: u32 = (1 << M98090_EQ3BANDEN_WIDTH);
pub const M98090_EQ5BANDEN_MASK: u32 = (1 << 1);
pub const M98090_EQ5BANDEN_SHIFT: u32 = 1;
pub const M98090_EQ5BANDEN_WIDTH: u32 = 1;
pub const M98090_EQ5BANDEN_NUM: u32 = (1 << M98090_EQ5BANDEN_WIDTH);
pub const M98090_EQ7BANDEN_MASK: u32 = (1 << 0);
pub const M98090_EQ7BANDEN_SHIFT: u32 = 0;
pub const M98090_EQ7BANDEN_WIDTH: u32 = 1;
pub const M98090_EQ7BANDEN_NUM: u32 = (1 << M98090_EQ7BANDEN_WIDTH);

/*
 * M98090_REG_BIAS_CONTROL
 */
pub const M98090_VCM_MODE_MASK: u32 = (1 << 0);
pub const M98090_VCM_MODE_SHIFT: u32 = 0;
pub const M98090_VCM_MODE_WIDTH: u32 = 1;
pub const M98090_VCM_MODE_NUM: u32 = (1 << M98090_VCM_MODE_WIDTH);

/*
 * M98090_REG_DAC_CONTROL
 */
pub const M98090_PERFMODE_MASK: u32 = (1 << 1);
pub const M98090_PERFMODE_SHIFT: u32 = 1;
pub const M98090_PERFMODE_WIDTH: u32 = 1;
pub const M98090_PERFMODE_NUM: u32 = (1 << M98090_PERFMODE_WIDTH);
pub const M98090_DACHP_MASK: u32 = (1 << 0);
pub const M98090_DACHP_SHIFT: u32 = 0;
pub const M98090_DACHP_WIDTH: u32 = 1;
pub const M98090_DACHP_NUM: u32 = (1 << M98090_DACHP_WIDTH);

/*
 * M98090_REG_ADC_CONTROL
 */
pub const M98090_OSR128_MASK: u32 = (1 << 2);
pub const M98090_OSR128_SHIFT: u32 = 2;
pub const M98090_OSR128_WIDTH: u32 = 1;
pub const M98090_ADCDITHER_MASK: u32 = (1 << 1);
pub const M98090_ADCDITHER_SHIFT: u32 = 1;
pub const M98090_ADCDITHER_WIDTH: u32 = 1;
pub const M98090_ADCDITHER_NUM: u32 = (1 << M98090_ADCDITHER_WIDTH);
pub const M98090_ADCHP_MASK: u32 = (1 << 0);
pub const M98090_ADCHP_SHIFT: u32 = 0;
pub const M98090_ADCHP_WIDTH: u32 = 1;
pub const M98090_ADCHP_NUM: u32 = (1 << M98090_ADCHP_WIDTH);

/*
 * M98090_REG_DEVICE_SHUTDOWN
 */
pub const M98090_SHDNN_MASK: u32 = (1 << 7);
pub const M98090_SHDNN_SHIFT: u32 = 7;
pub const M98090_SHDNN_WIDTH: u32 = 1;

/*
 * M98090_REG_EQUALIZER_BASE
 */
pub const M98090_B0_1_HI_MASK: u32 = (255 << 0);
pub const M98090_B0_1_HI_SHIFT: u32 = 0;
pub const M98090_B0_1_HI_WIDTH: u32 = 8;
pub const M98090_B0_1_MID_MASK: u32 = (255 << 0);
pub const M98090_B0_1_MID_SHIFT: u32 = 0;
pub const M98090_B0_1_MID_WIDTH: u32 = 8;
pub const M98090_B0_1_LO_MASK: u32 = (255 << 0);
pub const M98090_B0_1_LO_SHIFT: u32 = 0;
pub const M98090_B0_1_LO_WIDTH: u32 = 8;
pub const M98090_B1_1_HI_MASK: u32 = (255 << 0);
pub const M98090_B1_1_HI_SHIFT: u32 = 0;
pub const M98090_B1_1_HI_WIDTH: u32 = 8;
pub const M98090_B1_1_MID_MASK: u32 = (255 << 0);
pub const M98090_B1_1_MID_SHIFT: u32 = 0;
pub const M98090_B1_1_MID_WIDTH: u32 = 8;
pub const M98090_B1_1_LO_MASK: u32 = (255 << 0);
pub const M98090_B1_1_LO_SHIFT: u32 = 0;
pub const M98090_B1_1_LO_WIDTH: u32 = 8;
pub const M98090_B2_1_HI_MASK: u32 = (255 << 0);
pub const M98090_B2_1_HI_SHIFT: u32 = 0;
pub const M98090_B2_1_HI_WIDTH: u32 = 8;
pub const M98090_B2_1_MID_MASK: u32 = (255 << 0);
pub const M98090_B2_1_MID_SHIFT: u32 = 0;
pub const M98090_B2_1_MID_WIDTH: u32 = 8;
pub const M98090_B2_1_LO_MASK: u32 = (255 << 0);
pub const M98090_B2_1_LO_SHIFT: u32 = 0;
pub const M98090_B2_1_LO_WIDTH: u32 = 8;
pub const M98090_A1_1_HI_MASK: u32 = (255 << 0);
pub const M98090_A1_1_HI_SHIFT: u32 = 0;
pub const M98090_A1_1_HI_WIDTH: u32 = 8;
pub const M98090_A1_1_MID_MASK: u32 = (255 << 0);
pub const M98090_A1_1_MID_SHIFT: u32 = 0;
pub const M98090_A1_1_MID_WIDTH: u32 = 8;
pub const M98090_A1_1_LO_MASK: u32 = (255 << 0);
pub const M98090_A1_1_LO_SHIFT: u32 = 0;
pub const M98090_A1_1_LO_WIDTH: u32 = 8;
pub const M98090_A2_1_HI_MASK: u32 = (255 << 0);
pub const M98090_A2_1_HI_SHIFT: u32 = 0;
pub const M98090_A2_1_HI_WIDTH: u32 = 8;
pub const M98090_A2_1_MID_MASK: u32 = (255 << 0);
pub const M98090_A2_1_MID_SHIFT: u32 = 0;
pub const M98090_A2_1_MID_WIDTH: u32 = 8;
pub const M98090_A2_1_LO_MASK: u32 = (255 << 0);
pub const M98090_A2_1_LO_SHIFT: u32 = 0;
pub const M98090_A2_1_LO_WIDTH: u32 = 8;

pub const M98090_COEFS_PER_BAND: u32 = 5;
pub const M98090_COEFS_BLK_SZ: u32 = (M98090_COEFS_PER_BAND * 3);
pub const M98090_COEFS_MAX_SZ: u32 = (M98090_COEFS_BLK_SZ * 7);

/*
 * M98090_REG_RECORD_BIQUAD_BASE
 */
pub const M98090_REC_B0_HI_MASK: u32 = (255 << 0);
pub const M98090_REC_B0_HI_SHIFT: u32 = 0;
pub const M98090_REC_B0_HI_WIDTH: u32 = 8;
pub const M98090_REC_B0_MID_MASK: u32 = (255 << 0);
pub const M98090_REC_B0_MID_SHIFT: u32 = 0;
pub const M98090_REC_B0_MID_WIDTH: u32 = 8;
pub const M98090_REC_B0_LO_MASK: u32 = (255 << 0);
pub const M98090_REC_B0_LO_SHIFT: u32 = 0;
pub const M98090_REC_B0_LO_WIDTH: u32 = 8;
pub const M98090_REC_B1_HI_MASK: u32 = (255 << 0);
pub const M98090_REC_B1_HI_SHIFT: u32 = 0;
pub const M98090_REC_B1_HI_WIDTH: u32 = 8;
pub const M98090_REC_B1_MID_MASK: u32 = (255 << 0);
pub const M98090_REC_B1_MID_SHIFT: u32 = 0;
pub const M98090_REC_B1_MID_WIDTH: u32 = 8;
pub const M98090_REC_B1_LO_MASK: u32 = (255 << 0);
pub const M98090_REC_B1_LO_SHIFT: u32 = 0;
pub const M98090_REC_B1_LO_WIDTH: u32 = 8;
pub const M98090_REC_B2_HI_MASK: u32 = (255 << 0);
pub const M98090_REC_B2_HI_SHIFT: u32 = 0;
pub const M98090_REC_B2_HI_WIDTH: u32 = 8;
pub const M98090_REC_B2_MID_MASK: u32 = (255 << 0);
pub const M98090_REC_B2_MID_SHIFT: u32 = 0;
pub const M98090_REC_B2_MID_WIDTH: u32 = 8;
pub const M98090_REC_B2_LO_MASK: u32 = (255 << 0);
pub const M98090_REC_B2_LO_SHIFT: u32 = 0;
pub const M98090_REC_B2_LO_WIDTH: u32 = 8;
pub const M98090_REC_A1_HI_MASK: u32 = (255 << 0);
pub const M98090_REC_A1_HI_SHIFT: u32 = 0;
pub const M98090_REC_A1_HI_WIDTH: u32 = 8;
pub const M98090_REC_A1_MID_MASK: u32 = (255 << 0);
pub const M98090_REC_A1_MID_SHIFT: u32 = 0;
pub const M98090_REC_A1_MID_WIDTH: u32 = 8;
pub const M98090_REC_A1_LO_MASK: u32 = (255 << 0);
pub const M98090_REC_A1_LO_SHIFT: u32 = 0;
pub const M98090_REC_A1_LO_WIDTH: u32 = 8;
pub const M98090_REC_A2_HI_MASK: u32 = (255 << 0);
pub const M98090_REC_A2_HI_SHIFT: u32 = 0;
pub const M98090_REC_A2_HI_WIDTH: u32 = 8;
pub const M98090_REC_A2_MID_MASK: u32 = (255 << 0);
pub const M98090_REC_A2_MID_SHIFT: u32 = 0;
pub const M98090_REC_A2_MID_WIDTH: u32 = 8;
pub const M98090_REC_A2_LO_MASK: u32 = (255 << 0);
pub const M98090_REC_A2_LO_SHIFT: u32 = 0;
pub const M98090_REC_A2_LO_WIDTH: u32 = 8;

/*
 * M98090_REG_DMIC3_VOLUME
 */
pub const M98090_DMIC_AV3G_MASK: u32 = (7 << 4);
pub const M98090_DMIC_AV3G_SHIFT: u32 = 4;
pub const M98090_DMIC_AV3G_WIDTH: u32 = 3;
pub const M98090_DMIC_AV3G_NUM: u32 = (1 << M98090_DMIC_AV3G_WIDTH);
pub const M98090_DMIC_AV3_MASK: u32 = (15 << 0);
pub const M98090_DMIC_AV3_SHIFT: u32 = 0;
pub const M98090_DMIC_AV3_WIDTH: u32 = 4;
pub const M98090_DMIC_AV3_NUM: u32 = (1 << M98090_DMIC_AV3_WIDTH);

/*
 * M98090_REG_DMIC4_VOLUME
 */
pub const M98090_DMIC_AV4G_MASK: u32 = (7 << 4);
pub const M98090_DMIC_AV4G_SHIFT: u32 = 4;
pub const M98090_DMIC_AV4G_WIDTH: u32 = 3;
pub const M98090_DMIC_AV4G_NUM: u32 = (1 << M98090_DMIC_AV4G_WIDTH);
pub const M98090_DMIC_AV4_MASK: u32 = (15 << 0);
pub const M98090_DMIC_AV4_SHIFT: u32 = 0;
pub const M98090_DMIC_AV4_WIDTH: u32 = 4;
pub const M98090_DMIC_AV4_NUM: u32 = (1 << M98090_DMIC_AV4_WIDTH);

/*
 * M98090_REG_DMIC34_BQ_PREATTEN
 */
pub const M98090_AV34BQ_MASK: u32 = (15 << 0);
pub const M98090_AV34BQ_SHIFT: u32 = 0;
pub const M98090_AV34BQ_WIDTH: u32 = 4;
pub const M98090_AV34BQ_NUM: u32 = (1 << M98090_AV34BQ_WIDTH);

/*
 * M98090_REG_RECORD_TDM_SLOT
 */
pub const M98090_TDM_SLOTADCL_MASK: u32 = (3 << 6);
pub const M98090_TDM_SLOTADCL_SHIFT: u32 = 6;
pub const M98090_TDM_SLOTADCL_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTADCL_NUM: u32 = (1 << M98090_TDM_SLOTADCL_WIDTH);
pub const M98090_TDM_SLOTADCR_MASK: u32 = (3 << 4);
pub const M98090_TDM_SLOTADCR_SHIFT: u32 = 4;
pub const M98090_TDM_SLOTADCR_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTADCR_NUM: u32 = (1 << M98090_TDM_SLOTADCR_WIDTH);
pub const M98090_TDM_SLOTDMIC3_MASK: u32 = (3 << 2);
pub const M98090_TDM_SLOTDMIC3_SHIFT: u32 = 2;
pub const M98090_TDM_SLOTDMIC3_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTDMIC3_NUM: u32 = (1 << M98090_TDM_SLOTDMIC3_WIDTH);
pub const M98090_TDM_SLOTDMIC4_MASK: u32 = (3 << 0);
pub const M98090_TDM_SLOTDMIC4_SHIFT: u32 = 0;
pub const M98090_TDM_SLOTDMIC4_WIDTH: u32 = 2;
pub const M98090_TDM_SLOTDMIC4_NUM: u32 = (1 << M98090_TDM_SLOTDMIC4_WIDTH);

/*
 * M98090_REG_SAMPLE_RATE
 */
pub const M98090_DMIC34_ZEROPAD_MASK: u32 = (1 << 4);
pub const M98090_DMIC34_ZEROPAD_SHIFT: u32 = 4;
pub const M98090_DMIC34_ZEROPAD_WIDTH: u32 = 1;
pub const M98090_DMIC34_ZEROPAD_NUM: u32 = (1 << M98090_DIGMIC4_WIDTH);
pub const M98090_DMIC34_SRDIV_MASK: u32 = (7 << 0);
pub const M98090_DMIC34_SRDIV_SHIFT: u32 = 0;
pub const M98090_DMIC34_SRDIV_WIDTH: u32 = 3;

/*
 * M98090_REG_DMIC34_BIQUAD_BASE
 */
pub const M98090_DMIC34_B0_HI_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B0_HI_SHIFT: u32 = 0;
pub const M98090_DMIC34_B0_HI_WIDTH: u32 = 8;
pub const M98090_DMIC34_B0_MID_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B0_MID_SHIFT: u32 = 0;
pub const M98090_DMIC34_B0_MID_WIDTH: u32 = 8;
pub const M98090_DMIC34_B0_LO_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B0_LO_SHIFT: u32 = 0;
pub const M98090_DMIC34_B0_LO_WIDTH: u32 = 8;
pub const M98090_DMIC34_B1_HI_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B1_HI_SHIFT: u32 = 0;
pub const M98090_DMIC34_B1_HI_WIDTH: u32 = 8;
pub const M98090_DMIC34_B1_MID_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B1_MID_SHIFT: u32 = 0;
pub const M98090_DMIC34_B1_MID_WIDTH: u32 = 8;
pub const M98090_DMIC34_B1_LO_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B1_LO_SHIFT: u32 = 0;
pub const M98090_DMIC34_B1_LO_WIDTH: u32 = 8;
pub const M98090_DMIC34_B2_HI_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B2_HI_SHIFT: u32 = 0;
pub const M98090_DMIC34_B2_HI_WIDTH: u32 = 8;
pub const M98090_DMIC34_B2_MID_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B2_MID_SHIFT: u32 = 0;
pub const M98090_DMIC34_B2_MID_WIDTH: u32 = 8;
pub const M98090_DMIC34_B2_LO_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_B2_LO_SHIFT: u32 = 0;
pub const M98090_DMIC34_B2_LO_WIDTH: u32 = 8;
pub const M98090_DMIC34_A1_HI_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A1_HI_SHIFT: u32 = 0;
pub const M98090_DMIC34_A1_HI_WIDTH: u32 = 8;
pub const M98090_DMIC34_A1_MID_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A1_MID_SHIFT: u32 = 0;
pub const M98090_DMIC34_A1_MID_WIDTH: u32 = 8;
pub const M98090_DMIC34_A1_LO_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A1_LO_SHIFT: u32 = 0;
pub const M98090_DMIC34_A1_LO_WIDTH: u32 = 8;
pub const M98090_DMIC34_A2_HI_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A2_HI_SHIFT: u32 = 0;
pub const M98090_DMIC34_A2_HI_WIDTH: u32 = 8;
pub const M98090_DMIC34_A2_MID_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A2_MID_SHIFT: u32 = 0;
pub const M98090_DMIC34_A2_MID_WIDTH: u32 = 8;
pub const M98090_DMIC34_A2_LO_MASK: u32 = (255 << 0);
pub const M98090_DMIC34_A2_LO_SHIFT: u32 = 0;
pub const M98090_DMIC34_A2_LO_WIDTH: u32 = 8;

pub const M98090_JACK_STATE_NO_HEADSET: u32 = 0;
pub const M98090_JACK_STATE_NO_HEADSET_2: u32 = 1;
pub const M98090_JACK_STATE_HEADPHONE: u32 = 2;
pub const M98090_JACK_STATE_HEADSET: u32 = 3;

/*
 * M98090_REG_REVISION_ID
 */
pub const M98090_REVID_MASK: u32 = (255 << 0);
pub const M98090_REVID_SHIFT: u32 = 0;
pub const M98090_REVID_WIDTH: u32 = 8;
pub const M98090_REVID_NUM: u32 = (1 << M98090_REVID_WIDTH);

/* Silicon revision number */
pub const M98090_REVA: u32 = 0x40;
pub const M98091_REVA: u32 = 0x50;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum max98090_type {
    MAX98090,
    MAX98091,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct max98090_cdata {
    pub rate: u32,
    pub fmt: u32,
}

#[repr(C)]
pub struct max98090_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub devtype: max98090_type,
    pub pdata: *mut max98090_pdata,
    pub mclk: *mut clk,
    pub sysclk: u32,
    pub pclk: u32,
    pub bclk: u32,
    pub lrclk: u32,
    pub dmic_freq: u32,
    pub dai: [max98090_cdata; 1],
    pub jack_state: i32,
    pub jack_work: delayed_work,
    pub pll_det_enable_work: delayed_work,
    pub pll_det_disable_work: work_struct,
    pub jack: *mut snd_soc_jack,
    pub dai_fmt: u32,
    pub tdm_slots: i32,
    pub tdm_lslot: i32,
    pub tdm_rslot: i32,
    pub lin_state: u8,
    pub pa1en: u32,
    pub pa2en: u32,
    pub sidetone: u32,
    pub master: bool,
    pub shdn_pending: bool,
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
