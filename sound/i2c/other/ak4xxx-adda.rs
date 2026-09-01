// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for AK4524 / AK4528 / AK4529 / AK4355 / AK4358 / AK4381
 *   AD and DA converters
 *
 *	Copyright (c) 2000-2004 Jaroslav Kysela <perex@perex.cz>,
 *				Takashi Iwai <tiwai@suse.de>
 */

// Dependencies originally provided by:
// <linux/io.h>, <linux/delay.h>, <linux/interrupt.h>, <linux/init.h>,
// <linux/module.h>, <sound/core.h>, <sound/control.h>, <sound/tlv.h>,
// <sound/ak4xxx-adda.h>, <sound/info.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

extern "C" {
    fn snd_akm4xxx_set(ak: *mut snd_akm4xxx, chip: c_int, reg: u8, val: u8);
    fn snd_akm4xxx_get(ak: *mut snd_akm4xxx, chip: c_int, reg: c_int) -> u8;
    fn snd_akm4xxx_set_vol(ak: *mut snd_akm4xxx, chip: c_int, reg: c_int, val: c_int);
    fn snd_akm4xxx_get_vol(ak: *mut snd_akm4xxx, chip: c_int, reg: c_int) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_akm4xxx;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *mut snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_BUG();
    fn udelay(usecs: c_ulong);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> c_int;
}

extern "C" {
    static db_scale_vol_datt: [c_uint; 0];
    static db_scale_8bit: [c_uint; 0];
    static db_scale_7bit: [c_uint; 0];
    static db_scale_linear: [c_uint; 0];
}

const EINVAL: c_int = 22;

const SND_AK4524: c_int = 0;
const SND_AK4528: c_int = 1;
const SND_AK4529: c_int = 2;
const SND_AK4355: c_int = 3;
const SND_AK4358: c_int = 4;
const SND_AK4381: c_int = 5;
const SND_AK5365: c_int = 6;
const SND_AK4620: c_int = 7;

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0004_0000;

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut snd_akm4xxx,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

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

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
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
pub struct snd_akm4xxx_ops {
    pub lock: unsafe extern "C" fn(*mut snd_akm4xxx, c_int),
    pub unlock: unsafe extern "C" fn(*mut snd_akm4xxx, c_int),
    pub write: unsafe extern "C" fn(*mut snd_akm4xxx, c_int, u8, u8),
}

#[repr(C)]
pub struct snd_akm4xxx_dac_channel {
    pub name: *const c_char,
    pub switch_name: *const c_char,
    pub num_channels: c_int,
}

#[repr(C)]
pub struct snd_akm4xxx_adc_channel {
    pub name: *const c_char,
    pub switch_name: *const c_char,
    pub selector_name: *const c_char,
    pub input_names: *const *const c_char,
    pub num_channels: c_int,
}

#[repr(C)]
pub struct snd_akm4xxx {
    pub ops: snd_akm4xxx_ops,
    pub images: [u8; 16],
    pub volumes: [u8; 16],
    pub num_dacs: c_int,
    pub num_adcs: c_int,
    pub num_chips: c_int,
    pub idx_offset: c_int,
    pub type_: c_int,
    pub name: *const c_char,
    pub total_regs: c_int,
    pub card: *mut snd_card,
    pub dac_info: *mut snd_akm4xxx_dac_channel,
    pub adc_info: *mut snd_akm4xxx_adc_channel,
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/* write the given register and save the data to the cache */
#[no_mangle]
pub unsafe extern "C" fn snd_akm4xxx_write(
    ak: *mut snd_akm4xxx,
    chip: c_int,
    reg: u8,
    val: u8,
) {
    ((*ak).ops.lock)(ak, chip);
    ((*ak).ops.write)(ak, chip, reg, val);

    /* save the data */
    snd_akm4xxx_set(ak, chip, reg, val);
    ((*ak).ops.unlock)(ak, chip);
}

/* reset procedure for AK4524 and AK4528 */
unsafe extern "C" fn ak4524_reset(ak: *mut snd_akm4xxx, state: c_int) {
    let mut chip: c_uint = 0;
    while chip < ((*ak).num_dacs / 2) as c_uint {
        snd_akm4xxx_write(ak, chip as c_int, 0x01, if state != 0 { 0x00 } else { 0x03 });
        if state != 0 {
            chip += 1;
            continue;
        }
        /* DAC volumes */
        let mut reg: u8 = 0x04;
        while (reg as c_int) < (*ak).total_regs {
            snd_akm4xxx_write(ak, chip as c_int, reg, snd_akm4xxx_get(ak, chip as c_int, reg as c_int));
            reg = reg.wrapping_add(1);
        }
        chip += 1;
    }
}

/* reset procedure for AK4529 */
unsafe extern "C" fn ak4529_reset(ak: *mut snd_akm4xxx, state: c_int) {
    static REGS: [u8; 12] = [0x0a, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x0b, 0x0c, 0x08];

    if state != 0 {
        snd_akm4xxx_write(ak, 0, 0x09, snd_akm4xxx_get(ak, 0, 0x09) & !0x01);
        return;
    }

    let mut i: c_uint = 0;
    while (i as usize) < REGS.len() {
        let reg = REGS[i as usize];
        snd_akm4xxx_write(ak, 0, reg, snd_akm4xxx_get(ak, 0, reg as c_int));
        i += 1;
    }
    snd_akm4xxx_write(ak, 0, 0x09, snd_akm4xxx_get(ak, 0, 0x09) | 0x01);
}

/* reset procedure for AK4355 and AK4358 */
unsafe extern "C" fn ak435X_reset(ak: *mut snd_akm4xxx, state: c_int) {
    if state != 0 {
        snd_akm4xxx_write(ak, 0, 0x01, 0x02); /* reset and soft-mute */
        return;
    }
    let mut reg: u8 = 0x00;
    while (reg as c_int) < (*ak).total_regs {
        if reg != 0x01 {
            snd_akm4xxx_write(ak, 0, reg, snd_akm4xxx_get(ak, 0, reg as c_int));
        }
        reg = reg.wrapping_add(1);
    }
    snd_akm4xxx_write(ak, 0, 0x01, 0x01); /* un-reset, unmute */
}

/* reset procedure for AK4381 */
unsafe extern "C" fn ak4381_reset(ak: *mut snd_akm4xxx, state: c_int) {
    let mut chip: c_uint = 0;
    while chip < ((*ak).num_dacs / 2) as c_uint {
        snd_akm4xxx_write(ak, chip as c_int, 0x00, if state != 0 { 0x0c } else { 0x0f });
        if state != 0 {
            chip += 1;
            continue;
        }
        let mut reg: u8 = 0x01;
        while (reg as c_int) < (*ak).total_regs {
            snd_akm4xxx_write(ak, chip as c_int, reg, snd_akm4xxx_get(ak, chip as c_int, reg as c_int));
            reg = reg.wrapping_add(1);
        }
        chip += 1;
    }
}

/*
 * reset the AKM codecs
 * @state: 1 = reset codec, 0 = restore the registers
 *
 * assert the reset operation and restores the register values to the chips.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_akm4xxx_reset(ak: *mut snd_akm4xxx, state: c_int) {
    match (*ak).type_ {
        SND_AK4524 | SND_AK4528 | SND_AK4620 => ak4524_reset(ak, state),
        SND_AK4529 => ak4529_reset(ak, state),
        SND_AK4355 => ak435X_reset(ak, state),
        SND_AK4358 => ak435X_reset(ak, state),
        SND_AK4381 => ak4381_reset(ak, state),
        _ => {}
    }
}

/*
 * Volume conversion table for non-linear volumes
 * from -63.5dB (mute) to 0dB step 0.5dB
 *
 * Used for AK4524/AK4620 input/ouput attenuation, AK4528, and
 * AK5365 input attenuation
 */
static VOL_CVT_DATT: [u8; 128] = [
    0x00, 0x01, 0x01, 0x02, 0x02, 0x03, 0x03, 0x04,
    0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x06, 0x06,
    0x06, 0x07, 0x07, 0x08, 0x08, 0x08, 0x09, 0x0a,
    0x0a, 0x0b, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x0f,
    0x10, 0x10, 0x11, 0x12, 0x12, 0x13, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x17, 0x18, 0x19, 0x1a, 0x1c,
    0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x23,
    0x24, 0x25, 0x26, 0x28, 0x29, 0x2a, 0x2b, 0x2d,
    0x2e, 0x30, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
    0x37, 0x38, 0x39, 0x3b, 0x3c, 0x3e, 0x3f, 0x40,
    0x41, 0x42, 0x43, 0x44, 0x46, 0x47, 0x48, 0x4a,
    0x4b, 0x4d, 0x4e, 0x50, 0x51, 0x52, 0x53, 0x54,
    0x55, 0x56, 0x58, 0x59, 0x5b, 0x5c, 0x5e, 0x5f,
    0x60, 0x61, 0x62, 0x64, 0x65, 0x66, 0x67, 0x69,
    0x6a, 0x6c, 0x6d, 0x6f, 0x70, 0x71, 0x72, 0x73,
    0x75, 0x76, 0x77, 0x79, 0x7a, 0x7c, 0x7d, 0x7f,
];

/*
 * dB tables
 *
 * Original C uses:
 * static const DECLARE_TLV_DB_SCALE(db_scale_vol_datt, -6350, 50, 1);
 * static const DECLARE_TLV_DB_SCALE(db_scale_8bit, -12750, 50, 1);
 * static const DECLARE_TLV_DB_SCALE(db_scale_7bit, -6350, 50, 1);
 * static const DECLARE_TLV_DB_LINEAR(db_scale_linear, TLV_DB_GAIN_MUTE, 0);
 */

/*
 * initialize all the ak4xxx chips
 */
#[no_mangle]
pub unsafe extern "C" fn snd_akm4xxx_init(ak: *mut snd_akm4xxx) {
    static INITS_AK4524: [u8; 20] = [
        0x00, 0x07, /* 0: all power up */
        0x01, 0x00, /* 1: ADC/DAC reset */
        0x02, 0x60, /* 2: 24bit I2S */
        0x03, 0x19, /* 3: deemphasis off */
        0x01, 0x03, /* 1: ADC/DAC enable */
        0x04, 0x00, /* 4: ADC left muted */
        0x05, 0x00, /* 5: ADC right muted */
        0x06, 0x00, /* 6: DAC left muted */
        0x07, 0x00, /* 7: DAC right muted */
        0xff, 0xff,
    ];
    static INITS_AK4528: [u8; 16] = [
        0x00, 0x07, 0x01, 0x00, 0x02, 0x60, 0x03, 0x0d,
        0x01, 0x03, 0x04, 0x00, 0x05, 0x00, 0xff, 0xff,
    ];
    static INITS_AK4529: [u8; 28] = [
        0x09, 0x01, 0x0a, 0x3f, 0x00, 0x0c, 0x01, 0x00,
        0x02, 0xff, 0x03, 0xff, 0x04, 0xff, 0x05, 0xff,
        0x06, 0xff, 0x07, 0xff, 0x0b, 0xff, 0x0c, 0xff,
        0x08, 0x55, 0xff, 0xff,
    ];
    static INITS_AK4355: [u8; 26] = [
        0x01, 0x02, 0x00, 0x06, 0x02, 0x0e, /* 0x02, 0x2e, quad speed */
        0x03, 0x01, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x07, 0x00, 0x08, 0x00, 0x09, 0x00, 0x0a, 0x00,
        0x01, 0x01, 0xff, 0xff,
    ];
    static INITS_AK4358: [u8; 30] = [
        0x01, 0x02, 0x00, 0x06, 0x02, 0x4e, /* 0x02, 0x6e, quad speed */
        0x03, 0x01, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x07, 0x00, 0x08, 0x00, 0x09, 0x00, 0x0b, 0x00,
        0x0c, 0x00, 0x0a, 0x00, 0x01, 0x01, 0xff, 0xff,
    ];
    static INITS_AK4381: [u8; 16] = [
        0x00, 0x0c, 0x01, 0x02, /* 0x01, 0x12, quad speed */
        0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x0f,
        0xff, 0xff,
    ];
    static INITS_AK4620: [u8; 24] = [
        0x00, 0x07, 0x01, 0x00, 0x01, 0x02, 0x01, 0x03,
        0x01, 0x0f, 0x02, 0x60, 0x03, 0x01, 0x04, 0x00,
        0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0xff, 0xff,
    ];
    static AK5365_DEFAULTS: [u8; 8] = [0x01, 0x00, 0x00, 0x2b, 0x7f, 0x7f, 0x28, 0x89];

    let inits: *const u8;

    ptr::write_bytes((*ak).images.as_mut_ptr(), 0, (*ak).images.len());
    ptr::write_bytes((*ak).volumes.as_mut_ptr(), 0, (*ak).volumes.len());

    match (*ak).type_ {
        SND_AK4524 => {
            inits = INITS_AK4524.as_ptr();
            (*ak).num_chips = (*ak).num_dacs / 2;
            (*ak).name = cstr!("ak4524");
            (*ak).total_regs = 0x08;
        }
        SND_AK4528 => {
            inits = INITS_AK4528.as_ptr();
            (*ak).num_chips = (*ak).num_dacs / 2;
            (*ak).name = cstr!("ak4528");
            (*ak).total_regs = 0x06;
        }
        SND_AK4529 => {
            inits = INITS_AK4529.as_ptr();
            (*ak).num_chips = 1;
            (*ak).name = cstr!("ak4529");
            (*ak).total_regs = 0x0d;
        }
        SND_AK4355 => {
            inits = INITS_AK4355.as_ptr();
            (*ak).num_chips = 1;
            (*ak).name = cstr!("ak4355");
            (*ak).total_regs = 0x0b;
        }
        SND_AK4358 => {
            inits = INITS_AK4358.as_ptr();
            (*ak).num_chips = 1;
            (*ak).name = cstr!("ak4358");
            (*ak).total_regs = 0x10;
        }
        SND_AK4381 => {
            inits = INITS_AK4381.as_ptr();
            (*ak).num_chips = (*ak).num_dacs / 2;
            (*ak).name = cstr!("ak4381");
            (*ak).total_regs = 0x05;
        }
        SND_AK5365 => {
            (*ak).num_chips = 1;
            (*ak).name = cstr!("ak5365");
            (*ak).total_regs = 0x08;
            ptr::copy_nonoverlapping(AK5365_DEFAULTS.as_ptr(), (*ak).images.as_mut_ptr(), AK5365_DEFAULTS.len());
            snd_akm4xxx_set_vol(ak, 0, 0x04, 127);
            snd_akm4xxx_set_vol(ak, 0, 0x05, 127);
            return;
        }
        SND_AK4620 => {
            inits = INITS_AK4620.as_ptr();
            (*ak).num_chips = (*ak).num_dacs / 2;
            (*ak).name = cstr!("ak4620");
            (*ak).total_regs = 0x08;
        }
        _ => {
            snd_BUG();
            return;
        }
    }

    let mut chip: c_int = 0;
    while chip < (*ak).num_chips {
        let mut ptr_ = inits;
        while *ptr_ != 0xff {
            let reg = *ptr_;
            ptr_ = ptr_.add(1);
            let data = *ptr_;
            ptr_ = ptr_.add(1);
            snd_akm4xxx_write(ak, chip, reg, data);
            udelay(10);
        }
        chip += 1;
    }
}

/*
 * Mixer callbacks
 */
const AK_IPGA: c_ulong = 1 << 20; /* including IPGA */
const AK_VOL_CVT: c_ulong = 1 << 21; /* need dB conversion */
const AK_NEEDSMSB: c_ulong = 1 << 22; /* need MSB update bit */
const AK_INVERT: c_ulong = 1 << 23; /* data is inverted */

fn ak_get_chip(val: c_ulong) -> c_int {
    ((val >> 8) & 0xff) as c_int
}
fn ak_get_addr(val: c_ulong) -> c_int {
    (val & 0xff) as c_int
}
fn ak_get_shift(val: c_ulong) -> c_int {
    ((val >> 16) & 0x0f) as c_int
}
fn ak_get_vol_cvt(val: c_ulong) -> c_int {
    ((val >> 21) & 1) as c_int
}
fn ak_get_ipga(val: c_ulong) -> c_int {
    ((val >> 20) & 1) as c_int
}
fn ak_get_needsmsb(val: c_ulong) -> c_int {
    ((val >> 22) & 1) as c_int
}
fn ak_get_invert(val: c_ulong) -> c_int {
    ((val >> 23) & 1) as c_int
}
fn ak_get_mask(val: c_ulong) -> c_uint {
    ((val >> 24) & 0xff) as c_uint
}
fn ak_compose(chip: c_int, addr: c_int, shift: c_int, mask: c_int) -> c_ulong {
    (((chip as c_ulong) << 8)
        | (addr as c_ulong)
        | ((shift as c_ulong) << 16)
        | ((mask as c_ulong) << 24)) as c_ulong
}

unsafe extern "C" fn snd_akm4xxx_volume_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask = ak_get_mask((*kcontrol).private_value);

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_akm4xxx_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);

    (*ucontrol).value.integer.value[0] = snd_akm4xxx_get_vol(ak, chip, addr) as c_long;
    0
}

unsafe extern "C" fn put_ak_reg(kcontrol: *mut snd_kcontrol, addr: c_int, mut nval: u8) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let mask = ak_get_mask((*kcontrol).private_value);
    let chip = ak_get_chip((*kcontrol).private_value);

    if snd_akm4xxx_get_vol(ak, chip, addr) == nval as c_int {
        return 0;
    }

    snd_akm4xxx_set_vol(ak, chip, addr, nval as c_int);
    if ak_get_vol_cvt((*kcontrol).private_value) != 0 && nval < 128 {
        nval = VOL_CVT_DATT[nval as usize];
    }
    if ak_get_ipga((*kcontrol).private_value) != 0 && nval >= 128 {
        nval = nval.wrapping_add(1); /* need to correct + 1 since both 127 and 128 are 0dB */
    }
    if ak_get_invert((*kcontrol).private_value) != 0 {
        nval = (mask as u8).wrapping_sub(nval);
    }
    if ak_get_needsmsb((*kcontrol).private_value) != 0 {
        nval |= 0x80;
    }
    snd_akm4xxx_write(ak, chip, addr as u8, nval);
    1
}

unsafe extern "C" fn snd_akm4xxx_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mask = ak_get_mask((*kcontrol).private_value);
    let val = (*ucontrol).value.integer.value[0] as c_uint;
    if val > mask {
        return -EINVAL;
    }
    put_ak_reg(kcontrol, ak_get_addr((*kcontrol).private_value), val as u8)
}

unsafe extern "C" fn snd_akm4xxx_stereo_volume_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask = ak_get_mask((*kcontrol).private_value);

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_akm4xxx_stereo_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);

    (*ucontrol).value.integer.value[0] = snd_akm4xxx_get_vol(ak, chip, addr) as c_long;
    (*ucontrol).value.integer.value[1] = snd_akm4xxx_get_vol(ak, chip, addr + 1) as c_long;
    0
}

unsafe extern "C" fn snd_akm4xxx_stereo_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let addr = ak_get_addr((*kcontrol).private_value);
    let mask = ak_get_mask((*kcontrol).private_value);
    let val0 = (*ucontrol).value.integer.value[0] as c_uint;
    let val1 = (*ucontrol).value.integer.value[1] as c_uint;
    if val0 > mask || val1 > mask {
        return -EINVAL;
    }
    let mut change = put_ak_reg(kcontrol, addr, val0 as u8);
    change |= put_ak_reg(kcontrol, addr + 1, val1 as u8);
    change
}

unsafe extern "C" fn snd_akm4xxx_deemphasis_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 4] = [
        cstr!("44.1kHz"),
        cstr!("Off"),
        cstr!("48kHz"),
        cstr!("32kHz"),
    ];
    snd_ctl_enum_info(uinfo, 1, 4, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_akm4xxx_deemphasis_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let shift = ak_get_shift((*kcontrol).private_value);
    (*ucontrol).value.enumerated.item[0] =
        ((snd_akm4xxx_get(ak, chip, addr) as c_int >> shift) & 3) as c_uint;
    0
}

unsafe extern "C" fn snd_akm4xxx_deemphasis_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let shift = ak_get_shift((*kcontrol).private_value);
    let mut nval = ((*ucontrol).value.enumerated.item[0] & 3) as u8;
    let change: c_int;

    nval = ((nval as c_int) << shift | (snd_akm4xxx_get(ak, chip, addr) as c_int & !(3 << shift))) as u8;
    change = (snd_akm4xxx_get(ak, chip, addr) != nval) as c_int;
    if change != 0 {
        snd_akm4xxx_write(ak, chip, addr as u8, nval);
    }
    change
}

unsafe extern "C" fn ak4xxx_switch_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, uinfo)
}

unsafe extern "C" fn ak4xxx_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let shift = ak_get_shift((*kcontrol).private_value);
    let invert = ak_get_invert((*kcontrol).private_value);
    /* we observe the (1<<shift) bit only */
    let mut val = snd_akm4xxx_get(ak, chip, addr) & ((1 as c_int) << shift) as u8;
    if invert != 0 {
        val = (val == 0) as u8;
    }
    (*ucontrol).value.integer.value[0] = ((val & ((1 as c_int) << shift) as u8) != 0) as c_long;
    0
}

unsafe extern "C" fn ak4xxx_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let shift = ak_get_shift((*kcontrol).private_value);
    let invert = ak_get_invert((*kcontrol).private_value);
    let mut flag = (*ucontrol).value.integer.value[0];
    let val: u8;

    if invert != 0 {
        flag = (flag == 0) as c_long;
    }
    let oval = snd_akm4xxx_get(ak, chip, addr);
    if flag != 0 {
        val = oval | ((1 as c_int) << shift) as u8;
    } else {
        val = oval & !(((1 as c_int) << shift) as u8);
    }
    let change = (oval != val) as c_int;
    if change != 0 {
        snd_akm4xxx_write(ak, chip, addr as u8, val);
    }
    change
}

const AK5365_NUM_INPUTS: c_int = 5;

unsafe extern "C" fn ak4xxx_capture_num_inputs(ak: *mut snd_akm4xxx, mixer_ch: c_int) -> c_int {
    let input_names = (*(*ak).adc_info.add(mixer_ch as usize)).input_names;
    let mut num_names: c_int = 0;
    while num_names < AK5365_NUM_INPUTS && !(*input_names.add(num_names as usize)).is_null() {
        num_names += 1;
    }
    num_names
}

unsafe extern "C" fn ak4xxx_capture_source_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let mixer_ch = ak_get_shift((*kcontrol).private_value);
    let num_names = ak4xxx_capture_num_inputs(ak, mixer_ch) as c_uint;

    if num_names == 0 {
        return -EINVAL;
    }
    snd_ctl_enum_info(uinfo, 1, num_names, (*(*ak).adc_info.add(mixer_ch as usize)).input_names)
}

unsafe extern "C" fn ak4xxx_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let mask = ak_get_mask((*kcontrol).private_value) as c_int;
    let val = snd_akm4xxx_get(ak, chip, addr) as c_int & mask;
    (*ucontrol).value.enumerated.item[0] = val as c_uint;
    0
}

unsafe extern "C" fn ak4xxx_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak = snd_kcontrol_chip(kcontrol);
    let mixer_ch = ak_get_shift((*kcontrol).private_value);
    let chip = ak_get_chip((*kcontrol).private_value);
    let addr = ak_get_addr((*kcontrol).private_value);
    let mask = ak_get_mask((*kcontrol).private_value) as u8;
    let num_names = ak4xxx_capture_num_inputs(ak, mixer_ch);

    if (*ucontrol).value.enumerated.item[0] >= num_names as c_uint {
        return -EINVAL;
    }

    let oval = snd_akm4xxx_get(ak, chip, addr);
    let mut val = oval & !mask;
    val |= ((*ucontrol).value.enumerated.item[0] as u8) & mask;
    if val != oval {
        snd_akm4xxx_write(ak, chip, addr as u8, val);
        return 1;
    }
    0
}

/*
 * build AK4xxx controls
 */

unsafe extern "C" fn build_dac_controls(ak: *mut snd_akm4xxx) -> c_int {
    let mut mixer_ch: c_int = 0;
    let mut idx: c_int = 0;
    while idx < (*ak).num_dacs {
        let mut knew: snd_kcontrol_new = mem::zeroed();
        let mut num_stereo: c_int;

        /* mute control for Revolution 7.1 - AK4381 */
        if (*ak).type_ == SND_AK4381
            && !(*(*ak).dac_info.add(mixer_ch as usize)).switch_name.is_null()
        {
            ptr::write_bytes(&mut knew as *mut snd_kcontrol_new as *mut u8, 0, mem::size_of::<snd_kcontrol_new>());
            knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
            knew.count = 1;
            knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
            knew.name = (*(*ak).dac_info.add(mixer_ch as usize)).switch_name;
            knew.info = Some(ak4xxx_switch_info);
            knew.get = Some(ak4xxx_switch_get);
            knew.put = Some(ak4xxx_switch_put);
            knew.access = 0;
            /* register 1, bit 0 (SMUTE): 0 = normal operation,
               1 = mute */
            knew.private_value = ak_compose(idx / 2, 1, 0, 0) | AK_INVERT;
            let err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
            if err < 0 {
                return err;
            }
        }
        ptr::write_bytes(&mut knew as *mut snd_kcontrol_new as *mut u8, 0, mem::size_of::<snd_kcontrol_new>());
        if (*ak).dac_info.is_null() || (*(*ak).dac_info.add(mixer_ch as usize)).name.is_null() {
            knew.name = cstr!("DAC Volume");
            knew.index = (mixer_ch + (*ak).idx_offset * 2) as c_uint;
            num_stereo = 1;
        } else {
            knew.name = (*(*ak).dac_info.add(mixer_ch as usize)).name;
            num_stereo = (*(*ak).dac_info.add(mixer_ch as usize)).num_channels;
        }
        knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        knew.count = 1;
        knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ;
        if num_stereo == 2 {
            knew.info = Some(snd_akm4xxx_stereo_volume_info);
            knew.get = Some(snd_akm4xxx_stereo_volume_get);
            knew.put = Some(snd_akm4xxx_stereo_volume_put);
        } else {
            knew.info = Some(snd_akm4xxx_volume_info);
            knew.get = Some(snd_akm4xxx_volume_get);
            knew.put = Some(snd_akm4xxx_volume_put);
        }
        match (*ak).type_ {
            SND_AK4524 => {
                /* register 6 & 7 */
                knew.private_value = ak_compose(idx / 2, (idx % 2) + 6, 0, 127) | AK_VOL_CVT;
                knew.tlv.p = db_scale_vol_datt.as_ptr();
            }
            SND_AK4528 => {
                /* register 4 & 5 */
                knew.private_value = ak_compose(idx / 2, (idx % 2) + 4, 0, 127) | AK_VOL_CVT;
                knew.tlv.p = db_scale_vol_datt.as_ptr();
            }
            SND_AK4529 => {
                /* registers 2-7 and b,c */
                let val = if idx < 6 { idx + 2 } else { (idx - 6) + 0x0b };
                knew.private_value = ak_compose(0, val, 0, 255) | AK_INVERT;
                knew.tlv.p = db_scale_8bit.as_ptr();
            }
            SND_AK4355 => {
                /* register 4-9, chip #0 only */
                knew.private_value = ak_compose(0, idx + 4, 0, 255);
                knew.tlv.p = db_scale_8bit.as_ptr();
            }
            SND_AK4358 => {
                /* register 4-9 and 11-12, chip #0 only */
                let addr = if idx < 6 { idx + 4 } else { idx + 5 };
                knew.private_value = ak_compose(0, addr, 0, 127) | AK_NEEDSMSB;
                knew.tlv.p = db_scale_7bit.as_ptr();
            }
            SND_AK4381 => {
                /* register 3 & 4 */
                knew.private_value = ak_compose(idx / 2, (idx % 2) + 3, 0, 255);
                knew.tlv.p = db_scale_linear.as_ptr();
            }
            SND_AK4620 => {
                /* register 6 & 7 */
                knew.private_value = ak_compose(idx / 2, (idx % 2) + 6, 0, 255);
                knew.tlv.p = db_scale_linear.as_ptr();
            }
            _ => return -EINVAL,
        }

        let err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
        if err < 0 {
            return err;
        }

        idx += num_stereo;
        mixer_ch += 1;
    }
    0
}

unsafe extern "C" fn build_adc_controls(ak: *mut snd_akm4xxx) -> c_int {
    let mut mixer_ch: c_int = 0;
    if (*ak).type_ == SND_AK4528 {
        return 0; /* no controls */
    }
    let mut idx: c_int = 0;
    while idx < (*ak).num_adcs {
        let mut knew: snd_kcontrol_new = mem::zeroed();
        let num_stereo: c_int;
        if (*ak).adc_info.is_null() || (*(*ak).adc_info.add(mixer_ch as usize)).name.is_null() {
            knew.name = cstr!("ADC Volume");
            knew.index = (mixer_ch + (*ak).idx_offset * 2) as c_uint;
            num_stereo = 1;
        } else {
            knew.name = (*(*ak).adc_info.add(mixer_ch as usize)).name;
            num_stereo = (*(*ak).adc_info.add(mixer_ch as usize)).num_channels;
        }
        knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        knew.count = 1;
        knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ;
        if num_stereo == 2 {
            knew.info = Some(snd_akm4xxx_stereo_volume_info);
            knew.get = Some(snd_akm4xxx_stereo_volume_get);
            knew.put = Some(snd_akm4xxx_stereo_volume_put);
        } else {
            knew.info = Some(snd_akm4xxx_volume_info);
            knew.get = Some(snd_akm4xxx_volume_get);
            knew.put = Some(snd_akm4xxx_volume_put);
        }
        /* register 4 & 5 */
        let max_steps = if (*ak).type_ == SND_AK5365 { 152 } else { 164 };
        knew.private_value = ak_compose(idx / 2, (idx % 2) + 4, 0, max_steps) | AK_VOL_CVT | AK_IPGA;
        knew.tlv.p = db_scale_vol_datt.as_ptr();
        let mut err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
        if err < 0 {
            return err;
        }

        if (*ak).type_ == SND_AK5365 && (idx % 2) == 0 {
            if (*ak).adc_info.is_null() || (*(*ak).adc_info.add(mixer_ch as usize)).switch_name.is_null() {
                knew.name = cstr!("Capture Switch");
                knew.index = (mixer_ch + (*ak).idx_offset * 2) as c_uint;
            } else {
                knew.name = (*(*ak).adc_info.add(mixer_ch as usize)).switch_name;
            }
            knew.info = Some(ak4xxx_switch_info);
            knew.get = Some(ak4xxx_switch_get);
            knew.put = Some(ak4xxx_switch_put);
            knew.access = 0;
            /* register 2, bit 0 (SMUTE): 0 = normal operation,
               1 = mute */
            knew.private_value = ak_compose(idx / 2, 2, 0, 0) | AK_INVERT;
            err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
            if err < 0 {
                return err;
            }

            ptr::write_bytes(&mut knew as *mut snd_kcontrol_new as *mut u8, 0, mem::size_of::<snd_kcontrol_new>());
            if (*ak).adc_info.is_null() || (*(*ak).adc_info.add(mixer_ch as usize)).selector_name.is_null() {
                knew.name = cstr!("Capture Channel");
                knew.index = (mixer_ch + (*ak).idx_offset * 2) as c_uint;
            } else {
                knew.name = (*(*ak).adc_info.add(mixer_ch as usize)).selector_name;
            }

            knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
            knew.info = Some(ak4xxx_capture_source_info);
            knew.get = Some(ak4xxx_capture_source_get);
            knew.put = Some(ak4xxx_capture_source_put);
            knew.access = 0;
            /* input selector control: reg. 1, bits 0-2.
             * mis-use 'shift' to pass mixer_ch */
            knew.private_value = ak_compose(idx / 2, 1, mixer_ch, 0x07);
            err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
            if err < 0 {
                return err;
            }
        }

        idx += num_stereo;
        mixer_ch += 1;
    }
    0
}

unsafe extern "C" fn build_deemphasis(ak: *mut snd_akm4xxx, num_emphs: c_int) -> c_int {
    let mut idx: c_int = 0;
    while idx < num_emphs {
        let mut knew: snd_kcontrol_new = mem::zeroed();
        knew.name = cstr!("Deemphasis");
        knew.index = (idx + (*ak).idx_offset) as c_uint;
        knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        knew.count = 1;
        knew.info = Some(snd_akm4xxx_deemphasis_info);
        knew.get = Some(snd_akm4xxx_deemphasis_get);
        knew.put = Some(snd_akm4xxx_deemphasis_put);
        match (*ak).type_ {
            SND_AK4524 | SND_AK4528 | SND_AK4620 => {
                /* register 3 */
                knew.private_value = ak_compose(idx, 3, 0, 0);
            }
            SND_AK4529 => {
                let shift = if idx == 3 { 6 } else { (2 - idx) * 2 };
                /* register 8 with shift */
                knew.private_value = ak_compose(0, 8, shift, 0);
            }
            SND_AK4355 | SND_AK4358 => {
                knew.private_value = ak_compose(idx, 3, 0, 0);
            }
            SND_AK4381 => {
                knew.private_value = ak_compose(idx, 1, 1, 0);
            }
            _ => return -EINVAL,
        }
        let err = snd_ctl_add((*ak).card, snd_ctl_new1(&mut knew, ak as *mut c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    0
}

unsafe extern "C" fn proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ak = (*entry).private_data;
    let mut chip: c_int = 0;
    while chip < (*ak).num_chips {
        let mut reg: c_int = 0;
        while reg < (*ak).total_regs {
            let val = snd_akm4xxx_get(ak, chip, reg);
            snd_iprintf(
                buffer,
                cstr!("chip %d: 0x%02x = 0x%02x\n"),
                chip,
                reg,
                val as c_int,
            );
            reg += 1;
        }
        chip += 1;
    }
}

unsafe extern "C" fn proc_init(ak: *mut snd_akm4xxx) -> c_int {
    snd_card_ro_proc_new((*ak).card, (*ak).name, ak as *mut c_void, Some(proc_regs_read))
}

#[no_mangle]
pub unsafe extern "C" fn snd_akm4xxx_build_controls(ak: *mut snd_akm4xxx) -> c_int {
    let mut err = build_dac_controls(ak);
    if err < 0 {
        return err;
    }

    err = build_adc_controls(ak);
    if err < 0 {
        return err;
    }
    let num_emphs = if (*ak).type_ == SND_AK4355 || (*ak).type_ == SND_AK4358 {
        1
    } else if (*ak).type_ == SND_AK4620 {
        0
    } else {
        (*ak).num_dacs / 2
    };
    err = build_deemphasis(ak, num_emphs);
    if err < 0 {
        return err;
    }
    err = proc_init(ak);
    if err < 0 {
        return err;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
