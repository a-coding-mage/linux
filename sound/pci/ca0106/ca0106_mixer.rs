// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 2004 James Courtier-Dutton <James@superbug.demon.co.uk>
 *  Driver CA0106 chips. e.g. Sound Blaster Audigy LS and Live 24bit
 *  Version: 0.0.18
 *
 *  FEATURES currently supported:
 *    See ca0106_main.c for features.
 *
 *  Changelog:
 *    Support interrupts per period.
 *    Removed noise from Center/LFE channel when in Analog mode.
 *    Rename and remove mixer controls.
 *  0.0.6
 *    Use separate card based DMA buffer for periods table list.
 *  0.0.7
 *    Change remove and rename ctrls into lists.
 *  0.0.8
 *    Try to fix capture sources.
 *  0.0.9
 *    Fix AC3 output.
 *    Enable S32_LE format support.
 *  0.0.10
 *    Enable playback 48000 and 96000 rates. (Rates other that these do not work, even with "plug:front".)
 *  0.0.11
 *    Add Model name recognition.
 *  0.0.12
 *    Correct interrupt timing. interrupt at end of period, instead of in the middle of a playback period.
 *    Remove redundent "voice" handling.
 *  0.0.13
 *    Single trigger call for multi channels.
 *  0.0.14
 *    Set limits based on what the sound card hardware can do.
 *    playback periods_min=2, periods_max=8
 *    capture hw constraints require period_size = n * 64 bytes.
 *    playback hw constraints require period_size = n * 64 bytes.
 *  0.0.15
 *    Separated ca0106.c into separate functional .c files.
 *  0.0.16
 *    Modified Copyright message.
 *  0.0.17
 *    Implement Mic and Line in Capture.
 *  0.0.18
 *    Add support for mute control on SB Live 24bit (cards w/ SPI DAC)
 *
 *  This code was initially based on code from ALSA's emu10k1x.c which is:
 *  Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 */

// C include dependencies removed from executable Rust:
// linux/delay.h, linux/init.h, linux/interrupt.h, linux/moduleparam.h,
// sound/core.h, sound/initval.h, sound/pcm.h, sound/ac97_codec.h,
// sound/info.h, sound/tlv.h, linux/io.h, and "ca0106.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
use core::mem::{size_of, zeroed};
use core::ptr;

type u32 = c_uint;

extern "C" {
    fn snd_ca0106_ptr_write(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint, data: c_uint);
    fn snd_ca0106_ptr_read(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint) -> c_uint;
    fn snd_ca0106_i2c_write(emu: *mut snd_ca0106, reg: c_uint, data: c_uint);
    fn snd_ca0106_spi_write(emu: *mut snd_ca0106, data: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ca0106;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_uint;
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn inl(addr: c_ulong) -> c_uint;
    fn outl(value: c_uint, addr: c_ulong);
    fn memset(s: *mut core::ffi::c_void, c: c_int, n: usize) -> *mut core::ffi::c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_rename(card: *mut snd_card, kctl: *mut snd_kcontrol, name: *const c_char);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut snd_ca0106)
        -> *mut snd_kcontrol;
    fn snd_ctl_make_virtual_master(
        name: *const c_char,
        tlv: *const c_uint,
    ) -> *mut snd_kcontrol;
    fn snd_ctl_add_followers(
        card: *mut snd_card,
        master: *mut snd_kcontrol,
        followers: *const *const c_char,
    ) -> c_int;
}

extern "C" {
    static snd_ca0106_db_scale1: [c_uint; 4];
    static snd_ca0106_db_scale2: [c_uint; 4];
    static snd_ca0106_master_db_scale: [c_uint; 4];
}

unsafe fn ca0106_spdif_enable(emu: *mut snd_ca0106) {
    let mut val: c_uint;

    if (*emu).spdif_enable != 0 {
        /* Digital */
        snd_ca0106_ptr_write(emu, SPDIF_SELECT1, 0, 0xf);
        snd_ca0106_ptr_write(emu, SPDIF_SELECT2, 0, 0x0b000000);
        val = snd_ca0106_ptr_read(emu, CAPTURE_CONTROL, 0) & !0x1000;
        snd_ca0106_ptr_write(emu, CAPTURE_CONTROL, 0, val);
        val = inl((*emu).port + CA0106_GPIO) & !0x101;
        outl(val, (*emu).port + CA0106_GPIO);
    } else {
        /* Analog */
        snd_ca0106_ptr_write(emu, SPDIF_SELECT1, 0, 0xf);
        snd_ca0106_ptr_write(emu, SPDIF_SELECT2, 0, 0x000f0000);
        val = snd_ca0106_ptr_read(emu, CAPTURE_CONTROL, 0) | 0x1000;
        snd_ca0106_ptr_write(emu, CAPTURE_CONTROL, 0, val);
        val = inl((*emu).port + CA0106_GPIO) | 0x101;
        outl(val, (*emu).port + CA0106_GPIO);
    }
}

unsafe fn ca0106_set_capture_source(emu: *mut snd_ca0106) {
    let val: c_uint = (*emu).capture_source;
    let source: c_uint = (val << 28) | (val << 24) | (val << 20) | (val << 16);
    let mask: c_uint = snd_ca0106_ptr_read(emu, CAPTURE_SOURCE, 0) & 0xffff;
    snd_ca0106_ptr_write(emu, CAPTURE_SOURCE, 0, source | mask);
}

unsafe fn ca0106_set_i2c_capture_source(emu: *mut snd_ca0106, val: c_uint, force: c_int) {
    let mut ngain: c_uint;
    let mut ogain: c_uint;
    let source: u32;

    snd_ca0106_i2c_write(emu, ADC_MUX, 0); /* Mute input */
    ngain = (*emu).i2c_capture_volume[val as usize][0]; /* Left */
    ogain = (*emu).i2c_capture_volume[(*emu).i2c_capture_source as usize][0]; /* Left */
    if force != 0 || ngain != ogain {
        snd_ca0106_i2c_write(emu, ADC_ATTEN_ADCL, ngain & 0xff);
    }
    ngain = (*emu).i2c_capture_volume[val as usize][1]; /* Right */
    ogain = (*emu).i2c_capture_volume[(*emu).i2c_capture_source as usize][1]; /* Right */
    if force != 0 || ngain != ogain {
        snd_ca0106_i2c_write(emu, ADC_ATTEN_ADCR, ngain & 0xff);
    }
    source = 1 << val;
    snd_ca0106_i2c_write(emu, ADC_MUX, source); /* Set source */
    (*emu).i2c_capture_source = val;
}

unsafe fn ca0106_set_capture_mic_line_in(emu: *mut snd_ca0106) {
    let mut tmp: u32;

    if (*emu).capture_mic_line_in != 0 {
        /* snd_ca0106_i2c_write(emu, ADC_MUX, 0); */ /* Mute input */
        tmp = inl((*emu).port + CA0106_GPIO) & !0x400;
        tmp = tmp | 0x400;
        outl(tmp, (*emu).port + CA0106_GPIO);
        /* snd_ca0106_i2c_write(emu, ADC_MUX, ADC_MUX_MIC); */
    } else {
        /* snd_ca0106_i2c_write(emu, ADC_MUX, 0); */ /* Mute input */
        tmp = inl((*emu).port + CA0106_GPIO) & !0x400;
        outl(tmp, (*emu).port + CA0106_GPIO);
        /* snd_ca0106_i2c_write(emu, ADC_MUX, ADC_MUX_LINEIN); */
    }
}

unsafe fn ca0106_set_spdif_bits(emu: *mut snd_ca0106, idx: c_int) {
    snd_ca0106_ptr_write(emu, SPCS0 + idx as c_uint, 0, (*emu).spdif_str_bits[idx as usize]);
}

const snd_ca0106_shared_spdif_info: unsafe extern "C" fn(
    *mut snd_kcontrol,
    *mut snd_ctl_elem_info,
) -> c_int = snd_ctl_boolean_mono_info;

unsafe extern "C" fn snd_ca0106_shared_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] = (*emu).spdif_enable as c_long;
    0
}

unsafe extern "C" fn snd_ca0106_shared_spdif_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let val: c_uint;
    let mut change: c_int = 0;

    val = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };
    change = ((*emu).spdif_enable != val) as c_int;
    if change != 0 {
        (*emu).spdif_enable = val;
        ca0106_spdif_enable(emu);
    }
    change
}

unsafe extern "C" fn snd_ca0106_capture_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 6] = [
        b"IEC958 out\0".as_ptr() as *const c_char,
        b"i2s mixer out\0".as_ptr() as *const c_char,
        b"IEC958 in\0".as_ptr() as *const c_char,
        b"i2s in\0".as_ptr() as *const c_char,
        b"AC97 in\0".as_ptr() as *const c_char,
        b"SRC out\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 6, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_ca0106_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.enumerated.item[0] = (*emu).capture_source;
    0
}

unsafe extern "C" fn snd_ca0106_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let val: c_uint;
    let mut change: c_int = 0;

    val = (*ucontrol).value.enumerated.item[0];
    if val >= 6 {
        return -EINVAL;
    }
    change = ((*emu).capture_source != val) as c_int;
    if change != 0 {
        (*emu).capture_source = val;
        ca0106_set_capture_source(emu);
    }
    change
}

unsafe extern "C" fn snd_ca0106_i2c_capture_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 4] = [
        b"Phone\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char,
        b"Line in\0".as_ptr() as *const c_char,
        b"Aux\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 4, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_ca0106_i2c_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.enumerated.item[0] = (*emu).i2c_capture_source;
    0
}

unsafe extern "C" fn snd_ca0106_i2c_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let source_id: c_uint;
    let mut change: c_int = 0;
    /* If the capture source has changed,
     * update the capture volume from the cached value
     * for the particular source.
     */
    source_id = (*ucontrol).value.enumerated.item[0];
    if source_id >= 4 {
        return -EINVAL;
    }
    change = ((*emu).i2c_capture_source != source_id) as c_int;
    if change != 0 {
        ca0106_set_i2c_capture_source(emu, source_id, 0);
    }
    change
}

unsafe extern "C" fn snd_ca0106_capture_line_in_side_out_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 2] = [
        b"Side out\0".as_ptr() as *const c_char,
        b"Line in\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 2, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_ca0106_capture_mic_line_in_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 2] = [
        b"Line in\0".as_ptr() as *const c_char,
        b"Mic in\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 2, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_ca0106_capture_mic_line_in_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.enumerated.item[0] = (*emu).capture_mic_line_in;
    0
}

unsafe extern "C" fn snd_ca0106_capture_mic_line_in_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let val: c_uint;
    let mut change: c_int = 0;

    val = (*ucontrol).value.enumerated.item[0];
    if val > 1 {
        return -EINVAL;
    }
    change = ((*emu).capture_mic_line_in != val) as c_int;
    if change != 0 {
        (*emu).capture_mic_line_in = val;
        ca0106_set_capture_mic_line_in(emu);
    }
    change
}

static snd_ca0106_capture_mic_line_in: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Shared Mic/Line in Capture Switch\0".as_ptr() as *const c_char,
    info: Some(snd_ca0106_capture_mic_line_in_info),
    get: Some(snd_ca0106_capture_mic_line_in_get),
    put: Some(snd_ca0106_capture_mic_line_in_put),
    ..unsafe { zeroed() }
};

static snd_ca0106_capture_line_in_side_out: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Shared Line in/Side out Capture Switch\0".as_ptr() as *const c_char,
    info: Some(snd_ca0106_capture_line_in_side_out_info),
    get: Some(snd_ca0106_capture_mic_line_in_get),
    put: Some(snd_ca0106_capture_mic_line_in_put),
    ..unsafe { zeroed() }
};

unsafe extern "C" fn snd_ca0106_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe fn decode_spdif_bits(status: *mut u8, bits: c_uint) {
    *status.add(0) = ((bits >> 0) & 0xff) as u8;
    *status.add(1) = ((bits >> 8) & 0xff) as u8;
    *status.add(2) = ((bits >> 16) & 0xff) as u8;
    *status.add(3) = ((bits >> 24) & 0xff) as u8;
}

unsafe extern "C" fn snd_ca0106_spdif_get_default(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);

    decode_spdif_bits((*ucontrol).value.iec958.status.as_mut_ptr(), (*emu).spdif_bits[idx as usize]);
    0
}

unsafe extern "C" fn snd_ca0106_spdif_get_stream(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);

    decode_spdif_bits((*ucontrol).value.iec958.status.as_mut_ptr(), (*emu).spdif_str_bits[idx as usize]);
    0
}

unsafe extern "C" fn snd_ca0106_spdif_get_mask(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.iec958.status[0] = 0xff;
    (*ucontrol).value.iec958.status[1] = 0xff;
    (*ucontrol).value.iec958.status[2] = 0xff;
    (*ucontrol).value.iec958.status[3] = 0xff;
    0
}

unsafe fn encode_spdif_bits(status: *mut u8) -> c_uint {
    ((*status.add(0) as c_uint) << 0)
        | ((*status.add(1) as c_uint) << 8)
        | ((*status.add(2) as c_uint) << 16)
        | ((*status.add(3) as c_uint) << 24)
}

unsafe extern "C" fn snd_ca0106_spdif_put_default(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let val: c_uint;

    val = encode_spdif_bits((*ucontrol).value.iec958.status.as_mut_ptr());
    if val != (*emu).spdif_bits[idx as usize] {
        (*emu).spdif_bits[idx as usize] = val;
        /* FIXME: this isn't safe, but needed to keep the compatibility
         * with older alsa-lib config
         */
        (*emu).spdif_str_bits[idx as usize] = val;
        ca0106_set_spdif_bits(emu, idx as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn snd_ca0106_spdif_put_stream(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let val: c_uint;

    val = encode_spdif_bits((*ucontrol).value.iec958.status.as_mut_ptr());
    if val != (*emu).spdif_str_bits[idx as usize] {
        (*emu).spdif_str_bits[idx as usize] = val;
        ca0106_set_spdif_bits(emu, idx as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn snd_ca0106_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 255;
    0
}

unsafe extern "C" fn snd_ca0106_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let value: c_uint;
    let channel_id: c_int;
    let reg: c_int;

    channel_id = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    reg = ((*kcontrol).private_value & 0xff) as c_int;

    value = snd_ca0106_ptr_read(emu, reg as c_uint, channel_id as c_uint);
    (*ucontrol).value.integer.value[0] = (0xff - ((value >> 24) & 0xff)) as c_long; /* Left */
    (*ucontrol).value.integer.value[1] = (0xff - ((value >> 16) & 0xff)) as c_long; /* Right */
    0
}

unsafe extern "C" fn snd_ca0106_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let oval: c_uint;
    let mut nval: c_uint;
    let channel_id: c_int;
    let reg: c_int;

    channel_id = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    reg = ((*kcontrol).private_value & 0xff) as c_int;

    oval = snd_ca0106_ptr_read(emu, reg as c_uint, channel_id as c_uint);
    nval = (((0xff - (*ucontrol).value.integer.value[0]) as c_uint) << 24)
        | (((0xff - (*ucontrol).value.integer.value[1]) as c_uint) << 16);
    nval |= (((0xff - (*ucontrol).value.integer.value[0]) as c_uint) << 8)
        | ((0xff - (*ucontrol).value.integer.value[1]) as c_uint);
    if oval == nval {
        return 0;
    }
    snd_ca0106_ptr_write(emu, reg as c_uint, channel_id as c_uint, nval);
    1
}

unsafe extern "C" fn snd_ca0106_i2c_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 255;
    0
}

unsafe extern "C" fn snd_ca0106_i2c_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let source_id: c_int;

    source_id = (*kcontrol).private_value as c_int;

    (*ucontrol).value.integer.value[0] = (*emu).i2c_capture_volume[source_id as usize][0] as c_long;
    (*ucontrol).value.integer.value[1] = (*emu).i2c_capture_volume[source_id as usize][1] as c_long;
    0
}

unsafe extern "C" fn snd_ca0106_i2c_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let mut ogain: c_uint;
    let mut ngain: c_uint;
    let source_id: c_int;
    let mut change: c_int = 0;

    source_id = (*kcontrol).private_value as c_int;
    ogain = (*emu).i2c_capture_volume[source_id as usize][0]; /* Left */
    ngain = (*ucontrol).value.integer.value[0] as c_uint;
    if ngain > 0xff {
        return -EINVAL;
    }
    if ogain != ngain {
        if (*emu).i2c_capture_source == source_id as c_uint {
            snd_ca0106_i2c_write(emu, ADC_ATTEN_ADCL, ngain & 0xff);
        }
        (*emu).i2c_capture_volume[source_id as usize][0] = (*ucontrol).value.integer.value[0] as c_uint;
        change = 1;
    }
    ogain = (*emu).i2c_capture_volume[source_id as usize][1]; /* Right */
    ngain = (*ucontrol).value.integer.value[1] as c_uint;
    if ngain > 0xff {
        return -EINVAL;
    }
    if ogain != ngain {
        if (*emu).i2c_capture_source == source_id as c_uint {
            snd_ca0106_i2c_write(emu, ADC_ATTEN_ADCR, ngain & 0xff);
        }
        (*emu).i2c_capture_volume[source_id as usize][1] = (*ucontrol).value.integer.value[1] as c_uint;
        change = 1;
    }

    change
}

const spi_mute_info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int =
    snd_ctl_boolean_mono_info;

unsafe extern "C" fn spi_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let reg: c_uint = ((*kcontrol).private_value >> SPI_REG_SHIFT) as c_uint;
    let bit: c_uint = ((*kcontrol).private_value & SPI_REG_MASK) as c_uint;

    (*ucontrol).value.integer.value[0] = if ((*emu).spi_dac_reg[reg as usize] & bit) == 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn spi_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu: *mut snd_ca0106 = snd_kcontrol_chip(kcontrol);
    let reg: c_uint = ((*kcontrol).private_value >> SPI_REG_SHIFT) as c_uint;
    let bit: c_uint = ((*kcontrol).private_value & SPI_REG_MASK) as c_uint;
    let mut ret: c_int;

    ret = ((*emu).spi_dac_reg[reg as usize] & bit) as c_int;
    if (*ucontrol).value.integer.value[0] != 0 {
        if ret == 0 {
            /* bit already cleared, do nothing */
            return 0;
        }
        (*emu).spi_dac_reg[reg as usize] &= !bit;
    } else {
        if ret != 0 {
            /* bit already set, do nothing */
            return 0;
        }
        (*emu).spi_dac_reg[reg as usize] |= bit;
    }

    ret = snd_ca0106_spi_write(emu, (*emu).spi_dac_reg[reg as usize]);
    if ret != 0 { -EINVAL } else { 1 }
}

macro_rules! CA_VOLUME {
    ($xname:expr, $chid:expr, $reg:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            info: Some(snd_ca0106_volume_info),
            get: Some(snd_ca0106_volume_get),
            put: Some(snd_ca0106_volume_put),
            tlv: snd_kcontrol_new_tlv { p: snd_ca0106_db_scale1.as_ptr() },
            private_value: (($chid) << 8) | ($reg),
            ..unsafe { zeroed() }
        }
    };
}

static snd_ca0106_volume_ctls: [snd_kcontrol_new; 15] = [
    CA_VOLUME!(b"Analog Front Playback Volume\0", CONTROL_FRONT_CHANNEL, PLAYBACK_VOLUME2),
    CA_VOLUME!(b"Analog Rear Playback Volume\0", CONTROL_REAR_CHANNEL, PLAYBACK_VOLUME2),
    CA_VOLUME!(b"Analog Center/LFE Playback Volume\0", CONTROL_CENTER_LFE_CHANNEL, PLAYBACK_VOLUME2),
    CA_VOLUME!(b"Analog Side Playback Volume\0", CONTROL_UNKNOWN_CHANNEL, PLAYBACK_VOLUME2),
    CA_VOLUME!(b"IEC958 Front Playback Volume\0", CONTROL_FRONT_CHANNEL, PLAYBACK_VOLUME1),
    CA_VOLUME!(b"IEC958 Rear Playback Volume\0", CONTROL_REAR_CHANNEL, PLAYBACK_VOLUME1),
    CA_VOLUME!(b"IEC958 Center/LFE Playback Volume\0", CONTROL_CENTER_LFE_CHANNEL, PLAYBACK_VOLUME1),
    CA_VOLUME!(b"IEC958 Unknown Playback Volume\0", CONTROL_UNKNOWN_CHANNEL, PLAYBACK_VOLUME1),
    CA_VOLUME!(b"CAPTURE feedback Playback Volume\0", 1, CAPTURE_CONTROL),
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_PLAYBACK_MASK.as_ptr() as *const c_char,
        count: 4,
        info: Some(snd_ca0106_spdif_info),
        get: Some(snd_ca0106_spdif_get_mask),
        ..unsafe { zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"IEC958 Playback Switch\0".as_ptr() as *const c_char,
        info: Some(snd_ca0106_shared_spdif_info),
        get: Some(snd_ca0106_shared_spdif_get),
        put: Some(snd_ca0106_shared_spdif_put),
        ..unsafe { zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Digital Source Capture Enum\0".as_ptr() as *const c_char,
        info: Some(snd_ca0106_capture_source_info),
        get: Some(snd_ca0106_capture_source_get),
        put: Some(snd_ca0106_capture_source_put),
        ..unsafe { zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Analog Source Capture Enum\0".as_ptr() as *const c_char,
        info: Some(snd_ca0106_i2c_capture_source_info),
        get: Some(snd_ca0106_i2c_capture_source_get),
        put: Some(snd_ca0106_i2c_capture_source_put),
        ..unsafe { zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_PLAYBACK_DEFAULT.as_ptr() as *const c_char,
        count: 4,
        info: Some(snd_ca0106_spdif_info),
        get: Some(snd_ca0106_spdif_get_default),
        put: Some(snd_ca0106_spdif_put_default),
        ..unsafe { zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_PLAYBACK_PCM_STREAM.as_ptr() as *const c_char,
        count: 4,
        info: Some(snd_ca0106_spdif_info),
        get: Some(snd_ca0106_spdif_get_stream),
        put: Some(snd_ca0106_spdif_put_stream),
        ..unsafe { zeroed() }
    },
];

macro_rules! I2C_VOLUME {
    ($xname:expr, $chid:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            info: Some(snd_ca0106_i2c_volume_info),
            get: Some(snd_ca0106_i2c_volume_get),
            put: Some(snd_ca0106_i2c_volume_put),
            tlv: snd_kcontrol_new_tlv { p: snd_ca0106_db_scale2.as_ptr() },
            private_value: $chid,
            ..unsafe { zeroed() }
        }
    };
}

static snd_ca0106_volume_i2c_adc_ctls: [snd_kcontrol_new; 4] = [
    I2C_VOLUME!(b"Phone Capture Volume\0", 0),
    I2C_VOLUME!(b"Mic Capture Volume\0", 1),
    I2C_VOLUME!(b"Line in Capture Volume\0", 2),
    I2C_VOLUME!(b"Aux Capture Volume\0", 3),
];

static spi_dmute_reg: [c_int; 5] = [
    SPI_DMUTE0_REG,
    SPI_DMUTE1_REG,
    SPI_DMUTE2_REG,
    0,
    SPI_DMUTE4_REG,
];
static spi_dmute_bit: [c_int; 5] = [
    SPI_DMUTE0_BIT,
    SPI_DMUTE1_BIT,
    SPI_DMUTE2_BIT,
    0,
    SPI_DMUTE4_BIT,
];

unsafe fn snd_ca0106_volume_spi_dac_ctl(
    details: *const snd_ca0106_details,
    channel_id: c_int,
) -> snd_kcontrol_new {
    let mut spi_switch: snd_kcontrol_new = zeroed();
    let reg: c_int;
    let bit: c_int;
    let dac_id: c_int;

    spi_switch.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    spi_switch.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    spi_switch.info = Some(spi_mute_info);
    spi_switch.get = Some(spi_mute_get);
    spi_switch.put = Some(spi_mute_put);

    match channel_id {
        PCM_FRONT_CHANNEL => {
            spi_switch.name = b"Analog Front Playback Switch\0".as_ptr() as *const c_char;
            dac_id = (((*details).spi_dac & 0xf000) >> (4 * 3)) as c_int;
        }
        PCM_REAR_CHANNEL => {
            spi_switch.name = b"Analog Rear Playback Switch\0".as_ptr() as *const c_char;
            dac_id = (((*details).spi_dac & 0x0f00) >> (4 * 2)) as c_int;
        }
        PCM_CENTER_LFE_CHANNEL => {
            spi_switch.name = b"Analog Center/LFE Playback Switch\0".as_ptr() as *const c_char;
            dac_id = (((*details).spi_dac & 0x00f0) >> (4 * 1)) as c_int;
        }
        PCM_UNKNOWN_CHANNEL => {
            spi_switch.name = b"Analog Side Playback Switch\0".as_ptr() as *const c_char;
            dac_id = (((*details).spi_dac & 0x000f) >> (4 * 0)) as c_int;
        }
        _ => {
            /* Unused channel */
            spi_switch.name = ptr::null();
            dac_id = 0;
        }
    }
    reg = spi_dmute_reg[dac_id as usize];
    bit = spi_dmute_bit[dac_id as usize];

    spi_switch.private_value = ((reg << SPI_REG_SHIFT) | bit) as c_ulong;

    spi_switch
}

unsafe fn remove_ctl(card: *mut snd_card, name: *const c_char) -> c_int {
    let mut id: snd_ctl_elem_id = zeroed();
    memset(&mut id as *mut _ as *mut core::ffi::c_void, 0, size_of::<snd_ctl_elem_id>());
    strscpy(id.name.as_mut_ptr(), name);
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    snd_ctl_remove_id(card, &mut id)
}

unsafe fn rename_ctl(card: *mut snd_card, src: *const c_char, dst: *const c_char) -> c_int {
    let kctl: *mut snd_kcontrol = snd_ctl_find_id_mixer(card, src);
    if !kctl.is_null() {
        snd_ctl_rename(card, kctl, dst);
        return 0;
    }
    -ENOENT
}

macro_rules! ADD_CTLS {
    ($card:expr, $emu:expr, $ctls:expr) => {{
        let mut i: usize = 0;
        while i < $ctls.len() {
            let _err = snd_ctl_add($card, snd_ctl_new1(&$ctls[i], $emu));
            if _err < 0 {
                return _err;
            }
            i += 1;
        }
    }};
}

static follower_vols: [*const c_char; 10] = [
    b"Analog Front Playback Volume\0".as_ptr() as *const c_char,
    b"Analog Rear Playback Volume\0".as_ptr() as *const c_char,
    b"Analog Center/LFE Playback Volume\0".as_ptr() as *const c_char,
    b"Analog Side Playback Volume\0".as_ptr() as *const c_char,
    b"IEC958 Front Playback Volume\0".as_ptr() as *const c_char,
    b"IEC958 Rear Playback Volume\0".as_ptr() as *const c_char,
    b"IEC958 Center/LFE Playback Volume\0".as_ptr() as *const c_char,
    b"IEC958 Unknown Playback Volume\0".as_ptr() as *const c_char,
    b"CAPTURE feedback Playback Volume\0".as_ptr() as *const c_char,
    ptr::null(),
];

static follower_sws: [*const c_char; 6] = [
    b"Analog Front Playback Switch\0".as_ptr() as *const c_char,
    b"Analog Rear Playback Switch\0".as_ptr() as *const c_char,
    b"Analog Center/LFE Playback Switch\0".as_ptr() as *const c_char,
    b"Analog Side Playback Switch\0".as_ptr() as *const c_char,
    b"IEC958 Playback Switch\0".as_ptr() as *const c_char,
    ptr::null(),
];

#[no_mangle]
pub unsafe extern "C" fn snd_ca0106_mixer(emu: *mut snd_ca0106) -> c_int {
    let mut err: c_int;
    let card: *mut snd_card = (*emu).card;
    let mut c: *const *const c_char;
    let mut vmaster: *mut snd_kcontrol;
    static ca0106_remove_ctls: [*const c_char; 22] = [
        b"Master Mono Playback Switch\0".as_ptr() as *const c_char,
        b"Master Mono Playback Volume\0".as_ptr() as *const c_char,
        b"3D Control - Switch\0".as_ptr() as *const c_char,
        b"3D Control Sigmatel - Depth\0".as_ptr() as *const c_char,
        b"PCM Playback Switch\0".as_ptr() as *const c_char,
        b"PCM Playback Volume\0".as_ptr() as *const c_char,
        b"CD Playback Switch\0".as_ptr() as *const c_char,
        b"CD Playback Volume\0".as_ptr() as *const c_char,
        b"Phone Playback Switch\0".as_ptr() as *const c_char,
        b"Phone Playback Volume\0".as_ptr() as *const c_char,
        b"Video Playback Switch\0".as_ptr() as *const c_char,
        b"Video Playback Volume\0".as_ptr() as *const c_char,
        b"Beep Playback Switch\0".as_ptr() as *const c_char,
        b"Beep Playback Volume\0".as_ptr() as *const c_char,
        b"Mono Output Select\0".as_ptr() as *const c_char,
        b"Capture Source\0".as_ptr() as *const c_char,
        b"Capture Switch\0".as_ptr() as *const c_char,
        b"Capture Volume\0".as_ptr() as *const c_char,
        b"External Amplifier\0".as_ptr() as *const c_char,
        b"Sigmatel 4-Speaker Stereo Playback Switch\0".as_ptr() as *const c_char,
        b"Surround Phase Inversion Playback Switch\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    static ca0106_rename_ctls: [*const c_char; 21] = [
        b"Master Playback Switch\0".as_ptr() as *const c_char,
        b"Capture Switch\0".as_ptr() as *const c_char,
        b"Master Playback Volume\0".as_ptr() as *const c_char,
        b"Capture Volume\0".as_ptr() as *const c_char,
        b"Line Playback Switch\0".as_ptr() as *const c_char,
        b"AC97 Line Capture Switch\0".as_ptr() as *const c_char,
        b"Line Playback Volume\0".as_ptr() as *const c_char,
        b"AC97 Line Capture Volume\0".as_ptr() as *const c_char,
        b"Aux Playback Switch\0".as_ptr() as *const c_char,
        b"AC97 Aux Capture Switch\0".as_ptr() as *const c_char,
        b"Aux Playback Volume\0".as_ptr() as *const c_char,
        b"AC97 Aux Capture Volume\0".as_ptr() as *const c_char,
        b"Mic Playback Switch\0".as_ptr() as *const c_char,
        b"AC97 Mic Capture Switch\0".as_ptr() as *const c_char,
        b"Mic Playback Volume\0".as_ptr() as *const c_char,
        b"AC97 Mic Capture Volume\0".as_ptr() as *const c_char,
        b"Mic Select\0".as_ptr() as *const c_char,
        b"AC97 Mic Select\0".as_ptr() as *const c_char,
        b"Mic Boost (+20dB)\0".as_ptr() as *const c_char,
        b"AC97 Mic Boost (+20dB)\0".as_ptr() as *const c_char,
        ptr::null(),
    ];
    /* C source used #if 1 around the following remove/rename block. */
    c = ca0106_remove_ctls.as_ptr();
    while !(*c).is_null() {
        remove_ctl(card, *c);
        c = c.add(1);
    }
    c = ca0106_rename_ctls.as_ptr();
    while !(*c).is_null() {
        rename_ctl(card, *c, *c.add(1));
        c = c.add(2);
    }

    ADD_CTLS!(card, emu, snd_ca0106_volume_ctls);
    if (*(*emu).details).i2c_adc == 1 {
        ADD_CTLS!(card, emu, snd_ca0106_volume_i2c_adc_ctls);
        if (*(*emu).details).gpio_type == 1 {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_ca0106_capture_mic_line_in, emu));
        } else {
            /* gpio_type == 2 */
            err = snd_ctl_add(card, snd_ctl_new1(&snd_ca0106_capture_line_in_side_out, emu));
        }
        if err < 0 {
            return err;
        }
    }
    if (*(*emu).details).spi_dac != 0 {
        let mut i: c_int = 0;
        loop {
            let mut ctl: snd_kcontrol_new;
            ctl = snd_ca0106_volume_spi_dac_ctl((*emu).details, i);
            if ctl.name.is_null() {
                break;
            }
            err = snd_ctl_add(card, snd_ctl_new1(&ctl, emu));
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }

    /* Create virtual master controls */
    vmaster = snd_ctl_make_virtual_master(
        b"Master Playback Volume\0".as_ptr() as *const c_char,
        snd_ca0106_master_db_scale.as_ptr(),
    );
    if vmaster.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add(card, vmaster);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add_followers(card, vmaster, follower_vols.as_ptr());
    if err < 0 {
        return err;
    }

    if (*(*emu).details).spi_dac != 0 {
        vmaster = snd_ctl_make_virtual_master(b"Master Playback Switch\0".as_ptr() as *const c_char, ptr::null());
        if vmaster.is_null() {
            return -ENOMEM;
        }
        err = snd_ctl_add(card, vmaster);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add_followers(card, vmaster, follower_sws.as_ptr());
        if err < 0 {
            return err;
        }
    }

    strscpy((*card).mixername.as_mut_ptr(), b"CA0106\0".as_ptr() as *const c_char);
    0
}

/* C source condition: #ifdef CONFIG_PM_SLEEP */
#[cfg(CONFIG_PM_SLEEP)]
#[repr(C)]
struct ca0106_vol_tbl {
    channel_id: c_uint,
    reg: c_uint,
}

#[cfg(CONFIG_PM_SLEEP)]
static saved_volumes: [ca0106_vol_tbl; NUM_SAVED_VOLUMES] = [
    ca0106_vol_tbl { channel_id: CONTROL_FRONT_CHANNEL, reg: PLAYBACK_VOLUME2 },
    ca0106_vol_tbl { channel_id: CONTROL_REAR_CHANNEL, reg: PLAYBACK_VOLUME2 },
    ca0106_vol_tbl { channel_id: CONTROL_CENTER_LFE_CHANNEL, reg: PLAYBACK_VOLUME2 },
    ca0106_vol_tbl { channel_id: CONTROL_UNKNOWN_CHANNEL, reg: PLAYBACK_VOLUME2 },
    ca0106_vol_tbl { channel_id: CONTROL_FRONT_CHANNEL, reg: PLAYBACK_VOLUME1 },
    ca0106_vol_tbl { channel_id: CONTROL_REAR_CHANNEL, reg: PLAYBACK_VOLUME1 },
    ca0106_vol_tbl { channel_id: CONTROL_CENTER_LFE_CHANNEL, reg: PLAYBACK_VOLUME1 },
    ca0106_vol_tbl { channel_id: CONTROL_UNKNOWN_CHANNEL, reg: PLAYBACK_VOLUME1 },
    ca0106_vol_tbl { channel_id: 1, reg: CAPTURE_CONTROL },
];

#[cfg(CONFIG_PM_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn snd_ca0106_mixer_suspend(chip: *mut snd_ca0106) {
    let mut i: c_int;

    /* save volumes */
    i = 0;
    while i < NUM_SAVED_VOLUMES as c_int {
        (*chip).saved_vol[i as usize] = snd_ca0106_ptr_read(
            chip,
            saved_volumes[i as usize].reg,
            saved_volumes[i as usize].channel_id,
        );
        i += 1;
    }
}

#[cfg(CONFIG_PM_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn snd_ca0106_mixer_resume(chip: *mut snd_ca0106) {
    let mut i: c_int;

    i = 0;
    while i < NUM_SAVED_VOLUMES as c_int {
        snd_ca0106_ptr_write(
            chip,
            saved_volumes[i as usize].reg,
            saved_volumes[i as usize].channel_id,
            (*chip).saved_vol[i as usize],
        );
        i += 1;
    }

    ca0106_spdif_enable(chip);
    ca0106_set_capture_source(chip);
    ca0106_set_i2c_capture_source(chip, (*chip).i2c_capture_source, 1);
    i = 0;
    while i < 4 {
        ca0106_set_spdif_bits(chip, i);
        i += 1;
    }
    if (*(*chip).details).i2c_adc != 0 {
        ca0106_set_capture_mic_line_in(chip);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
