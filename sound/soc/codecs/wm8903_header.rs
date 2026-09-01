/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * wm8903.h - WM8903 audio codec interface
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Dependency intent from C header: #include <linux/i2c.h>

unsafe extern "C" {
    pub fn wm8903_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        det: core::ffi::c_int,
        shrt: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

/*
 * wm8903.h - WM8903 audio codec interface
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */





/*
 * Register values.
 */
pub const WM8903_SW_RESET_AND_ID: u32 = 0x00;
pub const WM8903_REVISION_NUMBER: u32 = 0x01;
pub const WM8903_BIAS_CONTROL_0: u32 = 0x04;
pub const WM8903_VMID_CONTROL_0: u32 = 0x05;
pub const WM8903_MIC_BIAS_CONTROL_0: u32 = 0x06;
pub const WM8903_ANALOGUE_DAC_0: u32 = 0x08;
pub const WM8903_ANALOGUE_ADC_0: u32 = 0x0A;
pub const WM8903_POWER_MANAGEMENT_0: u32 = 0x0C;
pub const WM8903_POWER_MANAGEMENT_1: u32 = 0x0D;
pub const WM8903_POWER_MANAGEMENT_2: u32 = 0x0E;
pub const WM8903_POWER_MANAGEMENT_3: u32 = 0x0F;
pub const WM8903_POWER_MANAGEMENT_4: u32 = 0x10;
pub const WM8903_POWER_MANAGEMENT_5: u32 = 0x11;
pub const WM8903_POWER_MANAGEMENT_6: u32 = 0x12;
pub const WM8903_CLOCK_RATES_0: u32 = 0x14;
pub const WM8903_CLOCK_RATES_1: u32 = 0x15;
pub const WM8903_CLOCK_RATES_2: u32 = 0x16;
pub const WM8903_AUDIO_INTERFACE_0: u32 = 0x18;
pub const WM8903_AUDIO_INTERFACE_1: u32 = 0x19;
pub const WM8903_AUDIO_INTERFACE_2: u32 = 0x1A;
pub const WM8903_AUDIO_INTERFACE_3: u32 = 0x1B;
pub const WM8903_DAC_DIGITAL_VOLUME_LEFT: u32 = 0x1E;
pub const WM8903_DAC_DIGITAL_VOLUME_RIGHT: u32 = 0x1F;
pub const WM8903_DAC_DIGITAL_0: u32 = 0x20;
pub const WM8903_DAC_DIGITAL_1: u32 = 0x21;
pub const WM8903_ADC_DIGITAL_VOLUME_LEFT: u32 = 0x24;
pub const WM8903_ADC_DIGITAL_VOLUME_RIGHT: u32 = 0x25;
pub const WM8903_ADC_DIGITAL_0: u32 = 0x26;
pub const WM8903_DIGITAL_MICROPHONE_0: u32 = 0x27;
pub const WM8903_DRC_0: u32 = 0x28;
pub const WM8903_DRC_1: u32 = 0x29;
pub const WM8903_DRC_2: u32 = 0x2A;
pub const WM8903_DRC_3: u32 = 0x2B;
pub const WM8903_ANALOGUE_LEFT_INPUT_0: u32 = 0x2C;
pub const WM8903_ANALOGUE_RIGHT_INPUT_0: u32 = 0x2D;
pub const WM8903_ANALOGUE_LEFT_INPUT_1: u32 = 0x2E;
pub const WM8903_ANALOGUE_RIGHT_INPUT_1: u32 = 0x2F;
pub const WM8903_ANALOGUE_LEFT_MIX_0: u32 = 0x32;
pub const WM8903_ANALOGUE_RIGHT_MIX_0: u32 = 0x33;
pub const WM8903_ANALOGUE_SPK_MIX_LEFT_0: u32 = 0x34;
pub const WM8903_ANALOGUE_SPK_MIX_LEFT_1: u32 = 0x35;
pub const WM8903_ANALOGUE_SPK_MIX_RIGHT_0: u32 = 0x36;
pub const WM8903_ANALOGUE_SPK_MIX_RIGHT_1: u32 = 0x37;
pub const WM8903_ANALOGUE_OUT1_LEFT: u32 = 0x39;
pub const WM8903_ANALOGUE_OUT1_RIGHT: u32 = 0x3A;
pub const WM8903_ANALOGUE_OUT2_LEFT: u32 = 0x3B;
pub const WM8903_ANALOGUE_OUT2_RIGHT: u32 = 0x3C;
pub const WM8903_ANALOGUE_OUT3_LEFT: u32 = 0x3E;
pub const WM8903_ANALOGUE_OUT3_RIGHT: u32 = 0x3F;
pub const WM8903_ANALOGUE_SPK_OUTPUT_CONTROL_0: u32 = 0x41;
pub const WM8903_DC_SERVO_0: u32 = 0x43;
pub const WM8903_DC_SERVO_2: u32 = 0x45;
pub const WM8903_DC_SERVO_4: u32 = 0x47;
pub const WM8903_DC_SERVO_5: u32 = 0x48;
pub const WM8903_DC_SERVO_6: u32 = 0x49;
pub const WM8903_DC_SERVO_7: u32 = 0x4A;
pub const WM8903_DC_SERVO_READBACK_1: u32 = 0x51;
pub const WM8903_DC_SERVO_READBACK_2: u32 = 0x52;
pub const WM8903_DC_SERVO_READBACK_3: u32 = 0x53;
pub const WM8903_DC_SERVO_READBACK_4: u32 = 0x54;
pub const WM8903_ANALOGUE_HP_0: u32 = 0x5A;
pub const WM8903_ANALOGUE_LINEOUT_0: u32 = 0x5E;
pub const WM8903_CHARGE_PUMP_0: u32 = 0x62;
pub const WM8903_CLASS_W_0: u32 = 0x68;
pub const WM8903_WRITE_SEQUENCER_0: u32 = 0x6C;
pub const WM8903_WRITE_SEQUENCER_1: u32 = 0x6D;
pub const WM8903_WRITE_SEQUENCER_2: u32 = 0x6E;
pub const WM8903_WRITE_SEQUENCER_3: u32 = 0x6F;
pub const WM8903_WRITE_SEQUENCER_4: u32 = 0x70;
pub const WM8903_CONTROL_INTERFACE: u32 = 0x72;
pub const WM8903_GPIO_CONTROL_1: u32 = 0x74;
pub const WM8903_GPIO_CONTROL_2: u32 = 0x75;
pub const WM8903_GPIO_CONTROL_3: u32 = 0x76;
pub const WM8903_GPIO_CONTROL_4: u32 = 0x77;
pub const WM8903_GPIO_CONTROL_5: u32 = 0x78;
pub const WM8903_INTERRUPT_STATUS_1: u32 = 0x79;
pub const WM8903_INTERRUPT_STATUS_1_MASK: u32 = 0x7A;
pub const WM8903_INTERRUPT_POLARITY_1: u32 = 0x7B;
pub const WM8903_INTERRUPT_CONTROL: u32 = 0x7E;
pub const WM8903_CLOCK_RATE_TEST_4: u32 = 0xA4;
pub const WM8903_ANALOGUE_OUTPUT_BIAS_0: u32 = 0xAC;

pub const WM8903_REGISTER_COUNT: u32 = 75;
pub const WM8903_MAX_REGISTER: u32 = 0xAC;

/*
 * Field Definitions.
 */

/*
 * R0 (0x00) - SW Reset and ID
 */
pub const WM8903_SW_RESET_DEV_ID1_MASK: u32 = 0xFFFF;   /* SW_RESET_DEV_ID1 - [15:0] */
pub const WM8903_SW_RESET_DEV_ID1_SHIFT: u32 = 0;   /* SW_RESET_DEV_ID1 - [15:0] */
pub const WM8903_SW_RESET_DEV_ID1_WIDTH: u32 = 16;   /* SW_RESET_DEV_ID1 - [15:0] */

/*
 * R1 (0x01) - Revision Number
 */
pub const WM8903_CHIP_REV_MASK: u32 = 0x000F;   /* CHIP_REV - [3:0] */
pub const WM8903_CHIP_REV_SHIFT: u32 = 0;   /* CHIP_REV - [3:0] */
pub const WM8903_CHIP_REV_WIDTH: u32 = 4;   /* CHIP_REV - [3:0] */

/*
 * R4 (0x04) - Bias Control 0
 */
pub const WM8903_POBCTRL: u32 = 0x0010;   /* POBCTRL */
pub const WM8903_POBCTRL_MASK: u32 = 0x0010;   /* POBCTRL */
pub const WM8903_POBCTRL_SHIFT: u32 = 4;   /* POBCTRL */
pub const WM8903_POBCTRL_WIDTH: u32 = 1;   /* POBCTRL */
pub const WM8903_ISEL_MASK: u32 = 0x000C;   /* ISEL - [3:2] */
pub const WM8903_ISEL_SHIFT: u32 = 2;   /* ISEL - [3:2] */
pub const WM8903_ISEL_WIDTH: u32 = 2;   /* ISEL - [3:2] */
pub const WM8903_STARTUP_BIAS_ENA: u32 = 0x0002;   /* STARTUP_BIAS_ENA */
pub const WM8903_STARTUP_BIAS_ENA_MASK: u32 = 0x0002;   /* STARTUP_BIAS_ENA */
pub const WM8903_STARTUP_BIAS_ENA_SHIFT: u32 = 1;   /* STARTUP_BIAS_ENA */
pub const WM8903_STARTUP_BIAS_ENA_WIDTH: u32 = 1;   /* STARTUP_BIAS_ENA */
pub const WM8903_BIAS_ENA: u32 = 0x0001;   /* BIAS_ENA */
pub const WM8903_BIAS_ENA_MASK: u32 = 0x0001;   /* BIAS_ENA */
pub const WM8903_BIAS_ENA_SHIFT: u32 = 0;   /* BIAS_ENA */
pub const WM8903_BIAS_ENA_WIDTH: u32 = 1;   /* BIAS_ENA */

/*
 * R5 (0x05) - VMID Control 0
 */
pub const WM8903_VMID_TIE_ENA: u32 = 0x0080;   /* VMID_TIE_ENA */
pub const WM8903_VMID_TIE_ENA_MASK: u32 = 0x0080;   /* VMID_TIE_ENA */
pub const WM8903_VMID_TIE_ENA_SHIFT: u32 = 7;   /* VMID_TIE_ENA */
pub const WM8903_VMID_TIE_ENA_WIDTH: u32 = 1;   /* VMID_TIE_ENA */
pub const WM8903_BUFIO_ENA: u32 = 0x0040;   /* BUFIO_ENA */
pub const WM8903_BUFIO_ENA_MASK: u32 = 0x0040;   /* BUFIO_ENA */
pub const WM8903_BUFIO_ENA_SHIFT: u32 = 6;   /* BUFIO_ENA */
pub const WM8903_BUFIO_ENA_WIDTH: u32 = 1;   /* BUFIO_ENA */
pub const WM8903_VMID_IO_ENA: u32 = 0x0020;   /* VMID_IO_ENA */
pub const WM8903_VMID_IO_ENA_MASK: u32 = 0x0020;   /* VMID_IO_ENA */
pub const WM8903_VMID_IO_ENA_SHIFT: u32 = 5;   /* VMID_IO_ENA */
pub const WM8903_VMID_IO_ENA_WIDTH: u32 = 1;   /* VMID_IO_ENA */
pub const WM8903_VMID_SOFT_MASK: u32 = 0x0018;   /* VMID_SOFT - [4:3] */
pub const WM8903_VMID_SOFT_SHIFT: u32 = 3;   /* VMID_SOFT - [4:3] */
pub const WM8903_VMID_SOFT_WIDTH: u32 = 2;   /* VMID_SOFT - [4:3] */
pub const WM8903_VMID_RES_MASK: u32 = 0x0006;   /* VMID_RES - [2:1] */
pub const WM8903_VMID_RES_SHIFT: u32 = 1;   /* VMID_RES - [2:1] */
pub const WM8903_VMID_RES_WIDTH: u32 = 2;   /* VMID_RES - [2:1] */
pub const WM8903_VMID_BUF_ENA: u32 = 0x0001;   /* VMID_BUF_ENA */
pub const WM8903_VMID_BUF_ENA_MASK: u32 = 0x0001;   /* VMID_BUF_ENA */
pub const WM8903_VMID_BUF_ENA_SHIFT: u32 = 0;   /* VMID_BUF_ENA */
pub const WM8903_VMID_BUF_ENA_WIDTH: u32 = 1;   /* VMID_BUF_ENA */

pub const WM8903_VMID_RES_50K: u32 = 2;
pub const WM8903_VMID_RES_250K: u32 = 4;
pub const WM8903_VMID_RES_5K: u32 = 6;

/*
 * R8 (0x08) - Analogue DAC 0
 */
pub const WM8903_DACBIAS_SEL_MASK: u32 = 0x0018;   /* DACBIAS_SEL - [4:3] */
pub const WM8903_DACBIAS_SEL_SHIFT: u32 = 3;   /* DACBIAS_SEL - [4:3] */
pub const WM8903_DACBIAS_SEL_WIDTH: u32 = 2;   /* DACBIAS_SEL - [4:3] */
pub const WM8903_DACVMID_BIAS_SEL_MASK: u32 = 0x0006;   /* DACVMID_BIAS_SEL - [2:1] */
pub const WM8903_DACVMID_BIAS_SEL_SHIFT: u32 = 1;   /* DACVMID_BIAS_SEL - [2:1] */
pub const WM8903_DACVMID_BIAS_SEL_WIDTH: u32 = 2;   /* DACVMID_BIAS_SEL - [2:1] */

/*
 * R10 (0x0A) - Analogue ADC 0
 */
pub const WM8903_ADC_OSR128: u32 = 0x0001;   /* ADC_OSR128 */
pub const WM8903_ADC_OSR128_MASK: u32 = 0x0001;   /* ADC_OSR128 */
pub const WM8903_ADC_OSR128_SHIFT: u32 = 0;   /* ADC_OSR128 */
pub const WM8903_ADC_OSR128_WIDTH: u32 = 1;   /* ADC_OSR128 */

/*
 * R12 (0x0C) - Power Management 0
 */
pub const WM8903_INL_ENA: u32 = 0x0002;   /* INL_ENA */
pub const WM8903_INL_ENA_MASK: u32 = 0x0002;   /* INL_ENA */
pub const WM8903_INL_ENA_SHIFT: u32 = 1;   /* INL_ENA */
pub const WM8903_INL_ENA_WIDTH: u32 = 1;   /* INL_ENA */
pub const WM8903_INR_ENA: u32 = 0x0001;   /* INR_ENA */
pub const WM8903_INR_ENA_MASK: u32 = 0x0001;   /* INR_ENA */
pub const WM8903_INR_ENA_SHIFT: u32 = 0;   /* INR_ENA */
pub const WM8903_INR_ENA_WIDTH: u32 = 1;   /* INR_ENA */

/*
 * R13 (0x0D) - Power Management 1
 */
pub const WM8903_MIXOUTL_ENA: u32 = 0x0002;   /* MIXOUTL_ENA */
pub const WM8903_MIXOUTL_ENA_MASK: u32 = 0x0002;   /* MIXOUTL_ENA */
pub const WM8903_MIXOUTL_ENA_SHIFT: u32 = 1;   /* MIXOUTL_ENA */
pub const WM8903_MIXOUTL_ENA_WIDTH: u32 = 1;   /* MIXOUTL_ENA */
pub const WM8903_MIXOUTR_ENA: u32 = 0x0001;   /* MIXOUTR_ENA */
pub const WM8903_MIXOUTR_ENA_MASK: u32 = 0x0001;   /* MIXOUTR_ENA */
pub const WM8903_MIXOUTR_ENA_SHIFT: u32 = 0;   /* MIXOUTR_ENA */
pub const WM8903_MIXOUTR_ENA_WIDTH: u32 = 1;   /* MIXOUTR_ENA */

/*
 * R14 (0x0E) - Power Management 2
 */
pub const WM8903_HPL_PGA_ENA: u32 = 0x0002;   /* HPL_PGA_ENA */
pub const WM8903_HPL_PGA_ENA_MASK: u32 = 0x0002;   /* HPL_PGA_ENA */
pub const WM8903_HPL_PGA_ENA_SHIFT: u32 = 1;   /* HPL_PGA_ENA */
pub const WM8903_HPL_PGA_ENA_WIDTH: u32 = 1;   /* HPL_PGA_ENA */
pub const WM8903_HPR_PGA_ENA: u32 = 0x0001;   /* HPR_PGA_ENA */
pub const WM8903_HPR_PGA_ENA_MASK: u32 = 0x0001;   /* HPR_PGA_ENA */
pub const WM8903_HPR_PGA_ENA_SHIFT: u32 = 0;   /* HPR_PGA_ENA */
pub const WM8903_HPR_PGA_ENA_WIDTH: u32 = 1;   /* HPR_PGA_ENA */

/*
 * R15 (0x0F) - Power Management 3
 */
pub const WM8903_LINEOUTL_PGA_ENA: u32 = 0x0002;   /* LINEOUTL_PGA_ENA */
pub const WM8903_LINEOUTL_PGA_ENA_MASK: u32 = 0x0002;   /* LINEOUTL_PGA_ENA */
pub const WM8903_LINEOUTL_PGA_ENA_SHIFT: u32 = 1;   /* LINEOUTL_PGA_ENA */
pub const WM8903_LINEOUTL_PGA_ENA_WIDTH: u32 = 1;   /* LINEOUTL_PGA_ENA */
pub const WM8903_LINEOUTR_PGA_ENA: u32 = 0x0001;   /* LINEOUTR_PGA_ENA */
pub const WM8903_LINEOUTR_PGA_ENA_MASK: u32 = 0x0001;   /* LINEOUTR_PGA_ENA */
pub const WM8903_LINEOUTR_PGA_ENA_SHIFT: u32 = 0;   /* LINEOUTR_PGA_ENA */
pub const WM8903_LINEOUTR_PGA_ENA_WIDTH: u32 = 1;   /* LINEOUTR_PGA_ENA */

/*
 * R16 (0x10) - Power Management 4
 */
pub const WM8903_MIXSPKL_ENA: u32 = 0x0002;   /* MIXSPKL_ENA */
pub const WM8903_MIXSPKL_ENA_MASK: u32 = 0x0002;   /* MIXSPKL_ENA */
pub const WM8903_MIXSPKL_ENA_SHIFT: u32 = 1;   /* MIXSPKL_ENA */
pub const WM8903_MIXSPKL_ENA_WIDTH: u32 = 1;   /* MIXSPKL_ENA */
pub const WM8903_MIXSPKR_ENA: u32 = 0x0001;   /* MIXSPKR_ENA */
pub const WM8903_MIXSPKR_ENA_MASK: u32 = 0x0001;   /* MIXSPKR_ENA */
pub const WM8903_MIXSPKR_ENA_SHIFT: u32 = 0;   /* MIXSPKR_ENA */
pub const WM8903_MIXSPKR_ENA_WIDTH: u32 = 1;   /* MIXSPKR_ENA */

/*
 * R17 (0x11) - Power Management 5
 */
pub const WM8903_SPKL_ENA: u32 = 0x0002;   /* SPKL_ENA */
pub const WM8903_SPKL_ENA_MASK: u32 = 0x0002;   /* SPKL_ENA */
pub const WM8903_SPKL_ENA_SHIFT: u32 = 1;   /* SPKL_ENA */
pub const WM8903_SPKL_ENA_WIDTH: u32 = 1;   /* SPKL_ENA */
pub const WM8903_SPKR_ENA: u32 = 0x0001;   /* SPKR_ENA */
pub const WM8903_SPKR_ENA_MASK: u32 = 0x0001;   /* SPKR_ENA */
pub const WM8903_SPKR_ENA_SHIFT: u32 = 0;   /* SPKR_ENA */
pub const WM8903_SPKR_ENA_WIDTH: u32 = 1;   /* SPKR_ENA */

/*
 * R18 (0x12) - Power Management 6
 */
pub const WM8903_DACL_ENA: u32 = 0x0008;   /* DACL_ENA */
pub const WM8903_DACL_ENA_MASK: u32 = 0x0008;   /* DACL_ENA */
pub const WM8903_DACL_ENA_SHIFT: u32 = 3;   /* DACL_ENA */
pub const WM8903_DACL_ENA_WIDTH: u32 = 1;   /* DACL_ENA */
pub const WM8903_DACR_ENA: u32 = 0x0004;   /* DACR_ENA */
pub const WM8903_DACR_ENA_MASK: u32 = 0x0004;   /* DACR_ENA */
pub const WM8903_DACR_ENA_SHIFT: u32 = 2;   /* DACR_ENA */
pub const WM8903_DACR_ENA_WIDTH: u32 = 1;   /* DACR_ENA */
pub const WM8903_ADCL_ENA: u32 = 0x0002;   /* ADCL_ENA */
pub const WM8903_ADCL_ENA_MASK: u32 = 0x0002;   /* ADCL_ENA */
pub const WM8903_ADCL_ENA_SHIFT: u32 = 1;   /* ADCL_ENA */
pub const WM8903_ADCL_ENA_WIDTH: u32 = 1;   /* ADCL_ENA */
pub const WM8903_ADCR_ENA: u32 = 0x0001;   /* ADCR_ENA */
pub const WM8903_ADCR_ENA_MASK: u32 = 0x0001;   /* ADCR_ENA */
pub const WM8903_ADCR_ENA_SHIFT: u32 = 0;   /* ADCR_ENA */
pub const WM8903_ADCR_ENA_WIDTH: u32 = 1;   /* ADCR_ENA */

/*
 * R20 (0x14) - Clock Rates 0
 */
pub const WM8903_MCLKDIV2: u32 = 0x0001;   /* MCLKDIV2 */
pub const WM8903_MCLKDIV2_MASK: u32 = 0x0001;   /* MCLKDIV2 */
pub const WM8903_MCLKDIV2_SHIFT: u32 = 0;   /* MCLKDIV2 */
pub const WM8903_MCLKDIV2_WIDTH: u32 = 1;   /* MCLKDIV2 */

/*
 * R21 (0x15) - Clock Rates 1
 */
pub const WM8903_CLK_SYS_RATE_MASK: u32 = 0x3C00;   /* CLK_SYS_RATE - [13:10] */
pub const WM8903_CLK_SYS_RATE_SHIFT: u32 = 10;   /* CLK_SYS_RATE - [13:10] */
pub const WM8903_CLK_SYS_RATE_WIDTH: u32 = 4;   /* CLK_SYS_RATE - [13:10] */
pub const WM8903_CLK_SYS_MODE_MASK: u32 = 0x0300;   /* CLK_SYS_MODE - [9:8] */
pub const WM8903_CLK_SYS_MODE_SHIFT: u32 = 8;   /* CLK_SYS_MODE - [9:8] */
pub const WM8903_CLK_SYS_MODE_WIDTH: u32 = 2;   /* CLK_SYS_MODE - [9:8] */
pub const WM8903_SAMPLE_RATE_MASK: u32 = 0x000F;   /* SAMPLE_RATE - [3:0] */
pub const WM8903_SAMPLE_RATE_SHIFT: u32 = 0;   /* SAMPLE_RATE - [3:0] */
pub const WM8903_SAMPLE_RATE_WIDTH: u32 = 4;   /* SAMPLE_RATE - [3:0] */

/*
 * R22 (0x16) - Clock Rates 2
 */
pub const WM8903_CLK_SYS_ENA: u32 = 0x0004;   /* CLK_SYS_ENA */
pub const WM8903_CLK_SYS_ENA_MASK: u32 = 0x0004;   /* CLK_SYS_ENA */
pub const WM8903_CLK_SYS_ENA_SHIFT: u32 = 2;   /* CLK_SYS_ENA */
pub const WM8903_CLK_SYS_ENA_WIDTH: u32 = 1;   /* CLK_SYS_ENA */
pub const WM8903_CLK_DSP_ENA: u32 = 0x0002;   /* CLK_DSP_ENA */
pub const WM8903_CLK_DSP_ENA_MASK: u32 = 0x0002;   /* CLK_DSP_ENA */
pub const WM8903_CLK_DSP_ENA_SHIFT: u32 = 1;   /* CLK_DSP_ENA */
pub const WM8903_CLK_DSP_ENA_WIDTH: u32 = 1;   /* CLK_DSP_ENA */
pub const WM8903_TO_ENA: u32 = 0x0001;   /* TO_ENA */
pub const WM8903_TO_ENA_MASK: u32 = 0x0001;   /* TO_ENA */
pub const WM8903_TO_ENA_SHIFT: u32 = 0;   /* TO_ENA */
pub const WM8903_TO_ENA_WIDTH: u32 = 1;   /* TO_ENA */

/*
 * R24 (0x18) - Audio Interface 0
 */
pub const WM8903_DACL_DATINV: u32 = 0x1000;   /* DACL_DATINV */
pub const WM8903_DACL_DATINV_MASK: u32 = 0x1000;   /* DACL_DATINV */
pub const WM8903_DACL_DATINV_SHIFT: u32 = 12;   /* DACL_DATINV */
pub const WM8903_DACL_DATINV_WIDTH: u32 = 1;   /* DACL_DATINV */
pub const WM8903_DACR_DATINV: u32 = 0x0800;   /* DACR_DATINV */
pub const WM8903_DACR_DATINV_MASK: u32 = 0x0800;   /* DACR_DATINV */
pub const WM8903_DACR_DATINV_SHIFT: u32 = 11;   /* DACR_DATINV */
pub const WM8903_DACR_DATINV_WIDTH: u32 = 1;   /* DACR_DATINV */
pub const WM8903_DAC_BOOST_MASK: u32 = 0x0600;   /* DAC_BOOST - [10:9] */
pub const WM8903_DAC_BOOST_SHIFT: u32 = 9;   /* DAC_BOOST - [10:9] */
pub const WM8903_DAC_BOOST_WIDTH: u32 = 2;   /* DAC_BOOST - [10:9] */
pub const WM8903_LOOPBACK: u32 = 0x0100;   /* LOOPBACK */
pub const WM8903_LOOPBACK_MASK: u32 = 0x0100;   /* LOOPBACK */
pub const WM8903_LOOPBACK_SHIFT: u32 = 8;   /* LOOPBACK */
pub const WM8903_LOOPBACK_WIDTH: u32 = 1;   /* LOOPBACK */
pub const WM8903_AIFADCL_SRC: u32 = 0x0080;   /* AIFADCL_SRC */
pub const WM8903_AIFADCL_SRC_MASK: u32 = 0x0080;   /* AIFADCL_SRC */
pub const WM8903_AIFADCL_SRC_SHIFT: u32 = 7;   /* AIFADCL_SRC */
pub const WM8903_AIFADCL_SRC_WIDTH: u32 = 1;   /* AIFADCL_SRC */
pub const WM8903_AIFADCR_SRC: u32 = 0x0040;   /* AIFADCR_SRC */
pub const WM8903_AIFADCR_SRC_MASK: u32 = 0x0040;   /* AIFADCR_SRC */
pub const WM8903_AIFADCR_SRC_SHIFT: u32 = 6;   /* AIFADCR_SRC */
pub const WM8903_AIFADCR_SRC_WIDTH: u32 = 1;   /* AIFADCR_SRC */
pub const WM8903_AIFDACL_SRC: u32 = 0x0020;   /* AIFDACL_SRC */
pub const WM8903_AIFDACL_SRC_MASK: u32 = 0x0020;   /* AIFDACL_SRC */
pub const WM8903_AIFDACL_SRC_SHIFT: u32 = 5;   /* AIFDACL_SRC */
pub const WM8903_AIFDACL_SRC_WIDTH: u32 = 1;   /* AIFDACL_SRC */
pub const WM8903_AIFDACR_SRC: u32 = 0x0010;   /* AIFDACR_SRC */
pub const WM8903_AIFDACR_SRC_MASK: u32 = 0x0010;   /* AIFDACR_SRC */
pub const WM8903_AIFDACR_SRC_SHIFT: u32 = 4;   /* AIFDACR_SRC */
pub const WM8903_AIFDACR_SRC_WIDTH: u32 = 1;   /* AIFDACR_SRC */
pub const WM8903_ADC_COMP: u32 = 0x0008;   /* ADC_COMP */
pub const WM8903_ADC_COMP_MASK: u32 = 0x0008;   /* ADC_COMP */
pub const WM8903_ADC_COMP_SHIFT: u32 = 3;   /* ADC_COMP */
pub const WM8903_ADC_COMP_WIDTH: u32 = 1;   /* ADC_COMP */
pub const WM8903_ADC_COMPMODE: u32 = 0x0004;   /* ADC_COMPMODE */
pub const WM8903_ADC_COMPMODE_MASK: u32 = 0x0004;   /* ADC_COMPMODE */
pub const WM8903_ADC_COMPMODE_SHIFT: u32 = 2;   /* ADC_COMPMODE */
pub const WM8903_ADC_COMPMODE_WIDTH: u32 = 1;   /* ADC_COMPMODE */
pub const WM8903_DAC_COMP: u32 = 0x0002;   /* DAC_COMP */
pub const WM8903_DAC_COMP_MASK: u32 = 0x0002;   /* DAC_COMP */
pub const WM8903_DAC_COMP_SHIFT: u32 = 1;   /* DAC_COMP */
pub const WM8903_DAC_COMP_WIDTH: u32 = 1;   /* DAC_COMP */
pub const WM8903_DAC_COMPMODE: u32 = 0x0001;   /* DAC_COMPMODE */
pub const WM8903_DAC_COMPMODE_MASK: u32 = 0x0001;   /* DAC_COMPMODE */
pub const WM8903_DAC_COMPMODE_SHIFT: u32 = 0;   /* DAC_COMPMODE */
pub const WM8903_DAC_COMPMODE_WIDTH: u32 = 1;   /* DAC_COMPMODE */

/*
 * R25 (0x19) - Audio Interface 1
 */
pub const WM8903_AIFDAC_TDM: u32 = 0x2000;   /* AIFDAC_TDM */
pub const WM8903_AIFDAC_TDM_MASK: u32 = 0x2000;   /* AIFDAC_TDM */
pub const WM8903_AIFDAC_TDM_SHIFT: u32 = 13;   /* AIFDAC_TDM */
pub const WM8903_AIFDAC_TDM_WIDTH: u32 = 1;   /* AIFDAC_TDM */
pub const WM8903_AIFDAC_TDM_CHAN: u32 = 0x1000;   /* AIFDAC_TDM_CHAN */
pub const WM8903_AIFDAC_TDM_CHAN_MASK: u32 = 0x1000;   /* AIFDAC_TDM_CHAN */
pub const WM8903_AIFDAC_TDM_CHAN_SHIFT: u32 = 12;   /* AIFDAC_TDM_CHAN */
pub const WM8903_AIFDAC_TDM_CHAN_WIDTH: u32 = 1;   /* AIFDAC_TDM_CHAN */
pub const WM8903_AIFADC_TDM: u32 = 0x0800;   /* AIFADC_TDM */
pub const WM8903_AIFADC_TDM_MASK: u32 = 0x0800;   /* AIFADC_TDM */
pub const WM8903_AIFADC_TDM_SHIFT: u32 = 11;   /* AIFADC_TDM */
pub const WM8903_AIFADC_TDM_WIDTH: u32 = 1;   /* AIFADC_TDM */
pub const WM8903_AIFADC_TDM_CHAN: u32 = 0x0400;   /* AIFADC_TDM_CHAN */
pub const WM8903_AIFADC_TDM_CHAN_MASK: u32 = 0x0400;   /* AIFADC_TDM_CHAN */
pub const WM8903_AIFADC_TDM_CHAN_SHIFT: u32 = 10;   /* AIFADC_TDM_CHAN */
pub const WM8903_AIFADC_TDM_CHAN_WIDTH: u32 = 1;   /* AIFADC_TDM_CHAN */
pub const WM8903_LRCLK_DIR: u32 = 0x0200;   /* LRCLK_DIR */
pub const WM8903_LRCLK_DIR_MASK: u32 = 0x0200;   /* LRCLK_DIR */
pub const WM8903_LRCLK_DIR_SHIFT: u32 = 9;   /* LRCLK_DIR */
pub const WM8903_LRCLK_DIR_WIDTH: u32 = 1;   /* LRCLK_DIR */
pub const WM8903_AIF_BCLK_INV: u32 = 0x0080;   /* AIF_BCLK_INV */
pub const WM8903_AIF_BCLK_INV_MASK: u32 = 0x0080;   /* AIF_BCLK_INV */
pub const WM8903_AIF_BCLK_INV_SHIFT: u32 = 7;   /* AIF_BCLK_INV */
pub const WM8903_AIF_BCLK_INV_WIDTH: u32 = 1;   /* AIF_BCLK_INV */
pub const WM8903_BCLK_DIR: u32 = 0x0040;   /* BCLK_DIR */
pub const WM8903_BCLK_DIR_MASK: u32 = 0x0040;   /* BCLK_DIR */
pub const WM8903_BCLK_DIR_SHIFT: u32 = 6;   /* BCLK_DIR */
pub const WM8903_BCLK_DIR_WIDTH: u32 = 1;   /* BCLK_DIR */
pub const WM8903_AIF_LRCLK_INV: u32 = 0x0010;   /* AIF_LRCLK_INV */
pub const WM8903_AIF_LRCLK_INV_MASK: u32 = 0x0010;   /* AIF_LRCLK_INV */
pub const WM8903_AIF_LRCLK_INV_SHIFT: u32 = 4;   /* AIF_LRCLK_INV */
pub const WM8903_AIF_LRCLK_INV_WIDTH: u32 = 1;   /* AIF_LRCLK_INV */
pub const WM8903_AIF_WL_MASK: u32 = 0x000C;   /* AIF_WL - [3:2] */
pub const WM8903_AIF_WL_SHIFT: u32 = 2;   /* AIF_WL - [3:2] */
pub const WM8903_AIF_WL_WIDTH: u32 = 2;   /* AIF_WL - [3:2] */
pub const WM8903_AIF_FMT_MASK: u32 = 0x0003;   /* AIF_FMT - [1:0] */
pub const WM8903_AIF_FMT_SHIFT: u32 = 0;   /* AIF_FMT - [1:0] */
pub const WM8903_AIF_FMT_WIDTH: u32 = 2;   /* AIF_FMT - [1:0] */

/*
 * R26 (0x1A) - Audio Interface 2
 */
pub const WM8903_BCLK_DIV_MASK: u32 = 0x001F;   /* BCLK_DIV - [4:0] */
pub const WM8903_BCLK_DIV_SHIFT: u32 = 0;   /* BCLK_DIV - [4:0] */
pub const WM8903_BCLK_DIV_WIDTH: u32 = 5;   /* BCLK_DIV - [4:0] */

/*
 * R27 (0x1B) - Audio Interface 3
 */
pub const WM8903_LRCLK_RATE_MASK: u32 = 0x07FF;   /* LRCLK_RATE - [10:0] */
pub const WM8903_LRCLK_RATE_SHIFT: u32 = 0;   /* LRCLK_RATE - [10:0] */
pub const WM8903_LRCLK_RATE_WIDTH: u32 = 11;   /* LRCLK_RATE - [10:0] */

/*
 * R30 (0x1E) - DAC Digital Volume Left
 */
pub const WM8903_DACVU: u32 = 0x0100;   /* DACVU */
pub const WM8903_DACVU_MASK: u32 = 0x0100;   /* DACVU */
pub const WM8903_DACVU_SHIFT: u32 = 8;   /* DACVU */
pub const WM8903_DACVU_WIDTH: u32 = 1;   /* DACVU */
pub const WM8903_DACL_VOL_MASK: u32 = 0x00FF;   /* DACL_VOL - [7:0] */
pub const WM8903_DACL_VOL_SHIFT: u32 = 0;   /* DACL_VOL - [7:0] */
pub const WM8903_DACL_VOL_WIDTH: u32 = 8;   /* DACL_VOL - [7:0] */

/*
 * R31 (0x1F) - DAC Digital Volume Right
 */
// Duplicate C macro: #define WM8903_DACVU 0x0100    /* DACVU */
// Duplicate C macro: #define WM8903_DACVU_MASK 0x0100    /* DACVU */
// Duplicate C macro: #define WM8903_DACVU_SHIFT 8    /* DACVU */
// Duplicate C macro: #define WM8903_DACVU_WIDTH 1    /* DACVU */
pub const WM8903_DACR_VOL_MASK: u32 = 0x00FF;   /* DACR_VOL - [7:0] */
pub const WM8903_DACR_VOL_SHIFT: u32 = 0;   /* DACR_VOL - [7:0] */
pub const WM8903_DACR_VOL_WIDTH: u32 = 8;   /* DACR_VOL - [7:0] */

/*
 * R32 (0x20) - DAC Digital 0
 */
pub const WM8903_ADCL_DAC_SVOL_MASK: u32 = 0x0F00;   /* ADCL_DAC_SVOL - [11:8] */
pub const WM8903_ADCL_DAC_SVOL_SHIFT: u32 = 8;   /* ADCL_DAC_SVOL - [11:8] */
pub const WM8903_ADCL_DAC_SVOL_WIDTH: u32 = 4;   /* ADCL_DAC_SVOL - [11:8] */
pub const WM8903_ADCR_DAC_SVOL_MASK: u32 = 0x00F0;   /* ADCR_DAC_SVOL - [7:4] */
pub const WM8903_ADCR_DAC_SVOL_SHIFT: u32 = 4;   /* ADCR_DAC_SVOL - [7:4] */
pub const WM8903_ADCR_DAC_SVOL_WIDTH: u32 = 4;   /* ADCR_DAC_SVOL - [7:4] */
pub const WM8903_ADC_TO_DACL_MASK: u32 = 0x000C;   /* ADC_TO_DACL - [3:2] */
pub const WM8903_ADC_TO_DACL_SHIFT: u32 = 2;   /* ADC_TO_DACL - [3:2] */
pub const WM8903_ADC_TO_DACL_WIDTH: u32 = 2;   /* ADC_TO_DACL - [3:2] */
pub const WM8903_ADC_TO_DACR_MASK: u32 = 0x0003;   /* ADC_TO_DACR - [1:0] */
pub const WM8903_ADC_TO_DACR_SHIFT: u32 = 0;   /* ADC_TO_DACR - [1:0] */
pub const WM8903_ADC_TO_DACR_WIDTH: u32 = 2;   /* ADC_TO_DACR - [1:0] */

/*
 * R33 (0x21) - DAC Digital 1
 */
pub const WM8903_DAC_MONO: u32 = 0x1000;   /* DAC_MONO */
pub const WM8903_DAC_MONO_MASK: u32 = 0x1000;   /* DAC_MONO */
pub const WM8903_DAC_MONO_SHIFT: u32 = 12;   /* DAC_MONO */
pub const WM8903_DAC_MONO_WIDTH: u32 = 1;   /* DAC_MONO */
pub const WM8903_DAC_SB_FILT: u32 = 0x0800;   /* DAC_SB_FILT */
pub const WM8903_DAC_SB_FILT_MASK: u32 = 0x0800;   /* DAC_SB_FILT */
pub const WM8903_DAC_SB_FILT_SHIFT: u32 = 11;   /* DAC_SB_FILT */
pub const WM8903_DAC_SB_FILT_WIDTH: u32 = 1;   /* DAC_SB_FILT */
pub const WM8903_DAC_MUTERATE: u32 = 0x0400;   /* DAC_MUTERATE */
pub const WM8903_DAC_MUTERATE_MASK: u32 = 0x0400;   /* DAC_MUTERATE */
pub const WM8903_DAC_MUTERATE_SHIFT: u32 = 10;   /* DAC_MUTERATE */
pub const WM8903_DAC_MUTERATE_WIDTH: u32 = 1;   /* DAC_MUTERATE */
pub const WM8903_DAC_MUTEMODE: u32 = 0x0200;   /* DAC_MUTEMODE */
pub const WM8903_DAC_MUTEMODE_MASK: u32 = 0x0200;   /* DAC_MUTEMODE */
pub const WM8903_DAC_MUTEMODE_SHIFT: u32 = 9;   /* DAC_MUTEMODE */
pub const WM8903_DAC_MUTEMODE_WIDTH: u32 = 1;   /* DAC_MUTEMODE */
pub const WM8903_DAC_MUTE: u32 = 0x0008;   /* DAC_MUTE */
pub const WM8903_DAC_MUTE_MASK: u32 = 0x0008;   /* DAC_MUTE */
pub const WM8903_DAC_MUTE_SHIFT: u32 = 3;   /* DAC_MUTE */
pub const WM8903_DAC_MUTE_WIDTH: u32 = 1;   /* DAC_MUTE */
pub const WM8903_DEEMPH_MASK: u32 = 0x0006;   /* DEEMPH - [2:1] */
pub const WM8903_DEEMPH_SHIFT: u32 = 1;   /* DEEMPH - [2:1] */
pub const WM8903_DEEMPH_WIDTH: u32 = 2;   /* DEEMPH - [2:1] */

/*
 * R36 (0x24) - ADC Digital Volume Left
 */
pub const WM8903_ADCVU: u32 = 0x0100;   /* ADCVU */
pub const WM8903_ADCVU_MASK: u32 = 0x0100;   /* ADCVU */
pub const WM8903_ADCVU_SHIFT: u32 = 8;   /* ADCVU */
pub const WM8903_ADCVU_WIDTH: u32 = 1;   /* ADCVU */
pub const WM8903_ADCL_VOL_MASK: u32 = 0x00FF;   /* ADCL_VOL - [7:0] */
pub const WM8903_ADCL_VOL_SHIFT: u32 = 0;   /* ADCL_VOL - [7:0] */
pub const WM8903_ADCL_VOL_WIDTH: u32 = 8;   /* ADCL_VOL - [7:0] */

/*
 * R37 (0x25) - ADC Digital Volume Right
 */
// Duplicate C macro: #define WM8903_ADCVU 0x0100    /* ADCVU */
// Duplicate C macro: #define WM8903_ADCVU_MASK 0x0100    /* ADCVU */
// Duplicate C macro: #define WM8903_ADCVU_SHIFT 8    /* ADCVU */
// Duplicate C macro: #define WM8903_ADCVU_WIDTH 1    /* ADCVU */
pub const WM8903_ADCR_VOL_MASK: u32 = 0x00FF;   /* ADCR_VOL - [7:0] */
pub const WM8903_ADCR_VOL_SHIFT: u32 = 0;   /* ADCR_VOL - [7:0] */
pub const WM8903_ADCR_VOL_WIDTH: u32 = 8;   /* ADCR_VOL - [7:0] */

/*
 * R38 (0x26) - ADC Digital 0
 */
pub const WM8903_ADC_HPF_CUT_MASK: u32 = 0x0060;   /* ADC_HPF_CUT - [6:5] */
pub const WM8903_ADC_HPF_CUT_SHIFT: u32 = 5;   /* ADC_HPF_CUT - [6:5] */
pub const WM8903_ADC_HPF_CUT_WIDTH: u32 = 2;   /* ADC_HPF_CUT - [6:5] */
pub const WM8903_ADC_HPF_ENA: u32 = 0x0010;   /* ADC_HPF_ENA */
pub const WM8903_ADC_HPF_ENA_MASK: u32 = 0x0010;   /* ADC_HPF_ENA */
pub const WM8903_ADC_HPF_ENA_SHIFT: u32 = 4;   /* ADC_HPF_ENA */
pub const WM8903_ADC_HPF_ENA_WIDTH: u32 = 1;   /* ADC_HPF_ENA */
pub const WM8903_ADCL_DATINV: u32 = 0x0002;   /* ADCL_DATINV */
pub const WM8903_ADCL_DATINV_MASK: u32 = 0x0002;   /* ADCL_DATINV */
pub const WM8903_ADCL_DATINV_SHIFT: u32 = 1;   /* ADCL_DATINV */
pub const WM8903_ADCL_DATINV_WIDTH: u32 = 1;   /* ADCL_DATINV */
pub const WM8903_ADCR_DATINV: u32 = 0x0001;   /* ADCR_DATINV */
pub const WM8903_ADCR_DATINV_MASK: u32 = 0x0001;   /* ADCR_DATINV */
pub const WM8903_ADCR_DATINV_SHIFT: u32 = 0;   /* ADCR_DATINV */
pub const WM8903_ADCR_DATINV_WIDTH: u32 = 1;   /* ADCR_DATINV */

/*
 * R39 (0x27) - Digital Microphone 0
 */
pub const WM8903_DIGMIC_MODE_SEL: u32 = 0x0100;   /* DIGMIC_MODE_SEL */
pub const WM8903_DIGMIC_MODE_SEL_MASK: u32 = 0x0100;   /* DIGMIC_MODE_SEL */
pub const WM8903_DIGMIC_MODE_SEL_SHIFT: u32 = 8;   /* DIGMIC_MODE_SEL */
pub const WM8903_DIGMIC_MODE_SEL_WIDTH: u32 = 1;   /* DIGMIC_MODE_SEL */
pub const WM8903_DIGMIC_CLK_SEL_L_MASK: u32 = 0x00C0;   /* DIGMIC_CLK_SEL_L - [7:6] */
pub const WM8903_DIGMIC_CLK_SEL_L_SHIFT: u32 = 6;   /* DIGMIC_CLK_SEL_L - [7:6] */
pub const WM8903_DIGMIC_CLK_SEL_L_WIDTH: u32 = 2;   /* DIGMIC_CLK_SEL_L - [7:6] */
pub const WM8903_DIGMIC_CLK_SEL_R_MASK: u32 = 0x0030;   /* DIGMIC_CLK_SEL_R - [5:4] */
pub const WM8903_DIGMIC_CLK_SEL_R_SHIFT: u32 = 4;   /* DIGMIC_CLK_SEL_R - [5:4] */
pub const WM8903_DIGMIC_CLK_SEL_R_WIDTH: u32 = 2;   /* DIGMIC_CLK_SEL_R - [5:4] */
pub const WM8903_DIGMIC_CLK_SEL_RT_MASK: u32 = 0x000C;   /* DIGMIC_CLK_SEL_RT - [3:2] */
pub const WM8903_DIGMIC_CLK_SEL_RT_SHIFT: u32 = 2;   /* DIGMIC_CLK_SEL_RT - [3:2] */
pub const WM8903_DIGMIC_CLK_SEL_RT_WIDTH: u32 = 2;   /* DIGMIC_CLK_SEL_RT - [3:2] */
pub const WM8903_DIGMIC_CLK_SEL_MASK: u32 = 0x0003;   /* DIGMIC_CLK_SEL - [1:0] */
pub const WM8903_DIGMIC_CLK_SEL_SHIFT: u32 = 0;   /* DIGMIC_CLK_SEL - [1:0] */
pub const WM8903_DIGMIC_CLK_SEL_WIDTH: u32 = 2;   /* DIGMIC_CLK_SEL - [1:0] */

/*
 * R40 (0x28) - DRC 0
 */
pub const WM8903_DRC_ENA: u32 = 0x8000;   /* DRC_ENA */
pub const WM8903_DRC_ENA_MASK: u32 = 0x8000;   /* DRC_ENA */
pub const WM8903_DRC_ENA_SHIFT: u32 = 15;   /* DRC_ENA */
pub const WM8903_DRC_ENA_WIDTH: u32 = 1;   /* DRC_ENA */
pub const WM8903_DRC_THRESH_HYST_MASK: u32 = 0x1800;   /* DRC_THRESH_HYST - [12:11] */
pub const WM8903_DRC_THRESH_HYST_SHIFT: u32 = 11;   /* DRC_THRESH_HYST - [12:11] */
pub const WM8903_DRC_THRESH_HYST_WIDTH: u32 = 2;   /* DRC_THRESH_HYST - [12:11] */
pub const WM8903_DRC_STARTUP_GAIN_MASK: u32 = 0x07C0;   /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8903_DRC_STARTUP_GAIN_SHIFT: u32 = 6;   /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8903_DRC_STARTUP_GAIN_WIDTH: u32 = 5;   /* DRC_STARTUP_GAIN - [10:6] */
pub const WM8903_DRC_FF_DELAY: u32 = 0x0020;   /* DRC_FF_DELAY */
pub const WM8903_DRC_FF_DELAY_MASK: u32 = 0x0020;   /* DRC_FF_DELAY */
pub const WM8903_DRC_FF_DELAY_SHIFT: u32 = 5;   /* DRC_FF_DELAY */
pub const WM8903_DRC_FF_DELAY_WIDTH: u32 = 1;   /* DRC_FF_DELAY */
pub const WM8903_DRC_SMOOTH_ENA: u32 = 0x0008;   /* DRC_SMOOTH_ENA */
pub const WM8903_DRC_SMOOTH_ENA_MASK: u32 = 0x0008;   /* DRC_SMOOTH_ENA */
pub const WM8903_DRC_SMOOTH_ENA_SHIFT: u32 = 3;   /* DRC_SMOOTH_ENA */
pub const WM8903_DRC_SMOOTH_ENA_WIDTH: u32 = 1;   /* DRC_SMOOTH_ENA */
pub const WM8903_DRC_QR_ENA: u32 = 0x0004;   /* DRC_QR_ENA */
pub const WM8903_DRC_QR_ENA_MASK: u32 = 0x0004;   /* DRC_QR_ENA */
pub const WM8903_DRC_QR_ENA_SHIFT: u32 = 2;   /* DRC_QR_ENA */
pub const WM8903_DRC_QR_ENA_WIDTH: u32 = 1;   /* DRC_QR_ENA */
pub const WM8903_DRC_ANTICLIP_ENA: u32 = 0x0002;   /* DRC_ANTICLIP_ENA */
pub const WM8903_DRC_ANTICLIP_ENA_MASK: u32 = 0x0002;   /* DRC_ANTICLIP_ENA */
pub const WM8903_DRC_ANTICLIP_ENA_SHIFT: u32 = 1;   /* DRC_ANTICLIP_ENA */
pub const WM8903_DRC_ANTICLIP_ENA_WIDTH: u32 = 1;   /* DRC_ANTICLIP_ENA */
pub const WM8903_DRC_HYST_ENA: u32 = 0x0001;   /* DRC_HYST_ENA */
pub const WM8903_DRC_HYST_ENA_MASK: u32 = 0x0001;   /* DRC_HYST_ENA */
pub const WM8903_DRC_HYST_ENA_SHIFT: u32 = 0;   /* DRC_HYST_ENA */
pub const WM8903_DRC_HYST_ENA_WIDTH: u32 = 1;   /* DRC_HYST_ENA */

/*
 * R41 (0x29) - DRC 1
 */
pub const WM8903_DRC_ATTACK_RATE_MASK: u32 = 0xF000;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8903_DRC_ATTACK_RATE_SHIFT: u32 = 12;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8903_DRC_ATTACK_RATE_WIDTH: u32 = 4;   /* DRC_ATTACK_RATE - [15:12] */
pub const WM8903_DRC_DECAY_RATE_MASK: u32 = 0x0F00;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8903_DRC_DECAY_RATE_SHIFT: u32 = 8;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8903_DRC_DECAY_RATE_WIDTH: u32 = 4;   /* DRC_DECAY_RATE - [11:8] */
pub const WM8903_DRC_THRESH_QR_MASK: u32 = 0x00C0;   /* DRC_THRESH_QR - [7:6] */
pub const WM8903_DRC_THRESH_QR_SHIFT: u32 = 6;   /* DRC_THRESH_QR - [7:6] */
pub const WM8903_DRC_THRESH_QR_WIDTH: u32 = 2;   /* DRC_THRESH_QR - [7:6] */
pub const WM8903_DRC_RATE_QR_MASK: u32 = 0x0030;   /* DRC_RATE_QR - [5:4] */
pub const WM8903_DRC_RATE_QR_SHIFT: u32 = 4;   /* DRC_RATE_QR - [5:4] */
pub const WM8903_DRC_RATE_QR_WIDTH: u32 = 2;   /* DRC_RATE_QR - [5:4] */
pub const WM8903_DRC_MINGAIN_MASK: u32 = 0x000C;   /* DRC_MINGAIN - [3:2] */
pub const WM8903_DRC_MINGAIN_SHIFT: u32 = 2;   /* DRC_MINGAIN - [3:2] */
pub const WM8903_DRC_MINGAIN_WIDTH: u32 = 2;   /* DRC_MINGAIN - [3:2] */
pub const WM8903_DRC_MAXGAIN_MASK: u32 = 0x0003;   /* DRC_MAXGAIN - [1:0] */
pub const WM8903_DRC_MAXGAIN_SHIFT: u32 = 0;   /* DRC_MAXGAIN - [1:0] */
pub const WM8903_DRC_MAXGAIN_WIDTH: u32 = 2;   /* DRC_MAXGAIN - [1:0] */

/*
 * R42 (0x2A) - DRC 2
 */
pub const WM8903_DRC_R0_SLOPE_COMP_MASK: u32 = 0x0038;   /* DRC_R0_SLOPE_COMP - [5:3] */
pub const WM8903_DRC_R0_SLOPE_COMP_SHIFT: u32 = 3;   /* DRC_R0_SLOPE_COMP - [5:3] */
pub const WM8903_DRC_R0_SLOPE_COMP_WIDTH: u32 = 3;   /* DRC_R0_SLOPE_COMP - [5:3] */
pub const WM8903_DRC_R1_SLOPE_COMP_MASK: u32 = 0x0007;   /* DRC_R1_SLOPE_COMP - [2:0] */
pub const WM8903_DRC_R1_SLOPE_COMP_SHIFT: u32 = 0;   /* DRC_R1_SLOPE_COMP - [2:0] */
pub const WM8903_DRC_R1_SLOPE_COMP_WIDTH: u32 = 3;   /* DRC_R1_SLOPE_COMP - [2:0] */

/*
 * R43 (0x2B) - DRC 3
 */
pub const WM8903_DRC_THRESH_COMP_MASK: u32 = 0x07E0;   /* DRC_THRESH_COMP - [10:5] */
pub const WM8903_DRC_THRESH_COMP_SHIFT: u32 = 5;   /* DRC_THRESH_COMP - [10:5] */
pub const WM8903_DRC_THRESH_COMP_WIDTH: u32 = 6;   /* DRC_THRESH_COMP - [10:5] */
pub const WM8903_DRC_AMP_COMP_MASK: u32 = 0x001F;   /* DRC_AMP_COMP - [4:0] */
pub const WM8903_DRC_AMP_COMP_SHIFT: u32 = 0;   /* DRC_AMP_COMP - [4:0] */
pub const WM8903_DRC_AMP_COMP_WIDTH: u32 = 5;   /* DRC_AMP_COMP - [4:0] */

/*
 * R44 (0x2C) - Analogue Left Input 0
 */
pub const WM8903_LINMUTE: u32 = 0x0080;   /* LINMUTE */
pub const WM8903_LINMUTE_MASK: u32 = 0x0080;   /* LINMUTE */
pub const WM8903_LINMUTE_SHIFT: u32 = 7;   /* LINMUTE */
pub const WM8903_LINMUTE_WIDTH: u32 = 1;   /* LINMUTE */
pub const WM8903_LIN_VOL_MASK: u32 = 0x001F;   /* LIN_VOL - [4:0] */
pub const WM8903_LIN_VOL_SHIFT: u32 = 0;   /* LIN_VOL - [4:0] */
pub const WM8903_LIN_VOL_WIDTH: u32 = 5;   /* LIN_VOL - [4:0] */

/*
 * R45 (0x2D) - Analogue Right Input 0
 */
pub const WM8903_RINMUTE: u32 = 0x0080;   /* RINMUTE */
pub const WM8903_RINMUTE_MASK: u32 = 0x0080;   /* RINMUTE */
pub const WM8903_RINMUTE_SHIFT: u32 = 7;   /* RINMUTE */
pub const WM8903_RINMUTE_WIDTH: u32 = 1;   /* RINMUTE */
pub const WM8903_RIN_VOL_MASK: u32 = 0x001F;   /* RIN_VOL - [4:0] */
pub const WM8903_RIN_VOL_SHIFT: u32 = 0;   /* RIN_VOL - [4:0] */
pub const WM8903_RIN_VOL_WIDTH: u32 = 5;   /* RIN_VOL - [4:0] */

/*
 * R46 (0x2E) - Analogue Left Input 1
 */
pub const WM8903_INL_CM_ENA: u32 = 0x0040;   /* INL_CM_ENA */
pub const WM8903_INL_CM_ENA_MASK: u32 = 0x0040;   /* INL_CM_ENA */
pub const WM8903_INL_CM_ENA_SHIFT: u32 = 6;   /* INL_CM_ENA */
pub const WM8903_INL_CM_ENA_WIDTH: u32 = 1;   /* INL_CM_ENA */
pub const WM8903_L_IP_SEL_N_MASK: u32 = 0x0030;   /* L_IP_SEL_N - [5:4] */
pub const WM8903_L_IP_SEL_N_SHIFT: u32 = 4;   /* L_IP_SEL_N - [5:4] */
pub const WM8903_L_IP_SEL_N_WIDTH: u32 = 2;   /* L_IP_SEL_N - [5:4] */
pub const WM8903_L_IP_SEL_P_MASK: u32 = 0x000C;   /* L_IP_SEL_P - [3:2] */
pub const WM8903_L_IP_SEL_P_SHIFT: u32 = 2;   /* L_IP_SEL_P - [3:2] */
pub const WM8903_L_IP_SEL_P_WIDTH: u32 = 2;   /* L_IP_SEL_P - [3:2] */
pub const WM8903_L_MODE_MASK: u32 = 0x0003;   /* L_MODE - [1:0] */
pub const WM8903_L_MODE_SHIFT: u32 = 0;   /* L_MODE - [1:0] */
pub const WM8903_L_MODE_WIDTH: u32 = 2;   /* L_MODE - [1:0] */

/*
 * R47 (0x2F) - Analogue Right Input 1
 */
pub const WM8903_INR_CM_ENA: u32 = 0x0040;   /* INR_CM_ENA */
pub const WM8903_INR_CM_ENA_MASK: u32 = 0x0040;   /* INR_CM_ENA */
pub const WM8903_INR_CM_ENA_SHIFT: u32 = 6;   /* INR_CM_ENA */
pub const WM8903_INR_CM_ENA_WIDTH: u32 = 1;   /* INR_CM_ENA */
pub const WM8903_R_IP_SEL_N_MASK: u32 = 0x0030;   /* R_IP_SEL_N - [5:4] */
pub const WM8903_R_IP_SEL_N_SHIFT: u32 = 4;   /* R_IP_SEL_N - [5:4] */
pub const WM8903_R_IP_SEL_N_WIDTH: u32 = 2;   /* R_IP_SEL_N - [5:4] */
pub const WM8903_R_IP_SEL_P_MASK: u32 = 0x000C;   /* R_IP_SEL_P - [3:2] */
pub const WM8903_R_IP_SEL_P_SHIFT: u32 = 2;   /* R_IP_SEL_P - [3:2] */
pub const WM8903_R_IP_SEL_P_WIDTH: u32 = 2;   /* R_IP_SEL_P - [3:2] */
pub const WM8903_R_MODE_MASK: u32 = 0x0003;   /* R_MODE - [1:0] */
pub const WM8903_R_MODE_SHIFT: u32 = 0;   /* R_MODE - [1:0] */
pub const WM8903_R_MODE_WIDTH: u32 = 2;   /* R_MODE - [1:0] */

/*
 * R50 (0x32) - Analogue Left Mix 0
 */
pub const WM8903_DACL_TO_MIXOUTL: u32 = 0x0008;   /* DACL_TO_MIXOUTL */
pub const WM8903_DACL_TO_MIXOUTL_MASK: u32 = 0x0008;   /* DACL_TO_MIXOUTL */
pub const WM8903_DACL_TO_MIXOUTL_SHIFT: u32 = 3;   /* DACL_TO_MIXOUTL */
pub const WM8903_DACL_TO_MIXOUTL_WIDTH: u32 = 1;   /* DACL_TO_MIXOUTL */
pub const WM8903_DACR_TO_MIXOUTL: u32 = 0x0004;   /* DACR_TO_MIXOUTL */
pub const WM8903_DACR_TO_MIXOUTL_MASK: u32 = 0x0004;   /* DACR_TO_MIXOUTL */
pub const WM8903_DACR_TO_MIXOUTL_SHIFT: u32 = 2;   /* DACR_TO_MIXOUTL */
pub const WM8903_DACR_TO_MIXOUTL_WIDTH: u32 = 1;   /* DACR_TO_MIXOUTL */
pub const WM8903_BYPASSL_TO_MIXOUTL: u32 = 0x0002;   /* BYPASSL_TO_MIXOUTL */
pub const WM8903_BYPASSL_TO_MIXOUTL_MASK: u32 = 0x0002;   /* BYPASSL_TO_MIXOUTL */
pub const WM8903_BYPASSL_TO_MIXOUTL_SHIFT: u32 = 1;   /* BYPASSL_TO_MIXOUTL */
pub const WM8903_BYPASSL_TO_MIXOUTL_WIDTH: u32 = 1;   /* BYPASSL_TO_MIXOUTL */
pub const WM8903_BYPASSR_TO_MIXOUTL: u32 = 0x0001;   /* BYPASSR_TO_MIXOUTL */
pub const WM8903_BYPASSR_TO_MIXOUTL_MASK: u32 = 0x0001;   /* BYPASSR_TO_MIXOUTL */
pub const WM8903_BYPASSR_TO_MIXOUTL_SHIFT: u32 = 0;   /* BYPASSR_TO_MIXOUTL */
pub const WM8903_BYPASSR_TO_MIXOUTL_WIDTH: u32 = 1;   /* BYPASSR_TO_MIXOUTL */

/*
 * R51 (0x33) - Analogue Right Mix 0
 */
pub const WM8903_DACL_TO_MIXOUTR: u32 = 0x0008;   /* DACL_TO_MIXOUTR */
pub const WM8903_DACL_TO_MIXOUTR_MASK: u32 = 0x0008;   /* DACL_TO_MIXOUTR */
pub const WM8903_DACL_TO_MIXOUTR_SHIFT: u32 = 3;   /* DACL_TO_MIXOUTR */
pub const WM8903_DACL_TO_MIXOUTR_WIDTH: u32 = 1;   /* DACL_TO_MIXOUTR */
pub const WM8903_DACR_TO_MIXOUTR: u32 = 0x0004;   /* DACR_TO_MIXOUTR */
pub const WM8903_DACR_TO_MIXOUTR_MASK: u32 = 0x0004;   /* DACR_TO_MIXOUTR */
pub const WM8903_DACR_TO_MIXOUTR_SHIFT: u32 = 2;   /* DACR_TO_MIXOUTR */
pub const WM8903_DACR_TO_MIXOUTR_WIDTH: u32 = 1;   /* DACR_TO_MIXOUTR */
pub const WM8903_BYPASSL_TO_MIXOUTR: u32 = 0x0002;   /* BYPASSL_TO_MIXOUTR */
pub const WM8903_BYPASSL_TO_MIXOUTR_MASK: u32 = 0x0002;   /* BYPASSL_TO_MIXOUTR */
pub const WM8903_BYPASSL_TO_MIXOUTR_SHIFT: u32 = 1;   /* BYPASSL_TO_MIXOUTR */
pub const WM8903_BYPASSL_TO_MIXOUTR_WIDTH: u32 = 1;   /* BYPASSL_TO_MIXOUTR */
pub const WM8903_BYPASSR_TO_MIXOUTR: u32 = 0x0001;   /* BYPASSR_TO_MIXOUTR */
pub const WM8903_BYPASSR_TO_MIXOUTR_MASK: u32 = 0x0001;   /* BYPASSR_TO_MIXOUTR */
pub const WM8903_BYPASSR_TO_MIXOUTR_SHIFT: u32 = 0;   /* BYPASSR_TO_MIXOUTR */
pub const WM8903_BYPASSR_TO_MIXOUTR_WIDTH: u32 = 1;   /* BYPASSR_TO_MIXOUTR */

/*
 * R52 (0x34) - Analogue Spk Mix Left 0
 */
pub const WM8903_DACL_TO_MIXSPKL: u32 = 0x0008;   /* DACL_TO_MIXSPKL */
pub const WM8903_DACL_TO_MIXSPKL_MASK: u32 = 0x0008;   /* DACL_TO_MIXSPKL */
pub const WM8903_DACL_TO_MIXSPKL_SHIFT: u32 = 3;   /* DACL_TO_MIXSPKL */
pub const WM8903_DACL_TO_MIXSPKL_WIDTH: u32 = 1;   /* DACL_TO_MIXSPKL */
pub const WM8903_DACR_TO_MIXSPKL: u32 = 0x0004;   /* DACR_TO_MIXSPKL */
pub const WM8903_DACR_TO_MIXSPKL_MASK: u32 = 0x0004;   /* DACR_TO_MIXSPKL */
pub const WM8903_DACR_TO_MIXSPKL_SHIFT: u32 = 2;   /* DACR_TO_MIXSPKL */
pub const WM8903_DACR_TO_MIXSPKL_WIDTH: u32 = 1;   /* DACR_TO_MIXSPKL */
pub const WM8903_BYPASSL_TO_MIXSPKL: u32 = 0x0002;   /* BYPASSL_TO_MIXSPKL */
pub const WM8903_BYPASSL_TO_MIXSPKL_MASK: u32 = 0x0002;   /* BYPASSL_TO_MIXSPKL */
pub const WM8903_BYPASSL_TO_MIXSPKL_SHIFT: u32 = 1;   /* BYPASSL_TO_MIXSPKL */
pub const WM8903_BYPASSL_TO_MIXSPKL_WIDTH: u32 = 1;   /* BYPASSL_TO_MIXSPKL */
pub const WM8903_BYPASSR_TO_MIXSPKL: u32 = 0x0001;   /* BYPASSR_TO_MIXSPKL */
pub const WM8903_BYPASSR_TO_MIXSPKL_MASK: u32 = 0x0001;   /* BYPASSR_TO_MIXSPKL */
pub const WM8903_BYPASSR_TO_MIXSPKL_SHIFT: u32 = 0;   /* BYPASSR_TO_MIXSPKL */
pub const WM8903_BYPASSR_TO_MIXSPKL_WIDTH: u32 = 1;   /* BYPASSR_TO_MIXSPKL */

/*
 * R53 (0x35) - Analogue Spk Mix Left 1
 */
pub const WM8903_DACL_MIXSPKL_VOL: u32 = 0x0008;   /* DACL_MIXSPKL_VOL */
pub const WM8903_DACL_MIXSPKL_VOL_MASK: u32 = 0x0008;   /* DACL_MIXSPKL_VOL */
pub const WM8903_DACL_MIXSPKL_VOL_SHIFT: u32 = 3;   /* DACL_MIXSPKL_VOL */
pub const WM8903_DACL_MIXSPKL_VOL_WIDTH: u32 = 1;   /* DACL_MIXSPKL_VOL */
pub const WM8903_DACR_MIXSPKL_VOL: u32 = 0x0004;   /* DACR_MIXSPKL_VOL */
pub const WM8903_DACR_MIXSPKL_VOL_MASK: u32 = 0x0004;   /* DACR_MIXSPKL_VOL */
pub const WM8903_DACR_MIXSPKL_VOL_SHIFT: u32 = 2;   /* DACR_MIXSPKL_VOL */
pub const WM8903_DACR_MIXSPKL_VOL_WIDTH: u32 = 1;   /* DACR_MIXSPKL_VOL */
pub const WM8903_BYPASSL_MIXSPKL_VOL: u32 = 0x0002;   /* BYPASSL_MIXSPKL_VOL */
pub const WM8903_BYPASSL_MIXSPKL_VOL_MASK: u32 = 0x0002;   /* BYPASSL_MIXSPKL_VOL */
pub const WM8903_BYPASSL_MIXSPKL_VOL_SHIFT: u32 = 1;   /* BYPASSL_MIXSPKL_VOL */
pub const WM8903_BYPASSL_MIXSPKL_VOL_WIDTH: u32 = 1;   /* BYPASSL_MIXSPKL_VOL */
pub const WM8903_BYPASSR_MIXSPKL_VOL: u32 = 0x0001;   /* BYPASSR_MIXSPKL_VOL */
pub const WM8903_BYPASSR_MIXSPKL_VOL_MASK: u32 = 0x0001;   /* BYPASSR_MIXSPKL_VOL */
pub const WM8903_BYPASSR_MIXSPKL_VOL_SHIFT: u32 = 0;   /* BYPASSR_MIXSPKL_VOL */
pub const WM8903_BYPASSR_MIXSPKL_VOL_WIDTH: u32 = 1;   /* BYPASSR_MIXSPKL_VOL */

/*
 * R54 (0x36) - Analogue Spk Mix Right 0
 */
pub const WM8903_DACL_TO_MIXSPKR: u32 = 0x0008;   /* DACL_TO_MIXSPKR */
pub const WM8903_DACL_TO_MIXSPKR_MASK: u32 = 0x0008;   /* DACL_TO_MIXSPKR */
pub const WM8903_DACL_TO_MIXSPKR_SHIFT: u32 = 3;   /* DACL_TO_MIXSPKR */
pub const WM8903_DACL_TO_MIXSPKR_WIDTH: u32 = 1;   /* DACL_TO_MIXSPKR */
pub const WM8903_DACR_TO_MIXSPKR: u32 = 0x0004;   /* DACR_TO_MIXSPKR */
pub const WM8903_DACR_TO_MIXSPKR_MASK: u32 = 0x0004;   /* DACR_TO_MIXSPKR */
pub const WM8903_DACR_TO_MIXSPKR_SHIFT: u32 = 2;   /* DACR_TO_MIXSPKR */
pub const WM8903_DACR_TO_MIXSPKR_WIDTH: u32 = 1;   /* DACR_TO_MIXSPKR */
pub const WM8903_BYPASSL_TO_MIXSPKR: u32 = 0x0002;   /* BYPASSL_TO_MIXSPKR */
pub const WM8903_BYPASSL_TO_MIXSPKR_MASK: u32 = 0x0002;   /* BYPASSL_TO_MIXSPKR */
pub const WM8903_BYPASSL_TO_MIXSPKR_SHIFT: u32 = 1;   /* BYPASSL_TO_MIXSPKR */
pub const WM8903_BYPASSL_TO_MIXSPKR_WIDTH: u32 = 1;   /* BYPASSL_TO_MIXSPKR */
pub const WM8903_BYPASSR_TO_MIXSPKR: u32 = 0x0001;   /* BYPASSR_TO_MIXSPKR */
pub const WM8903_BYPASSR_TO_MIXSPKR_MASK: u32 = 0x0001;   /* BYPASSR_TO_MIXSPKR */
pub const WM8903_BYPASSR_TO_MIXSPKR_SHIFT: u32 = 0;   /* BYPASSR_TO_MIXSPKR */
pub const WM8903_BYPASSR_TO_MIXSPKR_WIDTH: u32 = 1;   /* BYPASSR_TO_MIXSPKR */

/*
 * R55 (0x37) - Analogue Spk Mix Right 1
 */
pub const WM8903_DACL_MIXSPKR_VOL: u32 = 0x0008;   /* DACL_MIXSPKR_VOL */
pub const WM8903_DACL_MIXSPKR_VOL_MASK: u32 = 0x0008;   /* DACL_MIXSPKR_VOL */
pub const WM8903_DACL_MIXSPKR_VOL_SHIFT: u32 = 3;   /* DACL_MIXSPKR_VOL */
pub const WM8903_DACL_MIXSPKR_VOL_WIDTH: u32 = 1;   /* DACL_MIXSPKR_VOL */
pub const WM8903_DACR_MIXSPKR_VOL: u32 = 0x0004;   /* DACR_MIXSPKR_VOL */
pub const WM8903_DACR_MIXSPKR_VOL_MASK: u32 = 0x0004;   /* DACR_MIXSPKR_VOL */
pub const WM8903_DACR_MIXSPKR_VOL_SHIFT: u32 = 2;   /* DACR_MIXSPKR_VOL */
pub const WM8903_DACR_MIXSPKR_VOL_WIDTH: u32 = 1;   /* DACR_MIXSPKR_VOL */
pub const WM8903_BYPASSL_MIXSPKR_VOL: u32 = 0x0002;   /* BYPASSL_MIXSPKR_VOL */
pub const WM8903_BYPASSL_MIXSPKR_VOL_MASK: u32 = 0x0002;   /* BYPASSL_MIXSPKR_VOL */
pub const WM8903_BYPASSL_MIXSPKR_VOL_SHIFT: u32 = 1;   /* BYPASSL_MIXSPKR_VOL */
pub const WM8903_BYPASSL_MIXSPKR_VOL_WIDTH: u32 = 1;   /* BYPASSL_MIXSPKR_VOL */
pub const WM8903_BYPASSR_MIXSPKR_VOL: u32 = 0x0001;   /* BYPASSR_MIXSPKR_VOL */
pub const WM8903_BYPASSR_MIXSPKR_VOL_MASK: u32 = 0x0001;   /* BYPASSR_MIXSPKR_VOL */
pub const WM8903_BYPASSR_MIXSPKR_VOL_SHIFT: u32 = 0;   /* BYPASSR_MIXSPKR_VOL */
pub const WM8903_BYPASSR_MIXSPKR_VOL_WIDTH: u32 = 1;   /* BYPASSR_MIXSPKR_VOL */

/*
 * R57 (0x39) - Analogue OUT1 Left
 */
pub const WM8903_HPL_MUTE: u32 = 0x0100;   /* HPL_MUTE */
pub const WM8903_HPL_MUTE_MASK: u32 = 0x0100;   /* HPL_MUTE */
pub const WM8903_HPL_MUTE_SHIFT: u32 = 8;   /* HPL_MUTE */
pub const WM8903_HPL_MUTE_WIDTH: u32 = 1;   /* HPL_MUTE */
pub const WM8903_HPOUTVU: u32 = 0x0080;   /* HPOUTVU */
pub const WM8903_HPOUTVU_MASK: u32 = 0x0080;   /* HPOUTVU */
pub const WM8903_HPOUTVU_SHIFT: u32 = 7;   /* HPOUTVU */
pub const WM8903_HPOUTVU_WIDTH: u32 = 1;   /* HPOUTVU */
pub const WM8903_HPOUTLZC: u32 = 0x0040;   /* HPOUTLZC */
pub const WM8903_HPOUTLZC_MASK: u32 = 0x0040;   /* HPOUTLZC */
pub const WM8903_HPOUTLZC_SHIFT: u32 = 6;   /* HPOUTLZC */
pub const WM8903_HPOUTLZC_WIDTH: u32 = 1;   /* HPOUTLZC */
pub const WM8903_HPOUTL_VOL_MASK: u32 = 0x003F;   /* HPOUTL_VOL - [5:0] */
pub const WM8903_HPOUTL_VOL_SHIFT: u32 = 0;   /* HPOUTL_VOL - [5:0] */
pub const WM8903_HPOUTL_VOL_WIDTH: u32 = 6;   /* HPOUTL_VOL - [5:0] */

/*
 * R58 (0x3A) - Analogue OUT1 Right
 */
pub const WM8903_HPR_MUTE: u32 = 0x0100;   /* HPR_MUTE */
pub const WM8903_HPR_MUTE_MASK: u32 = 0x0100;   /* HPR_MUTE */
pub const WM8903_HPR_MUTE_SHIFT: u32 = 8;   /* HPR_MUTE */
pub const WM8903_HPR_MUTE_WIDTH: u32 = 1;   /* HPR_MUTE */
// Duplicate C macro: #define WM8903_HPOUTVU 0x0080    /* HPOUTVU */
// Duplicate C macro: #define WM8903_HPOUTVU_MASK 0x0080    /* HPOUTVU */
// Duplicate C macro: #define WM8903_HPOUTVU_SHIFT 7    /* HPOUTVU */
// Duplicate C macro: #define WM8903_HPOUTVU_WIDTH 1    /* HPOUTVU */
pub const WM8903_HPOUTRZC: u32 = 0x0040;   /* HPOUTRZC */
pub const WM8903_HPOUTRZC_MASK: u32 = 0x0040;   /* HPOUTRZC */
pub const WM8903_HPOUTRZC_SHIFT: u32 = 6;   /* HPOUTRZC */
pub const WM8903_HPOUTRZC_WIDTH: u32 = 1;   /* HPOUTRZC */
pub const WM8903_HPOUTR_VOL_MASK: u32 = 0x003F;   /* HPOUTR_VOL - [5:0] */
pub const WM8903_HPOUTR_VOL_SHIFT: u32 = 0;   /* HPOUTR_VOL - [5:0] */
pub const WM8903_HPOUTR_VOL_WIDTH: u32 = 6;   /* HPOUTR_VOL - [5:0] */

/*
 * R59 (0x3B) - Analogue OUT2 Left
 */
pub const WM8903_LINEOUTL_MUTE: u32 = 0x0100;   /* LINEOUTL_MUTE */
pub const WM8903_LINEOUTL_MUTE_MASK: u32 = 0x0100;   /* LINEOUTL_MUTE */
pub const WM8903_LINEOUTL_MUTE_SHIFT: u32 = 8;   /* LINEOUTL_MUTE */
pub const WM8903_LINEOUTL_MUTE_WIDTH: u32 = 1;   /* LINEOUTL_MUTE */
pub const WM8903_LINEOUTVU: u32 = 0x0080;   /* LINEOUTVU */
pub const WM8903_LINEOUTVU_MASK: u32 = 0x0080;   /* LINEOUTVU */
pub const WM8903_LINEOUTVU_SHIFT: u32 = 7;   /* LINEOUTVU */
pub const WM8903_LINEOUTVU_WIDTH: u32 = 1;   /* LINEOUTVU */
pub const WM8903_LINEOUTLZC: u32 = 0x0040;   /* LINEOUTLZC */
pub const WM8903_LINEOUTLZC_MASK: u32 = 0x0040;   /* LINEOUTLZC */
pub const WM8903_LINEOUTLZC_SHIFT: u32 = 6;   /* LINEOUTLZC */
pub const WM8903_LINEOUTLZC_WIDTH: u32 = 1;   /* LINEOUTLZC */
pub const WM8903_LINEOUTL_VOL_MASK: u32 = 0x003F;   /* LINEOUTL_VOL - [5:0] */
pub const WM8903_LINEOUTL_VOL_SHIFT: u32 = 0;   /* LINEOUTL_VOL - [5:0] */
pub const WM8903_LINEOUTL_VOL_WIDTH: u32 = 6;   /* LINEOUTL_VOL - [5:0] */

/*
 * R60 (0x3C) - Analogue OUT2 Right
 */
pub const WM8903_LINEOUTR_MUTE: u32 = 0x0100;   /* LINEOUTR_MUTE */
pub const WM8903_LINEOUTR_MUTE_MASK: u32 = 0x0100;   /* LINEOUTR_MUTE */
pub const WM8903_LINEOUTR_MUTE_SHIFT: u32 = 8;   /* LINEOUTR_MUTE */
pub const WM8903_LINEOUTR_MUTE_WIDTH: u32 = 1;   /* LINEOUTR_MUTE */
// Duplicate C macro: #define WM8903_LINEOUTVU 0x0080    /* LINEOUTVU */
// Duplicate C macro: #define WM8903_LINEOUTVU_MASK 0x0080    /* LINEOUTVU */
// Duplicate C macro: #define WM8903_LINEOUTVU_SHIFT 7    /* LINEOUTVU */
// Duplicate C macro: #define WM8903_LINEOUTVU_WIDTH 1    /* LINEOUTVU */
pub const WM8903_LINEOUTRZC: u32 = 0x0040;   /* LINEOUTRZC */
pub const WM8903_LINEOUTRZC_MASK: u32 = 0x0040;   /* LINEOUTRZC */
pub const WM8903_LINEOUTRZC_SHIFT: u32 = 6;   /* LINEOUTRZC */
pub const WM8903_LINEOUTRZC_WIDTH: u32 = 1;   /* LINEOUTRZC */
pub const WM8903_LINEOUTR_VOL_MASK: u32 = 0x003F;   /* LINEOUTR_VOL - [5:0] */
pub const WM8903_LINEOUTR_VOL_SHIFT: u32 = 0;   /* LINEOUTR_VOL - [5:0] */
pub const WM8903_LINEOUTR_VOL_WIDTH: u32 = 6;   /* LINEOUTR_VOL - [5:0] */

/*
 * R62 (0x3E) - Analogue OUT3 Left
 */
pub const WM8903_SPKL_MUTE: u32 = 0x0100;   /* SPKL_MUTE */
pub const WM8903_SPKL_MUTE_MASK: u32 = 0x0100;   /* SPKL_MUTE */
pub const WM8903_SPKL_MUTE_SHIFT: u32 = 8;   /* SPKL_MUTE */
pub const WM8903_SPKL_MUTE_WIDTH: u32 = 1;   /* SPKL_MUTE */
pub const WM8903_SPKVU: u32 = 0x0080;   /* SPKVU */
pub const WM8903_SPKVU_MASK: u32 = 0x0080;   /* SPKVU */
pub const WM8903_SPKVU_SHIFT: u32 = 7;   /* SPKVU */
pub const WM8903_SPKVU_WIDTH: u32 = 1;   /* SPKVU */
pub const WM8903_SPKLZC: u32 = 0x0040;   /* SPKLZC */
pub const WM8903_SPKLZC_MASK: u32 = 0x0040;   /* SPKLZC */
pub const WM8903_SPKLZC_SHIFT: u32 = 6;   /* SPKLZC */
pub const WM8903_SPKLZC_WIDTH: u32 = 1;   /* SPKLZC */
pub const WM8903_SPKL_VOL_MASK: u32 = 0x003F;   /* SPKL_VOL - [5:0] */
pub const WM8903_SPKL_VOL_SHIFT: u32 = 0;   /* SPKL_VOL - [5:0] */
pub const WM8903_SPKL_VOL_WIDTH: u32 = 6;   /* SPKL_VOL - [5:0] */

/*
 * R63 (0x3F) - Analogue OUT3 Right
 */
pub const WM8903_SPKR_MUTE: u32 = 0x0100;   /* SPKR_MUTE */
pub const WM8903_SPKR_MUTE_MASK: u32 = 0x0100;   /* SPKR_MUTE */
pub const WM8903_SPKR_MUTE_SHIFT: u32 = 8;   /* SPKR_MUTE */
pub const WM8903_SPKR_MUTE_WIDTH: u32 = 1;   /* SPKR_MUTE */
// Duplicate C macro: #define WM8903_SPKVU 0x0080    /* SPKVU */
// Duplicate C macro: #define WM8903_SPKVU_MASK 0x0080    /* SPKVU */
// Duplicate C macro: #define WM8903_SPKVU_SHIFT 7    /* SPKVU */
// Duplicate C macro: #define WM8903_SPKVU_WIDTH 1    /* SPKVU */
pub const WM8903_SPKRZC: u32 = 0x0040;   /* SPKRZC */
pub const WM8903_SPKRZC_MASK: u32 = 0x0040;   /* SPKRZC */
pub const WM8903_SPKRZC_SHIFT: u32 = 6;   /* SPKRZC */
pub const WM8903_SPKRZC_WIDTH: u32 = 1;   /* SPKRZC */
pub const WM8903_SPKR_VOL_MASK: u32 = 0x003F;   /* SPKR_VOL - [5:0] */
pub const WM8903_SPKR_VOL_SHIFT: u32 = 0;   /* SPKR_VOL - [5:0] */
pub const WM8903_SPKR_VOL_WIDTH: u32 = 6;   /* SPKR_VOL - [5:0] */

/*
 * R65 (0x41) - Analogue SPK Output Control 0
 */
pub const WM8903_SPK_DISCHARGE: u32 = 0x0002;   /* SPK_DISCHARGE */
pub const WM8903_SPK_DISCHARGE_MASK: u32 = 0x0002;   /* SPK_DISCHARGE */
pub const WM8903_SPK_DISCHARGE_SHIFT: u32 = 1;   /* SPK_DISCHARGE */
pub const WM8903_SPK_DISCHARGE_WIDTH: u32 = 1;   /* SPK_DISCHARGE */
pub const WM8903_VROI: u32 = 0x0001;   /* VROI */
pub const WM8903_VROI_MASK: u32 = 0x0001;   /* VROI */
pub const WM8903_VROI_SHIFT: u32 = 0;   /* VROI */
pub const WM8903_VROI_WIDTH: u32 = 1;   /* VROI */

/*
 * R67 (0x43) - DC Servo 0
 */
pub const WM8903_DCS_MASTER_ENA: u32 = 0x0010;   /* DCS_MASTER_ENA */
pub const WM8903_DCS_MASTER_ENA_MASK: u32 = 0x0010;   /* DCS_MASTER_ENA */
pub const WM8903_DCS_MASTER_ENA_SHIFT: u32 = 4;   /* DCS_MASTER_ENA */
pub const WM8903_DCS_MASTER_ENA_WIDTH: u32 = 1;   /* DCS_MASTER_ENA */
pub const WM8903_DCS_ENA_MASK: u32 = 0x000F;   /* DCS_ENA - [3:0] */
pub const WM8903_DCS_ENA_SHIFT: u32 = 0;   /* DCS_ENA - [3:0] */
pub const WM8903_DCS_ENA_WIDTH: u32 = 4;   /* DCS_ENA - [3:0] */

/*
 * R69 (0x45) - DC Servo 2
 */
pub const WM8903_DCS_MODE_MASK: u32 = 0x0003;   /* DCS_MODE - [1:0] */
pub const WM8903_DCS_MODE_SHIFT: u32 = 0;   /* DCS_MODE - [1:0] */
pub const WM8903_DCS_MODE_WIDTH: u32 = 2;   /* DCS_MODE - [1:0] */

/*
 * R90 (0x5A) - Analogue HP 0
 */
pub const WM8903_HPL_RMV_SHORT: u32 = 0x0080;   /* HPL_RMV_SHORT */
pub const WM8903_HPL_RMV_SHORT_MASK: u32 = 0x0080;   /* HPL_RMV_SHORT */
pub const WM8903_HPL_RMV_SHORT_SHIFT: u32 = 7;   /* HPL_RMV_SHORT */
pub const WM8903_HPL_RMV_SHORT_WIDTH: u32 = 1;   /* HPL_RMV_SHORT */
pub const WM8903_HPL_ENA_OUTP: u32 = 0x0040;   /* HPL_ENA_OUTP */
pub const WM8903_HPL_ENA_OUTP_MASK: u32 = 0x0040;   /* HPL_ENA_OUTP */
pub const WM8903_HPL_ENA_OUTP_SHIFT: u32 = 6;   /* HPL_ENA_OUTP */
pub const WM8903_HPL_ENA_OUTP_WIDTH: u32 = 1;   /* HPL_ENA_OUTP */
pub const WM8903_HPL_ENA_DLY: u32 = 0x0020;   /* HPL_ENA_DLY */
pub const WM8903_HPL_ENA_DLY_MASK: u32 = 0x0020;   /* HPL_ENA_DLY */
pub const WM8903_HPL_ENA_DLY_SHIFT: u32 = 5;   /* HPL_ENA_DLY */
pub const WM8903_HPL_ENA_DLY_WIDTH: u32 = 1;   /* HPL_ENA_DLY */
pub const WM8903_HPL_ENA: u32 = 0x0010;   /* HPL_ENA */
pub const WM8903_HPL_ENA_MASK: u32 = 0x0010;   /* HPL_ENA */
pub const WM8903_HPL_ENA_SHIFT: u32 = 4;   /* HPL_ENA */
pub const WM8903_HPL_ENA_WIDTH: u32 = 1;   /* HPL_ENA */
pub const WM8903_HPR_RMV_SHORT: u32 = 0x0008;   /* HPR_RMV_SHORT */
pub const WM8903_HPR_RMV_SHORT_MASK: u32 = 0x0008;   /* HPR_RMV_SHORT */
pub const WM8903_HPR_RMV_SHORT_SHIFT: u32 = 3;   /* HPR_RMV_SHORT */
pub const WM8903_HPR_RMV_SHORT_WIDTH: u32 = 1;   /* HPR_RMV_SHORT */
pub const WM8903_HPR_ENA_OUTP: u32 = 0x0004;   /* HPR_ENA_OUTP */
pub const WM8903_HPR_ENA_OUTP_MASK: u32 = 0x0004;   /* HPR_ENA_OUTP */
pub const WM8903_HPR_ENA_OUTP_SHIFT: u32 = 2;   /* HPR_ENA_OUTP */
pub const WM8903_HPR_ENA_OUTP_WIDTH: u32 = 1;   /* HPR_ENA_OUTP */
pub const WM8903_HPR_ENA_DLY: u32 = 0x0002;   /* HPR_ENA_DLY */
pub const WM8903_HPR_ENA_DLY_MASK: u32 = 0x0002;   /* HPR_ENA_DLY */
pub const WM8903_HPR_ENA_DLY_SHIFT: u32 = 1;   /* HPR_ENA_DLY */
pub const WM8903_HPR_ENA_DLY_WIDTH: u32 = 1;   /* HPR_ENA_DLY */
pub const WM8903_HPR_ENA: u32 = 0x0001;   /* HPR_ENA */
pub const WM8903_HPR_ENA_MASK: u32 = 0x0001;   /* HPR_ENA */
pub const WM8903_HPR_ENA_SHIFT: u32 = 0;   /* HPR_ENA */
pub const WM8903_HPR_ENA_WIDTH: u32 = 1;   /* HPR_ENA */

/*
 * R94 (0x5E) - Analogue Lineout 0
 */
pub const WM8903_LINEOUTL_RMV_SHORT: u32 = 0x0080;   /* LINEOUTL_RMV_SHORT */
pub const WM8903_LINEOUTL_RMV_SHORT_MASK: u32 = 0x0080;   /* LINEOUTL_RMV_SHORT */
pub const WM8903_LINEOUTL_RMV_SHORT_SHIFT: u32 = 7;   /* LINEOUTL_RMV_SHORT */
pub const WM8903_LINEOUTL_RMV_SHORT_WIDTH: u32 = 1;   /* LINEOUTL_RMV_SHORT */
pub const WM8903_LINEOUTL_ENA_OUTP: u32 = 0x0040;   /* LINEOUTL_ENA_OUTP */
pub const WM8903_LINEOUTL_ENA_OUTP_MASK: u32 = 0x0040;   /* LINEOUTL_ENA_OUTP */
pub const WM8903_LINEOUTL_ENA_OUTP_SHIFT: u32 = 6;   /* LINEOUTL_ENA_OUTP */
pub const WM8903_LINEOUTL_ENA_OUTP_WIDTH: u32 = 1;   /* LINEOUTL_ENA_OUTP */
pub const WM8903_LINEOUTL_ENA_DLY: u32 = 0x0020;   /* LINEOUTL_ENA_DLY */
pub const WM8903_LINEOUTL_ENA_DLY_MASK: u32 = 0x0020;   /* LINEOUTL_ENA_DLY */
pub const WM8903_LINEOUTL_ENA_DLY_SHIFT: u32 = 5;   /* LINEOUTL_ENA_DLY */
pub const WM8903_LINEOUTL_ENA_DLY_WIDTH: u32 = 1;   /* LINEOUTL_ENA_DLY */
pub const WM8903_LINEOUTL_ENA: u32 = 0x0010;   /* LINEOUTL_ENA */
pub const WM8903_LINEOUTL_ENA_MASK: u32 = 0x0010;   /* LINEOUTL_ENA */
pub const WM8903_LINEOUTL_ENA_SHIFT: u32 = 4;   /* LINEOUTL_ENA */
pub const WM8903_LINEOUTL_ENA_WIDTH: u32 = 1;   /* LINEOUTL_ENA */
pub const WM8903_LINEOUTR_RMV_SHORT: u32 = 0x0008;   /* LINEOUTR_RMV_SHORT */
pub const WM8903_LINEOUTR_RMV_SHORT_MASK: u32 = 0x0008;   /* LINEOUTR_RMV_SHORT */
pub const WM8903_LINEOUTR_RMV_SHORT_SHIFT: u32 = 3;   /* LINEOUTR_RMV_SHORT */
pub const WM8903_LINEOUTR_RMV_SHORT_WIDTH: u32 = 1;   /* LINEOUTR_RMV_SHORT */
pub const WM8903_LINEOUTR_ENA_OUTP: u32 = 0x0004;   /* LINEOUTR_ENA_OUTP */
pub const WM8903_LINEOUTR_ENA_OUTP_MASK: u32 = 0x0004;   /* LINEOUTR_ENA_OUTP */
pub const WM8903_LINEOUTR_ENA_OUTP_SHIFT: u32 = 2;   /* LINEOUTR_ENA_OUTP */
pub const WM8903_LINEOUTR_ENA_OUTP_WIDTH: u32 = 1;   /* LINEOUTR_ENA_OUTP */
pub const WM8903_LINEOUTR_ENA_DLY: u32 = 0x0002;   /* LINEOUTR_ENA_DLY */
pub const WM8903_LINEOUTR_ENA_DLY_MASK: u32 = 0x0002;   /* LINEOUTR_ENA_DLY */
pub const WM8903_LINEOUTR_ENA_DLY_SHIFT: u32 = 1;   /* LINEOUTR_ENA_DLY */
pub const WM8903_LINEOUTR_ENA_DLY_WIDTH: u32 = 1;   /* LINEOUTR_ENA_DLY */
pub const WM8903_LINEOUTR_ENA: u32 = 0x0001;   /* LINEOUTR_ENA */
pub const WM8903_LINEOUTR_ENA_MASK: u32 = 0x0001;   /* LINEOUTR_ENA */
pub const WM8903_LINEOUTR_ENA_SHIFT: u32 = 0;   /* LINEOUTR_ENA */
pub const WM8903_LINEOUTR_ENA_WIDTH: u32 = 1;   /* LINEOUTR_ENA */

/*
 * R98 (0x62) - Charge Pump 0
 */
pub const WM8903_CP_ENA: u32 = 0x0001;   /* CP_ENA */
pub const WM8903_CP_ENA_MASK: u32 = 0x0001;   /* CP_ENA */
pub const WM8903_CP_ENA_SHIFT: u32 = 0;   /* CP_ENA */
pub const WM8903_CP_ENA_WIDTH: u32 = 1;   /* CP_ENA */

/*
 * R104 (0x68) - Class W 0
 */
pub const WM8903_CP_DYN_FREQ: u32 = 0x0002;   /* CP_DYN_FREQ */
pub const WM8903_CP_DYN_FREQ_MASK: u32 = 0x0002;   /* CP_DYN_FREQ */
pub const WM8903_CP_DYN_FREQ_SHIFT: u32 = 1;   /* CP_DYN_FREQ */
pub const WM8903_CP_DYN_FREQ_WIDTH: u32 = 1;   /* CP_DYN_FREQ */
pub const WM8903_CP_DYN_V: u32 = 0x0001;   /* CP_DYN_V */
pub const WM8903_CP_DYN_V_MASK: u32 = 0x0001;   /* CP_DYN_V */
pub const WM8903_CP_DYN_V_SHIFT: u32 = 0;   /* CP_DYN_V */
pub const WM8903_CP_DYN_V_WIDTH: u32 = 1;   /* CP_DYN_V */

/*
 * R108 (0x6C) - Write Sequencer 0
 */
pub const WM8903_WSEQ_ENA: u32 = 0x0100;   /* WSEQ_ENA */
pub const WM8903_WSEQ_ENA_MASK: u32 = 0x0100;   /* WSEQ_ENA */
pub const WM8903_WSEQ_ENA_SHIFT: u32 = 8;   /* WSEQ_ENA */
pub const WM8903_WSEQ_ENA_WIDTH: u32 = 1;   /* WSEQ_ENA */
pub const WM8903_WSEQ_WRITE_INDEX_MASK: u32 = 0x001F;   /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8903_WSEQ_WRITE_INDEX_SHIFT: u32 = 0;   /* WSEQ_WRITE_INDEX - [4:0] */
pub const WM8903_WSEQ_WRITE_INDEX_WIDTH: u32 = 5;   /* WSEQ_WRITE_INDEX - [4:0] */

/*
 * R109 (0x6D) - Write Sequencer 1
 */
pub const WM8903_WSEQ_DATA_WIDTH_MASK: u32 = 0x7000;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8903_WSEQ_DATA_WIDTH_SHIFT: u32 = 12;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8903_WSEQ_DATA_WIDTH_WIDTH: u32 = 3;   /* WSEQ_DATA_WIDTH - [14:12] */
pub const WM8903_WSEQ_DATA_START_MASK: u32 = 0x0F00;   /* WSEQ_DATA_START - [11:8] */
pub const WM8903_WSEQ_DATA_START_SHIFT: u32 = 8;   /* WSEQ_DATA_START - [11:8] */
pub const WM8903_WSEQ_DATA_START_WIDTH: u32 = 4;   /* WSEQ_DATA_START - [11:8] */
pub const WM8903_WSEQ_ADDR_MASK: u32 = 0x00FF;   /* WSEQ_ADDR - [7:0] */
pub const WM8903_WSEQ_ADDR_SHIFT: u32 = 0;   /* WSEQ_ADDR - [7:0] */
pub const WM8903_WSEQ_ADDR_WIDTH: u32 = 8;   /* WSEQ_ADDR - [7:0] */

/*
 * R110 (0x6E) - Write Sequencer 2
 */
pub const WM8903_WSEQ_EOS: u32 = 0x4000;   /* WSEQ_EOS */
pub const WM8903_WSEQ_EOS_MASK: u32 = 0x4000;   /* WSEQ_EOS */
pub const WM8903_WSEQ_EOS_SHIFT: u32 = 14;   /* WSEQ_EOS */
pub const WM8903_WSEQ_EOS_WIDTH: u32 = 1;   /* WSEQ_EOS */
pub const WM8903_WSEQ_DELAY_MASK: u32 = 0x0F00;   /* WSEQ_DELAY - [11:8] */
pub const WM8903_WSEQ_DELAY_SHIFT: u32 = 8;   /* WSEQ_DELAY - [11:8] */
pub const WM8903_WSEQ_DELAY_WIDTH: u32 = 4;   /* WSEQ_DELAY - [11:8] */
pub const WM8903_WSEQ_DATA_MASK: u32 = 0x00FF;   /* WSEQ_DATA - [7:0] */
pub const WM8903_WSEQ_DATA_SHIFT: u32 = 0;   /* WSEQ_DATA - [7:0] */
pub const WM8903_WSEQ_DATA_WIDTH: u32 = 8;   /* WSEQ_DATA - [7:0] */

/*
 * R111 (0x6F) - Write Sequencer 3
 */
pub const WM8903_WSEQ_ABORT: u32 = 0x0200;   /* WSEQ_ABORT */
pub const WM8903_WSEQ_ABORT_MASK: u32 = 0x0200;   /* WSEQ_ABORT */
pub const WM8903_WSEQ_ABORT_SHIFT: u32 = 9;   /* WSEQ_ABORT */
pub const WM8903_WSEQ_ABORT_WIDTH: u32 = 1;   /* WSEQ_ABORT */
pub const WM8903_WSEQ_START: u32 = 0x0100;   /* WSEQ_START */
pub const WM8903_WSEQ_START_MASK: u32 = 0x0100;   /* WSEQ_START */
pub const WM8903_WSEQ_START_SHIFT: u32 = 8;   /* WSEQ_START */
pub const WM8903_WSEQ_START_WIDTH: u32 = 1;   /* WSEQ_START */
pub const WM8903_WSEQ_START_INDEX_MASK: u32 = 0x003F;   /* WSEQ_START_INDEX - [5:0] */
pub const WM8903_WSEQ_START_INDEX_SHIFT: u32 = 0;   /* WSEQ_START_INDEX - [5:0] */
pub const WM8903_WSEQ_START_INDEX_WIDTH: u32 = 6;   /* WSEQ_START_INDEX - [5:0] */

/*
 * R112 (0x70) - Write Sequencer 4
 */
pub const WM8903_WSEQ_CURRENT_INDEX_MASK: u32 = 0x03F0;   /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8903_WSEQ_CURRENT_INDEX_SHIFT: u32 = 4;   /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8903_WSEQ_CURRENT_INDEX_WIDTH: u32 = 6;   /* WSEQ_CURRENT_INDEX - [9:4] */
pub const WM8903_WSEQ_BUSY: u32 = 0x0001;   /* WSEQ_BUSY */
pub const WM8903_WSEQ_BUSY_MASK: u32 = 0x0001;   /* WSEQ_BUSY */
pub const WM8903_WSEQ_BUSY_SHIFT: u32 = 0;   /* WSEQ_BUSY */
pub const WM8903_WSEQ_BUSY_WIDTH: u32 = 1;   /* WSEQ_BUSY */

/*
 * R114 (0x72) - Control Interface
 */
pub const WM8903_MASK_WRITE_ENA: u32 = 0x0001;   /* MASK_WRITE_ENA */
pub const WM8903_MASK_WRITE_ENA_MASK: u32 = 0x0001;   /* MASK_WRITE_ENA */
pub const WM8903_MASK_WRITE_ENA_SHIFT: u32 = 0;   /* MASK_WRITE_ENA */
pub const WM8903_MASK_WRITE_ENA_WIDTH: u32 = 1;   /* MASK_WRITE_ENA */

/*
 * R121 (0x79) - Interrupt Status 1
 */
pub const WM8903_MICSHRT_EINT: u32 = 0x8000;   /* MICSHRT_EINT */
pub const WM8903_MICSHRT_EINT_MASK: u32 = 0x8000;   /* MICSHRT_EINT */
pub const WM8903_MICSHRT_EINT_SHIFT: u32 = 15;   /* MICSHRT_EINT */
pub const WM8903_MICSHRT_EINT_WIDTH: u32 = 1;   /* MICSHRT_EINT */
pub const WM8903_MICDET_EINT: u32 = 0x4000;   /* MICDET_EINT */
pub const WM8903_MICDET_EINT_MASK: u32 = 0x4000;   /* MICDET_EINT */
pub const WM8903_MICDET_EINT_SHIFT: u32 = 14;   /* MICDET_EINT */
pub const WM8903_MICDET_EINT_WIDTH: u32 = 1;   /* MICDET_EINT */
pub const WM8903_WSEQ_BUSY_EINT: u32 = 0x2000;   /* WSEQ_BUSY_EINT */
pub const WM8903_WSEQ_BUSY_EINT_MASK: u32 = 0x2000;   /* WSEQ_BUSY_EINT */
pub const WM8903_WSEQ_BUSY_EINT_SHIFT: u32 = 13;   /* WSEQ_BUSY_EINT */
pub const WM8903_WSEQ_BUSY_EINT_WIDTH: u32 = 1;   /* WSEQ_BUSY_EINT */
pub const WM8903_GP5_EINT: u32 = 0x0010;   /* GP5_EINT */
pub const WM8903_GP5_EINT_MASK: u32 = 0x0010;   /* GP5_EINT */
pub const WM8903_GP5_EINT_SHIFT: u32 = 4;   /* GP5_EINT */
pub const WM8903_GP5_EINT_WIDTH: u32 = 1;   /* GP5_EINT */
pub const WM8903_GP4_EINT: u32 = 0x0008;   /* GP4_EINT */
pub const WM8903_GP4_EINT_MASK: u32 = 0x0008;   /* GP4_EINT */
pub const WM8903_GP4_EINT_SHIFT: u32 = 3;   /* GP4_EINT */
pub const WM8903_GP4_EINT_WIDTH: u32 = 1;   /* GP4_EINT */
pub const WM8903_GP3_EINT: u32 = 0x0004;   /* GP3_EINT */
pub const WM8903_GP3_EINT_MASK: u32 = 0x0004;   /* GP3_EINT */
pub const WM8903_GP3_EINT_SHIFT: u32 = 2;   /* GP3_EINT */
pub const WM8903_GP3_EINT_WIDTH: u32 = 1;   /* GP3_EINT */
pub const WM8903_GP2_EINT: u32 = 0x0002;   /* GP2_EINT */
pub const WM8903_GP2_EINT_MASK: u32 = 0x0002;   /* GP2_EINT */
pub const WM8903_GP2_EINT_SHIFT: u32 = 1;   /* GP2_EINT */
pub const WM8903_GP2_EINT_WIDTH: u32 = 1;   /* GP2_EINT */
pub const WM8903_GP1_EINT: u32 = 0x0001;   /* GP1_EINT */
pub const WM8903_GP1_EINT_MASK: u32 = 0x0001;   /* GP1_EINT */
pub const WM8903_GP1_EINT_SHIFT: u32 = 0;   /* GP1_EINT */
pub const WM8903_GP1_EINT_WIDTH: u32 = 1;   /* GP1_EINT */

/*
 * R122 (0x7A) - Interrupt Status 1 Mask
 */
pub const WM8903_IM_MICSHRT_EINT: u32 = 0x8000;   /* IM_MICSHRT_EINT */
pub const WM8903_IM_MICSHRT_EINT_MASK: u32 = 0x8000;   /* IM_MICSHRT_EINT */
pub const WM8903_IM_MICSHRT_EINT_SHIFT: u32 = 15;   /* IM_MICSHRT_EINT */
pub const WM8903_IM_MICSHRT_EINT_WIDTH: u32 = 1;   /* IM_MICSHRT_EINT */
pub const WM8903_IM_MICDET_EINT: u32 = 0x4000;   /* IM_MICDET_EINT */
pub const WM8903_IM_MICDET_EINT_MASK: u32 = 0x4000;   /* IM_MICDET_EINT */
pub const WM8903_IM_MICDET_EINT_SHIFT: u32 = 14;   /* IM_MICDET_EINT */
pub const WM8903_IM_MICDET_EINT_WIDTH: u32 = 1;   /* IM_MICDET_EINT */
pub const WM8903_IM_WSEQ_BUSY_EINT: u32 = 0x2000;   /* IM_WSEQ_BUSY_EINT */
pub const WM8903_IM_WSEQ_BUSY_EINT_MASK: u32 = 0x2000;   /* IM_WSEQ_BUSY_EINT */
pub const WM8903_IM_WSEQ_BUSY_EINT_SHIFT: u32 = 13;   /* IM_WSEQ_BUSY_EINT */
pub const WM8903_IM_WSEQ_BUSY_EINT_WIDTH: u32 = 1;   /* IM_WSEQ_BUSY_EINT */
pub const WM8903_IM_GP5_EINT: u32 = 0x0010;   /* IM_GP5_EINT */
pub const WM8903_IM_GP5_EINT_MASK: u32 = 0x0010;   /* IM_GP5_EINT */
pub const WM8903_IM_GP5_EINT_SHIFT: u32 = 4;   /* IM_GP5_EINT */
pub const WM8903_IM_GP5_EINT_WIDTH: u32 = 1;   /* IM_GP5_EINT */
pub const WM8903_IM_GP4_EINT: u32 = 0x0008;   /* IM_GP4_EINT */
pub const WM8903_IM_GP4_EINT_MASK: u32 = 0x0008;   /* IM_GP4_EINT */
pub const WM8903_IM_GP4_EINT_SHIFT: u32 = 3;   /* IM_GP4_EINT */
pub const WM8903_IM_GP4_EINT_WIDTH: u32 = 1;   /* IM_GP4_EINT */
pub const WM8903_IM_GP3_EINT: u32 = 0x0004;   /* IM_GP3_EINT */
pub const WM8903_IM_GP3_EINT_MASK: u32 = 0x0004;   /* IM_GP3_EINT */
pub const WM8903_IM_GP3_EINT_SHIFT: u32 = 2;   /* IM_GP3_EINT */
pub const WM8903_IM_GP3_EINT_WIDTH: u32 = 1;   /* IM_GP3_EINT */
pub const WM8903_IM_GP2_EINT: u32 = 0x0002;   /* IM_GP2_EINT */
pub const WM8903_IM_GP2_EINT_MASK: u32 = 0x0002;   /* IM_GP2_EINT */
pub const WM8903_IM_GP2_EINT_SHIFT: u32 = 1;   /* IM_GP2_EINT */
pub const WM8903_IM_GP2_EINT_WIDTH: u32 = 1;   /* IM_GP2_EINT */
pub const WM8903_IM_GP1_EINT: u32 = 0x0001;   /* IM_GP1_EINT */
pub const WM8903_IM_GP1_EINT_MASK: u32 = 0x0001;   /* IM_GP1_EINT */
pub const WM8903_IM_GP1_EINT_SHIFT: u32 = 0;   /* IM_GP1_EINT */
pub const WM8903_IM_GP1_EINT_WIDTH: u32 = 1;   /* IM_GP1_EINT */

/*
 * R123 (0x7B) - Interrupt Polarity 1
 */
pub const WM8903_MICSHRT_INV: u32 = 0x8000;   /* MICSHRT_INV */
pub const WM8903_MICSHRT_INV_MASK: u32 = 0x8000;   /* MICSHRT_INV */
pub const WM8903_MICSHRT_INV_SHIFT: u32 = 15;   /* MICSHRT_INV */
pub const WM8903_MICSHRT_INV_WIDTH: u32 = 1;   /* MICSHRT_INV */
pub const WM8903_MICDET_INV: u32 = 0x4000;   /* MICDET_INV */
pub const WM8903_MICDET_INV_MASK: u32 = 0x4000;   /* MICDET_INV */
pub const WM8903_MICDET_INV_SHIFT: u32 = 14;   /* MICDET_INV */
pub const WM8903_MICDET_INV_WIDTH: u32 = 1;   /* MICDET_INV */

/*
 * R126 (0x7E) - Interrupt Control
 */
pub const WM8903_IRQ_POL: u32 = 0x0001;   /* IRQ_POL */
pub const WM8903_IRQ_POL_MASK: u32 = 0x0001;   /* IRQ_POL */
pub const WM8903_IRQ_POL_SHIFT: u32 = 0;   /* IRQ_POL */
pub const WM8903_IRQ_POL_WIDTH: u32 = 1;   /* IRQ_POL */

/*
 * R164 (0xA4) - Clock Rate Test 4
 */
pub const WM8903_ADC_DIG_MIC: u32 = 0x0200;   /* ADC_DIG_MIC */
pub const WM8903_ADC_DIG_MIC_MASK: u32 = 0x0200;   /* ADC_DIG_MIC */
pub const WM8903_ADC_DIG_MIC_SHIFT: u32 = 9;   /* ADC_DIG_MIC */
pub const WM8903_ADC_DIG_MIC_WIDTH: u32 = 1;   /* ADC_DIG_MIC */

/*
 * R172 (0xAC) - Analogue Output Bias 0
 */
pub const WM8903_PGA_BIAS_MASK: u32 = 0x0070;   /* PGA_BIAS - [6:4] */
pub const WM8903_PGA_BIAS_SHIFT: u32 = 4;   /* PGA_BIAS - [6:4] */
pub const WM8903_PGA_BIAS_WIDTH: u32 = 3;   /* PGA_BIAS - [6:4] */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
