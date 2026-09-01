/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctmixer.h
 *
 * @Brief
 * This file contains the definition of the mixer device functions.
 *
 * @Author	Liu Chun
 * @Date 	Mar 28 2008
 */

/* Dependencies from the original header:
 * #include "ctatc.h"
 * #include "ctresource.h"
 */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct ct_atc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amixer {
    _private: [u8; 0],
}

/* enum CTALSADEVS is supplied by ctatc.h. */
pub type CTALSADEVS = c_uint;

pub const INIT_VOL: c_uint = 0x1c00;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MIXER_PORT_T {
    MIX_WAVE_FRONT = 0,
    MIX_WAVE_REAR = 1,
    MIX_WAVE_CENTLFE = 2,
    MIX_WAVE_SURROUND = 3,
    MIX_SPDIF_OUT = 4,
    MIX_PCMO_FRONT = 5,
    MIX_MIC_IN = 6,
    MIX_LINE_IN = 7,
    MIX_SPDIF_IN = 8,
    MIX_PCMI_FRONT = 9,
    MIX_PCMI_REAR = 10,
    MIX_PCMI_CENTLFE = 11,
    MIX_PCMI_SURROUND = 12,

    NUM_MIX_PORTS = 13,
}

/* alsa mixer descriptor */
#[repr(C)]
pub struct ct_mixer {
    pub atc: *mut ct_atc,

    pub sums: *mut *mut sum, /* sum resources for signal collection */
    pub line_mic_kctls: [*mut snd_kcontrol; 2], /* line/mic capture switch controls */
    pub switch_state: c_uint, /* A bit-map to indicate state of switches */

    pub get_output_ports: Option<
        unsafe extern "C" fn(
            mixer: *mut ct_mixer,
            type_: MIXER_PORT_T,
            rleft: *mut *mut rsc,
            rright: *mut *mut rsc,
        ) -> c_int,
    >,

    pub set_input_left: Option<
        unsafe extern "C" fn(
            mixer: *mut ct_mixer,
            type_: MIXER_PORT_T,
            rsc: *mut rsc,
        ) -> c_int,
    >,
    pub set_input_right: Option<
        unsafe extern "C" fn(
            mixer: *mut ct_mixer,
            type_: MIXER_PORT_T,
            rsc: *mut rsc,
        ) -> c_int,
    >,

    /* Present in C only when CONFIG_PM_SLEEP is enabled:
     * int (*resume)(struct ct_mixer *mixer);
     */
    #[cfg(CONFIG_PM_SLEEP)]
    pub resume: Option<unsafe extern "C" fn(mixer: *mut ct_mixer) -> c_int>,

    pub amixers: [*mut amixer; 0], /* amixer resources for volume control */
}

unsafe extern "C" {
    pub fn ct_alsa_mix_create(
        atc: *mut ct_atc,
        device: CTALSADEVS,
        device_name: *const c_char,
    ) -> c_int;
    pub fn ct_mixer_create(atc: *mut ct_atc, rmixer: *mut *mut ct_mixer) -> c_int;
    pub fn ct_mixer_destroy(mixer: *mut ct_mixer) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
