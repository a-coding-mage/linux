/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Platform data for Madera codec driver
 *
 * Copyright (C) 2016-2019 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

// Dependency intent: u32 corresponds to the Linux kernel's __u32/u32 type.

pub const MADERA_MAX_INPUT: usize = 6;
pub const MADERA_MAX_MUXED_CHANNELS: usize = 4;
pub const MADERA_MAX_OUTPUT: usize = 6;
pub const MADERA_MAX_AIF: usize = 4;
pub const MADERA_MAX_PDM_SPK: usize = 2;
pub const MADERA_MAX_DSP: usize = 7;

/**
 * struct madera_codec_pdata
 *
 * @max_channels_clocked: Maximum number of channels that I2S clocks will be
 *                       generated for. Useful when clock master for systems
 *                       where the I2S bus has multiple data lines.
 * @dmic_ref:            Indicates how the MICBIAS pins have been externally
 *                       connected to DMICs on each input. A value of 0
 *                       indicates MICVDD and is the default. Other values are:
 *                       For CS47L35 one of the CS47L35_DMIC_REF_xxx values
 *                       For all other codecs one of the MADERA_DMIC_REF_xxx
 *                       Also see the datasheet for a description of the
 *                       INn_DMIC_SUP field.
 * @inmode:              Mode for the ADC inputs. One of the MADERA_INMODE_xxx
 *                       values. Two-dimensional array
 *                       [input_number][channel number], with four slots per
 *                       input in the order
 *                       [n][0]=INnAL [n][1]=INnAR [n][2]=INnBL [n][3]=INnBR
 * @out_mono:            For each output set the value to TRUE to indicate that
 *                       the output is mono. [0]=OUT1, [1]=OUT2, ...
 * @pdm_fmt:             PDM speaker data format. See the PDM_SPKn_FMT field in
 *                       the datasheet for a description of this value.
 * @pdm_mute:            PDM mute format. See the PDM_SPKn_CTRL_1 register
 *                       in the datasheet for a description of this value.
 */
#[repr(C)]
pub struct madera_codec_pdata {
    pub max_channels_clocked: [u32; MADERA_MAX_AIF],
    pub dmic_ref: [u32; MADERA_MAX_INPUT],
    pub inmode: [[u32; MADERA_MAX_MUXED_CHANNELS]; MADERA_MAX_INPUT],
    pub out_mono: [bool; MADERA_MAX_OUTPUT],
    pub pdm_fmt: [u32; MADERA_MAX_PDM_SPK],
    pub pdm_mute: [u32; MADERA_MAX_PDM_SPK],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
