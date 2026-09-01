// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for AudioTrak Prodigy 192 cards
 *   Supported IEC958 input from optional MI/ODI/O add-on card.
 *
 *   Specifics (SW, HW):
 *   -------------------
 *   	* 49.5MHz crystal
 *   	* SPDIF-OUT on the card:
 *  	  - coax (through isolation transformer)/toslink supplied by
 *          74HC04 gates - 3 in parallel
 *   	  - output switched between on-board CD drive dig-out connector
 *          and ice1724 SPDTX pin, using 74HC02 NOR gates, controlled
 *          by GPIO20 (0 = CD dig-out, 1 = SPDTX)
 *   	* SPDTX goes straight to MI/ODI/O card's SPDIF-OUT coax
 *
 *   	* MI/ODI/O card: AK4114 based, used for iec958 input only
 *   		- toslink input -> RX0
 *   		- coax input -> RX1
 *   		- 4wire protocol:
 *   			AK4114		ICE1724
 *   			------------------------------
 * 			CDTO (pin 32) -- GPIO11 pin 86
 * 			CDTI (pin 33) -- GPIO10 pin 77
 * 			CCLK (pin 34) -- GPIO9 pin 76
 * 			CSN  (pin 35) -- GPIO8 pin 75
 *   		- output data Mode 7 (24bit, I2S, slave)
 *		- both MCKO1 and MCKO2 of ak4114 are fed to FPGA, which
 *		  outputs master clock to SPMCLKIN of ice1724.
 *		  Experimentally I found out that only a combination of
 *		  OCKS0=1, OCKS1=1 (128fs, 64fs output) and ice1724 -
 *		  VT1724_MT_I2S_MCLK_128X=0 (256fs input) yields correct
 *		  sampling rate. That means that the FPGA doubles the
 *		  MCK01 rate.
 *
 *	Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
 *      Copyright (c) 2003 Dimitromanolakis Apostolos <apostol@cs.utoronto.ca>
 *      Copyright (c) 2004 Kouichi ONO <co2b@ceres.dti.ne.jp>
 */

/* Dependencies originally included from linux/delay.h, linux/interrupt.h,
 * linux/init.h, linux/slab.h, sound/core.h, ice1712.h, envy24ht.h,
 * prodigy192.h, stac946x.h, and sound/tlv.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ushort, c_void};
use core::ptr;

#[repr(C)]
pub struct prodigy192_spec {
    pub ak4114: *mut ak4114,
    /* rate change needs atomic mute/unmute of all dacs*/
    pub mute_mutex: mutex,
}

extern "C" {
    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, addr: c_uint, reg: c_int, val: c_uchar);
    fn snd_vt1724_read_i2c(ice: *mut snd_ice1712, addr: c_uint, reg: c_int) -> c_uchar;
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
    fn udelay(usecs: c_uint);
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, val: c_uint);
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> c_uint;
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712);
    fn snd_ak4114_create(
        card: *mut snd_card,
        read: Option<unsafe extern "C" fn(*mut c_void, c_uchar) -> c_uchar>,
        write: Option<unsafe extern "C" fn(*mut c_void, c_uchar, c_uchar)>,
        init_vals: *const c_uchar,
        init_txcsb: *const c_uchar,
        private_data: *mut c_void,
        r_ak4114: *mut *mut ak4114,
    ) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut snd_ice1712,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut snd_ice1712) -> *mut snd_kcontrol;
    fn snd_ak4114_build(ak4114: *mut ak4114, kcontrol: *mut c_void, substream: *mut snd_pcm_substream) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

#[inline]
unsafe fn stac9460_put(ice: *mut snd_ice1712, reg: c_int, val: c_uchar) {
    snd_vt1724_write_i2c(ice, PRODIGY192_STAC9460_ADDR, reg, val);
}

#[inline]
unsafe fn stac9460_get(ice: *mut snd_ice1712, reg: c_int) -> c_uchar {
    snd_vt1724_read_i2c(ice, PRODIGY192_STAC9460_ADDR, reg)
}

/*
 * DAC mute control
 */

/*
 * idx = STAC9460 volume register number, mute: 0 = mute, 1 = unmute
 */
unsafe fn stac9460_dac_mute(ice: *mut snd_ice1712, idx: c_int, mute: c_uchar) -> c_int {
    let old: c_uchar = stac9460_get(ice, idx);
    let new: c_uchar = (((!mute) << 7) & 0x80) | (old & !0x80u8);
    let change: c_int = (new != old) as c_int;
    if change != 0 {
        /* dev_dbg(ice->card->dev, "Volume register 0x%02x: 0x%02x\n", idx, new);*/
        stac9460_put(ice, idx, new);
    }
    change
}

const stac9460_dac_mute_info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int =
    snd_ctl_boolean_mono_info;

unsafe extern "C" fn stac9460_dac_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let idx: c_int;

    if (*kcontrol).private_value != 0 {
        idx = STAC946X_MASTER_VOLUME;
    } else {
        idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) + STAC946X_LF_VOLUME;
    }
    let val: c_uchar = stac9460_get(ice, idx);
    (*ucontrol).value.integer.value[0] = (((!val) >> 7) & 0x1) as c_long;
    0
}

unsafe extern "C" fn stac9460_dac_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut prodigy192_spec = (*ice).spec as *mut prodigy192_spec;
    let idx: c_int;

    if (*kcontrol).private_value != 0 {
        idx = STAC946X_MASTER_VOLUME;
    } else {
        idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) + STAC946X_LF_VOLUME;
    }
    /* due to possible conflicts with stac9460_set_rate_val, mutexing */
    mutex_lock(&mut (*spec).mute_mutex);
    /*
    dev_dbg(ice->card->dev, "Mute put: reg 0x%02x, ctrl value: 0x%02x\n", idx,
           ucontrol->value.integer.value[0]);
    */
    let ret = stac9460_dac_mute(ice, idx, (*ucontrol).value.integer.value[0] as c_uchar);
    mutex_unlock(&mut (*spec).mute_mutex);
    ret
}

/*
 * DAC volume attenuation mixer control
 */
unsafe extern "C" fn stac9460_dac_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;        /* mute */
    (*uinfo).value.integer.max = 0x7f;     /* 0dB */
    0
}

unsafe extern "C" fn stac9460_dac_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let idx: c_int;

    if (*kcontrol).private_value != 0 {
        idx = STAC946X_MASTER_VOLUME;
    } else {
        idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) + STAC946X_LF_VOLUME;
    }
    let vol: c_uchar = stac9460_get(ice, idx) & 0x7f;
    (*ucontrol).value.integer.value[0] = (0x7f - vol) as c_long;

    0
}

unsafe extern "C" fn stac9460_dac_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let idx: c_int;

    if (*kcontrol).private_value != 0 {
        idx = STAC946X_MASTER_VOLUME;
    } else {
        idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id) + STAC946X_LF_VOLUME;
    }
    let nvol: c_uchar = (*ucontrol).value.integer.value[0] as c_uchar;
    let tmp: c_uchar = stac9460_get(ice, idx);
    let ovol: c_uchar = 0x7f - (tmp & 0x7f);
    let change: c_int = (ovol != nvol) as c_int;
    if change != 0 {
        stac9460_put(ice, idx, (0x7f - nvol) | (tmp & 0x80));
    }

    change
}

/*
 * ADC mute control
 */
const stac9460_adc_mute_info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int =
    snd_ctl_boolean_stereo_info;

unsafe extern "C" fn stac9460_adc_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    for i in 0..2 {
        let val: c_uchar = stac9460_get(ice, STAC946X_MIC_L_VOLUME + i);
        (*ucontrol).value.integer.value[i as usize] = (((!val) >> 7) & 0x1) as c_long;
    }

    0
}

unsafe extern "C" fn stac9460_adc_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let mut change: c_int = 0;

    for i in 0..2 {
        let reg: c_int = STAC946X_MIC_L_VOLUME + i;
        let old: c_uchar = stac9460_get(ice, reg);
        let new: c_uchar =
            (((!((*ucontrol).value.integer.value[i as usize] as c_uchar)) << 7) & 0x80)
                | (old & !0x80u8);
        change = (new != old) as c_int;
        if change != 0 {
            stac9460_put(ice, reg, new);
        }
    }

    change
}

/*
 * ADC gain mixer control
 */
unsafe extern "C" fn stac9460_adc_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;        /* 0dB */
    (*uinfo).value.integer.max = 0x0f;     /* 22.5dB */
    0
}

unsafe extern "C" fn stac9460_adc_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    for i in 0..2 {
        let reg: c_int = STAC946X_MIC_L_VOLUME + i;
        let vol: c_uchar = stac9460_get(ice, reg) & 0x0f;
        (*ucontrol).value.integer.value[i as usize] = (0x0f - vol) as c_long;
    }

    0
}

unsafe extern "C" fn stac9460_adc_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let mut change: c_int = 0;

    for i in 0..2 {
        let reg: c_int = STAC946X_MIC_L_VOLUME + i;
        let nvol: c_uchar = ((*ucontrol).value.integer.value[i as usize] as c_uchar) & 0x0f;
        let ovol: c_uchar = 0x0f - stac9460_get(ice, reg);
        change = (((ovol & 0x0f) != nvol) as c_int);
        if change != 0 {
            stac9460_put(ice, reg, (0x0f - nvol) | (ovol & !0x0fu8));
        }
    }

    change
}

unsafe extern "C" fn stac9460_mic_sw_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT0: &[u8] = b"Line In\0";
    static TEXT1: &[u8] = b"Mic\0";
    let texts: [*const c_char; 2] = [TEXT0.as_ptr() as *const c_char, TEXT1.as_ptr() as *const c_char];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn stac9460_mic_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    let val: c_uchar = stac9460_get(ice, STAC946X_GENERAL_PURPOSE);
    (*ucontrol).value.enumerated.item[0] = ((val >> 7) & 0x1) as c_uint;
    0
}

unsafe extern "C" fn stac9460_mic_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let old: c_uchar = stac9460_get(ice, STAC946X_GENERAL_PURPOSE);
    let new: c_uchar =
        (((*ucontrol).value.enumerated.item[0] as c_uchar) << 7 & 0x80) | (old & !0x80u8);
    let change: c_int = (new != old) as c_int;
    if change != 0 {
        stac9460_put(ice, STAC946X_GENERAL_PURPOSE, new);
    }
    change
}

/*
 * Handler for setting correct codec rate - called when rate change is detected
 */
unsafe extern "C" fn stac9460_set_rate_val(ice: *mut snd_ice1712, rate: c_uint) {
    let new: c_uchar;
    let spec: *mut prodigy192_spec = (*ice).spec as *mut prodigy192_spec;

    if rate == 0 {
        /* no hint - S/PDIF input is master, simply return */
        return;
    } else if rate <= 48000 {
        new = 0x08; /* 256x, base rate mode */
    } else if rate <= 96000 {
        new = 0x11; /* 256x, mid rate mode */
    } else {
        new = 0x12; /* 128x, high rate mode */
    }
    let old: c_uchar = stac9460_get(ice, STAC946X_MASTER_CLOCKING);
    if old == new {
        return;
    }
    /* change detected, setting master clock, muting first */
    /* due to possible conflicts with mute controls - mutexing */
    mutex_lock(&mut (*spec).mute_mutex);
    /* we have to remember current mute status for each DAC */
    let mut changed: [c_uchar; 7] = [0; 7];
    for idx in 0..7 {
        changed[idx as usize] =
            stac9460_dac_mute(ice, STAC946X_MASTER_VOLUME + idx, 0) as c_uchar;
    }
    /*dev_dbg(ice->card->dev, "Rate change: %d, new MC: 0x%02x\n", rate, new);*/
    stac9460_put(ice, STAC946X_MASTER_CLOCKING, new);
    udelay(10);
    /* unmuting - only originally unmuted dacs -
     * i.e. those changed when muting */
    for idx in 0..7 {
        if changed[idx as usize] != 0 {
            stac9460_dac_mute(ice, STAC946X_MASTER_VOLUME + idx, 1);
        }
    }
    mutex_unlock(&mut (*spec).mute_mutex);
}

static_db_scale!(db_scale_dac, -19125, 75, 0);
static_db_scale!(db_scale_adc, 0, 150, 0);

/*
 * mixers
 */

static stac_controls: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Master Playback Switch\0".as_ptr() as *const c_char,
        info: Some(stac9460_dac_mute_info),
        get: Some(stac9460_dac_mute_get),
        put: Some(stac9460_dac_mute_put),
        private_value: 1,
        tlv: snd_kcontrol_tlv { p: db_scale_dac.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: b"Master Playback Volume\0".as_ptr() as *const c_char,
        info: Some(stac9460_dac_vol_info),
        get: Some(stac9460_dac_vol_get),
        put: Some(stac9460_dac_vol_put),
        private_value: 1,
        tlv: snd_kcontrol_tlv { p: db_scale_dac.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"DAC Switch\0".as_ptr() as *const c_char,
        count: 6,
        info: Some(stac9460_dac_mute_info),
        get: Some(stac9460_dac_mute_get),
        put: Some(stac9460_dac_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: b"DAC Volume\0".as_ptr() as *const c_char,
        count: 6,
        info: Some(stac9460_dac_vol_info),
        get: Some(stac9460_dac_vol_get),
        put: Some(stac9460_dac_vol_put),
        tlv: snd_kcontrol_tlv { p: db_scale_dac.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"ADC Capture Switch\0".as_ptr() as *const c_char,
        count: 1,
        info: Some(stac9460_adc_mute_info),
        get: Some(stac9460_adc_mute_get),
        put: Some(stac9460_adc_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: b"ADC Capture Volume\0".as_ptr() as *const c_char,
        count: 1,
        info: Some(stac9460_adc_vol_info),
        get: Some(stac9460_adc_vol_get),
        put: Some(stac9460_adc_vol_put),
        tlv: snd_kcontrol_tlv { p: db_scale_adc.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Capture Input\0".as_ptr() as *const c_char,
        info: Some(stac9460_mic_sw_info),
        get: Some(stac9460_mic_sw_get),
        put: Some(stac9460_mic_sw_put),
        ..unsafe { core::mem::zeroed() }
    },
];

/* AK4114 - ICE1724 connections on Prodigy192 + MI/ODI/O */
/* CDTO (pin 32) -- GPIO11 pin 86
 * CDTI (pin 33) -- GPIO10 pin 77
 * CCLK (pin 34) -- GPIO9 pin 76
 * CSN  (pin 35) -- GPIO8 pin 75
 */
const AK4114_ADDR: c_uint = 0x00; /* C1-C0: Chip Address
                                   * (According to datasheet fixed to "00")
                                   */

/*
 * 4wire ak4114 protocol - writing data
 */
unsafe fn write_data(ice: *mut snd_ice1712, mut gpio: c_uint, data: c_uint, mut idx: c_int) {
    while idx >= 0 {
        /* drop clock */
        gpio &= !VT1724_PRODIGY192_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* set data */
        if (data & (1u32 << idx)) != 0 {
            gpio |= VT1724_PRODIGY192_CDOUT;
        } else {
            gpio &= !VT1724_PRODIGY192_CDOUT;
        }
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* raise clock */
        gpio |= VT1724_PRODIGY192_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        idx -= 1;
    }
}

/*
 * 4wire ak4114 protocol - reading data
 */
unsafe fn read_data(ice: *mut snd_ice1712, mut gpio: c_uint, mut idx: c_int) -> c_uchar {
    let mut data: c_uchar = 0;

    while idx >= 0 {
        /* drop clock */
        gpio &= !VT1724_PRODIGY192_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* read data */
        if (snd_ice1712_gpio_read(ice) & VT1724_PRODIGY192_CDIN) != 0 {
            data |= (1u8 << idx) as c_uchar;
        }
        udelay(1);
        /* raise clock */
        gpio |= VT1724_PRODIGY192_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        idx -= 1;
    }
    data
}

/*
 * 4wire ak4114 protocol - starting sequence
 */
unsafe fn prodigy192_4wire_start(ice: *mut snd_ice1712) -> c_uint {
    snd_ice1712_save_gpio_status(ice);
    let mut tmp: c_uint = snd_ice1712_gpio_read(ice);

    tmp |= VT1724_PRODIGY192_CCLK; /* high at init */
    tmp &= !VT1724_PRODIGY192_CS; /* drop chip select */
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp
}

/*
 * 4wire ak4114 protocol - final sequence
 */
unsafe fn prodigy192_4wire_finish(ice: *mut snd_ice1712, mut tmp: c_uint) {
    tmp |= VT1724_PRODIGY192_CS; /* raise chip select */
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    snd_ice1712_restore_gpio_status(ice);
}

/*
 * Write data to addr register of ak4114
 */
unsafe extern "C" fn prodigy192_ak4114_write(
    private_data: *mut c_void,
    addr: c_uchar,
    data: c_uchar,
) {
    let ice: *mut snd_ice1712 = private_data as *mut snd_ice1712;
    let tmp: c_uint = prodigy192_4wire_start(ice);
    let mut addrdata: c_uint = (AK4114_ADDR << 6) | 0x20 | ((addr & 0x1f) as c_uint);
    addrdata = (addrdata << 8) | data as c_uint;
    write_data(ice, tmp, addrdata, 15);
    prodigy192_4wire_finish(ice, tmp);
}

/*
 * Read data from addr register of ak4114
 */
unsafe extern "C" fn prodigy192_ak4114_read(private_data: *mut c_void, addr: c_uchar) -> c_uchar {
    let ice: *mut snd_ice1712 = private_data as *mut snd_ice1712;

    let tmp: c_uint = prodigy192_4wire_start(ice);
    write_data(ice, tmp, (AK4114_ADDR << 6) | ((addr & 0x1f) as c_uint), 7);
    let data: c_uchar = read_data(ice, tmp, 7);
    prodigy192_4wire_finish(ice, tmp);
    data
}

unsafe extern "C" fn ak4114_input_sw_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT0: &[u8] = b"Toslink\0";
    static TEXT1: &[u8] = b"Coax\0";
    let texts: [*const c_char; 2] = [TEXT0.as_ptr() as *const c_char, TEXT1.as_ptr() as *const c_char];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn ak4114_input_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    let val: c_uchar = prodigy192_ak4114_read(ice as *mut c_void, AK4114_REG_IO1);
    /* AK4114_IPS0 bit = 0 -> RX0 = Toslink
     * AK4114_IPS0 bit = 1 -> RX1 = Coax
     */
    (*ucontrol).value.enumerated.item[0] = if (val & AK4114_IPS0) != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn ak4114_input_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    let old: c_uchar = prodigy192_ak4114_read(ice as *mut c_void, AK4114_REG_IO1);
    /* AK4114_IPS0 could be any bit */
    let itemvalue: c_uchar = if (*ucontrol).value.enumerated.item[0] != 0 { 0xff } else { 0x00 };

    let new: c_uchar = (itemvalue & AK4114_IPS0) | (old & !AK4114_IPS0);
    let change: c_int = (new != old) as c_int;
    if change != 0 {
        prodigy192_ak4114_write(ice as *mut c_void, AK4114_REG_IO1, new);
    }
    change
}

static ak4114_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"MIODIO IEC958 Capture Input\0".as_ptr() as *const c_char,
    info: Some(ak4114_input_sw_info),
    get: Some(ak4114_input_sw_get),
    put: Some(ak4114_input_sw_put),
    ..unsafe { core::mem::zeroed() }
}];

unsafe fn prodigy192_ak4114_init(ice: *mut snd_ice1712) -> c_int {
    static ak4114_init_vals: [c_uchar; 6] = [
        AK4114_RST | AK4114_PWN | AK4114_OCKS0 | AK4114_OCKS1,
        /* ice1724 expects I2S and provides clock,
         * DEM0 disables the deemphasis filter
         */
        AK4114_DIF_I24I2S | AK4114_DEM0,
        AK4114_TX1E,
        AK4114_EFH_1024 | AK4114_DIT, /* default input RX0 */
        0,
        0,
    ];
    static ak4114_init_txcsb: [c_uchar; 5] = [0x41, 0x02, 0x2c, 0x00, 0x00];
    let spec: *mut prodigy192_spec = (*ice).spec as *mut prodigy192_spec;

    let err: c_int = snd_ak4114_create(
        (*ice).card,
        Some(prodigy192_ak4114_read),
        Some(prodigy192_ak4114_write),
        ak4114_init_vals.as_ptr(),
        ak4114_init_txcsb.as_ptr(),
        ice as *mut c_void,
        &mut (*spec).ak4114,
    );
    if err < 0 {
        return err;
    }
    /* AK4114 in Prodigy192 cannot detect external rate correctly.
     * No reason to stop capture stream due to incorrect checks */
    (*(*spec).ak4114).check_flags = AK4114_CHECK_NO_RATE;
    0
}

unsafe extern "C" fn stac9460_proc_regs_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let ice: *mut snd_ice1712 = (*entry).private_data as *mut snd_ice1712;
    /* registers 0x0 - 0x14 */
    for reg in 0..=0x15 {
        let val: c_int = stac9460_get(ice, reg) as c_int;
        snd_iprintf(buffer, b"0x%02x = 0x%02x\n\0".as_ptr() as *const c_char, reg, val);
    }
}

unsafe fn stac9460_proc_init(ice: *mut snd_ice1712) {
    snd_card_ro_proc_new(
        (*ice).card,
        b"stac9460_codec\0".as_ptr() as *const c_char,
        ice,
        Some(stac9460_proc_regs_read),
    );
}

unsafe extern "C" fn prodigy192_add_controls(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut prodigy192_spec = (*ice).spec as *mut prodigy192_spec;

    for i in 0..stac_controls.len() {
        let err: c_int = snd_ctl_add((*ice).card, snd_ctl_new1(&stac_controls[i], ice));
        if err < 0 {
            return err;
        }
    }
    if !(*spec).ak4114.is_null() {
        /* ak4114 is connected */
        for i in 0..ak4114_controls.len() {
            let err: c_int = snd_ctl_add((*ice).card, snd_ctl_new1(&ak4114_controls[i], ice));
            if err < 0 {
                return err;
            }
        }
        let err: c_int = snd_ak4114_build(
            (*spec).ak4114,
            ptr::null_mut(), /* ak4114 in MIO/DI/O handles no IEC958 output */
            (*(*ice).pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream,
        );
        if err < 0 {
            return err;
        }
    }
    stac9460_proc_init(ice);
    0
}

/*
 * check for presence of MI/ODI/O add-on card with digital inputs
 */
unsafe fn prodigy192_miodio_exists(ice: *mut snd_ice1712) -> c_int {
    let test_data: c_uchar = 0xd1; /* random value */
    let addr: c_uchar = AK4114_REG_INT0_MASK; /* random SAFE address */
    let mut exists: c_int = 0;

    let orig_value: c_uchar = prodigy192_ak4114_read(ice as *mut c_void, addr);
    prodigy192_ak4114_write(ice as *mut c_void, addr, test_data);
    if prodigy192_ak4114_read(ice as *mut c_void, addr) == test_data {
        /* ak4114 seems to communicate, apparently exists */
        /* writing back original value */
        prodigy192_ak4114_write(ice as *mut c_void, addr, orig_value);
        exists = 1;
    }
    exists
}

/*
 * initialize the chip
 */
unsafe extern "C" fn prodigy192_init(ice: *mut snd_ice1712) -> c_int {
    static stac_inits_prodigy: [c_ushort; 5] = [
        STAC946X_RESET as c_ushort,
        0,
        STAC946X_MASTER_CLOCKING as c_ushort,
        0x11,
        (-1i16) as c_ushort,
    ];
    /*
        STAC946X_MASTER_VOLUME, 0,
        STAC946X_LF_VOLUME, 0,
        STAC946X_RF_VOLUME, 0,
        STAC946X_LR_VOLUME, 0,
        STAC946X_RR_VOLUME, 0,
        STAC946X_CENTER_VOLUME, 0,
        STAC946X_LFE_VOLUME, 0,
    */
    let mut err: c_int = 0;

    /* prodigy 192 */
    (*ice).num_total_dacs = 6;
    (*ice).num_total_adcs = 2;
    (*ice).vt1720 = 0; /* ice1724, e.g. 23 GPIOs */

    let spec: *mut prodigy192_spec =
        kzalloc(core::mem::size_of::<prodigy192_spec>(), GFP_KERNEL) as *mut prodigy192_spec;
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut c_void;
    mutex_init(&mut (*spec).mute_mutex);

    /* initialize codec */
    let mut p = stac_inits_prodigy.as_ptr();
    while *p != (-1i16) as c_ushort {
        stac9460_put(ice, *p as c_int, *p.add(1) as c_uchar);
        p = p.add(2);
    }
    (*ice).gpio.set_pro_rate = Some(stac9460_set_rate_val);

    /* MI/ODI/O add on card with AK4114 */
    if prodigy192_miodio_exists(ice) != 0 {
        err = prodigy192_ak4114_init(ice);
        /* from this moment if err = 0 then
         * spec->ak4114 should not be null
         */
        dev_dbg(
            (*(*ice).card).dev,
            b"AK4114 initialized with status %d\n\0".as_ptr() as *const c_char,
            err,
        );
    } else {
        dev_dbg(
            (*(*ice).card).dev,
            b"AK4114 not found\n\0".as_ptr() as *const c_char,
        );
    }

    err
}

/*
 * Aureon boards don't provide the EEPROM data except for the vendor IDs.
 * hence the driver needs to sets up it properly.
 */

static prodigy71_eeprom: [c_uchar; 14] = {
    let mut a = [0u8; 14];
    a[ICE_EEP2_SYSCONF as usize] = 0x6a; /* 49MHz crystal, mpu401,
                                           * spdif-in+ 1 stereo ADC,
                                           * 3 stereo DACs
                                           */
    a[ICE_EEP2_ACLINK as usize] = 0x80; /* I2S */
    a[ICE_EEP2_I2S as usize] = 0xf8; /* vol, 96k, 24bit, 192k */
    a[ICE_EEP2_SPDIF as usize] = 0xc3; /* out-en, out-int, spdif-in */
    a[ICE_EEP2_GPIO_DIR as usize] = 0xff;
    a[ICE_EEP2_GPIO_DIR1 as usize] = (!(VT1724_PRODIGY192_CDIN >> 8)) as c_uchar;
    a[ICE_EEP2_GPIO_DIR2 as usize] = 0xbf;
    a[ICE_EEP2_GPIO_MASK as usize] = 0x00;
    a[ICE_EEP2_GPIO_MASK1 as usize] = 0x00;
    a[ICE_EEP2_GPIO_MASK2 as usize] = 0x00;
    a[ICE_EEP2_GPIO_STATE as usize] = 0x00;
    a[ICE_EEP2_GPIO_STATE1 as usize] = 0x00;
    a[ICE_EEP2_GPIO_STATE2 as usize] = 0x10; /* GPIO20: 0 = CD drive dig. input
                                               * passthrough,
                                               * 1 = SPDIF-OUT from ice1724
                                               */
    a
};

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_prodigy192_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY192VE,
        name: b"Audiotrak Prodigy 192\0".as_ptr() as *const c_char,
        model: b"prodigy192\0".as_ptr() as *const c_char,
        chip_init: Some(prodigy192_init),
        build_controls: Some(prodigy192_add_controls),
        eeprom_size: core::mem::size_of_val(&prodigy71_eeprom),
        eeprom_data: prodigy71_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        ..unsafe { core::mem::zeroed() }
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
