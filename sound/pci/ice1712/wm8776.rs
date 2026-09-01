// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT17xx
 *
 *   Lowlevel functions for WM8776 codec
 *
 *	Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
 */

/* Dependencies from the original C includes:
 * <linux/delay.h>, <sound/core.h>, <sound/control.h>, <sound/tlv.h>,
 * and "wm8776.h".
 */

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

/* low-level access */

unsafe fn snd_wm8776_write(wm: *mut snd_wm8776, addr: u16, data: u16) {
    let bus_addr: u8 = ((addr << 1) | (data >> 8)) as u8; /* addr + 9th data bit */
    let bus_data: u8 = (data & 0xff) as u8;              /* remaining 8 data bits */

    if addr < WM8776_REG_RESET {
        (*wm).regs[addr as usize] = data;
    }
    ((*wm).ops.write)(wm, bus_addr, bus_data);
}

/* register-level functions */

unsafe fn snd_wm8776_activate_ctl(
    wm: *mut snd_wm8776,
    ctl_name: *const c_char,
    active: bool,
) {
    let card: *mut snd_card = (*wm).card;
    let kctl: *mut snd_kcontrol;
    let vd: *mut snd_kcontrol_volatile;
    let index_offset: c_uint;

    kctl = snd_ctl_find_id_mixer(card, ctl_name);
    if kctl.is_null() {
        return;
    }
    index_offset = snd_ctl_get_ioff(kctl, &mut (*kctl).id);
    vd = &mut *(*kctl).vd.add(index_offset as usize);
    if active {
        (*vd).access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    } else {
        (*vd).access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }
    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*kctl).id);
}

unsafe fn snd_wm8776_update_agc_ctl(wm: *mut snd_wm8776) {
    let mut flags_on: c_int = 0;
    let mut flags_off: c_int = 0;

    match (*wm).agc_mode {
        WM8776_AGC_OFF => {
            flags_off = WM8776_FLAG_LIM | WM8776_FLAG_ALC;
        }
        WM8776_AGC_LIM => {
            flags_off = WM8776_FLAG_ALC;
            flags_on = WM8776_FLAG_LIM;
        }
        WM8776_AGC_ALC_R | WM8776_AGC_ALC_L | WM8776_AGC_ALC_STEREO => {
            flags_off = WM8776_FLAG_LIM;
            flags_on = WM8776_FLAG_ALC;
        }
        _ => {}
    }

    for i in 0..WM8776_CTL_COUNT {
        if (*wm).ctl[i as usize].flags & flags_off != 0 {
            snd_wm8776_activate_ctl(wm, (*wm).ctl[i as usize].name, false);
        } else if (*wm).ctl[i as usize].flags & flags_on != 0 {
            snd_wm8776_activate_ctl(wm, (*wm).ctl[i as usize].name, true);
        }
    }
}

unsafe fn snd_wm8776_set_agc(wm: *mut snd_wm8776, agc: u16, _nothing: u16) {
    let mut alc1: u16 = (*wm).regs[WM8776_REG_ALCCTRL1 as usize] & !WM8776_ALC1_LCT_MASK;
    let mut alc2: u16 = (*wm).regs[WM8776_REG_ALCCTRL2 as usize] & !WM8776_ALC2_LCEN;

    match agc {
        0 => { /* Off */
            (*wm).agc_mode = WM8776_AGC_OFF;
        }
        1 => { /* Limiter */
            alc2 |= WM8776_ALC2_LCEN;
            (*wm).agc_mode = WM8776_AGC_LIM;
        }
        2 => { /* ALC Right */
            alc1 |= WM8776_ALC1_LCSEL_ALCR;
            alc2 |= WM8776_ALC2_LCEN;
            (*wm).agc_mode = WM8776_AGC_ALC_R;
        }
        3 => { /* ALC Left */
            alc1 |= WM8776_ALC1_LCSEL_ALCL;
            alc2 |= WM8776_ALC2_LCEN;
            (*wm).agc_mode = WM8776_AGC_ALC_L;
        }
        4 => { /* ALC Stereo */
            alc1 |= WM8776_ALC1_LCSEL_ALCSTEREO;
            alc2 |= WM8776_ALC2_LCEN;
            (*wm).agc_mode = WM8776_AGC_ALC_STEREO;
        }
        _ => {}
    }
    snd_wm8776_write(wm, WM8776_REG_ALCCTRL1, alc1);
    snd_wm8776_write(wm, WM8776_REG_ALCCTRL2, alc2);
    snd_wm8776_update_agc_ctl(wm);
}

unsafe fn snd_wm8776_get_agc(wm: *mut snd_wm8776, mode: *mut u16, _nothing: *mut u16) {
    *mode = (*wm).agc_mode;
}

/* mixer controls */

static WM8776_HP_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-7400, 100, 1);
static WM8776_DAC_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-12750, 50, 1);
static WM8776_ADC_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-10350, 50, 1);
static WM8776_LCT_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1600, 100, 0);
static WM8776_MAXGAIN_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(0, 400, 0);
static WM8776_NGTH_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-7800, 600, 0);
static WM8776_MAXATTEN_LIM_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1200, 100, 0);
static WM8776_MAXATTEN_ALC_TLV: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-2100, 400, 0);

static SND_WM8776_DEFAULT_CTL: [snd_wm8776_ctl; WM8776_CTL_COUNT as usize] = [
    snd_wm8776_ctl { name: b"Master Playback Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_DAC_TLV.as_ptr(), reg1: WM8776_REG_DACLVOL, reg2: WM8776_REG_DACRVOL, mask1: WM8776_DACVOL_MASK, mask2: WM8776_DACVOL_MASK, max: 0xff, flags: WM8776_FLAG_STEREO | WM8776_FLAG_VOL_UPDATE, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Master Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_DACCTRL1, reg2: WM8776_REG_DACCTRL1, mask1: WM8776_DAC_PL_LL, mask2: WM8776_DAC_PL_RR, flags: WM8776_FLAG_STEREO, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Master Zero Cross Detect Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_DACCTRL1, mask1: WM8776_DAC_DZCEN, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Headphone Playback Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_HP_TLV.as_ptr(), reg1: WM8776_REG_HPLVOL, reg2: WM8776_REG_HPRVOL, mask1: WM8776_HPVOL_MASK, mask2: WM8776_HPVOL_MASK, min: 0x2f, max: 0x7f, flags: WM8776_FLAG_STEREO | WM8776_FLAG_VOL_UPDATE, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Headphone Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_PWRDOWN, mask1: WM8776_PWR_HPPD, flags: WM8776_FLAG_INVERT, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Headphone Zero Cross Detect Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_HPLVOL, reg2: WM8776_REG_HPRVOL, mask1: WM8776_VOL_HPZCEN, mask2: WM8776_VOL_HPZCEN, flags: WM8776_FLAG_STEREO, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AUX Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_OUTMUX, mask1: WM8776_OUTMUX_AUX, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Bypass Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_OUTMUX, mask1: WM8776_OUTMUX_BYPASS, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Infinite Zero Detect Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_DACCTRL1, mask1: WM8776_DAC_IZD, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Phase Invert Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_PHASESWAP, reg2: WM8776_REG_PHASESWAP, mask1: WM8776_PHASE_INVERTL, mask2: WM8776_PHASE_INVERTR, flags: WM8776_FLAG_STEREO, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Deemphasis Playback Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_DACCTRL2, mask1: WM8776_DAC2_DEEMPH, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Input Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_ADC_TLV.as_ptr(), reg1: WM8776_REG_ADCLVOL, reg2: WM8776_REG_ADCRVOL, mask1: WM8776_ADC_GAIN_MASK, mask2: WM8776_ADC_GAIN_MASK, max: 0xff, flags: WM8776_FLAG_STEREO | WM8776_FLAG_VOL_UPDATE, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Input Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, reg2: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUTEL, mask2: WM8776_ADC_MUTER, flags: WM8776_FLAG_STEREO | WM8776_FLAG_INVERT, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AIN1 Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUX_AIN1, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AIN2 Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUX_AIN2, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AIN3 Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUX_AIN3, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AIN4 Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUX_AIN4, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AIN5 Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_ADCMUX, mask1: WM8776_ADC_MUX_AIN5, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"AGC Select Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"Off\0".as_ptr() as *const c_char, b"Limiter\0".as_ptr() as *const c_char, b"ALC Right\0".as_ptr() as *const c_char, b"ALC Left\0".as_ptr() as *const c_char, b"ALC Stereo\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 5, set: Some(snd_wm8776_set_agc), get: Some(snd_wm8776_get_agc), ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Limiter Threshold Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_LCT_TLV.as_ptr(), reg1: WM8776_REG_ALCCTRL1, mask1: WM8776_ALC1_LCT_MASK, max: 15, flags: WM8776_FLAG_LIM, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Limiter Attack Time Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"0.25 ms\0".as_ptr() as *const c_char, b"0.5 ms\0".as_ptr() as *const c_char, b"1 ms\0".as_ptr() as *const c_char, b"2 ms\0".as_ptr() as *const c_char, b"4 ms\0".as_ptr() as *const c_char, b"8 ms\0".as_ptr() as *const c_char, b"16 ms\0".as_ptr() as *const c_char, b"32 ms\0".as_ptr() as *const c_char, b"64 ms\0".as_ptr() as *const c_char, b"128 ms\0".as_ptr() as *const c_char, b"256 ms\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 11, reg1: WM8776_REG_ALCCTRL3, mask1: WM8776_ALC3_ATK_MASK, flags: WM8776_FLAG_LIM, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Limiter Decay Time Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"1.2 ms\0".as_ptr() as *const c_char, b"2.4 ms\0".as_ptr() as *const c_char, b"4.8 ms\0".as_ptr() as *const c_char, b"9.6 ms\0".as_ptr() as *const c_char, b"19.2 ms\0".as_ptr() as *const c_char, b"38.4 ms\0".as_ptr() as *const c_char, b"76.8 ms\0".as_ptr() as *const c_char, b"154 ms\0".as_ptr() as *const c_char, b"307 ms\0".as_ptr() as *const c_char, b"614 ms\0".as_ptr() as *const c_char, b"1.23 s\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 11, reg1: WM8776_REG_ALCCTRL3, mask1: WM8776_ALC3_DCY_MASK, flags: WM8776_FLAG_LIM, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Limiter Transient Window Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"0 us\0".as_ptr() as *const c_char, b"62.5 us\0".as_ptr() as *const c_char, b"125 us\0".as_ptr() as *const c_char, b"250 us\0".as_ptr() as *const c_char, b"500 us\0".as_ptr() as *const c_char, b"1 ms\0".as_ptr() as *const c_char, b"2 ms\0".as_ptr() as *const c_char, b"4 ms\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 8, reg1: WM8776_REG_LIMITER, mask1: WM8776_LIM_TRANWIN_MASK, flags: WM8776_FLAG_LIM, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Limiter Maximum Attenuation Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_MAXATTEN_LIM_TLV.as_ptr(), reg1: WM8776_REG_LIMITER, mask1: WM8776_LIM_MAXATTEN_MASK, min: 3, max: 12, flags: WM8776_FLAG_LIM | WM8776_FLAG_INVERT, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Target Level Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_LCT_TLV.as_ptr(), reg1: WM8776_REG_ALCCTRL1, mask1: WM8776_ALC1_LCT_MASK, max: 15, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Attack Time Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"8.40 ms\0".as_ptr() as *const c_char, b"16.8 ms\0".as_ptr() as *const c_char, b"33.6 ms\0".as_ptr() as *const c_char, b"67.2 ms\0".as_ptr() as *const c_char, b"134 ms\0".as_ptr() as *const c_char, b"269 ms\0".as_ptr() as *const c_char, b"538 ms\0".as_ptr() as *const c_char, b"1.08 s\0".as_ptr() as *const c_char, b"2.15 s\0".as_ptr() as *const c_char, b"4.3 s\0".as_ptr() as *const c_char, b"8.6 s\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 11, reg1: WM8776_REG_ALCCTRL3, mask1: WM8776_ALC3_ATK_MASK, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Decay Time Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"33.5 ms\0".as_ptr() as *const c_char, b"67.0 ms\0".as_ptr() as *const c_char, b"134 ms\0".as_ptr() as *const c_char, b"268 ms\0".as_ptr() as *const c_char, b"536 ms\0".as_ptr() as *const c_char, b"1.07 s\0".as_ptr() as *const c_char, b"2.14 s\0".as_ptr() as *const c_char, b"4.29 s\0".as_ptr() as *const c_char, b"8.58 s\0".as_ptr() as *const c_char, b"17.2 s\0".as_ptr() as *const c_char, b"34.3 s\0".as_ptr() as *const c_char, ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()], max: 11, reg1: WM8776_REG_ALCCTRL3, mask1: WM8776_ALC3_DCY_MASK, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Maximum Gain Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_MAXGAIN_TLV.as_ptr(), reg1: WM8776_REG_ALCCTRL1, mask1: WM8776_ALC1_MAXGAIN_MASK, min: 1, max: 7, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Maximum Attenuation Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_MAXATTEN_ALC_TLV.as_ptr(), reg1: WM8776_REG_LIMITER, mask1: WM8776_LIM_MAXATTEN_MASK, min: 10, max: 15, flags: WM8776_FLAG_ALC | WM8776_FLAG_INVERT, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"ALC Hold Time Capture Enum\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_ENUMERATED, enum_names: [b"0 ms\0".as_ptr() as *const c_char, b"2.67 ms\0".as_ptr() as *const c_char, b"5.33 ms\0".as_ptr() as *const c_char, b"10.6 ms\0".as_ptr() as *const c_char, b"21.3 ms\0".as_ptr() as *const c_char, b"42.7 ms\0".as_ptr() as *const c_char, b"85.3 ms\0".as_ptr() as *const c_char, b"171 ms\0".as_ptr() as *const c_char, b"341 ms\0".as_ptr() as *const c_char, b"683 ms\0".as_ptr() as *const c_char, b"1.37 s\0".as_ptr() as *const c_char, b"2.73 s\0".as_ptr() as *const c_char, b"5.46 s\0".as_ptr() as *const c_char, b"10.9 s\0".as_ptr() as *const c_char, b"21.8 s\0".as_ptr() as *const c_char, b"43.7 s\0".as_ptr() as *const c_char], max: 16, reg1: WM8776_REG_ALCCTRL2, mask1: WM8776_ALC2_HOLD_MASK, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Noise Gate Capture Switch\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN, reg1: WM8776_REG_NOISEGATE, mask1: WM8776_NGAT_ENABLE, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
    snd_wm8776_ctl { name: b"Noise Gate Threshold Capture Volume\0".as_ptr() as *const c_char, type_: SNDRV_CTL_ELEM_TYPE_INTEGER, tlv: WM8776_NGTH_TLV.as_ptr(), reg1: WM8776_REG_NOISEGATE, mask1: WM8776_NGAT_THR_MASK, max: 7, flags: WM8776_FLAG_ALC, ..SND_WM8776_CTL_ZERO },
];

/* exported functions */

#[no_mangle]
pub unsafe extern "C" fn snd_wm8776_init(wm: *mut snd_wm8776) {
    static DEFAULT_VALUES: [u16; 23] = [
        0x000, 0x100, 0x000,
        0x000, 0x100, 0x000,
        0x000, 0x090, 0x000, 0x000,
        0x022, 0x022, 0x022,
        0x008, 0x0cf, 0x0cf, 0x07b, 0x000,
        0x032, 0x000, 0x0a6, 0x001, 0x001,
    ];

    memcpy(
        (*wm).ctl.as_mut_ptr() as *mut core::ffi::c_void,
        SND_WM8776_DEFAULT_CTL.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&(*wm).ctl),
    );

    snd_wm8776_write(wm, WM8776_REG_RESET, 0x00); /* reset */
    udelay(10);
    /* load defaults */
    for i in 0..DEFAULT_VALUES.len() {
        snd_wm8776_write(wm, i as u16, DEFAULT_VALUES[i]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wm8776_resume(wm: *mut snd_wm8776) {
    for i in 0..WM8776_REG_COUNT {
        snd_wm8776_write(wm, i as u16, (*wm).regs[i as usize]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wm8776_set_power(wm: *mut snd_wm8776, power: u16) {
    snd_wm8776_write(wm, WM8776_REG_PWRDOWN, power);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wm8776_volume_restore(wm: *mut snd_wm8776) {
    let val: u16 = (*wm).regs[WM8776_REG_DACRVOL as usize];
    /* restore volume after MCLK stopped */
    snd_wm8776_write(wm, WM8776_REG_DACRVOL, val | WM8776_VOL_UPDATE);
}

/* mixer callbacks */

unsafe extern "C" fn snd_wm8776_volume_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let wm: *mut snd_wm8776 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0 { 2 } else { 1 };
    (*uinfo).value.integer.min = (*wm).ctl[n as usize].min as i64;
    (*uinfo).value.integer.max = (*wm).ctl[n as usize].max as i64;

    0
}

unsafe extern "C" fn snd_wm8776_enum_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let wm: *mut snd_wm8776 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    snd_ctl_enum_info(
        uinfo,
        1,
        (*wm).ctl[n as usize].max,
        (*wm).ctl[n as usize].enum_names.as_ptr(),
    )
}

unsafe extern "C" fn snd_wm8776_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let wm: *mut snd_wm8776 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;
    let mut val1: u16 = 0;
    let mut val2: u16 = 0;

    if let Some(get) = (*wm).ctl[n as usize].get {
        get(wm, &mut val1, &mut val2);
    } else {
        val1 = (*wm).regs[(*wm).ctl[n as usize].reg1 as usize] & (*wm).ctl[n as usize].mask1;
        val1 >>= __ffs((*wm).ctl[n as usize].mask1 as c_uint) as u16;
        if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0 {
            val2 = (*wm).regs[(*wm).ctl[n as usize].reg2 as usize] & (*wm).ctl[n as usize].mask2;
            val2 >>= __ffs((*wm).ctl[n as usize].mask2 as c_uint) as u16;
            if (*wm).ctl[n as usize].flags & WM8776_FLAG_VOL_UPDATE != 0 {
                val2 &= !WM8776_VOL_UPDATE;
            }
        }
    }
    if (*wm).ctl[n as usize].flags & WM8776_FLAG_INVERT != 0 {
        val1 = (*wm).ctl[n as usize].max - (val1 - (*wm).ctl[n as usize].min);
        if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0 {
            val2 = (*wm).ctl[n as usize].max - (val2 - (*wm).ctl[n as usize].min);
        }
    }
    (*ucontrol).value.integer.value[0] = val1 as i64;
    if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0 {
        (*ucontrol).value.integer.value[1] = val2 as i64;
    }

    0
}

unsafe extern "C" fn snd_wm8776_ctl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let wm: *mut snd_wm8776 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;
    let mut val: u16;
    let mut regval1: u16;
    let mut regval2: u16;

    /* this also works for enum because value is a union */
    regval1 = (*ucontrol).value.integer.value[0] as u16;
    regval2 = (*ucontrol).value.integer.value[1] as u16;
    if (*wm).ctl[n as usize].flags & WM8776_FLAG_INVERT != 0 {
        regval1 = (*wm).ctl[n as usize].max - (regval1 - (*wm).ctl[n as usize].min);
        regval2 = (*wm).ctl[n as usize].max - (regval2 - (*wm).ctl[n as usize].min);
    }
    if let Some(set) = (*wm).ctl[n as usize].set {
        set(wm, regval1, regval2);
    } else {
        val = (*wm).regs[(*wm).ctl[n as usize].reg1 as usize] & !(*wm).ctl[n as usize].mask1;
        val |= regval1 << __ffs((*wm).ctl[n as usize].mask1 as c_uint);
        /* both stereo controls in one register */
        if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0
            && (*wm).ctl[n as usize].reg1 == (*wm).ctl[n as usize].reg2
        {
            val &= !(*wm).ctl[n as usize].mask2;
            val |= regval2 << __ffs((*wm).ctl[n as usize].mask2 as c_uint);
        }
        snd_wm8776_write(wm, (*wm).ctl[n as usize].reg1, val);
        /* stereo controls in different registers */
        if (*wm).ctl[n as usize].flags & WM8776_FLAG_STEREO != 0
            && (*wm).ctl[n as usize].reg1 != (*wm).ctl[n as usize].reg2
        {
            val = (*wm).regs[(*wm).ctl[n as usize].reg2 as usize] & !(*wm).ctl[n as usize].mask2;
            val |= regval2 << __ffs((*wm).ctl[n as usize].mask2 as c_uint);
            if (*wm).ctl[n as usize].flags & WM8776_FLAG_VOL_UPDATE != 0 {
                val |= WM8776_VOL_UPDATE;
            }
            snd_wm8776_write(wm, (*wm).ctl[n as usize].reg2, val);
        }
    }

    0
}

unsafe fn snd_wm8776_add_control(wm: *mut snd_wm8776, num: c_int) -> c_int {
    let mut cont: snd_kcontrol_new = core::mem::zeroed();
    let ctl: *mut snd_kcontrol;

    cont.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    cont.private_value = num as usize;
    cont.name = (*wm).ctl[num as usize].name;
    cont.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    if (*wm).ctl[num as usize].flags & WM8776_FLAG_LIM != 0
        || (*wm).ctl[num as usize].flags & WM8776_FLAG_ALC != 0
    {
        cont.access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }
    cont.tlv.p = ptr::null();
    cont.get = Some(snd_wm8776_ctl_get);
    cont.put = Some(snd_wm8776_ctl_put);

    match (*wm).ctl[num as usize].type_ {
        SNDRV_CTL_ELEM_TYPE_INTEGER => {
            cont.info = Some(snd_wm8776_volume_info);
            cont.access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
            cont.tlv.p = (*wm).ctl[num as usize].tlv;
        }
        SNDRV_CTL_ELEM_TYPE_BOOLEAN => {
            (*wm).ctl[num as usize].max = 1;
            if (*wm).ctl[num as usize].flags & WM8776_FLAG_STEREO != 0 {
                cont.info = Some(snd_ctl_boolean_stereo_info);
            } else {
                cont.info = Some(snd_ctl_boolean_mono_info);
            }
        }
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
            cont.info = Some(snd_wm8776_enum_info);
        }
        _ => {
            return -EINVAL;
        }
    }
    ctl = snd_ctl_new1(&mut cont, wm as *mut core::ffi::c_void);
    if ctl.is_null() {
        return -ENOMEM;
    }

    snd_ctl_add((*wm).card, ctl)
}

#[no_mangle]
pub unsafe extern "C" fn snd_wm8776_build_controls(wm: *mut snd_wm8776) -> c_int {
    let mut err: c_int;

    for i in 0..WM8776_CTL_COUNT {
        if !(*wm).ctl[i as usize].name.is_null() {
            err = snd_wm8776_add_control(wm, i as c_int);
            if err < 0 {
                return err;
            }
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
