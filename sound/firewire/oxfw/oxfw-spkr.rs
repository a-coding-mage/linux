// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw-spkr.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependency: #include "oxfw.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_ = bool;
type s16 = i16;
type u8 = u8;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const EINVAL: c_int = 22;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct snd_card {
    pub card_dev: device,
}

#[repr(C)]
pub struct snd_oxfw_card {
    pub card_dev: device,
}

#[repr(C)]
pub struct snd_oxfw {
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
    pub spec: *mut c_void,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
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
struct fw_spkr {
    mute: bool_,
    volume: [s16; 6],
    volume_min: s16,
    volume_max: s16,

    mixer_channels: c_uint,
    mute_fb_id: u8,
    volume_fb_id: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum control_action {
    CTL_READ,
    CTL_WRITE,
}

#[repr(C)]
#[derive(Clone, Copy)]
enum control_attribute {
    CTL_MIN = 0x02,
    CTL_MAX = 0x03,
    CTL_CURRENT = 0x10,
}

unsafe extern "C" {
    fn kmalloc(size: usize, flags: c_uint) -> *mut u8;
    fn kfree(ptr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn fcp_avc_transaction(
        unit: *mut fw_unit,
        request: *mut u8,
        request_len: c_uint,
        response: *mut u8,
        response_len: c_uint,
        timeout: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_ctl_boolean_mono_info(
        control: *mut snd_kcontrol,
        info: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_new1(template_: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, control: *mut snd_kcontrol) -> c_int;
}

unsafe extern "C" fn avc_audio_feature_mute(
    unit: *mut fw_unit,
    fb_id: u8,
    value: *mut bool_,
    action: control_action,
) -> c_int {
    let buf: *mut u8;
    let response_ok: u8;
    let mut err: c_int;

    buf = kmalloc(11, GFP_KERNEL);
    if buf.is_null() {
        return -ENOMEM;
    }

    if action == control_action::CTL_READ {
        *buf.add(0) = 0x01; /* AV/C, STATUS */
        response_ok = 0x0c; /*       STABLE */
    } else {
        *buf.add(0) = 0x00; /* AV/C, CONTROL */
        response_ok = 0x09; /*       ACCEPTED */
    }
    *buf.add(1) = 0x08; /* audio unit 0 */
    *buf.add(2) = 0xb8; /* FUNCTION BLOCK */
    *buf.add(3) = 0x81; /* function block type: feature */
    *buf.add(4) = fb_id; /* function block ID */
    *buf.add(5) = 0x10; /* control attribute: current */
    *buf.add(6) = 0x02; /* selector length */
    *buf.add(7) = 0x00; /* audio channel number */
    *buf.add(8) = 0x01; /* control selector: mute */
    *buf.add(9) = 0x01; /* control data length */
    if action == control_action::CTL_READ {
        *buf.add(10) = 0xff;
    } else {
        *buf.add(10) = if *value { 0x70 } else { 0x60 };
    }

    err = fcp_avc_transaction(unit, buf, 11, buf, 11, 0x3fe);
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }
    if err < 11 {
        dev_err(&mut (*unit).device, c"short FCP response\n".as_ptr());
        err = -EIO;
        kfree(buf as *mut c_void);
        return err;
    }
    if *buf.add(0) != response_ok {
        dev_err(&mut (*unit).device, c"mute command failed\n".as_ptr());
        err = -EIO;
        kfree(buf as *mut c_void);
        return err;
    }
    if action == control_action::CTL_READ {
        *value = *buf.add(10) == 0x70;
    }

    err = 0;

    kfree(buf as *mut c_void);

    err
}

unsafe extern "C" fn avc_audio_feature_volume(
    unit: *mut fw_unit,
    fb_id: u8,
    value: *mut s16,
    channel: c_uint,
    attribute: control_attribute,
    action: control_action,
) -> c_int {
    let buf: *mut u8;
    let response_ok: u8;
    let mut err: c_int;

    buf = kmalloc(12, GFP_KERNEL);
    if buf.is_null() {
        return -ENOMEM;
    }

    if action == control_action::CTL_READ {
        *buf.add(0) = 0x01; /* AV/C, STATUS */
        response_ok = 0x0c; /*       STABLE */
    } else {
        *buf.add(0) = 0x00; /* AV/C, CONTROL */
        response_ok = 0x09; /*       ACCEPTED */
    }
    *buf.add(1) = 0x08; /* audio unit 0 */
    *buf.add(2) = 0xb8; /* FUNCTION BLOCK */
    *buf.add(3) = 0x81; /* function block type: feature */
    *buf.add(4) = fb_id; /* function block ID */
    *buf.add(5) = attribute as u8; /* control attribute */
    *buf.add(6) = 0x02; /* selector length */
    *buf.add(7) = channel as u8; /* audio channel number */
    *buf.add(8) = 0x02; /* control selector: volume */
    *buf.add(9) = 0x02; /* control data length */
    if action == control_action::CTL_READ {
        *buf.add(10) = 0xff;
        *buf.add(11) = 0xff;
    } else {
        *buf.add(10) = (*value >> 8) as u8;
        *buf.add(11) = *value as u8;
    }

    err = fcp_avc_transaction(unit, buf, 12, buf, 12, 0x3fe);
    if err < 0 {
        kfree(buf as *mut c_void);
        return err;
    }
    if err < 12 {
        dev_err(&mut (*unit).device, c"short FCP response\n".as_ptr());
        err = -EIO;
        kfree(buf as *mut c_void);
        return err;
    }
    if *buf.add(0) != response_ok {
        dev_err(&mut (*unit).device, c"volume command failed\n".as_ptr());
        err = -EIO;
        kfree(buf as *mut c_void);
        return err;
    }
    if action == control_action::CTL_READ {
        *value = (((*buf.add(10) as c_int) << 8) | *buf.add(11) as c_int) as s16;
    }

    err = 0;

    kfree(buf as *mut c_void);

    err
}

unsafe extern "C" fn spkr_mute_get(
    control: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let oxfw: *mut snd_oxfw = (*control).private_data as *mut snd_oxfw;
    let spkr: *mut fw_spkr = (*oxfw).spec as *mut fw_spkr;

    (*value).value.integer.value[0] = (!(*spkr).mute) as c_long;

    0
}

unsafe extern "C" fn spkr_mute_put(
    control: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let oxfw: *mut snd_oxfw = (*control).private_data as *mut snd_oxfw;
    let spkr: *mut fw_spkr = (*oxfw).spec as *mut fw_spkr;
    let mut mute: bool_;
    let err: c_int;

    mute = !((*value).value.integer.value[0] != 0);

    if mute == (*spkr).mute {
        return 0;
    }

    err = avc_audio_feature_mute((*oxfw).unit, (*spkr).mute_fb_id, &mut mute, control_action::CTL_WRITE);
    if err < 0 {
        return err;
    }
    (*spkr).mute = mute;

    1
}

unsafe extern "C" fn spkr_volume_info(
    control: *mut snd_kcontrol,
    info: *mut snd_ctl_elem_info,
) -> c_int {
    let oxfw: *mut snd_oxfw = (*control).private_data as *mut snd_oxfw;
    let spkr: *mut fw_spkr = (*oxfw).spec as *mut fw_spkr;

    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = (*spkr).mixer_channels;
    (*info).value.integer.min = (*spkr).volume_min as c_long;
    (*info).value.integer.max = (*spkr).volume_max as c_long;

    0
}

static channel_map: [u8; 6] = [0, 1, 4, 5, 2, 3];

unsafe extern "C" fn spkr_volume_get(
    control: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let oxfw: *mut snd_oxfw = (*control).private_data as *mut snd_oxfw;
    let spkr: *mut fw_spkr = (*oxfw).spec as *mut fw_spkr;
    let mut i: c_uint;

    i = 0;
    while i < (*spkr).mixer_channels {
        (*value).value.integer.value[channel_map[i as usize] as usize] = (*spkr).volume[i as usize] as c_long;
        i += 1;
    }

    0
}

unsafe extern "C" fn spkr_volume_put(
    control: *mut snd_kcontrol,
    value: *mut snd_ctl_elem_value,
) -> c_int {
    let oxfw: *mut snd_oxfw = (*control).private_data as *mut snd_oxfw;
    let spkr: *mut fw_spkr = (*oxfw).spec as *mut fw_spkr;
    let mut i: c_uint;
    let mut changed_channels: c_uint;
    let mut equal_values: bool_ = true;
    let mut volume: s16;
    let mut err: c_int;

    i = 0;
    while i < (*spkr).mixer_channels {
        if (*value).value.integer.value[i as usize] < (*spkr).volume_min as c_long
            || (*value).value.integer.value[i as usize] > (*spkr).volume_max as c_long
        {
            return -EINVAL;
        }
        if (*value).value.integer.value[i as usize] != (*value).value.integer.value[0] {
            equal_values = false;
        }
        i += 1;
    }

    changed_channels = 0;
    i = 0;
    while i < (*spkr).mixer_channels {
        if (*value).value.integer.value[channel_map[i as usize] as usize] != (*spkr).volume[i as usize] as c_long {
            changed_channels |= 1 << (i + 1);
        }
        i += 1;
    }

    if equal_values && changed_channels != 0 {
        changed_channels = 1 << 0;
    }

    i = 0;
    while i <= (*spkr).mixer_channels {
        volume = (*value).value.integer.value[channel_map[if i != 0 { (i - 1) as usize } else { 0 }] as usize] as s16;
        if changed_channels & (1 << i) != 0 {
            err = avc_audio_feature_volume(
                (*oxfw).unit,
                (*spkr).volume_fb_id,
                &mut volume,
                i,
                control_attribute::CTL_CURRENT,
                control_action::CTL_WRITE,
            );
            if err < 0 {
                return err;
            }
        }
        if i > 0 {
            (*spkr).volume[(i - 1) as usize] = volume;
        }
        i += 1;
    }

    (changed_channels != 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_oxfw_add_spkr(oxfw: *mut snd_oxfw, is_lacie: bool_) -> c_int {
    static controls: [snd_kcontrol_new; 2] = [
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: c"PCM Playback Switch".as_ptr(),
            info: Some(snd_ctl_boolean_mono_info),
            get: Some(spkr_mute_get),
            put: Some(spkr_mute_put),
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: c"PCM Playback Volume".as_ptr(),
            info: Some(spkr_volume_info),
            get: Some(spkr_volume_get),
            put: Some(spkr_volume_put),
        },
    ];
    let spkr: *mut fw_spkr;
    let mut i: c_uint;
    let first_ch: c_uint;
    let mut err: c_int;

    spkr = devm_kzalloc(
        &mut (*(*oxfw).card).card_dev,
        core::mem::size_of::<fw_spkr>(),
        GFP_KERNEL,
    ) as *mut fw_spkr;
    if spkr.is_null() {
        return -ENOMEM;
    }
    (*oxfw).spec = spkr as *mut c_void;

    if is_lacie {
        (*spkr).mixer_channels = 1;
        (*spkr).mute_fb_id = 0x01;
        (*spkr).volume_fb_id = 0x01;
    } else {
        (*spkr).mixer_channels = 6;
        (*spkr).mute_fb_id = 0x01;
        (*spkr).volume_fb_id = 0x02;
    }

    err = avc_audio_feature_volume(
        (*oxfw).unit,
        (*spkr).volume_fb_id,
        &mut (*spkr).volume_min,
        0,
        control_attribute::CTL_MIN,
        control_action::CTL_READ,
    );
    if err < 0 {
        return err;
    }
    err = avc_audio_feature_volume(
        (*oxfw).unit,
        (*spkr).volume_fb_id,
        &mut (*spkr).volume_max,
        0,
        control_attribute::CTL_MAX,
        control_action::CTL_READ,
    );
    if err < 0 {
        return err;
    }

    err = avc_audio_feature_mute(
        (*oxfw).unit,
        (*spkr).mute_fb_id,
        &mut (*spkr).mute,
        control_action::CTL_READ,
    );
    if err < 0 {
        return err;
    }

    first_ch = if (*spkr).mixer_channels == 1 { 0 } else { 1 };
    i = 0;
    while i < (*spkr).mixer_channels {
        err = avc_audio_feature_volume(
            (*oxfw).unit,
            (*spkr).volume_fb_id,
            &mut (*spkr).volume[i as usize],
            first_ch + i,
            control_attribute::CTL_CURRENT,
            control_action::CTL_READ,
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }

    i = 0;
    while i < controls.len() as c_uint {
        err = snd_ctl_add((*oxfw).card, snd_ctl_new1(&controls[i as usize], oxfw as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
