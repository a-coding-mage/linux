// SPDX-License-Identifier: GPL-2.0
/*
 * PC-Speaker driver for Linux
 *
 * Mixer implementation.
 * Copyright (C) 2001-2008  Stas Sergeev
 */

// C dependencies:
// #include <sound/core.h>
// #include <sound/control.h>
// #include "pcsp.h"

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_pcsp {
    pub card: *mut snd_card,
    pub enable: c_int,
    pub max_treble: c_int,
    pub treble: c_int,
    pub pcspkr: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_int,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
    pub enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_enumerated {
    pub items: c_uint,
    pub item: c_uint,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

type c_uint = u32;

unsafe extern "C" {
    static SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int;
    static SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pcsp;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void)
        -> *mut snd_kcontrol;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn PCSP_CALC_RATE(treble: c_uint) -> c_ulong;

    // Used only when the C PCSP_DEBUG preprocessor branch is enabled.
    fn dev_dbg(dev: *mut c_void, format: *const c_char, ...);
    fn PCSP_RATE() -> c_long;
}

unsafe extern "C" fn pcsp_enable_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 1;
    }
    0
}

unsafe extern "C" fn pcsp_enable_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] = (*chip).enable as c_long;
    }
    0
}

unsafe extern "C" fn pcsp_enable_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        let mut changed: c_int = 0;
        let enab: c_int = (*ucontrol).value.integer.value[0] as c_int;
        if enab != (*chip).enable {
            (*chip).enable = enab;
            changed = 1;
        }
        changed
    }
}

unsafe extern "C" fn pcsp_treble_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
        (*uinfo).count = 1;
        (*uinfo).value.enumerated.items = ((*chip).max_treble + 1) as c_uint;
        if (*uinfo).value.enumerated.item > (*chip).max_treble as c_uint {
            (*uinfo).value.enumerated.item = (*chip).max_treble as c_uint;
        }
        sprintf(
            (*uinfo).value.enumerated.name,
            b"%lu\0".as_ptr() as *const c_char,
            PCSP_CALC_RATE((*uinfo).value.enumerated.item),
        );
    }
    0
}

unsafe extern "C" fn pcsp_treble_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.enumerated.item[0] = (*chip).treble as c_uint;
    }
    0
}

unsafe extern "C" fn pcsp_treble_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        let mut changed: c_int = 0;
        let treble: c_int = (*ucontrol).value.enumerated.item[0] as c_int;
        if treble != (*chip).treble {
            (*chip).treble = treble;
            // C preprocessor condition preserved: #if PCSP_DEBUG
            #[cfg(PCSP_DEBUG)]
            {
                dev_dbg(
                    (*(*chip).card).dev,
                    b"PCSP: rate set to %li\n\0".as_ptr() as *const c_char,
                    PCSP_RATE(),
                );
            }
            changed = 1;
        }
        changed
    }
}

unsafe extern "C" fn pcsp_pcspkr_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 1;
    }
    0
}

unsafe extern "C" fn pcsp_pcspkr_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] = (*chip).pcspkr as c_long;
    }
    0
}

unsafe extern "C" fn pcsp_pcspkr_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut snd_pcsp = snd_kcontrol_chip(kcontrol);
        let mut changed: c_int = 0;
        let spkr: c_int = (*ucontrol).value.integer.value[0] as c_int;
        if spkr != (*chip).pcspkr {
            (*chip).pcspkr = spkr;
            changed = 1;
        }
        changed
    }
}

macro_rules! PCSP_MIXER_CONTROL {
    ($ctl_type:ident, $ctl_name:expr, $info:ident, $get:ident, $put:ident) => {
        snd_kcontrol_new {
            iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
            name: $ctl_name.as_ptr() as *const c_char,
            info: Some($info),
            get: Some($get),
            put: Some($put),
        }
    };
}

static SND_PCSP_CONTROLS_PCM: [snd_kcontrol_new; 2] = [
    PCSP_MIXER_CONTROL!(
        enable,
        b"Master Playback Switch\0",
        pcsp_enable_info,
        pcsp_enable_get,
        pcsp_enable_put
    ),
    PCSP_MIXER_CONTROL!(
        treble,
        b"BaseFRQ Playback Volume\0",
        pcsp_treble_info,
        pcsp_treble_get,
        pcsp_treble_put
    ),
];

static SND_PCSP_CONTROLS_SPKR: [snd_kcontrol_new; 1] = [PCSP_MIXER_CONTROL!(
    pcspkr,
    b"Beep Playback Switch\0",
    pcsp_pcspkr_info,
    pcsp_pcspkr_get,
    pcsp_pcspkr_put
)];

unsafe extern "C" fn snd_pcsp_ctls_add(
    chip: *mut snd_pcsp,
    ctls: *const snd_kcontrol_new,
    num: c_int,
) -> c_int {
    unsafe {
        let mut i: c_int;
        let mut err: c_int;
        let card: *mut snd_card = (*chip).card;

        i = 0;
        while i < num {
            err = snd_ctl_add(
                card,
                snd_ctl_new1(ctls.offset(i as isize), chip as *mut c_void),
            );
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pcsp_new_mixer(chip: *mut snd_pcsp, nopcm: c_int) -> c_int {
    unsafe {
        let mut err: c_int;
        let card: *mut snd_card = (*chip).card;

        if nopcm == 0 {
            err = snd_pcsp_ctls_add(
                chip,
                SND_PCSP_CONTROLS_PCM.as_ptr(),
                SND_PCSP_CONTROLS_PCM.len() as c_int,
            );
            if err < 0 {
                return err;
            }
        }
        err = snd_pcsp_ctls_add(
            chip,
            SND_PCSP_CONTROLS_SPKR.as_ptr(),
            SND_PCSP_CONTROLS_SPKR.len() as c_int,
        );
        if err < 0 {
            return err;
        }

        strscpy((*card).mixername, b"PC-Speaker\0".as_ptr() as *const c_char);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
