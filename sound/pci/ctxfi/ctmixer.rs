// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctmixer.c
 *
 * @Brief
 * This file contains the implementation of alsa mixer device functions.
 *
 * @Author	Liu Chun
 * @Date 	May 28 2008
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

/* Dependencies originally supplied by:
 * "ctmixer.h", "ctamixer.h", <linux/slab.h>, <sound/core.h>,
 * <sound/control.h>, <sound/asoundef.h>, <sound/pcm.h>, <sound/tlv.h>.
 */

const SUM_IN_F: c_int = 0;
const SUM_IN_R: c_int = 1;
const SUM_IN_C: c_int = 2;
const SUM_IN_S: c_int = 3;
const SUM_IN_F_C: c_int = 4;
const NUM_CT_SUMS: c_int = 5;

const AMIXER_MASTER_F: c_int = 0;
const AMIXER_MASTER_R: c_int = 1;
const AMIXER_MASTER_C: c_int = 2;
const AMIXER_MASTER_S: c_int = 3;
const AMIXER_PCM_F: c_int = 4;
const AMIXER_PCM_R: c_int = 5;
const AMIXER_PCM_C: c_int = 6;
const AMIXER_PCM_S: c_int = 7;
const AMIXER_SPDIFI: c_int = 8;
const AMIXER_LINEIN: c_int = 9;
const AMIXER_MIC: c_int = 10;
const AMIXER_SPDIFO: c_int = 11;
const AMIXER_WAVE_F: c_int = 12;
const AMIXER_WAVE_R: c_int = 13;
const AMIXER_WAVE_C: c_int = 14;
const AMIXER_WAVE_S: c_int = 15;
const AMIXER_MASTER_F_C: c_int = 16;
const AMIXER_PCM_F_C: c_int = 17;
const AMIXER_SPDIFI_C: c_int = 18;
const AMIXER_LINEIN_C: c_int = 19;
const AMIXER_MIC_C: c_int = 20;
const NUM_CT_AMIXERS: c_int = 21;

const MIXER_MASTER_P: c_int = 0;
const MIXER_PCM_P: c_int = 1;
const MIXER_LINEIN_P: c_int = 2;
const MIXER_MIC_P: c_int = 3;
const MIXER_SPDIFI_P: c_int = 4;
const MIXER_SPDIFO_P: c_int = 5;
const MIXER_WAVEF_P: c_int = 6;
const MIXER_WAVER_P: c_int = 7;
const MIXER_WAVEC_P: c_int = 8;
const MIXER_WAVES_P: c_int = 9;
const MIXER_MASTER_C: c_int = 10;
const MIXER_PCM_C: c_int = 11;
const MIXER_LINEIN_C: c_int = 12;
const MIXER_MIC_C: c_int = 13;
const MIXER_SPDIFI_C: c_int = 14;
const MIXER_PCM_C_S: c_int = 15;
const MIXER_LINEIN_C_S: c_int = 16;
const MIXER_MIC_C_S: c_int = 17;
const MIXER_SPDIFI_C_S: c_int = 18;
const MIXER_SPDIFO_P_S: c_int = 19;
const MIXER_WAVEF_P_S: c_int = 20;
const MIXER_WAVER_P_S: c_int = 21;
const MIXER_WAVEC_P_S: c_int = 22;
const MIXER_WAVES_P_S: c_int = 23;
const MIXER_DIGITAL_IO_S: c_int = 24;
const MIXER_IEC958_MASK: c_int = 25;
const MIXER_IEC958_DEFAULT: c_int = 26;
const MIXER_IEC958_STREAM: c_int = 27;
const NUM_CTALSA_MIXERS: c_int = 28;

const VOL_MIXER_START: c_int = MIXER_MASTER_P;
const VOL_MIXER_END: c_int = MIXER_SPDIFI_C;
const VOL_MIXER_NUM: c_int = VOL_MIXER_END - VOL_MIXER_START + 1;
const SWH_MIXER_START: c_int = MIXER_PCM_C_S;
const SWH_MIXER_END: c_int = MIXER_DIGITAL_IO_S;
const SWH_CAPTURE_START: c_int = MIXER_PCM_C_S;
const SWH_CAPTURE_END: c_int = MIXER_SPDIFI_C_S;
const CHN_NUM: c_int = 2;

#[repr(C)]
struct ct_kcontrol_init {
    ctl: u8,
    name: *mut c_char,
}

static mut ct_kcontrol_init_table: [ct_kcontrol_init; NUM_CTALSA_MIXERS as usize] = [
    ct_kcontrol_init { ctl: 1, name: b"Master Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"PCM Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Line Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Mic Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"IEC958 Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Digital Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Front Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Surround Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Center/LFE Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Side Playback Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Master Capture Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"PCM Capture Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Line Capture Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Mic Capture Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"IEC958 Capture Volume\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"PCM Capture Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Line Capture Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Mic Capture Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"IEC958 Capture Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Digital Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Front Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Surround Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Center/LFE Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 1, name: b"Side Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 0, name: b"Digit-IO Playback Switch\0".as_ptr() as *mut c_char },
    ct_kcontrol_init { ctl: 0, name: ptr::null_mut() },
    ct_kcontrol_init { ctl: 0, name: ptr::null_mut() },
    ct_kcontrol_init { ctl: 0, name: ptr::null_mut() },
];

fn get_amixer_index(alsa_index: c_int) -> c_int {
    match alsa_index {
        MIXER_MASTER_P => AMIXER_MASTER_F,
        MIXER_MASTER_C => AMIXER_MASTER_F_C,
        MIXER_PCM_P => AMIXER_PCM_F,
        MIXER_PCM_C | MIXER_PCM_C_S => AMIXER_PCM_F_C,
        MIXER_LINEIN_P => AMIXER_LINEIN,
        MIXER_LINEIN_C | MIXER_LINEIN_C_S => AMIXER_LINEIN_C,
        MIXER_MIC_P => AMIXER_MIC,
        MIXER_MIC_C | MIXER_MIC_C_S => AMIXER_MIC_C,
        MIXER_SPDIFI_P => AMIXER_SPDIFI,
        MIXER_SPDIFI_C | MIXER_SPDIFI_C_S => AMIXER_SPDIFI_C,
        MIXER_SPDIFO_P => AMIXER_SPDIFO,
        MIXER_WAVEF_P => AMIXER_WAVE_F,
        MIXER_WAVES_P => AMIXER_WAVE_S,
        MIXER_WAVEC_P => AMIXER_WAVE_C,
        MIXER_WAVER_P => AMIXER_WAVE_R,
        _ => NUM_CT_AMIXERS,
    }
}

fn get_recording_amixer(index: c_int) -> c_int {
    match index {
        AMIXER_MASTER_F => AMIXER_MASTER_F_C,
        AMIXER_PCM_F => AMIXER_PCM_F_C,
        AMIXER_SPDIFI => AMIXER_SPDIFI_C,
        AMIXER_LINEIN => AMIXER_LINEIN_C,
        AMIXER_MIC => AMIXER_MIC_C,
        _ => NUM_CT_AMIXERS,
    }
}

unsafe fn get_switch_state(mixer: *mut ct_mixer, type_: c_int) -> u8 {
    if ((*mixer).switch_state & (0x1 << (type_ - SWH_MIXER_START))) != 0 { 1 } else { 0 }
}

unsafe fn set_switch_state(mixer: *mut ct_mixer, type_: c_int, state: u8) {
    if state != 0 {
        (*mixer).switch_state |= 0x1 << (type_ - SWH_MIXER_START);
    } else {
        (*mixer).switch_state &= !(0x1 << (type_ - SWH_MIXER_START));
    }
}

/* #if 0, not used.
 * Map integer value ranging from 0 to 65535 to 14-bit float value ranging
 * from 2^-6 to (1+1023/1024)
 */
#[allow(dead_code)]
fn uint16_to_float14(mut x: c_uint) -> c_uint {
    let mut i: c_uint;
    if x < 17 {
        return 0;
    }
    x = x.wrapping_mul(2031);
    x /= 65535;
    x += 16;
    i = 0;
    while (x & 0x400) == 0 {
        x <<= 1;
        i += 1;
    }
    (((7 - i) & 0x7) << 10) | (x & 0x3ff)
}

#[allow(dead_code)]
fn float14_to_uint16(mut x: c_uint) -> c_uint {
    let e: c_uint;
    if x == 0 {
        return x;
    }
    e = (x >> 10) & 0x7;
    x &= 0x3ff;
    x += 1024;
    x >>= 7 - e;
    x -= 16;
    x = x.wrapping_mul(65535);
    x /= 2031;
    x
}

const VOL_SCALE: c_int = 0x1c;
const VOL_MAX: c_int = 0x100;
static ct_vol_db_scale: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2, -6400i32 as c_uint, 25 | (1u32 << 16)];

unsafe extern "C" fn ct_alsa_mix_volume_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = VOL_MAX as c_long;
    0
}

unsafe extern "C" fn ct_alsa_mix_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let type_ = get_amixer_index((*kcontrol).private_value as c_int);
    let mut i = 0;
    while i < 2 {
        let amixer = *(*((*atc).mixer as *mut ct_mixer)).amixers.offset((type_ * CHN_NUM + i) as isize);
        let mut val = ((*(*amixer).ops).get_scale.unwrap())(amixer) / VOL_SCALE;
        if val < 0 { val = 0; } else if val > VOL_MAX { val = VOL_MAX; }
        (*ucontrol).value.integer.value[i as usize] = val as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn ct_alsa_mix_volume_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let mixer = (*atc).mixer as *mut ct_mixer;
    let type_ = get_amixer_index((*kcontrol).private_value as c_int);
    let mut change = 0;
    let mut i = 0;
    while i < 2 {
        let mut val = (*ucontrol).value.integer.value[i as usize] as c_int;
        if val < 0 { val = 0; } else if val > VOL_MAX { val = VOL_MAX; }
        val *= VOL_SCALE;
        let mut amixer = *(*mixer).amixers.offset((type_ * CHN_NUM + i) as isize);
        let oval = ((*(*amixer).ops).get_scale.unwrap())(amixer);
        if val != oval {
            ((*(*amixer).ops).set_scale.unwrap())(amixer, val);
            ((*(*amixer).ops).commit_write.unwrap())(amixer);
            change = 1;
            /* Synchronize Master/PCM playback AMIXERs. */
            if AMIXER_MASTER_F == type_ || AMIXER_PCM_F == type_ {
                let mut j = 1;
                while j < 4 {
                    amixer = *(*mixer).amixers.offset(((type_ + j) * CHN_NUM + i) as isize);
                    ((*(*amixer).ops).set_scale.unwrap())(amixer, val);
                    ((*(*amixer).ops).commit_write.unwrap())(amixer);
                    j += 1;
                }
            }
        }
        i += 1;
    }
    change
}

static mut vol_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: ptr::null_mut(),
    count: 0,
    info: Some(ct_alsa_mix_volume_info),
    get: Some(ct_alsa_mix_volume_get),
    put: Some(ct_alsa_mix_volume_put),
    private_value: 0,
    tlv: snd_kcontrol_new_tlv { p: ct_vol_db_scale.as_ptr() },
};

unsafe extern "C" fn output_switch_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 3] = [
        b"FP Headphones\0".as_ptr() as *const c_char,
        b"Headphones\0".as_ptr() as *const c_char,
        b"Speakers\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(info, 1, 3, names.as_ptr())
}

unsafe extern "C" fn output_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    (*ucontrol).value.enumerated.item[0] = ((*atc).output_switch_get.unwrap())(atc);
    0
}

unsafe extern "C" fn output_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    if (*ucontrol).value.enumerated.item[0] > 2 {
        return -EINVAL;
    }
    ((*atc).output_switch_put.unwrap())(atc, (*ucontrol).value.enumerated.item[0])
}

static mut output_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: 0, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog Output Playback Enum\0".as_ptr() as *mut c_char,
    count: 0, info: Some(output_switch_info), get: Some(output_switch_get), put: Some(output_switch_put),
    private_value: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};

unsafe extern "C" fn mic_source_switch_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 3] = [
        b"Mic\0".as_ptr() as *const c_char,
        b"FP Mic\0".as_ptr() as *const c_char,
        b"Aux\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(info, 1, 3, names.as_ptr())
}

unsafe extern "C" fn mic_source_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    (*ucontrol).value.enumerated.item[0] = ((*atc).mic_source_switch_get.unwrap())(atc);
    0
}

unsafe extern "C" fn mic_source_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    if (*ucontrol).value.enumerated.item[0] > 2 {
        return -EINVAL;
    }
    ((*atc).mic_source_switch_put.unwrap())(atc, (*ucontrol).value.enumerated.item[0])
}

static mut mic_source_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: 0, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Mic Source Capture Enum\0".as_ptr() as *mut c_char,
    count: 0, info: Some(mic_source_switch_info), get: Some(mic_source_switch_get), put: Some(mic_source_switch_put),
    private_value: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};

unsafe fn do_line_mic_switch(atc: *mut ct_atc, type_: c_int) {
    let mixer = (*atc).mixer as *mut ct_mixer;
    if MIXER_LINEIN_C_S == type_ {
        ((*atc).select_line_in.unwrap())(atc);
        set_switch_state(mixer, MIXER_MIC_C_S, 0);
        snd_ctl_notify((*atc).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*(*mixer).line_mic_kctls.as_mut_ptr().add(1))).id);
    } else if MIXER_MIC_C_S == type_ {
        ((*atc).select_mic_in.unwrap())(atc);
        set_switch_state(mixer, MIXER_LINEIN_C_S, 0);
        snd_ctl_notify((*atc).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*(*mixer).line_mic_kctls.as_mut_ptr().add(0))).id);
    }
}

unsafe fn do_digit_io_switch(atc: *mut ct_atc, state: c_int) {
    let mixer = (*atc).mixer as *mut ct_mixer;
    if state != 0 {
        ((*atc).select_digit_io.unwrap())(atc);
        ((*atc).spdif_out_unmute.unwrap())(atc, get_switch_state(mixer, MIXER_SPDIFO_P_S) as c_int);
        ((*atc).spdif_in_unmute.unwrap())(atc, 1);
        ((*atc).line_in_unmute.unwrap())(atc, 0);
        return;
    }
    if get_switch_state(mixer, MIXER_LINEIN_C_S) != 0 {
        ((*atc).select_line_in.unwrap())(atc);
    } else if get_switch_state(mixer, MIXER_MIC_C_S) != 0 {
        ((*atc).select_mic_in.unwrap())(atc);
    }
    ((*atc).spdif_out_unmute.unwrap())(atc, 0);
    ((*atc).spdif_in_unmute.unwrap())(atc, 0);
    ((*atc).line_in_unmute.unwrap())(atc, 1);
}

unsafe fn do_switch(atc: *mut ct_atc, type_: c_int, state: c_int) {
    let mixer = (*atc).mixer as *mut ct_mixer;
    let cap = ((*atc).capabilities.unwrap())(atc);

    /* Do changes in mixer. */
    if SWH_CAPTURE_START <= type_ && SWH_CAPTURE_END >= type_ {
        if state != 0 {
            ct_mixer_recording_select(mixer, get_amixer_index(type_));
        } else {
            ct_mixer_recording_unselect(mixer, get_amixer_index(type_));
        }
    }
    /* Do changes out of mixer. */
    if cap.dedicated_mic == 0 && (MIXER_LINEIN_C_S == type_ || MIXER_MIC_C_S == type_) {
        if state != 0 {
            do_line_mic_switch(atc, type_);
        }
        ((*atc).line_in_unmute.unwrap())(atc, state);
    } else if cap.dedicated_mic != 0 && MIXER_LINEIN_C_S == type_ {
        ((*atc).line_in_unmute.unwrap())(atc, state);
    } else if cap.dedicated_mic != 0 && MIXER_MIC_C_S == type_ {
        ((*atc).mic_unmute.unwrap())(atc, state);
    } else if MIXER_SPDIFI_C_S == type_ {
        ((*atc).spdif_in_unmute.unwrap())(atc, state);
    } else if MIXER_WAVEF_P_S == type_ {
        if cap.dedicated_rca != 0 {
            ((*atc).rca_unmute.unwrap())(atc, if (*atc).rca_state != 0 { 0 } else { state });
            ((*atc).line_front_unmute.unwrap())(atc, if (*atc).rca_state != 0 { state } else { 0 });
        } else {
            ((*atc).line_front_unmute.unwrap())(atc, state);
        }
    } else if MIXER_WAVES_P_S == type_ {
        ((*atc).line_surround_unmute.unwrap())(atc, state);
    } else if MIXER_WAVEC_P_S == type_ {
        ((*atc).line_clfe_unmute.unwrap())(atc, state);
    } else if MIXER_WAVER_P_S == type_ {
        ((*atc).line_rear_unmute.unwrap())(atc, state);
    } else if MIXER_SPDIFO_P_S == type_ {
        ((*atc).spdif_out_unmute.unwrap())(atc, state);
    } else if MIXER_DIGITAL_IO_S == type_ {
        do_digit_io_switch(atc, state);
    }
}

unsafe extern "C" fn ct_alsa_mix_switch_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    (*uinfo).value.integer.step = 1;
    0
}

unsafe extern "C" fn ct_alsa_mix_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mixer = (*(snd_kcontrol_chip(kcontrol) as *mut ct_atc)).mixer as *mut ct_mixer;
    let type_ = (*kcontrol).private_value as c_int;
    (*ucontrol).value.integer.value[0] = get_switch_state(mixer, type_) as c_long;
    0
}

unsafe extern "C" fn ct_alsa_mix_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let mixer = (*atc).mixer as *mut ct_mixer;
    let type_ = (*kcontrol).private_value as c_int;
    let state = (*ucontrol).value.integer.value[0] as c_int;
    if get_switch_state(mixer, type_) as c_int == state {
        return 0;
    }
    set_switch_state(mixer, type_, state as u8);
    do_switch(atc, type_, state);
    1
}

static mut swh_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: ptr::null_mut(),
    count: 0, info: Some(ct_alsa_mix_switch_info), get: Some(ct_alsa_mix_switch_get), put: Some(ct_alsa_mix_switch_put),
    private_value: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};

unsafe extern "C" fn dedicated_rca_info(_kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [b"RCA\0".as_ptr() as *const c_char, b"Front\0".as_ptr() as *const c_char];
    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn dedicated_rca_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    (*ucontrol).value.enumerated.item[0] = (*atc).rca_state;
    0
}

unsafe extern "C" fn dedicated_rca_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let rca_state = (*ucontrol).value.enumerated.item[0];
    let state: u8;
    if rca_state > 1 { return -EINVAL; }
    if rca_state == (*atc).rca_state { return 0; }
    state = get_switch_state((*atc).mixer as *mut ct_mixer, MIXER_WAVEF_P_S);
    do_switch(atc, MIXER_WAVEF_P_S, 0);
    (*atc).rca_state = rca_state;
    ((*atc).dedicated_rca_select.unwrap())(atc);
    do_switch(atc, MIXER_WAVEF_P_S, state as c_int);
    1
}

static mut rca_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: 0, iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog Playback Route\0".as_ptr() as *mut c_char,
    count: 0, info: Some(dedicated_rca_info), get: Some(dedicated_rca_get), put: Some(dedicated_rca_put),
    private_value: 0, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};

unsafe extern "C" fn ct_spdif_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn ct_spdif_get_mask(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.iec958.status[0] = 0xff;
    (*ucontrol).value.iec958.status[1] = 0xff;
    (*ucontrol).value.iec958.status[2] = 0xff;
    (*ucontrol).value.iec958.status[3] = 0xff;
    0
}

unsafe extern "C" fn ct_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let mut status: c_uint = 0;
    ((*atc).spdif_out_get_status.unwrap())(atc, &mut status);
    if status == 0 {
        status = SNDRV_PCM_DEFAULT_CON_SPDIF;
    }
    (*ucontrol).value.iec958.status[0] = ((status >> 0) & 0xff) as u8;
    (*ucontrol).value.iec958.status[1] = ((status >> 8) & 0xff) as u8;
    (*ucontrol).value.iec958.status[2] = ((status >> 16) & 0xff) as u8;
    (*ucontrol).value.iec958.status[3] = ((status >> 24) & 0xff) as u8;
    0
}

unsafe extern "C" fn ct_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let atc = snd_kcontrol_chip(kcontrol) as *mut ct_atc;
    let status = (((*ucontrol).value.iec958.status[0] as c_uint) << 0)
        | (((*ucontrol).value.iec958.status[1] as c_uint) << 8)
        | (((*ucontrol).value.iec958.status[2] as c_uint) << 16)
        | (((*ucontrol).value.iec958.status[3] as c_uint) << 24);
    let mut old_status: c_uint = 0;
    ((*atc).spdif_out_get_status.unwrap())(atc, &mut old_status);
    let change = (old_status != status) as c_int;
    if change != 0 {
        ((*atc).spdif_out_set_status.unwrap())(atc, status);
    }
    change
}

static mut iec958_mask_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ, iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"IEC958 Playback Mask\0".as_ptr() as *mut c_char, count: 1,
    info: Some(ct_spdif_info), get: Some(ct_spdif_get_mask), put: None,
    private_value: MIXER_IEC958_MASK as c_ulong, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};
static mut iec958_default_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: 0, iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"IEC958 Playback Default\0".as_ptr() as *mut c_char, count: 1,
    info: Some(ct_spdif_info), get: Some(ct_spdif_get), put: Some(ct_spdif_put),
    private_value: MIXER_IEC958_DEFAULT as c_ulong, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};
static mut iec958_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE, iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"IEC958 Playback PCM Stream\0".as_ptr() as *mut c_char, count: 1,
    info: Some(ct_spdif_info), get: Some(ct_spdif_get), put: Some(ct_spdif_put),
    private_value: MIXER_IEC958_STREAM as c_ulong, tlv: snd_kcontrol_new_tlv { p: ptr::null() },
};

const NUM_IEC958_CTL: c_int = 3;

unsafe fn ct_mixer_kcontrol_new(mixer: *mut ct_mixer, new_: *mut snd_kcontrol_new) -> c_int {
    let kctl = snd_ctl_new1(new_, (*mixer).atc as *mut c_void);
    if kctl.is_null() {
        return -ENOMEM;
    }
    if SNDRV_CTL_ELEM_IFACE_PCM == (*kctl).id.iface {
        (*kctl).id.device = IEC958;
    }
    let err = snd_ctl_add((*(*mixer).atc).card, kctl);
    if err != 0 {
        return err;
    }
    match (*new_).private_value as c_int {
        MIXER_LINEIN_C_S => (*mixer).line_mic_kctls[0] = kctl,
        MIXER_MIC_C_S => (*mixer).line_mic_kctls[1] = kctl,
        _ => {}
    }
    0
}

unsafe fn ct_mixer_kcontrols_create(mixer: *mut ct_mixer) -> c_int {
    let atc = (*mixer).atc;
    let cap = ((*atc).capabilities.unwrap())(atc);
    let mut type_ = VOL_MIXER_START;
    while type_ <= VOL_MIXER_END {
        if ct_kcontrol_init_table[type_ as usize].ctl != 0 {
            vol_ctl.name = ct_kcontrol_init_table[type_ as usize].name;
            vol_ctl.private_value = type_ as c_ulong;
            let err = ct_mixer_kcontrol_new(mixer, &mut vol_ctl);
            if err != 0 { return err; }
        }
        type_ += 1;
    }
    ct_kcontrol_init_table[MIXER_DIGITAL_IO_S as usize].ctl = cap.digit_io_switch as u8;
    type_ = SWH_MIXER_START;
    while type_ <= SWH_MIXER_END {
        if ct_kcontrol_init_table[type_ as usize].ctl != 0 {
            swh_ctl.name = ct_kcontrol_init_table[type_ as usize].name;
            swh_ctl.private_value = type_ as c_ulong;
            let err = ct_mixer_kcontrol_new(mixer, &mut swh_ctl);
            if err != 0 { return err; }
        }
        type_ += 1;
    }
    let mut err = ct_mixer_kcontrol_new(mixer, &mut iec958_mask_ctl);
    if err != 0 { return err; }
    err = ct_mixer_kcontrol_new(mixer, &mut iec958_default_ctl);
    if err != 0 { return err; }
    err = ct_mixer_kcontrol_new(mixer, &mut iec958_ctl);
    if err != 0 { return err; }
    if cap.output_switch != 0 {
        err = ct_mixer_kcontrol_new(mixer, &mut output_ctl);
        if err != 0 { return err; }
    }
    if cap.mic_source_switch != 0 {
        err = ct_mixer_kcontrol_new(mixer, &mut mic_source_ctl);
        if err != 0 { return err; }
    }
    if cap.dedicated_rca != 0 {
        err = ct_mixer_kcontrol_new(mixer, &mut rca_ctl);
        if err != 0 { return err; }
        ((*atc).line_front_unmute.unwrap())(atc, 0);
        ((*atc).rca_unmute.unwrap())(atc, 1);
    } else {
        ((*atc).line_front_unmute.unwrap())(atc, 1);
    }
    set_switch_state(mixer, MIXER_WAVEF_P_S, 1);
    ((*atc).line_surround_unmute.unwrap())(atc, 0); set_switch_state(mixer, MIXER_WAVES_P_S, 0);
    ((*atc).line_clfe_unmute.unwrap())(atc, 0); set_switch_state(mixer, MIXER_WAVEC_P_S, 0);
    ((*atc).line_rear_unmute.unwrap())(atc, 0); set_switch_state(mixer, MIXER_WAVER_P_S, 0);
    ((*atc).spdif_out_unmute.unwrap())(atc, 0); set_switch_state(mixer, MIXER_SPDIFO_P_S, 0);
    ((*atc).line_in_unmute.unwrap())(atc, 0);
    if cap.dedicated_mic != 0 { ((*atc).mic_unmute.unwrap())(atc, 0); }
    ((*atc).spdif_in_unmute.unwrap())(atc, 0);
    set_switch_state(mixer, MIXER_PCM_C_S, 0);
    set_switch_state(mixer, MIXER_LINEIN_C_S, 0);
    set_switch_state(mixer, MIXER_SPDIFI_C_S, 0);
    0
}

unsafe fn ct_mixer_recording_select(mixer: *mut ct_mixer, type_: c_int) {
    let mut i = 0;
    while i < 2 {
        let amix_d = *(*mixer).amixers.offset((type_ * CHN_NUM + i) as isize);
        let sum_c = *(*mixer).sums.offset((SUM_IN_F_C * CHN_NUM + i) as isize);
        ((*(*amix_d).ops).set_sum.unwrap())(amix_d, sum_c);
        ((*(*amix_d).ops).commit_write.unwrap())(amix_d);
        i += 1;
    }
}

unsafe fn ct_mixer_recording_unselect(mixer: *mut ct_mixer, type_: c_int) {
    let mut i = 0;
    while i < 2 {
        let amix_d = *(*mixer).amixers.offset((type_ * CHN_NUM + i) as isize);
        ((*(*amix_d).ops).set_sum.unwrap())(amix_d, ptr::null_mut());
        ((*(*amix_d).ops).commit_write.unwrap())(amix_d);
        i += 1;
    }
}

unsafe fn ct_mixer_get_resources(mixer: *mut ct_mixer) -> c_int {
    let sum_mgr = (*(*mixer).atc).rsc_mgrs[SUM as usize] as *mut sum_mgr;
    let mut sum_desc: sum_desc = core::mem::zeroed();
    let amixer_mgr = (*(*mixer).atc).rsc_mgrs[AMIXER as usize] as *mut amixer_mgr;
    let mut am_desc: amixer_desc = core::mem::zeroed();
    let mut err = 0;
    let mut i = 0;
    sum_desc.msr = (*(*mixer).atc).msr;
    while i < NUM_CT_SUMS * CHN_NUM {
        let mut sum: *mut sum = ptr::null_mut();
        err = ((*sum_mgr).get_sum.unwrap())(sum_mgr, &mut sum_desc, &mut sum);
        if err != 0 {
            dev_err((*(*(*mixer).atc).card).dev, b"Failed to get sum resources for front output!\n\0".as_ptr() as *const c_char);
            break;
        }
        *(*mixer).sums.offset(i as isize) = sum;
        i += 1;
    }
    if err != 0 { goto_error1(mixer, sum_mgr); return err; }
    am_desc.msr = (*(*mixer).atc).msr;
    i = 0;
    while i < NUM_CT_AMIXERS * CHN_NUM {
        let mut amixer: *mut amixer = ptr::null_mut();
        err = ((*amixer_mgr).get_amixer.unwrap())(amixer_mgr, &mut am_desc, &mut amixer);
        if err != 0 {
            dev_err((*(*(*mixer).atc).card).dev, b"Failed to get amixer resources for mixer obj!\n\0".as_ptr() as *const c_char);
            break;
        }
        *(*mixer).amixers.offset(i as isize) = amixer;
        i += 1;
    }
    if err != 0 {
        i = 0;
        while i < NUM_CT_AMIXERS * CHN_NUM {
            let p = *(*mixer).amixers.offset(i as isize);
            if !p.is_null() {
                ((*amixer_mgr).put_amixer.unwrap())(amixer_mgr, p);
                *(*mixer).amixers.offset(i as isize) = ptr::null_mut();
            }
            i += 1;
        }
        goto_error1(mixer, sum_mgr);
        return err;
    }
    0
}

unsafe fn goto_error1(mixer: *mut ct_mixer, sum_mgr: *mut sum_mgr) {
    let mut i = 0;
    while i < NUM_CT_SUMS * CHN_NUM {
        let p = *(*mixer).sums.offset(i as isize);
        if !p.is_null() {
            ((*sum_mgr).put_sum.unwrap())(sum_mgr, p);
            *(*mixer).sums.offset(i as isize) = ptr::null_mut();
        }
        i += 1;
    }
}

unsafe fn ct_mixer_get_mem(rmixer: *mut *mut ct_mixer) -> c_int {
    *rmixer = ptr::null_mut();
    let mut alloc_size = struct_size_ct_mixer_amixers((NUM_CT_AMIXERS * CHN_NUM) as usize);
    alloc_size += core::mem::size_of::<*mut sum>() * (NUM_CT_SUMS * CHN_NUM) as usize;
    let mixer = kzalloc(alloc_size, GFP_KERNEL) as *mut ct_mixer;
    if mixer.is_null() {
        return -ENOMEM;
    }
    (*mixer).sums = (*mixer).amixers.offset((NUM_CT_AMIXERS * CHN_NUM) as isize) as *mut *mut sum;
    *rmixer = mixer;
    0
}

unsafe fn ct_mixer_topology_build(mixer: *mut ct_mixer) -> c_int {
    /* Build topology from destination to source */
    let mut i = AMIXER_MASTER_F;
    let mut k = SUM_IN_F;
    while i <= AMIXER_MASTER_S {
        setup_amixer_sum_pair(mixer, i, k, true);
        i += 1; k += 1;
    }
    i = AMIXER_WAVE_F;
    let mut j = AMIXER_MASTER_F;
    while i <= AMIXER_WAVE_S {
        setup_amixer_amixer_pair(mixer, i, j);
        i += 1; j += 1;
    }
    setup_amixer_amixer_pair(mixer, AMIXER_SPDIFO, AMIXER_MASTER_F);
    i = AMIXER_PCM_F; k = SUM_IN_F;
    while i <= AMIXER_PCM_S {
        setup_amixer_sum_pair(mixer, i, k, false);
        i += 1; k += 1;
    }
    setup_amixer_sum_pair(mixer, AMIXER_LINEIN, SUM_IN_F, false);
    setup_amixer_sum_pair(mixer, AMIXER_MIC, SUM_IN_F, false);
    setup_amixer_sum_pair(mixer, AMIXER_SPDIFI, SUM_IN_F, false);
    setup_amixer_sum_pair(mixer, AMIXER_MASTER_F_C, SUM_IN_F_C, true);
    setup_amixer_sum_pair(mixer, AMIXER_PCM_F_C, SUM_IN_F_C, false);
    setup_amixer_sum_pair(mixer, AMIXER_LINEIN_C, SUM_IN_F_C, false);
    setup_amixer_sum_pair(mixer, AMIXER_MIC_C, SUM_IN_F_C, false);
    setup_amixer_sum_pair(mixer, AMIXER_SPDIFI_C, SUM_IN_F_C, false);
    0
}

unsafe fn setup_amixer_sum_pair(mixer: *mut ct_mixer, amix: c_int, sum_index: c_int, output_sum: bool) {
    let mut ch = 0;
    while ch < 2 {
        let amix_d = *(*mixer).amixers.offset((amix * CHN_NUM + ch) as isize);
        let sum = *(*mixer).sums.offset((sum_index * CHN_NUM + ch) as isize);
        if output_sum {
            ((*(*amix_d).ops).setup.unwrap())(amix_d, &mut (*sum).rsc, INIT_VOL, ptr::null_mut());
        } else {
            ((*(*amix_d).ops).setup.unwrap())(amix_d, ptr::null_mut(), INIT_VOL, sum);
        }
        ch += 1;
    }
}

unsafe fn setup_amixer_amixer_pair(mixer: *mut ct_mixer, dst: c_int, src: c_int) {
    let mut ch = 0;
    while ch < 2 {
        let amix_d = *(*mixer).amixers.offset((dst * CHN_NUM + ch) as isize);
        let amix_s = *(*mixer).amixers.offset((src * CHN_NUM + ch) as isize);
        ((*(*amix_d).ops).setup.unwrap())(amix_d, &mut (*amix_s).rsc, INIT_VOL, ptr::null_mut());
        ch += 1;
    }
}

unsafe fn mixer_set_input_port(amixer: *mut amixer, rsc: *mut rsc) -> c_int {
    ((*(*amixer).ops).set_input.unwrap())(amixer, rsc);
    ((*(*amixer).ops).commit_write.unwrap())(amixer);
    0
}

fn port_to_amixer(type_: c_int) -> c_int {
    match type_ {
        MIX_WAVE_FRONT => AMIXER_WAVE_F,
        MIX_WAVE_SURROUND => AMIXER_WAVE_S,
        MIX_WAVE_CENTLFE => AMIXER_WAVE_C,
        MIX_WAVE_REAR => AMIXER_WAVE_R,
        MIX_PCMO_FRONT => AMIXER_MASTER_F_C,
        MIX_SPDIF_OUT => AMIXER_SPDIFO,
        MIX_LINE_IN => AMIXER_LINEIN,
        MIX_MIC_IN => AMIXER_MIC,
        MIX_SPDIF_IN => AMIXER_SPDIFI,
        MIX_PCMI_FRONT => AMIXER_PCM_F,
        MIX_PCMI_SURROUND => AMIXER_PCM_S,
        MIX_PCMI_CENTLFE => AMIXER_PCM_C,
        MIX_PCMI_REAR => AMIXER_PCM_R,
        _ => 0,
    }
}

unsafe extern "C" fn mixer_get_output_ports(mixer: *mut ct_mixer, type_: c_int, rleft: *mut *mut rsc, rright: *mut *mut rsc) -> c_int {
    let amix = port_to_amixer(type_);
    if !rleft.is_null() {
        *rleft = &mut (*(*(*mixer).amixers.offset((amix * CHN_NUM) as isize))).rsc;
    }
    if !rright.is_null() {
        *rright = &mut (*(*(*mixer).amixers.offset((amix * CHN_NUM + 1) as isize))).rsc;
    }
    0
}

unsafe extern "C" fn mixer_set_input_left(mixer: *mut ct_mixer, type_: c_int, rsc: *mut rsc) -> c_int {
    let mut amix = port_to_amixer(type_);
    mixer_set_input_port(*(*mixer).amixers.offset((amix * CHN_NUM) as isize), rsc);
    amix = get_recording_amixer(amix);
    if amix < NUM_CT_AMIXERS {
        mixer_set_input_port(*(*mixer).amixers.offset((amix * CHN_NUM) as isize), rsc);
    }
    0
}

unsafe extern "C" fn mixer_set_input_right(mixer: *mut ct_mixer, type_: c_int, rsc: *mut rsc) -> c_int {
    let mut amix = port_to_amixer(type_);
    mixer_set_input_port(*(*mixer).amixers.offset((amix * CHN_NUM + 1) as isize), rsc);
    amix = get_recording_amixer(amix);
    if amix < NUM_CT_AMIXERS {
        mixer_set_input_port(*(*mixer).amixers.offset((amix * CHN_NUM + 1) as isize), rsc);
    }
    0
}

/* CONFIG_PM_SLEEP conditional code from the C source. */
unsafe extern "C" fn mixer_resume(mixer: *mut ct_mixer) -> c_int {
    let mut i = 0;
    while i < NUM_CT_AMIXERS * CHN_NUM {
        let amixer = *(*mixer).amixers.offset(i as isize);
        ((*(*amixer).ops).commit_write.unwrap())(amixer);
        i += 1;
    }
    i = SWH_MIXER_START;
    while i <= SWH_MIXER_END {
        let state = get_switch_state(mixer, i) as c_int;
        do_switch((*mixer).atc, i, state);
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ct_mixer_destroy(mixer: *mut ct_mixer) -> c_int {
    let sum_mgr = (*(*mixer).atc).rsc_mgrs[SUM as usize] as *mut sum_mgr;
    let amixer_mgr = (*(*mixer).atc).rsc_mgrs[AMIXER as usize] as *mut amixer_mgr;
    let mut i = 0;
    /* Release amixer resources */
    while i < NUM_CT_AMIXERS * CHN_NUM {
        let amixer = *(*mixer).amixers.offset(i as isize);
        if !amixer.is_null() {
            ((*amixer_mgr).put_amixer.unwrap())(amixer_mgr, amixer);
        }
        i += 1;
    }
    /* Release sum resources */
    i = 0;
    while i < NUM_CT_SUMS * CHN_NUM {
        let sum = *(*mixer).sums.offset(i as isize);
        if !sum.is_null() {
            ((*sum_mgr).put_sum.unwrap())(sum_mgr, sum);
        }
        i += 1;
    }
    /* Release mem assigned to mixer object */
    kfree(mixer as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ct_mixer_create(atc: *mut ct_atc, rmixer: *mut *mut ct_mixer) -> c_int {
    let mut mixer: *mut ct_mixer = ptr::null_mut();
    *rmixer = ptr::null_mut();
    /* Allocate mem for mixer obj */
    let mut err = ct_mixer_get_mem(&mut mixer);
    if err != 0 {
        return err;
    }
    (*mixer).switch_state = 0;
    (*mixer).atc = atc;
    /* Set operations */
    (*mixer).get_output_ports = Some(mixer_get_output_ports);
    (*mixer).set_input_left = Some(mixer_set_input_left);
    (*mixer).set_input_right = Some(mixer_set_input_right);
    /* CONFIG_PM_SLEEP: mixer->resume = mixer_resume; */
    (*mixer).resume = Some(mixer_resume);
    /* Allocate chip resources for mixer obj */
    err = ct_mixer_get_resources(mixer);
    if err != 0 {
        ct_mixer_destroy(mixer);
        return err;
    }
    /* Build internal mixer topology */
    ct_mixer_topology_build(mixer);
    *rmixer = mixer;
    0
}

#[no_mangle]
pub unsafe extern "C" fn ct_alsa_mix_create(atc: *mut ct_atc, _device: c_int, device_name: *const c_char) -> c_int {
    /* Create snd kcontrol instances on demand */
    /* vol_ctl.device = swh_ctl.device = device; */ /* better w/ device 0 */
    let err = ct_mixer_kcontrols_create((*atc).mixer as *mut ct_mixer);
    if err != 0 {
        return err;
    }
    strscpy((*(*atc).card).mixername.as_mut_ptr(), device_name);
    0
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
