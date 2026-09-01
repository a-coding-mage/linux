// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA Driver for the PT2258 volume controller.
 *
 *      Copyright (c) 2006  Jochen Voss <voss@seehuhn.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// C include dependencies:
// <sound/core.h>, <sound/control.h>, <sound/tlv.h>, <sound/i2c.h>,
// <sound/pt2258.h>, <linux/module.h>

pub const PT2258_CMD_RESET: u8 = 0xc0;
pub const PT2258_CMD_UNMUTE: u8 = 0xf8;
pub const PT2258_CMD_MUTE: u8 = 0xf9;

pub const EIO: c_int = 5;
pub const EINVAL: c_int = 22;

pub const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
pub const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
pub const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
pub const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0004_0000;
pub const SNDRV_CTL_TLVT_DB_SCALE: u32 = 0x0000_0001;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_i2c_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_i2c_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pt2258 {
    pub card: *mut snd_card,
    pub i2c_bus: *mut snd_i2c_bus,
    pub i2c_dev: *mut snd_i2c_device,
    pub volume: [c_int; 6],
    pub mute: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_value_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub count: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_new_tlv,
    pub private_value: c_ulong,
}

unsafe extern "C" {
    fn snd_i2c_lock(bus: *mut snd_i2c_bus);
    fn snd_i2c_unlock(bus: *mut snd_i2c_bus);
    fn snd_i2c_sendbytes(dev: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
}

pub static pt2258_channel_code: [u8; 12] = [
    0x80, 0x90, /* channel 1: -10dB, -1dB */
    0x40, 0x50, /* channel 2: -10dB, -1dB */
    0x00, 0x10, /* channel 3: -10dB, -1dB */
    0x20, 0x30, /* channel 4: -10dB, -1dB */
    0x60, 0x70, /* channel 5: -10dB, -1dB */
    0xa0, 0xb0, /* channel 6: -10dB, -1dB */
];

#[no_mangle]
pub unsafe extern "C" fn snd_pt2258_reset(pt: *mut snd_pt2258) -> c_int {
    let mut bytes: [u8; 2] = [0; 2];
    let mut i: c_int;

    /* reset chip */
    bytes[0] = PT2258_CMD_RESET;
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 1) != 1 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 reset failed\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    /* mute all channels */
    (*pt).mute = 1;
    bytes[0] = PT2258_CMD_MUTE;
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 1) != 1 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 reset failed\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    /* set all channels to 0dB */
    i = 0;
    while i < 6 {
        (*pt).volume[i as usize] = 0;
        i += 1;
    }
    bytes[0] = 0xd0;
    bytes[1] = 0xe0;
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 2) != 2 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 reset failed\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    0
}

unsafe extern "C" fn pt2258_stereo_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 79;
    0
}

unsafe extern "C" fn pt2258_stereo_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let pt: *mut snd_pt2258 = snd_kcontrol_chip(kcontrol) as *mut snd_pt2258;
    let base: c_int = (*kcontrol).private_value as c_int;

    /* chip does not support register reads */
    (*ucontrol).value.integer.value[0] = (79 - (*pt).volume[base as usize]) as c_long;
    (*ucontrol).value.integer.value[1] = (79 - (*pt).volume[(base + 1) as usize]) as c_long;
    0
}

unsafe extern "C" fn pt2258_stereo_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let pt: *mut snd_pt2258 = snd_kcontrol_chip(kcontrol) as *mut snd_pt2258;
    let base: c_int = (*kcontrol).private_value as c_int;
    let mut bytes: [u8; 2] = [0; 2];
    let val0: c_int;
    let val1: c_int;

    val0 = (79 as c_long - (*ucontrol).value.integer.value[0]) as c_int;
    val1 = (79 as c_long - (*ucontrol).value.integer.value[1]) as c_int;
    if val0 < 0 || val0 > 79 || val1 < 0 || val1 > 79 {
        return -EINVAL;
    }
    if val0 == (*pt).volume[base as usize] && val1 == (*pt).volume[(base + 1) as usize] {
        return 0;
    }

    (*pt).volume[base as usize] = val0;
    bytes[0] = pt2258_channel_code[(2 * base) as usize] | (val0 / 10) as u8;
    bytes[1] = pt2258_channel_code[(2 * base + 1) as usize] | (val0 % 10) as u8;
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 2) != 2 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 access failed\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    (*pt).volume[(base + 1) as usize] = val1;
    bytes[0] = pt2258_channel_code[(2 * base + 2) as usize] | (val1 / 10) as u8;
    bytes[1] = pt2258_channel_code[(2 * base + 3) as usize] | (val1 % 10) as u8;
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 2) != 2 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 access failed\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    1
}

const pt2258_switch_info: unsafe extern "C" fn(
    *mut snd_kcontrol,
    *mut snd_ctl_elem_info,
) -> c_int = snd_ctl_boolean_mono_info;

unsafe extern "C" fn pt2258_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let pt: *mut snd_pt2258 = snd_kcontrol_chip(kcontrol) as *mut snd_pt2258;

    (*ucontrol).value.integer.value[0] = if (*pt).mute == 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn pt2258_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let pt: *mut snd_pt2258 = snd_kcontrol_chip(kcontrol) as *mut snd_pt2258;
    let mut bytes: [u8; 2] = [0; 2];
    let val: c_int;

    val = if (*ucontrol).value.integer.value[0] == 0 {
        1
    } else {
        0
    };
    if (*pt).mute == val {
        return 0;
    }

    (*pt).mute = val;
    bytes[0] = if val != 0 {
        PT2258_CMD_MUTE
    } else {
        PT2258_CMD_UNMUTE
    };
    snd_i2c_lock((*pt).i2c_bus);
    if snd_i2c_sendbytes((*pt).i2c_dev, bytes.as_mut_ptr(), 1) != 1 {
        snd_i2c_unlock((*pt).i2c_bus);
        dev_err((*(*pt).card).dev, c"PT2258 access failed 2\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*pt).i2c_bus);

    1
}

pub static pt2258_db_scale: [c_uint; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE,
    2 * core::mem::size_of::<c_uint>() as c_uint,
    (-7900i32) as c_uint,
    100,
];

#[no_mangle]
pub unsafe extern "C" fn snd_pt2258_build_controls(pt: *mut snd_pt2258) -> c_int {
    let mut knew: snd_kcontrol_new;
    let names: [*const c_char; 3] = [
        c"Mic Loopback Playback Volume".as_ptr(),
        c"Line Loopback Playback Volume".as_ptr(),
        c"CD Loopback Playback Volume".as_ptr(),
    ];
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < 3 {
        knew = core::mem::zeroed();
        knew.name = names[i as usize];
        knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER as c_uint;
        knew.count = 1;
        knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ;
        knew.private_value = (2 * i) as c_ulong;
        knew.info = Some(pt2258_stereo_volume_info);
        knew.get = Some(pt2258_stereo_volume_get);
        knew.put = Some(pt2258_stereo_volume_put);
        knew.tlv.p = pt2258_db_scale.as_ptr();

        err = snd_ctl_add((*pt).card, snd_ctl_new1(&knew, pt as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    knew = core::mem::zeroed();
    knew.name = c"Loopback Switch".as_ptr();
    knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER as c_uint;
    knew.info = Some(pt2258_switch_info);
    knew.get = Some(pt2258_switch_get);
    knew.put = Some(pt2258_switch_put);
    knew.access = 0;
    err = snd_ctl_add((*pt).card, snd_ctl_new1(&knew, pt as *mut c_void));
    if err < 0 {
        return err;
    }

    0
}

// EXPORT_SYMBOL(snd_pt2258_reset);
// EXPORT_SYMBOL(snd_pt2258_build_controls);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
