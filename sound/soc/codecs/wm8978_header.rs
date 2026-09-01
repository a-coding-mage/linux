/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8978.h		--  codec driver for WM8978
 *
 * Copyright 2009 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

/*
 * Register values.
 */
pub const WM8978_RESET: u32 = 0x00;
pub const WM8978_POWER_MANAGEMENT_1: u32 = 0x01;
pub const WM8978_POWER_MANAGEMENT_2: u32 = 0x02;
pub const WM8978_POWER_MANAGEMENT_3: u32 = 0x03;
pub const WM8978_AUDIO_INTERFACE: u32 = 0x04;
pub const WM8978_COMPANDING_CONTROL: u32 = 0x05;
pub const WM8978_CLOCKING: u32 = 0x06;
pub const WM8978_ADDITIONAL_CONTROL: u32 = 0x07;
pub const WM8978_GPIO_CONTROL: u32 = 0x08;
pub const WM8978_JACK_DETECT_CONTROL_1: u32 = 0x09;
pub const WM8978_DAC_CONTROL: u32 = 0x0A;
pub const WM8978_LEFT_DAC_DIGITAL_VOLUME: u32 = 0x0B;
pub const WM8978_RIGHT_DAC_DIGITAL_VOLUME: u32 = 0x0C;
pub const WM8978_JACK_DETECT_CONTROL_2: u32 = 0x0D;
pub const WM8978_ADC_CONTROL: u32 = 0x0E;
pub const WM8978_LEFT_ADC_DIGITAL_VOLUME: u32 = 0x0F;
pub const WM8978_RIGHT_ADC_DIGITAL_VOLUME: u32 = 0x10;
pub const WM8978_EQ1: u32 = 0x12;
pub const WM8978_EQ2: u32 = 0x13;
pub const WM8978_EQ3: u32 = 0x14;
pub const WM8978_EQ4: u32 = 0x15;
pub const WM8978_EQ5: u32 = 0x16;
pub const WM8978_DAC_LIMITER_1: u32 = 0x18;
pub const WM8978_DAC_LIMITER_2: u32 = 0x19;
pub const WM8978_NOTCH_FILTER_1: u32 = 0x1b;
pub const WM8978_NOTCH_FILTER_2: u32 = 0x1c;
pub const WM8978_NOTCH_FILTER_3: u32 = 0x1d;
pub const WM8978_NOTCH_FILTER_4: u32 = 0x1e;
pub const WM8978_ALC_CONTROL_1: u32 = 0x20;
pub const WM8978_ALC_CONTROL_2: u32 = 0x21;
pub const WM8978_ALC_CONTROL_3: u32 = 0x22;
pub const WM8978_NOISE_GATE: u32 = 0x23;
pub const WM8978_PLL_N: u32 = 0x24;
pub const WM8978_PLL_K1: u32 = 0x25;
pub const WM8978_PLL_K2: u32 = 0x26;
pub const WM8978_PLL_K3: u32 = 0x27;
pub const WM8978_3D_CONTROL: u32 = 0x29;
pub const WM8978_BEEP_CONTROL: u32 = 0x2b;
pub const WM8978_INPUT_CONTROL: u32 = 0x2c;
pub const WM8978_LEFT_INP_PGA_CONTROL: u32 = 0x2d;
pub const WM8978_RIGHT_INP_PGA_CONTROL: u32 = 0x2e;
pub const WM8978_LEFT_ADC_BOOST_CONTROL: u32 = 0x2f;
pub const WM8978_RIGHT_ADC_BOOST_CONTROL: u32 = 0x30;
pub const WM8978_OUTPUT_CONTROL: u32 = 0x31;
pub const WM8978_LEFT_MIXER_CONTROL: u32 = 0x32;
pub const WM8978_RIGHT_MIXER_CONTROL: u32 = 0x33;
pub const WM8978_LOUT1_HP_CONTROL: u32 = 0x34;
pub const WM8978_ROUT1_HP_CONTROL: u32 = 0x35;
pub const WM8978_LOUT2_SPK_CONTROL: u32 = 0x36;
pub const WM8978_ROUT2_SPK_CONTROL: u32 = 0x37;
pub const WM8978_OUT3_MIXER_CONTROL: u32 = 0x38;
pub const WM8978_OUT4_MIXER_CONTROL: u32 = 0x39;

pub const WM8978_MAX_REGISTER: u32 = 0x39;

pub const WM8978_CACHEREGNUM: u32 = 58;

/* Clock divider Id's */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wm8978_clk_id {
    WM8978_OPCLKRATE,
    WM8978_BCLKDIV,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wm8978_sysclk_src {
    WM8978_MCLK = 0,
    WM8978_PLL,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
