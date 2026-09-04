// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Dependencies: <linux/usb.h>, <sound/core.h>, <sound/control.h>, <sound/soc-usb.h>
// Dependencies: ../usbaudio.h, ../card.h, ../helper.h, ../mixer.h, mixer_usb_offload.h

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::c_int;
use std::os::raw::{c_char, c_void};

macro_rules! PCM_IDX {
    ($n:expr) => {
        (($n) & 0xffff)
    };
}

macro_rules! CARD_IDX {
    ($n:expr) => {
        (($n) >> 16)
    };
}

// Opaque external types
#[repr(C)]
pub struct device {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_stream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_substream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_audio {
    pub pcm_list: *mut snd_usb_stream,
    pub card: *mut snd_card,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub typ: c_int,
    pub count: c_int,
    pub value: snd_ctl_elem_info_union,
}

#[repr(C)]
pub union snd_ctl_elem_info_union {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
    _pad: [u8; 0],
}

pub type snd_kcontrol_info_t = unsafe extern "C" fn(*const snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
pub type snd_kcontrol_get_t = unsafe extern "C" fn(*const snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub access: c_int,
    pub info: Option<snd_kcontrol_info_t>,
    pub get: Option<snd_kcontrol_get_t>,
    pub count: c_int,
    pub private_value: c_int,
    pub name: *const c_char,
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *const snd_kcontrol) -> *mut c_void;
    fn snd_soc_usb_update_offload_route(
        sysdev: *mut device,
        card_idx: c_int,
        pcm_idx: c_int,
        stream: c_int,
        route_type: c_int,
        value: *mut i64,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(
        kctl_new: *const snd_kcontrol_new,
        private_data: *mut c_void,
    ) -> *mut snd_kcontrol;
    fn snprintf(s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_USB_KCTL_CARD_ROUTE: c_int = 0;
const SND_SOC_USB_KCTL_PCM_ROUTE: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_CARD: c_int = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 0;
const SNDRV_CARDS: i64 = 32;

static mut snd_usb_offload_mapped_card_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    info: Some(snd_usb_offload_card_route_info),
    get: Some(snd_usb_offload_card_route_get),
    count: 0,
    private_value: 0,
    name: std::ptr::null(),
};

static mut snd_usb_offload_mapped_pcm_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    info: Some(snd_usb_offload_pcm_route_info),
    get: Some(snd_usb_offload_pcm_route_get),
    count: 0,
    private_value: 0,
    name: std::ptr::null(),
};

unsafe extern "C" fn snd_usb_offload_card_route_get(
    kcontrol: *const snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sysdev = snd_kcontrol_chip(kcontrol) as *mut device;
    let mut ret: c_int;

    ret = snd_soc_usb_update_offload_route(
        sysdev,
        CARD_IDX!((*kcontrol).private_value as u32) as c_int,
        PCM_IDX!((*kcontrol).private_value as u32) as c_int,
        SNDRV_PCM_STREAM_PLAYBACK,
        SND_SOC_USB_KCTL_CARD_ROUTE,
        &mut (*ucontrol).value.integer.value[0],
    );
    if ret < 0 {
        (*ucontrol).value.integer.value[0] = -1;
        (*ucontrol).value.integer.value[1] = -1;
    }

    0
}

unsafe extern "C" fn snd_usb_offload_card_route_info(
    _kcontrol: *const snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).typ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = -1;
    (*uinfo).value.integer.max = SNDRV_CARDS;

    0
}

unsafe extern "C" fn snd_usb_offload_pcm_route_get(
    kcontrol: *const snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sysdev = snd_kcontrol_chip(kcontrol) as *mut device;
    let mut ret: c_int;

    ret = snd_soc_usb_update_offload_route(
        sysdev,
        CARD_IDX!((*kcontrol).private_value as u32) as c_int,
        PCM_IDX!((*kcontrol).private_value as u32) as c_int,
        SNDRV_PCM_STREAM_PLAYBACK,
        SND_SOC_USB_KCTL_PCM_ROUTE,
        &mut (*ucontrol).value.integer.value[0],
    );
    if ret < 0 {
        (*ucontrol).value.integer.value[0] = -1;
        (*ucontrol).value.integer.value[1] = -1;
    }

    0
}

unsafe extern "C" fn snd_usb_offload_pcm_route_info(
    _kcontrol: *const snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).typ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = -1;
    (*uinfo).value.integer.max = 0xff;

    0
}

/// snd_usb_offload_create_ctl() - Add USB offload bounded mixer
/// @chip: USB SND chip device
/// @bedev: Reference to USB backend DAI device
///
/// Creates a sound control for a USB audio device, so that applications can
/// query for if there is an available USB audio offload path, and which
/// card is managing it.
pub unsafe extern "C" fn snd_usb_offload_create_ctl(
    chip: *mut snd_usb_audio,
    bedev: *mut device,
) -> c_int {
    let mut chip_kctl: *mut snd_kcontrol_new;
    let mut subs: *mut snd_usb_substream;
    let mut as_: *mut snd_usb_stream;
    let mut ctl_name: [c_char; 48] = [0; 48];
    let mut ret: c_int = 0;

    let mut current = (*chip).pcm_list;
    while !current.is_null() {
        as_ = current;
        subs = &mut (*as_).substream[SNDRV_PCM_STREAM_PLAYBACK as usize] as *mut snd_usb_substream;

        if (*subs).ep_num == 0 || (*as_).pcm_index > 0xff {
            current = (*current).list;
            continue;
        }

        chip_kctl = &mut snd_usb_offload_mapped_card_ctl;
        (*chip_kctl).count = 1;
        (*chip_kctl).private_value = (*as_).pcm_index | ((*(*chip).card).number << 16);
        snprintf(
            &mut ctl_name[0],
            core::mem::size_of_val(&ctl_name),
            b"USB Offload Playback Card Route PCM#%d\0".as_ptr() as *const c_char,
            (*as_).pcm_index,
        );
        (*chip_kctl).name = &ctl_name[0];
        ret = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(chip_kctl, bedev as *mut c_void),
        );
        if ret < 0 {
            break;
        }

        chip_kctl = &mut snd_usb_offload_mapped_pcm_ctl;
        (*chip_kctl).count = 1;
        (*chip_kctl).private_value = (*as_).pcm_index | ((*(*chip).card).number << 16);
        snprintf(
            &mut ctl_name[0],
            core::mem::size_of_val(&ctl_name),
            b"USB Offload Playback PCM Route PCM#%d\0".as_ptr() as *const c_char,
            (*as_).pcm_index,
        );
        (*chip_kctl).name = &ctl_name[0];
        ret = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(chip_kctl, bedev as *mut c_void),
        );
        if ret < 0 {
            break;
        }

        current = (*current).list;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
