/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for AK4524 / AK4528 / AK4529 / AK4355 / AK4381
 *   AD and DA converters
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

/* AK4XXX_MAX_CHIPS defaults to 4 when not supplied by the build. */
pub const AK4XXX_MAX_CHIPS: usize = 4;

pub const AK4XXX_IMAGE_SIZE: usize = AK4XXX_MAX_CHIPS * 16; /* 64 bytes */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ak4xxx_ops {
    pub lock: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, chip: i32)>,
    pub unlock: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, chip: i32)>,
    pub write: Option<
        unsafe extern "C" fn(
            ak: *mut snd_akm4xxx,
            chip: i32,
            reg: u8,
            val: u8,
        ),
    >,
    pub set_rate_val: Option<unsafe extern "C" fn(ak: *mut snd_akm4xxx, rate: u32)>,
}

/* DAC label and channels */
#[repr(C)]
pub struct snd_akm4xxx_dac_channel {
    pub name: *mut c_char, /* mixer volume name */
    pub num_channels: u32,
    pub switch_name: *mut c_char, /* mixer switch */
}

/* ADC labels and channels */
#[repr(C)]
pub struct snd_akm4xxx_adc_channel {
    pub name: *mut c_char, /* capture gain volume label */
    pub switch_name: *mut c_char, /* capture switch */
    pub num_channels: u32,
    pub selector_name: *mut c_char, /* capture source select label */
    pub input_names: *const *const c_char, /* capture source names (NULL terminated) */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_akm4xxx_type {
    SND_AK4524,
    SND_AK4528,
    SND_AK4529,
    SND_AK4355,
    SND_AK4358,
    SND_AK4381,
    SND_AK5365,
    SND_AK4620,
}

#[repr(C)]
pub struct snd_akm4xxx {
    pub card: *mut snd_card,
    pub num_adcs: u32, /* AK4524 or AK4528 ADCs */
    pub num_dacs: u32, /* AK4524 or AK4528 DACs */
    pub images: [u8; AK4XXX_IMAGE_SIZE], /* saved register image */
    pub volumes: [u8; AK4XXX_IMAGE_SIZE], /* saved volume values */
    pub private_value: [c_ulong; AK4XXX_MAX_CHIPS], /* helper for driver */
    pub private_data: [*mut c_void; AK4XXX_MAX_CHIPS], /* helper for driver */
    /* template should fill the following fields */
    pub idx_offset: u32, /* control index offset */
    pub type_: snd_akm4xxx_type,
    /* (array) information of combined codecs */
    pub dac_info: *const snd_akm4xxx_dac_channel,
    pub adc_info: *const snd_akm4xxx_adc_channel,
    pub ops: snd_ak4xxx_ops,
    pub num_chips: u32,
    pub total_regs: u32,
    pub name: *const c_char,
}

pub type c_ulong = usize;

extern "C" {
    pub fn snd_akm4xxx_write(
        ak: *mut snd_akm4xxx,
        chip: i32,
        reg: u8,
        val: u8,
    );
    pub fn snd_akm4xxx_reset(ak: *mut snd_akm4xxx, state: i32);
    pub fn snd_akm4xxx_init(ak: *mut snd_akm4xxx);
    pub fn snd_akm4xxx_build_controls(ak: *mut snd_akm4xxx) -> i32;
}

#[macro_export]
macro_rules! snd_akm4xxx_get {
    ($ak:expr, $chip:expr, $reg:expr) => {
        (*$ak).images[($chip) * 16 + ($reg)]
    };
}

#[macro_export]
macro_rules! snd_akm4xxx_set {
    ($ak:expr, $chip:expr, $reg:expr, $val:expr) => {
        (*$ak).images[($chip) * 16 + ($reg)] = $val
    };
}

#[macro_export]
macro_rules! snd_akm4xxx_get_vol {
    ($ak:expr, $chip:expr, $reg:expr) => {
        (*$ak).volumes[($chip) * 16 + ($reg)]
    };
}

#[macro_export]
macro_rules! snd_akm4xxx_set_vol {
    ($ak:expr, $chip:expr, $reg:expr, $val:expr) => {
        (*$ak).volumes[($chip) * 16 + ($reg)] = $val
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
