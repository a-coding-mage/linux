// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OPL4 mixer functions
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 */

// Rust translation of includes:
// #include "opl4_local.h"
// #include <sound/control.h>

use core::ffi::{c_char, c_int, c_long, c_ulong};

type u8 = u8;

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_opl4;
    fn snd_opl4_read(opl4: *mut snd_opl4, reg: u8) -> u8;
    fn snd_opl4_write(opl4: *mut snd_opl4, reg: u8, value: u8);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut snd_opl4) -> *mut snd_kcontrol;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

extern "C" {
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static OPL4_REG_MIX_CONTROL_FM: c_ulong;
    static OPL4_REG_MIX_CONTROL_PCM: c_ulong;
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_card {
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_opl4 {
    pub card: *mut snd_card,
    pub reg_lock: spinlock_t,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

type c_uint = u32;

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
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
    pub value: [c_long; 2],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

unsafe extern "C" fn snd_opl4_ctl_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 7;
    }
    0
}

unsafe extern "C" fn snd_opl4_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let opl4: *mut snd_opl4 = snd_kcontrol_chip(kcontrol);
        let reg: u8 = (*kcontrol).private_value as u8;
        let value: u8;

        // C source uses guard(spinlock_irqsave)(&opl4->reg_lock) for scoped locking.
        value = snd_opl4_read(opl4, reg);
        (*ucontrol).value.integer.value[0] = (7 - (value & 7)) as c_long;
        (*ucontrol).value.integer.value[1] = (7 - ((value >> 3) & 7)) as c_long;
    }
    0
}

unsafe extern "C" fn snd_opl4_ctl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let opl4: *mut snd_opl4 = snd_kcontrol_chip(kcontrol);
        let reg: u8 = (*kcontrol).private_value as u8;
        let value: u8;
        let old_value: u8;

        value = ((7 - ((*ucontrol).value.integer.value[0] & 7)) as u8)
            | (((7 - ((*ucontrol).value.integer.value[1] & 7)) as u8) << 3);
        // C source uses guard(spinlock_irqsave)(&opl4->reg_lock) for scoped locking.
        old_value = snd_opl4_read(opl4, reg);
        snd_opl4_write(opl4, reg, value);
        return (value != old_value) as c_int;
    }
}

static snd_opl4_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
        name: b"FM Playback Volume\0".as_ptr() as *const c_char,
        info: Some(snd_opl4_ctl_info),
        get: Some(snd_opl4_ctl_get),
        put: Some(snd_opl4_ctl_put),
        private_value: unsafe { OPL4_REG_MIX_CONTROL_FM },
    },
    snd_kcontrol_new {
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
        name: b"Wavetable Playback Volume\0".as_ptr() as *const c_char,
        info: Some(snd_opl4_ctl_info),
        get: Some(snd_opl4_ctl_get),
        put: Some(snd_opl4_ctl_put),
        private_value: unsafe { OPL4_REG_MIX_CONTROL_PCM },
    },
];

#[no_mangle]
pub unsafe extern "C" fn snd_opl4_create_mixer(opl4: *mut snd_opl4) -> c_int {
    unsafe {
        let card: *mut snd_card = (*opl4).card;
        let mut i: c_int;
        let mut err: c_int;

        strcat((*card).mixername, b",OPL4\0".as_ptr() as *const c_char);

        i = 0;
        while i < 2 {
            err = snd_ctl_add(
                card,
                snd_ctl_new1(&snd_opl4_controls[i as usize], opl4),
            );
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
