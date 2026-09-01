/* SPDX-License-Identifier: GPL-2.0-only
 *
 * HDA audio driver for Texas Instruments TAS2781 smart amp
 *
 * Copyright (C) 2025 Texas Instruments, Inc.
 */

/* C header dependency: <sound/asound.h> */

/* Flag of calibration registers address. */
pub const TASDEV_UEFI_CALI_REG_ADDR_FLG: u32 = 1u32 << 7;

pub const TASDEV_CALIB_N: u32 = 5;

/*
 * No standard control callbacks for SNDRV_CTL_ELEM_IFACE_CARD
 * Define two controls, one is Volume control callbacks, the other is
 * flag setting control callbacks.
 */

/* Volume control callbacks for tas2781 */
#[macro_export]
macro_rules! ACARD_SINGLE_RANGE_EXT_TLV {
    ($xname:expr, $xreg:expr, $xshift:expr, $xmin:expr, $xmax:expr, $xinvert:expr,
     $xhandler_get:expr, $xhandler_put:expr, $tlv_array:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_CARD,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: snd_kcontrol_new__bindgen_ty_1 { p: $tlv_array },
            info: snd_soc_info_volsw,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: &soc_mixer_control {
                reg: $xreg,
                rreg: $xreg,
                shift: $xshift,
                rshift: $xshift,
                min: $xmin,
                max: $xmax,
                invert: $xinvert,
            } as *const soc_mixer_control as ::core::ffi::c_ulong,
        }
    };
}

/* Flag control callbacks for tas2781 */
#[macro_export]
macro_rules! ACARD_SINGLE_BOOL_EXT {
    ($xname:expr, $xdata:expr, $xhandler_get:expr, $xhandler_put:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_CARD,
            name: $xname,
            info: snd_ctl_boolean_mono_info,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: $xdata,
        }
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum device_catlog_id {
    DELL = 0,
    HP,
    LENOVO,
    OTHERS,
}

#[repr(C)]
pub struct tas2781_hda {
    pub dev: *mut device,
    pub r#priv: *mut tasdevice_priv,
    pub dsp_prog_ctl: *mut snd_kcontrol,
    pub dsp_conf_ctl: *mut snd_kcontrol,
    pub prof_ctl: *mut snd_kcontrol,
    pub catlog_id: device_catlog_id,
    pub hda_priv: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    pub static tasdev_fct_efi_guid: [efi_guid_t; 0];

    pub fn tas2781_save_calibration(p: *mut tas2781_hda) -> ::core::ffi::c_int;
    pub fn tas2781_hda_remove(dev: *mut device, ops: *const component_ops);
    pub fn tasdevice_info_profile(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_info,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_info_programs(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_info,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_info_config(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_info,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_set_profile_id(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_get_profile_id(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_program_get(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_program_put(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_config_put(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn tasdevice_config_get(
        kctl: *mut snd_kcontrol,
        uctl: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
