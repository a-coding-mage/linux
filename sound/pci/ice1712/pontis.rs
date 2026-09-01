// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Pontis MS300
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

/* dependencies: linux/delay.h, linux/interrupt.h, linux/init.h,
 * linux/slab.h, linux/mutex.h, sound/core.h, sound/info.h,
 * sound/tlv.h, ice1712.h, envy24ht.h, pontis.h
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_ushort, c_void};

#[repr(C)]
pub struct snd_ice1712 {
    pub akm: *mut snd_akm4xxx,
    pub akm_codecs: c_int,
    pub gpio_mutex: mutex,
    pub gpio: snd_ice1712_gpio,
    pub card: *mut snd_card,
    pub vt1720: c_int,
    pub num_total_dacs: c_int,
    pub num_total_adcs: c_int,
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    pub write_mask: c_uint,
    pub direction: c_uint,
    pub saved: [c_uint; 3],
}

#[repr(C)]
pub struct snd_akm4xxx {
    pub images: [c_uchar; 256],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: isize,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
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
    pub min: i64,
    pub max: i64,
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
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub access: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_new_tlv,
    pub private_value: isize,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut snd_ice1712,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
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
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub eeprom_size: usize,
    pub eeprom_data: *const c_uchar,
}

unsafe extern "C" {
    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, dev: c_uint, addr: c_uint, data: c_uint);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ice1712;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint;
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, val: c_uint);
    fn snd_ice1712_gpio_set_dir(ice: *mut snd_ice1712, val: c_uint);
    fn snd_ice1712_gpio_set_mask(ice: *mut snd_ice1712, val: c_uint);
    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, format: *const c_char, ...);
    fn snd_card_rw_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut snd_ice1712,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
        write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut snd_ice1712,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut snd_ice1712) -> *mut snd_kcontrol;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn schedule_timeout_uninterruptible(timeout: isize);
    fn inb(port: c_uint) -> c_uchar;
    fn outb(value: c_uchar, port: c_uint);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn ICEMT1724(ice: *mut snd_ice1712, reg: c_uint) -> c_uint;
}

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_CARD: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0040_0000;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const AC97_CMD: c_uint = 0;
const VT1720_SUBDEVICE_PONTIS_MS300: c_uint = 0;
const ICE_EEP2_SYSCONF: usize = 0;
const ICE_EEP2_ACLINK: usize = 1;
const ICE_EEP2_I2S: usize = 2;
const ICE_EEP2_SPDIF: usize = 3;
const ICE_EEP2_GPIO_DIR: usize = 4;
const ICE_EEP2_GPIO_DIR1: usize = 5;
const ICE_EEP2_GPIO_DIR2: usize = 6;
const ICE_EEP2_GPIO_MASK: usize = 7;
const ICE_EEP2_GPIO_MASK1: usize = 8;
const ICE_EEP2_GPIO_MASK2: usize = 9;
const ICE_EEP2_GPIO_STATE: usize = 10;
const ICE_EEP2_GPIO_STATE1: usize = 11;
const ICE_EEP2_GPIO_STATE2: usize = 12;

/* I2C addresses */
const WM_DEV: c_uint = 0x34;
const CS_DEV: c_uint = 0x20;

/* WM8776 registers */
const WM_HP_ATTEN_L: c_int = 0x00; /* headphone left attenuation */
const WM_HP_ATTEN_R: c_int = 0x01; /* headphone left attenuation */
const WM_HP_MASTER: c_int = 0x02; /* headphone master (both channels) */
                                  /* override LLR */
const WM_DAC_ATTEN_L: c_int = 0x03; /* digital left attenuation */
const WM_DAC_ATTEN_R: c_int = 0x04;
const WM_DAC_MASTER: c_int = 0x05;
const WM_PHASE_SWAP: c_int = 0x06; /* DAC phase swap */
const WM_DAC_CTRL1: c_int = 0x07;
const WM_DAC_MUTE: c_int = 0x08;
const WM_DAC_CTRL2: c_int = 0x09;
const WM_DAC_INT: c_int = 0x0a;
const WM_ADC_INT: c_int = 0x0b;
const WM_MASTER_CTRL: c_int = 0x0c;
const WM_POWERDOWN: c_int = 0x0d;
const WM_ADC_ATTEN_L: c_int = 0x0e;
const WM_ADC_ATTEN_R: c_int = 0x0f;
const WM_ALC_CTRL1: c_int = 0x10;
const WM_ALC_CTRL2: c_int = 0x11;
const WM_ALC_CTRL3: c_int = 0x12;
const WM_NOISE_GATE: c_int = 0x13;
const WM_LIMITER: c_int = 0x14;
const WM_ADC_MUX: c_int = 0x15;
const WM_OUT_MUX: c_int = 0x16;
const WM_RESET: c_int = 0x17;

/*
 * GPIO
 */
const PONTIS_CS_CS: c_uint = 1 << 4; /* CS */
const PONTIS_CS_CLK: c_uint = 1 << 5; /* CLK */
const PONTIS_CS_RDATA: c_uint = 1 << 6; /* CS8416 -> VT1720 */
const PONTIS_CS_WDATA: c_uint = 1 << 7; /* VT1720 -> CS8416 */

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

/*
 * get the current register value of WM codec
 */
unsafe extern "C" fn wm_get(ice: *mut snd_ice1712, mut reg: c_int) -> c_ushort {
    reg <<= 1;
    unsafe {
        (((*(*ice).akm).images[reg as usize] as c_ushort) << 8)
            | (*(*ice).akm).images[(reg + 1) as usize] as c_ushort
    }
}

/*
 * set the register value of WM codec and remember it
 */
unsafe extern "C" fn wm_put_nocache(ice: *mut snd_ice1712, reg: c_int, val: c_ushort) {
    let cval: c_ushort = (((reg as c_ushort) << 9) | val) as c_ushort;
    unsafe { snd_vt1724_write_i2c(ice, WM_DEV, (cval >> 8) as c_uint, (cval & 0xff) as c_uint) };
}

unsafe extern "C" fn wm_put(ice: *mut snd_ice1712, mut reg: c_int, val: c_ushort) {
    unsafe { wm_put_nocache(ice, reg, val) };
    reg <<= 1;
    unsafe {
        (*(*ice).akm).images[reg as usize] = (val >> 8) as c_uchar;
        (*(*ice).akm).images[(reg + 1) as usize] = val as c_uchar;
    }
}

/*
 * DAC volume attenuation mixer control (-64dB to 0dB)
 */

const DAC_0DB: c_ushort = 0xff;
const DAC_RES: c_ushort = 128;
const DAC_MIN: c_ushort = DAC_0DB - DAC_RES;

unsafe extern "C" fn wm_dac_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0; /* mute */
        (*uinfo).value.integer.max = DAC_RES as i64; /* 0dB, 0.5dB step */
    }
    0
}

unsafe extern "C" fn wm_dac_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val: c_ushort;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for i in 0..2 {
        val = unsafe { wm_get(ice, WM_DAC_ATTEN_L + i) & 0xff };
        val = if val > DAC_MIN { val - DAC_MIN } else { 0 };
        unsafe { (*ucontrol).value.integer.value[i as usize] = val as i64 };
    }
    0
}

unsafe extern "C" fn wm_dac_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut oval: c_ushort;
    let mut nval: c_ushort;
    let mut idx: c_int;
    let mut change: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for i in 0..2 {
        nval = unsafe { (*ucontrol).value.integer.value[i as usize] as c_ushort };
        nval = (if nval != 0 { nval + DAC_MIN } else { 0 }) & 0xff;
        idx = WM_DAC_ATTEN_L + i;
        oval = unsafe { wm_get(ice, idx) & 0xff };
        if oval != nval {
            unsafe {
                wm_put(ice, idx, nval);
                wm_put_nocache(ice, idx, nval | 0x100);
            }
            change = 1;
        }
    }
    change
}

/*
 * ADC gain mixer control (-64dB to 0dB)
 */

const ADC_0DB: c_ushort = 0xcf;
const ADC_RES: c_ushort = 128;
const ADC_MIN: c_ushort = ADC_0DB - ADC_RES;

unsafe extern "C" fn wm_adc_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0; /* mute (-64dB) */
        (*uinfo).value.integer.max = ADC_RES as i64; /* 0dB, 0.5dB step */
    }
    0
}

unsafe extern "C" fn wm_adc_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val: c_ushort;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for i in 0..2 {
        val = unsafe { wm_get(ice, WM_ADC_ATTEN_L + i) & 0xff };
        val = if val > ADC_MIN { val - ADC_MIN } else { 0 };
        unsafe { (*ucontrol).value.integer.value[i as usize] = val as i64 };
    }
    0
}

unsafe extern "C" fn wm_adc_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut ovol: c_ushort;
    let mut nvol: c_ushort;
    let mut idx: c_int;
    let mut change: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for i in 0..2 {
        nvol = unsafe { (*ucontrol).value.integer.value[i as usize] as c_ushort };
        nvol = if nvol != 0 { nvol + ADC_MIN } else { 0 };
        idx = WM_ADC_ATTEN_L + i;
        ovol = unsafe { wm_get(ice, idx) & 0xff };
        if ovol != nvol {
            unsafe { wm_put(ice, idx, nvol) };
            change = 1;
        }
    }
    change
}

/*
 * ADC input mux mixer control
 */
const wm_adc_mux_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_adc_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let bit = unsafe { (*kcontrol).private_value as c_int };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        (*ucontrol).value.integer.value[0] =
            if (wm_get(ice, WM_ADC_MUX) & ((1 << bit) as c_ushort)) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn wm_adc_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let bit = unsafe { (*kcontrol).private_value as c_int };
    let oval: c_ushort;
    let mut nval: c_ushort;
    let change: c_int;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        oval = wm_get(ice, WM_ADC_MUX);
        nval = oval;
        if (*ucontrol).value.integer.value[0] != 0 {
            nval |= (1 << bit) as c_ushort;
        } else {
            nval &= !((1 << bit) as c_ushort);
        }
        change = (nval != oval) as c_int;
        if change != 0 {
            wm_put(ice, WM_ADC_MUX, nval);
        }
    }
    change
}

/*
 * Analog bypass (In -> Out)
 */
const wm_bypass_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_bypass_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        (*ucontrol).value.integer.value[0] = if (wm_get(ice, WM_OUT_MUX) & 0x04) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn wm_bypass_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val: c_ushort;
    let oval: c_ushort;
    let mut change: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        oval = wm_get(ice, WM_OUT_MUX);
        val = oval;
        if (*ucontrol).value.integer.value[0] != 0 {
            val |= 0x04;
        } else {
            val &= !0x04;
        }
        if val != oval {
            wm_put(ice, WM_OUT_MUX, val);
            change = 1;
        }
    }
    change
}

/*
 * Left/Right swap
 */
const wm_chswap_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn wm_chswap_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        (*ucontrol).value.integer.value[0] = ((wm_get(ice, WM_DAC_CTRL1) & 0xf0) != 0x90) as i64;
    }
    0
}

unsafe extern "C" fn wm_chswap_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val: c_ushort;
    let oval: c_ushort;
    let mut change: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        oval = wm_get(ice, WM_DAC_CTRL1);
        val = oval & 0x0f;
        if (*ucontrol).value.integer.value[0] != 0 {
            val |= 0x60;
        } else {
            val |= 0x90;
        }
        if val != oval {
            wm_put(ice, WM_DAC_CTRL1, val);
            wm_put_nocache(ice, WM_DAC_CTRL1, val);
            change = 1;
        }
    }
    change
}

/*
 * write data in the SPI mode
 */
unsafe extern "C" fn set_gpio_bit(ice: *mut snd_ice1712, bit: c_uint, val: c_int) {
    let mut tmp = unsafe { snd_ice1712_gpio_read(ice) };
    if val != 0 {
        tmp |= bit;
    } else {
        tmp &= !bit;
    }
    unsafe { snd_ice1712_gpio_write(ice, tmp) };
}

unsafe extern "C" fn spi_send_byte(ice: *mut snd_ice1712, mut data: c_uchar) {
    for _i in 0..8 {
        unsafe {
            set_gpio_bit(ice, PONTIS_CS_CLK, 0);
            udelay(1);
            set_gpio_bit(ice, PONTIS_CS_WDATA, (data & 0x80) as c_int);
            udelay(1);
            set_gpio_bit(ice, PONTIS_CS_CLK, 1);
            udelay(1);
        }
        data <<= 1;
    }
}

unsafe extern "C" fn spi_read_byte(ice: *mut snd_ice1712) -> c_uint {
    let mut val: c_uint = 0;

    for _i in 0..8 {
        val <<= 1;
        unsafe {
            set_gpio_bit(ice, PONTIS_CS_CLK, 0);
            udelay(1);
            if (snd_ice1712_gpio_read(ice) & PONTIS_CS_RDATA) != 0 {
                val |= 1;
            }
            udelay(1);
            set_gpio_bit(ice, PONTIS_CS_CLK, 1);
            udelay(1);
        }
    }
    val
}

unsafe extern "C" fn spi_write(ice: *mut snd_ice1712, dev: c_uint, reg: c_uint, data: c_uint) {
    unsafe {
        snd_ice1712_gpio_set_dir(ice, PONTIS_CS_CS | PONTIS_CS_WDATA | PONTIS_CS_CLK);
        snd_ice1712_gpio_set_mask(ice, !(PONTIS_CS_CS | PONTIS_CS_WDATA | PONTIS_CS_CLK));
        set_gpio_bit(ice, PONTIS_CS_CS, 0);
        spi_send_byte(ice, (dev & !1) as c_uchar); /* WRITE */
        spi_send_byte(ice, reg as c_uchar); /* MAP */
        spi_send_byte(ice, data as c_uchar); /* DATA */
        /* trigger */
        set_gpio_bit(ice, PONTIS_CS_CS, 1);
        udelay(1);
        /* restore */
        snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
    }
}

unsafe extern "C" fn spi_read(ice: *mut snd_ice1712, dev: c_uint, reg: c_uint) -> c_uint {
    let val: c_uint;
    unsafe {
        snd_ice1712_gpio_set_dir(ice, PONTIS_CS_CS | PONTIS_CS_WDATA | PONTIS_CS_CLK);
        snd_ice1712_gpio_set_mask(ice, !(PONTIS_CS_CS | PONTIS_CS_WDATA | PONTIS_CS_CLK));
        set_gpio_bit(ice, PONTIS_CS_CS, 0);
        spi_send_byte(ice, (dev & !1) as c_uchar); /* WRITE */
        spi_send_byte(ice, reg as c_uchar); /* MAP */
        /* trigger */
        set_gpio_bit(ice, PONTIS_CS_CS, 1);
        udelay(1);
        set_gpio_bit(ice, PONTIS_CS_CS, 0);
        spi_send_byte(ice, (dev | 1) as c_uchar); /* READ */
        val = spi_read_byte(ice);
        /* trigger */
        set_gpio_bit(ice, PONTIS_CS_CS, 1);
        udelay(1);
        /* restore */
        snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
    }
    val
}

/*
 * SPDIF input source
 */
unsafe extern "C" fn cs_source_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXT0: &[u8] = b"Coax\0";
    static TEXT1: &[u8] = b"Optical\0";
    static TEXT2: &[u8] = b"CD\0";
    let texts: [*const c_char; 3] = [
        TEXT0.as_ptr() as *const c_char, /* RXP0 */
        TEXT1.as_ptr() as *const c_char, /* RXP1 */
        TEXT2.as_ptr() as *const c_char, /* RXP2 */
    ];
    unsafe { snd_ctl_enum_info(uinfo, 1, 3, texts.as_ptr()) }
}

unsafe extern "C" fn cs_source_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe { (*ucontrol).value.enumerated.item[0] = (*ice).gpio.saved[0] };
    0
}

unsafe extern "C" fn cs_source_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uchar;
    let mut change: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        if (*ucontrol).value.enumerated.item[0] != (*ice).gpio.saved[0] {
            (*ice).gpio.saved[0] = (*ucontrol).value.enumerated.item[0] & 3;
            val = (0x80 | ((*ice).gpio.saved[0] << 3)) as c_uchar;
            spi_write(ice, CS_DEV, 0x04, val as c_uint);
            change = 1;
        }
    }
    change
}

/*
 * GPIO controls
 */
unsafe extern "C" fn pontis_gpio_mask_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 0xffff; /* 16bit */
    }
    0
}

unsafe extern "C" fn pontis_gpio_mask_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    /* 4-7 reserved */
    unsafe { (*ucontrol).value.integer.value[0] = ((!(*ice).gpio.write_mask & 0xffff) | 0x00f0) as i64 };
    0
}

unsafe extern "C" fn pontis_gpio_mask_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uint;
    let changed: c_int;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    /* 4-7 reserved */
    unsafe {
        val = (!((*ucontrol).value.integer.value[0] as c_uint) & 0xffff) | 0x00f0;
        changed = (val != (*ice).gpio.write_mask) as c_int;
        (*ice).gpio.write_mask = val;
    }
    changed
}

unsafe extern "C" fn pontis_gpio_dir_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    /* 4-7 reserved */
    unsafe { (*ucontrol).value.integer.value[0] = ((*ice).gpio.direction & 0xff0f) as i64 };
    0
}

unsafe extern "C" fn pontis_gpio_dir_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uint;
    let changed: c_int;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    /* 4-7 reserved */
    unsafe {
        val = ((*ucontrol).value.integer.value[0] as c_uint) & 0xff0f;
        changed = (val != (*ice).gpio.direction) as c_int;
        (*ice).gpio.direction = val;
    }
    changed
}

unsafe extern "C" fn pontis_gpio_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
        snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
        (*ucontrol).value.integer.value[0] = (snd_ice1712_gpio_read(ice) & 0xffff) as i64;
    }
    0
}

unsafe extern "C" fn pontis_gpio_data_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = unsafe { snd_kcontrol_chip(kcontrol) };
    let val: c_uint;
    let nval: c_uint;
    let mut changed: c_int = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
        snd_ice1712_gpio_set_mask(ice, (*ice).gpio.write_mask);
        val = snd_ice1712_gpio_read(ice) & 0xffff;
        nval = ((*ucontrol).value.integer.value[0] as c_uint) & 0xffff;
        if val != nval {
            snd_ice1712_gpio_write(ice, nval);
            changed = 1;
        }
    }
    changed
}

static db_scale_volume: [c_uint; 4] = [0, (-6400i32) as c_uint, 50, 1];

/*
 * mixers
 */

static pontis_controls: [snd_kcontrol_new; 10] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
        info: Some(wm_dac_vol_info),
        get: Some(wm_dac_vol_get),
        put: Some(wm_dac_vol_put),
        tlv: snd_kcontrol_new_tlv { p: db_scale_volume.as_ptr() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: b"Capture Volume\0".as_ptr() as *const c_char,
        info: Some(wm_adc_vol_info),
        get: Some(wm_adc_vol_get),
        put: Some(wm_adc_vol_put),
        tlv: snd_kcontrol_new_tlv { p: db_scale_volume.as_ptr() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"CD Capture Switch\0".as_ptr() as *const c_char,
        info: wm_adc_mux_info,
        get: Some(wm_adc_mux_get),
        put: Some(wm_adc_mux_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"Line Capture Switch\0".as_ptr() as *const c_char,
        info: wm_adc_mux_info,
        get: Some(wm_adc_mux_get),
        put: Some(wm_adc_mux_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 1,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"Analog Bypass Switch\0".as_ptr() as *const c_char,
        info: wm_bypass_info,
        get: Some(wm_bypass_get),
        put: Some(wm_bypass_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"Swap Output Channels\0".as_ptr() as *const c_char,
        info: wm_chswap_info,
        get: Some(wm_chswap_get),
        put: Some(wm_chswap_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: 0,
        name: b"IEC958 Input Source\0".as_ptr() as *const c_char,
        info: Some(cs_source_info),
        get: Some(cs_source_get),
        put: Some(cs_source_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    /* FIXME: which interface? */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_CARD,
        access: 0,
        name: b"GPIO Mask\0".as_ptr() as *const c_char,
        info: Some(pontis_gpio_mask_info),
        get: Some(pontis_gpio_mask_get),
        put: Some(pontis_gpio_mask_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_CARD,
        access: 0,
        name: b"GPIO Direction\0".as_ptr() as *const c_char,
        info: Some(pontis_gpio_mask_info),
        get: Some(pontis_gpio_dir_get),
        put: Some(pontis_gpio_dir_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_CARD,
        access: 0,
        name: b"GPIO Data\0".as_ptr() as *const c_char,
        info: Some(pontis_gpio_mask_info),
        get: Some(pontis_gpio_data_get),
        put: Some(pontis_gpio_data_put),
        tlv: snd_kcontrol_new_tlv { p: core::ptr::null() },
        private_value: 0,
    },
];

/*
 * WM codec registers
 */
unsafe extern "C" fn wm_proc_regs_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice = unsafe { (*entry).private_data };
    let mut line = [0 as c_char; 64];
    let mut reg: c_uint = 0;
    let mut val: c_uint = 0;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    unsafe {
        while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
            if sscanf(line.as_ptr(), b"%x %x\0".as_ptr() as *const c_char, &mut reg, &mut val) != 2 {
                continue;
            }
            if reg <= 0x17 && val <= 0xffff {
                wm_put(ice, reg as c_int, val as c_ushort);
            }
        }
    }
}

unsafe extern "C" fn wm_proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice = unsafe { (*entry).private_data };
    let mut val: c_int;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for reg in 0..=0x17 {
        val = unsafe { wm_get(ice, reg) as c_int };
        unsafe { snd_iprintf(buffer, b"%02x = %04x\n\0".as_ptr() as *const c_char, reg, val) };
    }
}

unsafe extern "C" fn wm_proc_init(ice: *mut snd_ice1712) {
    unsafe {
        snd_card_rw_proc_new(
            (*ice).card,
            b"wm_codec\0".as_ptr() as *const c_char,
            ice,
            Some(wm_proc_regs_read),
            Some(wm_proc_regs_write),
        );
    }
}

unsafe extern "C" fn cs_proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice = unsafe { (*entry).private_data };
    let mut val: c_int;

    let _guard = unsafe { MutexGuard::new(core::ptr::addr_of_mut!((*ice).gpio_mutex)) };
    for reg in 0..=0x26 {
        val = unsafe { spi_read(ice, CS_DEV, reg as c_uint) as c_int };
        unsafe { snd_iprintf(buffer, b"%02x = %02x\n\0".as_ptr() as *const c_char, reg, val) };
    }
    val = unsafe { spi_read(ice, CS_DEV, 0x7f) as c_int };
    unsafe { snd_iprintf(buffer, b"%02x = %02x\n\0".as_ptr() as *const c_char, 0x7f, val) };
}

unsafe extern "C" fn cs_proc_init(ice: *mut snd_ice1712) {
    unsafe {
        snd_card_ro_proc_new(
            (*ice).card,
            b"cs_codec\0".as_ptr() as *const c_char,
            ice,
            Some(cs_proc_regs_read),
        );
    }
}

unsafe extern "C" fn pontis_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut err: c_int;

    for i in 0..pontis_controls.len() {
        unsafe {
            err = snd_ctl_add((*ice).card, snd_ctl_new1(&pontis_controls[i], ice));
        }
        if err < 0 {
            return err;
        }
    }

    unsafe {
        wm_proc_init(ice);
        cs_proc_init(ice);
    }

    0
}

/*
 * initialize the chip
 */
unsafe extern "C" fn pontis_init(ice: *mut snd_ice1712) -> c_int {
    static wm_inits: [c_ushort; 10] = [
        /* These come first to reduce init pop noise */
        WM_ADC_MUX as c_ushort, 0x00c0, /* ADC mute */
        WM_DAC_MUTE as c_ushort, 0x0001, /* DAC softmute */
        WM_DAC_CTRL1 as c_ushort, 0x0000, /* DAC mute */

        WM_POWERDOWN as c_ushort, 0x0008, /* All power-up except HP */
        WM_RESET as c_ushort, 0x0000, /* reset */
    ];
    static wm_inits2: [c_ushort; 34] = [
        WM_MASTER_CTRL as c_ushort, 0x0022, /* 256fs, slave mode */
        WM_DAC_INT as c_ushort, 0x0022, /* I2S, normal polarity, 24bit */
        WM_ADC_INT as c_ushort, 0x0022, /* I2S, normal polarity, 24bit */
        WM_DAC_CTRL1 as c_ushort, 0x0090, /* DAC L/R */
        WM_OUT_MUX as c_ushort, 0x0001, /* OUT DAC */
        WM_HP_ATTEN_L as c_ushort, 0x0179, /* HP 0dB */
        WM_HP_ATTEN_R as c_ushort, 0x0179, /* HP 0dB */
        WM_DAC_ATTEN_L as c_ushort, 0x0000, /* DAC 0dB */
        WM_DAC_ATTEN_L as c_ushort, 0x0100, /* DAC 0dB */
        WM_DAC_ATTEN_R as c_ushort, 0x0000, /* DAC 0dB */
        WM_DAC_ATTEN_R as c_ushort, 0x0100, /* DAC 0dB */
        /* WM_DAC_MASTER,	0x0100, */ /* DAC master muted */
        WM_PHASE_SWAP as c_ushort, 0x0000, /* phase normal */
        WM_DAC_CTRL2 as c_ushort, 0x0000, /* no deemphasis, no ZFLG */
        WM_ADC_ATTEN_L as c_ushort, 0x0000, /* ADC muted */
        WM_ADC_ATTEN_R as c_ushort, 0x0000, /* ADC muted */
        /* #if 0 disabled ALC/noise gate initialization:
         * WM_ALC_CTRL1 0x007b, WM_ALC_CTRL2 0x0000,
         * WM_ALC_CTRL3 0x0000, WM_NOISE_GATE 0x0000
         */
        WM_DAC_MUTE as c_ushort, 0x0000, /* DAC unmute */
        WM_ADC_MUX as c_ushort, 0x0003, /* ADC unmute, both CD/Line On */
    ];
    static cs_inits: [c_uchar; 10] = [
        0x04, 0x80, /* RUN, RXP0 */
        0x05, 0x05, /* slave, 24bit */
        0x01, 0x00,
        0x02, 0x00,
        0x03, 0x00,
    ];

    unsafe {
        (*ice).vt1720 = 1;
        (*ice).num_total_dacs = 2;
        (*ice).num_total_adcs = 2;

        /* to remember the register values */
        (*ice).akm = kzalloc(core::mem::size_of::<snd_akm4xxx>(), GFP_KERNEL) as *mut snd_akm4xxx;
        if (*ice).akm.is_null() {
            return -ENOMEM;
        }
        (*ice).akm_codecs = 1;

        /* HACK - use this as the SPDIF source.
         * don't call snd_ice1712_gpio_get/put(), otherwise it's overwritten
         */
        (*ice).gpio.saved[0] = 0;

        /* initialize WM8776 codec */
        let mut i = 0;
        while i < wm_inits.len() {
            wm_put(ice, wm_inits[i] as c_int, wm_inits[i + 1]);
            i += 2;
        }
        schedule_timeout_uninterruptible(1);
        i = 0;
        while i < wm_inits2.len() {
            wm_put(ice, wm_inits2[i] as c_int, wm_inits2[i + 1]);
            i += 2;
        }

        /* initialize CS8416 codec */
        /* assert PRST#; MT05 bit 7 */
        outb(inb(ICEMT1724(ice, AC97_CMD)) | 0x80, ICEMT1724(ice, AC97_CMD));
        mdelay(5);
        /* deassert PRST# */
        outb(inb(ICEMT1724(ice, AC97_CMD)) & !0x80, ICEMT1724(ice, AC97_CMD));

        i = 0;
        while i < cs_inits.len() {
            spi_write(ice, CS_DEV, cs_inits[i] as c_uint, cs_inits[i + 1] as c_uint);
            i += 2;
        }
    }

    0
}

/*
 * Pontis boards don't provide the EEPROM data at all.
 * hence the driver needs to sets up it properly.
 */

static pontis_eeprom: [c_uchar; 13] = {
    let mut data = [0 as c_uchar; 13];
    data[ICE_EEP2_SYSCONF] = 0x08; /* clock 256, mpu401, spdif-in/ADC, 1DAC */
    data[ICE_EEP2_ACLINK] = 0x80; /* I2S */
    data[ICE_EEP2_I2S] = 0xf8; /* vol, 96k, 24bit, 192k */
    data[ICE_EEP2_SPDIF] = 0xc3; /* out-en, out-int, spdif-in */
    data[ICE_EEP2_GPIO_DIR] = 0x07;
    data[ICE_EEP2_GPIO_DIR1] = 0x00;
    data[ICE_EEP2_GPIO_DIR2] = 0x00; /* ignored */
    data[ICE_EEP2_GPIO_MASK] = 0x0f; /* 4-7 reserved for CS8416 */
    data[ICE_EEP2_GPIO_MASK1] = 0xff;
    data[ICE_EEP2_GPIO_MASK2] = 0x00; /* ignored */
    data[ICE_EEP2_GPIO_STATE] = 0x06; /* 0-low, 1-high, 2-high */
    data[ICE_EEP2_GPIO_STATE1] = 0x00;
    data[ICE_EEP2_GPIO_STATE2] = 0x00; /* ignored */
    data
};

/* entry point */
#[no_mangle]
pub static mut snd_vt1720_pontis_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_PONTIS_MS300,
        name: b"Pontis MS300\0".as_ptr() as *const c_char,
        model: b"ms300\0".as_ptr() as *const c_char,
        chip_init: Some(pontis_init),
        build_controls: Some(pontis_add_controls),
        eeprom_size: core::mem::size_of_val(&pontis_eeprom),
        eeprom_data: pontis_eeprom.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: core::ptr::null(),
        model: core::ptr::null(),
        chip_init: None,
        build_controls: None,
        eeprom_size: 0,
        eeprom_data: core::ptr::null(),
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
