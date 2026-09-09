/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>
 *         for ST-Ericsson.
 *
 * License terms:
 */

/* Mic-types */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amic_type {
    AMIC_TYPE_SINGLE_ENDED,
    AMIC_TYPE_DIFFERENTIAL,
}

/* Mic-biases */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amic_micbias {
    AMIC_MICBIAS_VAMIC1,
    AMIC_MICBIAS_VAMIC2,
    AMIC_MICBIAS_UNKNOWN,
}

/* Bias-voltage */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ear_cm_voltage {
    EAR_CMV_0_95V,
    EAR_CMV_1_10V,
    EAR_CMV_1_27V,
    EAR_CMV_1_58V,
    EAR_CMV_UNKNOWN,
}

/* Analog microphone settings */
#[repr(C)]
pub struct amic_settings {
    pub mic1_type: amic_type,
    pub mic2_type: amic_type,
    pub mic1a_micbias: amic_micbias,
    pub mic1b_micbias: amic_micbias,
    pub mic2_micbias: amic_micbias,
}

/* Platform data structure for the audio-parts of the AB8500 */
#[repr(C)]
pub struct ab8500_codec_platform_data {
    pub amics: amic_settings,
    pub ear_cmv: ear_cm_voltage,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
