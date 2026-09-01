/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021-2022 Intel Corporation
 *
 * Authors: Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 *          Cezary Rojewski <cezary.rojewski@intel.com>
 */

/* Depends on declarations from <sound/control.h> and <uapi/sound/asoc.h>. */

#[repr(C)]
pub struct avs_control_data {
    pub id: u32,
    pub values: [core::ffi::c_long; SND_SOC_TPLG_MAX_CHAN],
}

unsafe extern "C" {
    pub fn avs_control_volume_get(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> core::ffi::c_int;
    pub fn avs_control_volume_put(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> core::ffi::c_int;
    pub fn avs_control_volume_info(
        kctl: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> core::ffi::c_int;
    pub fn avs_control_mute_get(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> core::ffi::c_int;
    pub fn avs_control_mute_put(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> core::ffi::c_int;
    pub fn avs_control_mute_info(
        kctl: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
