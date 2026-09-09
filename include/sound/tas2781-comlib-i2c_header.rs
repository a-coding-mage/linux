/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments TAS2563/TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2563/TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2563/TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//

use core::ffi::c_void;

// Types and symbols are supplied by the corresponding driver dependencies.
#[repr(C)]
pub struct tasdevice_priv {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}
#[repr(C)]
pub struct soc_mixer_control {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn tasdevice_reset(tas_dev: *mut tasdevice_priv);
    pub fn tascodec_init(
        tas_priv: *mut tasdevice_priv,
        codec: *mut c_void,
        module: *mut module,
        cont: Option<unsafe extern "C" fn(fw: *const firmware, context: *mut c_void)>,
    ) -> i32;
    pub fn tasdevice_kzalloc(i2c: *mut i2c_client) -> *mut tasdevice_priv;
    pub fn tasdevice_init(tas_priv: *mut tasdevice_priv) -> i32;
    pub fn tasdev_chn_switch(tas_priv: *mut tasdevice_priv, chn: u16) -> i32;
    pub fn tasdevice_dev_update_bits(
        tasdevice: *mut tasdevice_priv,
        chn: u16,
        reg: u32,
        mask: u32,
        value: u32,
    ) -> i32;
    pub fn tasdevice_amp_putvol(
        tas_priv: *mut tasdevice_priv,
        ucontrol: *mut snd_ctl_elem_value,
        mc: *mut soc_mixer_control,
    ) -> i32;
    pub fn tasdevice_amp_getvol(
        tas_priv: *mut tasdevice_priv,
        ucontrol: *mut snd_ctl_elem_value,
        mc: *mut soc_mixer_control,
    ) -> i32;
    pub fn tasdevice_digital_getvol(
        tas_priv: *mut tasdevice_priv,
        ucontrol: *mut snd_ctl_elem_value,
        mc: *mut soc_mixer_control,
    ) -> i32;
    pub fn tasdevice_digital_putvol(
        tas_priv: *mut tasdevice_priv,
        ucontrol: *mut snd_ctl_elem_value,
        mc: *mut soc_mixer_control,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
