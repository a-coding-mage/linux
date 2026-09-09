/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    m52790.h - definition for m52790 inputs and outputs

    Copyright (C) 2007 Hans Verkuil (hverkuil@kernel.org)

*/

/* Input routing switch 1 */

pub const M52790_SW1_IN_MASK: u16 = 0x0003;
pub const M52790_SW1_IN_TUNER: u16 = 0x0000;
pub const M52790_SW1_IN_V2: u16 = 0x0001;
pub const M52790_SW1_IN_V3: u16 = 0x0002;
pub const M52790_SW1_IN_V4: u16 = 0x0003;

/* Selects component input instead of composite */
pub const M52790_SW1_YCMIX: u16 = 0x0004;

/* Input routing switch 2 */

pub const M52790_SW2_IN_MASK: u16 = 0x0300;
pub const M52790_SW2_IN_TUNER: u16 = 0x0000;
pub const M52790_SW2_IN_V2: u16 = 0x0100;
pub const M52790_SW2_IN_V3: u16 = 0x0200;
pub const M52790_SW2_IN_V4: u16 = 0x0300;

/* Selects component input instead of composite */
pub const M52790_SW2_YCMIX: u16 = 0x0400;

/* Output routing switch 1 */

/* Enable 6dB amplifier for composite out */
pub const M52790_SW1_V_AMP: u16 = 0x0008;

/* Enable 6dB amplifier for component out */
pub const M52790_SW1_YC_AMP: u16 = 0x0010;

/* Audio output mode */
pub const M52790_SW1_AUDIO_MASK: u16 = 0x00c0;
pub const M52790_SW1_AUDIO_MUTE: u16 = 0x0000;
pub const M52790_SW1_AUDIO_R: u16 = 0x0040;
pub const M52790_SW1_AUDIO_L: u16 = 0x0080;
pub const M52790_SW1_AUDIO_STEREO: u16 = 0x00c0;

/* Output routing switch 2 */

/* Enable 6dB amplifier for composite out */
pub const M52790_SW2_V_AMP: u16 = 0x0800;

/* Enable 6dB amplifier for component out */
pub const M52790_SW2_YC_AMP: u16 = 0x1000;

/* Audio output mode */
pub const M52790_SW2_AUDIO_MASK: u16 = 0xc000;
pub const M52790_SW2_AUDIO_MUTE: u16 = 0x0000;
pub const M52790_SW2_AUDIO_R: u16 = 0x4000;
pub const M52790_SW2_AUDIO_L: u16 = 0x8000;
pub const M52790_SW2_AUDIO_STEREO: u16 = 0xc000;

/* Common values */
pub const M52790_IN_TUNER: u16 = M52790_SW1_IN_TUNER | M52790_SW2_IN_TUNER;
pub const M52790_IN_V2: u16 = M52790_SW1_IN_V2 | M52790_SW2_IN_V2;
pub const M52790_IN_V3: u16 = M52790_SW1_IN_V3 | M52790_SW2_IN_V3;
pub const M52790_IN_V4: u16 = M52790_SW1_IN_V4 | M52790_SW2_IN_V4;

pub const M52790_OUT_STEREO: u16 = M52790_SW1_AUDIO_STEREO | M52790_SW2_AUDIO_STEREO;
pub const M52790_OUT_AMP_STEREO: u16 = M52790_SW1_AUDIO_STEREO
    | M52790_SW1_V_AMP
    | M52790_SW2_AUDIO_STEREO
    | M52790_SW2_V_AMP;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
