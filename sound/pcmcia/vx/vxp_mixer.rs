// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VXpocket soundcards
 *
 * VX-pocket mixer
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from the original C includes:
 * <sound/core.h>, <sound/control.h>, <sound/tlv.h>, "vxpocket.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const MIC_LEVEL_MIN: c_uint = 0;
const MIC_LEVEL_MAX: c_uint = 8;

const EINVAL: c_int = 22;

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0004_0000;

const VX_TYPE_VXPOCKET: c_int = 1;
const VX_TYPE_VXP440: c_int = 2;

#[repr(C)]
pub struct vx_core {
    pub card: *mut snd_card,
    pub type_: c_int,
    pub mixer_mutex: mutex,
}

#[repr(C)]
pub struct snd_vxpocket {
    pub core: vx_core,
    pub mic_level: c_uint,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
    pub min: i64,
    pub max: i64,
    pub step: i64,
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
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub access: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut vx_core;
    fn to_vxpocket(chip: *mut vx_core) -> *mut snd_vxpocket;
    fn vx_set_mic_level(chip: *mut vx_core, level: c_uint);
    fn vx_set_mic_boost(chip: *mut vx_core, boost: c_int);
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe {
            mutex_lock(lock);
        }
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

/*
 * mic level control (for VXPocket)
 */
unsafe extern "C" fn vx_mic_level_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = MIC_LEVEL_MAX as i64;
    }
    0
}

unsafe extern "C" fn vx_mic_level_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let _chip = snd_kcontrol_chip(kcontrol);
        let chip = to_vxpocket(_chip);
        (*ucontrol).value.integer.value[0] = (*chip).mic_level as i64;
    }
    0
}

unsafe extern "C" fn vx_mic_level_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let _chip = snd_kcontrol_chip(kcontrol);
        let chip = to_vxpocket(_chip);
        let val: c_uint = (*ucontrol).value.integer.value[0] as c_uint;

        if val > MIC_LEVEL_MAX {
            return -EINVAL;
        }
        let _guard = MutexGuard::new(&mut (*_chip).mixer_mutex);
        if (*chip).mic_level != (*ucontrol).value.integer.value[0] as c_uint {
            vx_set_mic_level(_chip, (*ucontrol).value.integer.value[0] as c_uint);
            (*chip).mic_level = (*ucontrol).value.integer.value[0] as c_uint;
            return 1;
        }
    }
    0
}

static DB_SCALE_MIC: [c_uint; 4] = [SNDRV_CTL_ELEM_ACCESS_TLV_READ, 3, (-21i32) as c_uint, 0];

static VX_CONTROL_MIC_LEVEL_NAME: &[u8] = b"Mic Capture Volume\0";

static VX_CONTROL_MIC_LEVEL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: VX_CONTROL_MIC_LEVEL_NAME.as_ptr() as *const c_char,
    info: Some(vx_mic_level_info),
    get: Some(vx_mic_level_get),
    put: Some(vx_mic_level_put),
    tlv: snd_kcontrol_tlv {
        p: DB_SCALE_MIC.as_ptr(),
    },
};

/*
 * mic boost level control (for VXP440)
 */
const VX_MIC_BOOST_INFO: Option<
    unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int,
> = Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn vx_mic_boost_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let _chip = snd_kcontrol_chip(kcontrol);
        let chip = to_vxpocket(_chip);
        (*ucontrol).value.integer.value[0] = (*chip).mic_level as i64;
    }
    0
}

unsafe extern "C" fn vx_mic_boost_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let _chip = snd_kcontrol_chip(kcontrol);
        let chip = to_vxpocket(_chip);
        let val: c_int = ((*ucontrol).value.integer.value[0] != 0) as c_int;

        let _guard = MutexGuard::new(&mut (*_chip).mixer_mutex);
        if (*chip).mic_level != val as c_uint {
            vx_set_mic_boost(_chip, val);
            (*chip).mic_level = val as c_uint;
            return 1;
        }
    }
    0
}

static VX_CONTROL_MIC_BOOST_NAME: &[u8] = b"Mic Boost\0";

static VX_CONTROL_MIC_BOOST: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: 0,
    name: VX_CONTROL_MIC_BOOST_NAME.as_ptr() as *const c_char,
    info: VX_MIC_BOOST_INFO,
    get: Some(vx_mic_boost_get),
    put: Some(vx_mic_boost_put),
    tlv: snd_kcontrol_tlv {
        p: core::ptr::null(),
    },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vxp_add_mic_controls(_chip: *mut vx_core) -> c_int {
    unsafe {
        let chip = to_vxpocket(_chip);
        let mut err: c_int;

        /* mute input levels */
        (*chip).mic_level = 0;
        match (*_chip).type_ {
            VX_TYPE_VXPOCKET => {
                vx_set_mic_level(_chip, 0);
            }
            VX_TYPE_VXP440 => {
                vx_set_mic_boost(_chip, 0);
            }
            _ => {}
        }

        /* mic level */
        match (*_chip).type_ {
            VX_TYPE_VXPOCKET => {
                err = snd_ctl_add(
                    (*_chip).card,
                    snd_ctl_new1(
                        &VX_CONTROL_MIC_LEVEL,
                        chip as *mut c_void,
                    ),
                );
                if err < 0 {
                    return err;
                }
            }
            VX_TYPE_VXP440 => {
                err = snd_ctl_add(
                    (*_chip).card,
                    snd_ctl_new1(
                        &VX_CONTROL_MIC_BOOST,
                        chip as *mut c_void,
                    ),
                );
                if err < 0 {
                    return err;
                }
            }
            _ => {}
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
