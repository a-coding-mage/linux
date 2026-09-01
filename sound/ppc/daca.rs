// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PMac DACA lowlevel functions
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_ushort, c_void};

/* dependencies from linux/init.h, linux/i2c.h, linux/kmod.h, linux/slab.h,
 * sound/core.h, and "pmac.h"
 */

/* i2c address */
const DACA_I2C_ADDR: c_int = 0x4d;

/* registers */
const DACA_REG_SR: c_uchar = 0x01;
const DACA_REG_AVOL: c_uchar = 0x02;
const DACA_REG_GCFG: c_uchar = 0x03;

/* maximum volume value */
const DACA_VOL_MAX: c_uint = 0x38;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct pmac_keywest {
    pub client: *mut i2c_client,
    pub addr: c_int,
    pub init_client: Option<unsafe extern "C" fn(*mut pmac_keywest) -> c_int>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_card {
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_pmac {
    pub mixer_data: *mut c_void,
    pub mixer_free: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub card: *mut snd_card,
    /* CONFIG_PM: resume callback is present when power-management support is enabled. */
    pub resume: Option<unsafe extern "C" fn(*mut snd_pmac)>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_int,
    pub max: c_int,
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
    pub value: [c_int; 2],
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
struct pmac_daca {
    i2c: pmac_keywest,
    left_vol: c_int,
    right_vol: c_int,
    deemphasis: c_uint,
    amp_on: c_uint,
}

unsafe extern "C" {
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: c_uchar, value: c_uchar) -> c_int;
    fn i2c_smbus_write_block_data(
        client: *mut i2c_client,
        command: c_uchar,
        length: c_uchar,
        values: *const c_uchar,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pmac;
    fn snd_pmac_keywest_cleanup(i2c: *mut pmac_keywest);
    fn kfree(ptr: *mut c_void);
    fn request_module(name: *const c_char) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn snd_pmac_keywest_init(i2c: *mut pmac_keywest) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template_: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
}

/*
 * initialize / detect DACA
 */
unsafe extern "C" fn daca_init_client(i2c: *mut pmac_keywest) -> c_int {
    let mut wdata: c_ushort = 0x00;
    /* SR: no swap, 1bit delay, 32-48kHz */
    /* GCFG: power amp inverted, DAC on */
    if i2c_smbus_write_byte_data((*i2c).client, DACA_REG_SR, 0x08) < 0
        || i2c_smbus_write_byte_data((*i2c).client, DACA_REG_GCFG, 0x05) < 0
    {
        return -EINVAL;
    }
    i2c_smbus_write_block_data(
        (*i2c).client,
        DACA_REG_AVOL,
        2,
        (&mut wdata as *mut c_ushort).cast::<c_uchar>(),
    )
}

/*
 * update volume
 */
unsafe extern "C" fn daca_set_volume(mix: *mut pmac_daca) -> c_int {
    let mut data: [c_uchar; 2] = [0; 2];

    if (*mix).i2c.client.is_null() {
        return -ENODEV;
    }

    if (*mix).left_vol as c_uint > DACA_VOL_MAX {
        data[0] = DACA_VOL_MAX as c_uchar;
    } else {
        data[0] = (*mix).left_vol as c_uchar;
    }
    if (*mix).right_vol as c_uint > DACA_VOL_MAX {
        data[1] = DACA_VOL_MAX as c_uchar;
    } else {
        data[1] = (*mix).right_vol as c_uchar;
    }
    data[1] |= if (*mix).deemphasis != 0 { 0x40 } else { 0 };
    if i2c_smbus_write_block_data((*mix).i2c.client, DACA_REG_AVOL, 2, data.as_ptr()) < 0 {
        dev_err(
            &mut (*(*mix).i2c.client).dev,
            c"failed to set volume\n".as_ptr(),
        );
        return -EINVAL;
    }
    0
}

/* deemphasis switch */
unsafe extern "C" fn daca_info_deemphasis(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, uinfo)
}

unsafe extern "C" fn daca_get_deemphasis(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    (*ucontrol).value.integer.value[0] = if (*mix).deemphasis != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn daca_put_deemphasis(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    let change: c_int;

    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    change = ((*mix).deemphasis != (*ucontrol).value.integer.value[0] as c_uint) as c_int;
    if change != 0 {
        (*mix).deemphasis = ((*ucontrol).value.integer.value[0] != 0) as c_uint;
        daca_set_volume(mix);
    }
    change
}

/* output volume */
unsafe extern "C" fn daca_info_volume(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = DACA_VOL_MAX as c_int;
    0
}

unsafe extern "C" fn daca_get_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    (*ucontrol).value.integer.value[0] = (*mix).left_vol;
    (*ucontrol).value.integer.value[1] = (*mix).right_vol;
    0
}

unsafe extern "C" fn daca_put_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    let mut vol: [c_uint; 2] = [0; 2];
    let change: c_int;

    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    vol[0] = (*ucontrol).value.integer.value[0] as c_uint;
    vol[1] = (*ucontrol).value.integer.value[1] as c_uint;
    if vol[0] > DACA_VOL_MAX || vol[1] > DACA_VOL_MAX {
        return -EINVAL;
    }
    change = ((*mix).left_vol as c_uint != vol[0] || (*mix).right_vol as c_uint != vol[1]) as c_int;
    if change != 0 {
        (*mix).left_vol = vol[0] as c_int;
        (*mix).right_vol = vol[1] as c_int;
        daca_set_volume(mix);
    }
    change
}

/* amplifier switch */
unsafe extern "C" fn daca_info_amp(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    daca_info_deemphasis(kcontrol, uinfo)
}

unsafe extern "C" fn daca_get_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    (*ucontrol).value.integer.value[0] = if (*mix).amp_on != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn daca_put_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mix: *mut pmac_daca;
    let change: c_int;

    mix = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return -ENODEV;
    }
    change = ((*mix).amp_on != (*ucontrol).value.integer.value[0] as c_uint) as c_int;
    if change != 0 {
        (*mix).amp_on = ((*ucontrol).value.integer.value[0] != 0) as c_uint;
        i2c_smbus_write_byte_data(
            (*mix).i2c.client,
            DACA_REG_GCFG,
            if (*mix).amp_on != 0 { 0x05 } else { 0x04 },
        );
    }
    change
}

static DACA_MIXERS: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Deemphasis Switch".as_ptr(),
        info: Some(daca_info_deemphasis),
        get: Some(daca_get_deemphasis),
        put: Some(daca_put_deemphasis),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Master Playback Volume".as_ptr(),
        info: Some(daca_info_volume),
        get: Some(daca_get_volume),
        put: Some(daca_put_volume),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Power Amplifier Switch".as_ptr(),
        info: Some(daca_info_amp),
        get: Some(daca_get_amp),
        put: Some(daca_put_amp),
    },
];

/* CONFIG_PM */
unsafe extern "C" fn daca_resume(chip: *mut snd_pmac) {
    let mix: *mut pmac_daca = (*chip).mixer_data.cast::<pmac_daca>();
    i2c_smbus_write_byte_data((*mix).i2c.client, DACA_REG_SR, 0x08);
    i2c_smbus_write_byte_data(
        (*mix).i2c.client,
        DACA_REG_GCFG,
        if (*mix).amp_on != 0 { 0x05 } else { 0x04 },
    );
    daca_set_volume(mix);
}

unsafe extern "C" fn daca_cleanup(chip: *mut snd_pmac) {
    let mix: *mut pmac_daca = (*chip).mixer_data.cast::<pmac_daca>();
    if mix.is_null() {
        return;
    }
    snd_pmac_keywest_cleanup(&mut (*mix).i2c);
    kfree(mix.cast::<c_void>());
    (*chip).mixer_data = core::ptr::null_mut();
}

/* exported */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pmac_daca_init(chip: *mut snd_pmac) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let mix: *mut pmac_daca;

    request_module(c"i2c-powermac".as_ptr());

    mix = kzalloc(core::mem::size_of::<pmac_daca>(), 0).cast::<pmac_daca>();
    if mix.is_null() {
        return -ENOMEM;
    }
    (*chip).mixer_data = mix.cast::<c_void>();
    (*chip).mixer_free = Some(daca_cleanup);
    (*mix).amp_on = 1; /* default on */

    (*mix).i2c.addr = DACA_I2C_ADDR;
    (*mix).i2c.init_client = Some(daca_init_client);
    (*mix).i2c.name = c"DACA".as_ptr();
    err = snd_pmac_keywest_init(&mut (*mix).i2c);
    if err < 0 {
        return err;
    }

    /*
     * build mixers
     */
    strscpy((*(*chip).card).mixername, c"PowerMac DACA".as_ptr());

    i = 0;
    while (i as usize) < DACA_MIXERS.len() {
        err = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&DACA_MIXERS[i as usize], chip.cast::<c_void>()),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }

    /* CONFIG_PM */
    (*chip).resume = Some(daca_resume);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
