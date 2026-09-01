// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Audiotrak Prodigy 7.1 Hifi
 *   based on pontis.c
 *
 *      Copyright (c) 2007 Julian Scheel <julian@jusst.de>
 *      Copyright (c) 2007 allank
 *      Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

// Dependencies from linux/delay.h, linux/interrupt.h, linux/init.h,
// linux/slab.h, linux/mutex.h, sound/core.h, sound/info.h, sound/tlv.h,
// ice1712.h, envy24ht.h, and prodigy_hifi.h are external to this translation.

#[repr(C)]
struct prodigy_hifi_spec {
    master: [u16; 2],
    vol: [u16; 8],
}

/* I2C addresses */
const WM_DEV: u32 = 0x34;

/* WM8776 registers */
const WM_HP_ATTEN_L: u32 = 0x00; /* headphone left attenuation */
const WM_HP_ATTEN_R: u32 = 0x01; /* headphone left attenuation */
const WM_HP_MASTER: u32 = 0x02; /* headphone master (both channels),
                                 * override LLR */
const WM_DAC_ATTEN_L: u32 = 0x03; /* digital left attenuation */
const WM_DAC_ATTEN_R: u32 = 0x04;
const WM_DAC_MASTER: u32 = 0x05;
const WM_PHASE_SWAP: u32 = 0x06; /* DAC phase swap */
const WM_DAC_CTRL1: u32 = 0x07;
const WM_DAC_MUTE: u32 = 0x08;
const WM_DAC_CTRL2: u32 = 0x09;
const WM_DAC_INT: u32 = 0x0a;
const WM_ADC_INT: u32 = 0x0b;
const WM_MASTER_CTRL: u32 = 0x0c;
const WM_POWERDOWN: u32 = 0x0d;
const WM_ADC_ATTEN_L: u32 = 0x0e;
const WM_ADC_ATTEN_R: u32 = 0x0f;
const WM_ALC_CTRL1: u32 = 0x10;
const WM_ALC_CTRL2: u32 = 0x11;
const WM_ALC_CTRL3: u32 = 0x12;
const WM_NOISE_GATE: u32 = 0x13;
const WM_LIMITER: u32 = 0x14;
const WM_ADC_MUX: u32 = 0x15;
const WM_OUT_MUX: u32 = 0x16;
const WM_RESET: u32 = 0x17;

/* Analog Recording Source :- Mic, LineIn, CD/Video, */

/* implement capture source select control for WM8776 */

const WM_AIN1: &CStr = c"AIN1";
const WM_AIN2: &CStr = c"AIN2";
const WM_AIN3: &CStr = c"AIN3";
const WM_AIN4: &CStr = c"AIN4";
const WM_AIN5: &CStr = c"AIN5";

/* GPIO pins of envy24ht connected to wm8766 */
const WM8766_SPI_CLK: u32 = 1 << 17; /* CLK, Pin97 on ICE1724 */
const WM8766_SPI_MD: u32 = 1 << 16; /* DATA VT1724 -> WM8766, Pin96 */
const WM8766_SPI_ML: u32 = 1 << 18; /* Latch, Pin98 */

/* WM8766 registers */
const WM8766_DAC_CTRL: u32 = 0x02; /* DAC Control */
const WM8766_INT_CTRL: u32 = 0x03; /* Interface Control */
const WM8766_DAC_CTRL2: u32 = 0x09;
const WM8766_DAC_CTRL3: u32 = 0x0a;
const WM8766_RESET: u32 = 0x1f;
const WM8766_LDA1: u32 = 0x00;
const WM8766_LDA2: u32 = 0x04;
const WM8766_LDA3: u32 = 0x06;
const WM8766_RDA1: u32 = 0x01;
const WM8766_RDA2: u32 = 0x05;
const WM8766_RDA3: u32 = 0x07;
const WM8766_MUTE1: u32 = 0x0C;
const WM8766_MUTE2: u32 = 0x0F;

/*
 * Prodigy HD2
 */
const AK4396_ADDR: u32 = 0x00;
const AK4396_CSN: u32 = 1 << 8; /* CSN->GPIO8, pin 75 */
const AK4396_CCLK: u32 = 1 << 9; /* CCLK->GPIO9, pin 76 */
const AK4396_CDTI: u32 = 1 << 10; /* CDTI->GPIO10, pin 77 */

/* ak4396 registers */
const AK4396_CTRL1: u32 = 0x00;
const AK4396_CTRL2: u32 = 0x01;
const AK4396_CTRL3: u32 = 0x02;
const AK4396_LCH_ATT: u32 = 0x03;
const AK4396_RCH_ATT: u32 = 0x04;

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr};

extern "C" {
    static db_scale_wm_dac: [c_uint; 0];
    static ak4396_db_scale: [c_uint; 0];

    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, dev: c_uint, addr: c_uint, data: c_uint);
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint;
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, val: c_uint);
    fn snd_ice1712_gpio_set_dir(ice: *mut snd_ice1712, data: c_uint);
    fn snd_ice1712_gpio_set_mask(ice: *mut snd_ice1712, data: c_uint);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn schedule_timeout_uninterruptible(timeout: c_long) -> c_long;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, format: *const c_char, ...) -> c_int;
    fn snd_card_rw_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
        write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

type c_long = isize;

#[repr(C)]
struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 128],
}

#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_akm4xxx {
    images: [u8; 256],
}

#[repr(C)]
struct snd_ice1712_gpio {
    write_mask: c_uint,
    direction: c_uint,
    saved: [c_uint; 3],
}

#[repr(C)]
struct snd_ice1712 {
    akm: *mut snd_akm4xxx,
    gpio: snd_ice1712_gpio,
    gpio_mutex: c_void,
    spec: *mut prodigy_hifi_spec,
    vt1720: c_int,
    vt1724: c_int,
    num_total_dacs: c_int,
    num_total_adcs: c_int,
    akm_codecs: c_int,
    card: *mut snd_card,
    pm_resume: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pm_suspend_enabled: c_int,
}

#[repr(C)]
union snd_kcontrol_new_tlv {
    p: *const c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    access: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
    tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
struct snd_ice1712_card_info {
    subvendor: c_uint,
    name: *const c_char,
    model: *const c_char,
    chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    eeprom_size: usize,
    eeprom_data: *const u8,
    driver: *const c_char,
}

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0004;
const TLV_DB_GAIN_MUTE: c_int = -9999999;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const VT1724_SUBDEVICE_PRODIGY_HIFI: c_uint = 0;
const VT1724_SUBDEVICE_PRODIGY_HD2: c_uint = 0;
const VT1724_SUBDEVICE_FORTISSIMO4: c_uint = 0;

unsafe fn mutex_guard<T>(_mutex: *mut T) {}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

/*
 * get the current register value of WM codec
 */
unsafe extern "C" fn wm_get(ice: *mut snd_ice1712, mut reg: c_int) -> u16 {
    reg <<= 1;
    (((*(*ice).akm).images[reg as usize] as u16) << 8) | (*(*ice).akm).images[(reg + 1) as usize] as u16
}

/*
 * set the register value of WM codec and remember it
 */
unsafe extern "C" fn wm_put_nocache(ice: *mut snd_ice1712, reg: c_int, val: u16) {
    let cval: u16 = ((reg as u16) << 9) | val;
    snd_vt1724_write_i2c(ice, WM_DEV, (cval >> 8) as c_uint, (cval & 0xff) as c_uint);
}

unsafe extern "C" fn wm_put(ice: *mut snd_ice1712, mut reg: c_int, val: u16) {
    wm_put_nocache(ice, reg, val);
    reg <<= 1;
    (*(*ice).akm).images[reg as usize] = (val >> 8) as u8;
    (*(*ice).akm).images[(reg + 1) as usize] = val as u8;
}

/*
 * write data in the SPI mode
 */

unsafe extern "C" fn set_gpio_bit(ice: *mut snd_ice1712, bit: c_uint, val: c_int) {
    let mut tmp: c_uint = snd_ice1712_gpio_read(ice);
    if val != 0 {
        tmp |= bit;
    } else {
        tmp &= !bit;
    }
    snd_ice1712_gpio_write(ice, tmp);
}

/*
 * SPI implementation for WM8766 codec - only writing supported, no readback
 */

unsafe extern "C" fn wm8766_spi_send_word(ice: *mut snd_ice1712, mut data: c_uint) {
    let mut i: c_int = 0;
    while i < 16 {
        set_gpio_bit(ice, WM8766_SPI_CLK, 0);
        udelay(1);
        set_gpio_bit(ice, WM8766_SPI_MD, (data & 0x8000) as c_int);
        udelay(1);
        set_gpio_bit(ice, WM8766_SPI_CLK, 1);
        udelay(1);
        data <<= 1;
        i += 1;
    }
}

unsafe extern "C" fn wm8766_spi_write(ice: *mut snd_ice1712, reg: c_uint, data: c_uint) {
    let block: c_uint;

    snd_ice1712_gpio_set_dir(ice, WM8766_SPI_MD | WM8766_SPI_CLK | WM8766_SPI_ML);
    snd_ice1712_gpio_set_mask(ice, !(WM8766_SPI_MD | WM8766_SPI_CLK | WM8766_SPI_ML));
    /* latch must be low when writing */
    set_gpio_bit(ice, WM8766_SPI_ML, 0);
    block = (reg << 9) | (data & 0x1ff);
    wm8766_spi_send_word(ice, block); /* REGISTER ADDRESS */
    /* release latch */
    set_gpio_bit(ice, WM8766_SPI_ML, 1);
    udelay(1);
    /* restore */
    snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
    snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
}

/*
 * serial interface for ak4396 - only writing supported, no readback
 */

unsafe extern "C" fn ak4396_send_word(ice: *mut snd_ice1712, mut data: c_uint) {
    let mut i: c_int = 0;
    while i < 16 {
        set_gpio_bit(ice, AK4396_CCLK, 0);
        udelay(1);
        set_gpio_bit(ice, AK4396_CDTI, (data & 0x8000) as c_int);
        udelay(1);
        set_gpio_bit(ice, AK4396_CCLK, 1);
        udelay(1);
        data <<= 1;
        i += 1;
    }
}

unsafe extern "C" fn ak4396_write(ice: *mut snd_ice1712, reg: c_uint, data: c_uint) {
    let block: c_uint;

    snd_ice1712_gpio_set_dir(ice, AK4396_CSN | AK4396_CCLK | AK4396_CDTI);
    snd_ice1712_gpio_set_mask(ice, !(AK4396_CSN | AK4396_CCLK | AK4396_CDTI));
    /* latch must be low when writing */
    set_gpio_bit(ice, AK4396_CSN, 0);
    block = ((AK4396_ADDR & 0x03) << 14) | (1 << 13) | ((reg & 0x1f) << 8) | (data & 0xff);
    ak4396_send_word(ice, block); /* REGISTER ADDRESS */
    /* release latch */
    set_gpio_bit(ice, AK4396_CSN, 1);
    udelay(1);
    /* restore */
    snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
    snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
}

/*
 * ak4396 mixers
 */

/*
 * DAC volume attenuation mixer control (-64dB to 0dB)
 */

unsafe extern "C" fn ak4396_dac_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0; /* mute */
    (*uinfo).value.integer.max = 0xFF; /* linear */
    0
}

unsafe extern "C" fn ak4396_dac_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;

    while i < 2 {
        (*ucontrol).value.integer.value[i as usize] = (*spec).vol[i as usize] as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn ak4396_dac_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while i < 2 {
        if (*ucontrol).value.integer.value[i as usize] != (*spec).vol[i as usize] as c_long {
            (*spec).vol[i as usize] = (*ucontrol).value.integer.value[i as usize] as u16;
            ak4396_write(ice, AK4396_LCH_ATT + i as c_uint, ((*spec).vol[i as usize] & 0xff) as c_uint);
            change = 1;
        }
        i += 1;
    }
    change
}

// static const DECLARE_TLV_DB_SCALE(db_scale_wm_dac, -12700, 100, 1);
// static const DECLARE_TLV_DB_LINEAR(ak4396_db_scale, TLV_DB_GAIN_MUTE, 0);

static prodigy_hd2_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"Front Playback Volume".as_ptr(),
    info: Some(ak4396_dac_vol_info),
    get: Some(ak4396_dac_vol_get),
    put: Some(ak4396_dac_vol_put),
    private_value: 0,
    tlv: snd_kcontrol_new_tlv {
        p: unsafe { ak4396_db_scale.as_ptr() },
    },
}];

/* --------------- */

const WM_VOL_MAX: u16 = 255;
const WM_VOL_MUTE: u16 = 0x8000;

const DAC_0dB: u16 = 0xff;
const DAC_RES: u16 = 128;
const DAC_MIN: u16 = DAC_0dB - DAC_RES;

unsafe extern "C" fn wm_set_vol(ice: *mut snd_ice1712, index: c_uint, vol: u16, master: u16) {
    let nvol: u8;

    if (master & WM_VOL_MUTE) != 0 || (vol & WM_VOL_MUTE) != 0 {
        nvol = 0;
    } else {
        let mut tmp = ((((vol & !WM_VOL_MUTE) as c_uint) * ((master & !WM_VOL_MUTE) as c_uint)) / 128)
            & WM_VOL_MAX as c_uint;
        tmp = if tmp != 0 { tmp + DAC_MIN as c_uint } else { 0 } & 0xff;
        nvol = tmp as u8;
    }

    wm_put(ice, index as c_int, nvol as u16);
    wm_put_nocache(ice, index as c_int, 0x100 | nvol as u16);
}

unsafe extern "C" fn wm8766_set_vol(ice: *mut snd_ice1712, index: c_uint, vol: u16, master: u16) {
    let nvol: u8;

    if (master & WM_VOL_MUTE) != 0 || (vol & WM_VOL_MUTE) != 0 {
        nvol = 0;
    } else {
        let mut tmp = ((((vol & !WM_VOL_MUTE) as c_uint) * ((master & !WM_VOL_MUTE) as c_uint)) / 128)
            & WM_VOL_MAX as c_uint;
        tmp = if tmp != 0 { tmp + DAC_MIN as c_uint } else { 0 } & 0xff;
        nvol = tmp as u8;
    }

    wm8766_spi_write(ice, index, 0x0100 | nvol as c_uint);
}

/*
 * DAC volume attenuation mixer control (-64dB to 0dB)
 */

unsafe extern "C" fn wm_dac_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0; /* mute */
    (*uinfo).value.integer.max = DAC_RES as c_long; /* 0dB, 0.5dB step */
    0
}

unsafe extern "C" fn wm_dac_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;

    while i < 2 {
        (*ucontrol).value.integer.value[i as usize] =
            ((*spec).vol[(2 + i) as usize] & !WM_VOL_MUTE) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_dac_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;
    let mut idx: c_int;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while i < 2 {
        if (*ucontrol).value.integer.value[i as usize] != (*spec).vol[(2 + i) as usize] as c_long {
            idx = WM_DAC_ATTEN_L as c_int + i;
            (*spec).vol[(2 + i) as usize] &= WM_VOL_MUTE;
            (*spec).vol[(2 + i) as usize] |= (*ucontrol).value.integer.value[i as usize] as u16;
            wm_set_vol(ice, idx as c_uint, (*spec).vol[(2 + i) as usize], (*spec).master[i as usize]);
            change = 1;
        }
        i += 1;
    }
    change
}

/*
 * WM8766 DAC volume attenuation mixer control
 */
unsafe extern "C" fn wm8766_vol_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let voices: c_int = ((*kcontrol).private_value >> 8) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = voices as c_uint;
    (*uinfo).value.integer.min = 0; /* mute */
    (*uinfo).value.integer.max = DAC_RES as c_long; /* 0dB */
    0
}

unsafe extern "C" fn wm8766_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let voices: c_int = ((*kcontrol).private_value >> 8) as c_int;
    let ofs: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let mut i: c_int = 0;

    while i < voices {
        (*ucontrol).value.integer.value[i as usize] = (*spec).vol[(ofs + i) as usize] as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm8766_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let voices: c_int = ((*kcontrol).private_value >> 8) as c_int;
    let ofs: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let mut i: c_int = 0;
    let mut idx: c_int;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while i < voices {
        if (*ucontrol).value.integer.value[i as usize] != (*spec).vol[(ofs + i) as usize] as c_long {
            idx = WM8766_LDA1 as c_int + ofs + i;
            (*spec).vol[(ofs + i) as usize] &= WM_VOL_MUTE;
            (*spec).vol[(ofs + i) as usize] |= (*ucontrol).value.integer.value[i as usize] as u16;
            wm8766_set_vol(ice, idx as c_uint, (*spec).vol[(ofs + i) as usize], (*spec).master[i as usize]);
            change = 1;
        }
        i += 1;
    }
    change
}

/*
 * Master volume attenuation mixer control / applied to WM8776+WM8766
 */
unsafe extern "C" fn wm_master_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = DAC_RES as c_long;
    0
}

unsafe extern "C" fn wm_master_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;
    while i < 2 {
        (*ucontrol).value.integer.value[i as usize] = (*spec).master[i as usize] as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_master_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut ch: c_int = 0;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while ch < 2 {
        if (*ucontrol).value.integer.value[ch as usize] != (*spec).master[ch as usize] as c_long {
            (*spec).master[ch as usize] = (*ucontrol).value.integer.value[ch as usize] as u16;

            /* Apply to front DAC */
            wm_set_vol(ice, WM_DAC_ATTEN_L + ch as c_uint, (*spec).vol[(2 + ch) as usize], (*spec).master[ch as usize]);

            wm8766_set_vol(ice, WM8766_LDA1 + ch as c_uint, (*spec).vol[(0 + ch) as usize], (*spec).master[ch as usize]);

            wm8766_set_vol(ice, WM8766_LDA2 + ch as c_uint, (*spec).vol[(4 + ch) as usize], (*spec).master[ch as usize]);

            wm8766_set_vol(ice, WM8766_LDA3 + ch as c_uint, (*spec).vol[(6 + ch) as usize], (*spec).master[ch as usize]);
            change = 1;
        }
        ch += 1;
    }
    change
}

/* KONSTI */

unsafe extern "C" fn wm_adc_mux_enum_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 32] = [
        c"NULL".as_ptr(), c"AIN1".as_ptr(), c"AIN2".as_ptr(), c"AIN1+AIN2".as_ptr(),
        c"AIN3".as_ptr(), c"AIN1+AIN3".as_ptr(), c"AIN2+AIN3".as_ptr(),
        c"AIN1+AIN2+AIN3".as_ptr(),
        c"AIN4".as_ptr(), c"AIN1+AIN4".as_ptr(), c"AIN2+AIN4".as_ptr(),
        c"AIN1+AIN2+AIN4".as_ptr(),
        c"AIN3+AIN4".as_ptr(), c"AIN1+AIN3+AIN4".as_ptr(),
        c"AIN2+AIN3+AIN4".as_ptr(),
        c"AIN1+AIN2+AIN3+AIN4".as_ptr(),
        c"AIN5".as_ptr(), c"AIN1+AIN5".as_ptr(), c"AIN2+AIN5".as_ptr(),
        c"AIN1+AIN2+AIN5".as_ptr(),
        c"AIN3+AIN5".as_ptr(), c"AIN1+AIN3+AIN5".as_ptr(),
        c"AIN2+AIN3+AIN5".as_ptr(),
        c"AIN1+AIN2+AIN3+AIN5".as_ptr(),
        c"AIN4+AIN5".as_ptr(), c"AIN1+AIN4+AIN5".as_ptr(),
        c"AIN2+AIN4+AIN5".as_ptr(),
        c"AIN1+AIN2+AIN4+AIN5".as_ptr(),
        c"AIN3+AIN4+AIN5".as_ptr(),
        c"AIN1+AIN3+AIN4+AIN5".as_ptr(),
        c"AIN2+AIN3+AIN4+AIN5".as_ptr(),
        c"AIN1+AIN2+AIN3+AIN4+AIN5".as_ptr(),
    ];

    snd_ctl_enum_info(uinfo, 1, 32, TEXTS.as_ptr())
}

unsafe extern "C" fn wm_adc_mux_enum_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;

    mutex_guard(&mut (*ice).gpio_mutex);
    (*ucontrol).value.enumerated.item[0] = (wm_get(ice, WM_ADC_MUX as c_int) & 0x1f) as c_uint;
    0
}

unsafe extern "C" fn wm_adc_mux_enum_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let oval: u16;
    let nval: u16;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    oval = wm_get(ice, WM_ADC_MUX as c_int);
    nval = (oval & 0xe0) | (*ucontrol).value.enumerated.item[0] as u16;
    if nval != oval {
        wm_put(ice, WM_ADC_MUX as c_int, nval);
        change = 1;
    }
    change
}

/* KONSTI */

/*
 * ADC gain mixer control (-64dB to 0dB)
 */

const ADC_0dB: u16 = 0xcf;
const ADC_RES: u16 = 128;
const ADC_MIN: u16 = ADC_0dB - ADC_RES;

unsafe extern "C" fn wm_adc_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0; /* mute (-64dB) */
    (*uinfo).value.integer.max = ADC_RES as c_long; /* 0dB, 0.5dB step */
    0
}

unsafe extern "C" fn wm_adc_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let mut val: u16;
    let mut i: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while i < 2 {
        val = wm_get(ice, WM_ADC_ATTEN_L as c_int + i) & 0xff;
        val = if val > ADC_MIN { val - ADC_MIN } else { 0 };
        (*ucontrol).value.integer.value[i as usize] = val as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_adc_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let mut ovol: u16;
    let mut nvol: u16;
    let mut i: c_int = 0;
    let mut idx: c_int;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while i < 2 {
        nvol = (*ucontrol).value.integer.value[i as usize] as u16;
        nvol = if nvol != 0 { nvol + ADC_MIN } else { 0 };
        idx = WM_ADC_ATTEN_L as c_int + i;
        ovol = wm_get(ice, idx) & 0xff;
        if ovol != nvol {
            wm_put(ice, idx, nvol);
            change = 1;
        }
        i += 1;
    }
    change
}

/*
 * ADC input mux mixer control
 */
const wm_adc_mux_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_adc_mux_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let bit: c_int = (*kcontrol).private_value as c_int;

    mutex_guard(&mut (*ice).gpio_mutex);
    (*ucontrol).value.integer.value[0] =
        if (wm_get(ice, WM_ADC_MUX as c_int) & (1 << bit)) != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn wm_adc_mux_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let bit: c_int = (*kcontrol).private_value as c_int;
    let oval: u16;
    let mut nval: u16;
    let change: c_int;

    mutex_guard(&mut (*ice).gpio_mutex);
    nval = wm_get(ice, WM_ADC_MUX as c_int);
    oval = nval;
    if (*ucontrol).value.integer.value[0] != 0 {
        nval |= (1 << bit) as u16;
    } else {
        nval &= !((1 << bit) as u16);
    }
    change = (nval != oval) as c_int;
    if change != 0 {
        wm_put(ice, WM_ADC_MUX as c_int, nval);
    }
    0
}

/*
 * Analog bypass (In -> Out)
 */
const wm_bypass_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_bypass_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;

    mutex_guard(&mut (*ice).gpio_mutex);
    (*ucontrol).value.integer.value[0] = if (wm_get(ice, WM_OUT_MUX as c_int) & 0x04) != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn wm_bypass_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let mut val: u16;
    let oval: u16;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    val = wm_get(ice, WM_OUT_MUX as c_int);
    oval = val;
    if (*ucontrol).value.integer.value[0] != 0 {
        val |= 0x04;
    } else {
        val &= !0x04;
    }
    if val != oval {
        wm_put(ice, WM_OUT_MUX as c_int, val);
        change = 1;
    }
    change
}

/*
 * Left/Right swap
 */
const wm_chswap_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_chswap_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;

    mutex_guard(&mut (*ice).gpio_mutex);
    (*ucontrol).value.integer.value[0] = ((wm_get(ice, WM_DAC_CTRL1 as c_int) & 0xf0) != 0x90) as c_long;
    0
}

unsafe extern "C" fn wm_chswap_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let mut val: u16;
    let oval: u16;
    let mut change: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    oval = wm_get(ice, WM_DAC_CTRL1 as c_int);
    val = oval & 0x0f;
    if (*ucontrol).value.integer.value[0] != 0 {
        val |= 0x60;
    } else {
        val |= 0x90;
    }
    if val != oval {
        wm_put(ice, WM_DAC_CTRL1 as c_int, val);
        wm_put_nocache(ice, WM_DAC_CTRL1 as c_int, val);
        change = 1;
    }
    change
}

/*
 * mixers
 */

static prodigy_hifi_controls: [snd_kcontrol_new; 12] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Master Playback Volume".as_ptr(), info: Some(wm_master_vol_info), get: Some(wm_master_vol_get), put: Some(wm_master_vol_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Front Playback Volume".as_ptr(), info: Some(wm_dac_vol_info), get: Some(wm_dac_vol_get), put: Some(wm_dac_vol_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Rear Playback Volume".as_ptr(), info: Some(wm8766_vol_info), get: Some(wm8766_vol_get), put: Some(wm8766_vol_put), private_value: ((2 << 8) | 0) as c_ulong, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Center Playback Volume".as_ptr(), info: Some(wm8766_vol_info), get: Some(wm8766_vol_get), put: Some(wm8766_vol_put), private_value: ((1 << 8) | 4) as c_ulong, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"LFE Playback Volume".as_ptr(), info: Some(wm8766_vol_info), get: Some(wm8766_vol_get), put: Some(wm8766_vol_put), private_value: ((1 << 8) | 5) as c_ulong, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Side Playback Volume".as_ptr(), info: Some(wm8766_vol_info), get: Some(wm8766_vol_get), put: Some(wm8766_vol_put), private_value: ((2 << 8) | 6) as c_ulong, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Capture Volume".as_ptr(), info: Some(wm_adc_vol_info), get: Some(wm_adc_vol_get), put: Some(wm_adc_vol_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: unsafe { db_scale_wm_dac.as_ptr() } } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: 0, name: c"CD Capture Switch".as_ptr(), info: wm_adc_mux_info, get: Some(wm_adc_mux_get), put: Some(wm_adc_mux_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: core::ptr::null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: 0, name: c"Line Capture Switch".as_ptr(), info: wm_adc_mux_info, get: Some(wm_adc_mux_get), put: Some(wm_adc_mux_put), private_value: 1, tlv: snd_kcontrol_new_tlv { p: core::ptr::null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: 0, name: c"Analog Bypass Switch".as_ptr(), info: wm_bypass_info, get: Some(wm_bypass_get), put: Some(wm_bypass_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: core::ptr::null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: 0, name: c"Swap Output Channels".as_ptr(), info: wm_chswap_info, get: Some(wm_chswap_get), put: Some(wm_chswap_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: core::ptr::null() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: 0, name: c"Analog Capture Source".as_ptr(), info: Some(wm_adc_mux_enum_info), get: Some(wm_adc_mux_enum_get), put: Some(wm_adc_mux_enum_put), private_value: 0, tlv: snd_kcontrol_new_tlv { p: core::ptr::null() } },
];

/*
 * WM codec registers
 */
unsafe extern "C" fn wm_proc_regs_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice: *mut snd_ice1712 = (*entry).private_data as *mut snd_ice1712;
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: c_uint = 0;
    let mut val: c_uint = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    while snd_info_get_line(buffer, line.as_mut_ptr(), core::mem::size_of_val(&line) as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x".as_ptr(), &mut reg, &mut val) != 2 {
            continue;
        }
        if reg <= 0x17 && val <= 0xffff {
            wm_put(ice, reg as c_int, val as u16);
        }
    }
}

unsafe extern "C" fn wm_proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice: *mut snd_ice1712 = (*entry).private_data as *mut snd_ice1712;
    let mut reg: c_int = 0;
    let mut val: c_int;

    mutex_guard(&mut (*ice).gpio_mutex);
    while reg <= 0x17 {
        val = wm_get(ice, reg) as c_int;
        snd_iprintf(buffer, c"%02x = %04x\n".as_ptr(), reg, val);
        reg += 1;
    }
}

unsafe extern "C" fn wm_proc_init(ice: *mut snd_ice1712) {
    snd_card_rw_proc_new(
        (*ice).card,
        c"wm_codec".as_ptr(),
        ice as *mut c_void,
        Some(wm_proc_regs_read),
        Some(wm_proc_regs_write),
    );
}

unsafe extern "C" fn prodigy_hifi_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut i: c_uint = 0;
    let mut err: c_int;

    while (i as usize) < prodigy_hifi_controls.len() {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&prodigy_hifi_controls[i as usize], ice as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    wm_proc_init(ice);

    0
}

unsafe extern "C" fn prodigy_hd2_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut i: c_uint = 0;
    let mut err: c_int;

    while (i as usize) < prodigy_hd2_controls.len() {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&prodigy_hd2_controls[i as usize], ice as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    wm_proc_init(ice);

    0
}

unsafe extern "C" fn wm8766_init(ice: *mut snd_ice1712) {
    static wm8766_inits: [u16; 26] = [
        WM8766_RESET as u16, 0x0000,
        WM8766_DAC_CTRL as u16, 0x0120,
        WM8766_INT_CTRL as u16, 0x0022, /* I2S Normal Mode, 24 bit */
        WM8766_DAC_CTRL2 as u16, 0x0001,
        WM8766_DAC_CTRL3 as u16, 0x0080,
        WM8766_LDA1 as u16, 0x0100,
        WM8766_LDA2 as u16, 0x0100,
        WM8766_LDA3 as u16, 0x0100,
        WM8766_RDA1 as u16, 0x0100,
        WM8766_RDA2 as u16, 0x0100,
        WM8766_RDA3 as u16, 0x0100,
        WM8766_MUTE1 as u16, 0x0000,
        WM8766_MUTE2 as u16, 0x0000,
    ];
    let mut i: c_uint = 0;

    while (i as usize) < wm8766_inits.len() {
        wm8766_spi_write(ice, wm8766_inits[i as usize] as c_uint, wm8766_inits[(i + 1) as usize] as c_uint);
        i += 2;
    }
}

unsafe extern "C" fn wm8776_init(ice: *mut snd_ice1712) {
    static wm8776_inits: [u16; 10] = [
        /* These come first to reduce init pop noise */
        WM_ADC_MUX as u16, 0x0003, /* ADC mute */
        /* 0x00c0 replaced by 0x0003 */

        WM_DAC_MUTE as u16, 0x0001, /* DAC softmute */
        WM_DAC_CTRL1 as u16, 0x0000, /* DAC mute */

        WM_POWERDOWN as u16, 0x0008, /* All power-up except HP */
        WM_RESET as u16, 0x0000, /* reset */
    ];
    let mut i: c_uint = 0;

    while (i as usize) < wm8776_inits.len() {
        wm_put(ice, wm8776_inits[i as usize] as c_int, wm8776_inits[(i + 1) as usize]);
        i += 2;
    }
}

// #ifdef CONFIG_PM_SLEEP
unsafe extern "C" fn prodigy_hifi_resume(ice: *mut snd_ice1712) -> c_int {
    static wm8776_reinit_registers: [u16; 15] = [
        WM_MASTER_CTRL as u16,
        WM_DAC_INT as u16,
        WM_ADC_INT as u16,
        WM_OUT_MUX as u16,
        WM_HP_ATTEN_L as u16,
        WM_HP_ATTEN_R as u16,
        WM_PHASE_SWAP as u16,
        WM_DAC_CTRL2 as u16,
        WM_ADC_ATTEN_L as u16,
        WM_ADC_ATTEN_R as u16,
        WM_ALC_CTRL1 as u16,
        WM_ALC_CTRL2 as u16,
        WM_ALC_CTRL3 as u16,
        WM_NOISE_GATE as u16,
        WM_ADC_MUX as u16,
        /* no DAC attenuation here */
    ];
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;
    let mut ch: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);

    /* reinitialize WM8776 and re-apply old register values */
    wm8776_init(ice);
    schedule_timeout_uninterruptible(1);
    while (i as usize) < wm8776_reinit_registers.len() {
        wm_put(ice, wm8776_reinit_registers[i as usize] as c_int, wm_get(ice, wm8776_reinit_registers[i as usize] as c_int));
        i += 1;
    }

    /* reinitialize WM8766 and re-apply volumes for all DACs */
    wm8766_init(ice);
    while ch < 2 {
        wm_set_vol(ice, WM_DAC_ATTEN_L + ch as c_uint, (*spec).vol[(2 + ch) as usize], (*spec).master[ch as usize]);

        wm8766_set_vol(ice, WM8766_LDA1 + ch as c_uint, (*spec).vol[(0 + ch) as usize], (*spec).master[ch as usize]);

        wm8766_set_vol(ice, WM8766_LDA2 + ch as c_uint, (*spec).vol[(4 + ch) as usize], (*spec).master[ch as usize]);

        wm8766_set_vol(ice, WM8766_LDA3 + ch as c_uint, (*spec).vol[(6 + ch) as usize], (*spec).master[ch as usize]);
        ch += 1;
    }

    /* unmute WM8776 DAC */
    wm_put(ice, WM_DAC_MUTE as c_int, 0x00);
    wm_put(ice, WM_DAC_CTRL1 as c_int, 0x90);

    0
}
// #endif

/*
 * initialize the chip
 */
unsafe extern "C" fn prodigy_hifi_init(ice: *mut snd_ice1712) -> c_int {
    static wm8776_defaults: [u16; 40] = [
        WM_MASTER_CTRL as u16, 0x0022, /* 256fs, slave mode */
        WM_DAC_INT as u16, 0x0022, /* I2S, normal polarity, 24bit */
        WM_ADC_INT as u16, 0x0022, /* I2S, normal polarity, 24bit */
        WM_DAC_CTRL1 as u16, 0x0090, /* DAC L/R */
        WM_OUT_MUX as u16, 0x0001, /* OUT DAC */
        WM_HP_ATTEN_L as u16, 0x0179, /* HP 0dB */
        WM_HP_ATTEN_R as u16, 0x0179, /* HP 0dB */
        WM_DAC_ATTEN_L as u16, 0x0000, /* DAC 0dB */
        WM_DAC_ATTEN_L as u16, 0x0100, /* DAC 0dB */
        WM_DAC_ATTEN_R as u16, 0x0000, /* DAC 0dB */
        WM_DAC_ATTEN_R as u16, 0x0100, /* DAC 0dB */
        WM_PHASE_SWAP as u16, 0x0000, /* phase normal */
        // #if 0
        // WM_DAC_MASTER, 0x0100, /* DAC master muted */
        // #endif
        WM_DAC_CTRL2 as u16, 0x0000, /* no deemphasis, no ZFLG */
        WM_ADC_ATTEN_L as u16, 0x0000, /* ADC muted */
        WM_ADC_ATTEN_R as u16, 0x0000, /* ADC muted */
        // #if 1
        WM_ALC_CTRL1 as u16, 0x007b, /* */
        WM_ALC_CTRL2 as u16, 0x0000, /* */
        WM_ALC_CTRL3 as u16, 0x0000, /* */
        WM_NOISE_GATE as u16, 0x0000, /* */
        // #endif
        WM_DAC_MUTE as u16, 0x0000, /* DAC unmute */
        WM_ADC_MUX as u16, 0x0003, /* ADC unmute, both CD/Line On */
    ];
    let mut spec: *mut prodigy_hifi_spec;
    let mut i: c_uint = 0;

    (*ice).vt1720 = 0;
    (*ice).vt1724 = 1;

    (*ice).num_total_dacs = 8;
    (*ice).num_total_adcs = 1;

    /* HACK - use this as the SPDIF source.
     * don't call snd_ice1712_gpio_get/put(), otherwise it's overwritten
     */
    (*ice).gpio.saved[0] = 0;
    /* to remember the register values */

    (*ice).akm = kzalloc_obj::<snd_akm4xxx>();
    if (*ice).akm.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    spec = kzalloc_obj::<prodigy_hifi_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec;

    /* initialize WM8776 codec */
    wm8776_init(ice);
    schedule_timeout_uninterruptible(1);
    while (i as usize) < wm8776_defaults.len() {
        wm_put(ice, wm8776_defaults[i as usize] as c_int, wm8776_defaults[(i + 1) as usize]);
        i += 2;
    }

    wm8766_init(ice);

    // #ifdef CONFIG_PM_SLEEP
    (*ice).pm_resume = Some(prodigy_hifi_resume);
    (*ice).pm_suspend_enabled = 1;
    // #endif

    0
}

/*
 * initialize the chip
 */
unsafe extern "C" fn ak4396_init(ice: *mut snd_ice1712) {
    static ak4396_inits: [u16; 10] = [
        AK4396_CTRL1 as u16, 0x87, /* I2S Normal Mode, 24 bit */
        AK4396_CTRL2 as u16, 0x02,
        AK4396_CTRL3 as u16, 0x00,
        AK4396_LCH_ATT as u16, 0x00,
        AK4396_RCH_ATT as u16, 0x00,
    ];

    let mut i: c_uint = 0;

    /* initialize ak4396 codec */
    /* reset codec */
    ak4396_write(ice, AK4396_CTRL1, 0x86);
    msleep(100);
    ak4396_write(ice, AK4396_CTRL1, 0x87);

    while (i as usize) < ak4396_inits.len() {
        ak4396_write(ice, ak4396_inits[i as usize] as c_uint, ak4396_inits[(i + 1) as usize] as c_uint);
        i += 2;
    }
}

// #ifdef CONFIG_PM_SLEEP
unsafe extern "C" fn prodigy_hd2_resume(ice: *mut snd_ice1712) -> c_int {
    /* initialize ak4396 codec and restore previous mixer volumes */
    let spec: *mut prodigy_hifi_spec = (*ice).spec;
    let mut i: c_int = 0;

    mutex_guard(&mut (*ice).gpio_mutex);
    ak4396_init(ice);
    while i < 2 {
        ak4396_write(ice, AK4396_LCH_ATT + i as c_uint, ((*spec).vol[i as usize] & 0xff) as c_uint);
        i += 1;
    }
    0
}
// #endif

unsafe extern "C" fn prodigy_hd2_init(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut prodigy_hifi_spec;

    (*ice).vt1720 = 0;
    (*ice).vt1724 = 1;

    (*ice).num_total_dacs = 1;
    (*ice).num_total_adcs = 1;

    /* HACK - use this as the SPDIF source.
     * don't call snd_ice1712_gpio_get/put(), otherwise it's overwritten
     */
    (*ice).gpio.saved[0] = 0;
    /* to remember the register values */

    (*ice).akm = kzalloc_obj::<snd_akm4xxx>();
    if (*ice).akm.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    spec = kzalloc_obj::<prodigy_hifi_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec;

    // #ifdef CONFIG_PM_SLEEP
    (*ice).pm_resume = Some(prodigy_hd2_resume);
    (*ice).pm_suspend_enabled = 1;
    // #endif

    ak4396_init(ice);

    0
}

static prodigy71hifi_eeprom: [u8; 13] = [
    0x4b, /* SYSCONF: clock 512, spdif-in/ADC, 4DACs */
    0x80, /* ACLINK: I2S */
    0xfc, /* I2S: vol, 96k, 24bit, 192k */
    0xc3, /* SPDIF: out-en, out-int, spdif-in */
    0xff, /* GPIO_DIR */
    0xff, /* GPIO_DIR1 */
    0x5f, /* GPIO_DIR2 */
    0x00, /* GPIO_MASK */
    0x00, /* GPIO_MASK1 */
    0x00, /* GPIO_MASK2 */
    0x00, /* GPIO_STATE */
    0x00, /* GPIO_STATE1 */
    0x00, /* GPIO_STATE2 */
];

static prodigyhd2_eeprom: [u8; 13] = [
    0x4b, /* SYSCONF: clock 512, spdif-in/ADC, 4DACs */
    0x80, /* ACLINK: I2S */
    0xfc, /* I2S: vol, 96k, 24bit, 192k */
    0xc3, /* SPDIF: out-en, out-int, spdif-in */
    0xff, /* GPIO_DIR */
    0xff, /* GPIO_DIR1 */
    0x5f, /* GPIO_DIR2 */
    0x00, /* GPIO_MASK */
    0x00, /* GPIO_MASK1 */
    0x00, /* GPIO_MASK2 */
    0x00, /* GPIO_STATE */
    0x00, /* GPIO_STATE1 */
    0x00, /* GPIO_STATE2 */
];

static fortissimo4_eeprom: [u8; 13] = [
    0x43, /* SYSCONF: clock 512, ADC, 4DACs */
    0x80, /* ACLINK: I2S */
    0xfc, /* I2S: vol, 96k, 24bit, 192k */
    0xc1, /* SPDIF: out-en, out-int */
    0xff, /* GPIO_DIR */
    0xff, /* GPIO_DIR1 */
    0x5f, /* GPIO_DIR2 */
    0x00, /* GPIO_MASK */
    0x00, /* GPIO_MASK1 */
    0x00, /* GPIO_MASK2 */
    0x00, /* GPIO_STATE */
    0x00, /* GPIO_STATE1 */
    0x00, /* GPIO_STATE2 */
];

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_prodigy_hifi_cards: [snd_ice1712_card_info; 4] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY_HIFI,
        name: c"Audiotrak Prodigy 7.1 HiFi".as_ptr(),
        model: c"prodigy71hifi".as_ptr(),
        chip_init: Some(prodigy_hifi_init),
        build_controls: Some(prodigy_hifi_add_controls),
        eeprom_size: core::mem::size_of_val(&prodigy71hifi_eeprom),
        eeprom_data: prodigy71hifi_eeprom.as_ptr(),
        driver: c"Prodigy71HIFI".as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY_HD2,
        name: c"Audiotrak Prodigy HD2".as_ptr(),
        model: c"prodigyhd2".as_ptr(),
        chip_init: Some(prodigy_hd2_init),
        build_controls: Some(prodigy_hd2_add_controls),
        eeprom_size: core::mem::size_of_val(&prodigyhd2_eeprom),
        eeprom_data: prodigyhd2_eeprom.as_ptr(),
        driver: c"Prodigy71HD2".as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_FORTISSIMO4,
        name: c"Hercules Fortissimo IV".as_ptr(),
        model: c"fortissimo4".as_ptr(),
        chip_init: Some(prodigy_hifi_init),
        build_controls: Some(prodigy_hifi_add_controls),
        eeprom_size: core::mem::size_of_val(&fortissimo4_eeprom),
        eeprom_data: fortissimo4_eeprom.as_ptr(),
        driver: c"Fortissimo4".as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: core::ptr::null(),
        model: core::ptr::null(),
        chip_init: None,
        build_controls: None,
        eeprom_size: 0,
        eeprom_data: core::ptr::null(),
        driver: core::ptr::null(),
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
