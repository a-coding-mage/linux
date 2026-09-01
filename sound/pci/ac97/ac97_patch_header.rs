/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com).
 */

pub const fn AC97_SINGLE_VALUE(reg: u32, shift: u32, mask: u32, invert: u32) -> u32 {
    reg | (shift << 8) | (shift << 12) | (mask << 16) | (invert << 24)
}

pub const fn AC97_PAGE_SINGLE_VALUE(
    reg: u32,
    shift: u32,
    mask: u32,
    invert: u32,
    page: u32,
) -> u32 {
    AC97_SINGLE_VALUE(reg, shift, mask, invert) | (1 << 25) | (page << 26)
}

macro_rules! AC97_SINGLE {
    ($xname:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: Some(snd_ac97_info_volsw),
            get: Some(snd_ac97_get_volsw),
            put: Some(snd_ac97_put_volsw),
            private_value: AC97_SINGLE_VALUE($reg, $shift, $mask, $invert) as _,
        }
    };
}

macro_rules! AC97_PAGE_SINGLE {
    ($xname:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr, $page:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: Some(snd_ac97_info_volsw),
            get: Some(snd_ac97_get_volsw),
            put: Some(snd_ac97_put_volsw),
            private_value: AC97_PAGE_SINGLE_VALUE($reg, $shift, $mask, $invert, $page) as _,
        }
    };
}

macro_rules! AC97_DOUBLE {
    ($xname:expr, $reg:expr, $shift_left:expr, $shift_right:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: Some(snd_ac97_info_volsw),
            get: Some(snd_ac97_get_volsw),
            put: Some(snd_ac97_put_volsw),
            private_value: (($reg)
                | (($shift_left) << 8)
                | (($shift_right) << 12)
                | (($mask) << 16)
                | (($invert) << 24)) as _,
        }
    };
}

/* enum control */
#[repr(C)]
pub struct ac97_enum {
    pub reg: ::std::os::raw::c_uchar,
    pub shift_l: ::std::os::raw::c_uchar,
    pub shift_r: ::std::os::raw::c_uchar,
    pub mask: ::std::os::raw::c_ushort,
    pub texts: *const *const ::std::os::raw::c_char,
}

macro_rules! AC97_ENUM_DOUBLE {
    ($xreg:expr, $xshift_l:expr, $xshift_r:expr, $xmask:expr, $xtexts:expr) => {
        ac97_enum {
            reg: $xreg,
            shift_l: $xshift_l,
            shift_r: $xshift_r,
            mask: $xmask,
            texts: $xtexts,
        }
    };
}

macro_rules! AC97_ENUM_SINGLE {
    ($xreg:expr, $xshift:expr, $xmask:expr, $xtexts:expr) => {
        AC97_ENUM_DOUBLE!($xreg, $xshift, $xshift, $xmask, $xtexts)
    };
}

macro_rules! AC97_ENUM {
    ($xname:expr, $xenum:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: Some(snd_ac97_info_enum_double),
            get: Some(snd_ac97_get_enum_double),
            put: Some(snd_ac97_put_enum_double),
            private_value: (&$xenum as *const _ as ::std::os::raw::c_ulong) as _,
        }
    };
}

/* ac97_codec.c */
unsafe extern "C" {
    /*
     * C declared these as incomplete static const arrays:
     *   static const struct snd_kcontrol_new snd_ac97_controls_3d[];
     *   static const struct snd_kcontrol_new snd_ac97_controls_spdif[];
     */
    pub static snd_ac97_controls_3d: [snd_kcontrol_new; 0];
    pub static snd_ac97_controls_spdif: [snd_kcontrol_new; 0];

    pub fn snd_ac97_cnew(
        _template: *const snd_kcontrol_new,
        ac97: *mut snd_ac97,
    ) -> *mut snd_kcontrol;
    pub fn snd_ac97_info_volsw(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_get_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_put_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_try_bit(
        ac97: *mut snd_ac97,
        reg: ::std::os::raw::c_int,
        bit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_remove_ctl(
        ac97: *mut snd_ac97,
        name: *const ::std::os::raw::c_char,
        suffix: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_rename_ctl(
        ac97: *mut snd_ac97,
        src: *const ::std::os::raw::c_char,
        dst: *const ::std::os::raw::c_char,
        suffix: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_swap_ctl(
        ac97: *mut snd_ac97,
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
        suffix: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_rename_vol_ctl(
        ac97: *mut snd_ac97,
        src: *const ::std::os::raw::c_char,
        dst: *const ::std::os::raw::c_char,
    );

    /*
     * Present in C only when CONFIG_PM is defined:
     *   static void snd_ac97_restore_status(struct snd_ac97 *ac97);
     *   static void snd_ac97_restore_iec958(struct snd_ac97 *ac97);
     */
    pub fn snd_ac97_restore_status(ac97: *mut snd_ac97);
    pub fn snd_ac97_restore_iec958(ac97: *mut snd_ac97);

    pub fn snd_ac97_info_enum_double(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_get_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::std::os::raw::c_int;
    pub fn snd_ac97_put_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
