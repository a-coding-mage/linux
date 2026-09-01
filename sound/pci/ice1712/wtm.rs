// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 * Lowlevel functions for Ego Sys Waveterminal 192M
 *
 * Copyright (c) 2006 Guedez Clement <klem.dev@gmail.com>
 * Some functions are taken from the Prodigy192 driver source
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_uchar, c_ushort, c_void};

// Dependencies from linux/delay.h, sound/core.h, sound/tlv.h, ice1712.h,
// envy24ht.h, wtm.h, and stac946x.h are expected to be provided externally.

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
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
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
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
    pub private_value: c_ulong,
    pub tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    pub set_pro_rate: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub card: *mut snd_card,
    pub spec: *mut c_void,
    pub num_total_dacs: c_uint,
    pub num_total_adcs: c_uint,
    pub force_rdma1: c_uint,
    pub gpio: snd_ice1712_gpio,
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub eeprom_size: c_uint,
    pub eeprom_data: *const c_uchar,
}

#[repr(C)]
struct wtm_spec {
    /* rate change needs atomic mute/unmute of all dacs*/
    mute_mutex: mutex,
}

unsafe extern "C" {
    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, addr: c_uchar, reg: c_int, val: c_uchar);
    fn snd_vt1724_read_i2c(ice: *mut snd_ice1712, addr: c_uchar, reg: c_int) -> c_uchar;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ice1712;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn mutex_init(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn udelay(usecs: c_uint);
}

unsafe extern "C" {
    static STAC9460_I2C_ADDR: c_uchar;
    static STAC9460_2_I2C_ADDR: c_uchar;
    static STAC946X_MASTER_VOLUME: c_int;
    static STAC946X_LF_VOLUME: c_int;
    static STAC946X_MIC_L_VOLUME: c_int;
    static STAC946X_GENERAL_PURPOSE: c_int;
    static STAC946X_MASTER_CLOCKING: c_int;
    static STAC946X_RESET: c_ushort;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint;
    static ICE_EEP2_SYSCONF: usize;
    static ICE_EEP2_ACLINK: usize;
    static ICE_EEP2_I2S: usize;
    static ICE_EEP2_SPDIF: usize;
    static ICE_EEP2_GPIO_DIR: usize;
    static ICE_EEP2_GPIO_DIR1: usize;
    static ICE_EEP2_GPIO_DIR2: usize;
    static ICE_EEP2_GPIO_MASK: usize;
    static ICE_EEP2_GPIO_MASK1: usize;
    static ICE_EEP2_GPIO_MASK2: usize;
    static ICE_EEP2_GPIO_STATE: usize;
    static ICE_EEP2_GPIO_STATE1: usize;
    static ICE_EEP2_GPIO_STATE2: usize;
    static VT1724_SUBDEVICE_WTM: c_uint;
}

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

struct MutexGuard {
    mutex: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(mutex: *mut mutex) -> Self {
        unsafe { mutex_lock(mutex) };
        Self { mutex }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.mutex) };
    }
}

const fn declare_tlv_db_scale(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4] {
    [2, 2, min as c_uint, ((step as c_uint) & 0xffff) | ((mute as c_uint) << 16)]
}

/*
 * 2*ADC 6*DAC no1 ringbuffer r/w on i2c bus
 */
#[inline]
unsafe fn stac9460_put(ice: *mut snd_ice1712, reg: c_int, val: c_uchar) {
    unsafe { snd_vt1724_write_i2c(ice, STAC9460_I2C_ADDR, reg, val) };
}

#[inline]
unsafe fn stac9460_get(ice: *mut snd_ice1712, reg: c_int) -> c_uchar {
    unsafe { snd_vt1724_read_i2c(ice, STAC9460_I2C_ADDR, reg) }
}

/*
 * 2*ADC 2*DAC no2 ringbuffer r/w on i2c bus
 */
#[inline]
unsafe fn stac9460_2_put(ice: *mut snd_ice1712, reg: c_int, val: c_uchar) {
    unsafe { snd_vt1724_write_i2c(ice, STAC9460_2_I2C_ADDR, reg, val) };
}

#[inline]
unsafe fn stac9460_2_get(ice: *mut snd_ice1712, reg: c_int) -> c_uchar {
    unsafe { snd_vt1724_read_i2c(ice, STAC9460_2_I2C_ADDR, reg) }
}

/*
 * DAC mute control
 */
unsafe fn stac9460_dac_mute_all(
    ice: *mut snd_ice1712,
    mute: c_uchar,
    change_mask: *mut c_ushort,
) {
    let mut new: c_uchar;
    let mut old: c_uchar;
    let mut id: c_int;
    let mut idx: c_int;
    let mut change: c_int;

    /*stac9460 1*/
    id = 0;
    while id < 7 {
        if unsafe { *change_mask } & ((0x01 as c_ushort) << id) != 0 {
            if id == 0 {
                idx = unsafe { STAC946X_MASTER_VOLUME };
            } else {
                idx = unsafe { STAC946X_LF_VOLUME } - 1 + id;
            }
            old = unsafe { stac9460_get(ice, idx) };
            new = (((!mute as c_int) << 7) & 0x80 | ((old as c_int) & !0x80)) as c_uchar;
            change = (new != old) as c_int;
            if change != 0 {
                unsafe { stac9460_put(ice, idx, new) };
                unsafe { *change_mask = *change_mask | ((0x01 as c_ushort) << id) };
            } else {
                unsafe { *change_mask = *change_mask & !((0x01 as c_ushort) << id) };
            }
        }
        id += 1;
    }

    /*stac9460 2*/
    id = 0;
    while id < 3 {
        if unsafe { *change_mask } & ((0x01 as c_ushort) << (id + 7)) != 0 {
            if id == 0 {
                idx = unsafe { STAC946X_MASTER_VOLUME };
            } else {
                idx = unsafe { STAC946X_LF_VOLUME } - 1 + id;
            }
            old = unsafe { stac9460_2_get(ice, idx) };
            new = (((!mute as c_int) << 7) & 0x80 | ((old as c_int) & !0x80)) as c_uchar;
            change = (new != old) as c_int;
            if change != 0 {
                unsafe { stac9460_2_put(ice, idx, new) };
                unsafe { *change_mask = *change_mask | ((0x01 as c_ushort) << id) };
            } else {
                unsafe { *change_mask = *change_mask & !((0x01 as c_ushort) << id) };
            }
        }
        id += 1;
    }
}

unsafe extern "C" fn stac9460_dac_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let spec = unsafe { (*ice).spec as *mut wtm_spec };
    let val: c_uchar;
    let idx: c_int;
    let id: c_int;

    let _guard = unsafe { MutexGuard::new(&mut (*spec).mute_mutex) };

    if unsafe { (*kcontrol).private_value } != 0 {
        idx = unsafe { STAC946X_MASTER_VOLUME };
        id = 0;
    } else {
        id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
        idx = id + unsafe { STAC946X_LF_VOLUME };
    }
    if id < 6 {
        val = unsafe { stac9460_get(ice, idx) };
    } else {
        val = unsafe { stac9460_2_get(ice, idx - 6) };
    }
    unsafe { (*ucontrol).value.integer.value[0] = (((!val as c_int) >> 7) & 0x1) as c_long };

    0
}

unsafe extern "C" fn stac9460_dac_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let new: c_uchar;
    let old: c_uchar;
    let id: c_int;
    let idx: c_int;
    let change: c_int;

    if unsafe { (*kcontrol).private_value } != 0 {
        idx = unsafe { STAC946X_MASTER_VOLUME };
        old = unsafe { stac9460_get(ice, idx) };
        new = (((!unsafe { (*ucontrol).value.integer.value[0] } << 7) & 0x80)
            | ((old as c_long) & !0x80)) as c_uchar;
        change = (new != old) as c_int;
        if change != 0 {
            unsafe { stac9460_put(ice, idx, new) };
            unsafe { stac9460_2_put(ice, idx, new) };
        }
    } else {
        id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
        idx = id + unsafe { STAC946X_LF_VOLUME };
        if id < 6 {
            old = unsafe { stac9460_get(ice, idx) };
        } else {
            old = unsafe { stac9460_2_get(ice, idx - 6) };
        }
        new = (((!unsafe { (*ucontrol).value.integer.value[0] } << 7) & 0x80)
            | ((old as c_long) & !0x80)) as c_uchar;
        change = (new != old) as c_int;
        if change != 0 {
            if id < 6 {
                unsafe { stac9460_put(ice, idx, new) };
            } else {
                unsafe { stac9460_2_put(ice, idx - 6, new) };
            }
        }
    }
    change
}

/*
 * DAC volume attenuation mixer control
 */
unsafe extern "C" fn stac9460_dac_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0; /* mute */
        (*uinfo).value.integer.max = 0x7f; /* 0dB */
    }
    0
}

unsafe extern "C" fn stac9460_dac_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let idx: c_int;
    let id: c_int;
    let vol: c_uchar;

    if unsafe { (*kcontrol).private_value } != 0 {
        idx = unsafe { STAC946X_MASTER_VOLUME };
        id = 0;
    } else {
        id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
        idx = id + unsafe { STAC946X_LF_VOLUME };
    }
    if id < 6 {
        vol = unsafe { stac9460_get(ice, idx) } & 0x7f;
    } else {
        vol = unsafe { stac9460_2_get(ice, idx - 6) } & 0x7f;
    }
    unsafe { (*ucontrol).value.integer.value[0] = (0x7f - vol as c_int) as c_long };
    0
}

unsafe extern "C" fn stac9460_dac_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let idx: c_int;
    let id: c_int;
    let tmp: c_uchar;
    let ovol: c_uchar;
    let nvol: c_uchar;
    let change: c_int;

    if unsafe { (*kcontrol).private_value } != 0 {
        idx = unsafe { STAC946X_MASTER_VOLUME };
        nvol = (unsafe { (*ucontrol).value.integer.value[0] } & 0x7f) as c_uchar;
        tmp = unsafe { stac9460_get(ice, idx) };
        ovol = (0x7f - (tmp & 0x7f)) as c_uchar;
        change = (ovol != nvol) as c_int;
        if change != 0 {
            unsafe { stac9460_put(ice, idx, ((0x7f - nvol as c_int) | ((tmp as c_int) & 0x80)) as c_uchar) };
            unsafe { stac9460_2_put(ice, idx, ((0x7f - nvol as c_int) | ((tmp as c_int) & 0x80)) as c_uchar) };
        }
    } else {
        id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
        idx = id + unsafe { STAC946X_LF_VOLUME };
        nvol = (unsafe { (*ucontrol).value.integer.value[0] } & 0x7f) as c_uchar;
        if id < 6 {
            tmp = unsafe { stac9460_get(ice, idx) };
        } else {
            tmp = unsafe { stac9460_2_get(ice, idx - 6) };
        }
        ovol = (0x7f - (tmp & 0x7f)) as c_uchar;
        change = (ovol != nvol) as c_int;
        if change != 0 {
            if id < 6 {
                unsafe { stac9460_put(ice, idx, ((0x7f - nvol as c_int) | ((tmp as c_int) & 0x80)) as c_uchar) };
            } else {
                unsafe { stac9460_2_put(ice, idx - 6, ((0x7f - nvol as c_int) | ((tmp as c_int) & 0x80)) as c_uchar) };
            }
        }
    }
    change
}

/*
 * ADC mute control
 */
unsafe extern "C" fn stac9460_adc_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val: c_uchar;
    let mut i: c_int;
    let id: c_int;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        i = 0;
        while i < 2 {
            val = unsafe { stac9460_get(ice, STAC946X_MIC_L_VOLUME + i) };
            unsafe { (*ucontrol).value.integer.value[i as usize] = (((!val as c_int) >> 7) & 0x1) as c_long };
            i += 1;
        }
    } else {
        i = 0;
        while i < 2 {
            val = unsafe { stac9460_2_get(ice, STAC946X_MIC_L_VOLUME + i) };
            unsafe { (*ucontrol).value.integer.value[i as usize] = (((!val as c_int) >> 7) & 0x1) as c_long };
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn stac9460_adc_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut new: c_uchar;
    let mut old: c_uchar;
    let mut i: c_int;
    let mut reg: c_int;
    let id: c_int;
    let mut change: c_int = 0;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            old = unsafe { stac9460_get(ice, reg) };
            new = (((!unsafe { (*ucontrol).value.integer.value[i as usize] } << 7) & 0x80)
                | ((old as c_long) & !0x80)) as c_uchar;
            change = (new != old) as c_int;
            if change != 0 {
                unsafe { stac9460_put(ice, reg, new) };
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            old = unsafe { stac9460_2_get(ice, reg) };
            new = (((!unsafe { (*ucontrol).value.integer.value[i as usize] } << 7) & 0x80)
                | ((old as c_long) & !0x80)) as c_uchar;
            change = (new != old) as c_int;
            if change != 0 {
                unsafe { stac9460_2_put(ice, reg, new) };
            }
            i += 1;
        }
    }
    change
}

/*
 *ADC gain mixer control
 */
unsafe extern "C" fn stac9460_adc_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0; /* 0dB */
        (*uinfo).value.integer.max = 0x0f; /* 22.5dB */
    }
    0
}

unsafe extern "C" fn stac9460_adc_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut i: c_int;
    let mut reg: c_int;
    let id: c_int;
    let mut vol: c_uchar;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            vol = unsafe { stac9460_get(ice, reg) } & 0x0f;
            unsafe { (*ucontrol).value.integer.value[i as usize] = (0x0f - vol as c_int) as c_long };
            i += 1;
        }
    } else {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            vol = unsafe { stac9460_2_get(ice, reg) } & 0x0f;
            unsafe { (*ucontrol).value.integer.value[i as usize] = (0x0f - vol as c_int) as c_long };
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn stac9460_adc_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut i: c_int;
    let mut reg: c_int;
    let id: c_int;
    let mut ovol: c_uchar;
    let mut nvol: c_uchar;
    let mut change: c_int = 0;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            nvol = (unsafe { (*ucontrol).value.integer.value[i as usize] } & 0x0f) as c_uchar;
            ovol = (0x0f_u8).wrapping_sub(unsafe { stac9460_get(ice, reg) });
            change = (((ovol & 0x0f) != nvol) as c_int);
            if change != 0 {
                unsafe { stac9460_put(ice, reg, ((0x0f - nvol as c_int) | ((ovol as c_int) & !0x0f)) as c_uchar) };
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < 2 {
            reg = unsafe { STAC946X_MIC_L_VOLUME } + i;
            nvol = (unsafe { (*ucontrol).value.integer.value[i as usize] } & 0x0f) as c_uchar;
            ovol = (0x0f_u8).wrapping_sub(unsafe { stac9460_2_get(ice, reg) });
            change = (((ovol & 0x0f) != nvol) as c_int);
            if change != 0 {
                unsafe { stac9460_2_put(ice, reg, ((0x0f - nvol as c_int) | ((ovol as c_int) & !0x0f)) as c_uchar) };
            }
            i += 1;
        }
    }
    change
}

/*
 * MIC / LINE switch fonction
 */
unsafe extern "C" fn stac9460_mic_sw_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT0: &[u8] = b"Line In\0";
    static TEXT1: &[u8] = b"Mic\0";
    let texts: [*const c_char; 2] = [TEXT0.as_ptr() as *const c_char, TEXT1.as_ptr() as *const c_char];

    unsafe { snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr()) }
}

unsafe extern "C" fn stac9460_mic_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uchar;
    let id: c_int;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        val = unsafe { stac9460_get(ice, STAC946X_GENERAL_PURPOSE) };
    } else {
        val = unsafe { stac9460_2_get(ice, STAC946X_GENERAL_PURPOSE) };
    }
    unsafe { (*ucontrol).value.enumerated.item[0] = ((val >> 7) & 0x1) as c_uint };
    0
}

unsafe extern "C" fn stac9460_mic_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let new: c_uchar;
    let old: c_uchar;
    let change: c_int;
    let id: c_int;

    id = unsafe { snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) };
    if id == 0 {
        old = unsafe { stac9460_get(ice, STAC946X_GENERAL_PURPOSE) };
    } else {
        old = unsafe { stac9460_2_get(ice, STAC946X_GENERAL_PURPOSE) };
    }
    new = (((unsafe { (*ucontrol).value.enumerated.item[0] } << 7) & 0x80) | ((old as c_uint) & !0x80)) as c_uchar;
    change = (new != old) as c_int;
    if change != 0 {
        if id == 0 {
            unsafe { stac9460_put(ice, STAC946X_GENERAL_PURPOSE, new) };
        } else {
            unsafe { stac9460_2_put(ice, STAC946X_GENERAL_PURPOSE, new) };
        }
    }
    change
}

/*
 * Handler for setting correct codec rate - called when rate change is detected
 */
unsafe extern "C" fn stac9460_set_rate_val(ice: *mut snd_ice1712, rate: c_uint) {
    let old: c_uchar;
    let new: c_uchar;
    let mut changed: c_ushort;
    let spec = unsafe { (*ice).spec as *mut wtm_spec };

    if rate == 0 {
        return; /* no hint - S/PDIF input is master, simply return */
    } else if rate <= 48000 {
        new = 0x08; /* 256x, base rate mode */
    } else if rate <= 96000 {
        new = 0x11; /* 256x, mid rate mode */
    } else {
        new = 0x12; /* 128x, high rate mode */
    }

    old = unsafe { stac9460_get(ice, STAC946X_MASTER_CLOCKING) };
    if old == new {
        return;
    }
    /* change detected, setting master clock, muting first */
    /* due to possible conflicts with mute controls - mutexing */
    let _guard = unsafe { MutexGuard::new(&mut (*spec).mute_mutex) };
    /* we have to remember current mute status for each DAC */
    changed = 0xffff;
    unsafe { stac9460_dac_mute_all(ice, 0, &mut changed) };
    /*printk(KERN_DEBUG "Rate change: %d, new MC: 0x%02x\n", rate, new);*/
    unsafe { stac9460_put(ice, STAC946X_MASTER_CLOCKING, new) };
    unsafe { stac9460_2_put(ice, STAC946X_MASTER_CLOCKING, new) };
    unsafe { udelay(10) };
    /* unmuting - only originally unmuted dacs -
     * i.e. those changed when muting */
    unsafe { stac9460_dac_mute_all(ice, 1, &mut changed) };
}

/*Limits value in dB for fader*/
static DB_SCALE_DAC: [c_uint; 4] = declare_tlv_db_scale(-19125, 75, 0);
static DB_SCALE_ADC: [c_uint; 4] = declare_tlv_db_scale(0, 150, 0);

unsafe fn stac9640_controls() -> [snd_kcontrol_new; 7] {
    [
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"Master Playback Switch\0".as_ptr() as *const c_char,
            index: 0,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            count: 0,
            info: Some(snd_ctl_boolean_mono_info),
            get: Some(stac9460_dac_mute_get),
            put: Some(stac9460_dac_mute_put),
            private_value: 1,
            tlv: snd_kcontrol_new_tlv { p: DB_SCALE_DAC.as_ptr() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"Master Playback Volume\0".as_ptr() as *const c_char,
            index: 0,
            access: 0,
            count: 0,
            info: Some(stac9460_dac_vol_info),
            get: Some(stac9460_dac_vol_get),
            put: Some(stac9460_dac_vol_put),
            private_value: 1,
            tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"MIC/Line Input Enum\0".as_ptr() as *const c_char,
            index: 0,
            access: 0,
            count: 2,
            info: Some(stac9460_mic_sw_info),
            get: Some(stac9460_mic_sw_get),
            put: Some(stac9460_mic_sw_put),
            private_value: 0,
            tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"DAC Switch\0".as_ptr() as *const c_char,
            index: 0,
            access: 0,
            count: 8,
            info: Some(snd_ctl_boolean_mono_info),
            get: Some(stac9460_dac_mute_get),
            put: Some(stac9460_dac_mute_put),
            private_value: 0,
            tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"DAC Volume\0".as_ptr() as *const c_char,
            index: 0,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            count: 8,
            info: Some(stac9460_dac_vol_info),
            get: Some(stac9460_dac_vol_get),
            put: Some(stac9460_dac_vol_put),
            private_value: 0,
            tlv: snd_kcontrol_new_tlv { p: DB_SCALE_DAC.as_ptr() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"ADC Switch\0".as_ptr() as *const c_char,
            index: 0,
            access: 0,
            count: 2,
            info: Some(snd_ctl_boolean_stereo_info),
            get: Some(stac9460_adc_mute_get),
            put: Some(stac9460_adc_mute_put),
            private_value: 0,
            tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        },
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            device: 0,
            subdevice: 0,
            name: b"ADC Volume\0".as_ptr() as *const c_char,
            index: 0,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            count: 2,
            info: Some(stac9460_adc_vol_info),
            get: Some(stac9460_adc_vol_get),
            put: Some(stac9460_adc_vol_put),
            private_value: 0,
            tlv: snd_kcontrol_new_tlv { p: DB_SCALE_ADC.as_ptr() },
        },
    ]
}

/*INIT*/
unsafe extern "C" fn wtm_add_controls(ice: *mut snd_ice1712) -> c_int {
    let controls = unsafe { stac9640_controls() };
    let mut i: c_uint;
    let mut err: c_int;

    i = 0;
    while (i as usize) < controls.len() {
        err = unsafe {
            snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&controls[i as usize], ice as *mut c_void),
            )
        };
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn wtm_init(ice: *mut snd_ice1712) -> c_int {
    let stac_inits_wtm: [c_ushort; 5] = [
        unsafe { STAC946X_RESET },
        0,
        unsafe { STAC946X_MASTER_CLOCKING as c_ushort },
        0x11,
        (-1_i16) as c_ushort,
    ];
    let mut p: *const c_ushort;
    let spec: *mut wtm_spec;

    /*WTM 192M*/
    unsafe {
        (*ice).num_total_dacs = 8;
        (*ice).num_total_adcs = 4;
        (*ice).force_rdma1 = 1;
    }

    /*init mutex for dac mute conflict*/
    spec = unsafe { kzalloc(core::mem::size_of::<wtm_spec>(), GFP_KERNEL) as *mut wtm_spec };
    if spec.is_null() {
        return -ENOMEM;
    }
    unsafe {
        (*ice).spec = spec as *mut c_void;
        mutex_init(&mut (*spec).mute_mutex);
    }

    /*initialize codec*/
    p = stac_inits_wtm.as_ptr();
    while unsafe { *p } != ((-1_i16) as c_ushort) {
        unsafe {
            stac9460_put(ice, *p as c_int, *p.add(1) as c_uchar);
            stac9460_2_put(ice, *p as c_int, *p.add(1) as c_uchar);
            p = p.add(2);
        }
    }
    unsafe { (*ice).gpio.set_pro_rate = Some(stac9460_set_rate_val) };
    0
}

unsafe fn wtm_eeprom() -> [c_uchar; 256] {
    let mut data = [0_u8; 256];
    data[ICE_EEP2_SYSCONF] = 0x67; /*SYSCONF: clock 192KHz, mpu401,
                                    * 4ADC, 8DAC */
    data[ICE_EEP2_ACLINK] = 0x80; /* ACLINK : I2S */
    data[ICE_EEP2_I2S] = 0xf8; /* I2S: vol; 96k, 24bit, 192k */
    data[ICE_EEP2_SPDIF] = 0xc1; /*SPDIF: out-en, spidf ext out*/
    data[ICE_EEP2_GPIO_DIR] = 0x9f;
    data[ICE_EEP2_GPIO_DIR1] = 0xff;
    data[ICE_EEP2_GPIO_DIR2] = 0x7f;
    data[ICE_EEP2_GPIO_MASK] = 0x9f;
    data[ICE_EEP2_GPIO_MASK1] = 0xff;
    data[ICE_EEP2_GPIO_MASK2] = 0x7f;
    data[ICE_EEP2_GPIO_STATE] = 0x16;
    data[ICE_EEP2_GPIO_STATE1] = 0x80;
    data[ICE_EEP2_GPIO_STATE2] = 0x00;
    data
}

/*entry point*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_vt1724_wtm_cards() -> [snd_ice1712_card_info; 2] {
    let eeprom = unsafe { wtm_eeprom() };
    [
        snd_ice1712_card_info {
            subvendor: unsafe { VT1724_SUBDEVICE_WTM },
            name: b"ESI Waveterminal 192M\0".as_ptr() as *const c_char,
            model: b"WT192M\0".as_ptr() as *const c_char,
            chip_init: Some(wtm_init),
            build_controls: Some(wtm_add_controls),
            eeprom_size: core::mem::size_of_val(&eeprom) as c_uint,
            eeprom_data: eeprom.as_ptr(),
        },
        snd_ice1712_card_info {
            subvendor: 0,
            name: core::ptr::null(),
            model: core::ptr::null(),
            chip_init: None,
            build_controls: None,
            eeprom_size: 0,
            eeprom_data: core::ptr::null(),
        },
    ]
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
