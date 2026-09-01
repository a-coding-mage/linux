// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver for Onyx codec
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 *
 * This is a driver for the pcm3052 codec chip (codenamed Onyx)
 * that is present in newer Apple hardware (with digital output).
 *
 * The Onyx codec has the following connections (listed by the bit
 * to be used in aoa_codec.connected):
 *  0: analog output
 *  1: digital output
 *  2: line input
 *  3: microphone input
 * Note that even though I know of no machine that has for example
 * the digital output connected but not the analog, I have handled
 * all the different cases in the code so that this driver may serve
 * as a good example of what to do.
 *
 * NOTE: This driver assumes that there's at most one chip to be
 * 	 used with one alsa card, in form of creating all kinds
 *	 of mixer elements without regard for their existence.
 *	 But snd-aoa assumes that there's at most one card, so
 *	 this means you can only have one onyx on a system. This
 *	 should probably be fixed by changing the assumption of
 *	 having just a single card on a system, and making the
 *	 'card' pointer accessible to anyone who needs it instead
 *	 of hiding it in the aoa_snd_* functions...
 */

use crate::*;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

const PFX: &[u8] = b"snd-aoa-codec-onyx: \0";

#[repr(C)]
pub struct onyx {
    /* cache registers 65 to 80, they are write-only! */
    pub cache: [u8; 16],
    pub i2c: *mut i2c_client,
    pub codec: aoa_codec,
    pub initialised: u32,
    pub spdif_locked: u32,
    pub analog_locked: u32,
    pub original_mute: u32,
    pub open_count: c_int,
    pub codec_info: *mut codec_info,

    /* mutex serializes concurrent access to the device
     * and this structure.
     */
    pub mutex: mutex,
}

unsafe fn codec_to_onyx(c: *mut aoa_codec) -> *mut onyx {
    (c as *mut u8).sub(mem::offset_of!(onyx, codec)) as *mut onyx
}

/* both return 0 if all ok, else on error */
unsafe fn onyx_read_register(onyx: *mut onyx, reg: u8, value: *mut u8) -> c_int {
    let v: s32;

    if reg != ONYX_REG_CONTROL {
        *value = (*onyx).cache[(reg - FIRSTREGISTER) as usize];
        return 0;
    }
    v = i2c_smbus_read_byte_data((*onyx).i2c, reg);
    if v < 0 {
        *value = 0;
        return -1;
    }
    *value = v as u8;
    (*onyx).cache[(ONYX_REG_CONTROL - FIRSTREGISTER) as usize] = *value;
    0
}

unsafe fn onyx_write_register(onyx: *mut onyx, reg: u8, value: u8) -> c_int {
    let result: c_int;

    result = i2c_smbus_write_byte_data((*onyx).i2c, reg, value);
    if result == 0 {
        (*onyx).cache[(reg - FIRSTREGISTER) as usize] = value;
    }
    result
}

/* alsa stuff */

unsafe extern "C" fn onyx_dev_register(_dev: *mut snd_device) -> c_int {
    0
}

static ops: snd_device_ops = snd_device_ops {
    dev_register: Some(onyx_dev_register),
};

/* this is necessary because most alsa mixer programs
 * can't properly handle the negative range */
const VOLUME_RANGE_SHIFT: c_long = 128;

unsafe extern "C" fn onyx_snd_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = -128 + VOLUME_RANGE_SHIFT;
    (*uinfo).value.integer.max = -1 + VOLUME_RANGE_SHIFT;
    0
}

unsafe extern "C" fn onyx_snd_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut l: i8 = 0;
    let mut r: i8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DAC_ATTEN_LEFT, &mut l as *mut i8 as *mut u8);
    onyx_read_register(onyx, ONYX_REG_DAC_ATTEN_RIGHT, &mut r as *mut i8 as *mut u8);

    (*ucontrol).value.integer.value[0] = l as c_long + VOLUME_RANGE_SHIFT;
    (*ucontrol).value.integer.value[1] = r as c_long + VOLUME_RANGE_SHIFT;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_snd_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut l: i8 = 0;
    let mut r: i8 = 0;

    if (*ucontrol).value.integer.value[0] < -128 + VOLUME_RANGE_SHIFT
        || (*ucontrol).value.integer.value[0] > -1 + VOLUME_RANGE_SHIFT
    {
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[1] < -128 + VOLUME_RANGE_SHIFT
        || (*ucontrol).value.integer.value[1] > -1 + VOLUME_RANGE_SHIFT
    {
        return -EINVAL;
    }

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DAC_ATTEN_LEFT, &mut l as *mut i8 as *mut u8);
    onyx_read_register(onyx, ONYX_REG_DAC_ATTEN_RIGHT, &mut r as *mut i8 as *mut u8);

    if l as c_long + VOLUME_RANGE_SHIFT == (*ucontrol).value.integer.value[0]
        && r as c_long + VOLUME_RANGE_SHIFT == (*ucontrol).value.integer.value[1]
    {
        mutex_unlock(&mut (*onyx).mutex);
        return 0;
    }

    onyx_write_register(
        onyx,
        ONYX_REG_DAC_ATTEN_LEFT,
        ((*ucontrol).value.integer.value[0] - VOLUME_RANGE_SHIFT) as u8,
    );
    onyx_write_register(
        onyx,
        ONYX_REG_DAC_ATTEN_RIGHT,
        ((*ucontrol).value.integer.value[1] - VOLUME_RANGE_SHIFT) as u8,
    );
    mutex_unlock(&mut (*onyx).mutex);

    1
}

static volume_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Playback Volume\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(onyx_snd_vol_info),
    get: Some(onyx_snd_vol_get),
    put: Some(onyx_snd_vol_put),
    ..unsafe { mem::zeroed() }
};

/* like above, this is necessary because a lot
 * of alsa mixer programs don't handle ranges
 * that don't start at 0 properly.
 * even alsamixer is one of them... */
const INPUTGAIN_RANGE_SHIFT: c_long = -3;

unsafe extern "C" fn onyx_snd_inputgain_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 3 + INPUTGAIN_RANGE_SHIFT;
    (*uinfo).value.integer.max = 28 + INPUTGAIN_RANGE_SHIFT;
    0
}

unsafe extern "C" fn onyx_snd_inputgain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut ig: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_ADC_CONTROL, &mut ig);

    (*ucontrol).value.integer.value[0] =
        ((ig & ONYX_ADC_PGA_GAIN_MASK) as c_long) + INPUTGAIN_RANGE_SHIFT;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_snd_inputgain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: u8 = 0;
    let mut n: u8;

    if (*ucontrol).value.integer.value[0] < 3 + INPUTGAIN_RANGE_SHIFT
        || (*ucontrol).value.integer.value[0] > 28 + INPUTGAIN_RANGE_SHIFT
    {
        return -EINVAL;
    }
    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_ADC_CONTROL, &mut v);
    n = v;
    n &= !ONYX_ADC_PGA_GAIN_MASK;
    n |= ((*ucontrol).value.integer.value[0] - INPUTGAIN_RANGE_SHIFT) as u8
        & ONYX_ADC_PGA_GAIN_MASK;
    onyx_write_register(onyx, ONYX_REG_ADC_CONTROL, n);
    mutex_unlock(&mut (*onyx).mutex);

    (n != v) as c_int
}

static inputgain_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Capture Volume\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(onyx_snd_inputgain_info),
    get: Some(onyx_snd_inputgain_get),
    put: Some(onyx_snd_inputgain_put),
    ..unsafe { mem::zeroed() }
};

unsafe extern "C" fn onyx_snd_capture_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXT0: &[u8] = b"Line-In\0";
    static TEXT1: &[u8] = b"Microphone\0";
    let texts: [*const c_char; 2] = [
        TEXT0.as_ptr() as *const c_char,
        TEXT1.as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn onyx_snd_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: i8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_ADC_CONTROL, &mut v as *mut i8 as *mut u8);

    (*ucontrol).value.enumerated.item[0] = ((v as u8 & ONYX_ADC_INPUT_MIC) != 0) as c_uint;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe fn onyx_set_capture_source(onyx: *mut onyx, mic: c_int) {
    let mut v: i8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_ADC_CONTROL, &mut v as *mut i8 as *mut u8);
    v = (v as u8 & !ONYX_ADC_INPUT_MIC) as i8;
    if mic != 0 {
        v = (v as u8 | ONYX_ADC_INPUT_MIC) as i8;
    }
    onyx_write_register(onyx, ONYX_REG_ADC_CONTROL, v as u8);
    mutex_unlock(&mut (*onyx).mutex);
}

unsafe extern "C" fn onyx_snd_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -EINVAL;
    }
    onyx_set_capture_source(
        snd_kcontrol_chip(kcontrol) as *mut onyx,
        (*ucontrol).value.enumerated.item[0] as c_int,
    );
    1
}

static capture_source_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    /* If we name this 'Input Source', it properly shows up in
     * alsamixer as a selection, * but it's shown under the
     * 'Playback' category.
     * If I name it 'Capture Source', it shows up in strange
     * ways (two bools of which one can be selected at a
     * time) but at least it's shown in the 'Capture'
     * category.
     * I was told that this was due to backward compatibility,
     * but I don't understand then why the mangling is *not*
     * done when I name it "Input Source".....
     */
    name: b"Capture Source\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_stereo_info),
    get: Some(onyx_snd_capture_source_get),
    put: Some(onyx_snd_capture_source_put),
    ..unsafe { mem::zeroed() }
};

unsafe extern "C" fn onyx_snd_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut c: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DAC_CONTROL, &mut c);

    (*ucontrol).value.integer.value[0] = ((c & ONYX_MUTE_LEFT) == 0) as c_long;
    (*ucontrol).value.integer.value[1] = ((c & ONYX_MUTE_RIGHT) == 0) as c_long;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_snd_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: u8 = 0;
    let mut c: u8;
    let mut err: c_int = -EBUSY;

    mutex_lock(&mut (*onyx).mutex);
    if (*onyx).analog_locked != 0 {
        mutex_unlock(&mut (*onyx).mutex);
        return -EBUSY;
    }

    onyx_read_register(onyx, ONYX_REG_DAC_CONTROL, &mut v);
    c = v;
    c &= !(ONYX_MUTE_RIGHT | ONYX_MUTE_LEFT);
    if (*ucontrol).value.integer.value[0] == 0 {
        c |= ONYX_MUTE_LEFT;
    }
    if (*ucontrol).value.integer.value[1] == 0 {
        c |= ONYX_MUTE_RIGHT;
    }
    err = onyx_write_register(onyx, ONYX_REG_DAC_CONTROL, c);
    mutex_unlock(&mut (*onyx).mutex);

    if err == 0 { (v != c) as c_int } else { err }
}

static mute_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Playback Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_stereo_info),
    get: Some(onyx_snd_mute_get),
    put: Some(onyx_snd_mute_put),
    ..unsafe { mem::zeroed() }
};

const FLAG_POLARITY_INVERT: u8 = 1;
const FLAG_SPDIFLOCK: u8 = 2;

unsafe extern "C" fn onyx_snd_single_bit_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut c: u8 = 0;
    let pv: c_long = (*kcontrol).private_value;
    let polarity: u8 = ((pv >> 16) as u8) & FLAG_POLARITY_INVERT;
    let address: u8 = ((pv >> 8) & 0xff) as u8;
    let mask: u8 = (pv & 0xff) as u8;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, address, &mut c);

    (*ucontrol).value.integer.value[0] = (((c & mask) != 0) as u8 ^ polarity) as c_long;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_snd_single_bit_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: u8 = 0;
    let mut c: u8;
    let err: c_int;
    let pv: c_long = (*kcontrol).private_value;
    let polarity: u8 = ((pv >> 16) as u8) & FLAG_POLARITY_INVERT;
    let spdiflock: u8 = ((pv >> 16) as u8) & FLAG_SPDIFLOCK;
    let address: u8 = ((pv >> 8) & 0xff) as u8;
    let mask: u8 = (pv & 0xff) as u8;

    mutex_lock(&mut (*onyx).mutex);
    if spdiflock != 0 && (*onyx).spdif_locked != 0 {
        /* even if alsamixer doesn't care.. */
        mutex_unlock(&mut (*onyx).mutex);
        return -EBUSY;
    }
    onyx_read_register(onyx, address, &mut v);
    c = v;
    c &= !mask;
    if (((*ucontrol).value.integer.value[0] != 0) as u8 ^ polarity) != 0 {
        c |= mask;
    }
    err = onyx_write_register(onyx, address, c);
    mutex_unlock(&mut (*onyx).mutex);

    if err == 0 { (v != c) as c_int } else { err }
}

const fn single_bit_private_value(flags: u8, address: u8, mask: u8) -> c_long {
    (((flags as c_long) << 16) | ((address as c_long) << 8) | mask as c_long) as c_long
}

static spdif_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: SNDRV_CTL_NAME_IEC958_PLAYBACK_SWITCH,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(onyx_snd_single_bit_get),
    put: Some(onyx_snd_single_bit_put),
    private_value: single_bit_private_value(FLAG_SPDIFLOCK, ONYX_REG_DIG_INFO4, ONYX_SPDIF_ENABLE),
    ..unsafe { mem::zeroed() }
};

static ovr1_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Oversampling Rate\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(onyx_snd_single_bit_get),
    put: Some(onyx_snd_single_bit_put),
    private_value: single_bit_private_value(0, ONYX_REG_DAC_CONTROL, ONYX_OVR1),
    ..unsafe { mem::zeroed() }
};

static flt0_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Fast Digital Filter Rolloff\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(onyx_snd_single_bit_get),
    put: Some(onyx_snd_single_bit_put),
    private_value: single_bit_private_value(FLAG_POLARITY_INVERT, ONYX_REG_DAC_FILTER, ONYX_ROLLOFF_FAST),
    ..unsafe { mem::zeroed() }
};

static hpf_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Highpass Filter\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(onyx_snd_single_bit_get),
    put: Some(onyx_snd_single_bit_put),
    private_value: single_bit_private_value(FLAG_POLARITY_INVERT, ONYX_REG_ADC_HPF_BYPASS, ONYX_HPF_DISABLE),
    ..unsafe { mem::zeroed() }
};

static dm12_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Digital De-Emphasis\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(onyx_snd_single_bit_get),
    put: Some(onyx_snd_single_bit_put),
    private_value: single_bit_private_value(0, ONYX_REG_DAC_DEEMPH, ONYX_DIGDEEMPH_CTRL),
    ..unsafe { mem::zeroed() }
};

unsafe extern "C" fn onyx_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn onyx_spdif_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    /* datasheet page 30, all others are 0 */
    (*ucontrol).value.iec958.status[0] = 0x3e;
    (*ucontrol).value.iec958.status[1] = 0xff;

    (*ucontrol).value.iec958.status[3] = 0x3f;
    (*ucontrol).value.iec958.status[4] = 0x0f;

    0
}

static onyx_spdif_mask: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_PLAYBACK_CON_MASK,
    info: Some(onyx_spdif_info),
    get: Some(onyx_spdif_mask_get),
    ..unsafe { mem::zeroed() }
};

unsafe extern "C" fn onyx_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DIG_INFO1, &mut v);
    (*ucontrol).value.iec958.status[0] = v & 0x3e;

    onyx_read_register(onyx, ONYX_REG_DIG_INFO2, &mut v);
    (*ucontrol).value.iec958.status[1] = v;

    onyx_read_register(onyx, ONYX_REG_DIG_INFO3, &mut v);
    (*ucontrol).value.iec958.status[3] = v & 0x3f;

    onyx_read_register(onyx, ONYX_REG_DIG_INFO4, &mut v);
    (*ucontrol).value.iec958.status[4] = v & 0x0f;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_spdif_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let onyx: *mut onyx = snd_kcontrol_chip(kcontrol) as *mut onyx;
    let mut v: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DIG_INFO1, &mut v);
    v = (v & !0x3e) | ((*ucontrol).value.iec958.status[0] & 0x3e);
    onyx_write_register(onyx, ONYX_REG_DIG_INFO1, v);

    v = (*ucontrol).value.iec958.status[1];
    onyx_write_register(onyx, ONYX_REG_DIG_INFO2, v);

    onyx_read_register(onyx, ONYX_REG_DIG_INFO3, &mut v);
    v = (v & !0x3f) | ((*ucontrol).value.iec958.status[3] & 0x3f);
    onyx_write_register(onyx, ONYX_REG_DIG_INFO3, v);

    onyx_read_register(onyx, ONYX_REG_DIG_INFO4, &mut v);
    v = (v & !0x0f) | ((*ucontrol).value.iec958.status[4] & 0x0f);
    onyx_write_register(onyx, ONYX_REG_DIG_INFO4, v);
    mutex_unlock(&mut (*onyx).mutex);

    1
}

unsafe fn onyx_set_spdif_pcm_rate(onyx: *mut onyx, rate: c_uint) -> c_int {
    let mut dig_info3: u8 = 0;
    let fs: u8;

    match rate {
        32000 => fs = IEC958_AES3_CON_FS_32000,
        44100 => fs = IEC958_AES3_CON_FS_44100,
        48000 => fs = IEC958_AES3_CON_FS_48000,
        _ => return -EINVAL,
    }

    if onyx_read_register(onyx, ONYX_REG_DIG_INFO3, &mut dig_info3) != 0 {
        return -EBUSY;
    }
    dig_info3 = (dig_info3 & !IEC958_AES3_CON_FS) | fs;
    if onyx_write_register(onyx, ONYX_REG_DIG_INFO3, dig_info3) != 0 {
        return -EBUSY;
    }

    0
}

static onyx_spdif_ctrl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: SNDRV_CTL_NAME_IEC958_PLAYBACK_DEFAULT,
    info: Some(onyx_spdif_info),
    get: Some(onyx_spdif_get),
    put: Some(onyx_spdif_put),
    ..unsafe { mem::zeroed() }
};

/* our registers */

static register_map: [u8; 13] = [
    ONYX_REG_DAC_ATTEN_LEFT,
    ONYX_REG_DAC_ATTEN_RIGHT,
    ONYX_REG_CONTROL,
    ONYX_REG_DAC_CONTROL,
    ONYX_REG_DAC_DEEMPH,
    ONYX_REG_DAC_FILTER,
    ONYX_REG_DAC_OUTPHASE,
    ONYX_REG_ADC_CONTROL,
    ONYX_REG_ADC_HPF_BYPASS,
    ONYX_REG_DIG_INFO1,
    ONYX_REG_DIG_INFO2,
    ONYX_REG_DIG_INFO3,
    ONYX_REG_DIG_INFO4,
];

static initial_values: [u8; 13] = [
    0x80,
    0x80, /* muted */
    ONYX_MRST | ONYX_SRST, /* but handled specially! */
    ONYX_MUTE_LEFT | ONYX_MUTE_RIGHT,
    0, /* no deemphasis */
    ONYX_DAC_FILTER_ALWAYS,
    ONYX_OUTPHASE_INVERTED,
    ((-1i32 /*dB*/ + 8) & 0xF) as u8, /* line in selected, -1 dB gain*/
    ONYX_ADC_HPF_ALWAYS,
    (1 << 2), /* pcm audio */
    2, /* category: pcm coder */
    0, /* sampling frequency 44.1 kHz, clock accuracy level II */
    1, /* 24 bit depth */
];

/* reset registers of chip, either to initial or to previous values */
unsafe fn onyx_register_init(onyx: *mut onyx) -> c_int {
    let mut i: usize;
    let mut val: u8 = 0;
    let mut regs: [u8; 13] = [0; 13];

    if (*onyx).initialised == 0 {
        ptr::copy_nonoverlapping(initial_values.as_ptr(), regs.as_mut_ptr(), initial_values.len());
        if onyx_read_register(onyx, ONYX_REG_CONTROL, &mut val) != 0 {
            return -1;
        }
        val &= !ONYX_SILICONVERSION;
        val |= initial_values[3];
        regs[3] = val;
    } else {
        i = 0;
        while i < register_map.len() {
            regs[i] = (*onyx).cache[(register_map[i] - FIRSTREGISTER) as usize];
            i += 1;
        }
    }

    i = 0;
    while i < register_map.len() {
        if onyx_write_register(onyx, register_map[i], regs[i]) != 0 {
            return -1;
        }
        i += 1;
    }
    (*onyx).initialised = 1;
    0
}

static mut onyx_transfers: [transfer_info; 5] = [
    transfer_info {
        /* analog input */
        formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_8000_96000,
        transfer_in: 1,
        must_be_clock_source: 0,
        tag: 0,
        ..unsafe { mem::zeroed() }
    },
    transfer_info {
        /* if analog and digital are currently off, anything should go,
         * so this entry describes everything we can do... */
        formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        /* If SNDRV_PCM_FMTBIT_COMPRESSED_16BE is available, C also ORs it here. */
        rates: SNDRV_PCM_RATE_8000_96000,
        tag: 0,
        ..unsafe { mem::zeroed() }
    },
    transfer_info {
        /* analog output */
        formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_8000_96000,
        transfer_in: 0,
        must_be_clock_source: 0,
        tag: 1,
        ..unsafe { mem::zeroed() }
    },
    transfer_info {
        /* digital pcm output, also possible for analog out */
        formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
        transfer_in: 0,
        must_be_clock_source: 0,
        tag: 2,
        ..unsafe { mem::zeroed() }
    },
    /* If SNDRV_PCM_FMTBIT_COMPRESSED_16BE is defined, C inserts a digital compressed output
     * transfer before this sentinel.
     */
    transfer_info { ..unsafe { mem::zeroed() } },
];

unsafe extern "C" fn onyx_usable(
    cii: *mut codec_info_item,
    ti: *mut transfer_info,
    _out: *mut transfer_info,
) -> c_int {
    let mut v: u8 = 0;
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;
    let spdif_enabled: c_int;
    let analog_enabled: c_int;

    mutex_lock(&mut (*onyx).mutex);
    onyx_read_register(onyx, ONYX_REG_DIG_INFO4, &mut v);
    spdif_enabled = ((v & ONYX_SPDIF_ENABLE) != 0) as c_int;
    onyx_read_register(onyx, ONYX_REG_DAC_CONTROL, &mut v);
    analog_enabled =
        ((v & (ONYX_MUTE_RIGHT | ONYX_MUTE_LEFT)) != (ONYX_MUTE_RIGHT | ONYX_MUTE_LEFT)) as c_int;
    mutex_unlock(&mut (*onyx).mutex);

    match (*ti).tag {
        0 => return 1,
        1 => return analog_enabled,
        2 => return spdif_enabled,
        _ => {}
    }
    1
}

unsafe extern "C" fn onyx_prepare(
    cii: *mut codec_info_item,
    _bi: *mut bus_info,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut v: u8 = 0;
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;

    mutex_lock(&mut (*onyx).mutex);

    /* If SNDRV_PCM_FMTBIT_COMPRESSED_16BE is defined, C mutes and locks analog output
     * when substream->runtime->format equals that format.
     */
    match (*(*substream).runtime).rate {
        32000 | 44100 | 48000 => {
            if (*onyx).codec.connected & 2 != 0 {
                let ret = onyx_set_spdif_pcm_rate(onyx, (*(*substream).runtime).rate);
                mutex_unlock(&mut (*onyx).mutex);
                return ret;
            }
            mutex_unlock(&mut (*onyx).mutex);
            return 0;
        }
        _ => {
            /* got some rate that the digital output can't do,
             * so disable and lock it */
            onyx_read_register((*cii).codec_data as *mut onyx, ONYX_REG_DIG_INFO4, &mut v);
            if onyx_write_register(onyx, ONYX_REG_DIG_INFO4, v & !ONYX_SPDIF_ENABLE) != 0 {
                mutex_unlock(&mut (*onyx).mutex);
                return -EBUSY;
            }
            (*onyx).spdif_locked = 1;
            mutex_unlock(&mut (*onyx).mutex);
            return 0;
        }
    }
}

unsafe extern "C" fn onyx_open(
    cii: *mut codec_info_item,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;

    mutex_lock(&mut (*onyx).mutex);
    (*onyx).open_count += 1;
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_close(
    cii: *mut codec_info_item,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;

    mutex_lock(&mut (*onyx).mutex);
    (*onyx).open_count -= 1;
    if (*onyx).open_count == 0 {
        (*onyx).analog_locked = 0;
        (*onyx).spdif_locked = (*onyx).analog_locked;
    }
    mutex_unlock(&mut (*onyx).mutex);

    0
}

unsafe extern "C" fn onyx_switch_clock(
    cii: *mut codec_info_item,
    what: clock_switch,
) -> c_int {
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;

    mutex_lock(&mut (*onyx).mutex);
    /* this *MUST* be more elaborate later... */
    match what {
        CLOCK_SWITCH_PREPARE_SLAVE => {
            ((*(*(*onyx).codec.gpio).methods).all_amps_off.unwrap())((*onyx).codec.gpio);
        }
        CLOCK_SWITCH_SLAVE => {
            ((*(*(*onyx).codec.gpio).methods).all_amps_restore.unwrap())((*onyx).codec.gpio);
        }
        _ => {}
    }
    mutex_unlock(&mut (*onyx).mutex);

    0
}

/* CONFIG_PM */
unsafe extern "C" fn onyx_suspend(cii: *mut codec_info_item, _state: pm_message_t) -> c_int {
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;
    let mut v: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);
    if onyx_read_register(onyx, ONYX_REG_CONTROL, &mut v) != 0 {
        mutex_unlock(&mut (*onyx).mutex);
        return -ENXIO;
    }
    onyx_write_register(onyx, ONYX_REG_CONTROL, v | ONYX_ADPSV | ONYX_DAPSV);
    /* Apple does a sleep here but the datasheet says to do it on resume */
    mutex_unlock(&mut (*onyx).mutex);
    0
}

unsafe extern "C" fn onyx_resume(cii: *mut codec_info_item) -> c_int {
    let onyx: *mut onyx = (*cii).codec_data as *mut onyx;
    let mut v: u8 = 0;

    mutex_lock(&mut (*onyx).mutex);

    /* reset codec */
    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 0);
    msleep(1);
    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 1);
    msleep(1);
    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 0);
    msleep(1);

    /* take codec out of suspend (if it still is after reset) */
    if onyx_read_register(onyx, ONYX_REG_CONTROL, &mut v) != 0 {
        mutex_unlock(&mut (*onyx).mutex);
        return -ENXIO;
    }
    onyx_write_register(onyx, ONYX_REG_CONTROL, v & !(ONYX_ADPSV | ONYX_DAPSV));
    /* FIXME: should divide by sample rate, but 8k is the lowest we go */
    msleep(2205000 / 8000);
    /* reset all values */
    onyx_register_init(onyx);
    mutex_unlock(&mut (*onyx).mutex);
    0
}

static mut onyx_codec_info: codec_info = codec_info {
    transfers: unsafe { onyx_transfers.as_mut_ptr() },
    sysclock_factor: 256,
    bus_factor: 64,
    owner: THIS_MODULE,
    usable: Some(onyx_usable),
    prepare: Some(onyx_prepare),
    open: Some(onyx_open),
    close: Some(onyx_close),
    switch_clock: Some(onyx_switch_clock),
    /* CONFIG_PM */
    suspend: Some(onyx_suspend),
    resume: Some(onyx_resume),
    ..unsafe { mem::zeroed() }
};

unsafe extern "C" fn onyx_init_codec(codec: *mut aoa_codec) -> c_int {
    let onyx: *mut onyx = codec_to_onyx(codec);
    let mut ctl: *mut snd_kcontrol;
    let mut ci: *mut codec_info = &raw mut onyx_codec_info;
    let mut v: u8 = 0;
    let mut err: c_int;

    if (*onyx).codec.gpio.is_null() || (*(*onyx).codec.gpio).methods.is_null() {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"gpios not assigned!!\n\0".as_ptr());
        return -EINVAL;
    }

    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 0);
    msleep(1);
    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 1);
    msleep(1);
    ((*(*(*onyx).codec.gpio).methods).set_hw_reset.unwrap())((*onyx).codec.gpio, 0);
    msleep(1);

    if onyx_register_init(onyx) != 0 {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"failed to initialise onyx registers\n\0".as_ptr());
        return -ENODEV;
    }

    if aoa_snd_device_new(SNDRV_DEV_CODEC, onyx as *mut c_void, &ops) != 0 {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"failed to create onyx snd device!\n\0".as_ptr());
        return -ENODEV;
    }

    /* nothing connected? what a joke! */
    if ((*onyx).codec.connected & 0xF) == 0 {
        return -ENOTCONN;
    }

    /* if no inputs are present... */
    if ((*onyx).codec.connected & 0xC) == 0 {
        if (*onyx).codec_info.is_null() {
            (*onyx).codec_info = kmalloc_obj(mem::size_of::<codec_info>()) as *mut codec_info;
        }
        if (*onyx).codec_info.is_null() {
            return -ENOMEM;
        }
        ci = (*onyx).codec_info;
        *ci = onyx_codec_info;
        (*ci).transfers = (*ci).transfers.add(1);
    }

    /* if no outputs are present... */
    if ((*onyx).codec.connected & 3) == 0 {
        if (*onyx).codec_info.is_null() {
            (*onyx).codec_info = kmalloc_obj(mem::size_of::<codec_info>()) as *mut codec_info;
        }
        if (*onyx).codec_info.is_null() {
            return -ENOMEM;
        }
        ci = (*onyx).codec_info;
        /* this is fine as there have to be inputs
         * if we end up in this part of the code */
        *ci = onyx_codec_info;
        (*(*ci).transfers.add(1)).formats = 0;
    }

    if ((*(*onyx).codec.soundbus_dev).attach_codec.unwrap())(
        (*onyx).codec.soundbus_dev,
        aoa_get_card(),
        ci,
        onyx as *mut c_void,
    ) != 0
    {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"error creating onyx pcm\n\0".as_ptr());
        return -ENODEV;
    }

    macro_rules! addctl {
        ($n:expr) => {{
            ctl = snd_ctl_new1(&$n, onyx as *mut c_void);
            if !ctl.is_null() {
                (*ctl).id.device = (*(*(*onyx).codec.soundbus_dev).pcm).device;
                err = aoa_snd_ctl_add(ctl);
                if err != 0 {
                    ((*(*onyx).codec.soundbus_dev).detach_codec.unwrap())(
                        (*onyx).codec.soundbus_dev,
                        onyx as *mut c_void,
                    );
                    snd_device_free(aoa_get_card(), onyx as *mut c_void);
                    return err;
                }
            }
        }};
    }

    if !(*(*onyx).codec.soundbus_dev).pcm.is_null() {
        /* give the user appropriate controls
         * depending on what inputs are connected */
        if ((*onyx).codec.connected & 0xC) == 0xC {
            addctl!(capture_source_control);
        } else if ((*onyx).codec.connected & 4) != 0 {
            onyx_set_capture_source(onyx, 0);
        } else {
            onyx_set_capture_source(onyx, 1);
        }
        if ((*onyx).codec.connected & 0xC) != 0 {
            addctl!(inputgain_control);
        }

        /* depending on what output is connected,
         * give the user appropriate controls */
        if ((*onyx).codec.connected & 1) != 0 {
            addctl!(volume_control);
            addctl!(mute_control);
            addctl!(ovr1_control);
            addctl!(flt0_control);
            addctl!(hpf_control);
            addctl!(dm12_control);
            /* spdif control defaults to off */
        }
        if ((*onyx).codec.connected & 2) != 0 {
            addctl!(onyx_spdif_mask);
            addctl!(onyx_spdif_ctrl);
        }
        if ((*onyx).codec.connected & 3) == 3 {
            addctl!(spdif_control);
        }
        /* if only S/PDIF is connected, enable it unconditionally */
        if ((*onyx).codec.connected & 3) == 2 {
            onyx_read_register(onyx, ONYX_REG_DIG_INFO4, &mut v);
            v |= ONYX_SPDIF_ENABLE;
            onyx_write_register(onyx, ONYX_REG_DIG_INFO4, v);
        }
    }
    printk(KERN_INFO, PFX.as_ptr() as *const c_char, b"attached to onyx codec via i2c\n\0".as_ptr());

    0
}

unsafe extern "C" fn onyx_exit_codec(codec: *mut aoa_codec) {
    let onyx: *mut onyx = codec_to_onyx(codec);

    if (*onyx).codec.soundbus_dev.is_null() {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"onyx_exit_codec called without soundbus_dev!\n\0".as_ptr());
        return;
    }
    ((*(*onyx).codec.soundbus_dev).detach_codec.unwrap())(
        (*onyx).codec.soundbus_dev,
        onyx as *mut c_void,
    );
}

unsafe extern "C" fn onyx_i2c_probe(client: *mut i2c_client) -> c_int {
    let node: *mut device_node = (*client).dev.of_node;
    let onyx: *mut onyx;
    let mut dummy: u8 = 0;

    onyx = kzalloc_obj(mem::size_of::<onyx>()) as *mut onyx;

    if onyx.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*onyx).mutex);
    (*onyx).i2c = client;
    i2c_set_clientdata(client, onyx as *mut c_void);

    /* we try to read from register ONYX_REG_CONTROL
     * to check if the codec is present */
    if onyx_read_register(onyx, ONYX_REG_CONTROL, &mut dummy) != 0 {
        printk(KERN_ERR, PFX.as_ptr() as *const c_char, b"failed to read control register\n\0".as_ptr());
        kfree(onyx as *mut c_void);
        return -ENODEV;
    }

    strscpy((*onyx).codec.name.as_mut_ptr(), b"onyx\0".as_ptr() as *const c_char);
    (*onyx).codec.owner = THIS_MODULE;
    (*onyx).codec.init = Some(onyx_init_codec);
    (*onyx).codec.exit = Some(onyx_exit_codec);
    (*onyx).codec.node = of_node_get(node);

    if aoa_codec_register(&mut (*onyx).codec) != 0 {
        of_node_put((*onyx).codec.node);
        kfree(onyx as *mut c_void);
        return -ENODEV;
    }
    printk(KERN_DEBUG, PFX.as_ptr() as *const c_char, b"created and attached onyx instance\n\0".as_ptr());
    0
}

unsafe extern "C" fn onyx_i2c_remove(client: *mut i2c_client) {
    let onyx: *mut onyx = i2c_get_clientdata(client) as *mut onyx;

    aoa_codec_unregister(&mut (*onyx).codec);
    of_node_put((*onyx).codec.node);
    kfree((*onyx).codec_info as *mut c_void);
    kfree(onyx as *mut c_void);
}

static onyx_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"MAC,pcm3052\0",
        ..unsafe { mem::zeroed() }
    },
    i2c_device_id {
        ..unsafe { mem::zeroed() }
    },
];

static mut onyx_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"aoa_codec_onyx\0".as_ptr() as *const c_char,
        ..unsafe { mem::zeroed() }
    },
    probe: Some(onyx_i2c_probe),
    remove: Some(onyx_i2c_remove),
    id_table: onyx_i2c_id.as_ptr(),
    ..unsafe { mem::zeroed() }
};

module_i2c_driver!(onyx_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
