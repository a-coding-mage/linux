/* SPDX-License-Identifier: GPL-2.0 */
/*
 * descriptions for simple tuners.
 */

/**
 * enum param_type - type of the tuner pameters
 *
 * @TUNER_PARAM_TYPE_RADIO: Tuner params are for FM and/or AM radio
 * @TUNER_PARAM_TYPE_PAL: Tuner params are for PAL color TV standard
 * @TUNER_PARAM_TYPE_SECAM: Tuner params are for SECAM color TV standard
 * @TUNER_PARAM_TYPE_NTSC: Tuner params are for NTSC color TV standard
 * @TUNER_PARAM_TYPE_DIGITAL: Tuner params are for digital TV
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum param_type {
    TUNER_PARAM_TYPE_RADIO,
    TUNER_PARAM_TYPE_PAL,
    TUNER_PARAM_TYPE_SECAM,
    TUNER_PARAM_TYPE_NTSC,
    TUNER_PARAM_TYPE_DIGITAL,
}

/**
 * struct tuner_range - define the frequencies supported by the tuner
 *
 * @limit: Max frequency supported by that range, in 62.5 kHz (TV) or 62.5 Hz
 * (Radio), as defined by V4L2_TUNER_CAP_LOW.
 * @config: Value of the band switch byte (BB) to setup this mode.
 * @cb: Value of the CB byte to setup this mode.
 */
#[repr(C)]
pub struct tuner_range {
    pub limit: u16,
    pub config: u8,
    pub cb: u8,
}

/**
 * struct tuner_params - Parameters to be used to setup the tuner.
 * The remaining field descriptions are preserved from the C declaration.
 */
#[repr(C)]
pub struct tuner_params {
    pub r#type: param_type,

    // C bit-fields, each occupying one bit (or the specified width).
    pub cb_first_if_lower_freq: u32,
    pub has_tda9887: u32,
    pub port1_fm_high_sensitivity: u32,
    pub port2_fm_high_sensitivity: u32,
    pub fm_gain_normal: u32,
    pub intercarrier_mode: u32,
    pub port1_active: u32,
    pub port2_active: u32,
    pub port1_invert_for_secam_lc: u32,
    pub port2_invert_for_secam_lc: u32,
    pub port1_set_for_fm_mono: u32,
    pub default_pll_gating_18: u32,
    pub radio_if: u32,
    pub default_top_low: i32,
    pub default_top_mid: i32,
    pub default_top_high: i32,
    pub default_top_secam_low: i32,
    pub default_top_secam_mid: i32,
    pub default_top_secam_high: i32,

    pub iffreq: u16,
    pub count: u32,
    pub ranges: *const tuner_range,
}

/**
 * struct tunertype - describes the known tuners.
 */
#[repr(C)]
pub struct tunertype {
    pub name: *mut i8,
    pub count: u32,
    pub params: *const tuner_params,

    pub min: u16,
    pub max: u16,
    pub stepsize: u32,

    pub initdata: *mut u8,
    pub sleepdata: *mut u8,
}

unsafe extern "C" {
    pub static tuners: [tunertype; 0];
    pub static tuner_count: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
